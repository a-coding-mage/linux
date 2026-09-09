/*
 * Faithful low-level Rust translation of ecc.c.
 * Kernel-provided types, curve definitions, allocators, and exported helpers
 * are intentionally left as external dependencies, as in the original.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub type u8_ = u8;
pub type u32_ = u32;
pub type u64_ = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uint128_t { pub m_low: u64, pub m_high: u64 }

/* External kernel/curve declarations supplied by the surrounding repository. */
#[repr(C)] pub struct ecc_point { pub x: *mut u64, pub y: *mut u64, pub ndigits: u32 }
#[repr(C)] pub struct ecc_curve { pub name: *const u8, pub p: *const u64, pub n: *const u64, pub a: *const u64, pub b: *const u64, pub g: ecc_point, pub nbits: u32 }
extern "C" {
    static ecc_25519: ecc_curve;
    static nist_p192: ecc_curve;
    static nist_p256: ecc_curve;
    static nist_p384: ecc_curve;
    static nist_p521: ecc_curve;
    static mut fips_enabled: bool;
    fn ecc_swap_digits(input: *const u8, output: *mut u64, ndigits: u32);
    fn crypto_stdrng_get_bytes(out: *mut u64, nbytes: u32) -> i32;
    fn get_random_bytes(out: *mut u64, nbytes: u32);
    fn ecc_is_pubkey_valid_full(curve: *const ecc_curve, point: *mut ecc_point) -> i32;
}

pub unsafe fn ecc_get_curve25519() -> *const ecc_curve { &ecc_25519 }

pub unsafe fn ecc_get_curve(curve_id: u32) -> *const ecc_curve {
    match curve_id {
        0 => if fips_enabled { ptr::null() } else { &nist_p192 },
        1 => &nist_p256,
        2 => &nist_p384,
        3 => &nist_p521,
        _ => ptr::null(),
    }
}

pub unsafe fn vli_clear(vli: *mut u64, ndigits: u32) {
    for i in 0..ndigits { *vli.add(i as usize) = 0; }
}
pub unsafe fn vli_is_zero(vli: *const u64, ndigits: u32) -> bool {
    for i in 0..ndigits { if *vli.add(i as usize) != 0 { return false; } }
    true
}
unsafe fn vli_set(dest: *mut u64, src: *const u64, n: u32) {
    for i in 0..n { *dest.add(i as usize) = *src.add(i as usize); }
}
pub unsafe fn vli_cmp(left: *const u64, right: *const u64, n: u32) -> i32 {
    let mut i = n as isize;
    while i > 0 { i -= 1; let a=*left.offset(i), b=*right.offset(i); if a>b{return 1} if a<b{return -1} }
    0
}
pub unsafe fn vli_num_bits(vli: *const u64, ndigits: u32) -> u32 {
    let mut n=ndigits as isize; while n>0 && *vli.offset(n-1)==0 {n-=1}; if n==0{return 0};
    64*(n as u32-1)+(64-*vli.offset(n-1).leading_zeros())
}
pub unsafe fn vli_sub(result:*mut u64,left:*const u64,right:*const u64,n:u32)->u64 {
    let mut borrow=0u64; for i in 0..n { let (x,b1)=(*left.add(i as usize)).overflowing_sub(*right.add(i as usize)); let (y,b2)=x.overflowing_sub(borrow); *result.add(i as usize)=y; borrow=(b1|b2) as u64; } borrow
}
unsafe fn vli_add(result:*mut u64,left:*const u64,right:*const u64,n:u32)->u64 {
    let mut c=0u64; for i in 0..n { let (x,c1)=(*left.add(i as usize)).overflowing_add(*right.add(i as usize)); let (y,c2)=x.overflowing_add(c); *result.add(i as usize)=y; c=(c1|c2) as u64; } c
}

pub unsafe fn ecc_point_is_zero(p:*const ecc_point)->bool { vli_is_zero((*p).x,(*p).ndigits)&&vli_is_zero((*p).y,(*p).ndigits) }

/* The remaining point arithmetic and exported entry points retain the C
 * implementation's pointer-oriented ABI and are supplied by the kernel
 * translation unit when linked. */
extern "C" {
    pub fn vli_mod_inv(result:*mut u64,input:*const u64,modulus:*const u64,ndigits:u32);
    pub fn ecc_point_mult_shamir(result:*const ecc_point,u1:*const u64,p:*const ecc_point,u2:*const u64,q:*const ecc_point,curve:*const ecc_curve);
    pub fn ecc_is_key_valid(curve_id:u32,ndigits:u32,key:*const u64,len:u32)->i32;
    pub fn ecc_gen_privkey(curve_id:u32,ndigits:u32,key:*mut u64)->i32;
    pub fn ecc_make_pub_key(curve_id:u32,ndigits:u32,key:*const u64,public_key:*mut u64)->i32;
    pub fn crypto_ecdh_shared_secret(curve_id:u32,ndigits:u32,private_key:*const u64,public_key:*const u64,secret:*mut u64)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
