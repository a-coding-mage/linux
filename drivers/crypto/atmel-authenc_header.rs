/* SPDX-License-Identifier: GPL-2.0 */
/*
 * API for Atmel Secure Protocol Layers Improved Performances (SPLIP)
 *
 * Copyright (C) 2016 Atmel Corporation
 *
 * Author: Cyrille Pitchen <cyrille.pitchen@atmel.com>
 *
 * This driver is based on drivers/mtd/spi-nor/fsl-quadspi.c from Freescale.
 */

// Declarations below are enabled when CONFIG_CRYPTO_DEV_ATMEL_AUTHENC is enabled.

// Dependencies supplied by the surrounding kernel translation:
// crypto/authenc.h, crypto/hash.h, crypto/sha1.h, crypto/sha2.h,
// and atmel-sha-regs.h.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct atmel_aes_dev {
    _private: [u8; 0],
}

pub type atmel_aes_authenc_fn_t = Option<unsafe extern "C" fn(
    dev: *mut atmel_aes_dev,
    arg1: c_int,
    arg2: bool,
) -> c_int>;

#[repr(C)]
pub struct atmel_sha_authenc_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn atmel_sha_authenc_is_ready() -> bool;
    pub fn atmel_sha_authenc_get_reqsize() -> u32;

    pub fn atmel_sha_authenc_spawn(mode: c_ulong) -> *mut atmel_sha_authenc_ctx;
    pub fn atmel_sha_authenc_free(auth: *mut atmel_sha_authenc_ctx);
    pub fn atmel_sha_authenc_setkey(
        auth: *mut atmel_sha_authenc_ctx,
        key: *const u8,
        keylen: u32,
        flags: u32,
    ) -> c_int;

    pub fn atmel_sha_authenc_schedule(
        req: *mut ahash_request,
        auth: *mut atmel_sha_authenc_ctx,
        cb: atmel_aes_authenc_fn_t,
        dd: *mut atmel_aes_dev,
    ) -> c_int;
    pub fn atmel_sha_authenc_init(
        req: *mut ahash_request,
        assoc: *mut scatterlist,
        assoclen: u32,
        textlen: u32,
        cb: atmel_aes_authenc_fn_t,
        dd: *mut atmel_aes_dev,
    ) -> c_int;
    pub fn atmel_sha_authenc_final(
        req: *mut ahash_request,
        digest: *mut u32,
        digestlen: u32,
        cb: atmel_aes_authenc_fn_t,
        dd: *mut atmel_aes_dev,
    ) -> c_int;
    pub fn atmel_sha_authenc_abort(req: *mut ahash_request);
}

// External types supplied by crypto/hash.h and the kernel scatterlist API.
extern "C" {
    pub type ahash_request;
    pub type scatterlist;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
