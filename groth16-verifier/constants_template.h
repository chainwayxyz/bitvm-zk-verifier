#define NPI 4

// #######################
// ### RISC0 CONSTANTS ###
// #######################

// vk
const unsigned char BYTES_ALPHA[32] = {226, 242, 109, 190, 162, 153, 245, 34, 59, 100, 108, 177, 251, 51, 234, 219, 5, 157, 148, 7, 85, 157, 116, 65, 223, 217, 2, 227, 167, 154, 77, 45};
const unsigned char BYTES_BETA[64] = {171, 183, 61, 193, 127, 188, 19, 2, 30, 36, 113, 224, 192, 139, 214, 125, 132, 1, 245, 43, 115, 214, 208, 116, 131, 121, 76, 173, 71, 120, 24, 14, 12, 6, 243, 59, 188, 76, 121, 169, 202, 222, 242, 83, 166, 128, 132, 211, 130, 241, 119, 136, 248, 133, 201, 175, 209, 118, 247, 203, 47, 3, 103, 9};
const unsigned char BYTES_GAMMA[64] = {237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25};
const unsigned char BYTES_DELTA[64] = {227, 202, 126, 90, 112, 58, 8, 221, 146, 228, 249, 78, 27, 238, 250, 73, 9, 19, 166, 142, 59, 145, 36, 124, 102, 204, 121, 74, 187, 143, 118, 46, 21, 178, 98, 239, 185, 69, 233, 26, 86, 194, 76, 16, 76, 25, 253, 208, 149, 111, 58, 190, 39, 204, 63, 143, 106, 73, 168, 218, 242, 168, 160, 45};
const unsigned char BYTES_GAMMA_ABC[5][32] = {{187, 87, 190, 34, 163, 78, 141, 241, 142, 40, 142, 34, 90, 165, 48, 49, 167, 98, 162, 144, 183, 121, 111, 186, 248, 149, 68, 228, 55, 10, 41, 138}, {21, 241, 220, 44, 117, 83, 183, 92, 121, 116, 89, 60, 73, 234, 40, 28, 250, 79, 142, 195, 206, 104, 243, 48, 133, 89, 10, 252, 4, 200, 72, 42}, {179, 215, 155, 62, 101, 97, 145, 53, 248, 5, 51, 187, 178, 28, 36, 213, 182, 189, 61, 255, 134, 88, 241, 140, 208, 73, 151, 42, 69, 212, 169, 149}, {70, 154, 73, 253, 250, 44, 93, 206, 83, 205, 22, 195, 5, 19, 165, 80, 90, 157, 124, 228, 13, 46, 226, 143, 197, 198, 30, 89, 185, 146, 87, 138}, {21, 137, 189, 23, 223, 1, 40, 128, 37, 33, 45, 252, 91, 43, 238, 46, 182, 82, 96, 94, 9, 16, 147, 183, 215, 93, 161, 206, 240, 245, 245, 166}};

// first two public inputs are constant
unsigned char BYTES_PUBLIC_INPUTS[NPI][32] = {{public_input_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {public_input_1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}};

// helper
const unsigned char ZEROS[] = {zeroes};
const unsigned char TWO_U16[] = {two_u16};
const unsigned char FOUR_U16[] = {four_u16};
const unsigned char ZERO_U32[] = {zero_u32};

// #########################
// ### CIRCUIT CONSTANTS ###
// #########################

const unsigned char RECEIPT_CLAIM_TAG[] = {receipt_claim_tag};
const unsigned char OUTPUT_TAG[] = {output_tag};
const unsigned char CLAIM_INPUT[] = {claim_input};
const unsigned char CLAIM_PRE[] = {178, 3, 235, 227, 173, 4, 140, 14, 8, 82, 19, 22, 206, 243, 26, 103, 182, 58, 190, 224, 119, 246, 26, 23, 200, 99, 229, 164, 163, 211, 253, 91};
const unsigned char CLAIM_POST[] = {73, 23, 30, 65, 63, 87, 220, 212, 89, 52, 47, 51, 2, 248, 37, 190, 62, 44, 75, 72, 38, 238, 199, 26, 94, 1, 252, 9, 247, 215, 115, 159};

// #######################
// ### PROOF CONSTANTS ###
// #######################

// proof
const unsigned char BYTES_PROOF_A[32] = {proof_a};
const unsigned char BYTES_PROOF_B[64] = {proof_b};
const unsigned char BYTES_PROOF_C[32] = {proof_c};

// journal
const unsigned char JOURNAL[] = {journalx};
