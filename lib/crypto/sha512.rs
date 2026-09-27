// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SHA-384, SHA-512, HMAC-SHA384, and HMAC-SHA512 library functions
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2003 Kyle McMartin <kyle@debian.org>
 * Copyright 2025 Google LLC
 */

// Dependencies: crypto/hmac.h, crypto/sha2.h, linux/export.h, linux/kernel.h,
// linux/module.h, linux/overflow.h, linux/string.h, linux/unaligned.h,
// linux/wordpart.h, "fips-sha.h".

use core::mem::size_of_val;
use core::ptr::{addr_of, addr_of_mut};

static sha384_iv: sha512_block_state = sha512_block_state {
    h: [
        SHA384_H0, SHA384_H1, SHA384_H2, SHA384_H3,
        SHA384_H4, SHA384_H5, SHA384_H6, SHA384_H7,
    ],
};

static sha512_iv: sha512_block_state = sha512_block_state {
    h: [
        SHA512_H0, SHA512_H1, SHA512_H2, SHA512_H3,
        SHA512_H4, SHA512_H5, SHA512_H6, SHA512_H7,
    ],
};

static sha512_K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc, 0x3956c25bf348b538, 0x59f111f1b605d019,
    0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242,
    0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
    0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 0x2de92c6f592b0275,
    0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f,
    0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
    0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc,
    0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6,
    0x92722c851482353b, 0xa2bfe8a14cf10364, 0xa81a664bbc423001,
    0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
    0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc,
    0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915,
    0xc67178f2e372532b, 0xca273eceea26619c, 0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba,
    0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

#[inline(always)]
const fn Ch(x: u64, y: u64, z: u64) -> u64 {
    z ^ (x & (y ^ z))
}
#[inline(always)]
const fn Maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) | (z & (x | y))
}
#[inline(always)]
const fn e0(x: u64) -> u64 {
    x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
}
#[inline(always)]
const fn e1(x: u64) -> u64 {
    x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
}
#[inline(always)]
const fn s0(x: u64) -> u64 {
    x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
}
#[inline(always)]
const fn s1(x: u64) -> u64 {
    x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
}

unsafe fn sha512_block_generic(state: *mut sha512_block_state, data: *const u8) {
    let hs = &mut (*state).h;
    let (mut a, mut b, mut c, mut d) = (hs[0], hs[1], hs[2], hs[3]);
    let (mut e, mut f, mut g, mut h) = (hs[4], hs[5], hs[6], hs[7]);
    let mut t1: u64;
    let mut t2: u64;
    let mut W = [0u64; 16];

    for (j, w) in W.iter_mut().enumerate() {
        *w = u64::from_be_bytes(data.add(j * 8).cast::<[u8; 8]>().read_unaligned());
    }

    let mut i = 0;
    while i < 80 {
        if (i & 15) == 0 && i != 0 {
            for j in 0..16usize {
                W[j & 15] = W[j & 15].wrapping_add(
                    s1(W[j.wrapping_sub(2) & 15])
                        .wrapping_add(W[j.wrapping_sub(7) & 15])
                        .wrapping_add(s0(W[j.wrapping_sub(15) & 15])),
                );
            }
        }
        macro_rules! R {
            ($k:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident) => {
                t1 = $h
                    .wrapping_add(e1($e))
                    .wrapping_add(Ch($e, $f, $g))
                    .wrapping_add(sha512_K[i + $k])
                    .wrapping_add(W[(i & 15) + $k]);
                t2 = e0($a).wrapping_add(Maj($a, $b, $c));
                $d = $d.wrapping_add(t1);
                $h = t1.wrapping_add(t2);
            };
        }
        R!(0, a, b, c, d, e, f, g, h);
        R!(1, h, a, b, c, d, e, f, g);
        R!(2, g, h, a, b, c, d, e, f);
        R!(3, f, g, h, a, b, c, d, e);
        R!(4, e, f, g, h, a, b, c, d);
        R!(5, d, e, f, g, h, a, b, c);
        R!(6, c, d, e, f, g, h, a, b);
        R!(7, b, c, d, e, f, g, h, a);
        i += 8;
    }

    hs[0] = hs[0].wrapping_add(a);
    hs[1] = hs[1].wrapping_add(b);
    hs[2] = hs[2].wrapping_add(c);
    hs[3] = hs[3].wrapping_add(d);
    hs[4] = hs[4].wrapping_add(e);
    hs[5] = hs[5].wrapping_add(f);
    hs[6] = hs[6].wrapping_add(g);
    hs[7] = hs[7].wrapping_add(h);
}

