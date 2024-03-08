use risc0tobitvm::risc0_test;

// use hello_world::multiply;
use bitcoin_pow::calculate_pow;

fn main() {
    // let (receipt, _) = multiply(101, 31);
    let (receipt, _) = calculate_pow();
    risc0_test(&receipt);
}
