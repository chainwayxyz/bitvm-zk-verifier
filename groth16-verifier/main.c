#include <mcl/bn_c384_256.h>
#include <string.h>
#include <stddef.h>
#include <stdint.h>

// Sha256 implementaion from https://github.com/B-Con/crypto-algorithms/blob/master/sha256.c
#include "sha256.h"
#include "constants.h"

BYTE buf[SHA256_BLOCK_SIZE];
SHA256_CTX output_ctx, claim_ctx, journal_ctx;

typedef struct {
    mclBnG1 a;
    mclBnG2 b;
    mclBnG1 c;
} Proof;

typedef struct {
    mclBnG1 alpha;
    mclBnG2 beta;
    mclBnG2 gamma;
    mclBnG2 delta;
    mclBnG1 gamma_abc[NPI + 1]; 
    mclBnG2 gamma_neg;
    mclBnG2 delta_neg;
    mclBnGT alpha_beta;
} VerifyKey;

typedef struct {
    mclBnFr public[NPI];
} PublicInputs;

Proof get_proof() {
    Proof proof;
    mclBnG1_deserialize(&proof.a, BYTES_PROOF_A, 32);
    mclBnG2_deserialize(&proof.b, BYTES_PROOF_B, 64);
    mclBnG1_deserialize(&proof.c, BYTES_PROOF_C, 32);
    return proof;
}

VerifyKey get_vk() {
    VerifyKey vk;
    mclBnG1_deserialize(&vk.alpha, BYTES_ALPHA, 32);
    mclBnG2_deserialize(&vk.beta, BYTES_BETA, 64);
    mclBnG2_deserialize(&vk.gamma, BYTES_GAMMA, 64);
    mclBnG2_deserialize(&vk.delta, BYTES_DELTA, 64);
    for (int i = 0; i < NPI + 1; i++) {
        mclBnG1_deserialize(&vk.gamma_abc[i], BYTES_GAMMA_ABC[i], 32);
    }
    mclBn_pairing(&vk.alpha_beta, &vk.alpha, &vk.beta);
    mclBnG2_neg(&vk.gamma_neg, &vk.gamma);
    mclBnG2_neg(&vk.delta_neg, &vk.delta);
    return vk;
}

PublicInputs get_public_inputs() {
    PublicInputs public_inputs;
    for (int i = 0; i < NPI / 2; i++) {
        mclBnFr_setLittleEndian(&public_inputs.public[i], BYTES_PUBLIC_INPUTS[i], 32);
    }
    return public_inputs;
}

mclBnG1 prepare_inputs_pre_cutoff(VerifyKey vk, PublicInputs public_inputs) {
    mclBnG1 res = vk.gamma_abc[0];
    mclBnG1 mul;
    for (int i = 0; i < NPI / 2; i++) {
        mclBnG1_mul(&mul, &vk.gamma_abc[i + 1], &public_inputs.public[i]);
        mclBnG1_add(&res, &res, &mul);
    }
    return res;
}

mclBnG1 prepare_inputs(VerifyKey vk, PublicInputs public_inputs, mclBnG1 partially_prepared_input) {
    mclBnG1 res = partially_prepared_input;
    mclBnG1 mul;
    for (int i = NPI / 2; i < NPI; i++) {
        mclBnG1_mul(&mul, &vk.gamma_abc[i + 1], &public_inputs.public[i]);
        mclBnG1_add(&res, &res, &mul);
    }
    return res;
}

int verify_proof_with_prepared_inputs(VerifyKey vk, Proof proof, mclBnG1 prepared_input) {
    mclBnGT ml1, ml2, ml3;
    mclBn_millerLoop(&ml1, &proof.a, &proof.b);
    mclBn_millerLoop(&ml2, &prepared_input, &vk.gamma_neg);
    mclBn_millerLoop(&ml3, &proof.c, &vk.delta_neg);
    mclBnGT mul = ml1;
    mclBnGT_mul(&mul, &mul, &ml2);
    mclBnGT_mul(&mul, &mul, &ml3);
    mclBn_finalExp(&mul, &mul);
    return mclBnGT_isEqual(&mul, &vk.alpha_beta);
}

int main() {
    int ret = mclBn_init(MCL_BN_SNARK1, MCLBN_COMPILED_TIME_VAR);
    if (ret != 0) {return 31;}
    VerifyKey vk = get_vk();


    sha256_init(&journal_ctx);
    sha256_init(&output_ctx);
    sha256_init(&claim_ctx);
    sha256_update(&claim_ctx, RECEIPT_CLAIM_TAG, sizeof(RECEIPT_CLAIM_TAG)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_INPUT, sizeof(CLAIM_INPUT)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_PRE, sizeof(CLAIM_PRE)/sizeof(unsigned char));
    sha256_update(&output_ctx, OUTPUT_TAG, sizeof(OUTPUT_TAG)/sizeof(unsigned char));
    sha256_update(&claim_ctx, CLAIM_POST, sizeof(CLAIM_POST)/sizeof(unsigned char));
    
    // not fully initialized (missing 3rd and 4th public inputs)
    PublicInputs public_inputs = get_public_inputs();
    mclBnG1 partially_prepared_input = prepare_inputs_pre_cutoff(vk, public_inputs);
    // /// CUTOFF

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
        BYTES_PUBLIC_INPUTS[2][i] = buf[i];
        BYTES_PUBLIC_INPUTS[3][i] = buf[i + 16];
    }
    
    // fill the missing public inputs
    for (int i = NPI / 2; i < NPI; i++) {
        mclBnFr_setLittleEndian(&public_inputs.public[i], BYTES_PUBLIC_INPUTS[i], 32);
    }

    Proof proof = get_proof();
    mclBnG1 prepared_input = prepare_inputs(vk, public_inputs, partially_prepared_input);
    return 1 - verify_proof_with_prepared_inputs(vk, proof, prepared_input);
}


/// For bare metal:

void *memcpy(void *dest, const void *src, size_t n) {
    char *dp = dest;
    const char *sp = src;
    while (n--) *dp++ = *sp++;
    return dest;
}

void *memset(void *s, int c, size_t n) {
    unsigned char* p = s;
    while (n--) *p++ = (unsigned char)c;
    return s;
}

size_t strlen(const char *str) {
    const char *s;
    for (s = str; *s; ++s);
    return (s - str);
}

int strcmp(const char *str1, const char *str2) {
    while (*str1 && (*str1 == *str2)) {
        str1++, str2++;
    }
    return *(const unsigned char*)str1 - *(const unsigned char*)str2;
}

void __cxa_atexit() {}
void *__dso_handle;


void *__wrap_malloc(size_t size) {
    return NULL;
}

void *__wrap_free(void *ptr) {
    return NULL;
}


typedef struct _FILE FILE;
FILE* fopen(const char *filename, const char *mode) {
    return NULL;
}

int fclose(FILE *stream) {
    return 0;
}

size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream) {
    return 0;
}
