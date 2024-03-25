pub use groth16_verifier_methods::GROTH16_VERIFIER_ELF;
use risc0_groth16_old::{
    circom::{CircomProof, CircomPublic, CircomVKey},
    Groth16,
};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

const VK: &str = include_str!("data/groth16/verification_key.json");
pub const PROOF: &str = include_str!("data/groth16/proof.json");
pub const PUBLIC: &str = include_str!("data/groth16/public.json");

pub fn get_groth16() -> Groth16 {
    let vk: CircomVKey = serde_json::from_str(VK).unwrap();
    let proof: CircomProof = serde_json::from_str(PROOF).unwrap();
    let public = CircomPublic {
        values: serde_json::from_str(PUBLIC).unwrap(),
    };
    Groth16::from_circom(vk, proof, public).unwrap()
}

pub fn verify_groth16(public3: &String, public4: &String, pi_a: Vec<String>, pi_b: Vec<Vec<String>>, pi_c: Vec<String>) -> (Receipt, [u8; 32]) {
    let mut envb = ExecutorEnv::builder();
    let proof = CircomProof {
        pi_a, pi_b, pi_c, curve: "bn128".to_string(), protocol: "groth16".to_string()
    };
    envb.write(&proof).unwrap();
    envb.write(&public3).unwrap();
    envb.write(&public4).unwrap();
    let env = envb.build().unwrap();
    let receipt = default_prover().prove(env, GROTH16_VERIFIER_ELF).unwrap();
    let output_digest: [u8; 32] = receipt.journal.decode().unwrap();
    (receipt, output_digest)
}
