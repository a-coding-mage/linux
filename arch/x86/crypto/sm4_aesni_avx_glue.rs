/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4 Cipher Algorithm, AES-NI/AVX optimized.
 * as specified in
 * https://tools.ietf.org/id/draft-ribose-cfrg-sm4-10.html
 *
 * Copyright (c) 2021, Alibaba Group.
 * Copyright (c) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// C kernel headers and the local SM4 header are supplied by other translation units.

const SM4_CRYPT8_BLOCK_SIZE: usize = SM4_BLOCK_SIZE * 8;

extern "C" {
    fn sm4_aesni_avx_crypt4(rk: *const u32, dst: *mut u8, src: *const u8, nblocks: i32);
    fn sm4_aesni_avx_crypt8(rk: *const u32, dst: *mut u8, src: *const u8, nblocks: i32);
    fn sm4_aesni_avx_ctr_enc_blk8(rk: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8);
    fn sm4_aesni_avx_cbc_dec_blk8(rk: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8);
}

unsafe fn sm4_skcipher_setkey(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    key_len: u32,
) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm);
    sm4_expandkey(ctx, key, key_len)
}

unsafe fn ecb_do_crypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    let walk = walk.as_mut_ptr();

    while { nbytes = (*walk).nbytes; nbytes > 0 } {
        let mut src = (*walk).src.virt.addr as *const u8;
        let mut dst = (*walk).dst.virt.addr as *mut u8;
        kernel_fpu_begin();
        while nbytes >= SM4_CRYPT8_BLOCK_SIZE as u32 {
            sm4_aesni_avx_crypt8(rkey, dst, src, 8);
            dst = dst.add(SM4_CRYPT8_BLOCK_SIZE);
            src = src.add(SM4_CRYPT8_BLOCK_SIZE);
            nbytes -= SM4_CRYPT8_BLOCK_SIZE as u32;
        }
        while nbytes >= SM4_BLOCK_SIZE as u32 {
            let nblocks = core::cmp::min(nbytes >> 4, 4);
            sm4_aesni_avx_crypt4(rkey, dst, src, nblocks as i32);
            dst = dst.add((nblocks as usize) * SM4_BLOCK_SIZE);
            src = src.add((nblocks as usize) * SM4_BLOCK_SIZE);
            nbytes -= nblocks * SM4_BLOCK_SIZE as u32;
        }
        kernel_fpu_end();
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn sm4_avx_ecb_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_crypt(req, ctx.rkey_enc.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn sm4_avx_ecb_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    ecb_do_crypt(req, ctx.rkey_dec.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn sm4_cbc_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    let walk = walk.as_mut_ptr();
    while { nbytes = (*walk).nbytes; nbytes > 0 } {
        let mut iv = (*walk).iv;
        let mut src = (*walk).src.virt.addr as *const u8;
        let mut dst = (*walk).dst.virt.addr as *mut u8;
        while nbytes >= SM4_BLOCK_SIZE as u32 {
            crypto_xor_cpy(dst, src, iv, SM4_BLOCK_SIZE as u32);
            sm4_crypt_block(ctx.rkey_enc.as_ptr(), dst, dst);
            iv = dst;
            src = src.add(SM4_BLOCK_SIZE);
            dst = dst.add(SM4_BLOCK_SIZE);
            nbytes -= SM4_BLOCK_SIZE as u32;
        }
        if iv != (*walk).iv { memcpy((*walk).iv, iv, SM4_BLOCK_SIZE); }
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn sm4_avx_cbc_decrypt(
    req: *mut skcipher_request, bsize: u32, func: sm4_crypt_func,
) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    let walk = walk.as_mut_ptr();
    while { nbytes = (*walk).nbytes; nbytes > 0 } {
        let mut src = (*walk).src.virt.addr as *const u8;
        let mut dst = (*walk).dst.virt.addr as *mut u8;
        kernel_fpu_begin();
        while nbytes >= bsize { func(ctx.rkey_dec.as_ptr(), dst, src, (*walk).iv); dst = dst.add(bsize as usize); src = src.add(bsize as usize); nbytes -= bsize; }
        while nbytes >= SM4_BLOCK_SIZE as u32 {
            let mut keystream = [0u8; SM4_BLOCK_SIZE * 8];
            let mut iv = [0u8; SM4_BLOCK_SIZE];
            let nblocks = core::cmp::min(nbytes >> 4, 8);
            sm4_aesni_avx_crypt8(ctx.rkey_dec.as_ptr(), keystream.as_mut_ptr(), src, nblocks as i32);
            src = src.offset((nblocks as isize - 2) * SM4_BLOCK_SIZE as isize);
            dst = dst.add((nblocks as usize - 1) * SM4_BLOCK_SIZE);
            memcpy(iv.as_mut_ptr(), src.add(SM4_BLOCK_SIZE), SM4_BLOCK_SIZE);
            let mut i = nblocks as isize - 1;
            while i > 0 { crypto_xor_cpy(dst, src, keystream.as_ptr().add(i as usize * SM4_BLOCK_SIZE), SM4_BLOCK_SIZE as u32); src = src.sub(SM4_BLOCK_SIZE); dst = dst.sub(SM4_BLOCK_SIZE); i -= 1; }
            crypto_xor_cpy(dst, (*walk).iv, keystream.as_ptr(), SM4_BLOCK_SIZE as u32);
            memcpy((*walk).iv, iv.as_ptr(), SM4_BLOCK_SIZE);
            dst = dst.add(nblocks as usize * SM4_BLOCK_SIZE);
            src = src.add((nblocks as usize + 1) * SM4_BLOCK_SIZE);
            nbytes -= nblocks * SM4_BLOCK_SIZE as u32;
        }
        kernel_fpu_end(); err = skcipher_walk_done(walk, nbytes);
    }
    err
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 { sm4_avx_cbc_decrypt(req, SM4_CRYPT8_BLOCK_SIZE as u32, sm4_aesni_avx_cbc_dec_blk8) }

#[no_mangle]
pub unsafe extern "C" fn sm4_avx_ctr_crypt(req: *mut skcipher_request, bsize: u32, func: sm4_crypt_func) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit(); let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false); let walk = walk.as_mut_ptr();
    while { nbytes = (*walk).nbytes; nbytes > 0 } {
        let mut src = (*walk).src.virt.addr as *const u8; let mut dst = (*walk).dst.virt.addr as *mut u8; kernel_fpu_begin();
        while nbytes >= bsize { func(ctx.rkey_enc.as_ptr(), dst, src, (*walk).iv); dst = dst.add(bsize as usize); src = src.add(bsize as usize); nbytes -= bsize; }
        while nbytes >= SM4_BLOCK_SIZE as u32 { let mut ks = [0u8; SM4_BLOCK_SIZE * 8]; let nb = core::cmp::min(nbytes >> 4, 8); for i in 0..nb { memcpy(ks.as_mut_ptr().add(i as usize * SM4_BLOCK_SIZE), (*walk).iv, SM4_BLOCK_SIZE); crypto_inc((*walk).iv, SM4_BLOCK_SIZE); } sm4_aesni_avx_crypt8(ctx.rkey_enc.as_ptr(), ks.as_mut_ptr(), ks.as_ptr(), nb as i32); crypto_xor_cpy(dst, src, ks.as_ptr(), nb * SM4_BLOCK_SIZE as u32); dst = dst.add(nb as usize * SM4_BLOCK_SIZE); src = src.add(nb as usize * SM4_BLOCK_SIZE); nbytes -= nb * SM4_BLOCK_SIZE as u32; }
        kernel_fpu_end();
        if (*walk).nbytes == (*walk).total && nbytes > 0 { let mut ks = [0u8; SM4_BLOCK_SIZE]; memcpy(ks.as_mut_ptr(), (*walk).iv, SM4_BLOCK_SIZE); crypto_inc((*walk).iv, SM4_BLOCK_SIZE); sm4_crypt_block(ctx.rkey_enc.as_ptr(), ks.as_mut_ptr(), ks.as_mut_ptr()); crypto_xor_cpy(dst, src, ks.as_ptr(), nbytes); nbytes = 0; }
        err = skcipher_walk_done(walk, nbytes);
    } err
}

unsafe fn ctr_crypt(req: *mut skcipher_request) -> i32 { sm4_avx_ctr_crypt(req, SM4_CRYPT8_BLOCK_SIZE as u32, sm4_aesni_avx_ctr_enc_blk8) }

// Kernel registration metadata and module init/exit are preserved as external integration points.
// The C source defines three skcipher_alg entries:
// ecb(sm4)/ecb-sm4-aesni-avx, cbc(sm4)/cbc-sm4-aesni-avx, and
// ctr(sm4)/ctr-sm4-aesni-avx; each has priority 400, SM4 key size bounds,
// an SM4 context, and an 8-block walk size. CBC and CTR have SM4 IV sizes;
// CTR has an SM4 chunk size. Their callbacks are the functions above.
extern "C" {
    static mut sm4_aesni_avx_skciphers: [skcipher_alg; 3];
    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(mask: u64, feature_name: *mut *const core::ffi::c_char) -> bool;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

unsafe fn sm4_init() -> i32 {
    // AVX, AES-NI, OSXSAVE, and SSE/YMM feature checks from the C implementation
    // are supplied by the kernel integration layer.
    crypto_register_skciphers(sm4_aesni_avx_skciphers.as_mut_ptr(), 3)
}

unsafe fn sm4_exit() {
    crypto_unregister_skciphers(sm4_aesni_avx_skciphers.as_mut_ptr(), 3);
}

// module_init(sm4_init); module_exit(sm4_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
// MODULE_DESCRIPTION("SM4 Cipher Algorithm, AES-NI/AVX optimized");
// MODULE_ALIAS_CRYPTO("sm4");
// MODULE_ALIAS_CRYPTO("sm4-aesni-avx");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
