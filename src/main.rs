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

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let url = "ws://127.0.0.1:9944";
    let signer = "//Alice";
    let schedule = "/Volumes/Bifrost/my-repo/bifrost-xt/src/data/producer_authority_schedule_v2.json";
    let block_hash = producers_schedule::save_producer_schedule_call(signer, url, schedule).await?;
    println!("block hash: {:?}", block_hash);

    Ok(())
}
