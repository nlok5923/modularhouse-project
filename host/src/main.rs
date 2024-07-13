use actix_web::Error;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, http::StatusCode};
use fhe_clob::algo::AlgoRunner;
use methods::{FHE_ORDERBOOK_ELF, FHE_ORDERBOOK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use bonsai_sdk::alpha as bonsai_sdk;
use hex;
use std::time::{Duration};
use serde::{Deserialize, Serialize};
use risc0_zkvm::{compute_image_id, serde::to_vec, Receipt};



#[get("/gen_proof")]
async fn generate_proof() -> Result<HttpResponse, Error> {
    println!("Received Proof generation request");

    // For example:
    let input: u32 = 15 * u32::pow(2, 27) + 1;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover.prove(env, FHE_ORDERBOOK_ELF).unwrap();

    let receipt = prove_info.receipt;
    println!("Proof Receipt: {:?}", receipt);
    println!("Proof Generated");
    return Ok(HttpResponse::build(StatusCode::OK).json(
        serde_json::json!({
            "Success": true,
        }),    
    ));
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProofInput {
    input: [Vec<String>; 1],
    signal: [String; 1]
}

pub fn convert_proof_to_stark(pr_updated: [Vec<String>; 1], signal: [String; 1]) -> Receipt {
    println!("Converter: {:?}", pr_updated);

    let proof_input = ProofInput {
        input: pr_updated,
        signal
    };

    // to run recursive proving locally

    // let env = ExecutorEnv::builder()
    // .write(&proof_input)
    // .unwrap()
    // .build()
    // .unwrap();

    // let receipt = default_prover().prove(env, VERIFIER_ELF).unwrap();
    // receipt.verify(VERIFIER_ID).unwrap();
    // receipt

    // recursive proving outsourced using bonsai
    let url = "https://api.bonsai.xyz/".to_string();
    let api_key = "API_KEY".to_string();
    let client = bonsai_sdk::Client::from_parts(url, api_key, risc0_zkvm::VERSION)
        .expect("Failed to construct sdk client");
    println!("Reached here");

    let image_id = hex::encode(compute_image_id(FHE_ORDERBOOK_ELF).unwrap());
    println!("Image ID done: {}", image_id);
    client.upload_img(&image_id, FHE_ORDERBOOK_ELF.to_vec()).unwrap();

    println!("Image ID: {}", image_id);

    let input_data = to_vec(&proof_input).unwrap();
    let input_data = bytemuck::cast_slice(&input_data).to_vec();
    let input_id = client.upload_input(input_data).unwrap();
    let final_receipt: Receipt;

    let assumptions: Vec<String> = vec![];

    let proving_and_conversion_start_time = Instant::now();

    let session = client.create_session(image_id, input_id, assumptions).unwrap();
    loop {
        let res = session.status(&client).unwrap();
        if res.status == "RUNNING" {
            eprintln!(
                "Current status: {} - state: {} - continue polling...",
                res.status,
                res.state.unwrap_or_default()
            );
            std::thread::sleep(Duration::from_secs(15));
            continue;
        }
        if res.status == "SUCCEEDED" {
            // Download the receipt, containing the output
            let receipt_url = res
                .receipt_url
                .expect("API error, missing receipt on completed session");

            let receipt_buf = client.download(&receipt_url).unwrap();
            let receipt: Receipt = bincode::deserialize(&receipt_buf).unwrap();
            final_receipt = receipt.clone();

            // code to serialize and save
            // let mut file = File::create("stark.bin").unwrap();
            // let mut another_file = File::create("stark2.txt").unwrap();
            
            // Write the byte data to the file
            // file.write_all(&receipt_buf).unwrap();
            // another_file.write_all(&receipt_buf).unwrap();
            // println!("Journal: {:?}", receipt.journal);

            receipt
                .verify(FHE_ORDERBOOK_ID)
                .expect("Receipt verification failed");
        } else {
            panic!(
                "Workflow exited: {} - | err: {}",
                res.status,
                res.error_msg.unwrap_or_default()
            );
        }

        break;
    }

    // let snark_session = client.create_snark(session.uuid).unwrap();
    // eprintln!("Created snark session: {}", snark_session.uuid);

    // let mut snark_receipt_resp: SnarkReceipt;
    // loop {
    //     let res: bonsai_sdk::responses::SnarkStatusRes = snark_session.status(&client).unwrap();
    //     match res.status.as_str() {
    //         "RUNNING" => {
    //             eprintln!("Current status: {} - continue polling...", res.status,);
    //             std::thread::sleep(Duration::from_secs(15));
    //             continue;
    //         }
    //         "SUCCEEDED" => {
    //             let snark_receipt = res.output;
    //             eprintln!("Snark proof!: {snark_receipt:?}");
    //             snark_receipt_resp = snark_receipt.unwrap();
    //             // return snark_receipt?;
    //             break;
    //         }
    //         _ => {
    //             panic!(
    //                 "Workflow exited: {} err: {}",
    //                 res.status,
    //                 res.error_msg.unwrap_or_default()
    //             );
    //         }
    //     }
    // }

    let proving_and_conversion_end_time = Instant::now();
    let elapsed_time = proving_and_conversion_end_time.duration_since(proving_and_conversion_start_time);
    println!(
        "Time for recursive proving of zkevm proof {:?}",
        elapsed_time.as_secs_f64()
    );

    final_receipt
}

#[actix_web::main]
async fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    println!("Running prover on 8081");

    HttpServer::new(|| App::new().service(generate_proof))
        .bind(("127.0.0.1", 8081))
        .unwrap()
        .run()
        .await;

}
