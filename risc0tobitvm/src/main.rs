use risc0tobitvm::{setup, prove};
use std::fs::{write, read_to_string};

// use hello_world::multiply;
use bitcoin_pow::{calculate_pow, CALCULATE_POW_ELF};

fn main() {
    let mut template = read_to_string("templates/constants_template.h").unwrap();
    setup(CALCULATE_POW_ELF, &mut template);
    // let (receipt, _) = multiply(101, 31);
    let (receipt, _) = calculate_pow();
    prove(&receipt, &mut template);
    write("groth16-verifier/constants.h", template).unwrap();
}
