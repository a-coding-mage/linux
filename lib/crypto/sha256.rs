// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256 library functions
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2014 Red Hat Inc.
 * Copyright 2025 Google LLC
 */

// Dependencies: crypto/hmac.h, crypto/sha2.h, linux/export.h, linux/kernel.h,
// linux/module.h, linux/string.h, linux/unaligned.h, linux/wordpart.h,
// "fips-sha.h".

use core::mem::size_of_val;
use core::ptr::{addr_of, addr_of_mut};

static sha224_iv: sha256_block_state = sha256_block_state {
    h: [
        SHA224_H0, SHA224_H1, SHA224_H2, SHA224_H3,
        SHA224_H4, SHA224_H5, SHA224_H6, SHA224_H7,
    ],
};

static initial_sha256_ctx: sha256_ctx = sha256_ctx {
    ctx: __sha256_ctx {
        state: sha256_block_state {
            h: [
                SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3,
                SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7,
            ],
        },
        bytecount: 0,
        buf: [0; SHA256_BLOCK_SIZE as usize],
    },
};

// #define sha256_iv (initial_sha256_ctx.ctx.state)
#[inline(always)]
fn sha256_iv() -> *const sha256_block_state {
    addr_of!(initial_sha256_ctx.ctx.state)
}

static sha256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline(always)]
const fn Ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}
#[inline(always)]
const fn Maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (z & (x | y))
}
#[inline(always)]
const fn e0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
#[inline(always)]
const fn e1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
#[inline(always)]
const fn s0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}
#[inline(always)]
const fn s1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[inline(always)]
unsafe fn LOAD_OP(I: usize, W: &mut [u32; 64], input: *const u8) {
    W[I] = u32::from_be_bytes(input.add(4 * I).cast::<[u8; 4]>().read_unaligned());
}

#[inline(always)]
fn BLEND_OP(I: usize, W: &mut [u32; 64]) {
    W[I] = s1(W[I - 2])
        .wrapping_add(W[I - 7])
        .wrapping_add(s0(W[I - 15]))
        .wrapping_add(W[I - 16]);
}

macro_rules! SHA256_ROUND {
    ($W:ident, $i:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident) => {{
        let t1 = $h
            .wrapping_add(e1($e))
            .wrapping_add(Ch($e, $f, $g))
            .wrapping_add(sha256_K[$i])
            .wrapping_add($W[$i]);
        let t2 = e0($a).wrapping_add(Maj($a, $b, $c));
        $d = $d.wrapping_add(t1);
        $h = t1.wrapping_add(t2);
    }};
}

unsafe fn sha256_block_generic(state: *mut sha256_block_state, input: *const u8, W: &mut [u32; 64]) {
    /* load the input */
    let mut i = 0;
    while i < 16 {
        LOAD_OP(i, W, input);
        LOAD_OP(i + 1, W, input);
        LOAD_OP(i + 2, W, input);
        LOAD_OP(i + 3, W, input);
        LOAD_OP(i + 4, W, input);
        LOAD_OP(i + 5, W, input);
        LOAD_OP(i + 6, W, input);
        LOAD_OP(i + 7, W, input);
        i += 8;
    }

    /* now blend */
    i = 16;
    while i < 64 {
        BLEND_OP(i, W);
        BLEND_OP(i + 1, W);
        BLEND_OP(i + 2, W);
        BLEND_OP(i + 3, W);
        BLEND_OP(i + 4, W);
        BLEND_OP(i + 5, W);
        BLEND_OP(i + 6, W);
        BLEND_OP(i + 7, W);
        i += 8;
    }

    /* load the state into our registers */
    let h_ = &mut (*state).h;
    let (mut a, mut b, mut c, mut d) = (h_[0], h_[1], h_[2], h_[3]);
    let (mut e, mut f, mut g, mut h) = (h_[4], h_[5], h_[6], h_[7]);

    /* now iterate */
    i = 0;
    while i < 64 {
        SHA256_ROUND!(W, i, a, b, c, d, e, f, g, h);
        SHA256_ROUND!(W, i + 1, h, a, b, c, d, e, f, g);
        SHA256_ROUND!(W, i + 2, g, h, a, b, c, d, e, f);
        SHA256_ROUND!(W, i + 3, f, g, h, a, b, c, d, e);
        SHA256_ROUND!(W, i + 4, e, f, g, h, a, b, c, d);
        SHA256_ROUND!(W, i + 5, d, e, f, g, h, a, b, c);
        SHA256_ROUND!(W, i + 6, c, d, e, f, g, h, a, b);
        SHA256_ROUND!(W, i + 7, b, c, d, e, f, g, h, a);
        i += 8;
    }

    h_[0] = h_[0].wrapping_add(a);
    h_[1] = h_[1].wrapping_add(b);
    h_[2] = h_[2].wrapping_add(c);
    h_[3] = h_[3].wrapping_add(d);
    h_[4] = h_[4].wrapping_add(e);
    h_[5] = h_[5].wrapping_add(f);
    h_[6] = h_[6].wrapping_add(g);
    h_[7] = h_[7].wrapping_add(h);
}