#[allow(dead_code)]
unsafe fn sha512_blocks_generic(state: *mut sha512_block_state, mut data: *const u8, mut nblocks: usize) {
    loop {
        sha512_block_generic(state, data);
        data = data.add(SHA512_BLOCK_SIZE as usize);
        nblocks -= 1;
        if nblocks == 0 {
            break;
        }
    }
}

// With CONFIG_CRYPTO_LIB_SHA512_ARCH the architecture's sha512.h
// ($(SRCARCH)/sha512.h) provides sha512_blocks().
#[cfg(not(CONFIG_CRYPTO_LIB_SHA512_ARCH))]
#[inline(always)]
unsafe fn sha512_blocks(state: *mut sha512_block_state, data: *const u8, nblocks: usize) {
    sha512_blocks_generic(state, data, nblocks);
}

unsafe fn __sha512_init(ctx: *mut __sha512_ctx, iv: *const sha512_block_state, initial_bytecount: u64) {
    (*ctx).state = *iv;
    (*ctx).bytecount_lo = initial_bytecount;
    (*ctx).bytecount_hi = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sha384_init(ctx: *mut sha384_ctx) {
    __sha512_init(addr_of_mut!((*ctx).ctx), addr_of!(sha384_iv), 0);
}
// EXPORT_SYMBOL_GPL(sha384_init);

#[no_mangle]
pub unsafe extern "C" fn sha512_init(ctx: *mut sha512_ctx) {
    __sha512_init(addr_of_mut!((*ctx).ctx), addr_of!(sha512_iv), 0);
}
// EXPORT_SYMBOL_GPL(sha512_init);

#[no_mangle]
pub unsafe extern "C" fn __sha512_update(ctx: *mut __sha512_ctx, mut data: *const u8, mut len: usize) {
    const BLOCK: usize = SHA512_BLOCK_SIZE as usize;
    let mut partial = ((*ctx).bytecount_lo % BLOCK as u64) as usize;

    let (lo, overflowed) = (*ctx).bytecount_lo.overflowing_add(len as u64);
    (*ctx).bytecount_lo = lo;
    if overflowed {
        (*ctx).bytecount_hi = (*ctx).bytecount_hi.wrapping_add(1);
    }

    if partial + len >= BLOCK {
        if partial != 0 {
            let l = BLOCK - partial;

            core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), l);
            data = data.add(l);
            len -= l;

            sha512_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);
        }

        let nblocks = len / BLOCK;
        len %= BLOCK;

        if nblocks != 0 {
            sha512_blocks(addr_of_mut!((*ctx).state), data, nblocks);
            data = data.add(nblocks * BLOCK);
        }
        partial = 0;
    }
    if len != 0 {
        core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), len);
    }
}
// EXPORT_SYMBOL_GPL(__sha512_update);

