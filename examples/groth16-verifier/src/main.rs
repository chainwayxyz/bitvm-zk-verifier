use groth16_verifier_methods::GROTH16_VERIFIER_ID;
use risc0_groth16_old::circom::CircomProof;
use risc0_groth16_verifier::{get_groth16, verify_groth16, PROOF, PUBLIC};

fn main() {
    let proof: CircomProof = serde_json::from_str(PROOF).unwrap();
    let publics: Vec<String> = serde_json::from_str(PUBLIC).unwrap();
    let public3 = &publics[2];
    let public4 = &publics[3];
    let (receipt, output) = verify_groth16(public3, public4, proof.pi_a, proof.pi_b, proof.pi_c);
    receipt.verify(GROTH16_VERIFIER_ID).unwrap();
    let groth16 = get_groth16();
    groth16.verify().unwrap();
    assert_eq!(output, groth16.digest());
    println!("Verification: OK!");
}