#[allow(dead_code)]
unsafe fn sha256_blocks_generic(state: *mut sha256_block_state, mut data: *const u8, mut nblocks: usize) {
    let mut W = [0u32; 64];

    loop {
        sha256_block_generic(state, data, &mut W);
        data = data.add(SHA256_BLOCK_SIZE as usize);
        nblocks -= 1;
        if nblocks == 0 {
            break;
        }
    }

    memzero_explicit(W.as_mut_ptr().cast(), size_of_val(&W));
}

// With CONFIG_CRYPTO_LIB_SHA256_ARCH (and exports enabled) the architecture's
// sha256.h ($(SRCARCH)/sha256.h) provides sha256_blocks().
#[cfg(not(all(CONFIG_CRYPTO_LIB_SHA256_ARCH, not(__DISABLE_EXPORTS))))]
#[inline(always)]
unsafe fn sha256_blocks(state: *mut sha256_block_state, data: *const u8, nblocks: usize) {
    sha256_blocks_generic(state, data, nblocks);
}

unsafe fn __sha256_init(ctx: *mut __sha256_ctx, iv: *const sha256_block_state, initial_bytecount: u64) {
    (*ctx).state = *iv;
    (*ctx).bytecount = initial_bytecount;
}

#[no_mangle]
pub unsafe extern "C" fn sha224_init(ctx: *mut sha224_ctx) {
    __sha256_init(addr_of_mut!((*ctx).ctx), addr_of!(sha224_iv), 0);
}
// EXPORT_SYMBOL_GPL(sha224_init);

#[no_mangle]
pub unsafe extern "C" fn sha256_init(ctx: *mut sha256_ctx) {
    __sha256_init(addr_of_mut!((*ctx).ctx), sha256_iv(), 0);
}
// EXPORT_SYMBOL_GPL(sha256_init);

#[no_mangle]
pub unsafe extern "C" fn __sha256_update(ctx: *mut __sha256_ctx, mut data: *const u8, mut len: usize) {
    const BLOCK: usize = SHA256_BLOCK_SIZE as usize;
    let mut partial = ((*ctx).bytecount % BLOCK as u64) as usize;

    (*ctx).bytecount = (*ctx).bytecount.wrapping_add(len as u64);

    if partial + len >= BLOCK {
        if partial != 0 {
            let l = BLOCK - partial;

            core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), l);
            data = data.add(l);
            len -= l;

            sha256_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);
        }

        let nblocks = len / BLOCK;
        len %= BLOCK;

        if nblocks != 0 {
            sha256_blocks(addr_of_mut!((*ctx).state), data, nblocks);
            data = data.add(nblocks * BLOCK);
        }
        partial = 0;
    }
    if len != 0 {
        core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), len);
    }
}
// EXPORT_SYMBOL(__sha256_update);

