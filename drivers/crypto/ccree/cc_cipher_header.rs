/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_cipher.h
 * ARM CryptoCell Cipher Crypto API
 */

/* Dependencies supplied by the surrounding kernel translation unit:
 * linux/kernel.h, crypto/algapi.h, cc_driver.h, and cc_buffer_mgr.h.
 */

#[repr(C)]
pub struct cipher_req_ctx {
    pub gen_ctx: async_gen_req_ctx,
    pub dma_buf_type: cc_req_dma_buf_type,
    pub in_nents: u32,
    pub in_mlli_nents: u32,
    pub out_nents: u32,
    pub out_mlli_nents: u32,
    pub iv: *mut u8,
    pub mlli_params: mlli_params,
}

extern "C" {
    pub fn cc_cipher_alloc(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;

    pub fn cc_cipher_free(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;
}

#[repr(C, packed)]
pub struct cc_hkey_info {
    pub keylen: u16,
    pub hw_key1: u8,
    pub hw_key2: u8,
}

pub const CC_HW_KEY_SIZE: usize = ::core::mem::size_of::<cc_hkey_info>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
