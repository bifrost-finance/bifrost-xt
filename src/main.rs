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
//mod batch_trades;

use codec::{Decode, Encode};
use std::error::Error;
use std::str::FromStr;
use std::{thread, time};
use sp_runtime::AccountId32;
use crate::voucher::{issue_voucher_call, Vouchers};

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IssueVEOS {
    who: AccountId32,
    balance: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let url = "ws://127.0.0.1:9944";
    let signer = "//Alice";

    let alice_voucher = r#"{
        "nickname": "jdeng",
        "u_id": 2,
        "amount": "10",
        "account": "gXCcrjjFX3RPyhHYgwZDmw8oe4JFpd5anko3nTY8VrmnJpe"
    }"#;

    let v: Vouchers =  serde_json::from_str(&alice_voucher)?;
    let block_hash = issue_voucher_call(signer, url, &v).await?;
    println!("block hash: {:?}", block_hash);

    let schedule = "/Volumes/Bifrost 1/my-repo/bifrost-xt/src/data/producer_authority_schedule_v2-55.json";

    let veos_issued = "/Users/liebi/my-repo/bifrost-peers-status/missed_trx_history_latest.json";
//    let json = utils::read_json_from_file(veos_issued)?;
//    let veos: Vec<IssueVEOS> = serde_json::from_str(&json)?;

//    let six_secs = time::Duration::from_secs(7);

//    let mut i = 0u32;
//    for v in veos.iter() {
//        if v.balance <= 0.0f64 {
//            continue
//        }
//        let to_be_issued = (v.balance * 10f64.powi(14i32)) as u128;
//
//        match assets::issue_assets(signer, url, &v.who, to_be_issued).await {
//            Ok(hash) => {
//                println!("{:?} reveived {:?} with hash: {:?}", &v.who.to_string(), to_be_issued, hash);
//                i += 1;
//            },
//            Err(e) => println!("{:?} didn't reveive {:?} with error: {:?}", &v.who.to_string(), to_be_issued, e),
//        }
//
//
//        thread::sleep(six_secs);
//    }
//    println!("sent {:?} trades.", i);

//    let target = AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
//    let r = assets::issue_assets(signer, url, target, 1020 * 10u128.pow(12u32)).await;
//    dbg!(r);
//    let block_hash = producers_schedule::save_producer_schedule_call(signer, url, schedule).await?;
//    let block_hash = producers_schedule::test_random_nonce().await?;
//    println!("block hash: {:?}", block_hash);

//    let client = batch_trades::create_client(
//        "wss://n1.testnet.liebi.com"
////        "wss://n2.testnet.liebi.com"
////        "wss://n3.testnet.liebi.com"
////        "ws://n4.testnet.liebi.com:9944"
////        "ws://n5.testnet.liebi.com:9944"
//    ).await;

//    let mut i: u32 = 0;
//    loop {
////        batch_trades::get_nonce(&client, "//bifrost-sudo").await?;
//        batch_trades::get_nonce1("wss://n1.testnet.liebi.com", "//bifrost-sudo").await?;
//        i += 1;
//        if (i >= 1000) { break; }
//    }

    Ok(())
}