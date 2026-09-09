/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values and helper functions for the ChaCha and XChaCha stream ciphers.
 *
 * XChaCha extends ChaCha's nonce to 192 bits, while provably retaining ChaCha's
 * security.  Here they share the same key size, tfm context, and setkey
 * function; only their IV size and encrypt/decrypt function differ.
 *
 * The ChaCha paper specifies 20, 12, and 8-round variants.  In general, it is
 * recommended to use the 20-round variant ChaCha20.  However, the other
 * variants can be needed in some performance-sensitive scenarios.  The generic
 * ChaCha code currently allows only the 20 and 12-round variants.
 */

// C header dependencies: linux/unaligned.h, linux/string.h, linux/types.h

/* 32-bit stream position, then 96-bit nonce (RFC7539 convention) */
pub const CHACHA_IV_SIZE: usize = 16;

pub const CHACHA_KEY_SIZE: usize = 32;
pub const CHACHA_BLOCK_SIZE: usize = 64;
pub const CHACHAPOLY_IV_SIZE: usize = 12;

pub const CHACHA_KEY_WORDS: usize = 8;
pub const CHACHA_STATE_WORDS: usize = 16;
pub const HCHACHA_OUT_WORDS: usize = 8;

/* 192-bit nonce, then 64-bit stream position */
pub const XCHACHA_IV_SIZE: usize = 32;

#[repr(C)]
pub struct chacha_state {
    pub x: [u32; CHACHA_STATE_WORDS],
}

extern "C" {
    pub fn chacha_block_generic(
        state: *mut chacha_state,
        out: *mut u8,
        nrounds: i32,
    );

    pub fn hchacha_block_generic(
        state: *const chacha_state,
        out: *mut u32,
        nrounds: i32,
    );

    pub fn hchacha_block(
        state: *const chacha_state,
        out: *mut u32,
        nrounds: i32,
    );

    pub fn chacha_crypt(
        state: *mut chacha_state,
        dst: *mut u8,
        src: *const u8,
        bytes: u32,
        nrounds: i32,
    );

    pub fn get_unaligned_le32(ptr: *const u8) -> u32;
    pub fn memzero_explicit(ptr: *mut core::ffi::c_void, size: usize);
}

pub unsafe fn chacha20_block(state: *mut chacha_state, out: *mut u8) {
    chacha_block_generic(state, out, 20);
}

#[repr(i32)]
pub enum chacha_constants {
    /* expand 32-byte k */
    CHACHA_CONSTANT_EXPA = 0x61707865,
    CHACHA_CONSTANT_ND_3 = 0x3320646e,
    CHACHA_CONSTANT_2_BY = 0x79622d32,
    CHACHA_CONSTANT_TE_K = 0x6b206574,
}

pub unsafe fn chacha_init_consts(state: *mut chacha_state) {
    (*state).x[0] = chacha_constants::CHACHA_CONSTANT_EXPA as u32;
    (*state).x[1] = chacha_constants::CHACHA_CONSTANT_ND_3 as u32;
    (*state).x[2] = chacha_constants::CHACHA_CONSTANT_2_BY as u32;
    (*state).x[3] = chacha_constants::CHACHA_CONSTANT_TE_K as u32;
}

pub unsafe fn chacha_init(
    state: *mut chacha_state,
    key: *const u32,
    iv: *const u8,
) {
    chacha_init_consts(state);
    (*state).x[4] = *key.add(0);
    (*state).x[5] = *key.add(1);
    (*state).x[6] = *key.add(2);
    (*state).x[7] = *key.add(3);
    (*state).x[8] = *key.add(4);
    (*state).x[9] = *key.add(5);
    (*state).x[10] = *key.add(6);
    (*state).x[11] = *key.add(7);
    (*state).x[12] = get_unaligned_le32(iv.add(0));
    (*state).x[13] = get_unaligned_le32(iv.add(4));
    (*state).x[14] = get_unaligned_le32(iv.add(8));
    (*state).x[15] = get_unaligned_le32(iv.add(12));
}

pub unsafe fn chacha20_crypt(
    state: *mut chacha_state,
    dst: *mut u8,
    src: *const u8,
    bytes: u32,
) {
    chacha_crypt(state, dst, src, bytes, 20);
}

pub unsafe fn chacha_zeroize_state(state: *mut chacha_state) {
    memzero_explicit(
        state.cast::<core::ffi::c_void>(),
        core::mem::size_of::<chacha_state>(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
