/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OpenSSL/Cryptogams accelerated Poly1305 transform for MIPS
 *
 * Copyright (C) 2019 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// `asmlinkage` is a C calling-convention annotation; the declarations below
// use the platform C ABI.
extern "C" {
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
