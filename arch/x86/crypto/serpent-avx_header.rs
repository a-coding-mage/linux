/* SPDX-License-Identifier: GPL-2.0 */

// Translated from serpent-avx.h.
//
// C dependencies:
//   <crypto/b128ops.h>
//   <crypto/serpent.h>
//   <linux/types.h>

/// Forward declaration of the kernel skcipher type.
#[repr(C)]
pub struct CryptoSkcipher {
    _private: [u8; 0],
}

pub const SERPENT_PARALLEL_BLOCKS: usize = 8;

unsafe extern "C" {
    pub fn serpent_ecb_enc_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_ecb_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn serpent_cbc_dec_8way_avx(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
