use hex::FromHex;
use risc0_binfmt::{MemoryImage, Program};
use risc0_groth16::{to_json, ProofJson, Verifier};
use risc0_groth16::{verifying_key, PublicInputsJson, Seal};
use risc0_zkvm::sha::{Digest, Digestible};
use risc0_zkvm::{get_prover_server, recursion::identity_p254, ProverOpts, GUEST_MAX_MEM};
use risc0_zkvm::{MaybePruned, Receipt, ReceiptClaim};
use serde_json::from_str;
use sha2::Digest as OtherDigest;
use sha2::Sha256;
use std::fs::read_to_string;
use std::{fs::File, io::Cursor, path::Path};

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

pub fn bytes_to_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(", ")
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

pub fn setup(elf: &[u8], template: &mut String) {
    const PAGE_SIZE: u32 = 1024;
    let program = Program::load_elf(elf, GUEST_MAX_MEM as u32).unwrap();
    let image = MemoryImage::new(&program, PAGE_SIZE).unwrap();
    let system_state = image.get_system_state();

    let (a0, a1) = split_digest_custom(Digest::from_hex(ALLOWED_IDS_ROOT).unwrap()); // This part is constant
    *template = template.replace("public_input_0", &bytes_to_str(&a0.to_le_bytes()));
    *template = template.replace("public_input_1", &bytes_to_str(&a1.to_le_bytes()));
    *template = template.replace(
        "receipt_claim_tag",
        &bytes_to_str(&sha256_hash!("risc0.ReceiptClaim".as_bytes())),
    );
    *template = template.replace(
        "output_tag",
        &bytes_to_str(&sha256_hash!("risc0.Output".as_bytes())),
    );
    *template = template.replace("claim_input", &bytes_to_str(&[0u8; 32]));
    *template = template.replace(
        "claim_pre",
        &bytes_to_str(&system_state.digest().as_bytes()),
    );
    *template = template.replace("zeroes", &bytes_to_str(&[0u8; 32]));
    *template = template.replace("two_u16", &bytes_to_str(&2u16.to_le_bytes()));
    *template = template.replace("four_u16", &bytes_to_str(&4u16.to_le_bytes()));
    *template = template.replace("zero_u32", &bytes_to_str(&0u32.to_le_bytes()));
}

pub fn prove(receipt: &Receipt, template: &mut String) {
    // let claim = receipt.inner.claim().unwrap().into();
    // let claim_digest = claim.digest();
    let opts = ProverOpts::default();
    let prover = get_prover_server(&opts).unwrap();
    let composite_receipt = receipt.inner.composite().unwrap();
    let succinct_receipt = prover.composite_to_succinct(&composite_receipt).unwrap();
    // let ident_receipt = identity_p254(&succinct_receipt).unwrap();
    // let seal_bytes = ident_receipt.get_seal_bytes();
    let groth16_receipt = prover.succinct_to_groth16(&succinct_receipt).unwrap();

    // let seal_json = File::create(Path::new("./work_dir").join("input.json")).unwrap();
    // let seal_json = File::create(Path::new("./work_dir").join("seal.json")).unwrap();
    // let mut seal_reader = Cursor::new(&groth16_receipt.seal);
    // to_json(&mut seal_reader, &seal_json).unwrap();
    println!("groth16 seal: {:#?}", groth16_receipt.seal);
    println!("groth16 claim: {:#?}", groth16_receipt.claim);
    let res = groth16_receipt.verify_integrity();
    println!("res: {:#?}", res);

    // let (a0, a1) = split_digest_custom(Digest::from_hex(ALLOWED_IDS_ROOT).unwrap()); // This part is constant
    // let claim = groth16_receipt.claim;

    let claim_digest = sha256_hash!(
        sha256_hash!("risc0.ReceiptClaim".as_bytes()),
        [0u8; 32],
        [
            187u8, 206u8, 146u8, 165u8, 135u8, 55u8, 168u8, 186u8, 228u8, 43u8, 84u8, 147u8, 76u8,
            253u8, 128u8, 197u8, 186u8, 84u8, 210u8, 110u8, 249u8, 92u8, 184u8, 56u8, 27u8, 133u8,
            189u8, 33u8, 58u8, 233u8, 229u8, 58u8
        ],
        [
            163u8, 172u8, 194u8, 113u8, 23u8, 65u8, 137u8, 150u8, 52u8, 11u8, 132u8, 229u8, 169u8,
            15u8, 62u8, 244u8, 196u8, 157u8, 34u8, 199u8, 158u8, 68u8, 170u8, 216u8, 34u8, 236u8,
            156u8, 49u8, 62u8, 30u8, 184u8, 226u8
        ],
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
    println!("claim_digest: {:#?}", claim_digest);
    // let (c0, c1) = split_digest_custom(claim_digest.into());

    // Test to verify
    // let public_inputs: PublicInputsJson = PublicInputsJson {
    //     values: vec![
    //         a0.to_string(),
    //         a1.to_string(),
    //         c0.to_string(),
    //         c1.to_string(),
    //     ],
    // };
    // let proof: ProofJson = from_str(&read_to_string("./work_dir/proof.json").unwrap()).unwrap();
    // let proof2: ProofJson = from_str(&read_to_string("./work_dir/proof.json").unwrap()).unwrap();

    // let seal: Seal = proof2.try_into().unwrap();
    // let mut proof_a = seal.a;
    // let proof_b = seal.b;
    // let mut proof_c = seal.c;

    // if proof_a[1][31] % 2 == 1 {
    //     proof_a[0][0] += 128;
    // }
    // proof_a[0].reverse();
    // let bytes_proof_a = &proof_a[0];

    // let mut proof_b_x = proof_b[0].clone();
    // if proof_b[1][1][31] % 2 == 1 {
    //     proof_b_x[0][0] += 128;
    // }
    // proof_b_x[0].reverse();
    // proof_b_x[1].reverse();
    // let mut bytes_proof_b = proof_b_x[1].clone();
    // bytes_proof_b.extend(proof_b_x[0].iter());

    // if proof_c[1][31] % 2 == 1 {
    //     proof_c[0][0] += 128;
    // }
    // proof_c[0].reverse();
    // let bytes_proof_c = &proof_c[0];

    // let verifier = Verifier::new(
    //     &proof.try_into().unwrap(),
    //     &public_inputs.to_scalar().unwrap(),
    //     verifying_key(),
    // )
    // .unwrap();

    // let verification_result = verifier.verify();

    // if verification_result.is_err() {
    //     println!("Groth16 input is written to work_dir/input.json");
    //     println!("run:\nsudo docker run --rm -v ./work_dir:/mnt risc0-groth16-prover");
    // }

    // *template = template.replace("claim_post", &bytes_to_str(&claim.post.digest().as_bytes()));
    // *template = template.replace("journalx", &bytes_to_str(&receipt.journal.bytes));
    // *template = template.replace("proof_a", &bytes_to_str(&bytes_proof_a));
    // *template = template.replace("proof_b", &bytes_to_str(&bytes_proof_b));
    // *template = template.replace("proof_c", &bytes_to_str(&bytes_proof_c));
}
