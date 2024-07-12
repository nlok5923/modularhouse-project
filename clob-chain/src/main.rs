use fhe_clob::algo::AlgoRunner;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_path = "order.json";
    let mut file = File::open(file_path).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let algo_runner = AlgoRunner::new();

    algo_runner.run_bfv_clob_algo(contents);

    // send data to avail 
    
}
