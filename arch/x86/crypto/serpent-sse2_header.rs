/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/crypto.h and crypto/serpent.h

// CONFIG_X86_32 selects the 4-way implementation; other targets use 8-way.
#[cfg(target_arch = "x86")]
pub const SERPENT_PARALLEL_BLOCKS: usize = 4;

#[cfg(target_arch = "x86")]
extern "C" {
    pub fn __serpent_enc_blk_4way(
        ctx: *const serpent_ctx,
        dst: *mut u8,
        src: *const u8,
        xor: bool,
    );
    pub fn serpent_dec_blk_4way(ctx: *const serpent_ctx, dst: *mut u8, src: *const u8);
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn serpent_enc_blk_xway(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    __serpent_enc_blk_4way(ctx as *const serpent_ctx, dst, src, false);
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn serpent_enc_blk_xway_xor(
    ctx: *const serpent_ctx,
    dst: *mut u8,
    src: *const u8,
) {
    __serpent_enc_blk_4way(ctx, dst, src, true);
}

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn serpent_dec_blk_xway(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    serpent_dec_blk_4way(ctx as *const serpent_ctx, dst, src);
}

#[cfg(not(target_arch = "x86"))]
pub const SERPENT_PARALLEL_BLOCKS: usize = 8;

#[cfg(not(target_arch = "x86"))]
extern "C" {
    pub fn __serpent_enc_blk_8way(
        ctx: *const serpent_ctx,
        dst: *mut u8,
        src: *const u8,
        xor: bool,
    );
    pub fn serpent_dec_blk_8way(ctx: *const serpent_ctx, dst: *mut u8, src: *const u8);
}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub unsafe fn serpent_enc_blk_xway(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    __serpent_enc_blk_8way(ctx as *const serpent_ctx, dst, src, false);
}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub unsafe fn serpent_enc_blk_xway_xor(
    ctx: *const serpent_ctx,
    dst: *mut u8,
    src: *const u8,
) {
    __serpent_enc_blk_8way(ctx, dst, src, true);
}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub unsafe fn serpent_dec_blk_xway(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    serpent_dec_blk_8way(ctx as *const serpent_ctx, dst, src);
}

// Opaque type supplied by crypto/serpent.h.
#[allow(non_camel_case_types)]
pub enum serpent_ctx {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
