/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Translated from the C header. Linux build-time configuration and diagnostics
// remain external to this translation.

#[repr(u32)]
pub enum Blake2sLengths {
    BLAKE2S_BLOCK_SIZE = 64,
    BLAKE2S_HASH_SIZE = 32,
    BLAKE2S_KEY_SIZE = 32,
    BLAKE2S_128_HASH_SIZE = 16,
    BLAKE2S_160_HASH_SIZE = 20,
    BLAKE2S_224_HASH_SIZE = 28,
    BLAKE2S_256_HASH_SIZE = 32,
}

#[repr(C)]
pub struct blake2s_ctx {
    // 'h', 't', and 'f' are used in assembly code, so keep them as-is.
    pub h: [u32; 8],
    pub t: [u32; 2],
    pub f: [u32; 2],
    pub buf: [u8; BLAKE2S_BLOCK_SIZE],
    pub buflen: u32,
    pub outlen: usize,
}

#[repr(u32)]
pub enum Blake2sIv {
    BLAKE2S_IV0 = 0x6A09E667,
    BLAKE2S_IV1 = 0xBB67AE85,
    BLAKE2S_IV2 = 0x3C6EF372,
    BLAKE2S_IV3 = 0xA54FF53A,
    BLAKE2S_IV4 = 0x510E527F,
    BLAKE2S_IV5 = 0x9B05688C,
    BLAKE2S_IV6 = 0x1F83D9AB,
    BLAKE2S_IV7 = 0x5BE0CD19,
}

pub const BLAKE2S_BLOCK_SIZE: usize = 64;
pub const BLAKE2S_HASH_SIZE: usize = 32;
pub const BLAKE2S_KEY_SIZE: usize = 32;
pub const BLAKE2S_128_HASH_SIZE: usize = 16;
pub const BLAKE2S_160_HASH_SIZE: usize = 20;
pub const BLAKE2S_224_HASH_SIZE: usize = 28;
pub const BLAKE2S_256_HASH_SIZE: usize = 32;

pub unsafe fn __blake2s_init(
    ctx: *mut blake2s_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    (*ctx).h[0] = (BLAKE2S_IV0 as u32) ^ (0x01010000u32 | ((keylen as u32) << 8) | outlen as u32);
    (*ctx).h[1] = BLAKE2S_IV1 as u32;
    (*ctx).h[2] = BLAKE2S_IV2 as u32;
    (*ctx).h[3] = BLAKE2S_IV3 as u32;
    (*ctx).h[4] = BLAKE2S_IV4 as u32;
    (*ctx).h[5] = BLAKE2S_IV5 as u32;
    (*ctx).h[6] = BLAKE2S_IV6 as u32;
    (*ctx).h[7] = BLAKE2S_IV7 as u32;
    (*ctx).t = [0; 2];
    (*ctx).f = [0; 2];
    (*ctx).buflen = 0;
    (*ctx).outlen = outlen;
    if keylen != 0 {
        core::ptr::copy_nonoverlapping(key as *const u8, (*ctx).buf.as_mut_ptr(), keylen);
        core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(keylen), 0, BLAKE2S_BLOCK_SIZE - keylen);
        (*ctx).buflen = BLAKE2S_BLOCK_SIZE as u32;
    }
}

pub unsafe fn blake2s_init(ctx: *mut blake2s_ctx, outlen: usize) {
    __blake2s_init(ctx, outlen, core::ptr::null(), 0);
}

pub unsafe fn blake2s_init_key(
    ctx: *mut blake2s_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    // Equivalent to the source's WARN_ON(IS_ENABLED(DEBUG) && condition).
    debug_assert!(outlen != 0 && outlen <= BLAKE2S_HASH_SIZE && !key.is_null() && keylen != 0 && keylen <= BLAKE2S_KEY_SIZE);
    __blake2s_init(ctx, outlen, key, keylen);
}

extern "C" {
    pub fn blake2s_update(ctx: *mut blake2s_ctx, input: *const u8, inlen: usize);
    pub fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8);
}

pub unsafe fn blake2s(
    key: *const u8,
    keylen: usize,
    input: *const u8,
    inlen: usize,
    out: *mut u8,
    outlen: usize,
) {
    // Equivalent to the source's WARN_ON(IS_ENABLED(DEBUG) && condition).
    debug_assert!((!input.is_null() || inlen == 0) && !out.is_null() && outlen != 0 && outlen <= BLAKE2S_HASH_SIZE && keylen <= BLAKE2S_KEY_SIZE && (!key.is_null() || keylen == 0));
    let mut ctx = core::mem::MaybeUninit::<blake2s_ctx>::uninit();
    __blake2s_init(ctx.as_mut_ptr(), outlen, key as *const core::ffi::c_void, keylen);
    blake2s_update(ctx.as_mut_ptr(), input, inlen);
    blake2s_final(ctx.as_mut_ptr(), out);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
