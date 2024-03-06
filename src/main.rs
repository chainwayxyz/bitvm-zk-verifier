use hex::FromHex;
use num_bigint::BigUint;
use risc0_groth16::verifier::prepared_verifying_key;
use risc0_groth16::PublicInputsJson;
use risc0_groth16::{to_json, ProofJson, Verifier};
use risc0_zkvm::sha::{Digest, Digestible};
use risc0_zkvm::{get_prover_server, recursion::identity_p254, ProverOpts};
use serde::Deserialize;
use serde_json::from_str;
use sha2::Digest as OtherDigest;
use sha2::Sha256;
use std::fs;
use std::str::FromStr;
use std::{fs::File, io::Cursor, path::Path};

use hello_world::multiply;
// use bitcoin_pow::calculate_pow;

/// Merkle root of the RECURSION_CONTROL_IDS
pub const ALLOWED_IDS_ROOT: &str =
    "6df708447638d36828ebf4545980ff39315562181c926d3a9e2697405f3acf15";

pub fn split_digest_custom(d: Digest) -> (u128, u128) {
    let big_endian: Vec<u8> = d.as_bytes().to_vec().iter().rev().cloned().collect();
    let middle = big_endian.len() / 2;
    let (b, a) = big_endian.split_at(middle);
    (
        u128::from_be_bytes(a.try_into().unwrap()),
        u128::from_be_bytes(b.try_into().unwrap()),
    )
}

pub fn c_print(variable_name: &str, bytes: &[u8]) {
    print!("const unsigned char {}[] = {{", variable_name);
    for (i, byte) in bytes.iter().enumerate() {
        print!("{}", byte);
        if i < bytes.len() - 1 {
            print!(", ");
        }
    }
    println!("}};");
}

pub fn c_print2(variable_name: &str, bytes: &[u8]) -> String {
    format!("const unsigned char {variable_name}[] = {{{b}}};", b = bytes.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", "))
}

macro_rules! sha256_hash {
    ($($data:expr),+) => {{
        let mut hasher = Sha256::new();
        $(
            hasher.update($data);
        )+
        let result: [u8; 32] = hasher.finalize().try_into().expect("SHA256 should produce a 32-byte output");
        result
    }};
}

/// Groth16 Proof encoded as JSON.
#[derive(Deserialize, Debug)]
pub struct PublicProofJson {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: Option<String>,
    pub curve: Option<String>,
}
fn main() {
    let (receipt, _) = multiply(101, 97);
    // let (receipt, _) = calculate_pow();
    let claim = receipt.get_claim().unwrap();

    let opts = ProverOpts::default();
    // let ctx = VerifierContext::default();
    let prover = get_prover_server(&opts).unwrap();
    let composite_receipt = receipt.inner.composite().unwrap();
    let succinct_receipt = prover.compress(composite_receipt).unwrap();
    // let journal = session.journal.unwrap().bytes;
    let ident_receipt = identity_p254(&succinct_receipt).unwrap();
    let seal_bytes = ident_receipt.get_seal_bytes();
    let seal_json = File::create(Path::new("./work_dir").join("input.json")).unwrap();
    let mut seal_reader = Cursor::new(&seal_bytes);
    to_json(&mut seal_reader, &seal_json).unwrap();
    c_print(
        "RECEIPT_CLAIM_TAG",
        &sha256_hash!("risc0.ReceiptClaim".as_bytes()),
    );
    println!("test: {:?}", c_print2("CLAIM_INPUT", &claim.input.as_bytes()));
    c_print("CLAIM_INPUT", &claim.input.as_bytes());
    c_print("CLAIM_PRE", &claim.pre.digest().as_bytes());
    c_print("CLAIM_POST", &claim.post.digest().as_bytes());
    c_print("OUTPUT_TAG", &sha256_hash!("risc0.Output".as_bytes()));
    c_print("JOURNAL", &receipt.journal.bytes);
    c_print("ZEROS", &[0u8; 32]);
    c_print("TWO_U16", &2u16.to_le_bytes());
    c_print("FOUR_U16", &4u16.to_le_bytes());
    c_print("ZERO_U32", &0u32.to_le_bytes());
    let (a0, a1) = split_digest_custom(Digest::from_hex(ALLOWED_IDS_ROOT).unwrap()); // This part is constant

    // println!("const char* public_input_1 = {:?};", a0);
    // println!("const char* public_input_2 = {:?};", a1);
    c_print("FIRST_PUBLIC_INPUT", &a0.to_be_bytes());
    c_print("SECOND_PUBLIC_INPUT", &a1.to_be_bytes());

    let claim_digest = sha256_hash!(
        sha256_hash!("risc0.ReceiptClaim".as_bytes()),
        claim.input,
        claim.pre.digest(),
        claim.post.digest(),
        sha256_hash!(
            sha256_hash!("risc0.Output".as_bytes()),
            sha256_hash!(&receipt.journal.bytes),
            [0u8; 32], // Assumptions' digest
            2u16.to_le_bytes()
        ),
        0u32.to_le_bytes(),
        0u32.to_le_bytes(),
        4u16.to_le_bytes()
    );
    let (c0, c1) = split_digest_custom(claim_digest.into());
    c_print("EXPECTED_THIRD_PUBLIC_INPUT", &c0.to_be_bytes());
    c_print("EXPECTED_FOURTH_PUBLIC_INPUT", &c1.to_be_bytes());

    // Test to verify
    let public_inputs: PublicInputsJson = PublicInputsJson {
        values: vec![
            a0.to_string(),
            a1.to_string(),
            c0.to_string(),
            c1.to_string(),
        ],
    };
    println!("Public inputs: {:?}", public_inputs);
    let proof: ProofJson = from_str(&fs::read_to_string("./work_dir/proof.json").unwrap()).unwrap();

    let verifier = Verifier::new(
        &proof.try_into().unwrap(),
        public_inputs.to_scalar().unwrap(),
        prepared_verifying_key().unwrap(),
    )
    .unwrap();

    let verification_result = verifier.verify();

    if verification_result.is_err() {
        println!("Groth16 input is written to work_dir/input.json");
        println!("run:\nsudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover");
    }

    let proof: PublicProofJson =
        from_str(&fs::read_to_string("./work_dir/proof.json").unwrap()).unwrap();
    c_print(
        "PROOF_PI_A0",
        &BigUint::from_str(&proof.pi_a[0]).unwrap().to_bytes_be(),
    );
}
