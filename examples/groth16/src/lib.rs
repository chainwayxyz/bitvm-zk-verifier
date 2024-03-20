#![doc = include_str!("../README.md")]

pub use groth16_methods::VERIFY_GROTH16_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use risc0_groth16::{ProofJson, PublicInputsJson, Verifier, VerifyingKeyJson};

const VK: &str = include_str!("../data/vk.json");
const PROOF: &str = include_str!("../data/proof.json");
const PUBLIC: &str = include_str!("../data/public.json");

pub fn verify_groth16() -> (Receipt, u32) {
    let mut env = ExecutorEnv::builder();

    let a: u32 = 5;

    println!("groth16 verify start");
    let verifying_key: VerifyingKeyJson = serde_json::from_str(VK).unwrap();
    let proof: ProofJson = serde_json::from_str(PROOF).unwrap();
    let public_inputs = PublicInputsJson {
        values: serde_json::from_str(PUBLIC).unwrap(),
    };
    let verifier = Verifier::from_json(proof, public_inputs, verifying_key).unwrap();
    verifier.verify().unwrap();
    println!("groth16 verify done");

    env.write(&a).unwrap();
    let env = env.build().unwrap();

    let prover = default_prover();
    let receipt = prover.prove(env, VERIFY_GROTH16_ELF).unwrap();
    let c: u32 = receipt.journal.decode().expect("Journal output should deserialize into the same types (& order) that it was written");
    println!("Verify: {}", if c > 0 { "True" } else { "False" });
    (receipt, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_groth16() {
        let (_, result) = verify_groth16();
        assert_eq!(result, 1, "Verify False");
    }
}
