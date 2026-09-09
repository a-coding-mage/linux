/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Poly1305 authenticator algorithm, RFC7539.
 *
 * Copyright 2023- IBM Corp. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    pub fn poly1305_p10le_4blocks(
        state: *mut poly1305_block_state,
        m: *const u8,
        mlen: u32,
    );
    pub fn poly1305_64s(
        state: *mut poly1305_block_state,
        m: *const u8,
        mlen: u32,
        highbit: core::ffi::c_int,
    );
    pub fn poly1305_emit_64(
        state: *const poly1305_state,
        nonce: *const u32,
        digest: *mut u8,
    );
    pub static mut have_p10: static_key_false;
}

extern "C" {
    pub fn poly1305_block_init_generic(
        dctx: *mut poly1305_block_state,
        raw_key: *const u8,
    );
    pub fn poly1305_blocks_generic(
        state: *mut poly1305_block_state,
        src: *const u8,
        len: core::ffi::c_uint,
        padbit: u32,
    );
    pub fn poly1305_emit_generic(
        state: *const poly1305_state,
        digest: *mut u8,
        nonce: *const u32,
    );
    pub fn static_key_enabled(key: *const static_key_false) -> bool;
    pub fn static_branch_enable(key: *mut static_key_false);
    pub fn cpu_has_feature(feature: core::ffi::c_uint) -> bool;
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn enable_kernel_vsx();
    pub fn disable_kernel_vsx();
    pub fn get_unaligned_le64(ptr: *const u8) -> u64;
}

// Types and constants are supplied by the surrounding Poly1305 implementation.
#[allow(non_camel_case_types)]
pub type static_key_false = core::ffi::c_void;
extern "C" {
    pub static POLY1305_BLOCK_SIZE: usize;
    pub static POLY1305_DIGEST_SIZE: usize;
}

#[repr(C)]
pub struct poly1305_block_state {
    pub h: poly1305_state,
    pub core_r: poly1305_core_r,
}

#[repr(C)]
pub struct poly1305_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poly1305_core_r {
    pub key: poly1305_key,
}

#[repr(C)]
pub union poly1305_key {
    pub r64: [u64; 2],
}

unsafe fn vsx_begin() {
    preempt_disable();
    enable_kernel_vsx();
}

unsafe fn vsx_end() {
    disable_kernel_vsx();
    preempt_enable();
}

unsafe fn poly1305_block_init(
    dctx: *mut poly1305_block_state,
    raw_key: *const u8,
) {
    if !static_key_enabled(&have_p10) {
        return poly1305_block_init_generic(dctx, raw_key);
    }

    (*dctx).h = core::mem::zeroed();
    (*dctx).core_r.key.r64[0] = get_unaligned_le64(raw_key.add(0));
    (*dctx).core_r.key.r64[1] = get_unaligned_le64(raw_key.add(8));
}

unsafe fn poly1305_blocks(
    state: *mut poly1305_block_state,
    mut src: *const u8,
    mut len: core::ffi::c_uint,
    padbit: u32,
) {
    if !static_key_enabled(&have_p10) {
        return poly1305_blocks_generic(state, src, len, padbit);
    }
    vsx_begin();
    if len >= (POLY1305_BLOCK_SIZE * 4) as core::ffi::c_uint {
        poly1305_p10le_4blocks(state, src, len);
        src = src.add((len - (len % (POLY1305_BLOCK_SIZE * 4) as core::ffi::c_uint)) as usize);
        len %= (POLY1305_BLOCK_SIZE * 4) as core::ffi::c_uint;
    }
    while len >= POLY1305_BLOCK_SIZE as core::ffi::c_uint {
        poly1305_64s(state, src, POLY1305_BLOCK_SIZE as u32, padbit as core::ffi::c_int);
        len -= POLY1305_BLOCK_SIZE as core::ffi::c_uint;
        src = src.add(POLY1305_BLOCK_SIZE as usize);
    }
    vsx_end();
}

unsafe fn poly1305_emit(
    state: *const poly1305_state,
    digest: *mut u8,
    nonce: *const u32,
) {
    if !static_key_enabled(&have_p10) {
        return poly1305_emit_generic(state, digest, nonce);
    }
    poly1305_emit_64(state, nonce, digest);
}

unsafe fn poly1305_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_31) {
        static_branch_enable(&mut have_p10);
    }
}

extern "C" {
    pub static CPU_FTR_ARCH_31: core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