unsafe fn __sha256_final(ctx: *mut __sha256_ctx, out: *mut u8, digest_size: usize) {
    const BLOCK: usize = SHA256_BLOCK_SIZE as usize;
    let bitcount: u64 = (*ctx).bytecount << 3;
    let mut partial = ((*ctx).bytecount % BLOCK as u64) as usize;
    let buf = &mut (*ctx).buf;

    buf[partial] = 0x80;
    partial += 1;
    if partial > BLOCK - 8 {
        buf[partial..BLOCK].fill(0);
        sha256_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);
        partial = 0;
    }
    let buf = &mut (*ctx).buf;
    buf[partial..BLOCK - 8].fill(0);
    buf[BLOCK - 8..].copy_from_slice(&bitcount.to_be_bytes());
    sha256_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);

    let mut i = 0;
    while i < digest_size {
        out.add(i).cast::<[u8; 4]>().write_unaligned((*ctx).state.h[i / 4].to_be_bytes());
        i += 4;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sha224_final(ctx: *mut sha224_ctx, out: *mut u8) {
    __sha256_final(addr_of_mut!((*ctx).ctx), out, SHA224_DIGEST_SIZE as usize);
    memzero_explicit(ctx.cast(), core::mem::size_of::<sha224_ctx>());
}
// EXPORT_SYMBOL(sha224_final);

#[no_mangle]
pub unsafe extern "C" fn sha256_final(ctx: *mut sha256_ctx, out: *mut u8) {
    __sha256_final(addr_of_mut!((*ctx).ctx), out, SHA256_DIGEST_SIZE as usize);
    memzero_explicit(ctx.cast(), core::mem::size_of::<sha256_ctx>());
}
// EXPORT_SYMBOL(sha256_final);

#[no_mangle]
pub unsafe extern "C" fn sha224(data: *const u8, len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<sha224_ctx>::uninit();

    sha224_init(ctx.as_mut_ptr());
    sha224_update(ctx.as_mut_ptr(), data, len);
    sha224_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL(sha224);

#[no_mangle]
pub unsafe extern "C" fn sha256(data: *const u8, len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<sha256_ctx>::uninit();

    sha256_init(ctx.as_mut_ptr());
    sha256_update(ctx.as_mut_ptr(), data, len);
    sha256_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL(sha256);

/*
 * Pre-boot environments (as indicated by __DISABLE_EXPORTS being defined) just
 * need the generic SHA-256 code.  Omit all other features from them.
 */

// #ifndef sha256_finup_2x_arch: only arm64 and x86 provide it with the arch code.
#[cfg(all(
    not(__DISABLE_EXPORTS),
    not(all(CONFIG_CRYPTO_LIB_SHA256_ARCH, any(CONFIG_ARM64, CONFIG_X86)))
))]
unsafe fn sha256_finup_2x_arch(
    _ctx: *const __sha256_ctx,
    _data1: *const u8,
    _data2: *const u8,
    _len: usize,
    _out1: *mut u8,
    _out2: *mut u8,
) -> bool {
    false
}
#[cfg(all(
    not(__DISABLE_EXPORTS),
    not(all(CONFIG_CRYPTO_LIB_SHA256_ARCH, any(CONFIG_ARM64, CONFIG_X86)))
))]
fn sha256_finup_2x_is_optimized_arch() -> bool {
    false
}

/* Sequential fallback implementation of sha256_finup_2x() */
#[cfg(not(__DISABLE_EXPORTS))]
#[inline(never)]
unsafe fn sha256_finup_2x_sequential(
    ctx: *const __sha256_ctx,
    data1: *const u8,
    data2: *const u8,
    len: usize,
    out1: *mut u8,
    out2: *mut u8,
) {
    let mut mut_ctx: __sha256_ctx = *ctx;
    __sha256_update(&mut mut_ctx, data1, len);
    __sha256_final(&mut mut_ctx, out1, SHA256_DIGEST_SIZE as usize);

    mut_ctx = *ctx;
    __sha256_update(&mut mut_ctx, data2, len);
    __sha256_final(&mut mut_ctx, out2, SHA256_DIGEST_SIZE as usize);
}

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn sha256_finup_2x(
    mut ctx: *const sha256_ctx,
    data1: *const u8,
    data2: *const u8,
    len: usize,
    out1: *mut u8,
    out2: *mut u8,
) {
    if ctx.is_null() {
        ctx = addr_of!(initial_sha256_ctx);
    }

    if likely(sha256_finup_2x_arch(addr_of!((*ctx).ctx), data1, data2, len, out1, out2)) {
        return;
    }
    sha256_finup_2x_sequential(addr_of!((*ctx).ctx), data1, data2, len, out1, out2);
}
// EXPORT_SYMBOL_GPL(sha256_finup_2x);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub extern "C" fn sha256_finup_2x_is_optimized() -> bool {
    sha256_finup_2x_is_optimized_arch()
}
// EXPORT_SYMBOL_GPL(sha256_finup_2x_is_optimized);

#[cfg(not(__DISABLE_EXPORTS))]
#[repr(C)]
union hmac_derived_key {
    b: [u8; SHA256_BLOCK_SIZE as usize],
    w: [kernel::ffi::c_ulong; SHA256_BLOCK_SIZE as usize / core::mem::size_of::<kernel::ffi::c_ulong>()],
}

