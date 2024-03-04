#![doc = include_str!("../README.md")]

use bitcoin::{block::Header, consensus::deserialize, secp256k1};
use bitcoin_pow_methods::CALCULATE_POW_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use secp256k1::hashes::Hash;

const HEADERS_HEX: [&str; 11] = [
    "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e36299",
    "010000004860eb18bf1b1620e37e9490fc8a427514416fd75159ab86688e9a8300000000d5fdcc541e25de1c7a5addedf24858b8bb665c9f36ef744ee42c316022c90f9bb0bc6649ffff001d08d2bd61",
    "01000000bddd99ccfda39da1b108ce1a5d70038d0a967bacb68b6b63065f626a0000000044f672226090d85db9a9f2fbfe5f0f9609b387af7be5b7fbb7a1767c831c9e995dbe6649ffff001d05e0ed6d",
    "010000004944469562ae1c2c74d9a535e00b6f3e40ffbad4f2fda3895501b582000000007a06ea98cd40ba2e3288262b28638cec5337c1456aaf5eedc8e9e5a20f062bdf8cc16649ffff001d2bfee0a9",
    "0100000085144a84488ea88d221c8bd6c059da090e88f8a2c99690ee55dbba4e00000000e11c48fecdd9e72510ca84f023370c9a38bf91ac5cae88019bee94d24528526344c36649ffff001d1d03e477",
    "01000000fc33f596f822a0a1951ffdbf2a897b095636ad871707bf5d3162729b00000000379dfb96a5ea8c81700ea4ac6b97ae9a9312b2d4301a29580e924ee6761a2520adc46649ffff001d189c4c97",
    "010000008d778fdc15a2d3fb76b7122a3b5582bea4f21f5a0c693537e7a03130000000003f674005103b42f984169c7d008370967e91920a6a5d64fd51282f75bc73a68af1c66649ffff001d39a59c86",
    "010000004494c8cf4154bdcc0720cd4a59d9c9b285e4b146d45f061d2b6c967100000000e3855ed886605b6d4a99d5fa2ef2e9b0b164e63df3c4136bebf2d0dac0f1f7a667c86649ffff001d1c4b5666",
    "01000000c60ddef1b7618ca2348a46e868afc26e3efc68226c78aa47f8488c4000000000c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd37047fca6649ffff001d28404f53",
    "010000000508085c47cc849eb80ea905cc7800a3be674ffc57263cf210c59d8d00000000112ba175a1e04b14ba9e7ea5f76ab640affeef5ec98173ac9799a852fa39add320cd6649ffff001d1e2de565",
    "01000000e915d9a478e3adf3186c07c61a22228b10fd87df343c92782ecc052c000000006e06373c80de397406dc3d19c90d71d230058d28293614ea58d6a57f8f5d32f8b8ce6649ffff001d173807f8"
];

pub fn calculate_pow() -> (Receipt, ([u8; 32], [u8; 32])) {
    let mut env = ExecutorEnv::builder();
    // for i in 0..BLOCK_HEADERS.len() {
    //     // env.write(&BLOCK_HEADERS[i]).unwrap();
    //     env.write(&BLOCK_HEADERS[i][0..4].try_into()).unwrap();
    // }
    // let env = env.build().unwrap();

    let headers: Vec<Header> = HEADERS_HEX
        .iter()
        .map(|h| deserialize(&hex::decode(h).unwrap()).unwrap())
        .collect();
    env.write(&(headers.len() as u32)).unwrap();
    for header in headers.iter() {
        let version = header.version.to_consensus();
        let mr = header.merkle_root;
        let merkle_root = mr.as_byte_array();
        let time = header.time;
        let bits = header.bits.to_consensus();
        let nonce = header.nonce;
        env.write(&version).unwrap();
        env.write(&merkle_root).unwrap();
        env.write(&time).unwrap();
        env.write(&bits).unwrap();
        env.write(&nonce).unwrap();
    }
    let env = env.build().unwrap();
    // let start_block_hash = headers[0].prev_blockhash.to_byte_array();
    // println!("start_block_hash: {:?}", start_block_hash);

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, CALCULATE_POW_ELF).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let (last_block_hash, pow): ([u8; 32], [u8; 32]) = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    let last_block_header_str: &str = "01000000e915d9a478e3adf3186c07c61a22228b10fd87df343c92782ecc052c000000006e06373c80de397406dc3d19c90d71d230058d28293614ea58d6a57f8f5d32f8b8ce6649ffff001d173807f8";
    let last_block_header: Header =
        deserialize(&hex::decode(last_block_header_str).unwrap()).unwrap();
    // println!("{:?}", last_block_header.block_hash().to_byte_array());
    assert_eq!(
        last_block_header.block_hash().to_byte_array(),
        last_block_hash
    );

    // Report the product
    println!("PoW up until block: {:?} is {:?}", last_block_hash, pow);
    (receipt, (last_block_hash, pow))
}

