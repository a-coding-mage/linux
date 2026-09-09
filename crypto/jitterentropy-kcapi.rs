/*
 * Non-physical true random number generator based on timing jitter --
 * Linux Kernel Crypto API specific code
 *
 * Copyright Stephan Mueller <smueller@chronox.de>, 2015 - 2023
 *
 * This file is a source-level Rust translation of jitterentropy-kcapi.c.
 * Kernel and jitterentropy dependencies are supplied externally.
 */

// C includes and build-time kernel configuration are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u64 = u64;
type u8 = core::ffi::c_uchar;

#[repr(C)] pub struct sha3_ctx { _private: [u8; 0] }
#[repr(C)] pub struct rand_data { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
#[repr(C)] pub struct crypto_rng { _private: [u8; 0] }

extern "C" {
    fn kvzalloc(len: c_uint, flags: c_uint) -> *mut c_void;
    fn kvfree_sensitive(ptr: *mut c_void, len: c_uint);
    fn kzalloc(len: c_uint, flags: c_uint) -> *mut c_void;
    fn kfree_sensitive(ptr: *mut c_void);
    fn random_get_entropy() -> __u64;
    fn ktime_get_ns() -> __u64;
    fn jent_raw_hires_entropy_store(value: __u64);
    fn kmsan_unpoison_memory(ptr: *mut c_void, len: usize);
    fn sha3_256_init(ctx: *mut sha3_ctx);
    fn sha3_update(ctx: *mut sha3_ctx, data: *const c_void, len: usize);
    fn sha3_final(ctx: *mut sha3_ctx, data: *mut u8);
    fn memzero_explicit(ptr: *mut c_void, len: usize);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn jent_entropy_collector_free(data: *mut rand_data);
    fn jent_entropy_collector_alloc(osr: c_uint, flags: c_uint, hash: *mut sha3_ctx) -> *mut rand_data;
    fn jent_read_entropy(data: *mut rand_data, dst: *mut u8, len: c_uint) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn fips_enabled() -> c_int;
    fn panic(fmt: *const c_char, ...);
    fn jent_testing_init();
    fn jent_testing_exit();
    fn jent_entropy_init(osr: c_uint, flags: c_uint, hash: *mut sha3_ctx, arg: *mut c_void) -> c_int;
}

const SHA3_256_DIGEST_SIZE: usize = 32;

pub unsafe fn jent_kvzalloc(len: c_uint) -> *mut c_void { kvzalloc(len, 0) }
pub unsafe fn jent_kvzfree(ptr: *mut c_void, len: c_uint) { kvfree_sensitive(ptr, len); }
pub unsafe fn jent_zalloc(len: c_uint) -> *mut c_void { kzalloc(len, 0) }
pub unsafe fn jent_zfree(ptr: *mut c_void) { kfree_sensitive(ptr); }

pub unsafe fn jent_get_nstime(out: *mut __u64) {
    let mut tmp = random_get_entropy();
    if tmp == 0 { tmp = ktime_get_ns(); }
    *out = tmp;
    jent_raw_hires_entropy_store(tmp);
}

pub unsafe fn jent_hash_time(hash_state: *mut sha3_ctx, mut time: __u64,
                              addtl: *mut u8, addtl_len: c_uint,
                              hash_loop_cnt: __u64, stuck: c_uint) {
    let mut tmp_state: sha3_ctx = core::mem::zeroed();
    let mut intermediary = [0u8; SHA3_256_DIGEST_SIZE];
    kmsan_unpoison_memory(intermediary.as_mut_ptr() as *mut c_void, intermediary.len());
    let mut j = 0;
    while j < hash_loop_cnt {
        sha3_256_init(&mut tmp_state);
        sha3_update(&mut tmp_state, intermediary.as_ptr() as *const c_void, intermediary.len());
        sha3_update(&mut tmp_state, addtl as *const c_void, addtl_len as usize);
        sha3_final(&mut tmp_state, intermediary.as_mut_ptr());
        j += 1;
    }
    sha3_update(hash_state, intermediary.as_ptr() as *const c_void, intermediary.len());
    if stuck != 0 { time = 0; }
    sha3_update(hash_state, (&time as *const __u64) as *const c_void, core::mem::size_of::<__u64>());
    memzero_explicit(intermediary.as_mut_ptr() as *mut c_void, intermediary.len());
}

pub unsafe fn jent_read_random_block(hash_state: *mut sha3_ctx, dst: *mut c_char, dst_len: c_uint) {
    let mut jent_block = [0u8; SHA3_256_DIGEST_SIZE];
    sha3_final(hash_state, jent_block.as_mut_ptr());
    sha3_256_init(hash_state);
    sha3_update(hash_state, jent_block.as_ptr() as *const c_void, jent_block.len());
    if dst_len != 0 { memcpy(dst as *mut c_void, jent_block.as_ptr() as *const c_void, dst_len as usize); }
    memzero_explicit(jent_block.as_mut_ptr() as *mut c_void, jent_block.len());
}

#[repr(C)]
pub struct jitterentropy {
    pub jent_lock: mutex,
    pub entropy_collector: *mut rand_data,
    pub hash_state: sha3_ctx,
}

pub unsafe fn jent_kcapi_cleanup(tfm: *mut crypto_tfm) {
    let rng = tfm as *mut jitterentropy;
    mutex_lock(&mut (*rng).jent_lock);
    memzero_explicit(&mut (*rng).hash_state as *mut _ as *mut c_void, core::mem::size_of::<sha3_ctx>());
    if !(*rng).entropy_collector.is_null() { jent_entropy_collector_free((*rng).entropy_collector); }
    (*rng).entropy_collector = core::ptr::null_mut();
    mutex_unlock(&mut (*rng).jent_lock);
}

pub unsafe fn jent_kcapi_init(tfm: *mut crypto_tfm) -> c_int {
    let rng = tfm as *mut jitterentropy;
    mutex_init(&mut (*rng).jent_lock);
    sha3_256_init(&mut (*rng).hash_state);
    (*rng).entropy_collector = jent_entropy_collector_alloc(0, 0, &mut (*rng).hash_state);
    if (*rng).entropy_collector.is_null() { jent_kcapi_cleanup(tfm); return -12; }
    0
}

pub unsafe fn jent_kcapi_random(tfm: *mut crypto_rng, _src: *const u8, _slen: c_uint,
                                rdata: *mut u8, dlen: c_uint) -> c_int {
    let rng = tfm as *mut jitterentropy;
    mutex_lock(&mut (*rng).jent_lock);
    let mut ret = jent_read_entropy((*rng).entropy_collector, rdata, dlen);
    if ret == -3 { ret = -14; } else if ret == -2 { ret = -11; } else if ret == -1 { ret = -22; }
    mutex_unlock(&mut (*rng).jent_lock);
    ret
}

pub unsafe fn jent_kcapi_reset(_tfm: *mut crypto_rng, _seed: *const u8, _slen: c_uint) -> c_int { 0 }

pub unsafe fn jent_mod_init() -> c_int {
    let mut hash_state: sha3_ctx = core::mem::zeroed();
    jent_testing_init();
    sha3_256_init(&mut hash_state);
    let ret = jent_entropy_init(0, 0, &mut hash_state, core::ptr::null_mut());
    memzero_explicit(&mut hash_state as *mut _ as *mut c_void, core::mem::size_of::<sha3_ctx>());
    if ret != 0 { jent_testing_exit(); return -14; }
    0
}

pub unsafe fn jent_mod_exit() { jent_testing_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
