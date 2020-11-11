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
use crate::utils::read_json_from_file;
use codec::{Decode, Encode};
use core::marker::PhantomData;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded, Event,
	sudo::{Sudo, SudoEventsDecoder, SudoCall}, balances, UncheckedExtrinsic,
};
use subxt::balances::BalancesEventsDecoder;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use crate::voucher::*;

#[subxt::module]
pub trait Utility: System + Voucher {}

impl Utility for BifrostRuntime {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BatchCall<T: Utility> {
	pub calls: Vec<Encoded>,
	pub _runtime: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BatchCompletedEvent<T: Utility> {
	pub _runtime: PhantomData<T>,
}

#[allow(dead_code)]
pub async fn batch_calls(
	calls: impl IntoIterator<Item=Encoded>, 
	client: &Client<BifrostRuntime>, 
	signer: &PairSigner::<BifrostRuntime, Pair>
) -> Result<String, Box<dyn Error>> {
	let batch_call = BatchCall::<BifrostRuntime> {
		calls: calls.into_iter().collect(),
		_runtime: PhantomData,
	};

	// let block_hash = client.submit(batch_call, signer).await?;

	let extrinsic = client.create_signed(batch_call, signer).await?;

	let mut decoder = client.events_decoder::<BatchCall<BifrostRuntime>>();
	decoder.with_utility();

	let batch_events = client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
	let event = batch_events.find_event::<BatchCompletedEvent::<BifrostRuntime>>()?.ok_or("No Event found or decoded.")?;
	let block_hash = batch_events.block;

	Ok(block_hash.to_string())
}
