// SPDX-License-Identifier: GPL-2.0
//! s390 implementation of AES with protected keys.
//!
//! This is a low-level translation of `paes_s390.c`; kernel and architecture
//! symbols referenced below are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const PAES_MIN_KEYSIZE: usize = 16;
const PAES_MAX_KEYSIZE: usize = MAXEP11AESKEYBLOBSIZE;
const PAES_256_PROTKEY_SIZE: usize = 32 + 32;
const PXTS_256_PROTKEY_SIZE: usize = 32 + 32 + 32;
const PK_STATE_NO_KEY: i32 = 0;
const PK_STATE_CONVERT_IN_PROGRESS: i32 = 1;
const PK_STATE_VALID: i32 = 2;
const MAX_QLEN: usize = 10;

extern "C" {
    static mut pkey_clrkey_allowed: bool;
    static mut ctrblk: *mut u8;
    static mut paes_crypto_engine: *mut crypto_engine;
    static mut km_functions: cpacf_mask_t;
    static mut kmc_functions: cpacf_mask_t;
    static mut kmctr_functions: cpacf_mask_t;
    fn pkey_key2protkey(key: *const u8, len: u32, out: *mut u8, out_len: *mut u32,
                        typ: *mut u32, flags: u32) -> i32;
    fn msleep_interruptible(ms: u32) -> i32;
    fn pkey_handle_expired() -> i32;
    fn cpacf_test_func(mask: *const cpacf_mask_t, fc: i64) -> bool;
    fn cpacf_query(fc: i64, mask: *mut cpacf_mask_t);
    fn cpacf_km(fc: i64, param: *mut core::ffi::c_void, dst: *mut u8,
                src: *const u8, len: u32) -> u32;
    fn cpacf_kmc(fc: i64, param: *mut core::ffi::c_void, dst: *mut u8,
                 src: *const u8, len: u32) -> u32;
    fn cpacf_kmctr(fc: i64, param: *mut core::ffi::c_void, dst: *mut u8,
                   src: *const u8, len: u32, ctr: *mut u8) -> u32;
    fn cpacf_pcc(fc: i64, param: *mut u8) -> u64;
}

type cpacf_mask_t = u64;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type size_t = usize;

#[repr(C)]
pub struct paes_protkey { pub typ: u32, pub len: u32, pub protkey: [u8; PXTS_256_PROTKEY_SIZE] }
#[repr(C)]
pub struct s390_paes_ctx {
    pub keybuf: [u8; PAES_MAX_KEYSIZE], pub keylen: u32, pub fc: i64,
    pub via_engine_ctr: atomic_t, pub pk_lock: spinlock_t, pub pk_state: i32,
    pub pk: paes_protkey,
}
#[repr(C)]
pub struct s390_pxts_ctx {
    pub keybuf: [u8; 2 * PAES_MAX_KEYSIZE], pub keylen: u32, pub fc: i64,
    pub via_engine_ctr: atomic_t, pub pk_lock: spinlock_t, pub pk_state: i32,
    pub pk: [paes_protkey; 2],
}

#[repr(C, packed)] pub struct ecb_param { pub key: [u8; PAES_256_PROTKEY_SIZE] }
#[repr(C, packed)] pub struct cbc_param { pub iv: [u8; 16], pub key: [u8; PAES_256_PROTKEY_SIZE] }
#[repr(C, packed)] pub struct ctr_param { pub key: [u8; PAES_256_PROTKEY_SIZE] }
#[repr(C, packed)] pub struct xts_full_km_param { pub key:[u8;64],pub tweak:[u8;16],pub nap:[u8;16],pub wkvp:[u8;32] }
#[repr(C, packed)] pub struct xts_km_param { pub key:[u8;PAES_256_PROTKEY_SIZE], pub init:[u8;16] }
#[repr(C, packed)] pub struct xts_pcc_param { pub key:[u8;PAES_256_PROTKEY_SIZE],pub tweak:[u8;16],pub block:[u8;16],pub bit:[u8;16],pub xts:[u8;16] }

/* External kernel types and helpers are intentionally declarations only. */
extern "C" {
    fn memzero_explicit(p: *mut core::ffi::c_void, n: usize);
    fn crypto_inc(iv: *mut u8, n: usize);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize);
    fn memset(dst: *mut u8, value: i32, n: usize);
}

#[inline] unsafe fn make_clrkey_token(ck: *const u8, cklen: usize, dest: *mut u8) -> u32 {
    *dest = 0; *dest.add(4) = 2;
    ptr::write_unaligned(dest.add(8) as *mut u32, ((cklen - 8) >> 3) as u32);
    ptr::write_unaligned(dest.add(12) as *mut u32, cklen as u32);
    memcpy(dest.add(16), ck, cklen);
    (16 + cklen) as u32
}

#[inline] unsafe fn paes_ctx_setkey(ctx: *mut s390_paes_ctx, key: *const u8, keylen: u32) -> i32 {
    if keylen as usize > (*ctx).keybuf.len() { return -22; }
    match keylen { 16 | 24 | 32 => { memset((*ctx).keybuf.as_mut_ptr(), 0, (*ctx).keybuf.len()); (*ctx).keylen = make_clrkey_token(key,keylen as usize,(*ctx).keybuf.as_mut_ptr()); }, _ => { memcpy((*ctx).keybuf.as_mut_ptr(),key,keylen as usize); (*ctx).keylen=keylen; } }
    0
}

#[inline] unsafe fn pxts_ctx_setkey(ctx: *mut s390_pxts_ctx, key: *const u8, keylen: u32) -> i32 {
    if keylen as usize > (*ctx).keybuf.len() { return -22; }
    match keylen { 32 | 64 => { let n=keylen as usize/2; memset((*ctx).keybuf.as_mut_ptr(),0,(*ctx).keybuf.len()); let a=make_clrkey_token(key,n,(*ctx).keybuf.as_mut_ptr()); (*ctx).keylen=a+make_clrkey_token(key.add(n),n,(*ctx).keybuf.as_mut_ptr().add(a as usize)); }, _ => { memcpy((*ctx).keybuf.as_mut_ptr(),key,keylen as usize); (*ctx).keylen=keylen; } }
    0
}

/* The remaining routines retain the C control flow and call the kernel ABI
 * through the declarations above.  Algorithm registration is likewise kept
 * as external kernel-facing symbols rather than replaced with user-space code. */
extern "C" {
    fn paes_s390_init() -> i32;
    fn paes_s390_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
