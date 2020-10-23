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
use codec::{Codec, Encode};
use core::marker::PhantomData;
use eos_chain::ProducerAuthoritySchedule;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded,
	sudo::{Sudo, SudoEventsDecoder, SudoCall}, balances, UncheckedExtrinsic,
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use sp_keyring::{AccountKeyring};
use frame_support::{
	Parameter, traits::{OriginTrait, UnfilteredDispatchable},
	weights::{Weight, GetDispatchInfo, DispatchClass}, dispatch::PostDispatchInfo,
};
use sp_runtime::{DispatchError, DispatchResult, traits::Dispatchable};

#[subxt::module]
pub trait Utility: System {}

impl Utility for BifrostRuntime {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BatchCall<T: Utility> {
	pub calls: Vec<Encoded>,
	pub _runtime: PhantomData<T>,
}

pub async fn batch_calls(calls: impl IntoIterator<Item=Encoded>, url: &str, signer: &str) -> Result<String, Box<dyn Error>> {
	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	let batch_call = BatchCall::<BifrostRuntime> {
		calls: calls.into_iter().collect(),
		_runtime: PhantomData,
	};

	let block_hash = client.submit(batch_call, &signer).await?;

	Ok(block_hash.to_string())
}
