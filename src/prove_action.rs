use codec::{Decode, Encode};
use core::marker::PhantomData;
use crate::error_types::Error as BifrostXTErr;
use eos_chain::{
    Action, ActionReceipt, Checksum256, Digest, IncrementalMerkle,
    ProducerAuthoritySchedule, Signature, SignedBlockHeader
};
use serde::{Deserialize, Serialize};
use subxt::{
    PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
    system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded, Event, Store,
    sudo::{Sudo, SudoEventsDecoder, SudoCall}
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::{thread, time};

#[subxt::module]
pub trait BridgeEos: System {}

impl BridgeEos for BifrostRuntime {}

#[derive(Clone, Copy, Debug, Decode, Encode, Deserialize, Serialize)]
pub enum Status {
    Undone,
    Done,
}

#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ChangeScheduleArgs {
    legacy_schedule_hash: Checksum256,
    schedule:             ProducerAuthoritySchedule,
    merkle:               IncrementalMerkle,
    block_headers:        Vec<SignedBlockHeader>,
    block_ids_list:       Vec<Vec<Checksum256>>,
}

#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ProveActionArgs {
    action:               Action,
    action_receipt:       ActionReceipt,
    action_merkle_paths:  Vec<Checksum256>,
    merkle:               IncrementalMerkle,
    block_headers:        Vec<SignedBlockHeader>,
    block_ids_list:       Vec<Vec<Checksum256>>,
    trx_id:               Checksum256,
    block_id:             Option<String>, //this is block id from bifrost node if this transaction is submitted
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DepositEvent<T: BridgeEos> {
    /// Account voucher was issued to.
    pub from: Vec<u8>,
    /// Amount of voucher that was issued.
    pub to: <T as System>::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DepositFailEvent<T: BridgeEos> {
    pub place_holder:         PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ChangeScheduleCall<T: BridgeEos> {
    legacy_schedule_hash: Checksum256,
    schedule:             ProducerAuthoritySchedule,
    merkle:               IncrementalMerkle,
    block_headers:        Vec<SignedBlockHeader>,
    block_ids_list:       Vec<Vec<Checksum256>>,
    pub _runtime:         PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ProveActionCall<T: BridgeEos> {
    action:               Action,
    action_receipt:       ActionReceipt,
    action_merkle_paths:  Vec<Checksum256>,
    merkle:               IncrementalMerkle,
    block_headers:        Vec<SignedBlockHeader>,
    block_ids_list:       Vec<Vec<Checksum256>>,
    trx_id:               Checksum256,
    pub _runtime:         PhantomData<T>,
}

#[allow(dead_code)]
pub async fn prove_action_call(db_path: &str, signer: &str, url: &str) -> Result<(), BifrostXTErr> {
    let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
        .set_url(url)
        .build()
        .await
        .map_err(|_| BifrostXTErr::SubxtError("failed to create client"))?;

    let signer = Pair::from_string(signer.as_ref(), None)
        .map_err(|_| BifrostXTErr::WrongSudoSeed)?;
    let signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

    let on_sec = time::Duration::from_secs(1);

    loop {
        thread::sleep(on_sec);
        let tree = sled::open(db_path);
        if tree.is_err() {
            println!("db is open by bifrost-eos-relay node");
            continue;
        }
        let tree = tree.unwrap();
        for item in tree.iter() {
            if let Ok((key, val)) = item {
                println!("index: {:?}", String::from_utf8(key.as_ref().to_vec()), );
                let mut call_args: ProveActionArgs = serde_json::from_str(&String::from_utf8_lossy(val.as_ref()))
                    .map_err(|e| {
                        BifrostXTErr::SubxtError("failed to deserialzie prove action args")
                    })?;

                // it means that this transaction has been sent
                if call_args.block_id.is_some() {
                    continue;
                }

                let call = ProveActionCall::<BifrostRuntime> {
                    action: call_args.action.clone(),
                    action_receipt: call_args.action_receipt.clone(),
                    action_merkle_paths: call_args.action_merkle_paths.clone(),
                    merkle: call_args.merkle.clone(),
                    block_headers: call_args.block_headers.clone(),
                    block_ids_list: call_args.block_ids_list.clone(),
                    trx_id: call_args.trx_id.clone(),
                    _runtime: PhantomData
                };

                let extrinsic = client
                    .create_signed(call, &signer)
                    .await
                    .map_err(|_| BifrostXTErr::SubxtError("failed to create extrinsic"))?;

                let mut decoder = client.events_decoder::<ProveActionCall<BifrostRuntime>>();
                decoder.with_bridge_eos();

                let bridge_eos_events = match client.submit_and_watch_extrinsic(extrinsic, decoder).await {
                    Ok(event) => event,
                    Err(e) => {
                        if e.to_string().as_str().contains("DuplicatedCrossChainTransaction") {
                            call_args.block_id = Some("DuplicatedCrossChainTransaction".to_string());

                            let call_args_str = serde_json::to_vec(&call_args)
                                .map_err(|_| BifrostXTErr::SubxtError("failed to serialize extrinsic"))?;
                            tree.insert(key, call_args_str);
                        }
                        continue;
                        unreachable!();
                    }
                };
                let event = bridge_eos_events
                    .find_event::<DepositEvent::<BifrostRuntime>>()
                    .map_err(|_| BifrostXTErr::SubxtError("No Event found or decoded"))?;
                let block_hash = bridge_eos_events.block;
                println!("block id: {:?}", block_hash);

                call_args.block_id = Some(block_hash.to_string());
                let call_args_str = serde_json::to_vec(&call_args)
                    .map_err(|_| BifrostXTErr::SubxtError("failed to serialize extrinsic"))?;

                // update status
                tree.insert(key, call_args_str);
            }
        }
    }

    Ok(())
}
