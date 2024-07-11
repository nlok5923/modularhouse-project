use bfv::*;
use operators::*;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Orders {
    pair: String,
    buy_orders: Vec<u64>,
    sell_orders: Vec<u64>,
}

pub struct AlgoRunner;

impl AlgoRunner {
    pub fn new() -> AlgoRunner {
        AlgoRunner
    }

    pub fn run_bfv_clob_algo() {
        println!("Running bfv clob algo");

        // plaintext modulus
        let t = 65537;

        // no of slots
        let slots = 1 << 4;

        println!("slots: {}", slots);

        let mut rng = thread_rng();

        let mut params = BfvParameters::new(&[60; 10], t, slots);

        // P - 180 bits
        params.enable_hybrid_key_switching(&[60; 3]);

        // generate secret key
        let sk = SecretKey::random_with_params(&params, &mut rng);

        // Create evaluator to evaluate arithmetic operarions
        let evaluator = Evaluator::new(params);

        let ek = EvaluationKey::new(evaluator.params(), &sk, &[0], &[0], &[1], &mut rng);

        // Open and read the file containing the order
        let file_path = "order.json";
        let mut file = File::open(file_path).expect("File not found");

        // Read the contents of the file into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
    }
}
