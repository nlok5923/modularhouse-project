use fhe_clob::algo::AlgoRunner;
use methods::{FHE_ORDERBOOK_ELF, FHE_ORDERBOOK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::Read;


fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let ws = "	wss://turing-rpc.avail.so/ws".to_string();

    // post batch data to avail

    let file_path = "order.json";
    let mut file = File::open(file_path).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let algo_runner = AlgoRunner::new();

    algo_runner.run_bfv_clob_algo(contents);

    // For example:
    let input: u32 = 15 * u32::pow(2, 27) + 1;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, FHE_ORDERBOOK_ELF).unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    println!("Generated receipt");
}
