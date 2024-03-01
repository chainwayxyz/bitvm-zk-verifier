#![no_main]
#![no_std]

use circuit_helpers::bitcoin::{validate_threshold_and_add_work, BlockHeader};
use circuit_helpers::config::PREV_BLOCK_HASH;
use circuit_helpers::double_sha256_hash;
use crypto_bigint::Encoding;
use crypto_bigint::U256;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let do_you_want_to_prove: u32 = env::read();
    env::commit(&do_you_want_to_prove);
    if do_you_want_to_prove == 1 {
        let mut previous_block_hash = PREV_BLOCK_HASH;
        let mut work = U256::ZERO;
        let num_blocks: u32 = env::read();

        for _ in 0..num_blocks {
            let block_header: BlockHeader = env::read();
            assert_eq!(block_header.previous_block_hash, previous_block_hash);
            let data = &block_header.as_bytes();
            let block_hash = double_sha256_hash!(data);
            work = validate_threshold_and_add_work(block_header, block_hash, work);
            previous_block_hash = block_hash;
        }
        // Outputs:
        env::commit(&previous_block_hash);
        env::commit(&work.to_be_bytes());
        env::commit(&31u8);
    }
}
