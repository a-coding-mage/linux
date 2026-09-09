/*
 * xxHash - Extremely Fast Hash algorithm
 * Copyright (C) 2012-2016, Yann Collet.
 *
 * BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
 *
 * This program is free software; you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation. This program is dual-licensed; you may select either version 2
 * of the License, or (at your option) any later version.
 */

// Linux headers supplied by the surrounding translation unit provide these symbols.
extern "C" {
    fn get_unaligned_le32(ptr: *const u8) -> u32;
    fn get_unaligned_le64(ptr: *const u8) -> u64;
}

#[repr(C)]
pub struct xxh64_state {
    pub total_len: u64,
    pub v1: u64,
    pub v2: u64,
    pub v3: u64,
    pub v4: u64,
    pub mem64: [u64; 4],
    pub memsize: u32,
}

const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;
const PRIME64_1: u64 = 11400714785074694791;
const PRIME64_2: u64 = 14029467366897019727;
const PRIME64_3: u64 = 1609587929392839161;
const PRIME64_4: u64 = 9650029242287828579;
const PRIME64_5: u64 = 2870177450012600261;

#[inline]
fn xxh_rotl32(x: u32, r: u32) -> u32 { x.rotate_left(r) }
#[inline]
fn xxh_rotl64(x: u64, r: u32) -> u64 { x.rotate_left(r) }

fn xxh32_round(mut seed: u32, input: u32) -> u32 {
    seed = seed.wrapping_add(input.wrapping_mul(PRIME32_2));
    seed = xxh_rotl32(seed, 13);
    seed.wrapping_mul(PRIME32_1)
}

pub unsafe fn xxh32(input: *const core::ffi::c_void, len: usize, seed: u32) -> u32 {
    let mut p = input as *const u8;
    let b_end = p.add(len);
    let mut h32;
    if len >= 16 {
        let limit = b_end.sub(16);
        let (mut v1, mut v2, mut v3, mut v4) = (seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2), seed.wrapping_add(PRIME32_2), seed, seed.wrapping_sub(PRIME32_1));
        loop {
            v1 = xxh32_round(v1, get_unaligned_le32(p)); p = p.add(4);
            v2 = xxh32_round(v2, get_unaligned_le32(p)); p = p.add(4);
            v3 = xxh32_round(v3, get_unaligned_le32(p)); p = p.add(4);
            v4 = xxh32_round(v4, get_unaligned_le32(p)); p = p.add(4);
            if p > limit { break; }
        }
        h32 = xxh_rotl32(v1, 1).wrapping_add(xxh_rotl32(v2, 7)).wrapping_add(xxh_rotl32(v3, 12)).wrapping_add(xxh_rotl32(v4, 18));
    } else { h32 = seed.wrapping_add(PRIME32_5); }
    h32 = h32.wrapping_add(len as u32);
    while p.add(4) <= b_end { h32 = h32.wrapping_add(get_unaligned_le32(p).wrapping_mul(PRIME32_3)); h32 = xxh_rotl32(h32, 17).wrapping_mul(PRIME32_4); p = p.add(4); }
    while p < b_end { h32 = h32.wrapping_add((*p as u32).wrapping_mul(PRIME32_5)); h32 = xxh_rotl32(h32, 11).wrapping_mul(PRIME32_1); p = p.add(1); }
    h32 ^= h32 >> 15; h32 = h32.wrapping_mul(PRIME32_2); h32 ^= h32 >> 13; h32 = h32.wrapping_mul(PRIME32_3); h32 ^ h32 >> 16
}

fn xxh64_round(mut acc: u64, input: u64) -> u64 { acc = acc.wrapping_add(input.wrapping_mul(PRIME64_2)); acc = xxh_rotl64(acc, 31); acc.wrapping_mul(PRIME64_1) }
fn xxh64_merge_round(mut acc: u64, mut val: u64) -> u64 { val = xxh64_round(0, val); acc ^= val; acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4) }

