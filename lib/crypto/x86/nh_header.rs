/* SPDX-License-Identifier: GPL-2.0 */
/*
 * x86_64 accelerated implementation of NH
 *
 * Copyright 2018 Google LLC
 */

// C dependencies from <asm/fpu/api.h> and <linux/static_call.h> are supplied
// by the surrounding kernel translation.

// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_sse2);
// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_avx2);
extern "C" {
    static mut have_sse2: bool;
    static mut have_avx2: bool;

    fn nh_sse2(key: *const u32, message: *const u8, message_len: usize,
               hash: *mut u64);
    fn nh_avx2(key: *const u32, message: *const u8, message_len: usize,
               hash: *mut u64);

    fn static_branch_likely(key: *const bool) -> bool;
    fn irq_fpu_usable() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(features: u64, feature: *const core::ffi::c_void) -> bool;
    fn static_branch_enable(key: *mut bool);
}

// #define nh_mod_init_arch nh_mod_init_arch
#[allow(non_snake_case)]
unsafe fn nh_arch(key: *const u32, message: *const u8, message_len: usize,
                  hash: *mut u64) -> bool {
    if message_len >= 64
        && static_branch_likely(&raw const have_sse2)
        && irq_fpu_usable()
    {
        kernel_fpu_begin();
        if static_branch_likely(&raw const have_avx2) {
            nh_avx2(key, message, message_len, hash);
        } else {
            nh_sse2(key, message, message_len, hash);
        }
        kernel_fpu_end();
        return true;
    }
    false
}

unsafe fn nh_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_XMM2) {
        static_branch_enable(&raw mut have_sse2);
        if boot_cpu_has(X86_FEATURE_AVX2)
            && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null())
        {
            static_branch_enable(&raw mut have_avx2);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
