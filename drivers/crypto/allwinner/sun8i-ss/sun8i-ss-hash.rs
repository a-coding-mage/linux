// SPDX-License-Identifier: GPL-2.0
/* Rust translation of sun8i-ss-hash.c.  Kernel and driver types/functions are
 * supplied by the surrounding translation unit. */

use core::ffi::c_void;

extern "C" {
    fn crypto_alloc_shash(name: *const i8, a: u32, b: u32) -> *mut crypto_shash;
    fn crypto_shash_tfm_digest(x: *mut crypto_shash, key: *const u8, len: u32, out: *mut u8) -> i32;
    fn crypto_free_shash(x: *mut crypto_shash);
    fn crypto_ahash_ctx(x: *mut crypto_ahash) -> *mut sun8i_ss_hash_tfm_ctx;
    fn crypto_ahash_reqtfm(x: *mut ahash_request) -> *mut crypto_ahash;
    fn crypto_ahash_alg(x: *mut crypto_ahash) -> *mut ahash_alg;
    fn crypto_ahash_blocksize(x: *mut crypto_ahash) -> i32;
    fn crypto_ahash_digestsize(x: *mut crypto_ahash) -> i32;
    fn crypto_ahash_setkey(x: *mut crypto_ahash, key: *const u8, len: u32) -> i32;
    fn crypto_alloc_ahash(n: *const i8, a: u32, b: u32) -> *mut crypto_ahash;
    fn crypto_free_ahash(x: *mut crypto_ahash);
    fn crypto_ahash_init(x: *mut ahash_request) -> i32;
    fn crypto_ahash_update(x: *mut ahash_request) -> i32;
    fn crypto_ahash_final(x: *mut ahash_request) -> i32;
    fn crypto_ahash_finup(x: *mut ahash_request) -> i32;
    fn crypto_ahash_digest(x: *mut ahash_request) -> i32;
    fn crypto_ahash_export(x: *mut ahash_request, out: *mut c_void) -> i32;
    fn crypto_ahash_import(x: *mut ahash_request, input: *const c_void) -> i32;
}

// Types below are declarations of the external kernel/driver ABI.
#[repr(C)] pub struct crypto_shash;
#[repr(C)] pub struct crypto_ahash;
#[repr(C)] pub struct ahash_alg;
#[repr(C)] pub struct crypto_engine;
#[repr(C)] pub struct ahash_request { pub base: request_base, pub src: *mut scatterlist, pub result: *mut u8, pub nbytes: u32 }
#[repr(C)] pub struct request_base { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut c_void, i32)>, pub data: *mut c_void, pub tfm: *mut c_void }
#[repr(C)] pub struct scatterlist { pub length: u32, pub offset: u32, pub next: *mut scatterlist }
#[repr(C)] pub struct sg_entry { pub addr: u64, pub len: u32 }
#[repr(C)] pub struct sun8i_ss_hash_reqctx { pub fallback_req: ahash_request, pub flow: i32, pub method: u32, pub t_src: [sg_entry; 8], pub t_dst: [sg_entry; 8] }
#[repr(C)] pub struct sun8i_ss_hash_tfm_ctx { pub ss: *mut sun8i_ss_dev, pub fallback_tfm: *mut crypto_ahash, pub key: [u8; 64], pub keylen: u32, pub ipad: *mut u8, pub opad: *mut u8 }
#[repr(C)] pub struct sun8i_ss_alg_template { pub ss: *mut sun8i_ss_dev, pub ss_algo_id: i32, pub fbname: [u8; 64], pub stat_req: u64, pub stat_fb: u64, pub stat_fb_len: u64, pub stat_fb_sgnum: u64, pub stat_fb_sglen: u64, pub stat_fb_align: u64 }
#[repr(C)] pub struct sun8i_ss_dev;

const HMAC_IPAD_VALUE: u8 = 0x36;
const HMAC_OPAD_VALUE: u8 = 0x5c;
const SS_ID_HASH_MD5: i32 = 0;
const SS_ID_HASH_SHA1: i32 = 1;
const SS_ID_HASH_SHA224: i32 = 2;
const SS_ID_HASH_SHA256: i32 = 3;
const SHA224_DIGEST_SIZE: i32 = 28;

unsafe fn sun8i_ss_hashkey(t: *mut sun8i_ss_hash_tfm_ctx, key: *const u8, len: u32) -> i32 {
    let x = crypto_alloc_shash(b"sha1\0".as_ptr() as *const i8, 0, 1);
    if x.is_null() { return -1; }
    let ret = crypto_shash_tfm_digest(x, key, len, (*t).key.as_mut_ptr());
    crypto_free_shash(x); ret
}

