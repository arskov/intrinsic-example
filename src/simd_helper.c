#include <immintrin.h>

__m256i call_mm256_mul_ph(__m256i a, __m256i b) {
    return _mm256_mul_ph(a, b);
}

__m512d call_mm512_mul_pd(__m512d a, __m512d b) {
    return _mm512_mul_pd(a, b);
}