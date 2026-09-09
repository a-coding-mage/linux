// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES CTR routines supporting VMX instructions on the Power 8
 *
 * Copyright (C) 2015 International Business Machines Inc.
 *
 * Author: Marcelo Henrique Cerri <mhcerri@br.ibm.com>
 */

// External kernel, crypto, and Power8 AES declarations are supplied by the
// surrounding kernel translation unit.

#[repr(C)]
pub struct crypto_skcipher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_walk {
    pub src: skcipher_walk_addr,
    pub dst: skcipher_walk_addr,
    pub iv: *mut u8,
    pub nbytes: u32,
}

#[repr(C)]
pub struct skcipher_walk_addr {
    pub virt: skcipher_walk_virt_addr,
}

#[repr(C)]
pub struct skcipher_walk_virt_addr {
    pub addr: *mut u8,
}

#[repr(C)]
pub struct p8_aes_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skcipher_alg {
    _private: [u8; 0],
}

extern "C" {
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut core::ffi::c_void;
    fn crypto_alloc_skcipher(
        name: *const u8,
        type_: u32,
        mask: u32,
    ) -> *mut crypto_skcipher;
    fn crypto_skcipher_reqsize(tfm: *mut crypto_skcipher) -> usize;
    fn crypto_skcipher_set_reqsize(tfm: *mut crypto_skcipher, reqsize: usize);
    fn crypto_free_skcipher(tfm: *mut crypto_skcipher);
    fn crypto_skcipher_setkey(
        tfm: *mut crypto_skcipher,
        key: *const u8,
        keylen: u32,
    ) -> i32;
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> i32;
    fn skcipher_request_ctx(req: *mut skcipher_request) -> *mut core::ffi::c_void;
    fn skcipher_request_set_tfm(req: *mut skcipher_request, tfm: *mut crypto_skcipher);
    fn skcipher_walk_virt(
        walk: *mut skcipher_walk,
        req: *mut skcipher_request,
        atomic: bool,
    ) -> i32;
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: u32) -> i32;
    fn crypto_simd_usable() -> bool;
    fn crypto_xor_cpy(dst: *mut u8, src1: *const u8, src2: *const u8, nbytes: u32);
    fn crypto_inc(iv: *mut u8, len: u32);
    fn aes_p8_set_encrypt_key(key: *const u8, bits: u32, enc_key: *mut p8_aes_key) -> i32;
    fn aes_p8_encrypt(in_: *const u8, out: *mut u8, key: *const p8_aes_key);
    fn aes_p8_ctr32_encrypt_blocks(
        src: *mut u8,
        dst: *mut u8,
        blocks: u32,
        key: *const p8_aes_key,
        iv: *mut u8,
    );
    fn preempt_disable();
    fn preempt_enable();
    fn pagefault_disable();
    fn pagefault_enable();
    fn enable_kernel_vsx();
    fn disable_kernel_vsx();
    fn pr_err(fmt: *const u8, ...);
    fn ptr_err(ptr: *const core::ffi::c_void) -> isize;
}

const AES_BLOCK_SIZE: u32 = 16;
const AES_MIN_KEY_SIZE: u32 = 16;
const AES_MAX_KEY_SIZE: u32 = 32;
const CRYPTO_ALG_NEED_FALLBACK: u32 = 1 << 15;
const CRYPTO_ALG_ASYNC: u32 = 1 << 4;

#[repr(C)]
pub struct p8_aes_ctr_ctx {
    pub fallback: *mut crypto_skcipher,
    pub enc_key: p8_aes_key,
}

unsafe fn p8_aes_ctr_init(tfm: *mut crypto_skcipher) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut p8_aes_ctr_ctx;
    let fallback = crypto_alloc_skcipher(
        b"ctr(aes)\0".as_ptr(),
        0,
        CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC,
    );
    if fallback.is_null() {
        pr_err(b"Failed to allocate ctr(aes) fallback: %ld\n\0".as_ptr(), ptr_err(fallback.cast()));
        return ptr_err(fallback.cast()) as i32;
    }
    crypto_skcipher_set_reqsize(
        tfm,
        core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize(fallback),
    );
    (*ctx).fallback = fallback;
    0
}

unsafe fn p8_aes_ctr_exit(tfm: *mut crypto_skcipher) {
    let ctx = crypto_skcipher_ctx(tfm) as *mut p8_aes_ctr_ctx;
    crypto_free_skcipher((*ctx).fallback);
}

unsafe fn p8_aes_ctr_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut p8_aes_ctr_ctx;
    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    let mut ret = aes_p8_set_encrypt_key(key, keylen.wrapping_mul(8), &mut (*ctx).enc_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();
    ret |= crypto_skcipher_setkey((*ctx).fallback, key, keylen);
    if ret != 0 { -22 } else { 0 }
}

unsafe fn p8_aes_ctr_final(ctx: *const p8_aes_ctr_ctx, walk: *mut skcipher_walk) {
    let src = (*walk).src.virt.addr as *const u8;
    let ctrblk = (*walk).iv;
    let mut keystream = [0u8; AES_BLOCK_SIZE as usize];
    let dst = (*walk).dst.virt.addr;
    let nbytes = (*walk).nbytes;
    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    aes_p8_encrypt(ctrblk, keystream.as_mut_ptr(), &(*ctx).enc_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();
    crypto_xor_cpy(dst, keystream.as_ptr(), src, nbytes);
    crypto_inc(ctrblk, AES_BLOCK_SIZE);
}

unsafe fn p8_aes_ctr_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const p8_aes_ctr_ctx;
    let mut walk = core::mem::zeroed::<skcipher_walk>();
    let mut nbytes: u32;
    let mut ret: i32;
    if !crypto_simd_usable() {
        let subreq = skcipher_request_ctx(req) as *mut skcipher_request;
        core::ptr::copy_nonoverlapping(req, subreq, 1);
        skcipher_request_set_tfm(subreq, (*ctx).fallback);
        return crypto_skcipher_encrypt(subreq);
    }
    ret = skcipher_walk_virt(&mut walk, req, false);
    while { nbytes = walk.nbytes; nbytes >= AES_BLOCK_SIZE } {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        aes_p8_ctr32_encrypt_blocks(walk.src.virt.addr, walk.dst.virt.addr,
            nbytes / AES_BLOCK_SIZE, &(*ctx).enc_key, walk.iv);
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();
        loop {
            crypto_inc(walk.iv, AES_BLOCK_SIZE);
            nbytes = nbytes.wrapping_sub(AES_BLOCK_SIZE);
            if nbytes < AES_BLOCK_SIZE { break; }
        }
        ret = skcipher_walk_done(&mut walk, nbytes);
    }
    if nbytes != 0 {
        p8_aes_ctr_final(ctx, &mut walk);
        ret = skcipher_walk_done(&mut walk, 0);
    }
    ret
}

// The kernel's designated-initializer algorithm registration is retained as
// an external object layout supplied by the surrounding translation.
extern "C" {
    pub static mut p8_aes_ctr_alg: skcipher_alg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