unsafe fn __sha512_final(ctx: *mut __sha512_ctx, out: *mut u8, digest_size: usize) {
    const BLOCK: usize = SHA512_BLOCK_SIZE as usize;
    let bitcount_hi: u64 = ((*ctx).bytecount_hi << 3) | ((*ctx).bytecount_lo >> 61);
    let bitcount_lo: u64 = (*ctx).bytecount_lo << 3;
    let mut partial = ((*ctx).bytecount_lo % BLOCK as u64) as usize;

    (*ctx).buf[partial] = 0x80;
    partial += 1;
    if partial > BLOCK - 16 {
        (&mut (*ctx).buf)[partial..BLOCK].fill(0);
        sha512_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);
        partial = 0;
    }
    let buf = &mut (*ctx).buf;
    buf[partial..BLOCK - 16].fill(0);
    buf[BLOCK - 16..BLOCK - 8].copy_from_slice(&bitcount_hi.to_be_bytes());
    buf[BLOCK - 8..].copy_from_slice(&bitcount_lo.to_be_bytes());
    sha512_blocks(addr_of_mut!((*ctx).state), (*ctx).buf.as_ptr(), 1);

    let mut i = 0;
    while i < digest_size {
        out.add(i).cast::<[u8; 8]>().write_unaligned((*ctx).state.h[i / 8].to_be_bytes());
        i += 8;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sha384_final(ctx: *mut sha384_ctx, out: *mut u8) {
    __sha512_final(addr_of_mut!((*ctx).ctx), out, SHA384_DIGEST_SIZE as usize);
    memzero_explicit(ctx.cast(), core::mem::size_of::<sha384_ctx>());
}
// EXPORT_SYMBOL(sha384_final);

#[no_mangle]
pub unsafe extern "C" fn sha512_final(ctx: *mut sha512_ctx, out: *mut u8) {
    __sha512_final(addr_of_mut!((*ctx).ctx), out, SHA512_DIGEST_SIZE as usize);
    memzero_explicit(ctx.cast(), core::mem::size_of::<sha512_ctx>());
}
// EXPORT_SYMBOL(sha512_final);

#[no_mangle]
pub unsafe extern "C" fn sha384(data: *const u8, len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<sha384_ctx>::uninit();

    sha384_init(ctx.as_mut_ptr());
    sha384_update(ctx.as_mut_ptr(), data, len);
    sha384_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL(sha384);

#[no_mangle]
pub unsafe extern "C" fn sha512(data: *const u8, len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<sha512_ctx>::uninit();

    sha512_init(ctx.as_mut_ptr());
    sha512_update(ctx.as_mut_ptr(), data, len);
    sha512_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL(sha512);

#[repr(C)]
union hmac512_derived_key {
    b: [u8; SHA512_BLOCK_SIZE as usize],
    w: [kernel::ffi::c_ulong; SHA512_BLOCK_SIZE as usize / core::mem::size_of::<kernel::ffi::c_ulong>()],
}

unsafe fn __hmac_sha512_preparekey(
    istate: *mut sha512_block_state,
    ostate: *mut sha512_block_state,
    raw_key: *const u8,
    raw_key_len: usize,
    iv: *const sha512_block_state,
) {
    let mut derived_key = hmac512_derived_key { b: [0; SHA512_BLOCK_SIZE as usize] };

    if unlikely(raw_key_len > SHA512_BLOCK_SIZE as usize) {
        if iv == addr_of!(sha384_iv) {
            sha384(raw_key, raw_key_len, derived_key.b.as_mut_ptr());
        } else {
            sha512(raw_key, raw_key_len, derived_key.b.as_mut_ptr());
        }
    } else {
        core::ptr::copy_nonoverlapping(raw_key, derived_key.b.as_mut_ptr(), raw_key_len);
    }

    for w in derived_key.w.iter_mut() {
        *w ^= REPEAT_BYTE(HMAC_IPAD_VALUE as u8);
    }
    *istate = *iv;
    sha512_blocks(istate, derived_key.b.as_ptr(), 1);

    for w in derived_key.w.iter_mut() {
        *w ^= REPEAT_BYTE((HMAC_OPAD_VALUE ^ HMAC_IPAD_VALUE) as u8);
    }
    *ostate = *iv;
    sha512_blocks(ostate, derived_key.b.as_ptr(), 1);

    memzero_explicit(addr_of_mut!(derived_key).cast(), core::mem::size_of::<hmac512_derived_key>());
}

#[no_mangle]
pub unsafe extern "C" fn hmac_sha384_preparekey(key: *mut hmac_sha384_key, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha512_preparekey(
        addr_of_mut!((*key).key.istate),
        addr_of_mut!((*key).key.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha384_iv),
    );
}
// EXPORT_SYMBOL_GPL(hmac_sha384_preparekey);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha512_preparekey(key: *mut hmac_sha512_key, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha512_preparekey(
        addr_of_mut!((*key).key.istate),
        addr_of_mut!((*key).key.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha512_iv),
    );
}
// EXPORT_SYMBOL_GPL(hmac_sha512_preparekey);

#[no_mangle]
pub unsafe extern "C" fn __hmac_sha512_init(ctx: *mut __hmac_sha512_ctx, key: *const __hmac_sha512_key) {
    __sha512_init(addr_of_mut!((*ctx).sha_ctx), addr_of!((*key).istate), SHA512_BLOCK_SIZE as u64);
    (*ctx).ostate = (*key).ostate;
}
// EXPORT_SYMBOL_GPL(__hmac_sha512_init);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha384_init_usingrawkey(ctx: *mut hmac_sha384_ctx, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha512_preparekey(
        addr_of_mut!((*ctx).ctx.sha_ctx.state),
        addr_of_mut!((*ctx).ctx.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha384_iv),
    );
    (*ctx).ctx.sha_ctx.bytecount_lo = SHA512_BLOCK_SIZE as u64;
    (*ctx).ctx.sha_ctx.bytecount_hi = 0;
}
// EXPORT_SYMBOL_GPL(hmac_sha384_init_usingrawkey);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha512_init_usingrawkey(ctx: *mut hmac_sha512_ctx, raw_key: *const u8, raw_key_len: usize) {
    __hmac_sha512_preparekey(
        addr_of_mut!((*ctx).ctx.sha_ctx.state),
        addr_of_mut!((*ctx).ctx.ostate),
        raw_key,
        raw_key_len,
        addr_of!(sha512_iv),
    );
    (*ctx).ctx.sha_ctx.bytecount_lo = SHA512_BLOCK_SIZE as u64;
    (*ctx).ctx.sha_ctx.bytecount_hi = 0;
}
// EXPORT_SYMBOL_GPL(hmac_sha512_init_usingrawkey);

