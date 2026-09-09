/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM/SEC 4.x definitions for handling key-generation jobs
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 *
 */

/*
 * split_key_len - Compute MDHA split key length for a given algorithm
 * @hash: Hashing algorithm selection, one of OP_ALG_ALGSEL_* - MD5, SHA1,
 *        SHA224, SHA384, SHA512.
 *
 * Return: MDHA split key length
 */
#[inline]
pub fn split_key_len(hash: u32) -> u32 {
    /* Sizes for MDHA pads (*not* keys): MD5, SHA1, 224, 256, 384, 512 */
    const MDPADLEN: [u8; 6] = [16, 20, 32, 32, 64, 64];
    let idx: u32;

    idx = (hash & OP_ALG_ALGSEL_SUBMASK) >> OP_ALG_ALGSEL_SHIFT;

    unsafe { (MDPADLEN.get_unchecked(idx as usize) as *const u8).read() as u32 * 2 }
}

/*
 * split_key_pad_len - Compute MDHA split key pad length for a given algorithm
 * @hash: Hashing algorithm selection, one of OP_ALG_ALGSEL_* - MD5, SHA1,
 *        SHA224, SHA384, SHA512.
 *
 * Return: MDHA split key pad length
 */
#[inline]
pub fn split_key_pad_len(hash: u32) -> u32 {
    let len = split_key_len(hash);
    (len.wrapping_add(15)) & !15
}

pub struct completion;
pub struct device;
pub struct alginfo;

#[repr(C)]
pub struct split_key_result {
    pub completion: completion,
    pub err: core::ffi::c_int,
}

extern "C" {
    pub fn split_key_done(
        dev: *mut device,
        desc: *mut u32,
        err: u32,
        context: *mut core::ffi::c_void,
    );

    pub fn gen_split_key(
        jrdev: *mut device,
        key_out: *mut u8,
        adata: *const alginfo,
        key_in: *const u8,
        keylen: u32,
        max_keylen: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
