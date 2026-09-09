/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Glue Code for the AVX512/GFNI assembler implementation of the ARIA Cipher
 *
 * Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
 */

// Kernel headers and the included helper/assembler declarations are supplied
// by the surrounding translation unit.

extern "C" {
    fn aria_gfni_avx512_encrypt_64way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_gfni_avx512_decrypt_64way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn aria_gfni_avx512_ctr_crypt_64way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8, keystream: *mut u8, iv: *mut u8);
}

static mut aria_ops: aria_avx_ops = aria_avx_ops {
    aria_encrypt_16way: None, aria_decrypt_16way: None, aria_ctr_crypt_16way: None,
    aria_encrypt_32way: None, aria_decrypt_32way: None, aria_ctr_crypt_32way: None,
    aria_encrypt_64way: None, aria_decrypt_64way: None, aria_ctr_crypt_64way: None,
};

#[repr(C)]
struct aria_avx512_request_ctx {
    keystream: [u8; ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE],
}

unsafe fn ecb_do_encrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    // ECB_WALK_START/ECB_BLOCK/ECB_WALK_END are kernel helper macros. Their
    // original ordering and block widths are preserved here.
    ECB_WALK_START!(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(ARIA_GFNI_AVX512_PARALLEL_BLOCKS, (*aria_ops).aria_encrypt_64way);
    ECB_BLOCK!(ARIA_AESNI_AVX2_PARALLEL_BLOCKS, (*aria_ops).aria_encrypt_32way);
    ECB_BLOCK!(ARIA_AESNI_PARALLEL_BLOCKS, (*aria_ops).aria_encrypt_16way);
    ECB_BLOCK!(1, aria_encrypt);
    ECB_WALK_END!()
}

unsafe fn ecb_do_decrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    ECB_WALK_START!(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(ARIA_GFNI_AVX512_PARALLEL_BLOCKS, (*aria_ops).aria_decrypt_64way);
    ECB_BLOCK!(ARIA_AESNI_AVX2_PARALLEL_BLOCKS, (*aria_ops).aria_decrypt_32way);
    ECB_BLOCK!(ARIA_AESNI_PARALLEL_BLOCKS, (*aria_ops).aria_decrypt_16way);
    ECB_BLOCK!(1, aria_decrypt);
    ECB_WALK_END!()
}

unsafe fn aria_avx512_ecb_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_encrypt(req, (*ctx).enc_key[0].as_ptr())
}

unsafe fn aria_avx512_ecb_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_decrypt(req, (*ctx).dec_key[0].as_ptr())
}

unsafe fn aria_avx512_set_key(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    aria_set_key(&mut (*tfm).base, key, keylen)
}

unsafe fn aria_avx512_ctr_encrypt(req: *mut skcipher_request) -> i32 {
    let req_ctx = skcipher_request_ctx(req) as *mut aria_avx512_request_ctx;
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: usize;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    let walk = walk.as_mut_ptr();

    while {
        nbytes = (*walk).nbytes;
        nbytes > 0
    } {
        let mut src = (*walk).src.virt.addr;
        let mut dst = (*walk).dst.virt.addr;
        while nbytes >= ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*aria_ops).aria_ctr_crypt_64way.unwrap())(ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), (*walk).iv);
            kernel_fpu_end();
            dst = dst.add(ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE); src = src.add(ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE); nbytes -= ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE;
        }
        while nbytes >= ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*aria_ops).aria_ctr_crypt_32way.unwrap())(ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), (*walk).iv);
            kernel_fpu_end();
            dst = dst.add(ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE); src = src.add(ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE); nbytes -= ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE;
        }
        while nbytes >= ARIA_AESNI_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*aria_ops).aria_ctr_crypt_16way.unwrap())(ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), (*walk).iv);
            kernel_fpu_end();
            dst = dst.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE); src = src.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE); nbytes -= ARIA_AESNI_PARALLEL_BLOCK_SIZE;
        }
        while nbytes >= ARIA_BLOCK_SIZE {
            core::ptr::copy_nonoverlapping((*walk).iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE);
            crypto_inc((*walk).iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_mut_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), ARIA_BLOCK_SIZE);
            dst = dst.add(ARIA_BLOCK_SIZE); src = src.add(ARIA_BLOCK_SIZE); nbytes -= ARIA_BLOCK_SIZE;
        }
        if (*walk).nbytes == (*walk).total && nbytes > 0 {
            core::ptr::copy_nonoverlapping((*walk).iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE);
            crypto_inc((*walk).iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_mut_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), nbytes);
            nbytes = 0;
        }
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

unsafe fn aria_avx512_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<aria_avx512_request_ctx>());
    0
}

// The skcipher_alg table and module registration retain the C kernel ABI
// fields and are declared by the surrounding Rust kernel bindings.
static mut aria_algs: [skcipher_alg; 2] = [
    skcipher_alg::ecb("ecb(aria)", "ecb-aria-avx512", aria_avx512_ecb_encrypt, aria_avx512_ecb_decrypt),
    skcipher_alg::ctr("ctr(aria)", "ctr-aria-avx512", aria_avx512_ctr_encrypt, aria_avx512_ctr_encrypt, aria_avx512_init_tfm),
];

unsafe fn aria_avx512_init() -> i32 {
    let mut feature_name: *const core::ffi::c_char = core::ptr::null();
    if !boot_cpu_has(X86_FEATURE_AVX) || !boot_cpu_has(X86_FEATURE_AVX2) || !boot_cpu_has(X86_FEATURE_AVX512F) || !boot_cpu_has(X86_FEATURE_AVX512VL) || !boot_cpu_has(X86_FEATURE_GFNI) || !boot_cpu_has(X86_FEATURE_OSXSAVE) {
        pr_info!("AVX512/GFNI instructions are not detected.\n");
        return -ENODEV;
    }
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM | XFEATURE_MASK_AVX512, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }
    aria_ops.aria_encrypt_16way = Some(aria_aesni_avx_gfni_encrypt_16way);
    aria_ops.aria_decrypt_16way = Some(aria_aesni_avx_gfni_decrypt_16way);
    aria_ops.aria_ctr_crypt_16way = Some(aria_aesni_avx_gfni_ctr_crypt_16way);
    aria_ops.aria_encrypt_32way = Some(aria_aesni_avx2_gfni_encrypt_32way);
    aria_ops.aria_decrypt_32way = Some(aria_aesni_avx2_gfni_decrypt_32way);
    aria_ops.aria_ctr_crypt_32way = Some(aria_aesni_avx2_gfni_ctr_crypt_32way);
    aria_ops.aria_encrypt_64way = Some(aria_gfni_avx512_encrypt_64way);
    aria_ops.aria_decrypt_64way = Some(aria_gfni_avx512_decrypt_64way);
    aria_ops.aria_ctr_crypt_64way = Some(aria_gfni_avx512_ctr_crypt_64way);
    crypto_register_skciphers(aria_algs.as_mut_ptr(), aria_algs.len())
}

unsafe fn aria_avx512_exit() {
    crypto_unregister_skciphers(aria_algs.as_mut_ptr(), aria_algs.len());
}

module_init!(aria_avx512_init);
module_exit!(aria_avx512_exit);
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Taehee Yoo <ap420073@gmail.com>");
MODULE_DESCRIPTION!("ARIA Cipher Algorithm, AVX512/GFNI optimized");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
