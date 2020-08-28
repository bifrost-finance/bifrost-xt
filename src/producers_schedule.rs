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
use codec::Encode;
use core::marker::PhantomData;
use eos_chain::ProducerAuthoritySchedule;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded,
	sudo::{Sudo, SudoEventsDecoder, SudoCall}, balances
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use sp_keyring::{AccountKeyring};

#[subxt::module]
pub trait BridgeEos: System + Sudo {}

impl BridgeEos for BifrostRuntime {}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SaveProducerScheduleCall<T: BridgeEos + Sudo> {
	pub ps: ProducerAuthoritySchedule,
	pub _runtime: PhantomData<T>,
}

#[allow(dead_code)]
pub fn create_sudo_call<'a, T: Sudo>(call: &'a Encoded) -> SudoCall<T> {
	SudoCall {
		call,
		_runtime: PhantomData,
	}
}

#[allow(dead_code)]
pub async fn save_producer_schedule_call(signer: &str, url: &str, json_path: &str) -> Result<String, Box<dyn Error>> {
	let schedule = read_json_from_file(json_path)?;
	let ps: ProducerAuthoritySchedule = serde_json::from_str(&schedule)?;

	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	let args = SaveProducerScheduleCall {
		ps,
		_runtime: PhantomData,
	};

	let proposal = client.encode(args)?;
	let call = create_sudo_call(&proposal);

	let block_hash = client.submit(call, &signer).await?;

	Ok(block_hash.to_string())
}

pub async fn test_random_nonce() -> Result<(), Box<dyn Error>> {
	let mut signer = PairSigner::new(AccountKeyring::Alice.pair());
	let dest = AccountKeyring::Bob.to_account_id().into();

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url("ws://127.0.0.1:9944").build().await?;

	let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
	println!("current nonce: {:?}", nonce);
//	signer.set_nonce(nonce + 3);

	client.submit(
		balances::TransferCall {
			to: &dest,
			amount: 1234 * 10u128.pow(12u32),
		},
		&signer,
	)
	.await?;
//	client.submit(
//		balances::TransferCall {
//			to: &dest,
//			amount: 2234 * 10u128.pow(12u32),
//		},
//		&signer,
//	)
//		.await?;

	// check that nonce is handled correctly
//	signer.increment_nonce();
//	let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
//	println!("after trade nonce: {:?}", nonce);
////	signer.set_nonce(nonce + 5);
//	client.submit(
//		balances::TransferCall {
//			to: &dest,
//			amount: 1000 * 10u128.pow(12u32),
//		},
//		&signer,
//	)
//	.await?;

//	for i in 0..7 {
//		let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
//		signer.set_nonce(nonce+i);
//		println!("after trade nonce: {:?}", nonce);
//		client.submit(
//			balances::TransferCall {
//				to: &dest,
//				amount: 500 * 10u128.pow(12u32),
//			},
//			&signer,
//		)
//		.await?;
////		signer.increment_nonce();
//	}

	Ok(())
}