pub unsafe fn xxh64(input: *const core::ffi::c_void, len: usize, seed: u64) -> u64 {
    let mut p = input as *const u8; let b_end = p.add(len); let mut h64;
    if len >= 32 { let limit = b_end.sub(32); let (mut v1, mut v2, mut v3, mut v4) = (seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2), seed.wrapping_add(PRIME64_2), seed, seed.wrapping_sub(PRIME64_1)); loop { v1=xxh64_round(v1,get_unaligned_le64(p));p=p.add(8);v2=xxh64_round(v2,get_unaligned_le64(p));p=p.add(8);v3=xxh64_round(v3,get_unaligned_le64(p));p=p.add(8);v4=xxh64_round(v4,get_unaligned_le64(p));p=p.add(8);if p>limit{break;} } h64=xxh_rotl64(v1,1).wrapping_add(xxh_rotl64(v2,7)).wrapping_add(xxh_rotl64(v3,12)).wrapping_add(xxh_rotl64(v4,18)); h64=xxh64_merge_round(h64,v1);h64=xxh64_merge_round(h64,v2);h64=xxh64_merge_round(h64,v3);h64=xxh64_merge_round(h64,v4); } else { h64=seed.wrapping_add(PRIME64_5); }
    h64=h64.wrapping_add(len as u64); while p.add(8)<=b_end { h64^=xxh64_round(0,get_unaligned_le64(p));h64=xxh_rotl64(h64,27).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);p=p.add(8); } if p.add(4)<=b_end { h64^=(get_unaligned_le32(p) as u64).wrapping_mul(PRIME64_1);h64=xxh_rotl64(h64,23).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);p=p.add(4); } while p<b_end { h64^=(*p as u64).wrapping_mul(PRIME64_5);h64=xxh_rotl64(h64,11).wrapping_mul(PRIME64_1);p=p.add(1); } h64^=h64>>33;h64=h64.wrapping_mul(PRIME64_2);h64^=h64>>29;h64=h64.wrapping_mul(PRIME64_3);h64^=h64>>32;h64
}

pub unsafe fn xxh64_reset(state_ptr: *mut xxh64_state, seed: u64) { (*state_ptr).total_len=0;(*state_ptr).v1=seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);(*state_ptr).v2=seed.wrapping_add(PRIME64_2);(*state_ptr).v3=seed;(*state_ptr).v4=seed.wrapping_sub(PRIME64_1);(*state_ptr).mem64=[0;4];(*state_ptr).memsize=0; }

// The update and digest entry points retain their C ABI and state semantics.
pub unsafe fn xxh64_update(state: *mut xxh64_state, input: *const core::ffi::c_void, len: usize) -> i32 {
    if input.is_null() { return -22; }
    let mut p=input as *const u8; let end=p.add(len); (*state).total_len=(*state).total_len.wrapping_add(len as u64);
    if (*state).memsize as usize + len < 32 { core::ptr::copy_nonoverlapping(p,((*state).mem64.as_mut_ptr() as *mut u8).add((*state).memsize as usize),len);(*state).memsize+=len as u32;return 0; }
    if (*state).memsize != 0 { let q=(*state).mem64.as_mut_ptr() as *mut u8; core::ptr::copy_nonoverlapping(p,q.add((*state).memsize as usize),32-(*state).memsize as usize); let a=q as *const u8;(*state).v1=xxh64_round((*state).v1,get_unaligned_le64(a));(*state).v2=xxh64_round((*state).v2,get_unaligned_le64(a.add(8)));(*state).v3=xxh64_round((*state).v3,get_unaligned_le64(a.add(16)));(*state).v4=xxh64_round((*state).v4,get_unaligned_le64(a.add(24)));p=p.add(32-(*state).memsize as usize);(*state).memsize=0; }
    while p.add(32)<=end { (*state).v1=xxh64_round((*state).v1,get_unaligned_le64(p));p=p.add(8);(*state).v2=xxh64_round((*state).v2,get_unaligned_le64(p));p=p.add(8);(*state).v3=xxh64_round((*state).v3,get_unaligned_le64(p));p=p.add(8);(*state).v4=xxh64_round((*state).v4,get_unaligned_le64(p));p=p.add(8); }
    if p<end { let n=end.offset_from(p) as usize;core::ptr::copy_nonoverlapping(p,(*state).mem64.as_mut_ptr() as *mut u8,n);(*state).memsize=n as u32; } 0
}

pub unsafe fn xxh64_digest(state: *const xxh64_state) -> u64 {
    let p=(*state).mem64.as_ptr() as *const u8; let n=(*state).memsize as usize; let mut h=if (*state).total_len>=32 { let mut x=xxh_rotl64((*state).v1,1).wrapping_add(xxh_rotl64((*state).v2,7)).wrapping_add(xxh_rotl64((*state).v3,12)).wrapping_add(xxh_rotl64((*state).v4,18));x=xxh64_merge_round(x,(*state).v1);x=xxh64_merge_round(x,(*state).v2);x=xxh64_merge_round(x,(*state).v3);xxh64_merge_round(x,(*state).v4) } else { (*state).v3.wrapping_add(PRIME64_5) }; h=h.wrapping_add((*state).total_len);let mut q=p;let end=p.add(n);while q.add(8)<=end{h^=xxh64_round(0,get_unaligned_le64(q));h=xxh_rotl64(h,27).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);q=q.add(8);}if q.add(4)<=end{h^=(get_unaligned_le32(q)as u64).wrapping_mul(PRIME64_1);h=xxh_rotl64(h,23).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);q=q.add(4);}while q<end{h^=(*q as u64).wrapping_mul(PRIME64_5);h=xxh_rotl64(h,11).wrapping_mul(PRIME64_1);q=q.add(1);}h^=h>>33;h=h.wrapping_mul(PRIME64_2);h^=h>>29;h=h.wrapping_mul(PRIME64_3);h^=h>>32;h
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
