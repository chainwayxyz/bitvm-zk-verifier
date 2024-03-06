#include <string.h>
#include <mcl/bn_c256.h>
#include "vk.h"
#include "proof.h"
#include "first_two_public.h"
#include "last_two_public.h"

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
    mclBnG1_deserialize(&proof.a, bytes_proof_a, 32);
    mclBnG2_deserialize(&proof.b, bytes_proof_b, 64);
    mclBnG1_deserialize(&proof.c, bytes_proof_c, 32);
    return proof;
}

VerifyKey get_vk() {
    VerifyKey vk;
    mclBnG1_deserialize(&vk.alpha, bytes_alpha, 32);
    mclBnG2_deserialize(&vk.beta, bytes_beta, 64);
    mclBnG2_deserialize(&vk.gamma, bytes_gamma, 64);
    mclBnG2_deserialize(&vk.delta, bytes_delta, 64);
    for (int i = 0; i < NPI + 1; i++) {
        mclBnG1_deserialize(&vk.gamma_abc[i], bytes_gamma_abc[i], 32);
    }
    mclBn_pairing(&vk.alpha_beta, &vk.alpha, &vk.beta);
    mclBnG2_neg(&vk.gamma_neg, &vk.gamma);
    mclBnG2_neg(&vk.delta_neg, &vk.delta);
    return vk;
}

PublicInputs get_public_inputs() {
    complete_public_inputs(bytes_public_inputs);
    PublicInputs public_inputs;
    for (int i = 0; i < NPI; i++) {
        mclBnFr_setLittleEndian(&public_inputs.public[i], bytes_public_inputs[i], 32);
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
