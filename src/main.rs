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

#![allow(dead_code)]

mod error_types;
mod producers_schedule;
mod voucher;
mod utils;
mod assets;
mod batch_trades;
mod scheduler;
mod balances;
// mod prove_action;

use std::error::Error;
use sp_runtime::AccountId32;
use crate::voucher::{issue_voucher_call, Reward, create_encoded_call};
use crate::batch_trades::batch_calls;
use crate::utils::{read_json_from_file, write_json_to_file};
use crate::error_types::Error as BifrostxtError;
use subxt::{PairSigner, DefaultNodeRuntime as BifrostRuntime, Client};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_core::crypto::Ss58Codec;

use std::{thread, time};

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IssueVEOS {
    who: AccountId32,
    balance: f64,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AllBNC {
    pub account: String,
	pub amount: u128,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // pub use hashing::{blake2_128, blake2_256, twox_64, twox_128, twox_256, keccak_256};
    let module_hash = sp_core::twox_128(b"Voucher");
    println!("module hash: {:?}", module_hash);
    println!("module hash: {:?}", hex::encode(module_hash));

    let storage_hash = sp_core::twox_128(b"BalancesVoucher");
    println!("module hash: {:?}", storage_hash);
    println!("module hash: {:?}", hex::encode(storage_hash));
    // let all_bnc_path = "/home/bifrost/jdeng/bifrost-xt/all_bncs.json";
    // let all_bnc_content = read_json_from_file(all_bnc_path)?;
    // let mut all_bnc: Vec<AllBNC> = serde_json::from_str(&all_bnc_content)?;

    // let rewards_path1 = "/home/bifrost/jdeng/bifrost-xt/diff1.json";
    // let json_content1 = read_json_from_file(rewards_path1)?;
    // let rewards1: Vec<Reward> = serde_json::from_str(&json_content1)?;

    // AccountId32::from_ss58check(&reward.account);
    // for r in rewards.iter() {
    //     match AccountId32::from_ss58check(&r.accunt) {
    //         Ok(acct) => {
    //             for j in rewards1.iter() {
    //                 let acc2 = AccountId32::from_ss58check(&j.account);
    //                 dbg!(&acct, &acc2);
    //                 if acct == acc2.unwrap() {
    //                     println!("they are the same!");
    //                 }
    //             }
    //         }
    //         Err(e) => println!("bad address due to: {:?}", e),
    //     }
    // }

    // 2021-03-02 21:36:24


    // let ss58 = "0x0C4364cdB4E09C051A5Ee40Ac713baFB0F4F138d";
    // let a1 = AccountId32::from_ss58check(ss58);
    // println!("{:?}", a1);

    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/11.json";
    // let json_content = read_json_from_file(rewards_path)?;
    // let rewards: Vec<Reward> = serde_json::from_str(&json_content)?;

    // let ss = r#"
    // {
    //     "ethAddr": "0xA5446d4aE751f5C4fB2BF475dbfB1dAbDf309D9f",
    //     "bifrostAddr": "1512npKjwqk7sD7issUJS97Pnw1FJcVqeqjBqj7DwQF1tTFV",
    //     "rewards": "3901659588683099205"
    // }"#;
    // let k: Result<BNCReward, _> = serde_json::from_str(&ss);
    // let n = k.unwrap().rewards.parse::<u128>().unwrap() / 10u128.pow(6);
    // dbg!(n);
    // lwt t = 3_901_659_588_683_099_205u128;
    // let _ = 3_901_659_588_683u128;
    // let url = "ws://150.109.194.40:9944";
    let url = "ws://10.115.27.96:9988";
    let signer = "//Alice";

    // let _ = crate::voucher::get_all_voucher(url).await?;

    // return Ok(());

    // let schedule_path = "/home/bifrost/jdeng/bifrost-xt/2021-02-26/2.8 DeFiGo ama 活动发奖.xlsx.json";
    // let block = producers_schedule::save_producer_schedule_call(signer, url, schedule_path).await;
    // dbg!(block);

    let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/8.18首期wiki翻译奖励补发.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/cctip 发奖.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/10.21海外 AMA 有奖问答.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/波卡之夜奖励.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/bifrost 抹茶中奖用户名单_2020-10-21(1)(1).json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/11.5 BML & Bifrost 直播活动发奖.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/11.5 BML & Bifrost 直播活动发奖-tcp-closed.json";
    let rewards_path = "/home/bifrost/jdeng/bifrost-xt/3.3 Polkawarriors 问答及Quiz 奖励.json";
    let json_content = read_json_from_file(rewards_path)?;
    let rewards: Vec<Reward> = serde_json::from_str(&json_content)?;

    let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
        .set_url(url)
        .skip_type_sizes_check()
        .build().await?;
    dbg!(2222);

    let batch_size = 8;
    // let times = rewards.len() / batch_size;
    let mut sum = 0.0f64;
    println!("{}", rewards.len());
    let mut unissued = vec![];
    for (index, _) in rewards.iter().step_by(batch_size).enumerate() {
        // break;
        let range = { 
            println!("{}", index);
            if rewards.len() - batch_size * index <= batch_size {
                println!("issued from {} to {}", batch_size * index, rewards.len());
                &rewards[batch_size * index..]
            } else {
                println!("issued from {} to {}", batch_size * index, batch_size * index + batch_size);
                &rewards[batch_size * index..batch_size * index + batch_size]
            }
        };

        let mut calls = vec![];
        
        let batch_rewards = range.iter().cloned();
        for reward in batch_rewards {
            println!("{:?}", reward);
            let who: Result<AccountId32, _> = AccountId32::from_ss58check(&reward.account);
            // if reward.rewards == "0" {
            //     println!("zero reward: {:?}", reward);
            //     continue;
            // }

            // let bnc = reward.amount.parse::<f64>()? / 10u128.pow(18) as f64;
            
            // let reward1 = Reward {
            //     account: reward.account.clone(),
	        //     amount: bnc,
            // };
            let reward1 = reward.clone();
            match who {
                Ok(account) => {
                    println!("who: {:?}", account);
                    let call = create_encoded_call(&client, &reward1, &account)?;
                    calls.push(call);
                    sum += reward1.amount;
                }
                Err(e) => {
                    unissued.push(reward.clone());
                    println!("failed to send this reward to bifrost: {:?}, due to: {:?}", reward1, e);
                }
            }
        }

        if calls.len() > batch_size {
            println!("Call size is two many: {:?}", calls.len());
            break;
        } else {
            println!("There will be {:?} calls sent", calls.len());
        }

        let trx_id = batch_calls(calls.into_iter(), &client, &signer).await?;
        println!("transaction id: {:?}", trx_id);

        let ten_millis = time::Duration::from_secs(2);

        thread::sleep(ten_millis);
    }
    println!("The whole BNC has been issued: {:?}", sum);

    let unissued_str = serde_json::to_string(&unissued)?;
    println!("unissued_str id: {:?}", unissued_str);

    Ok(())
}
