// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MD5 and HMAC-MD5 library functions
 *
 * md5_block() is derived from cryptoapi implementation, originally based on the
 * public domain implementation written by Colin Plumb in 1993.
 *
 * Copyright (c) Cryptoapi developers.
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright 2025 Google LLC
 */

// C dependencies: crypto/hmac.h, crypto/md5.h, linux/export.h, linux/kernel.h,
// linux/module.h, linux/string.h, linux/unaligned.h, and linux/wordpart.h.

const MD5_H0: u32 = 0x67452301;
const MD5_H1: u32 = 0xefcdab89;
const MD5_H2: u32 = 0x98badcfe;
const MD5_H3: u32 = 0x10325476;
const MD5_BLOCK_SIZE: usize = 64;
const MD5_BLOCK_WORDS: usize = 16;
const MD5_DIGEST_SIZE: usize = 16;
const HMAC_IPAD_VALUE: u8 = 0x36;
const HMAC_OPAD_VALUE: u8 = 0x5c;

#[inline]
fn f1(x: u32, y: u32, z: u32) -> u32 { z ^ (x & (y ^ z)) }
#[inline]
fn f2(x: u32, y: u32, z: u32) -> u32 { f1(z, x, y) }
#[inline]
fn f3(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
#[inline]
fn f4(x: u32, y: u32, z: u32) -> u32 { y ^ (x | !z) }

macro_rules! md5step {
    ($f:ident, $w:ident, $x:ident, $y:ident, $z:ident, $input:expr, $s:expr) => {
        $w = $w.wrapping_add($f($x, $y, $z)).wrapping_add($input);
        $w = $w.rotate_left($s).wrapping_add($x);
    };
}

#[inline]
unsafe fn md5_block(state: *mut md5_block_state, data: *const u8) {
    let mut input = [0u32; MD5_BLOCK_WORDS];
    for i in 0..MD5_BLOCK_WORDS {
        let p = data.add(i * 4);
        input[i] = u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)]);
    }
    let mut a = (*state).h[0];
    let mut b = (*state).h[1];
    let mut c = (*state).h[2];
    let mut d = (*state).h[3];

    md5step!(f1, a, b, c, d, input[0].wrapping_add(0xd76aa478), 7); md5step!(f1, d, a, b, c, input[1].wrapping_add(0xe8c7b756), 12);
    md5step!(f1, c, d, a, b, input[2].wrapping_add(0x242070db), 17); md5step!(f1, b, c, d, a, input[3].wrapping_add(0xc1bdceee), 22);
    md5step!(f1, a, b, c, d, input[4].wrapping_add(0xf57c0faf), 7); md5step!(f1, d, a, b, c, input[5].wrapping_add(0x4787c62a), 12);
    md5step!(f1, c, d, a, b, input[6].wrapping_add(0xa8304613), 17); md5step!(f1, b, c, d, a, input[7].wrapping_add(0xfd469501), 22);
    md5step!(f1, a, b, c, d, input[8].wrapping_add(0x698098d8), 7); md5step!(f1, d, a, b, c, input[9].wrapping_add(0x8b44f7af), 12);
    md5step!(f1, c, d, a, b, input[10].wrapping_add(0xffff5bb1), 17); md5step!(f1, b, c, d, a, input[11].wrapping_add(0x895cd7be), 22);
    md5step!(f1, a, b, c, d, input[12].wrapping_add(0x6b901122), 7); md5step!(f1, d, a, b, c, input[13].wrapping_add(0xfd987193), 12);
    md5step!(f1, c, d, a, b, input[14].wrapping_add(0xa679438e), 17); md5step!(f1, b, c, d, a, input[15].wrapping_add(0x49b40821), 22);

    md5step!(f2, a, b, c, d, input[1].wrapping_add(0xf61e2562), 5); md5step!(f2, d, a, b, c, input[6].wrapping_add(0xc040b340), 9);
    md5step!(f2, c, d, a, b, input[11].wrapping_add(0x265e5a51), 14); md5step!(f2, b, c, d, a, input[0].wrapping_add(0xe9b6c7aa), 20);
    md5step!(f2, a, b, c, d, input[5].wrapping_add(0xd62f105d), 5); md5step!(f2, d, a, b, c, input[10].wrapping_add(0x02441453), 9);
    md5step!(f2, c, d, a, b, input[15].wrapping_add(0xd8a1e681), 14); md5step!(f2, b, c, d, a, input[4].wrapping_add(0xe7d3fbc8), 20);
    md5step!(f2, a, b, c, d, input[9].wrapping_add(0x21e1cde6), 5); md5step!(f2, d, a, b, c, input[14].wrapping_add(0xc33707d6), 9);
    md5step!(f2, c, d, a, b, input[3].wrapping_add(0xf4d50d87), 14); md5step!(f2, b, c, d, a, input[8].wrapping_add(0x455a14ed), 20);
    md5step!(f2, a, b, c, d, input[13].wrapping_add(0xa9e3e905), 5); md5step!(f2, d, a, b, c, input[2].wrapping_add(0xfcefa3f8), 9);
    md5step!(f2, c, d, a, b, input[7].wrapping_add(0x676f02d9), 14); md5step!(f2, b, c, d, a, input[12].wrapping_add(0x8d2a4c8a), 20);

    md5step!(f3, a, b, c, d, input[5].wrapping_add(0xfffa3942), 4); md5step!(f3, d, a, b, c, input[8].wrapping_add(0x8771f681), 11);
    md5step!(f3, c, d, a, b, input[11].wrapping_add(0x6d9d6122), 16); md5step!(f3, b, c, d, a, input[14].wrapping_add(0xfde5380c), 23);
    md5step!(f3, a, b, c, d, input[1].wrapping_add(0xa4beea44), 4); md5step!(f3, d, a, b, c, input[4].wrapping_add(0x4bdecfa9), 11);
    md5step!(f3, c, d, a, b, input[7].wrapping_add(0xf6bb4b60), 16); md5step!(f3, b, c, d, a, input[10].wrapping_add(0xbebfbc70), 23);
    md5step!(f3, a, b, c, d, input[13].wrapping_add(0x289b7ec6), 4); md5step!(f3, d, a, b, c, input[0].wrapping_add(0xeaa127fa), 11);
    md5step!(f3, c, d, a, b, input[3].wrapping_add(0xd4ef3085), 16); md5step!(f3, b, c, d, a, input[6].wrapping_add(0x04881d05), 23);
    md5step!(f3, a, b, c, d, input[9].wrapping_add(0xd9d4d039), 4); md5step!(f3, d, a, b, c, input[12].wrapping_add(0xe6db99e5), 11);
    md5step!(f3, c, d, a, b, input[15].wrapping_add(0x1fa27cf8), 16); md5step!(f3, b, c, d, a, input[2].wrapping_add(0xc4ac5665), 23);

    md5step!(f4, a, b, c, d, input[0].wrapping_add(0xf4292244), 6); md5step!(f4, d, a, b, c, input[7].wrapping_add(0x432aff97), 10);
    md5step!(f4, c, d, a, b, input[14].wrapping_add(0xab9423a7), 15); md5step!(f4, b, c, d, a, input[5].wrapping_add(0xfc93a039), 21);
    md5step!(f4, a, b, c, d, input[12].wrapping_add(0x655b59c3), 6); md5step!(f4, d, a, b, c, input[3].wrapping_add(0x8f0ccc92), 10);
    md5step!(f4, c, d, a, b, input[10].wrapping_add(0xffeff47d), 15); md5step!(f4, b, c, d, a, input[1].wrapping_add(0x85845dd1), 21);
    md5step!(f4, a, b, c, d, input[8].wrapping_add(0x6fa87e4f), 6); md5step!(f4, d, a, b, c, input[15].wrapping_add(0xfe2ce6e0), 10);
    md5step!(f4, c, d, a, b, input[6].wrapping_add(0xa3014314), 15); md5step!(f4, b, c, d, a, input[13].wrapping_add(0x4e0811a1), 21);
    md5step!(f4, a, b, c, d, input[4].wrapping_add(0xf7537e82), 6); md5step!(f4, d, a, b, c, input[11].wrapping_add(0xbd3af235), 10);
    md5step!(f4, c, d, a, b, input[2].wrapping_add(0x2ad7d2bb), 15); md5step!(f4, b, c, d, a, input[9].wrapping_add(0xeb86d391), 21);

    (*state).h[0] = (*state).h[0].wrapping_add(a); (*state).h[1] = (*state).h[1].wrapping_add(b);
    (*state).h[2] = (*state).h[2].wrapping_add(c); (*state).h[3] = (*state).h[3].wrapping_add(d);
}

