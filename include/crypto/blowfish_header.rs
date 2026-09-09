/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for blowfish algorithms
 */

// Dependencies supplied by the surrounding translation unit:
// <linux/types.h> and <linux/crypto.h>

pub const BF_BLOCK_SIZE: u32 = 8;
pub const BF_MIN_KEY_SIZE: u32 = 4;
pub const BF_MAX_KEY_SIZE: u32 = 56;

#[repr(C)]
pub struct bf_ctx {
    pub p: [u32; 18],
    pub s: [u32; 1024],
}

extern "C" {
    pub fn blowfish_setkey(
        tfm: *mut crypto_tfm,
        key: *const u8,
        key_len: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
