use risc0_zkvm::Receipt;
use risc0tobitvm::{setup, prove};
use serde_json::Value;
use std::fs::{write, read_to_string};

use hello_world::{multiply, MULTIPLY_ELF};
fn hello(template: &mut String) -> Receipt {
    setup(MULTIPLY_ELF, template);
    let (receipt, _) = multiply(17, 23);
    receipt
}

use bitcoin_pow::{calculate_pow, CALCULATE_POW_ELF};
fn bitcoin(template: &mut String) -> Receipt {
    setup(CALCULATE_POW_ELF, template);
    let (receipt, _) = calculate_pow();
    receipt
}

use risc0_groth16_verifier::{GROTH16_VERIFIER_ELF, verify_groth16, PROOF, PUBLIC};
fn groth16(template: &mut String) -> Receipt {
    setup(GROTH16_VERIFIER_ELF, template);
    let proof_value: Value = serde_json::from_str(PROOF).unwrap();
    let proof_json = proof_value.as_object().unwrap();
    let publics: Vec<String> = serde_json::from_str(PUBLIC).unwrap();
    let public3 = &publics[2];
    let public4 = &publics[3];
    let pi_a: Vec<String> = serde_json::from_value(proof_json.get("pi_a").unwrap().clone()).unwrap();
    let pi_b: Vec<Vec<String>> = serde_json::from_value(proof_json.get("pi_b").unwrap().clone()).unwrap();
    let pi_c: Vec<String> = serde_json::from_value(proof_json.get("pi_c").unwrap().clone()).unwrap();
    let (receipt, _) = verify_groth16(public3, public4, pi_a, pi_b, pi_c);
    receipt
}

fn main() {
    let mut template = read_to_string("templates/constants_template.h").unwrap();
    // let receipt = bitcoin(&mut template);
    let receipt = groth16(&mut template);
    prove(&receipt, &mut template);
    write("groth16-verifier/constants.h", template).unwrap();
}
