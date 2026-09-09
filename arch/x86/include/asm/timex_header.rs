/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding processor and TSC headers.
extern "C" {
    fn cpu_feature_enabled(feature: i32) -> bool;
    fn random_get_entropy_fallback() -> usize;
    fn rdtsc() -> usize;
}

/// Architecture feature identifier supplied by the processor header.
extern "C" {
    static X86_FEATURE_TSC: i32;
}

pub unsafe fn random_get_entropy() -> usize {
    if !cpu_feature_enabled(X86_FEATURE_TSC) {
        return random_get_entropy_fallback();
    }
    rdtsc()
}

// Preserve the C preprocessor alias: #define random_get_entropy random_get_entropy

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
