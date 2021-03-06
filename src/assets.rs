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

use codec::{Decode, Encode};
use crate::error_types::Error as BifrostxtError;
use core::marker::PhantomData;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::System, Encoded, sudo::{Sudo, SudoCall}, Error as SubxtErr,
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use sp_runtime::traits::{Member, AtLeast32Bit, MaybeSerialize};
use sp_runtime::AccountId32;

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum TokenSymbol {
	aUSD = 0,
	DOT = 1,
	vDOT = 2,
	KSM = 3,
	vKSM = 4,
	EOS = 5,
	vEOS = 6,
	IOST = 7,
	vIOST = 8,
}

impl Default for TokenSymbol {
	fn default() -> Self { Self::aUSD }
}

#[subxt::module]
pub trait Assets: System + Sudo {
	type Balance: Member
	+ AtLeast32Bit
	+ codec::Codec
	+ Default
	+ Copy
	+ MaybeSerialize
	+ std::fmt::Debug
	+ From<<Self as System>::BlockNumber>;
}

impl Assets for BifrostRuntime {
	type Balance = u128;
}

#[derive(Clone, Debug, PartialEq, Decode, Call, Encode)]
pub struct IssueCall<'a, T: Assets + Sudo> {
	pub token_symbol: TokenSymbol,
	pub target: &'a <T as System>::Address,
	#[codec(compact)]
	pub amount: <T as Assets>::Balance,
}

#[allow(dead_code)]
pub fn issue<'a, T: Sudo>(call: &'a Encoded) -> SudoCall<T> {
	SudoCall {
		call,
		_runtime: PhantomData,
	}
}

#[allow(dead_code)]
pub async fn issue_assets(
	signer: &str,
	url: &str,
	target: &AccountId32,
	amount: u128
) -> Result<String, Box<dyn Error>> {
	let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

	let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
		.set_url(url)
		.skip_type_sizes_check()
		.build().await?;

	let args = IssueCall {
		token_symbol: TokenSymbol::aUSD,
		target: &target.clone().into(),
		amount: amount.into(),
	};

	let proposal = client.encode(args)?;
	let call = issue(&proposal);

//	let extrinsic = client.watch(call, &signer).await?;
	let block_hash = client.submit(call, &signer).await.map_err(|e| {
		if let SubxtErr::Rpc(err) = e {
			if err.to_string().as_str().contains("Priority is too low") {
				println!("eee");
			}
		}
		"HHH"
	})?;
//	let block_hash = extrinsic.block.to_string();

	Ok(block_hash.to_string())
}
