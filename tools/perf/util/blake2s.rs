// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is an implementation of the BLAKE2s hash and PRF functions.
 *
 * Information: https://blake2.net/
 */

pub type u8 = ::std::os::raw::c_uchar;
pub type u32 = ::std::os::raw::c_uint;
pub type size_t = usize;

pub const BLAKE2S_BLOCK_SIZE: size_t = 64;
pub const BLAKE2S_IV0: u32 = 0x6A09E667;
pub const BLAKE2S_IV1: u32 = 0xBB67AE85;
pub const BLAKE2S_IV2: u32 = 0x3C6EF372;
pub const BLAKE2S_IV3: u32 = 0xA54FF53A;
pub const BLAKE2S_IV4: u32 = 0x510E527F;
pub const BLAKE2S_IV5: u32 = 0x9B05688C;
pub const BLAKE2S_IV6: u32 = 0x1F83D9AB;
pub const BLAKE2S_IV7: u32 = 0x5BE0CD19;

#[repr(C)]
pub struct blake2s_ctx {
    pub h: [u32; 8],
    pub t: [u32; 2],
    pub f: [u32; 2],
    pub buf: [u8; BLAKE2S_BLOCK_SIZE],
    pub buflen: size_t,
    pub outlen: size_t,
}

unsafe extern "C" {
    fn memcpy(dest: *mut ::std::ffi::c_void, src: *const ::std::ffi::c_void, n: size_t)
        -> *mut ::std::ffi::c_void;
    fn memset(s: *mut ::std::ffi::c_void, c: ::std::os::raw::c_int, n: size_t)
        -> *mut ::std::ffi::c_void;
}

#[inline]
fn ror32(v: u32, n: ::std::os::raw::c_int) -> u32 {
    (v >> n) | (v << (32 - n))
}

#[inline]
fn le32_to_cpu_array(a: *mut u32, n: size_t) {
    let mut i: size_t = 0;

    while i < n {
        unsafe {
            *a.add(i) = u32::from_le(*a.add(i));
        }
        i += 1;
    }
}

#[inline]
fn cpu_to_le32_array(a: *mut u32, n: size_t) {
    let mut i: size_t = 0;

    while i < n {
        unsafe {
            *a.add(i) = (*a.add(i)).to_le();
        }
        i += 1;
    }
}

static blake2s_sigma: [[u8; 16]; 10] = [
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
    let old = (*ctx).t[0];

    (*ctx).t[0] = (*ctx).t[0].wrapping_add(inc);
    (*ctx).t[1] = (*ctx).t[1].wrapping_add(((*ctx).t[0] < inc) as u32);
    let _ = old;
}

