
#include "crypto_api.h"
#include <stdio.h>

int main() {

    CryptoContext ctx = {0};
    CryptoDigest digest = {0};
    uint8_t input[5] = {0x68, 0x65, 0x6c, 0x6c, 0x6f};

    if(crypto_init(&ctx, SHA3_256_ALG_ID) != Success) {
        printf("Error while initializing\n");
        return 1;
    }
    if (crypto_update(&ctx, input, 5) != Success) {
        printf("Error while updating\n");
        return 1;
    }
    if (crypto_finalize(&ctx, &digest) != Success) {
        printf("Error while finalizing\n");
        return 1;

    }

    uint8_t output[32] = {
        0x33, 0x38, 0xbe, 0x69, 0x4f, 0x50, 0xc5, 0xf3, 0x38, 0x81, 0x49, 0x86, 0xcd, 0xf0,
        0x68, 0x64, 0x53, 0xa8, 0x88, 0xb8, 0x4f, 0x42, 0x4d, 0x79, 0x2a, 0xf4, 0xb9, 0x20,
        0x23, 0x98, 0xf3, 0x92,
    };

    for (int i = 0; i < 32; i++) {
        if(output[i] != digest[i]) {
            printf("Wrong output at position %d, expected %d but found %d! :(\n", i, output[i], digest[i]);
            return 1;
        }
    }
    printf("Correct output! :)\nBye!\n");
    return 0;

}