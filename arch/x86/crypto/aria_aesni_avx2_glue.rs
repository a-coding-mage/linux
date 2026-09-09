/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Glue Code for the AVX2/AES-NI/GFNI assembler implementation of the ARIA Cipher
 *
 * Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
 */

// C dependencies supplied by the surrounding kernel and architecture code.

extern "C" {
    pub fn aria_aesni_avx2_encrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_decrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_ctr_crypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, keystream: *mut u8, iv: *mut u8);
    pub fn aria_aesni_avx2_gfni_encrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_gfni_decrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_gfni_ctr_crypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, keystream: *mut u8, iv: *mut u8);
}

#[repr(C)]
pub struct aria_avx_ops {
    pub aria_encrypt_16way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8),
    pub aria_decrypt_16way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8),
    pub aria_ctr_crypt_16way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8, *mut u8, *mut u8),
    pub aria_encrypt_32way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8),
    pub aria_decrypt_32way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8),
    pub aria_ctr_crypt_32way: unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8, *mut u8, *mut u8),
}

static mut aria_ops: aria_avx_ops = unsafe { core::mem::zeroed() };

#[repr(C)]
pub struct aria_avx2_request_ctx {
    pub keystream: [u8; ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE],
}

// ECB_WALK_START/ECB_BLOCK/ECB_WALK_END are kernel walk macros. Their direct
// operations are retained here as the equivalent source-level translation.
unsafe fn ecb_do_encrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    ecb_walk_start(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ecb_block(ARIA_AESNI_AVX2_PARALLEL_BLOCKS, (*core::ptr::addr_of!(aria_ops)).aria_encrypt_32way, rkey);
    ecb_block(ARIA_AESNI_PARALLEL_BLOCKS, (*core::ptr::addr_of!(aria_ops)).aria_encrypt_16way, rkey);
    ecb_block(1, aria_encrypt, rkey);
    ecb_walk_end()
}

unsafe fn ecb_do_decrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    ecb_walk_start(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ecb_block(ARIA_AESNI_AVX2_PARALLEL_BLOCKS, (*core::ptr::addr_of!(aria_ops)).aria_decrypt_32way, rkey);
    ecb_block(ARIA_AESNI_PARALLEL_BLOCKS, (*core::ptr::addr_of!(aria_ops)).aria_decrypt_16way, rkey);
    ecb_block(1, aria_decrypt, rkey);
    ecb_walk_end()
}

unsafe fn aria_avx2_ecb_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_encrypt(req, (*ctx).enc_key.as_ptr())
}

unsafe fn aria_avx2_ecb_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_decrypt(req, (*ctx).dec_key.as_ptr())
}

unsafe fn aria_avx2_set_key(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    aria_set_key(&mut (*tfm).base, key, keylen)
}

unsafe fn aria_avx2_ctr_encrypt(req: *mut skcipher_request) -> i32 {
    let req_ctx = skcipher_request_ctx(req) as *mut aria_avx2_request_ctx;
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: usize;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    let walk = walk.as_mut_ptr();
    while { nbytes = (*walk).nbytes; nbytes > 0 } {
        let mut src = (*walk).src.virt.addr;
        let mut dst = (*walk).dst.virt.addr;
        while nbytes >= ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*core::ptr::addr_of!(aria_ops)).aria_ctr_crypt_32way)(ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), (*walk).iv);
            kernel_fpu_end();
            dst = dst.add(ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE); src = src.add(ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE); nbytes -= ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE;
        }
        while nbytes >= ARIA_AESNI_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*core::ptr::addr_of!(aria_ops)).aria_ctr_crypt_16way)(ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), (*walk).iv);
            kernel_fpu_end();
            dst = dst.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE); src = src.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE); nbytes -= ARIA_AESNI_PARALLEL_BLOCK_SIZE;
        }
        while nbytes >= ARIA_BLOCK_SIZE {
            core::ptr::copy_nonoverlapping((*walk).iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE);
            crypto_inc((*walk).iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx as *const _, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), ARIA_BLOCK_SIZE);
            dst = dst.add(ARIA_BLOCK_SIZE); src = src.add(ARIA_BLOCK_SIZE); nbytes -= ARIA_BLOCK_SIZE;
        }
        if (*walk).nbytes == (*walk).total && nbytes > 0 {
            core::ptr::copy_nonoverlapping((*walk).iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE);
            crypto_inc((*walk).iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx as *const _, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), nbytes);
            nbytes = 0;
        }
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

unsafe fn aria_avx2_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<aria_avx2_request_ctx>()); 0
}

// The following kernel declarations represent names supplied by the included
// kernel headers; no dependency implementations are introduced here.
extern "C" {
    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(mask: u64, feature_name: *mut *const u8) -> bool;
    fn pr_info(message: *const u8, ...);
    fn aria_aesni_avx_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_aesni_avx_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_aesni_avx_ctr_crypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, keystream: *mut u8, iv: *mut u8);
    fn aria_aesni_avx_gfni_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_aesni_avx_gfni_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_aesni_avx_gfni_ctr_crypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, keystream: *mut u8, iv: *mut u8);
}

static mut aria_algs: [skcipher_alg; 2] = [
    skcipher_alg::ecb_aria_avx2(),
    skcipher_alg::ctr_aria_avx2(),
];

unsafe extern "C" fn aria_avx2_init() -> i32 {
    let mut feature_name: *const u8 = core::ptr::null();
    if !boot_cpu_has(X86_FEATURE_AVX) || !boot_cpu_has(X86_FEATURE_AVX2) ||
       !boot_cpu_has(X86_FEATURE_AES) || !boot_cpu_has(X86_FEATURE_OSXSAVE) {
        pr_info(b"AVX2 or AES-NI instructions are not detected.\0".as_ptr());
        return -ENODEV;
    }
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info(b"CPU feature '%s' is not supported.\n\0".as_ptr(), feature_name);
        return -ENODEV;
    }
    if boot_cpu_has(X86_FEATURE_GFNI) {
        aria_ops.aria_encrypt_16way = aria_aesni_avx_gfni_encrypt_16way;
        aria_ops.aria_decrypt_16way = aria_aesni_avx_gfni_decrypt_16way;
        aria_ops.aria_ctr_crypt_16way = aria_aesni_avx_gfni_ctr_crypt_16way;
        aria_ops.aria_encrypt_32way = aria_aesni_avx2_gfni_encrypt_32way;
        aria_ops.aria_decrypt_32way = aria_aesni_avx2_gfni_decrypt_32way;
        aria_ops.aria_ctr_crypt_32way = aria_aesni_avx2_gfni_ctr_crypt_32way;
    } else {
        aria_ops.aria_encrypt_16way = aria_aesni_avx_encrypt_16way;
        aria_ops.aria_decrypt_16way = aria_aesni_avx_decrypt_16way;
        aria_ops.aria_ctr_crypt_16way = aria_aesni_avx_ctr_crypt_16way;
        aria_ops.aria_encrypt_32way = aria_aesni_avx2_encrypt_32way;
        aria_ops.aria_decrypt_32way = aria_aesni_avx2_decrypt_32way;
        aria_ops.aria_ctr_crypt_32way = aria_aesni_avx2_ctr_crypt_32way;
    }
    crypto_register_skciphers(aria_algs.as_mut_ptr(), aria_algs.len())
}

unsafe extern "C" fn aria_avx2_exit() {
    crypto_unregister_skciphers(aria_algs.as_mut_ptr(), aria_algs.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
