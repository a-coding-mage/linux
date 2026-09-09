// SPDX-License-Identifier: GPL-2.0
// The C implementation includes the platform-specific CRUSH hash header here.

use core::ffi::c_char;

// Provided by the CRUSH hash interface.
// const CRUSH_HASH_RJENKINS1: i32 = ...;

const CRUSH_HASH_SEED: u32 = 1315423911;

#[inline]
fn crush_hashmix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 13;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= (*a).wrapping_shl(8);
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 13;
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 12;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= (*a).wrapping_shl(16);
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 5;
    *a = a.wrapping_sub(*b);
    *a = a.wrapping_sub(*c);
    *a ^= *c >> 3;
    *b = b.wrapping_sub(*c);
    *b = b.wrapping_sub(*a);
    *b ^= b.wrapping_shl(10);
    *c = c.wrapping_sub(*a);
    *c = c.wrapping_sub(*b);
    *c ^= *b >> 15;
}

fn crush_hash32_rjenkins1(mut a: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ a;
    let mut b = a;
    let mut x = 231232;
    let mut y = 1232;
    crush_hashmix(&mut b, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut a, &mut hash);
    hash
}

fn crush_hash32_rjenkins1_2(mut a: u32, mut b: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ a ^ b;
    let mut x = 231232;
    let mut y = 1232;
    crush_hashmix(&mut a, &mut b, &mut hash);
    crush_hashmix(&mut x, &mut a, &mut hash);
    crush_hashmix(&mut b, &mut y, &mut hash);
    hash
}

fn crush_hash32_rjenkins1_3(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ a ^ b ^ c;
    let mut x = 231232;
    let mut y = 1232;
    crush_hashmix(&mut a, &mut b, &mut hash);
    crush_hashmix(&mut c, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut a, &mut hash);
    crush_hashmix(&mut b, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut c, &mut hash);
    hash
}

fn crush_hash32_rjenkins1_4(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ a ^ b ^ c ^ d;
    let mut x = 231232;
    let mut y = 1232;
    crush_hashmix(&mut a, &mut b, &mut hash);
    crush_hashmix(&mut c, &mut d, &mut hash);
    crush_hashmix(&mut a, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut b, &mut hash);
    crush_hashmix(&mut c, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut d, &mut hash);
    hash
}

fn crush_hash32_rjenkins1_5(mut a: u32, mut b: u32, mut c: u32, mut d: u32, mut e: u32) -> u32 {
    let mut hash = CRUSH_HASH_SEED ^ a ^ b ^ c ^ d ^ e;
    let mut x = 231232;
    let mut y = 1232;
    crush_hashmix(&mut a, &mut b, &mut hash);
    crush_hashmix(&mut c, &mut d, &mut hash);
    crush_hashmix(&mut e, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut a, &mut hash);
    crush_hashmix(&mut b, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut c, &mut hash);
    crush_hashmix(&mut d, &mut x, &mut hash);
    crush_hashmix(&mut y, &mut e, &mut hash);
    hash
}

pub fn crush_hash32(type_: i32, a: u32) -> u32 {
    match type_ {
        CRUSH_HASH_RJENKINS1 => crush_hash32_rjenkins1(a),
        _ => 0,
    }
}

pub fn crush_hash32_2(type_: i32, a: u32, b: u32) -> u32 {
    match type_ {
        CRUSH_HASH_RJENKINS1 => crush_hash32_rjenkins1_2(a, b),
        _ => 0,
    }
}

pub fn crush_hash32_3(type_: i32, a: u32, b: u32, c: u32) -> u32 {
    match type_ {
        CRUSH_HASH_RJENKINS1 => crush_hash32_rjenkins1_3(a, b, c),
        _ => 0,
    }
}

pub fn crush_hash32_4(type_: i32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    match type_ {
        CRUSH_HASH_RJENKINS1 => crush_hash32_rjenkins1_4(a, b, c, d),
        _ => 0,
    }
}

pub fn crush_hash32_5(type_: i32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    match type_ {
        CRUSH_HASH_RJENKINS1 => crush_hash32_rjenkins1_5(a, b, c, d, e),
        _ => 0,
    }
}

pub fn crush_hash_name(type_: i32) -> *const c_char {
    match type_ {
        CRUSH_HASH_RJENKINS1 => b"rjenkins1\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
