use bitcoin_circuit::{GUEST_ELF, GUEST_ID};
use circuit_helpers::bitcoin::validate_threshold_and_add_work;

use circuit_helpers::bitcoin::BlockHeader;
use circuit_helpers::double_sha256_hash;
use circuit_helpers::sha256_hash;
use crypto_bigint::{Encoding, U256};
use end2endbitvm::data::{ALLOWED_IDS_ROOT, BLOCK_HEADERS};
use hello_world::multiply;
use hex::FromHex;
use risc0_groth16::verifier::prepared_verifying_key;
use risc0_groth16::PublicInputsJson;
use risc0_groth16::{split_digest, to_json, ProofJson, Verifier};
use risc0_zkvm::sha::{Digest, Digestible};
use risc0_zkvm::{
    get_prover_server, recursion::identity_p254, ExecutorEnv, ExecutorImpl, ProverOpts,
    VerifierContext,
};
use serde::Deserialize;
use serde_json::from_str;
use sha2::Digest as OtherDigest;
use sha2::Sha256;
use std::fs;
use std::hash;
use std::io::{Read, Write};
use std::str::FromStr;
use std::{fs::File, io::Cursor, path::Path};
use num_bigint::BigUint;


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

    let proof: PublicProofJson = from_str(&fs::read_to_string("./work_dir/proof.json").unwrap()).unwrap();
    c_print("PROOF_PI_A0", &BigUint::from_str(&proof.pi_a[0]).unwrap().to_bytes_be());
}

// fn main2() {
//     env_logger::init();
//     let mut env = ExecutorEnv::builder();
//     env.write(&1u32).unwrap();
//     env.write(&(BLOCK_HEADERS.len() as u32)).unwrap();
//     let mut work = U256::ZERO;
//     let mut last_block_hash = [0u8; 32];
//     for i in 0..BLOCK_HEADERS.len() {
//         let block_header = BlockHeader::from_slice(&BLOCK_HEADERS[i]);
//         last_block_hash = double_sha256_hash!(&block_header.as_bytes());
//         work = validate_threshold_and_add_work(block_header.clone(), last_block_hash, work);
//         env.write(&block_header).unwrap();
//     }
//     println!("public output 2: {:?}", hex::encode(last_block_hash));
//     println!("public output 3: {:?}", hex::encode(work.to_be_bytes()));

//     let env = env.build().unwrap();

//     tracing::info!("execute");

//     let mut exec = ExecutorImpl::from_elf(env, GUEST_ELF).unwrap();
//     let session = exec.run().unwrap();

//     tracing::info!("prove");
//     let opts = ProverOpts::default();
//     let ctx = VerifierContext::default();
//     let prover = get_prover_server(&opts).unwrap();
//     let receipt = prover.prove_session(&ctx, &session).unwrap();
//     let (_, block_hash, pow, period): (u32, [u8; 32], [u8; 32], u32) =
//         receipt.journal.decode().unwrap();

//     let claim = receipt.get_claim().unwrap();
//     println!("claim : {:?}", claim);
//     let composite_receipt = receipt.inner.composite().unwrap();
//     let succinct_receipt = prover.compress(composite_receipt).unwrap();
//     let journal = session.journal.unwrap().bytes;

//     tracing::info!("identity_p254");
//     let ident_receipt = identity_p254(&succinct_receipt).unwrap();
//     let seal_bytes = ident_receipt.get_seal_bytes();

//     tracing::info!("stark-to-snark");
//     let seal_json = File::create(Path::new("./work_dir").join("input.json")).unwrap();
//     let mut seal_reader = Cursor::new(&seal_bytes);
//     to_json(&mut seal_reader, &seal_json).unwrap();
//     println!("Groth16 input is written to work_dir/input.json");
//     println!("run:\nsudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover");

