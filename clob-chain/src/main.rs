use fhe_clob::algo::AlgoRunner;
use std::fs::File;
use std::io::Read;
// use avail_subxt::{
//     api, api::data_availability::calls::types::SubmitData, rpc::KateRpcClient, AvailClient,
//     BoundedVec,
// };
// use avail_subxt::api::runtime_types::avail_core::header::extension::v3;
// use avail_subxt::api::runtime_types::avail_core::header::extension::HeaderExtension;
// use avail_subxt::AvailConfig;
// use avail_core::{AppId as AvailAppID};
// use subxt::{
//     ext::sp_core::sr25519::Pair,
//     ext::sp_core::Pair as PairT,
//     ext::sp_core::H256 as AvailH256,
//     tx::{PairSigner, Payload},
// };
use anyhow::{anyhow, Context, Error};
use reqwest;
use std::str::FromStr;
use tokio::*;

// async fn send_tx(
//     tx: Payload<SubmitData>,
//     signer: &PairSigner<AvailConfig, Pair>,
//     client: &AvailClient,
// ) -> Result<(AvailH256, u32), Error> {
//     let nonce = client
//         .legacy_rpc()
//         .system_account_next_index(signer.account_id())
//         .await.unwrap();

//     let e_event = client
//         .tx()
//         .create_signed_with_nonce(
//             &tx,
//             signer,
//             nonce,
//             avail_subxt::primitives::new_params_from_app_id(AvailAppID(self.app_id.0)),
//         )?
//         .submit_and_watch()
//         .await
//         .context("Submission failed")
//         .unwrap()
//         .wait_for_finalized_success()
//         .await
//         .context("Waiting for success failed")
//         .unwrap();
//     let block_hash = e_event.block_hash();
//     let extrinsic_hash = e_event.extrinsic_index();
//     Ok((block_hash, extrinsic_hash))
// }

#[tokio::main]
async fn main() {
    // send data to avail first
    let ws = "wss://turing-rpc.avail.so/ws".to_string();
    // let client = build_client(ws, false).await?;

    let file_path = "order.json";
    let mut file = File::open(file_path).expect("File not found");

    let mut order_contents = String::new();
    file.read_to_string(&mut order_contents)
        .expect("Failed to read file");

    // let client = AvailClient::new(ws).await.unwrap();
    // let mnemonic = "bulk impact process private orange motion roof force clean recall filter secret";
    // let app_id = 0;

    // let sender = PairT::from_string_with_seed(mnemonic, None).unwrap();
    // let signer = PairSigner::<AvailConfig, Pair>::new(sender.0);

    // let data = BoundedVec(contents.into_bytes());
    // let call = api::tx().data_availability().submit_data(data);

    // let (block_hash, transaction_index) = send_tx(call, &signer, &client).await;
    let route = "http://localhost:8080/submit_data";
    let body = reqwest::get(route)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Submitted order data to avail");

    println!("Running algo!!");
    let algo_runner = AlgoRunner::new();
    algo_runner.run_bfv_clob_algo(order_contents);
}
