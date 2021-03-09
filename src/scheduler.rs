// Copyright 2019-2020 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.

use crate::error_types::Error as BifrostxtError;
use codec::Encode;
use core::marker::PhantomData;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{System, SetCodeCall}, Encoded,
	sudo::{Sudo, SudoCall}
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;

#[subxt::module]
pub trait Scheduler: System {}

impl Scheduler for BifrostRuntime {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ScheduleAfterCall<T: Scheduler> {
	pub after: <T as System>::BlockNumber,
	pub maybe_periodic: Option<(<T as System>::BlockNumber, u32)>,
	pub priority: u8,
	pub call: Encoded,
}

#[allow(dead_code)]
pub fn create_sudo_call<'a, T: Sudo>(call: &'a Encoded) -> SudoCall<T> {
	SudoCall {
		call,
		_runtime: PhantomData,
	}
}

#[allow(dead_code)]
pub async fn upgrade_runtime(signer: &str, url: &str) -> Result<String, Box<dyn Error>> {
	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
		.set_url(url)
		.skip_type_sizes_check()
		.build().await?;

	// let wasm = include_bytes!("/Users/liebi/node_runtime.compact.wasm");
	let wasm = b"123";

	let wasm_args = SetCodeCall {
		_runtime: PhantomData,
		code: wasm,
	};
	let wasm_proposal = client.encode(wasm_args)?;

	let schedule_args = ScheduleAfterCall {
		after: 600,
		maybe_periodic: None,
		priority: 0,
		call: wasm_proposal,
	};
	let schedule_proposal = client.encode(schedule_args)?;

	let call = create_sudo_call(&schedule_proposal);
	let trx_id = client.watch(call, &signer).await?;

	Ok(trx_id.block.to_string())
}
