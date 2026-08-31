/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

/* C header dependencies: <string.h>, <linux/types.h> */

pub const BLAKE2S_BLOCK_SIZE: usize = 64;

#[repr(C)]
pub struct blake2s_ctx {
    pub h: [u32; 8],
    pub t: [u32; 2],
    pub f: [u32; 2],
    pub buf: [u8; BLAKE2S_BLOCK_SIZE],
    pub buflen: core::ffi::c_uint,
    pub outlen: core::ffi::c_uint,
}

#[repr(u32)]
pub enum blake2s_iv {
    BLAKE2S_IV0 = 0x6A09E667,
    BLAKE2S_IV1 = 0xBB67AE85,
    BLAKE2S_IV2 = 0x3C6EF372,
    BLAKE2S_IV3 = 0xA54FF53A,
    BLAKE2S_IV4 = 0x510E527F,
    BLAKE2S_IV5 = 0x9B05688C,
    BLAKE2S_IV6 = 0x1F83D9AB,
    BLAKE2S_IV7 = 0x5BE0CD19,
}

pub unsafe fn __blake2s_init(
    ctx: *mut blake2s_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    unsafe {
        (*ctx).h[0] = (blake2s_iv::BLAKE2S_IV0 as u32)
            ^ (0x01010000usize | (keylen << 8) | outlen) as u32;
        (*ctx).h[1] = blake2s_iv::BLAKE2S_IV1 as u32;
        (*ctx).h[2] = blake2s_iv::BLAKE2S_IV2 as u32;
        (*ctx).h[3] = blake2s_iv::BLAKE2S_IV3 as u32;
        (*ctx).h[4] = blake2s_iv::BLAKE2S_IV4 as u32;
        (*ctx).h[5] = blake2s_iv::BLAKE2S_IV5 as u32;
        (*ctx).h[6] = blake2s_iv::BLAKE2S_IV6 as u32;
        (*ctx).h[7] = blake2s_iv::BLAKE2S_IV7 as u32;
        (*ctx).t[0] = 0;
        (*ctx).t[1] = 0;
        (*ctx).f[0] = 0;
        (*ctx).f[1] = 0;
        (*ctx).buflen = 0;
        (*ctx).outlen = outlen as core::ffi::c_uint;
        if keylen != 0 {
            core::ptr::copy_nonoverlapping(key as *const u8, (*ctx).buf.as_mut_ptr(), keylen);
            core::ptr::write_bytes(
                (*ctx).buf.as_mut_ptr().add(keylen),
                0,
                BLAKE2S_BLOCK_SIZE - keylen,
            );
            (*ctx).buflen = BLAKE2S_BLOCK_SIZE as core::ffi::c_uint;
        }
    }
}

pub unsafe fn blake2s_init(ctx: *mut blake2s_ctx, outlen: usize) {
    unsafe {
        __blake2s_init(ctx, outlen, core::ptr::null(), 0);
    }
}

pub unsafe fn blake2s_init_key(
    ctx: *mut blake2s_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    unsafe {
        __blake2s_init(ctx, outlen, key, keylen);
    }
}

unsafe extern "C" {
    pub fn blake2s_update(ctx: *mut blake2s_ctx, in_: *const u8, inlen: usize);

    pub fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8);
}
