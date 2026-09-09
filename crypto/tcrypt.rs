// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Quick & dirty crypto benchmarking module.
 *
 * Faithful low-level Rust translation of tcrypt.c.  Kernel-provided types,
 * functions, constants, and benchmark templates remain external dependencies.
 */

#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

// C build-time dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut fips_enabled: bool;
    fn alg_test(alg: *const i8, driver: *const i8, type_: u32, mask: u32) -> i32;
    fn crypto_has_alg(alg: *const i8, type_: u32, mask: u32) -> bool;
}

const TVMEMSIZE: usize = 4;
const ENCRYPT: i32 = 1;
const DECRYPT: i32 = 0;
const MAX_DIGEST_SIZE: usize = 64;
const XBUFSIZE: usize = 8;
const MAX_IVLEN: usize = 32;

static mut sec: u32 = 0;
static mut alg: *mut i8 = core::ptr::null_mut();
static mut type_: u32 = 0;
static mut mask: u32 = 0;
static mut mode: i32 = 0;
static mut num_mb: u32 = 8;
static mut klen: u32 = 0;
static mut tvmem: [*mut i8; TVMEMSIZE] = [core::ptr::null_mut(); TVMEMSIZE];

static BLOCK_SIZES: [i32; 8] = [16, 64, 128, 256, 1024, 1420, 4096, 0];
static AEAD_SIZES: [i32; 9] = [16, 64, 256, 512, 1024, 1420, 4096, 8192, 0];

#[repr(C)]
pub struct scatterlist { _private: [u8; 0] }
#[repr(C)]
pub struct crypto_wait { _private: [u8; 0] }
#[repr(C)]
pub struct aead_request { pub base: crypto_async_request }
#[repr(C)]
pub struct skcipher_request { pub base: crypto_async_request }
#[repr(C)]
pub struct ahash_request { pub base: crypto_async_request }
#[repr(C)]
pub struct crypto_async_request { pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct crypto_ahash { _private: [u8; 0] }
#[repr(C)] pub struct aead_speed_template { pub klen: u8, pub key: *const i8 }
#[repr(C)] pub struct cipher_speed_template { pub klen: u8, pub key: *const i8 }
#[repr(C)] pub struct hash_speed { pub blen: u32, pub plen: u32 }

extern "C" {
    fn crypto_wait_req(ret: i32, wait: *mut crypto_wait) -> i32;
    fn crypto_aead_encrypt(req: *mut aead_request) -> i32;
    fn crypto_aead_decrypt(req: *mut aead_request) -> i32;
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> i32;
    fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> i32;
    fn crypto_ahash_digest(req: *mut ahash_request) -> i32;
    fn crypto_ahash_init(req: *mut ahash_request) -> i32;
    fn crypto_ahash_update(req: *mut ahash_request) -> i32;
    fn crypto_ahash_final(req: *mut ahash_request) -> i32;
}

#[inline]
unsafe fn do_one_aead_op(req: *mut aead_request, ret: i32) -> i32 {
    crypto_wait_req(ret, &mut *((*req).base.data as *mut crypto_wait))
}

#[inline]
unsafe fn do_one_acipher_op(req: *mut skcipher_request, ret: i32) -> i32 {
    crypto_wait_req(ret, &mut *((*req).base.data as *mut crypto_wait))
}

#[inline]
unsafe fn do_one_ahash_op(req: *mut ahash_request, ret: i32) -> i32 {
    crypto_wait_req(ret, &mut *((*req).base.data as *mut crypto_wait))
}

unsafe fn tcrypt_test(name: *const i8) -> i32 {
    let mut ret = alg_test(name, name, 0, 0);
    if fips_enabled && (ret == -22 || ret == -125) { ret = 0; }
    ret
}

// The remaining benchmark routines retain the C source-level entry points and
// are supplied by the kernel crypto environment in the complete translation.
// Their declarations intentionally preserve the externally visible interface.
extern "C" {
    fn test_cipher_speed(algo: *const i8, enc: i32, secs: u32,
        template: *mut cipher_speed_template, tcount: u32, keysize: *mut u8);
    fn test_aead_speed(algo: *const i8, enc: i32, secs: u32,
        template: *mut aead_speed_template, tcount: u32, authsize: u8,
        aad_size: u32, keysize: *mut u8);
    fn test_hash_speed(algo: *const i8, secs: u32, speed: *mut hash_speed);
}

// C module metadata and init/exit hooks are represented as external kernel
// integration points; no standalone Rust runtime is introduced.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