static mut md5_iv: md5_block_state = md5_block_state { h: [MD5_H0, MD5_H1, MD5_H2, MD5_H3] };

unsafe fn md5_blocks(state: *mut md5_block_state, mut data: *const u8, mut nblocks: usize) {
    loop { md5_block(state, data); data = data.add(MD5_BLOCK_SIZE); nblocks -= 1; if nblocks == 0 { break; } }
}

pub unsafe fn md5_init(ctx: *mut md5_ctx) { (*ctx).state = md5_iv; (*ctx).bytecount = 0; }

pub unsafe fn md5_update(ctx: *mut md5_ctx, mut data: *const u8, mut len: usize) {
    let mut partial = (*ctx).bytecount % MD5_BLOCK_SIZE;
    (*ctx).bytecount += len;
    if partial + len >= MD5_BLOCK_SIZE {
        if partial != 0 { let l = MD5_BLOCK_SIZE - partial; core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), l); data = data.add(l); len -= l; md5_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1); }
        let nblocks = len / MD5_BLOCK_SIZE; len %= MD5_BLOCK_SIZE;
        if nblocks != 0 { md5_blocks(&mut (*ctx).state, data, nblocks); data = data.add(nblocks * MD5_BLOCK_SIZE); }
        partial = 0;
    }
    if len != 0 { core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), len); }
}

