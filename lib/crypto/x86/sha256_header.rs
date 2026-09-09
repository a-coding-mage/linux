/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 optimized for x86_64
 *
 * Copyright 2025 Google LLC
 */

// C dependencies supplied by the surrounding kernel translation unit:
// struct sha256_block_state, struct __sha256_ctx, SHA256_DIGEST_SIZE,
// SHA256_BLOCK_SIZE, and the CPU/FPU/static-call helpers.

#[allow(non_upper_case_globals)]
static mut have_sha_ni: bool = false;

extern "C" {
    fn sha256_blocks_generic(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn sha256_transform_ssse3(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn sha256_transform_avx(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn sha256_transform_rorx(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn sha256_ni_transform(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn sha256_ni_finup2x(ctx: *const __sha256_ctx, data1: *const u8, data2: *const u8,
                         len: i32, out1: *mut u8, out2: *mut u8);
}

// External C types from the SHA-256 implementation.
#[allow(non_camel_case_types)]
type sha256_block_state = crate::sha256_block_state;
#[allow(non_camel_case_types)]
type __sha256_ctx = crate::__sha256_ctx;

#[inline]
unsafe fn sha256_blocks_ssse3(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha256_transform_ssse3(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

#[inline]
unsafe fn sha256_blocks_avx(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha256_transform_avx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

#[inline]
unsafe fn sha256_blocks_avx2(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha256_transform_rorx(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

#[inline]
unsafe fn sha256_blocks_ni(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    if irq_fpu_usable() {
        kernel_fpu_begin();
        sha256_ni_transform(state, data, nblocks);
        kernel_fpu_end();
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

const PHE_ALIGNMENT: usize = 16;

unsafe fn sha256_blocks_phe(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    /*
     * On Zhaoxin processors, XSHA256 requires the %rdi register
     * in 64-bit mode (or %edi in 32-bit mode) to point to
     * a 32-byte, 16-byte-aligned buffer.
     */
    let mut buf = [0u8; 32 + PHE_ALIGNMENT - 1];
    let dst = ((buf.as_mut_ptr() as usize + PHE_ALIGNMENT - 1) & !(PHE_ALIGNMENT - 1)) as *mut u8;
    let mut padding: usize = usize::MAX;
    core::ptr::copy_nonoverlapping(state as *const u8, dst, SHA256_DIGEST_SIZE);
    core::arch::asm!(
        ".byte 0xf3,0x0f,0xa6,0xd0", // REP XSHA256
        inout("rax") padding,
        inout("rcx") nblocks,
        inout("rsi") data => _,
        in("rdi") dst,
        options(nostack)
    );
    core::ptr::copy_nonoverlapping(dst, state as *mut u8, SHA256_DIGEST_SIZE);
}

unsafe fn sha256_blocks(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    static_call_sha256_blocks_x86(state, data, nblocks);
}

extern "C" {
    fn static_call_sha256_blocks_x86(state: *mut sha256_block_state, data: *const u8, nblocks: usize);
    fn irq_fpu_usable() -> bool;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn kmsan_unpoison_memory(ptr: *mut u8, len: usize);
    fn static_branch_likely(key: *const bool) -> bool;
    fn static_key_enabled(key: *const bool) -> bool;
    fn boot_cpu_has(feature: u32) -> bool;
    fn static_call_update(call: *const (), target: *const ());
    fn cpu_has_xfeatures(features: u64, ptr: *const core::ffi::c_void) -> bool;
}

unsafe fn sha256_finup_2x_arch(ctx: *const __sha256_ctx, data1: *const u8, data2: *const u8,
                               len: usize, out1: *mut u8, out2: *mut u8) -> bool {
    if static_branch_likely(&have_sha_ni) && len >= SHA256_BLOCK_SIZE && len <= 65536 && irq_fpu_usable() {
        kernel_fpu_begin();
        sha256_ni_finup2x(ctx, data1, data2, len as i32, out1, out2);
        kernel_fpu_end();
        kmsan_unpoison_memory(out1, SHA256_DIGEST_SIZE);
        kmsan_unpoison_memory(out2, SHA256_DIGEST_SIZE);
        return true;
    }
    false
}

unsafe fn sha256_finup_2x_is_optimized_arch() -> bool {
    static_key_enabled(&have_sha_ni)
}

unsafe fn sha256_mod_init_arch() {
    // The original selects the appropriate static-call target based on runtime x86 features.
    // Feature constants and boot_cpu_data are supplied by the surrounding kernel translation.
    if boot_cpu_has(X86_FEATURE_SHA_NI) {
        static_call_update(core::ptr::null(), sha256_blocks_ni as *const ());
        static_branch_enable(&mut have_sha_ni);
    } else if IS_ENABLED_CONFIG_CPU_SUP_ZHAOXIN && boot_cpu_has(X86_FEATURE_PHE_EN) && boot_cpu_x86 >= 0x07 {
        static_call_update(core::ptr::null(), sha256_blocks_phe as *const ());
    } else if cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null()) && boot_cpu_has(X86_FEATURE_AVX) {
        if boot_cpu_has(X86_FEATURE_AVX2) && boot_cpu_has(X86_FEATURE_BMI2) {
            static_call_update(core::ptr::null(), sha256_blocks_avx2 as *const ());
        } else {
            static_call_update(core::ptr::null(), sha256_blocks_avx as *const ());
        }
    } else if boot_cpu_has(X86_FEATURE_SSSE3) {
        static_call_update(core::ptr::null(), sha256_blocks_ssse3 as *const ());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
