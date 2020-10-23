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

mod error_types;
mod producers_schedule;
mod voucher;
mod utils;
mod assets;
mod batch_trades;
mod scheduler;
mod balances;
mod prove_action;

use codec::{Decode, Encode};
use std::error::Error;
use std::str::FromStr;
use std::{thread, time};
use sp_runtime::AccountId32;
use crate::voucher::{issue_voucher_call, CC2Voucher};
use crate::voucher::IssueVoucherCall;
use crate::assets::{TokenSymbol, IssueCall};
use crate::batch_trades::batch_calls;

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IssueVEOS {
    who: AccountId32,
    balance: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let url = "wss://n1.testnet.liebi.com";
    let signer = "//Alice";

    // let url = "ws://127.0.0.1:9944";
    // let signer = "//Alice";

    let db_path = concat!("/home","/sled/cross-chain");

    let r = crate::prove_action::prove_action_call(db_path, signer, url).await;
    println!("error happened while submit transaction: {:?}", r);

    Ok(())
}
