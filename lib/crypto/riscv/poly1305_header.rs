/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OpenSSL/Cryptogams accelerated Poly1305 transform for riscv
 *
 * Copyright (C) 2025 Institute of Software, CAS.
 */

// `asmlinkage` is a C calling-convention annotation with no direct Rust
// equivalent; preserve the external declarations and their ABI intent.

pub struct poly1305_block_state;
pub struct poly1305_state;

unsafe extern "C" {
    pub fn poly1305_block_init(
        state: *mut poly1305_block_state,
        raw_key: *const u8,
    );

    pub fn poly1305_blocks(
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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
