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
pub trait Utility: System {
//	type Call: From<frame_system::Call<Self as System>>;
//	type Call: Parameter + Default + Codec + Send + Sync + 'static;
//	type Call: Dispatchable + Codec;
}

impl Utility for BifrostRuntime {
//	type Call = ();
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BatchCall<T: Utility> {
//	pub calls: Vec<Box<dyn Call<BifrostRuntime>>>,
	pub calls: Vec<Encoded>,
	pub _runtime: PhantomData<T>,
}

//pub async fn batch_calls(calls: Vec<impl Call<BifrostRuntime>>) -> Result<String, Box<dyn Error>> {
pub async fn batch_calls(calls: impl IntoIterator<Item=impl Call<BifrostRuntime>>, url: &str, signer: &str) -> Result<String, Box<dyn Error>> {
	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	let encodeds = calls.into_iter().map(|call| client.encode(call).unwrap()).collect();

	let batch_call = BatchCall::<BifrostRuntime> {
		calls: encodeds,
		_runtime: PhantomData,
	};

	let block_hash = client.submit(batch_call, &signer).await?;

	Ok(block_hash.to_string())
}


//#[allow(dead_code)]
//pub async fn save_producer_schedule_call(signer: &str, url: &str, json_path: &str) -> Result<String, Box<dyn Error>> {
//	let schedule = read_json_from_file(json_path)?;
//	let ps: ProducerAuthoritySchedule = serde_json::from_str(&schedule)?;
//
//	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
//	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);
//
//	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;
//
//	let args = SaveProducerScheduleCall {
//		ps,
//		_runtime: PhantomData,
//	};
//
//	let proposal = client.encode(args)?;
//	let call = create_sudo_call(&proposal);
//
//	let block_hash = client.submit(call, &signer).await?;
//
//	Ok(block_hash.to_string())
//}
//
//pub async fn get_nonce(client: &Client<BifrostRuntime>, signer: &str) -> Result<(), Box<dyn Error>> {
//	let signer = Pair::from_string(signer, None).map_err(|_| "failed to create signer")?;
//	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);
//
//	let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
//	println!("current nonce: {:?}", nonce);
//
//	Ok(())
//}

//pub async fn get_nonce1(url: &str, signer: &str) -> Result<(), Box<dyn Error>> {
//	let signer = Pair::from_string(signer, None).map_err(|_| "failed to create signer")?;
//	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);
//
//	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
//	  .set_url(url)
//	  .build()
//	  .await.unwrap();
////	let client = global_client(url).as_ref().unwrap();
//
//	let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
//	println!("current nonce: {:?}", nonce);
//
//	Ok(())
//}
//
//pub async fn create_client(ws: &str) -> Client<BifrostRuntime> {
//	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(ws).build().await.unwrap();
//
//	client
//}
//
//fn global_client(url: &str) -> Option<&'static subxt::Client<BifrostRuntime>> {
//	static INSTANCE: OnceCell<Option<subxt::Client<BifrostRuntime>>> = OnceCell::new();
//	INSTANCE.get_or_init(|| {
//		println!("creating client");
//		let builder: Option<subxt::Client<BifrostRuntime>> = futures::executor::block_on(async {
//			subxt::ClientBuilder::new().set_url(url).build().await.ok()
//		});
//		builder
//	})
//}
