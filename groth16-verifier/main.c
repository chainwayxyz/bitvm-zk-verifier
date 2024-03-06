#include <string.h>
#include <mcl/bn_c256.h>
#include "sha256.h"
#include "constants.h"

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
    complete_public_inputs(BYTES_PUBLIC_INPUTS);
    PublicInputs public_inputs;
    for (int i = 0; i < NPI; i++) {
        mclBnFr_setLittleEndian(&public_inputs.public[i], BYTES_PUBLIC_INPUTS[i], 32);
    }
    return public_inputs;
}

mclBnG1 prepare_inputs(VerifyKey vk, PublicInputs public_inputs) {
    mclBnG1 res = vk.gamma_abc[0];
    mclBnG1 mul;
    for (int i = 0; i < NPI; i++) {
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

int verify_proof(VerifyKey vk, Proof proof, PublicInputs public_inputs) {
    mclBnG1 prepared_input = prepare_inputs(vk, public_inputs);
    return verify_proof_with_prepared_inputs(vk, proof, prepared_input);
}

int main() {
    int ret = mclBn_init(MCL_BN_SNARK1, MCLBN_COMPILED_TIME_VAR);
    if (ret != 0) {return 1;}
    VerifyKey vk = get_vk();
    PublicInputs public_inputs = get_public_inputs();
    Proof proof = get_proof();
    int a = verify_proof(vk, proof, public_inputs);
    return 1 - a;
}
