/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM3 optimized for x86_64
 *
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct sm3_block_state {
    _private: [u8; 0],
}

pub type u8 = ::core::ffi::c_uchar;
pub type size_t = usize;

unsafe extern "C" {
    pub fn sm3_transform_avx(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: size_t,
    );

    pub fn sm3_blocks_generic(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: size_t,
    );

    pub fn irq_fpu_usable() -> bool;
    pub fn kernel_fpu_begin();
    pub fn kernel_fpu_end();
    pub fn boot_cpu_has(feature: u32) -> bool;
    pub fn cpu_has_xfeatures(features: u64, feature: *mut c_void) -> bool;
    pub fn static_call_update(call: *mut Sm3BlocksFn, function: Sm3BlocksFn);
}

pub type Sm3BlocksFn = unsafe extern "C" fn(
    state: *mut sm3_block_state,
    data: *const u8,
    nblocks: size_t,
);

unsafe fn sm3_blocks_avx(
    state: *mut sm3_block_state,
    data: *const u8,
    nblocks: size_t,
) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sm3_transform_avx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sm3_blocks_generic(state, data, nblocks);
    }
}

// Equivalent of DEFINE_STATIC_CALL(sm3_blocks_x86, sm3_blocks_generic).
static mut sm3_blocks_x86: Sm3BlocksFn = sm3_blocks_generic;

unsafe fn sm3_blocks(
    state: *mut sm3_block_state,
    data: *const u8,
    nblocks: size_t,
) {
    let call = sm3_blocks_x86;
    call(state, data, nblocks);
}

// Equivalent of the self-referential preprocessor definition:
// #define sm3_mod_init_arch sm3_mod_init_arch
unsafe fn sm3_mod_init_arch() {
    const X86_FEATURE_AVX: u32 = 0;
    const X86_FEATURE_BMI2: u32 = 0;
    const XFEATURE_MASK_SSE: u64 = 0;
    const XFEATURE_MASK_YMM: u64 = 0;

    if boot_cpu_has(X86_FEATURE_AVX)
        && boot_cpu_has(X86_FEATURE_BMI2)
        && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null_mut())
    {
        static_call_update(
            core::ptr::addr_of_mut!(sm3_blocks_x86),
            sm3_blocks_avx,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
