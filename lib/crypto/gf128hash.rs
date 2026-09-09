// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GF(2^128) polynomial hashing: GHASH and POLYVAL
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: <crypto/gf128hash.h>, architecture implementation headers,
// Linux export/module/string/unaligned helpers.

pub const GHASH_BLOCK_SIZE: usize = 16;
pub const POLYVAL_BLOCK_SIZE: usize = 16;

#[repr(C)]
pub struct PolyvalElem {
    pub lo: u64,
    pub hi: u64,
    pub bytes: [u8; 16],
}

#[repr(C)]
pub struct GhashKey { pub h: PolyvalElem }
#[repr(C)]
pub struct PolyvalKey { pub h: PolyvalElem }
#[repr(C)]
pub struct GhashCtx { pub acc: PolyvalElem, pub key: *const GhashKey, pub partial: usize }
#[repr(C)]
pub struct PolyvalCtx { pub acc: PolyvalElem, pub key: *const PolyvalKey, pub partial: usize }

#[cfg(feature = "config_arch_supports_int128")]
unsafe fn clmul64(a: u64, b: u64, out_lo: *mut u64, out_hi: *mut u64) {
    let a0 = a & 0x1111111111111110;
    let a1 = a & 0x2222222222222220;
    let a2 = a & 0x4444444444444440;
    let a3 = a & 0x8888888888888880;
    let b0 = b & 0x1111111111111111;
    let b1 = b & 0x2222222222222222;
    let b2 = b & 0x4444444444444444;
    let b3 = b & 0x8888888888888888;
    let c0 = (a0 as u128 * b0 as u128) ^ (a1 as u128 * b3 as u128) ^
        (a2 as u128 * b2 as u128) ^ (a3 as u128 * b1 as u128);
    let c1 = (a0 as u128 * b1 as u128) ^ (a1 as u128 * b0 as u128) ^
        (a2 as u128 * b3 as u128) ^ (a3 as u128 * b2 as u128);
    let c2 = (a0 as u128 * b2 as u128) ^ (a1 as u128 * b1 as u128) ^
        (a2 as u128 * b0 as u128) ^ (a3 as u128 * b3 as u128);
    let c3 = (a0 as u128 * b3 as u128) ^ (a1 as u128 * b2 as u128) ^
        (a2 as u128 * b1 as u128) ^ (a3 as u128 * b0 as u128);
    let e0 = 0u64.wrapping_sub(a & 1) & b;
    let e1 = 0u64.wrapping_sub((a >> 1) & 1) & b;
    let e2 = 0u64.wrapping_sub((a >> 2) & 1) & b;
    let e3 = 0u64.wrapping_sub((a >> 3) & 1) & b;
    let extra_lo = e0 ^ (e1 << 1) ^ (e2 << 2) ^ (e3 << 3);
    let extra_hi = (e1 >> 63) ^ (e2 >> 62) ^ (e3 >> 61);
    *out_lo = (c0 as u64 & 0x1111111111111111) ^ (c1 as u64 & 0x2222222222222222) ^
        (c2 as u64 & 0x4444444444444444) ^ (c3 as u64 & 0x8888888888888888) ^ extra_lo;
    *out_hi = ((c0 >> 64) as u64 & 0x1111111111111111) ^ ((c1 >> 64) as u64 & 0x2222222222222222) ^
        ((c2 >> 64) as u64 & 0x4444444444444444) ^ ((c3 >> 64) as u64 & 0x8888888888888888) ^ extra_hi;
}