//     println!("Guest ID: {:?}", give_image_id(GUEST_ID));
//     println!("claim.pre.digest(): {:?}", claim.pre.digest());
//     // let merkle_root = hex::decode("674798ef6b82d76f962faa6318f27f85cddfb18e6e212bf161ecd98db906908d").unwrap();
//     let claim_digest = sha256_hash!(
//         sha256_hash!("risc0.ReceiptClaim".as_bytes()),
//         claim.input,
//         claim.pre.digest(),
//         claim.post.digest(),
//         sha256_hash!(
//             sha256_hash!("risc0.Output".as_bytes()),
//             sha256_hash!(&journal),
//             [0u8; 32], // Assumptions' digest
//             2u16.to_le_bytes()
//         ),
//         0u32.to_le_bytes(),
//         0u32.to_le_bytes(),
//         4u16.to_le_bytes()
//     );
//     println!("expected Claim digest: {:?}", claim.digest().as_bytes());
//     println!(
//         "expected output digest: {:?}",
//         claim.output.digest().as_bytes()
//     );
//     c_print(
//         "RECEIPT_CLAIM_TAG",
//         &sha256_hash!("risc0.ReceiptClaim".as_bytes()),
//     );
//     c_print("CLAIM_INPUT", &claim.input.as_bytes());
//     c_print("CLAIM_PRE", &claim.pre.digest().as_bytes());
//     c_print("CLAIM_POST", &claim.post.digest().as_bytes());
//     c_print("OUTPUT_TAG", &sha256_hash!("risc0.Output".as_bytes()));
//     c_print("JOURNAL", &journal);
//     c_print("ZEROS", &[0u8; 32]);
//     c_print("TWO_U16", &2u16.to_le_bytes());
//     c_print("FOUR_U16", &4u16.to_le_bytes());
//     c_print("ZERO_U32", &0u32.to_le_bytes());
//     let (a0, a1) = split_digest_custom(Digest::from_hex(ALLOWED_IDS_ROOT).unwrap()); // This part is constant
//     println!("const char* public_input_1 = {:?};", a0);
//     println!("const char* public_input_2 = {:?};", a1);
//     let (c0, c1) = split_digest_custom(claim_digest.into());

//     c_print("BLOCK_HASH", &block_hash);
//     c_print("POW", &pow);
//     println!("const unsigned char PERIOD = {};", period);

//     // let public_inputs = vec![a0, a1, c0, c1];
//     // // write public inputs to work_dir/public_inputs.json
//     // // Serialize public_inputs to a JSON string
//     // let serialized = serde_json::to_string(&public_inputs).unwrap();

//     // // Create a file in work_dir for public_inputs.json
//     // let path = Path::new("./work_dir").join("public_inputs.json");
//     // let mut file = File::create(path).unwrap();

//     // Write the serialized string to the file
//     // file.write_all(serialized.as_bytes()).unwrap();
//     let public_inputs: PublicInputsJson = PublicInputsJson {
//         values: vec![
//             a0.to_string(),
//             a1.to_string(),
//             c0.to_string(),
//             c1.to_string(),
//         ],
//     };
//     // let proof =
//     // let proof = ProofJson {

//     // };
//     // read the proof from work_dir/proof.json to ProofJson struct
//     // Define the path to the proof.json file
//     let path = Path::new("./work_dir").join("proof.json");

//     // Open the file
//     let mut file = File::open(path).unwrap();

//     // Read the file's contents into a string
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     // Deserialize the JSON string into a ProofJson instance
//     let proof: ProofJson = serde_json::from_str(&contents).unwrap();

//     println!("public inpts: {:?}", public_inputs);

//     let verifier = Verifier::new(
//         &proof.try_into().unwrap(),
//         public_inputs.to_scalar().unwrap(),
//         prepared_verifying_key().unwrap(),
//     )
//     .unwrap();

//     println!("Verification status: {:?}", verifier.verify());

//     // tracing::info!("Receipt");
//     // let receipt = Receipt::new(
//     //     InnerReceipt::Compact(CompactReceipt { seal, claim }),
//     //     journal,
//     // );

//     // receipt.verify(MULTI_TEST_ID).unwrap();
// }
