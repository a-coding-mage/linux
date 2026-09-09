/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Common values for ARC4 Cipher Algorithm
 */

// Dependency supplied by the surrounding translation unit: <linux/types.h>

pub const MD4_DIGEST_SIZE: usize = 16;
pub const MD4_HMAC_BLOCK_SIZE: usize = 64;
pub const MD4_BLOCK_WORDS: usize = 16;
pub const MD4_HASH_WORDS: usize = 4;

#[repr(C)]
pub struct md4_ctx {
    pub hash: [u32; MD4_HASH_WORDS],
    pub block: [u32; MD4_BLOCK_WORDS],
    pub byte_count: u64,
}

extern "C" {
    pub fn cifs_md4_init(mctx: *mut md4_ctx) -> ::core::ffi::c_int;
    pub fn cifs_md4_update(
        mctx: *mut md4_ctx,
        data: *const u8,
        len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn cifs_md4_final(mctx: *mut md4_ctx, out: *mut u8) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