#[cfg(not(feature = "config_arch_supports_int128"))]
unsafe fn clmul32(a: u32, b: u32) -> u64 {
    let a0 = a & 0x11111111; let a1 = a & 0x22222222;
    let a2 = a & 0x44444444; let a3 = a & 0x88888888;
    let b0 = b & 0x11111111; let b1 = b & 0x22222222;
    let b2 = b & 0x44444444; let b3 = b & 0x88888888;
    let c0 = a0 as u64 * b0 as u64 ^ a1 as u64 * b3 as u64 ^ a2 as u64 * b2 as u64 ^ a3 as u64 * b1 as u64;
    let c1 = a0 as u64 * b1 as u64 ^ a1 as u64 * b0 as u64 ^ a2 as u64 * b3 as u64 ^ a3 as u64 * b2 as u64;
    let c2 = a0 as u64 * b2 as u64 ^ a1 as u64 * b1 as u64 ^ a2 as u64 * b0 as u64 ^ a3 as u64 * b3 as u64;
    let c3 = a0 as u64 * b3 as u64 ^ a1 as u64 * b2 as u64 ^ a2 as u64 * b1 as u64 ^ a3 as u64 * b0 as u64;
    c0 & 0x1111111111111111 ^ c1 & 0x2222222222222222 ^ c2 & 0x4444444444444444 ^ c3 & 0x8888888888888888
}

#[cfg(not(feature = "config_arch_supports_int128"))]
unsafe fn clmul64(a: u64, b: u64, out_lo: *mut u64, out_hi: *mut u64) {
    let lo = clmul32(a as u32, b as u32);
    let hi = clmul32((a >> 32) as u32, (b >> 32) as u32);
    let mi = clmul32((a as u32) ^ (a >> 32) as u32, (b as u32) ^ (b >> 32) as u32) ^ lo ^ hi;
    *out_lo = lo ^ (mi << 32); *out_hi = hi ^ (mi >> 32);
}

unsafe fn polyval_mul_generic(a: *mut PolyvalElem, b: *const PolyvalElem) {
    let (mut c0, mut c1, mut c2, mut c3, mut mi0, mut mi1) = (0, 0, 0, 0, 0, 0);
    clmul64(u64::from_le((*a).lo), u64::from_le((*b).lo), &mut c0, &mut c1);
    clmul64(u64::from_le((*a).hi), u64::from_le((*b).hi), &mut c2, &mut c3);
    clmul64(u64::from_le((*a).lo ^ (*a).hi), u64::from_le((*b).lo ^ (*b).hi), &mut mi0, &mut mi1);
    mi0 ^= c0 ^ c2; mi1 ^= c1 ^ c3; c1 ^= mi0; c2 ^= mi1;
    c1 ^= c0 << 63 ^ c0 << 62 ^ c0 << 57;
    c2 ^= c0 ^ c0 >> 1 ^ c0 >> 2 ^ c0 >> 7;
    c2 ^= c1 << 63 ^ c1 << 62 ^ c1 << 57;
    c3 ^= c1 ^ c1 >> 1 ^ c1 >> 2 ^ c1 >> 7;
    (*a).lo = c2.to_le(); (*a).hi = c3.to_le();
}

unsafe fn ghash_key_to_polyval(input: *const u8, out: *mut PolyvalElem) {
    let hi = u64::from_be_bytes(*(input as *const [u8; 8]));
    let lo = u64::from_be_bytes(*(input.add(8) as *const [u8; 8]));
    let mask = (hi as i64 >> 63) as u64;
    let hi = (hi << 1) ^ (lo >> 63) ^ (mask & (0xc2u64 << 56));
    let lo = (lo << 1) ^ (mask & 1);
    (*out).lo = lo.to_le(); (*out).hi = hi.to_le();
}

unsafe fn polyval_acc_to_ghash(input: *const PolyvalElem, out: *mut u8) {
    *(out as *mut [u8; 8]) = u64::from_le((*input).hi).to_be_bytes();
    *(out.add(8) as *mut [u8; 8]) = u64::from_le((*input).lo).to_be_bytes();
}

unsafe fn ghash_acc_to_polyval(input: *const u8, out: *mut PolyvalElem) {
    (*out).lo = u64::from_be_bytes(*(input.add(8) as *const [u8; 8])).to_le();
    (*out).hi = u64::from_be_bytes(*(input as *const [u8; 8])).to_le();
}

pub unsafe fn ghash_preparekey(key: *mut GhashKey, raw_key: *const u8) { ghash_key_to_polyval(raw_key, &mut (*key).h); }

unsafe fn ghash_mul(ctx: *mut GhashCtx) { polyval_mul_generic(&mut (*ctx).acc, &(*(*ctx).key).h); }

