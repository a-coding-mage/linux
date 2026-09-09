// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is an implementation of the BLAKE2s hash and PRF functions.
 *
 * Information: https://blake2.net/
 */

// Dependencies supplied by the surrounding kernel translation:
// blake2s_ctx, BLAKE2S_BLOCK_SIZE, BLAKE2S_IV0..BLAKE2S_IV7,
// ror32, le32_to_cpu_array, cpu_to_le32_array, memzero_explicit.

static const blake2s_sigma: [[u8; 16]; 10] = [
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
];

#[inline]
unsafe fn blake2s_increment_counter(ctx: *mut blake2s_ctx, inc: u32) {
    (*ctx).t[0] = (*ctx).t[0].wrapping_add(inc);
    (*ctx).t[1] = (*ctx).t[1].wrapping_add(((*ctx).t[0] < inc) as u32);
}

unsafe fn blake2s_compress_generic(
    ctx: *mut blake2s_ctx,
    mut data: *const u8,
    mut nblocks: usize,
    inc: u32,
) {
    let mut m = [0u32; 16];
    let mut v = [0u32; 16];

    while nblocks > 0 {
        blake2s_increment_counter(ctx, inc);
        core::ptr::copy_nonoverlapping(data, m.as_mut_ptr() as *mut u8, BLAKE2S_BLOCK_SIZE);
        le32_to_cpu_array(m.as_mut_ptr(), m.len());
        core::ptr::copy_nonoverlapping((*ctx).h.as_ptr(), v.as_mut_ptr(), 8);
        v[8] = BLAKE2S_IV0;
        v[9] = BLAKE2S_IV1;
        v[10] = BLAKE2S_IV2;
        v[11] = BLAKE2S_IV3;
        v[12] = BLAKE2S_IV4 ^ (*ctx).t[0];
        v[13] = BLAKE2S_IV5 ^ (*ctx).t[1];
        v[14] = BLAKE2S_IV6 ^ (*ctx).f[0];
        v[15] = BLAKE2S_IV7 ^ (*ctx).f[1];

        macro_rules! g {
            ($r:expr, $i:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
                $a = $a.wrapping_add($b).wrapping_add(m[blake2s_sigma[$r][$i * 2] as usize]);
                $d = ror32($d ^ $a, 16);
                $c = $c.wrapping_add($d);
                $b = ror32($b ^ $c, 12);
                $a = $a.wrapping_add($b).wrapping_add(m[blake2s_sigma[$r][$i * 2 + 1] as usize]);
                $d = ror32($d ^ $a, 8);
                $c = $c.wrapping_add($d);
                $b = ror32($b ^ $c, 7);
            }};
        }

        for r in 0..10 {
            g!(r, 0, v[0], v[4], v[8], v[12]); g!(r, 1, v[1], v[5], v[9], v[13]);
            g!(r, 2, v[2], v[6], v[10], v[14]); g!(r, 3, v[3], v[7], v[11], v[15]);
            g!(r, 4, v[0], v[5], v[10], v[15]); g!(r, 5, v[1], v[6], v[11], v[12]);
            g!(r, 6, v[2], v[7], v[8], v[13]); g!(r, 7, v[3], v[4], v[9], v[14]);
        }
        for i in 0..8 { (*ctx).h[i] ^= v[i] ^ v[i + 8]; }
        data = data.add(BLAKE2S_BLOCK_SIZE);
        nblocks -= 1;
    }
}

#[inline]
unsafe fn blake2s_set_lastblock(ctx: *mut blake2s_ctx) { (*ctx).f[0] = u32::MAX; }

pub unsafe fn blake2s_update(ctx: *mut blake2s_ctx, mut input: *const u8, mut inlen: usize) {
    let fill = BLAKE2S_BLOCK_SIZE - (*ctx).buflen;
    if inlen == 0 { return; }
    if inlen > fill {
        core::ptr::copy_nonoverlapping(input, (*ctx).buf.as_mut_ptr().add((*ctx).buflen), fill);
        blake2s_compress_generic(ctx, (*ctx).buf.as_ptr(), 1, BLAKE2S_BLOCK_SIZE as u32);
        (*ctx).buflen = 0; input = input.add(fill); inlen -= fill;
    }
    if inlen > BLAKE2S_BLOCK_SIZE {
        let nblocks = (inlen + BLAKE2S_BLOCK_SIZE - 1) / BLAKE2S_BLOCK_SIZE;
        blake2s_compress_generic(ctx, input, nblocks - 1, BLAKE2S_BLOCK_SIZE as u32);
        input = input.add(BLAKE2S_BLOCK_SIZE * (nblocks - 1));
        inlen -= BLAKE2S_BLOCK_SIZE * (nblocks - 1);
    }
    core::ptr::copy_nonoverlapping(input, (*ctx).buf.as_mut_ptr().add((*ctx).buflen), inlen);
    (*ctx).buflen += inlen;
}

pub unsafe fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8) {
    blake2s_set_lastblock(ctx);
    core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add((*ctx).buflen), 0, BLAKE2S_BLOCK_SIZE - (*ctx).buflen);
    blake2s_compress_generic(ctx, (*ctx).buf.as_ptr(), 1, (*ctx).buflen as u32);
    cpu_to_le32_array((*ctx).h.as_mut_ptr(), (*ctx).h.len());
    core::ptr::copy_nonoverlapping((*ctx).h.as_ptr() as *const u8, out, (*ctx).outlen);
    memzero_explicit(ctx as *mut u8, core::mem::size_of::<blake2s_ctx>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
