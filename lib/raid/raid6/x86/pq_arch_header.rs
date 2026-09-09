/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the surrounding x86/kernel translation.

extern "C" {
    pub static raid6_mmxx1: raid6_calls;
    pub static raid6_mmxx2: raid6_calls;
    pub static raid6_sse1x1: raid6_calls;
    pub static raid6_sse1x2: raid6_calls;
    pub static raid6_sse2x1: raid6_calls;
    pub static raid6_sse2x2: raid6_calls;
    pub static raid6_sse2x4: raid6_calls;
    pub static raid6_avx2x1: raid6_calls;
    pub static raid6_avx2x2: raid6_calls;
    pub static raid6_avx2x4: raid6_calls;
    pub static raid6_avx512x1: raid6_calls;
    pub static raid6_avx512x2: raid6_calls;
    pub static raid6_avx512x4: raid6_calls;

    pub static raid6_recov_ssse3: raid6_recov_calls;
    pub static raid6_recov_avx2: raid6_recov_calls;
    pub static raid6_recov_avx512: raid6_recov_calls;
}

#[inline]
fn raid6_has_avx512() -> i32 {
    unsafe {
        (boot_cpu_has(X86_FEATURE_AVX2)
            && boot_cpu_has(X86_FEATURE_AVX)
            && boot_cpu_has(X86_FEATURE_AVX512F)
            && boot_cpu_has(X86_FEATURE_AVX512BW)
            && boot_cpu_has(X86_FEATURE_AVX512VL)
            && boot_cpu_has(X86_FEATURE_AVX512DQ)) as i32
    }
}

#[inline]
fn raid6_has_avx2() -> bool {
    unsafe { boot_cpu_has(X86_FEATURE_AVX2) && boot_cpu_has(X86_FEATURE_AVX) }
}

#[inline]
fn raid6_has_ssse3() -> bool {
    unsafe {
        boot_cpu_has(X86_FEATURE_XMM)
            && boot_cpu_has(X86_FEATURE_XMM2)
            && boot_cpu_has(X86_FEATURE_SSSE3)
    }
}

#[inline]
fn raid6_has_sse2() -> bool {
    unsafe {
        boot_cpu_has(X86_FEATURE_MMX)
            && boot_cpu_has(X86_FEATURE_FXSR)
            && boot_cpu_has(X86_FEATURE_XMM)
            && boot_cpu_has(X86_FEATURE_XMM2)
    }
}

#[inline]
fn raid6_has_sse1_or_mmxext() -> bool {
    unsafe {
        boot_cpu_has(X86_FEATURE_MMX)
            && (boot_cpu_has(X86_FEATURE_XMM) || boot_cpu_has(X86_FEATURE_MMXEXT))
    }
}

#[inline(always)]
pub unsafe fn arch_raid6_init() {
    // IS_ENABLED(CONFIG_X86_64) is represented by the target architecture.
    let x86_64 = cfg!(target_arch = "x86_64");

    if raid6_has_avx2() {
        raid6_algo_add(&raid6_avx2x1);
        raid6_algo_add(&raid6_avx2x2);
        if x86_64 {
            raid6_algo_add(&raid6_avx2x4);
        }
        if raid6_has_avx512() != 0 {
            raid6_algo_add(&raid6_avx512x1);
            raid6_algo_add(&raid6_avx512x2);
            if x86_64 {
                raid6_algo_add(&raid6_avx512x4);
            }
        }
    } else if x86_64 || raid6_has_sse2() {
        /* x86_64 can assume SSE2 as baseline */
        raid6_algo_add(&raid6_sse2x1);
        raid6_algo_add(&raid6_sse2x2);
        if x86_64 {
            raid6_algo_add(&raid6_sse2x4);
        }
    } else {
        raid6_algo_add_default();
        if raid6_has_sse1_or_mmxext() {
            raid6_algo_add(&raid6_sse1x1);
            raid6_algo_add(&raid6_sse1x2);
        } else if boot_cpu_has(X86_FEATURE_MMX) {
            raid6_algo_add(&raid6_mmxx1);
            raid6_algo_add(&raid6_mmxx2);
        }
    }

    if raid6_has_avx512() != 0 {
        raid6_recov_algo_add(&raid6_recov_avx512);
    } else if raid6_has_avx2() {
        raid6_recov_algo_add(&raid6_recov_avx2);
    } else if raid6_has_ssse3() {
        raid6_recov_algo_add(&raid6_recov_ssse3);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
