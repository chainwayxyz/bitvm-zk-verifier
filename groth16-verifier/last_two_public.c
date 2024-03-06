#include "sha256.h"
#include "last_two_constants.h"

BYTE buf[SHA256_BLOCK_SIZE];

void complete_public_inputs(unsigned char public_inputs[][32]) {
    SHA256_CTX output_ctx, claim_ctx, journal_ctx;
    sha256_init(&journal_ctx);
    sha256_init(&output_ctx);
    sha256_init(&claim_ctx);
    
    sha256_update(&claim_ctx, RECEIPT_CLAIM_TAG, sizeof(RECEIPT_CLAIM_TAG)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_INPUT, sizeof(CLAIM_INPUT)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_PRE, sizeof(CLAIM_PRE)/sizeof(unsigned char));

    sha256_update(&output_ctx, OUTPUT_TAG, sizeof(OUTPUT_TAG)/sizeof(unsigned char));

    sha256_update(&claim_ctx, CLAIM_POST, sizeof(CLAIM_POST)/sizeof(unsigned char));

    /// CUTOFF
    sha256_update(&journal_ctx, JOURNAL, sizeof(JOURNAL)/sizeof(unsigned char));
    sha256_final(&journal_ctx, buf);
    
    sha256_update(&output_ctx, buf, SHA256_BLOCK_SIZE);
    sha256_update(&output_ctx, ZEROS, sizeof(ZEROS)/sizeof(unsigned char));
    sha256_update(&output_ctx, TWO_U16, sizeof(TWO_U16)/sizeof(unsigned char));
    sha256_final(&output_ctx, buf);

    sha256_update(&claim_ctx, buf, SHA256_BLOCK_SIZE);
    sha256_update(&claim_ctx, ZERO_U32, sizeof(ZERO_U32)/sizeof(unsigned char));
    sha256_update(&claim_ctx, ZERO_U32, sizeof(ZERO_U32)/sizeof(unsigned char));
    sha256_update(&claim_ctx, FOUR_U16, sizeof(FOUR_U16)/sizeof(unsigned char));
    sha256_final(&claim_ctx, buf);

    for(int i = 0; i < 16; i++){
        public_inputs[2][i] = buf[i];
        public_inputs[3][i] = buf[i + 16];
    }
}
