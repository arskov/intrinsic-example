#![feature(simd_ffi)]
#![feature(stdarch_x86_avx512)]
use std::arch::x86_64::{
    __m256, __m256i, __m512d, _mm256_castps_si256, _mm256_loadu_ps, _mm512_set1_pd,
};

extern "C" {
    fn call_mm256_mul_ph(a: __m256i, b: __m256i) -> __m256i;
    fn call_mm512_mul_pd(a: __m512d, b: __m512d) -> __m512d;
}

fn main() {
    // example 1: works only for CPU with AVX512_FP16
    unsafe {
        let a: [f32; 8] = [1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2];
        let fp32_v1: __m256 = _mm256_loadu_ps(a.as_ptr());
        let fp32_pack_v1: __m256i = _mm256_castps_si256(fp32_v1);

        let b: [f32; 8] = [2.1, 2.1, 2.1, 2.1, 2.1, 2.1, 2.1, 2.1];
        let fp32_v2: __m256 = _mm256_loadu_ps(b.as_ptr());
        let fp32_pack_v2: __m256i = _mm256_castps_si256(fp32_v2);

        let result = call_mm256_mul_ph(fp32_pack_v1, fp32_pack_v2);
        println!("{:?}", result);
    }
    // example 2: works for AVX512F, to check that it works
    unsafe {
        let a: __m512d = _mm512_set1_pd(1.2);
        let b: __m512d = _mm512_set1_pd(2.1);
        let result = call_mm512_mul_pd(a, b);
        println!("{:?}", result);
    }
}
