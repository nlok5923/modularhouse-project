use actix_web::Error;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, http::StatusCode};
use fhe_clob::algo::AlgoRunner;
use methods::{FHE_ORDERBOOK_ELF, FHE_ORDERBOOK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::Read;

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
