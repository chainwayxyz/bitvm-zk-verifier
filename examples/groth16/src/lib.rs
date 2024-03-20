#![doc = include_str!("../README.md")]

pub use groth16_methods::VERIFY_GROTH16_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

pub fn verify_groth16() -> (Receipt, u32) {
    let mut env = ExecutorEnv::builder();
    let a: u32 = 5;
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
