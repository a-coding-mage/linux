/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4 common functions for Crypto Extensions
 * Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

unsafe extern "C" {
    pub fn sm4_ce_expand_key(
        key: *const u8,
        rkey_enc: *mut u32,
        rkey_dec: *mut u32,
        fk: *const u32,
        ck: *const u32,
    );

    pub fn sm4_ce_crypt_block(rkey: *const u32, dst: *mut u8, src: *const u8);

    pub fn sm4_ce_cbc_enc(
        rkey_enc: *const u32,
        dst: *mut u8,
        src: *const u8,
        iv: *mut u8,
        nblocks: ::core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
