use risc0tobitvm::{setup, prove};
use std::fs::{write, read_to_string};

// use hello_world::multiply;
// use bitcoin_pow::{calculate_pow, CALCULATE_POW_ELF};
use groth16::{verify_groth16, VERIFY_GROTH16_ELF};

fn main() {
    let mut template = read_to_string("templates/constants_template.h").unwrap();
    setup(VERIFY_GROTH16_ELF, &mut template);
    // let (receipt, _) = multiply(101, 31);
    let (receipt, _) = verify_groth16();
    prove(&receipt, &mut template);
    write("groth16-verifier/constants.h", template).unwrap();
}
