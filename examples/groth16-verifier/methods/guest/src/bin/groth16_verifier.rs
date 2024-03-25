#![no_main]

use risc0_groth16_old::{circom::{CircomProof, CircomPublic, CircomVKey}, Groth16};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

const VK: &str = include_str!("../../../../src/data/groth16/verification_key.json");
const PUBLIC1: &str = "77092634684695146210658225760653342573";
const PUBLIC2: &str = "28989777287622164098132409305706681649";

pub fn main() {
    let vk: CircomVKey = serde_json::from_str(VK).unwrap();
    let proof: CircomProof = env::read();
    let public3: String = env::read();
    let public4: String = env::read();
    let public = CircomPublic {
        values: vec![PUBLIC1.to_string(), PUBLIC2.to_string(), public3, public4],
    };

    let groth16 = Groth16::from_circom(vk, proof, public).unwrap();
    groth16.verify().unwrap();

    env::commit(&groth16.digest());
}