#[cfg(not(__DISABLE_EXPORTS))]
unsafe fn __hmac_sha256_preparekey(
    istate: *mut sha256_block_state,
    ostate: *mut sha256_block_state,
    raw_key: *const u8,
    raw_key_len: usize,
    iv: *const sha256_block_state,
) {
    let mut derived_key = hmac_derived_key { b: [0; SHA256_BLOCK_SIZE as usize] };

    if unlikely(raw_key_len > SHA256_BLOCK_SIZE as usize) {
        if iv == addr_of!(sha224_iv) {
            sha224(raw_key, raw_key_len, derived_key.b.as_mut_ptr());
        } else {
            sha256(raw_key, raw_key_len, derived_key.b.as_mut_ptr());
        }
    } else {
        core::ptr::copy_nonoverlapping(raw_key, derived_key.b.as_mut_ptr(), raw_key_len);
    }

    for w in derived_key.w.iter_mut() {
        *w ^= REPEAT_BYTE(HMAC_IPAD_VALUE as u8);
    }
    *istate = *iv;
    sha256_blocks(istate, derived_key.b.as_ptr(), 1);

    for w in derived_key.w.iter_mut() {
        *w ^= REPEAT_BYTE((HMAC_OPAD_VALUE ^ HMAC_IPAD_VALUE) as u8);
    }
    *ostate = *iv;
    sha256_blocks(ostate, derived_key.b.as_ptr(), 1);

    memzero_explicit(addr_of_mut!(derived_key).cast(), core::mem::size_of::<hmac_derived_key>());
}

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha224_preparekey(key: *mut hmac_sha224_key, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha256_preparekey(
        addr_of_mut!((*key).key.istate),
        addr_of_mut!((*key).key.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha224_iv),
    );
}
// EXPORT_SYMBOL_GPL(hmac_sha224_preparekey);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha256_preparekey(key: *mut hmac_sha256_key, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha256_preparekey(
        addr_of_mut!((*key).key.istate),
        addr_of_mut!((*key).key.ostate),
        raw_key,
        raw_key_len,
        sha256_iv(),
    );
}
// EXPORT_SYMBOL_GPL(hmac_sha256_preparekey);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn __hmac_sha256_init(ctx: *mut __hmac_sha256_ctx, key: *const __hmac_sha256_key) {
    __sha256_init(addr_of_mut!((*ctx).sha_ctx), addr_of!((*key).istate), SHA256_BLOCK_SIZE as u64);
    (*ctx).ostate = (*key).ostate;
}
// EXPORT_SYMBOL_GPL(__hmac_sha256_init);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha224_init_usingrawkey(ctx: *mut hmac_sha224_ctx, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha256_preparekey(
        addr_of_mut!((*ctx).ctx.sha_ctx.state),
        addr_of_mut!((*ctx).ctx.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha224_iv),
    );
    (*ctx).ctx.sha_ctx.bytecount = SHA256_BLOCK_SIZE as u64;
}
// EXPORT_SYMBOL_GPL(hmac_sha224_init_usingrawkey);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha256_init_usingrawkey(ctx: *mut hmac_sha256_ctx, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha256_preparekey(
        addr_of_mut!((*ctx).ctx.sha_ctx.state),
        addr_of_mut!((*ctx).ctx.ostate),
        raw_key,
        raw_key_len,
        sha256_iv(),
    );
    (*ctx).ctx.sha_ctx.bytecount = SHA256_BLOCK_SIZE as u64;
}
// EXPORT_SYMBOL_GPL(hmac_sha256_init_usingrawkey);

