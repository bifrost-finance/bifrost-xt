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
	system::{System, SystemEventsDecoder}, Encoded, Event,
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

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct IssuedVoucherEvent<T: Voucher> {
	/// Account voucher was issued to.
	pub to: <T as System>::AccountId,
	/// Amount of voucher that was issued.
	pub amount: T::Balance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vouchers {
	#[serde(skip)]
	pub nickname: String,
	#[serde(skip)]
	pub u_id: u32,
	pub amount: String,
	pub account: AccountId32,
}

#[allow(dead_code)]
pub async fn issue_voucher_call(signer: &str, url: &str, voucher: &Vouchers) -> Result<String, Box<dyn Error>> {
	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

	let signer = Pair::from_string(signer, None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let amount = {
		let amount_f64 = voucher.amount.parse::<f64>()?;
		(amount_f64 * 10f64.powi(12i32)) as u128
	};

	let args = IssueVoucherCall {
		dest: &voucher.account.clone().into(),
		amount,
	};
	let proposal = client.encode(args)?;
	let call = create_sudo_call(&proposal);

	let extrinsic = client.create_signed(call, &signer).await?;

	let mut decoder = client.events_decoder::<IssueVoucherCall<BifrostRuntime>>();
	decoder.with_voucher();

	let voucher_events = client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
	let event = voucher_events.find_event::<IssuedVoucherEvent::<BifrostRuntime>>()?.ok_or("No Event found or decoded.")?;
	println!("event: {:?}", event);
	let block_hash = voucher_events.block;

	Ok(block_hash.to_string())
}