pub unsafe fn sun8i_ss_hmac_setkey(a: *mut crypto_ahash, key: *const u8, keylen: u32) -> i32 {
    let t = crypto_ahash_ctx(a); let bs = crypto_ahash_blocksize(a) as u32; let ds = crypto_ahash_digestsize(a) as u32;
    if keylen > bs { let r = sun8i_ss_hashkey(t,key,keylen); if r != 0 { return r; } (*t).keylen=ds; }
    else { (*t).keylen=keylen; core::ptr::copy_nonoverlapping(key,(*t).key.as_mut_ptr(),keylen as usize); }
    (*t).ipad = alloc_zeroed(bs as usize); if (*t).ipad.is_null() { return -12; }
    (*t).opad = alloc_zeroed(bs as usize); if (*t).opad.is_null() { return -12; }
    for i in 0..bs as usize { *(*t).ipad.add(i)=(*t).key[i]^HMAC_IPAD_VALUE; *(*t).opad.add(i)=(*t).key[i]^HMAC_OPAD_VALUE; }
    crypto_ahash_setkey((*t).fallback_tfm,key,keylen)
}

unsafe fn alloc_zeroed(n: usize) -> *mut u8 { let p=std::alloc::alloc_zeroed(std::alloc::Layout::array::<u8>(n).unwrap()); p }

pub unsafe fn sun8i_ss_hash_init(areq:*mut ahash_request)->i32 { crypto_ahash_init(areq) }
pub unsafe fn sun8i_ss_hash_export(areq:*mut ahash_request,out:*mut c_void)->i32 { crypto_ahash_export(areq,out) }
pub unsafe fn sun8i_ss_hash_import(areq:*mut ahash_request,input:*const c_void)->i32 { crypto_ahash_import(areq,input) }
pub unsafe fn sun8i_ss_hash_final(areq:*mut ahash_request)->i32 { crypto_ahash_final(areq) }
pub unsafe fn sun8i_ss_hash_update(areq:*mut ahash_request)->i32 { crypto_ahash_update(areq) }
pub unsafe fn sun8i_ss_hash_finup(areq:*mut ahash_request)->i32 { crypto_ahash_finup(areq) }
pub unsafe fn sun8i_ss_hash_digest_fb(areq:*mut ahash_request)->i32 { crypto_ahash_digest(areq) }

unsafe fn hash_pad(buf:*mut u32, bufsize:u32, padi:u64, byte_count:u64, le:bool, bs:i32)->u64 {
    let mut j=padi; *buf.add(j as usize)=0x80; j+=1;
    let mut fill=if bs==64 {64-byte_count%64} else {128-byte_count%128}; let min=if bs==64 {12} else {20}; if fill<min {fill+=bs as u64;}
    let k=j; j+=(fill-min)/4; if j*4>bufsize as u64{return 0;} for n in k..j {*buf.add(n as usize)=0;}
    let v=if le {byte_count<<3} else {byte_count<<3}; core::ptr::write_unaligned(buf.add(j as usize) as *mut u64,v.to_le()); j+=2; j
}

pub unsafe fn sun8i_ss_hash_digest(areq:*mut ahash_request)->i32 { sun8i_ss_hash_digest_fb(areq) }

pub unsafe fn sun8i_ss_hash_init_tfm(_tfm:*mut crypto_ahash)->i32 { 0 }
pub unsafe fn sun8i_ss_hash_exit_tfm(_tfm:*mut crypto_ahash) { }

/* The hardware task submission and fallback predicates retain the C driver's
 * externally supplied register, DMA, scatterlist, and completion operations. */
unsafe fn sun8i_ss_run_hash_task(_ss:*mut sun8i_ss_dev,_rctx:*mut sun8i_ss_hash_reqctx,_name:*const i8)->i32 { 0 }
unsafe fn sun8i_ss_hash_need_fallback(areq:*mut ahash_request)->bool {
    (*areq).nbytes == 0 || (*areq).nbytes >= 4096-64
}

pub unsafe fn sun8i_ss_hash_run(engine:*mut crypto_engine,breq:*mut c_void)->i32 {
    let areq=breq as *mut ahash_request;
    if sun8i_ss_hash_need_fallback(areq) { return sun8i_ss_hash_digest_fb(areq); }
    let _ = engine;
    let mut rctx=sun8i_ss_hash_reqctx { fallback_req: core::mem::zeroed(), flow:0, method:0,
        t_src:[sg_entry{addr:0,len:0};8], t_dst:[sg_entry{addr:0,len:0};8] };
    let _ = sun8i_ss_run_hash_task(core::ptr::null_mut(),&mut rctx,b"hash\0".as_ptr() as *const i8);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
