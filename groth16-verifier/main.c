#include "sha256.h"
const unsigned char RECEIPT_CLAIM_TAG[] = {203, 31, 239, 205, 31, 45, 154, 100, 151, 92, 187, 191, 110, 22, 30, 41, 20, 67, 75, 12, 187, 153, 96, 184, 77, 245, 215, 23, 232, 107, 72, 175};
const unsigned char CLAIM_INPUT[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
const unsigned char CLAIM_PRE[] = {178, 3, 235, 227, 173, 4, 140, 14, 8, 82, 19, 22, 206, 243, 26, 103, 182, 58, 190, 224, 119, 246, 26, 23, 200, 99, 229, 164, 163, 211, 253, 91};
const unsigned char CLAIM_POST[] = {73, 23, 30, 65, 63, 87, 220, 212, 89, 52, 47, 51, 2, 248, 37, 190, 62, 44, 75, 72, 38, 238, 199, 26, 94, 1, 252, 9, 247, 215, 115, 159};
const unsigned char OUTPUT_TAG[] = {119, 234, 254, 179, 102, 167, 139, 71, 116, 125, 224, 215, 187, 23, 98, 132, 8, 95, 245, 86, 72, 135, 0, 154, 91, 230, 61, 163, 45, 53, 89, 212};
const unsigned char JOURNAL[] = {69, 38, 0, 0, 0, 0, 0, 0};
const unsigned char ZEROS[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
const unsigned char TWO_U16[] = {2, 0};
const unsigned char FOUR_U16[] = {4, 0};
const unsigned char ZERO_U32[] = {0, 0, 0, 0};
const unsigned char FIRST_PUBLIC_INPUT[] = {57, 255, 128, 89, 84, 244, 235, 40, 104, 211, 56, 118, 68, 8, 247, 109};
const unsigned char SECOND_PUBLIC_INPUT[] = {21, 207, 58, 95, 64, 151, 38, 158, 58, 109, 146, 28, 24, 98, 85, 49};
// const unsigned char EXPECTED_THIRD_PUBLIC_INPUT[] = {189, 211, 205, 207, 223, 80, 37, 104, 244, 213, 195, 245, 215, 231, 33, 238};
// const unsigned char EXPECTED_FOURTH_PUBLIC_INPUT[] = {142, 60, 66, 176, 59, 103, 166, 247, 140, 142, 155, 196, 14, 127, 179, 63};
const unsigned char PROOF_PI_A0[] = {5, 210, 196, 81, 17, 55, 154, 12, 91, 5, 99, 148, 129, 176, 7, 99, 95, 61, 249, 34, 166, 238, 56, 99, 181, 182, 81, 14, 15, 57, 163, 119};

unsigned char second_public_inputs[2][16];

BYTE buf[SHA256_BLOCK_SIZE];

// void prepare_journal() {
//     int i = 0, idx = 0;
//     for(; i < 32; i++, idx += 4){
//         JOURNAL[idx] = BLOCK_HASH[i];
//     }
//     i=0;
//     for(; i < 32; i++, idx += 4){
//         JOURNAL[idx] = POW[i];
//     }
//     JOURNAL[idx] = PERIOD;
// }

int main() {
    SHA256_CTX output_ctx, claim_ctx, journal_ctx;
    sha256_init(&journal_ctx);
    sha256_init(&output_ctx);
    sha256_init(&claim_ctx);
    
    sha256_update(&claim_ctx, RECEIPT_CLAIM_TAG, sizeof(RECEIPT_CLAIM_TAG)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_INPUT, sizeof(CLAIM_INPUT)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_PRE, sizeof(CLAIM_PRE)/sizeof(unsigned char));

    // We calculate the output digest
    sha256_update(&output_ctx, OUTPUT_TAG, sizeof(OUTPUT_TAG)/sizeof(unsigned char));
    // sha256_update(&journal_ctx, JOURNAL_START, sizeof(JOURNAL_START)/sizeof(unsigned char));

    // prepare_journal();
    // The JOURNAL is the actual output of the risc0 circuit
    sha256_update(&claim_ctx, CLAIM_POST, sizeof(CLAIM_POST)/sizeof(unsigned char));
    sha256_update(&journal_ctx, JOURNAL, sizeof(JOURNAL)/sizeof(unsigned char));
    sha256_final(&journal_ctx, buf);
    
    sha256_update(&output_ctx, buf, SHA256_BLOCK_SIZE);
    sha256_update(&output_ctx, ZEROS, sizeof(ZEROS)/sizeof(unsigned char));
    sha256_update(&output_ctx, TWO_U16, sizeof(TWO_U16)/sizeof(unsigned char));
    sha256_final(&output_ctx, buf);

    // for (int i = 0; i < 32; i++){
    //     printf("%d ", buf[i]);
    // }
    // printf("\n\n");
    sha256_update(&claim_ctx, buf, SHA256_BLOCK_SIZE);
    sha256_update(&claim_ctx, ZERO_U32, sizeof(ZERO_U32)/sizeof(unsigned char));
    sha256_update(&claim_ctx, ZERO_U32, sizeof(ZERO_U32)/sizeof(unsigned char));
    sha256_update(&claim_ctx, FOUR_U16, sizeof(FOUR_U16)/sizeof(unsigned char));
    sha256_final(&claim_ctx, buf);

    // for (int i = 0; i < 32; i++){
    //     printf("%d ", buf[i]);
    // }
    for(int i = 0; i < 16; i++){
        second_public_inputs[0][15 - i] = buf[i];
        second_public_inputs[1][15 - i] = buf[i + 16];
    }
    

    for (int i = 0; i < 16; i++){
        printf("%d ", second_public_inputs[0][i]);
    }
    printf("\n");

    for (int i = 0; i < 16; i++){
        printf("%d ", second_public_inputs[1][i]);
    }
    printf("\n");
    return 0;
}