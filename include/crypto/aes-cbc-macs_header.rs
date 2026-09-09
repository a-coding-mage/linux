/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for AES-CMAC, AES-XCBC-MAC, and AES-CBC-MAC
 *
 * Copyright 2026 Google LLC
 */

/* C dependencies: crypto/aes.h and linux/string.h. */

#[repr(C)]
pub union AesCmacKFinal {
    pub b: [u8; AES_BLOCK_SIZE],
    pub w: [__be64; 2],
}

/// Prepared key for AES-CMAC or AES-XCBC-MAC.
#[repr(C)]
pub struct aes_cmac_key {
    pub aes: aes_enckey,
    /// Finalization subkeys for the final block. k_final[0] (CMAC K1,
    /// XCBC-MAC K2) is used if it's a full block. k_final[1] (CMAC K2,
    /// XCBC-MAC K3) is used if it's a partial block.
    pub k_final: [AesCmacKFinal; 2],
}

extern "C" {
    pub fn memzero_explicit(s: *mut core::ffi::c_void, count: usize);
    pub fn aes_cmac_preparekey(key: *mut aes_cmac_key, in_key: *const u8,
                               key_len: usize) -> core::ffi::c_int;
    pub fn aes_xcbcmac_preparekey(key: *mut aes_cmac_key,
                                  in_key: *const u8);
    pub fn aes_cmac_update(ctx: *mut aes_cmac_ctx, data: *const u8,
                           data_len: usize);
    pub fn aes_cmac_final(ctx: *mut aes_cmac_ctx, out: *mut u8);
    pub fn aes_cbcmac_update(ctx: *mut aes_cbcmac_ctx, data: *const u8,
                             data_len: usize);
    pub fn aes_cbcmac_final(ctx: *mut aes_cbcmac_ctx, out: *mut u8);
}

/// Zeroize an aes_cmac_key structure.
#[inline]
pub unsafe fn aes_cmac_zeroize_key(key: *mut aes_cmac_key) {
    memzero_explicit(key.cast(), core::mem::size_of::<aes_cmac_key>());
}

/// Context for computing an AES-CMAC or AES-XCBC-MAC value.
#[repr(C)]
pub struct aes_cmac_ctx {
    /// Pointer to the key struct. The key must live at least as long as the context.
    pub key: *const aes_cmac_key,
    /// Number of bytes XOR'ed into h since the last AES encryption.
    pub partial_len: usize,
    /// The current chaining value.
    pub h: [u8; AES_BLOCK_SIZE],
}

/// Zeroize an aes_cmac_ctx structure.
#[inline]
pub unsafe fn aes_cmac_zeroize_ctx(ctx: *mut aes_cmac_ctx) {
    memzero_explicit(ctx.cast(), core::mem::size_of::<aes_cmac_ctx>());
}

/// Start computing an AES-CMAC or AES-XCBC-MAC value.
#[inline]
pub unsafe fn aes_cmac_init(ctx: *mut aes_cmac_ctx, key: *const aes_cmac_key) {
    core::ptr::write(ctx, aes_cmac_ctx {
        key,
        partial_len: 0,
        h: [0; AES_BLOCK_SIZE],
    });
}

/// Compute AES-CMAC or AES-XCBC-MAC in one shot.
#[inline]
pub unsafe fn aes_cmac(key: *const aes_cmac_key, data: *const u8,
                       data_len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<aes_cmac_ctx>::uninit();
    aes_cmac_init(ctx.as_mut_ptr(), key);
    aes_cmac_update(ctx.as_mut_ptr(), data, data_len);
    aes_cmac_final(ctx.as_mut_ptr(), out);
}

/*
 * AES-CBC-MAC support. This is provided only for use by the implementation of
 * AES-CCM. It should have no other users. Warning: unlike AES-CMAC and
 * AES-XCBC-MAC, AES-CBC-MAC isn't a secure MAC for variable-length messages.
 */
#[repr(C)]
pub struct aes_cbcmac_ctx {
    pub key: *const aes_enckey,
    pub partial_len: usize,
    pub h: [u8; AES_BLOCK_SIZE],
}

#[inline]
pub unsafe fn aes_cbcmac_init(ctx: *mut aes_cbcmac_ctx, key: *const aes_enckey) {
    core::ptr::write(ctx, aes_cbcmac_ctx {
        key,
        partial_len: 0,
        h: [0; AES_BLOCK_SIZE],
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