#[inline]
fn g(v: &mut [u32; 16], m: &[u32; 16], r: usize, i: usize, a: usize, b: usize, c: usize, d: usize) {
    v[a] = v[a]
        .wrapping_add(v[b])
        .wrapping_add(m[blake2s_sigma[r][2 * i + 0] as usize]);
    v[d] = ror32(v[d] ^ v[a], 16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = ror32(v[b] ^ v[c], 12);
    v[a] = v[a]
        .wrapping_add(v[b])
        .wrapping_add(m[blake2s_sigma[r][2 * i + 1] as usize]);
    v[d] = ror32(v[d] ^ v[a], 8);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = ror32(v[b] ^ v[c], 7);
}

#[inline]
fn round(v: &mut [u32; 16], m: &[u32; 16], r: usize) {
    g(v, m, r, 0, 0, 4, 8, 12);
    g(v, m, r, 1, 1, 5, 9, 13);
    g(v, m, r, 2, 2, 6, 10, 14);
    g(v, m, r, 3, 3, 7, 11, 15);
    g(v, m, r, 4, 0, 5, 10, 15);
    g(v, m, r, 5, 1, 6, 11, 12);
    g(v, m, r, 6, 2, 7, 8, 13);
    g(v, m, r, 7, 3, 4, 9, 14);
}

unsafe fn blake2s_compress(
    ctx: *mut blake2s_ctx,
    mut data: *const u8,
    mut nblocks: size_t,
    inc: u32,
) {
    let mut m: [u32; 16] = [0; 16];
    let mut v: [u32; 16] = [0; 16];
    let mut i: ::std::os::raw::c_int;

    while nblocks > 0 {
        blake2s_increment_counter(ctx, inc);
        memcpy(
            m.as_mut_ptr() as *mut ::std::ffi::c_void,
            data as *const ::std::ffi::c_void,
            BLAKE2S_BLOCK_SIZE,
        );
        le32_to_cpu_array(m.as_mut_ptr(), m.len());
        memcpy(
            v.as_mut_ptr() as *mut ::std::ffi::c_void,
            (*ctx).h.as_ptr() as *const ::std::ffi::c_void,
            32,
        );
        v[8] = BLAKE2S_IV0;
        v[9] = BLAKE2S_IV1;
        v[10] = BLAKE2S_IV2;
        v[11] = BLAKE2S_IV3;
        v[12] = BLAKE2S_IV4 ^ (*ctx).t[0];
        v[13] = BLAKE2S_IV5 ^ (*ctx).t[1];
        v[14] = BLAKE2S_IV6 ^ (*ctx).f[0];
        v[15] = BLAKE2S_IV7 ^ (*ctx).f[1];

        round(&mut v, &m, 0);
        round(&mut v, &m, 1);
        round(&mut v, &m, 2);
        round(&mut v, &m, 3);
        round(&mut v, &m, 4);
        round(&mut v, &m, 5);
        round(&mut v, &m, 6);
        round(&mut v, &m, 7);
        round(&mut v, &m, 8);
        round(&mut v, &m, 9);

        i = 0;
        while i < 8 {
            (*ctx).h[i as usize] ^= v[i as usize] ^ v[i as usize + 8];
            i += 1;
        }

        data = data.add(BLAKE2S_BLOCK_SIZE);
        nblocks -= 1;
    }
}

#[inline]
unsafe fn blake2s_set_lastblock(ctx: *mut blake2s_ctx) {
    (*ctx).f[0] = (-1i32) as u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blake2s_update(
    ctx: *mut blake2s_ctx,
    mut in_: *const u8,
    mut inlen: size_t,
) {
    let fill: size_t = BLAKE2S_BLOCK_SIZE - (*ctx).buflen;

    if inlen == 0 {
        return;
    }
    if inlen > fill {
        memcpy(
            (*ctx).buf.as_mut_ptr().add((*ctx).buflen) as *mut ::std::ffi::c_void,
            in_ as *const ::std::ffi::c_void,
            fill,
        );
        blake2s_compress(ctx, (*ctx).buf.as_ptr(), 1, BLAKE2S_BLOCK_SIZE as u32);
        (*ctx).buflen = 0;
        in_ = in_.add(fill);
        inlen -= fill;
    }
    if inlen > BLAKE2S_BLOCK_SIZE {
        let nblocks: size_t = inlen.div_ceil(BLAKE2S_BLOCK_SIZE);

        blake2s_compress(ctx, in_, nblocks - 1, BLAKE2S_BLOCK_SIZE as u32);
        in_ = in_.add(BLAKE2S_BLOCK_SIZE * (nblocks - 1));
        inlen -= BLAKE2S_BLOCK_SIZE * (nblocks - 1);
    }
    memcpy(
        (*ctx).buf.as_mut_ptr().add((*ctx).buflen) as *mut ::std::ffi::c_void,
        in_ as *const ::std::ffi::c_void,
        inlen,
    );
    (*ctx).buflen += inlen;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8) {
    blake2s_set_lastblock(ctx);
    memset(
        (*ctx).buf.as_mut_ptr().add((*ctx).buflen) as *mut ::std::ffi::c_void,
        0,
        BLAKE2S_BLOCK_SIZE - (*ctx).buflen,
    ); /* Padding */
    blake2s_compress(ctx, (*ctx).buf.as_ptr(), 1, (*ctx).buflen as u32);
    cpu_to_le32_array((*ctx).h.as_mut_ptr(), (*ctx).h.len());
    memcpy(
        out as *mut ::std::ffi::c_void,
        (*ctx).h.as_ptr() as *const ::std::ffi::c_void,
        (*ctx).outlen,
    );
    memset(
        ctx as *mut ::std::ffi::c_void,
        0,
        ::std::mem::size_of::<blake2s_ctx>(),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
