use risc0_zkvm::guest::env;
use clob_chain::algo::AlgoRunner;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let algo_runnner = AlgoRunner::new();

    algo_runnner.run_bfv_clob_algo();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
