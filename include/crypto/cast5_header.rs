/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
//   linux/types.h, linux/crypto.h, crypto/cast_common.h

pub const CAST5_BLOCK_SIZE: usize = 8;
pub const CAST5_MIN_KEY_SIZE: usize = 5;
pub const CAST5_MAX_KEY_SIZE: usize = 16;

#[repr(C)]
pub struct cast5_ctx {
    pub Km: [u32; 16],
    pub Kr: [u8; 16],
    // rr ? rounds = 12 : rounds = 16; (rfc 2144)
    pub rr: i32,
}

// Declaration supplied by linux/crypto.h.
pub enum crypto_tfm {}

unsafe extern "C" {
    pub fn cast5_setkey(
        tfm: *mut crypto_tfm,
        key: *const u8,
        keylen: u32,
    ) -> i32;

    pub fn __cast5_encrypt(
        ctx: *mut cast5_ctx,
        dst: *mut u8,
        src: *const u8,
    );

    pub fn __cast5_decrypt(
        ctx: *mut cast5_ctx,
        dst: *mut u8,
        src: *const u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
