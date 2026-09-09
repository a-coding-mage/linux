// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 * Copyright 2025 Google LLC
 *
 * This is an implementation of the BLAKE2b hash and PRF functions.
 *
 * Information: https://blake2.net/
 */

use crate::{blake2b_ctx, BLAKE2B_BLOCK_SIZE, BLAKE2B_IV0, BLAKE2B_IV1,
    BLAKE2B_IV2, BLAKE2B_IV3, BLAKE2B_IV4, BLAKE2B_IV5, BLAKE2B_IV6,
    BLAKE2B_IV7};

static BLAKE2B_SIGMA: [[u8; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

#[inline]
unsafe fn blake2b_increment_counter(ctx: *mut blake2b_ctx, inc: u32) {
    (*ctx).t[0] = (*ctx).t[0].wrapping_add(inc);
    (*ctx).t[1] = (*ctx).t[1].wrapping_add(((*ctx).t[0] < inc) as u32);
}

unsafe fn blake2b_compress_generic(ctx: *mut blake2b_ctx, mut data: *const u8,
                                   mut nblocks: usize, inc: u32) {
    let mut m = [0u64; 16];
    let mut v = [0u64; 16];
    while nblocks > 0 {
        blake2b_increment_counter(ctx, inc);
        core::ptr::copy_nonoverlapping(data, m.as_mut_ptr() as *mut u8, BLAKE2B_BLOCK_SIZE);
        for x in &mut m { *x = u64::from_le(*x); }
        core::ptr::copy_nonoverlapping((*ctx).h.as_ptr(), v.as_mut_ptr(), 8);
        v[8] = BLAKE2B_IV0; v[9] = BLAKE2B_IV1; v[10] = BLAKE2B_IV2; v[11] = BLAKE2B_IV3;
        v[12] = BLAKE2B_IV4 ^ (*ctx).t[0] as u64;
        v[13] = BLAKE2B_IV5 ^ (*ctx).t[1] as u64;
        v[14] = BLAKE2B_IV6 ^ (*ctx).f[0] as u64;
        v[15] = BLAKE2B_IV7 ^ (*ctx).f[1] as u64;
        macro_rules! g { ($r:expr, $i:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
            v[$a] = v[$a].wrapping_add(v[$b]).wrapping_add(m[BLAKE2B_SIGMA[$r][2 * $i] as usize]);
            v[$d] = (v[$d] ^ v[$a]).rotate_right(32);
            v[$c] = v[$c].wrapping_add(v[$d]);
            v[$b] = (v[$b] ^ v[$c]).rotate_right(24);
            v[$a] = v[$a].wrapping_add(v[$b]).wrapping_add(m[BLAKE2B_SIGMA[$r][2 * $i + 1] as usize]);
            v[$d] = (v[$d] ^ v[$a]).rotate_right(16);
            v[$c] = v[$c].wrapping_add(v[$d]);
            v[$b] = (v[$b] ^ v[$c]).rotate_right(63);
        }} }
        for r in 0..12 {
            g!(r,0,0,4,8,12); g!(r,1,1,5,9,13); g!(r,2,2,6,10,14); g!(r,3,3,7,11,15);
            g!(r,4,0,5,10,15); g!(r,5,1,6,11,12); g!(r,6,2,7,8,13); g!(r,7,3,4,9,14);
        }
        for i in 0..8 { (*ctx).h[i] ^= v[i] ^ v[i + 8]; }
        data = data.add(BLAKE2B_BLOCK_SIZE); nblocks -= 1;
    }
}

#[inline] unsafe fn blake2b_set_lastblock(ctx: *mut blake2b_ctx) { (*ctx).f[0] = u64::MAX; }

pub unsafe fn blake2b_update(ctx: *mut blake2b_ctx, mut input: *const u8, mut inlen: usize) {
    let fill = BLAKE2B_BLOCK_SIZE - (*ctx).buflen;
    if inlen == 0 { return; }
    if inlen > fill {
        core::ptr::copy_nonoverlapping(input, (*ctx).buf.as_mut_ptr().add((*ctx).buflen), fill);
        blake2b_compress_generic(ctx, (*ctx).buf.as_ptr(), 1, BLAKE2B_BLOCK_SIZE as u32);
        (*ctx).buflen = 0; input = input.add(fill); inlen -= fill;
    }
    if inlen > BLAKE2B_BLOCK_SIZE {
        let nblocks = (inlen + BLAKE2B_BLOCK_SIZE - 1) / BLAKE2B_BLOCK_SIZE;
        blake2b_compress_generic(ctx, input, nblocks - 1, BLAKE2B_BLOCK_SIZE as u32);
        input = input.add(BLAKE2B_BLOCK_SIZE * (nblocks - 1));
        inlen -= BLAKE2B_BLOCK_SIZE * (nblocks - 1);
    }
    core::ptr::copy_nonoverlapping(input, (*ctx).buf.as_mut_ptr().add((*ctx).buflen), inlen);
    (*ctx).buflen += inlen;
}

pub unsafe fn blake2b_final(ctx: *mut blake2b_ctx, out: *mut u8) {
    blake2b_set_lastblock(ctx);
    core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add((*ctx).buflen), 0,
                           BLAKE2B_BLOCK_SIZE - (*ctx).buflen);
    blake2b_compress_generic(ctx, (*ctx).buf.as_ptr(), 1, (*ctx).buflen as u32);
    for x in &mut (*ctx).h { *x = u64::to_le(*x); }
    core::ptr::copy_nonoverlapping((*ctx).h.as_ptr() as *const u8, out, (*ctx).outlen);
    core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<blake2b_ctx>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
