/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/types.h>, <linux/crypto.h>, <crypto/cast_common.h>

pub const CAST6_BLOCK_SIZE: usize = 16;
pub const CAST6_MIN_KEY_SIZE: usize = 16;
pub const CAST6_MAX_KEY_SIZE: usize = 32;

#[repr(C)]
pub struct cast6_ctx {
    pub Km: [[u32; 4]; 12],
    pub Kr: [[u8; 4]; 12],
}

// Opaque type supplied by the Linux crypto API dependency.
#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __cast6_setkey(ctx: *mut cast6_ctx, key: *const u8, keylen: u32) -> i32;
    pub fn cast6_setkey(tfm: *mut crypto_tfm, key: *const u8, keylen: u32) -> i32;

    pub fn __cast6_encrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn __cast6_decrypt(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
