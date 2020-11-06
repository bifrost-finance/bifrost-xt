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
// mod prove_action;

use std::error::Error;
use std::str::FromStr;
use sp_runtime::AccountId32;
use crate::voucher::{issue_voucher_call, Reward, create_encoded_call};
use crate::voucher::IssueVoucherCall;
use crate::batch_trades::batch_calls;
use crate::utils::read_json_from_file;
use crate::error_types::Error as BifrostxtError;
use subxt::{PairSigner, DefaultNodeRuntime as BifrostRuntime, Client, Encoded};
use sp_core::{sr25519::Pair, Pair as TraitPair};

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

    let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
	let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/8.18首期wiki翻译奖励补发.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/cctip 发奖.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/10.21海外 AMA 有奖问答.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/波卡之夜奖励.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/bifrost 抹茶中奖用户名单_2020-10-21(1)(1).json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/11.5 BML & Bifrost 直播活动发奖.json";
    // let rewards_path = "/home/bifrost/jdeng/bifrost-xt/11.5 BML & Bifrost 直播活动发奖-tcp-closed.json";
    let rewards_path = "/home/bifrost/jdeng/bifrost-xt/Asgard CC3 BNC 奖励记录.json";
    let json_content = read_json_from_file(rewards_path)?;
    let rewards: Vec<Reward> = serde_json::from_str(&json_content)?;

    let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

    let batch_size = 20usize;
    let times = rewards.len() / batch_size;
    let mut sum = 0.0f64;
    println!("{}", rewards.len());
    for (index, _) in rewards.iter().step_by(batch_size).enumerate() {
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
            let who: Result<AccountId32, _> = AccountId32::from_str(&reward.account);
            match who {
                Ok(account) => {
                    let call = create_encoded_call(&client, &reward, &account)?;
                    calls.push(call);
                    sum += reward.amount;
                }
                Err(e) => {
                    println!("failed to send this reward to bifrost: {:?}, due to: {:?}", reward, e);
                }
            }
        }
        let trx_id = batch_calls(calls.into_iter(), &client, &signer).await?;
        println!("transaction id: {:?}", trx_id);
    }
    println!("The whole BNC has been issued: {:?}", sum);


    Ok(())
}
