/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for serpent algorithms
 */

// The Linux type and crypto headers are C dependencies of this interface.

pub const SERPENT_MIN_KEY_SIZE: usize = 0;
pub const SERPENT_MAX_KEY_SIZE: usize = 32;
pub const SERPENT_EXPKEY_WORDS: usize = 132;
pub const SERPENT_BLOCK_SIZE: usize = 16;

#[repr(C)]
pub struct serpent_ctx {
    pub expkey: [u32; SERPENT_EXPKEY_WORDS],
}

// Opaque type supplied by the Linux crypto dependency.
pub enum crypto_tfm {}

unsafe extern "C" {
    pub fn __serpent_setkey(
        ctx: *mut serpent_ctx,
        key: *const u8,
        keylen: u32,
    ) -> core::ffi::c_int;

    pub fn serpent_setkey(
        tfm: *mut crypto_tfm,
        key: *const u8,
        keylen: u32,
    ) -> core::ffi::c_int;

    pub fn __serpent_encrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn __serpent_decrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