unsafe fn md5_final_inner(ctx: *mut md5_ctx, out: *mut u8) {
    let bitcount = (*ctx).bytecount.wrapping_shl(3);
    let mut partial = (*ctx).bytecount % MD5_BLOCK_SIZE;
    (*ctx).buf[partial] = 0x80; partial += 1;
    if partial > MD5_BLOCK_SIZE - 8 { (*ctx).buf[partial..MD5_BLOCK_SIZE].fill(0); md5_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1); partial = 0; }
    (*ctx).buf[partial..MD5_BLOCK_SIZE - 8].fill(0);
    (*ctx).buf[MD5_BLOCK_SIZE - 8..].copy_from_slice(&bitcount.to_le_bytes());
    md5_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1);
    for x in &mut (*ctx).state.h { *x = x.to_le(); }
    core::ptr::copy_nonoverlapping((*ctx).state.h.as_ptr() as *const u8, out, MD5_DIGEST_SIZE);
}

pub unsafe fn md5_final(ctx: *mut md5_ctx, out: *mut u8) { md5_final_inner(ctx, out); core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<md5_ctx>()); }

pub unsafe fn md5(data: *const u8, len: usize, out: *mut u8) { let mut ctx: md5_ctx = core::mem::zeroed(); md5_init(&mut ctx); md5_update(&mut ctx, data, len); md5_final(&mut ctx, out); }

unsafe fn __hmac_md5_preparekey(istate: *mut md5_block_state, ostate: *mut md5_block_state, raw_key: *const u8, raw_key_len: usize) {
    let mut key = [0u8; MD5_BLOCK_SIZE];
    if raw_key_len > MD5_BLOCK_SIZE { md5(raw_key, raw_key_len, key.as_mut_ptr()); } else { core::ptr::copy_nonoverlapping(raw_key, key.as_mut_ptr(), raw_key_len); }
    for x in &mut key { *x ^= HMAC_IPAD_VALUE; }
    *istate = md5_iv; md5_blocks(istate, key.as_ptr(), 1);
    for x in &mut key { *x ^= HMAC_OPAD_VALUE ^ HMAC_IPAD_VALUE; }
    *ostate = md5_iv; md5_blocks(ostate, key.as_ptr(), 1);
    core::ptr::write_bytes(key.as_mut_ptr(), 0, key.len());
}

pub unsafe fn hmac_md5_preparekey(key: *mut hmac_md5_key, raw_key: *const u8, raw_key_len: usize) {
    __hmac_md5_preparekey(&mut (*key).istate, &mut (*key).ostate, raw_key, raw_key_len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
