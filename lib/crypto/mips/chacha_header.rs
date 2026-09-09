/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChaCha and HChaCha functions (MIPS optimized)
 *
 * Copyright (C) 2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 */

// Dependency supplied by another translation unit: struct chacha_state.
#[repr(C)]
pub struct chacha_state {
    _private: [u8; 0],
}

// Dependency supplied by another translation unit: HCHACHA_OUT_WORDS.
extern "C" {
    pub fn chacha_crypt_arch(
        state: *mut chacha_state,
        dst: *mut u8,
        src: *const u8,
        bytes: core::ffi::c_uint,
        nrounds: core::ffi::c_int,
    );

    pub fn hchacha_block_arch(
        state: *const chacha_state,
        out: *mut u32,
        nrounds: core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
