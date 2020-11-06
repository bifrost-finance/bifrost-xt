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
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded, Event, Store,
	sudo::{Sudo, SudoEventsDecoder, SudoCall}
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::{AccountId32, traits::{AtLeast32Bit, MaybeSerialize, Member}};
use std::error::Error;

#[subxt::module]
pub trait Voucher: System + Sudo {
	type Balance: Member
	+ AtLeast32Bit
	+ codec::Codec
	+ Default
	+ Copy
	+ MaybeSerialize
	+ std::fmt::Debug
	+ From<<Self as System>::BlockNumber>;
}

impl Voucher for BifrostRuntime {
	type Balance = u128;
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct IssueVoucherCall<'a, T: Voucher + Sudo> {
	pub dest: &'a <T as System>::Address,
	#[codec(compact)]
	pub amount: <T as Voucher>::Balance,
}

#[allow(dead_code)]
pub fn create_sudo_call<'a, T: Sudo>(call: &'a Encoded) -> SudoCall<T> {
	SudoCall {
		call,
		_runtime: PhantomData,
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct BalancesVoucherStore<'a, T: Voucher> {
	#[store(returns = T::Balance)]
	/// according account to get voucher
	pub account_id: &'a T::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct IssuedVoucherEvent<T: Voucher> {
	/// Account voucher was issued to.
	pub to: <T as System>::AccountId,
	/// Amount of voucher that was issued.
	pub amount: T::Balance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vouchers {
	pub amount: String,
	pub account: AccountId32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reward {
	pub account: String,
	pub amount: f64,
}

#[allow(dead_code)]
pub async fn issue_voucher_call(signer: &PairSigner::<BifrostRuntime, Pair>, client: &Client<BifrostRuntime>, reward: &Reward, who: &AccountId32) -> Result<String, Box<dyn Error>> {
	let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;

	let amount = {
		// let amount_f64 = reward.amount.parse::<f64>()?;
		let amount_f64 = reward.amount;
		(amount_f64 * 10f64.powi(12i32)) as u128
	};

	let args = IssueVoucherCall {
		dest: &who.clone().into(),
		amount,
	};
	let proposal = client.encode(args)?;
	let call = create_sudo_call(&proposal);

	let extrinsic = client.create_signed(call, signer).await?;

	let mut decoder = client.events_decoder::<IssueVoucherCall<BifrostRuntime>>();
	decoder.with_voucher();

	let voucher_events = client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
	let event = voucher_events.find_event::<IssuedVoucherEvent::<BifrostRuntime>>()?.ok_or("No Event found or decoded.")?;
	let block_hash = voucher_events.block;

	Ok(block_hash.to_string())
}

#[allow(dead_code)]
pub async fn get_voucher_by_account(signer: &str, url: &str, who: &AccountId32) -> Result<u128, Box<dyn std::error::Error>> {
	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	let voucher = client.balances_voucher(&who.clone().into(), None).await?;

	Ok(voucher)
}

#[allow(dead_code)]
pub async fn get_all_voucher(signer: &str, url: &str) -> Result<Vec<(AccountId32, u128)>, Box<dyn std::error::Error>> {
	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	// None means get all of the storage
	let mut iter = client.balances_voucher_iter(None).await?;

	let mut all_vouchers: Vec<(AccountId32, u128)> = vec![];
	while let Some((key, val)) = iter.next().await? {
		println!("{:?}: {}", key.0, val);
		println!("{:?}: {}", key.0.len(), val);
		let who = serde_json::from_slice(key.0.as_slice())?;
		let voucher = (who, val);
		all_vouchers.push(voucher);
	}

	Ok(all_vouchers)

}

#[allow(dead_code)]
pub fn create_encoded_call(client: &Client<BifrostRuntime>, reward: &Reward, who: &AccountId32) -> Result<Encoded, Box<dyn Error>> {
	let amount = {
		// let amount_f64 = reward.amount.parse::<f64>()?;
		let amount_f64 = reward.amount;
		(amount_f64 * 10f64.powi(12i32)) as u128
	};

	let args = IssueVoucherCall {
		dest: &who.clone().into(),
		amount,
	};
	let proposal = client.encode(args)?;
	let call = create_sudo_call(&proposal);
	let encoded_call = client.encode(call)?;

	Ok(encoded_call)
}