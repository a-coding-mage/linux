// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * The ChaCha stream cipher (RFC7539)
 *
 * Copyright (C) 2015 Martin Willi
 */

// The following types, constants, and functions are supplied by the
// corresponding crypto headers and other translation units.
use crate::{chacha_state, CHACHA_BLOCK_SIZE, HCHACHA_OUT_WORDS};

extern "C" {
    fn chacha_block_generic(state: *mut chacha_state, stream: *mut u8, nrounds: i32);
    fn hchacha_block_generic(
        state: *const chacha_state,
        out: *mut u32,
        nrounds: i32,
    );
    fn crypto_xor_cpy(dst: *mut u8, src: *const u8, src2: *const u8, bytes: usize);
}

#[inline]
unsafe fn chacha_crypt_generic(
    state: *mut chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: u32,
    nrounds: i32,
) {
    /* aligned to potentially speed up crypto_xor() */
    #[repr(align(8))]
    struct AlignedStream([u8; CHACHA_BLOCK_SIZE]);
    let mut stream = AlignedStream([0; CHACHA_BLOCK_SIZE]);

    while bytes >= CHACHA_BLOCK_SIZE as u32 {
        chacha_block_generic(state, stream.0.as_mut_ptr(), nrounds);
        crypto_xor_cpy(dst, src, stream.0.as_ptr(), CHACHA_BLOCK_SIZE as usize);
        bytes -= CHACHA_BLOCK_SIZE as u32;
        dst = dst.add(CHACHA_BLOCK_SIZE as usize);
        src = src.add(CHACHA_BLOCK_SIZE as usize);
    }
    if bytes != 0 {
        chacha_block_generic(state, stream.0.as_mut_ptr(), nrounds);
        crypto_xor_cpy(dst, src, stream.0.as_ptr(), bytes as usize);
    }
}

// CONFIG_CRYPTO_LIB_CHACHA_ARCH selects architecture-specific implementations
// when enabled; otherwise the generic implementations are used.
#[cfg(feature = "CONFIG_CRYPTO_LIB_CHACHA_ARCH")]
extern "C" {
    fn chacha_crypt_arch(
        state: *mut chacha_state,
        dst: *mut u8,
        src: *const u8,
        bytes: u32,
        nrounds: i32,
    );
    fn hchacha_block_arch(state: *const chacha_state, out: *mut u32, nrounds: i32);
}

#[cfg(not(feature = "CONFIG_CRYPTO_LIB_CHACHA_ARCH"))]
unsafe fn chacha_crypt_arch(
    state: *mut chacha_state,
    dst: *mut u8,
    src: *const u8,
    bytes: u32,
    nrounds: i32,
) {
    chacha_crypt_generic(state, dst, src, bytes, nrounds);
}

#[cfg(not(feature = "CONFIG_CRYPTO_LIB_CHACHA_ARCH"))]
unsafe fn hchacha_block_arch(state: *const chacha_state, out: *mut u32, nrounds: i32) {
    hchacha_block_generic(state, out, nrounds);
}

pub unsafe fn chacha_crypt(
    state: *mut chacha_state,
    dst: *mut u8,
    src: *const u8,
    bytes: u32,
    nrounds: i32,
) {
    chacha_crypt_arch(state, dst, src, bytes, nrounds);
}

pub unsafe fn hchacha_block(
    state: *const chacha_state,
    out: *mut u32,
    nrounds: i32,
) {
    hchacha_block_arch(state, out, nrounds);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
