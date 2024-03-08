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
unsigned char BYTES_PUBLIC_INPUTS[NPI][32] = {{109, 247, 8, 68, 118, 56, 211, 104, 40, 235, 244, 84, 89, 128, 255, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {49, 85, 98, 24, 28, 146, 109, 58, 158, 38, 151, 64, 95, 58, 207, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}};

// receipt claim tag, output tag, claim input
const unsigned char RECEIPT_CLAIM_TAG[] = {203, 31, 239, 205, 31, 45, 154, 100, 151, 92, 187, 191, 110, 22, 30, 41, 20, 67, 75, 12, 187, 153, 96, 184, 77, 245, 215, 23, 232, 107, 72, 175};
const unsigned char OUTPUT_TAG[] = {119, 234, 254, 179, 102, 167, 139, 71, 116, 125, 224, 215, 187, 23, 98, 132, 8, 95, 245, 86, 72, 135, 0, 154, 91, 230, 61, 163, 45, 53, 89, 212};
const unsigned char CLAIM_INPUT[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

// helper
const unsigned char ZEROS[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
const unsigned char TWO_U16[] = {2, 0};
const unsigned char FOUR_U16[] = {4, 0};
const unsigned char ZERO_U32[] = {0, 0, 0, 0};

// #########################
// ### CIRCUIT CONSTANTS ###
// #########################

const unsigned char CLAIM_PRE[] = {159, 69, 212, 17, 42, 22, 145, 162, 63, 225, 8, 173, 87, 196, 165, 193, 51, 11, 8, 204, 150, 186, 156, 96, 32, 167, 56, 30, 112, 63, 196, 60};

// #######################
// ### PROOF CONSTANTS ###
// #######################

// claim post
const unsigned char CLAIM_POST[] = {161, 229, 91, 57, 226, 138, 38, 194, 128, 177, 139, 19, 22, 92, 215, 11, 111, 115, 176, 199, 179, 83, 97, 51, 197, 252, 66, 209, 61, 249, 74, 173};

// proof
const unsigned char BYTES_PROOF_A[32] = {244, 79, 147, 160, 93, 139, 179, 88, 39, 151, 255, 184, 219, 77, 186, 30, 133, 169, 253, 190, 41, 99, 239, 100, 80, 42, 175, 205, 200, 219, 209, 162};
const unsigned char BYTES_PROOF_B[64] = {181, 159, 143, 241, 29, 40, 29, 104, 230, 136, 234, 0, 240, 186, 44, 37, 194, 205, 105, 60, 170, 131, 29, 218, 48, 25, 69, 140, 179, 215, 169, 39, 182, 145, 223, 114, 156, 242, 5, 236, 198, 206, 72, 123, 193, 177, 145, 28, 37, 152, 173, 59, 146, 35, 120, 31, 78, 153, 20, 10, 175, 101, 173, 173};
const unsigned char BYTES_PROOF_C[32] = {174, 227, 142, 18, 91, 226, 144, 94, 32, 35, 76, 92, 82, 39, 12, 194, 81, 149, 141, 190, 83, 104, 194, 39, 87, 144, 57, 87, 252, 94, 16, 0};

// journal
const unsigned char JOURNAL[] = {115, 0, 0, 0, 48, 0, 0, 0, 215, 0, 0, 0, 173, 0, 0, 0, 242, 0, 0, 0, 97, 0, 0, 0, 198, 0, 0, 0, 152, 0, 0, 0, 145, 0, 0, 0, 230, 0, 0, 0, 171, 0, 0, 0, 8, 0, 0, 0, 54, 0, 0, 0, 125, 0, 0, 0, 149, 0, 0, 0, 126, 0, 0, 0, 116, 0, 0, 0, 212, 0, 0, 0, 4, 0, 0, 0, 75, 0, 0, 0, 197, 0, 0, 0, 217, 0, 0, 0, 205, 0, 0, 0, 6, 0, 0, 0, 214, 0, 0, 0, 86, 0, 0, 0, 190, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0};
