/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Glue Code for the AVX/AES-NI/GFNI assembler implementation of the ARIA Cipher
 *
 * Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
 */

// Kernel headers and local headers provide the types, constants, macros, and
// helper functions referenced below.

extern "C" {
    pub fn aria_aesni_avx_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_ctr_crypt_16way(
        ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8,
        keystream: *mut u8, iv: *mut u8,
    );
    pub fn aria_aesni_avx_gfni_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_gfni_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_gfni_ctr_crypt_16way(
        ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8,
        keystream: *mut u8, iv: *mut u8,
    );
}

static mut aria_ops: aria_avx_ops = aria_avx_ops {
    aria_encrypt_16way: None,
    aria_decrypt_16way: None,
    aria_ctr_crypt_16way: None,
};

#[repr(C)]
pub struct aria_avx_request_ctx {
    pub keystream: [u8; ARIA_AESNI_PARALLEL_BLOCK_SIZE],
}

unsafe fn ecb_do_encrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    ECB_WALK_START!(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(ARIA_AESNI_PARALLEL_BLOCKS, (*aria_ops).aria_encrypt_16way);
    ECB_BLOCK!(1, aria_encrypt);
    ECB_WALK_END!();
}

unsafe fn ecb_do_decrypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    ECB_WALK_START!(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(ARIA_AESNI_PARALLEL_BLOCKS, (*aria_ops).aria_decrypt_16way);
    ECB_BLOCK!(1, aria_decrypt);
    ECB_WALK_END!();
}

unsafe fn aria_avx_ecb_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_encrypt(req, (*ctx).enc_key.as_ptr())
}

unsafe fn aria_avx_ecb_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_decrypt(req, (*ctx).dec_key.as_ptr())
}

unsafe fn aria_avx_set_key(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    aria_set_key(&mut (*tfm).base, key, keylen)
}

unsafe fn aria_avx_ctr_encrypt(req: *mut skcipher_request) -> i32 {
    let req_ctx = skcipher_request_ctx(req) as *mut aria_avx_request_ctx;
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: u32;
    let mut err: i32;

    err = skcipher_walk_virt(&mut walk, req, false);

    while { nbytes = walk.nbytes; nbytes > 0 } {
        let mut src = walk.src.virt.addr;
        let mut dst = walk.dst.virt.addr;

        while nbytes >= ARIA_AESNI_PARALLEL_BLOCK_SIZE {
            kernel_fpu_begin();
            ((*aria_ops).aria_ctr_crypt_16way.unwrap())(
                ctx as *const _, dst, src, (*req_ctx).keystream.as_mut_ptr(), walk.iv,
            );
            kernel_fpu_end();
            dst = dst.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE as usize);
            src = src.add(ARIA_AESNI_PARALLEL_BLOCK_SIZE as usize);
            nbytes -= ARIA_AESNI_PARALLEL_BLOCK_SIZE;
        }

        while nbytes >= ARIA_BLOCK_SIZE {
            core::ptr::copy_nonoverlapping(walk.iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE as usize);
            crypto_inc(walk.iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_mut_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), ARIA_BLOCK_SIZE);
            dst = dst.add(ARIA_BLOCK_SIZE as usize);
            src = src.add(ARIA_BLOCK_SIZE as usize);
            nbytes -= ARIA_BLOCK_SIZE;
        }

        if walk.nbytes == walk.total && nbytes > 0 {
            core::ptr::copy_nonoverlapping(walk.iv, (*req_ctx).keystream.as_mut_ptr(), ARIA_BLOCK_SIZE as usize);
            crypto_inc(walk.iv, ARIA_BLOCK_SIZE);
            aria_encrypt(ctx, (*req_ctx).keystream.as_mut_ptr(), (*req_ctx).keystream.as_mut_ptr());
            crypto_xor_cpy(dst, src, (*req_ctx).keystream.as_ptr(), nbytes);
            dst = dst.add(nbytes as usize);
            src = src.add(nbytes as usize);
            nbytes = 0;
        }
        err = skcipher_walk_done(&mut walk, nbytes);
    }
    err
}

unsafe fn aria_avx_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<aria_avx_request_ctx>());
    0
}

// The skcipher_alg table is a direct translation of the C initializer; its
// kernel-specific field definitions and constants are supplied by dependencies.
static mut aria_algs: [skcipher_alg; 2] = [
    skcipher_alg::ecb("ecb(aria)", "ecb-aria-avx", aria_avx_ecb_encrypt, aria_avx_ecb_decrypt),
    skcipher_alg::ctr("ctr(aria)", "ctr-aria-avx", aria_avx_ctr_encrypt, aria_avx_ctr_encrypt, aria_avx_init_tfm),
];

unsafe fn aria_avx_init() -> i32 {
    let mut feature_name: *const i8 = core::ptr::null();
    if !boot_cpu_has(X86_FEATURE_AVX) || !boot_cpu_has(X86_FEATURE_AES) || !boot_cpu_has(X86_FEATURE_OSXSAVE) {
        pr_info!("AVX or AES-NI instructions are not detected.\n");
        return -ENODEV;
    }
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }
    if boot_cpu_has(X86_FEATURE_GFNI) {
        aria_ops.aria_encrypt_16way = Some(aria_aesni_avx_gfni_encrypt_16way);
        aria_ops.aria_decrypt_16way = Some(aria_aesni_avx_gfni_decrypt_16way);
        aria_ops.aria_ctr_crypt_16way = Some(aria_aesni_avx_gfni_ctr_crypt_16way);
    } else {
        aria_ops.aria_encrypt_16way = Some(aria_aesni_avx_encrypt_16way);
        aria_ops.aria_decrypt_16way = Some(aria_aesni_avx_decrypt_16way);
        aria_ops.aria_ctr_crypt_16way = Some(aria_aesni_avx_ctr_crypt_16way);
    }
    crypto_register_skciphers(aria_algs.as_mut_ptr(), ARRAY_SIZE!(aria_algs))
}

unsafe fn aria_avx_exit() {
    crypto_unregister_skciphers(aria_algs.as_mut_ptr(), ARRAY_SIZE!(aria_algs));
}

module_init!(aria_avx_init);
module_exit!(aria_avx_exit);
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Taehee Yoo <ap420073@gmail.com>");
MODULE_DESCRIPTION!("ARIA Cipher Algorithm, AVX/AES-NI/GFNI optimized");
MODULE_ALIAS_CRYPTO!("aria");
MODULE_ALIAS_CRYPTO!("aria-aesni-avx");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
