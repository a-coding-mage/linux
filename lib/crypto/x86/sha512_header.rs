/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * x86-optimized SHA-512 block function
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit:
// <asm/fpu/api.h>
// <linux/static_call.h>

#[repr(C)]
pub struct sha512_block_state {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sha512_blocks_generic(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sha512_transform_ssse3(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sha512_transform_avx(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sha512_transform_rorx(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn irq_fpu_usable() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn cpu_has_xfeatures(xfeatures: u64, feature: *mut core::ffi::c_void) -> bool;
    fn boot_cpu_has(feature: u64) -> bool;
}

const XFEATURE_MASK_SSE: u64 = 1 << 1;
const XFEATURE_MASK_YMM: u64 = 1 << 2;
const X86_FEATURE_AVX: u64 = 0;
const X86_FEATURE_AVX2: u64 = 0;
const X86_FEATURE_BMI2: u64 = 0;
const X86_FEATURE_SSSE3: u64 = 0;

type Sha512BlocksFn = unsafe extern "C" fn(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
);

static mut SHA512_BLOCKS_X86: Sha512BlocksFn = sha512_blocks_generic;

unsafe extern "C" fn sha512_blocks_ssse3(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha512_transform_ssse3(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha512_blocks_generic(state, data, nblocks);
    }
}

unsafe extern "C" fn sha512_blocks_avx(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha512_transform_avx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha512_blocks_generic(state, data, nblocks);
    }
}

unsafe extern "C" fn sha512_blocks_avx2(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha512_transform_rorx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha512_blocks_generic(state, data, nblocks);
    }
}

unsafe extern "C" fn sha512_blocks(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    (SHA512_BLOCKS_X86)(state, data, nblocks);
}

// #define sha512_mod_init_arch sha512_mod_init_arch
unsafe extern "C" fn sha512_mod_init_arch() {
    if cpu_has_xfeatures(
        XFEATURE_MASK_SSE | XFEATURE_MASK_YMM,
        core::ptr::null_mut(),
    ) && boot_cpu_has(X86_FEATURE_AVX)
    {
        if boot_cpu_has(X86_FEATURE_AVX2) && boot_cpu_has(X86_FEATURE_BMI2) {
            SHA512_BLOCKS_X86 = sha512_blocks_avx2;
        } else {
            SHA512_BLOCKS_X86 = sha512_blocks_avx;
        }
    } else if boot_cpu_has(X86_FEATURE_SSSE3) {
        SHA512_BLOCKS_X86 = sha512_blocks_ssse3;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
