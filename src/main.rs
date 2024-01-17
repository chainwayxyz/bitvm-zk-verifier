use circuit_helpers::bitcoin::BlockHeader;
use circuit_helpers::config::NUM_BLOCKS;
use reqwest;

use bitcoin_circuit::{GUEST_ELF, GUEST_ID};
use risc0_seal_to_json;
use risc0_zkvm::{
    get_prover_server,
    recursion::{identity_p254, lift},
    ExecutorEnv, ExecutorImpl, Groth16Receipt, Groth16Seal, InnerReceipt, ProverOpts, Receipt,
    VerifierContext,
};
use std::{fs::File, io::Cursor, path::Path, process::Command};

fn get_block_hash(block_height: usize) -> String {
    let url = format!("https://mempool.space/api/block-height/{}", block_height);
    reqwest::blocking::get(&url).unwrap().text().unwrap()
}

fn get_block_header(block_height: usize) -> BlockHeader {
    // let url = format!("https://mempool.space/api/block-height/{}", block_height);
    // let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let url = format!(
        "https://mempool.space/api/block/{}/header",
        get_block_hash(block_height)
    );
    let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let block_header: [u8; 80] = hex::decode(response).unwrap().try_into().unwrap();
    BlockHeader::from_slice(&block_header)
}

fn main() {
    env_logger::init();
    let mut env = ExecutorEnv::builder();
    let end_block_height = 100001;
    let prev_block_hash: [u8; 32] = hex::decode(get_block_hash(end_block_height - NUM_BLOCKS - 2))
        .unwrap()
        .try_into()
        .unwrap();
    env.write(&prev_block_hash).unwrap();
    println!("prev_block_hash: {:?}", prev_block_hash);
    for i in end_block_height - NUM_BLOCKS..end_block_height {
        let block_header = get_block_header(i);
        println!("block_header: {:?}", block_header);
        env.write(&block_header).unwrap();
    }

    let env_build = env.build().unwrap();

    println!("Executing guest program...");
    let mut exec = ExecutorImpl::from_elf(env_build, GUEST_ELF).unwrap();
    let session = exec.run().unwrap();
    let segments = session.resolve().unwrap();
    assert_eq!(segments.len(), 1);

    println!("Proving segment...");
    let opts = ProverOpts::default();
    let ctx = VerifierContext::default();
    let prover = get_prover_server(&opts).unwrap();
    let segment_receipt = prover.prove_segment(&ctx, &segments[0]).unwrap();


    println!("Lifting segment...");
    let lift_receipt = lift(&segment_receipt).unwrap();
    lift_receipt.verify_integrity().unwrap();

    println!("identity_p254...");
    let ident_receipt = identity_p254(&lift_receipt).unwrap();
    let seal_bytes = ident_receipt.get_seal_bytes();

    let journal = session.journal.unwrap().bytes;
    let succinct_receipt = Receipt::new(InnerReceipt::Succinct(ident_receipt), journal.clone());

    println!("Seal to json...");
    let work_dir = "./work_dir";
    let work_dir = Path::new(&work_dir);
    println!("work_dir: {:?}", work_dir);
    let seal_path = work_dir.join("input.json");
    let snark_path = work_dir.join("output.json");

    let seal_json = File::create(&seal_path).unwrap();
    let mut seal_reader = Cursor::new(&seal_bytes);
    risc0_seal_to_json::to_json(&mut seal_reader, &seal_json).unwrap();

}
