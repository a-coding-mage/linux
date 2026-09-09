/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <crypto/b128ops.h>
// #include <linux/crypto.h>
// #include <linux/kernel.h>

pub const CAMELLIA_MIN_KEY_SIZE: usize = 16;
pub const CAMELLIA_MAX_KEY_SIZE: usize = 32;
pub const CAMELLIA_BLOCK_SIZE: usize = 16;
pub const CAMELLIA_TABLE_BYTE_LEN: usize = 272;
pub const CAMELLIA_PARALLEL_BLOCKS: usize = 2;

pub struct crypto_skcipher;

#[repr(C)]
pub struct camellia_ctx {
    pub key_table: [u64; CAMELLIA_TABLE_BYTE_LEN / core::mem::size_of::<u64>()],
    pub key_length: u32,
}

unsafe extern "C" {
    pub fn __camellia_setkey(
        cctx: *mut camellia_ctx,
        key: *const u8,
        key_len: u32,
    ) -> i32;

    /* regular block cipher functions */
    pub fn __camellia_enc_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, xor: bool);
    pub fn camellia_dec_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    /* 2-way parallel cipher functions */
    pub fn __camellia_enc_blk_2way(
        ctx: *const core::ffi::c_void,
        dst: *mut u8,
        src: *const u8,
        xor: bool,
    );
    pub fn camellia_dec_blk_2way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    /* 16-way parallel cipher functions (avx/aes-ni) */
    pub fn camellia_ecb_enc_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_ecb_dec_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    pub fn camellia_cbc_dec_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);

    /* glue helpers */
    pub fn camellia_decrypt_cbc_2way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

#[inline]
pub unsafe fn camellia_enc_blk(
    ctx: *const core::ffi::c_void,
    dst: *mut u8,
    src: *const u8,
) {
    unsafe { __camellia_enc_blk(ctx, dst, src, false) };
}

#[inline]
pub unsafe fn camellia_enc_blk_xor(
    ctx: *const core::ffi::c_void,
    dst: *mut u8,
    src: *const u8,
) {
    unsafe { __camellia_enc_blk(ctx, dst, src, true) };
}

#[inline]
pub unsafe fn camellia_enc_blk_2way(
    ctx: *const core::ffi::c_void,
    dst: *mut u8,
    src: *const u8,
) {
    unsafe { __camellia_enc_blk_2way(ctx, dst, src, false) };
}

#[inline]
pub unsafe fn camellia_enc_blk_xor_2way(
    ctx: *const core::ffi::c_void,
    dst: *mut u8,
    src: *const u8,
) {
    unsafe { __camellia_enc_blk_2way(ctx, dst, src, true) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
