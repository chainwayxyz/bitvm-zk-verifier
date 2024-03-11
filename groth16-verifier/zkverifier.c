// #include <stdio.h>
#include <string.h>
#include <mcl/bn_c384_256.h>


#include <stddef.h>
#include <stdint.h>

// Memory Pool Size
#define MEMORY_POOL_SIZE 1024 * 4 // 1MB

// Custom memory pool
static uint8_t memory_pool[MEMORY_POOL_SIZE];
static size_t memory_pool_index = 0;

void *custom_malloc(size_t size) {
    if (memory_pool_index + size > MEMORY_POOL_SIZE){
        return NULL;}
    void *ptr = memory_pool + memory_pool_index;
    memory_pool_index += size;
    return ptr;
}

void custom_free(void *ptr) {
    // Simplistic implementation, does not actually free memory
}

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

// Dummy implementations for required functions
void __cxa_atexit() {}
void *__dso_handle;



void *sbrk(intptr_t increment) {
    static char *heap_end;
    char *prev_heap_end;

    if (heap_end == 0) {
        heap_end = (char *)&memory_pool;
    }
    prev_heap_end = heap_end;

    if (heap_end + increment > memory_pool + MEMORY_POOL_SIZE) {
        return (void *)-1;
    }

    heap_end += increment;
    return (void *)prev_heap_end;
}

void *__wrap_malloc(size_t size) {
    return custom_malloc(size);
}

void *__wrap_free(void *ptr) {
    custom_free(ptr);
}


typedef struct _FILE FILE;
FILE* fopen(const char *filename, const char *mode) {
    // Stub implementation
    return NULL;
}

int fclose(FILE *stream) {
    // while(1);
    // Stub implementation
    return 0; // Typically EOF is used to indicate failure
}

size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream) {
    // while(1);
    // Stub implementation
    return 0; // Indicating no items were read
}


#define NPI 1

const char* str_public_inputs[NPI] = {"13611538109402622838647648799309573680190580823435798300810952335263203112052"};
const char* str_proof_a = "1 19326975765597999023026309242851464618295625967876195641207265702903971944714 63835113064883221423828958902442994957834669122245713027012799265225733314";
const char* str_proof_b = "1 15926258242285065430281020483827261910589109953358642697562602626998569607754 15482775443177023962384497625528997151098293635727021527127534892096530509952 10937776115245591292644949547008094885240742374542777857501160790374675834371 6691169928456984700056024515478772626466687376966760922939239504103847739755";
const char* str_proof_c = "1 18548825552947889373136407684636046437705490810597995577018381312170231914454 9890815383090898074006770716054740281870916546141552254207684185588486179606";
const char* str_gamma_abc[NPI + 1] = {"1 8279781786940010385190155571855963988492685408361021424114669994156938727612 14410809027374663415870055712464930833085570569392210261413081131467855918386", "1 10075286447651977124250383403562829564641026794366918560412684031493085120161 13393494704849149158598989372790154670517815409968589728896053397011959461594"};
const char* str_alpha = "1 6763126530687886999315782887200758703366235230289874831627658839515656330867 12297948670392550312636836114470404429657568989657927437959695771502446445179";
const char* str_beta = "1 15362786867599176251482538547160991918100063526460909721657878971551583339657 3804423004921008809819632629079723167970572551072432396497601916259815496626 21885719103633717693283841528133243510750001708857084897139570082577218850374 2076817281717432063622727433912740683541778328445173073030513609350245776784";
const char* str_gamma = "1 1505558511994093266228972967760414664043255115544025409518939393775943607863 21131173266568468249589649137903719095480044620502529067534622738225157042304 4008759115482693545406793535591568078300615151288108694080317738431649117177 18835856718271757625037377080288624550370480296914695806777038708085497610013";
const char* str_delta = "1 1497911744463986566314308077983046202449361313910668647770797503379177516252 10829154948357654897792444316512827659620136273388886760324770466776134105520 10850392992008761830625471778404650447428083833210258292805429019728339148884 12593805385728178657844996215584371401133999503150901444097670307277076679963";

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
    mclBnG1_setStr(&proof.a, str_proof_a, strlen(str_proof_a), 10);
    mclBnG2_setStr(&proof.b, str_proof_b, strlen(str_proof_b), 10);
    mclBnG1_setStr(&proof.c, str_proof_c, strlen(str_proof_c), 10);
    return proof;
}

VerifyKey get_vk() {
    VerifyKey vk;
    mclBnG1_setStr(&vk.alpha, str_alpha, strlen(str_alpha), 10);
    mclBnG2_setStr(&vk.beta, str_beta, strlen(str_beta), 10);
    mclBnG2_setStr(&vk.gamma, str_gamma, strlen(str_gamma), 10);
    mclBnG2_setStr(&vk.delta, str_delta, strlen(str_delta), 10);
    for (int i = 0; i < NPI + 1; i++) {
        mclBnG1_setStr(&vk.gamma_abc[i], str_gamma_abc[i], strlen(str_gamma_abc[i]), 10);
    }
    mclBn_pairing(&vk.alpha_beta, &vk.alpha, &vk.beta);
    mclBnG2_neg(&vk.gamma_neg, &vk.gamma);
    mclBnG2_neg(&vk.delta_neg, &vk.delta);
    return vk;
}

PublicInputs get_public_inputs() {
    PublicInputs public_inputs;
    for (int i = 0; i < NPI; i++) {
        mclBnFr_setStr(&public_inputs.public[i], str_public_inputs[i], strlen(str_public_inputs[i]), 10);
    }
    return public_inputs;
}

void printFr(mclBnFr a, const char* info) {
    char s[100];
    mclBnFr_getStr(s, sizeof(s), &a, 16);
    // printf("%s = %s\n", info, s);
}

void printFp(mclBnFp a, const char* info) {
    char s[100];
    mclBnFp_getStr(s, sizeof(s), &a, 16);
    // printf("%s = %s\n", info, s);
}

void printFp2(mclBnFp2 a, const char* info) {
    char s0[100], s1[100];
    mclBnFp a0, a1;
    a0 = a.d[0];
    a1 = a.d[1];
    mclBnFp_getStr(s0, sizeof(s0), &a0, 16);
    mclBnFp_getStr(s1, sizeof(s1), &a1, 16);
    // printf("%s = (%s, %s)\n", info, s0, s1);
}

void printG1(mclBnG1 P, const char* info) {
    char s[200];
    //mclBnG1 Pn;
    //mclBnG1_normalize(&Pn, &P);
    mclBnG1_getStr(s, sizeof(s), &P, 16);
    // printf("%s = %s\n", info, s);
}

void printG2(mclBnG2 P, const char* info) {
    char s[400];
    //mclBnG1 Pn;
    //mclBnG1_normalize(&Pn, &P);
    mclBnG2_getStr(s, sizeof(s), &P, 16);
    // printf("%s = %s\n", info, s);
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
    int res = verify_proof_with_prepared_inputs(vk, proof, prepared_input);
    if(res){
        return 311;
    } else {
        return 69;
    }
    // while(1);
    // if(res){
    //     while(1);
    // }
    // printf("%s\n", res ? "Verify: True" : "Verify: False");
}

int main() {
    int ret = mclBn_init(MCL_BN_SNARK1, MCLBN_COMPILED_TIME_VAR);
    if (ret != 0) {
        return 32;
    }
    VerifyKey vk = get_vk();
    // while(1);
    // return 312;
    PublicInputs public_inputs = get_public_inputs();
    Proof proof = get_proof();
    return verify_proof(vk, proof, public_inputs);
}