unsafe fn __hmac_sha512_final(ctx: *mut __hmac_sha512_ctx, out: *mut u8, digest_size: usize) {
    const BLOCK: usize = SHA512_BLOCK_SIZE as usize;

    /* Generate the padded input for the outer hash in ctx->sha_ctx.buf. */
    __sha512_final(addr_of_mut!((*ctx).sha_ctx), (*ctx).sha_ctx.buf.as_mut_ptr(), digest_size);
    let buf = &mut (*ctx).sha_ctx.buf;
    buf[digest_size..BLOCK].fill(0);
    buf[digest_size] = 0x80;
    buf[BLOCK - 4..].copy_from_slice(&((8 * (BLOCK + digest_size)) as u32).to_be_bytes());

    /* Compute the outer hash, which gives the HMAC value. */
    sha512_blocks(addr_of_mut!((*ctx).ostate), (*ctx).sha_ctx.buf.as_ptr(), 1);
    let mut i = 0;
    while i < digest_size {
        out.add(i).cast::<[u8; 8]>().write_unaligned((*ctx).ostate.h[i / 8].to_be_bytes());
        i += 8;
    }

    memzero_explicit(ctx.cast(), core::mem::size_of::<__hmac_sha512_ctx>());
}

#[no_mangle]
pub unsafe extern "C" fn hmac_sha384_final(ctx: *mut hmac_sha384_ctx, out: *mut u8) {
    __hmac_sha512_final(addr_of_mut!((*ctx).ctx), out, SHA384_DIGEST_SIZE as usize);
}
// EXPORT_SYMBOL_GPL(hmac_sha384_final);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha512_final(ctx: *mut hmac_sha512_ctx, out: *mut u8) {
    __hmac_sha512_final(addr_of_mut!((*ctx).ctx), out, SHA512_DIGEST_SIZE as usize);
}
// EXPORT_SYMBOL_GPL(hmac_sha512_final);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha384(key: *const hmac_sha384_key, data: *const u8, data_len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha384_ctx>::uninit();

    hmac_sha384_init(ctx.as_mut_ptr(), key);
    hmac_sha384_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha384_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha384);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha512(key: *const hmac_sha512_key, data: *const u8, data_len: usize, out: *mut u8) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha512_ctx>::uninit();

    hmac_sha512_init(ctx.as_mut_ptr(), key);
    hmac_sha512_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha512_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha512);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha384_usingrawkey(
    raw_key: *const u8,
    raw_key_len: usize,
    data: *const u8,
    data_len: usize,
    out: *mut u8,
) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha384_ctx>::uninit();

    hmac_sha384_init_usingrawkey(ctx.as_mut_ptr(), raw_key, raw_key_len);
    hmac_sha384_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha384_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha384_usingrawkey);

