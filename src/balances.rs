use crate::error_types::Error as BifrostxtError;
use subxt::{
    PairSigner, DefaultNodeRuntime as BifrostRuntime, Client,
    system::AccountStoreExt, balances,
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use sp_keyring::{AccountKeyring};
use sp_runtime::AccountId32;

#[allow(dead_code)]
pub async fn balance_transfer(signer: &str, url: &str, to: &AccountId32, i: u32) -> Result<String, Box<dyn Error>> {
    let client: Client<BifrostRuntime> = subxt::ClientBuilder::new().set_url(url).build().await?;

    let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BifrostxtError::WrongSudoSeed)?;
    let mut signer = PairSigner::<BifrostRuntime, Pair>::new(signer);

    let nonce = client.account(&signer.signer().public().into(), None).await?.nonce;
    println!("current nonce: {:?}", nonce);
    signer.set_nonce(i);

    let call = balances::TransferCall {
        to: &to.clone().into(),
        amount: 1,
    };

    let trx_id = client.submit(call, &signer).await?;

    Ok(trx_id.to_string())
}
