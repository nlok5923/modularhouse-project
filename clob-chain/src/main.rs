use avail_subxt::{
    api::{
        self,
        runtime_types::{
            bounded_collections::bounded_vec::BoundedVec, da_control::pallet::Call as DaCall,
        },
    },
    avail::AppUncheckedExtrinsic,
    build_client,
    primitives::AvailExtrinsicParams,
    Call, Opts,
};
use fhe_clob::algo::AlgoRunner;
use std::fs::File;
use std::io::Read;

fn main() {
    // send data to avail first
    let ws = "wss://turing-rpc.avail.so/ws".to_string();
    let client = build_client(ws, false).await?;

    let file_path = "order.json";
    let mut file = File::open(file_path).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let contents_data = contents.into_bytes();
    let data_transfer = api::tx()
        .data_availability()
        .submit_data(BoundedVec(contents_data.clone()));
    let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1.into());

    println!("Sending example data...");
    let h = client
        .tx()
        .sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
        .await?
        .wait_for_finalized_success()
        .await?;

    let submitted_block = client.rpc().block(Some(h.block_hash())).await?.unwrap();

    println!("Submitted order data to avail");
    let algo_runner = AlgoRunner::new();
    algo_runner.run_bfv_clob_algo(contents);

    // send data to avail
}