#[no_mangle]
pub unsafe extern "C" fn hmac_sha512_usingrawkey(
    raw_key: *const u8,
    raw_key_len: usize,
    data: *const u8,
    data_len: usize,
    out: *mut u8,
) {
    let mut ctx = core::mem::MaybeUninit::<hmac_sha512_ctx>::uninit();

    hmac_sha512_init_usingrawkey(ctx.as_mut_ptr(), raw_key, raw_key_len);
    hmac_sha512_update(ctx.as_mut_ptr(), data, data_len);
    hmac_sha512_final(ctx.as_mut_ptr(), out);
}
// EXPORT_SYMBOL_GPL(hmac_sha512_usingrawkey);

// #if defined(sha512_mod_init_arch) || defined(CONFIG_CRYPTO_FIPS):
// sha512_mod_init_arch is defined by the arm, arm64, riscv, s390, sparc and
// x86 architecture code.
#[cfg(any(
    CONFIG_CRYPTO_FIPS,
    all(
        CONFIG_CRYPTO_LIB_SHA512_ARCH,
        any(CONFIG_ARM, CONFIG_ARM64, CONFIG_RISCV, CONFIG_S390, CONFIG_SPARC, CONFIG_X86)
    )
))]
#[link_section = ".init.text"]
unsafe extern "C" fn sha512_mod_init() -> kernel::ffi::c_int {
    #[cfg(all(
        CONFIG_CRYPTO_LIB_SHA512_ARCH,
        any(CONFIG_ARM, CONFIG_ARM64, CONFIG_RISCV, CONFIG_S390, CONFIG_SPARC, CONFIG_X86)
    ))]
    sha512_mod_init_arch();
    if fips_enabled != 0 {
        /*
         * FIPS cryptographic algorithm self-test.  As per the FIPS
         * Implementation Guidance, testing HMAC-SHA512 satisfies the
         * test requirement for SHA-384, SHA-512, and HMAC-SHA384 too.
         */
        let mut mac = [0u8; SHA512_DIGEST_SIZE as usize];

        hmac_sha512_usingrawkey(
            fips_test_key.as_ptr(),
            size_of_val(&fips_test_key),
            fips_test_data.as_ptr(),
            size_of_val(&fips_test_data),
            mac.as_mut_ptr(),
        );
        if fips_test_hmac_sha512_value[..] != mac[..] {
            panic(c"sha512: FIPS self-test failed\n".as_ptr());
        }
    }
    0
}
// subsys_initcall(sha512_mod_init);

// static void __exit sha512_mod_exit(void) {}  module_exit(sha512_mod_exit);

// MODULE_DESCRIPTION("SHA-384, SHA-512, HMAC-SHA384, and HMAC-SHA512 library functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
