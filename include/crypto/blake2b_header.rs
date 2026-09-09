/* SPDX-License-Identifier: GPL-2.0 OR MIT */

// C header dependencies: linux/bug.h, linux/types.h, linux/string.h

pub const BLAKE2B_BLOCK_SIZE: usize = 128;
pub const BLAKE2B_HASH_SIZE: usize = 64;
pub const BLAKE2B_KEY_SIZE: usize = 64;
pub const BLAKE2B_160_HASH_SIZE: usize = 20;
pub const BLAKE2B_256_HASH_SIZE: usize = 32;
pub const BLAKE2B_384_HASH_SIZE: usize = 48;
pub const BLAKE2B_512_HASH_SIZE: usize = 64;

/// Context for hashing a message with BLAKE2b.
#[repr(C)]
pub struct blake2b_ctx {
    // 'h', 't', and 'f' are used in assembly code, so keep them as-is.
    pub h: [u64; 8],
    pub t: [u64; 2],
    pub f: [u64; 2],
    pub buf: [u8; BLAKE2B_BLOCK_SIZE],
    pub buflen: u32,
    pub outlen: u32,
}

pub const BLAKE2B_IV0: u64 = 0x6A09E667F3BCC908;
pub const BLAKE2B_IV1: u64 = 0xBB67AE8584CAA73B;
pub const BLAKE2B_IV2: u64 = 0x3C6EF372FE94F82B;
pub const BLAKE2B_IV3: u64 = 0xA54FF53A5F1D36F1;
pub const BLAKE2B_IV4: u64 = 0x510E527FADE682D1;
pub const BLAKE2B_IV5: u64 = 0x9B05688C2B3E6C1F;
pub const BLAKE2B_IV6: u64 = 0x1F83D9ABFB41BD6B;
pub const BLAKE2B_IV7: u64 = 0x5BE0CD19137E2179;

#[inline]
pub unsafe fn __blake2b_init(
    ctx: *mut blake2b_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    (*ctx).h[0] = BLAKE2B_IV0 ^ (0x01010000u64 | ((keylen as u64) << 8) | outlen as u64);
    (*ctx).h[1] = BLAKE2B_IV1;
    (*ctx).h[2] = BLAKE2B_IV2;
    (*ctx).h[3] = BLAKE2B_IV3;
    (*ctx).h[4] = BLAKE2B_IV4;
    (*ctx).h[5] = BLAKE2B_IV5;
    (*ctx).h[6] = BLAKE2B_IV6;
    (*ctx).h[7] = BLAKE2B_IV7;
    (*ctx).t = [0; 2];
    (*ctx).f = [0; 2];
    (*ctx).buflen = 0;
    (*ctx).outlen = outlen as u32;
    if keylen != 0 {
        core::ptr::copy_nonoverlapping(key as *const u8, (*ctx).buf.as_mut_ptr(), keylen);
        core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(keylen), 0, BLAKE2B_BLOCK_SIZE - keylen);
        (*ctx).buflen = BLAKE2B_BLOCK_SIZE as u32;
    }
}

#[inline]
pub unsafe fn blake2b_init(ctx: *mut blake2b_ctx, outlen: usize) {
    __blake2b_init(ctx, outlen, core::ptr::null(), 0);
}

#[inline]
pub unsafe fn blake2b_init_key(
    ctx: *mut blake2b_ctx,
    outlen: usize,
    key: *const core::ffi::c_void,
    keylen: usize,
) {
    // WARN_ON(IS_ENABLED(DEBUG) && (...)) is supplied by the kernel build environment.
    __blake2b_init(ctx, outlen, key, keylen);
}

extern "C" {
    pub fn blake2b_update(ctx: *mut blake2b_ctx, input: *const u8, inlen: usize);
    pub fn blake2b_final(ctx: *mut blake2b_ctx, out: *mut u8);
}

#[inline]
pub unsafe fn blake2b(
    key: *const u8,
    keylen: usize,
    input: *const u8,
    inlen: usize,
    out: *mut u8,
    outlen: usize,
) {
    // WARN_ON(IS_ENABLED(DEBUG) && (...)) is supplied by the kernel build environment.
    let mut ctx: blake2b_ctx = core::mem::zeroed();
    __blake2b_init(&mut ctx, outlen, key as *const core::ffi::c_void, keylen);
    blake2b_update(&mut ctx, input, inlen);
    blake2b_final(&mut ctx, out);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