unsafe fn ghash_blocks(ctx: *mut GhashCtx, data: *const u8, nblocks: usize) {
    let mut p = data;
    for _ in 0..nblocks {
        (*ctx).acc.lo ^= u64::from_be_bytes(*(p.add(8) as *const [u8; 8])).to_le();
        (*ctx).acc.hi ^= u64::from_be_bytes(*(p as *const [u8; 8])).to_le();
        ghash_mul(ctx); p = p.add(GHASH_BLOCK_SIZE);
    }
}

pub unsafe fn ghash_update(ctx: *mut GhashCtx, mut data: *const u8, mut len: usize) {
    if (*ctx).partial != 0 {
        let n = core::cmp::min(len, GHASH_BLOCK_SIZE - (*ctx).partial); len -= n;
        for _ in 0..n { (*ctx).acc.bytes[GHASH_BLOCK_SIZE - 1 - (*ctx).partial] ^= *data; (*ctx).partial += 1; data = data.add(1); }
        if (*ctx).partial < GHASH_BLOCK_SIZE { return; } ghash_mul(ctx);
    }
    if len >= GHASH_BLOCK_SIZE { let n = len / GHASH_BLOCK_SIZE; ghash_blocks(ctx, data, n); data = data.add(len & !(GHASH_BLOCK_SIZE - 1)); len &= GHASH_BLOCK_SIZE - 1; }
    for i in 0..len { (*ctx).acc.bytes[GHASH_BLOCK_SIZE - 1 - i] ^= *data.add(i); } (*ctx).partial = len;
}

pub unsafe fn ghash_final(ctx: *mut GhashCtx, out: *mut u8) { if (*ctx).partial != 0 { ghash_mul(ctx); } polyval_acc_to_ghash(&(*ctx).acc, out); core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<GhashCtx>()); }

pub unsafe fn polyval_preparekey(key: *mut PolyvalKey, raw_key: *const u8) { core::ptr::copy_nonoverlapping(raw_key, (*key).h.bytes.as_mut_ptr(), POLYVAL_BLOCK_SIZE); (*key).h.lo = u64::from_le_bytes((*key).h.bytes[0..8].try_into().unwrap()); (*key).h.hi = u64::from_le_bytes((*key).h.bytes[8..16].try_into().unwrap()); }

unsafe fn polyval_mul(ctx: *mut PolyvalCtx) { polyval_mul_generic(&mut (*ctx).acc, &(*(*ctx).key).h); }

unsafe fn polyval_blocks(ctx: *mut PolyvalCtx, data: *const u8, nblocks: usize) { let mut p = data; for _ in 0..nblocks { (*ctx).acc.lo ^= u64::from_le_bytes(*(p as *const [u8; 8])); (*ctx).acc.hi ^= u64::from_le_bytes(*(p.add(8) as *const [u8; 8])); polyval_mul(ctx); p = p.add(POLYVAL_BLOCK_SIZE); } }

pub unsafe fn polyval_update(ctx: *mut PolyvalCtx, mut data: *const u8, mut len: usize) {
    if (*ctx).partial != 0 { let n = core::cmp::min(len, POLYVAL_BLOCK_SIZE - (*ctx).partial); len -= n; for _ in 0..n { (*ctx).acc.bytes[(*ctx).partial] ^= *data; (*ctx).partial += 1; data = data.add(1); } if (*ctx).partial < POLYVAL_BLOCK_SIZE { return; } polyval_mul(ctx); }
    if len >= POLYVAL_BLOCK_SIZE { let n = len / POLYVAL_BLOCK_SIZE; polyval_blocks(ctx, data, n); data = data.add(len & !(POLYVAL_BLOCK_SIZE - 1)); len &= POLYVAL_BLOCK_SIZE - 1; }
    for i in 0..len { (*ctx).acc.bytes[i] ^= *data.add(i); } (*ctx).partial = len;
}

pub unsafe fn polyval_final(ctx: *mut PolyvalCtx, out: *mut u8) { if (*ctx).partial != 0 { polyval_mul(ctx); } core::ptr::copy_nonoverlapping(&(*ctx).acc as *const PolyvalElem as *const u8, out, POLYVAL_BLOCK_SIZE); core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<PolyvalCtx>()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