#[cfg(not(__DISABLE_EXPORTS))]
unsafe fn __hmac_sha256_final(ctx: *mut __hmac_sha256_ctx, out: *mut u8, digest_size: usize) {
    const BLOCK: usize = SHA256_BLOCK_SIZE as usize;

    /* Generate the padded input for the outer hash in ctx->sha_ctx.buf. */
    __sha256_final(addr_of_mut!((*ctx).sha_ctx), (*ctx).sha_ctx.buf.as_mut_ptr(), digest_size);
    let buf = &mut (*ctx).sha_ctx.buf;
    buf[digest_size..BLOCK].fill(0);
    buf[digest_size] = 0x80;
    buf[BLOCK - 4..].copy_from_slice(&((8 * (BLOCK + digest_size)) as u32).to_be_bytes());

    /* Compute the outer hash, which gives the HMAC value. */
    sha256_blocks(addr_of_mut!((*ctx).ostate), (*ctx).sha_ctx.buf.as_ptr(), 1);
    let mut i = 0;
    while i < digest_size {
        out.add(i).cast::<[u8; 4]>().write_unaligned((*ctx).ostate.h[i / 4].to_be_bytes());
        i += 4;
    }

    memzero_explicit(ctx.cast(), core::mem::size_of::<__hmac_sha256_ctx>());
}

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha224_final(ctx: *mut hmac_sha224_ctx, out: *mut u8) {
    __hmac_sha256_final(addr_of_mut!((*ctx).ctx), out, SHA224_DIGEST_SIZE as usize);
}
// EXPORT_SYMBOL_GPL(hmac_sha224_final);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha256_final(ctx: *mut hmac_sha256_ctx, out: *mut u8) {
    __hmac_sha256_final(addr_of_mut!((*ctx).ctx), out, SHA256_DIGEST_SIZE as usize);
}
// EXPORT_SYMBOL_GPL(hmac_sha256_final);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha224(key: *const hmac_sha224_key, data: *const u8, data_len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha224_ctx>::uninit();

    hmac_sha224_init(ctx.as_mut_ptr(), key);
    hmac_sha224_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha224_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha224);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha256(key: *const hmac_sha256_key, data: *const u8, data_len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha256_ctx>::uninit();

    hmac_sha256_init(ctx.as_mut_ptr(), key);
    hmac_sha256_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha256_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha256);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha224_usingrawkey(
    raw_key: *const u8,
    raw_key_len: usize,
    data: *const u8,
    data_len: usize,
    out: *mut u8,
) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha224_ctx>::uninit();

    hmac_sha224_init_usingrawkey(ctx.as_mut_ptr(), raw_key, raw_key_len);
    hmac_sha224_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha224_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha224_usingrawkey);

#[cfg(not(__DISABLE_EXPORTS))]
#[no_mangle]
pub unsafe extern "C" fn hmac_sha256_usingrawkey(
    raw_key: *const u8,
    raw_key_len: usize,
    data: *const u8,
    data_len: usize,
    out: *mut u8,
) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha256_ctx>::uninit();

    hmac_sha256_init_usingrawkey(ctx.as_mut_ptr(), raw_key, raw_key_len);
    hmac_sha256_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha256_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha256_usingrawkey);

// #if defined(sha256_mod_init_arch) || defined(CONFIG_CRYPTO_FIPS):
// sha256_mod_init_arch is defined by the arm, arm64, riscv, s390, sparc and
// x86 architecture code.
#[cfg(all(
    not(__DISABLE_EXPORTS),
    any(
        CONFIG_CRYPTO_FIPS,
        all(
            CONFIG_CRYPTO_LIB_SHA256_ARCH,
            any(CONFIG_ARM, CONFIG_ARM64, CONFIG_RISCV, CONFIG_S390, CONFIG_SPARC, CONFIG_X86)
        )
    )
))]
#[link_section = ".init.text"]
unsafe extern "C" fn sha256_mod_init() -> kernel::ffi::c_int {
    #[cfg(all(
        CONFIG_CRYPTO_LIB_SHA256_ARCH,
        any(CONFIG_ARM, CONFIG_ARM64, CONFIG_RISCV, CONFIG_S390, CONFIG_SPARC, CONFIG_X86)
    ))]
    sha256_mod_init_arch();
    if fips_enabled != 0 {
        /*
         * FIPS cryptographic algorithm self-test.  As per the FIPS
         * Implementation Guidance, testing HMAC-SHA256 satisfies the
         * test requirement for SHA-224, SHA-256, and HMAC-SHA224 too.
         */
        let mut mac = [0u8; SHA256_DIGEST_SIZE as usize];

        hmac_sha256_usingrawkey(
            fips_test_key.as_ptr(),
            size_of_val(&fips_test_key),
            fips_test_data.as_ptr(),
            size_of_val(&fips_test_data),
            mac.as_mut_ptr(),
        );
        if fips_test_hmac_sha256_value[..] != mac[..] {
            panic(c"sha256: FIPS self-test failed\n".as_ptr());
        }
    }
    0
}
// subsys_initcall(sha256_mod_init);

// static void __exit sha256_mod_exit(void) {}  module_exit(sha256_mod_exit);

// MODULE_DESCRIPTION("SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256 library functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
