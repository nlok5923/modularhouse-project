use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here
    let input: u32 = env::read();
    // it's a mock proof for now
    env::commit(&input);
}
