/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OpenSSL/Cryptogams accelerated Poly1305 transform for arm64
 *
 * Copyright (C) 2019 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// Kernel headers and architecture feature definitions are supplied by the
// surrounding translation unit.

extern "C" {
    pub fn poly1305_block_init(
        state: *mut poly1305_block_state,
        raw_key: *const u8,
    );
    pub fn poly1305_blocks_arm64(
        state: *mut poly1305_block_state,
        src: *const u8,
        len: u32,
        hibit: u32,
    );
    pub fn poly1305_blocks_neon(
        state: *mut poly1305_block_state,
        src: *const u8,
        len: u32,
        hibit: u32,
    );
    pub fn poly1305_emit(
        state: *const poly1305_state,
        digest: *mut u8,
        nonce: *const u32,
    );
}

// These C types are defined by the surrounding Poly1305 implementation.
#[repr(C)]
pub struct poly1305_block_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poly1305_state {
    _private: [u8; 0],
}

// Equivalent storage for DEFINE_STATIC_KEY_FALSE(have_neon).
static mut HAVE_NEON: bool = false;

unsafe fn poly1305_blocks(
    state: *mut poly1305_block_state,
    src: *const u8,
    len: u32,
    padbit: u32,
) {
    // static_branch_likely(&have_neon) && likely(may_use_simd())
    // The scoped_ksimd() guard is a kernel SIMD-context primitive.
    if HAVE_NEON && may_use_simd() {
        poly1305_blocks_neon(state, src, len, padbit);
    } else {
        poly1305_blocks_arm64(state, src, len, padbit);
    }
}

// Supplied by the kernel SIMD support used by the original header.
extern "C" {
    fn may_use_simd() -> bool;
    fn cpu_have_named_feature(feature: u32) -> bool;
    fn static_branch_enable(key: *mut bool);
}

// #define poly1305_mod_init_arch poly1305_mod_init_arch
unsafe fn poly1305_mod_init_arch() {
    // ASIMD is the architecture feature used by the original C code.
    const ASIMD: u32 = 0;
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&raw mut HAVE_NEON);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
