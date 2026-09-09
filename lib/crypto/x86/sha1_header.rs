/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-1 optimized for x86_64
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by other translation units or platform bindings.
use core::ffi::c_void;

#[allow(non_camel_case_types)]
pub type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct sha1_block_state {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sha1_blocks_generic(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn irq_fpu_usable() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn sha1_transform_ssse3(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn sha1_transform_avx(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn sha1_ni_transform(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn sha1_transform_avx2(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(features: u64, enabled: *mut c_void) -> bool;
}

const X86_FEATURE_SHA_NI: u32 = 0;
const X86_FEATURE_AVX: u32 = 0;
const X86_FEATURE_AVX2: u32 = 0;
const X86_FEATURE_BMI1: u32 = 0;
const X86_FEATURE_BMI2: u32 = 0;
const XFEATURE_MASK_SSE: u64 = 0;
const XFEATURE_MASK_YMM: u64 = 0;

type Sha1BlocksFn = unsafe extern "C" fn(*mut sha1_block_state, *const u8, usize);

// DEFINE_STATIC_CALL(sha1_blocks_x86, sha1_blocks_generic)
static mut sha1_blocks_x86: Sha1BlocksFn = sha1_blocks_generic;

unsafe fn sha1_blocks_ssse3(state: *mut sha1_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha1_transform_ssse3(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

unsafe fn sha1_blocks_avx(state: *mut sha1_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha1_transform_avx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

unsafe fn sha1_blocks_ni(state: *mut sha1_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha1_ni_transform(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

const SHA1_AVX2_BLOCK_OPTSIZE: usize = 4; /* optimal 4*64 bytes of SHA1 blocks */

unsafe fn sha1_blocks_avx2(state: *mut sha1_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        /* Select the optimal transform based on the number of blocks */
        if nblocks >= SHA1_AVX2_BLOCK_OPTSIZE {
            sha1_transform_avx2(state, data, nblocks);
        } else {
            sha1_transform_avx(state, data, nblocks);
        }
        kernel_fpu_end();
    } else {
        sha1_blocks_generic(state, data, nblocks);
    }
}

unsafe fn sha1_blocks(state: *mut sha1_block_state, data: *const u8, nblocks: usize) {
    (sha1_blocks_x86)(state, data, nblocks);
}

// #define sha1_mod_init_arch sha1_mod_init_arch
unsafe fn sha1_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_SHA_NI) {
        sha1_blocks_x86 = sha1_blocks_ni;
    } else if cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null_mut())
        && boot_cpu_has(X86_FEATURE_AVX)
    {
        if boot_cpu_has(X86_FEATURE_AVX2)
            && boot_cpu_has(X86_FEATURE_BMI1)
            && boot_cpu_has(X86_FEATURE_BMI2)
        {
            sha1_blocks_x86 = sha1_blocks_avx2;
        } else {
            sha1_blocks_x86 = sha1_blocks_avx;
        }
    } else if boot_cpu_has(X86_FEATURE_SSSE3) {
        sha1_blocks_x86 = sha1_blocks_ssse3;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
