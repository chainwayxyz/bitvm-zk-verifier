#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let a: u32 = env::read();
    if a != 5 {
        panic!("a is not 5")
    }
    let verify_result = 1;
    env::commit(&verify_result);
}
