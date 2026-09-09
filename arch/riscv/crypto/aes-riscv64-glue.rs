// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES modes using the RISC-V vector crypto extensions
 *
 * Copyright (C) 2023 VRULL GmbH
 * Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 *
 * Copyright 2024 Google LLC
 */

// Kernel headers and symbols are supplied by the surrounding Rust kernel environment.

unsafe extern "C" {
    fn aes_ecb_encrypt_zvkned(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize);
    fn aes_ecb_decrypt_zvkned(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize);
    fn aes_cbc_encrypt_zvkned(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, iv: *mut u8);
    fn aes_cbc_decrypt_zvkned(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, iv: *mut u8);
    fn aes_cbc_cts_crypt_zvkned(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, iv: *const u8, enc: bool);
    fn aes_ctr32_crypt_zvkned_zvkb(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, iv: *mut u8);
    fn aes_xts_encrypt_zvkned_zvbb_zvkg(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, tweak: *mut u8);
    fn aes_xts_decrypt_zvkned_zvbb_zvkg(key: *const crypto_aes_ctx, input: *const u8, output: *mut u8, len: usize, tweak: *mut u8);
}

unsafe fn riscv64_aes_setkey(ctx: *mut crypto_aes_ctx, key: *const u8, keylen: u32) -> i32 {
    aes_expandkey(ctx, key, keylen)
}

unsafe fn riscv64_aes_setkey_skcipher(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    riscv64_aes_setkey(crypto_skcipher_ctx(tfm), key, keylen)
}

/* AES-ECB */

unsafe fn riscv64_aes_ecb_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const crypto_aes_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    while { nbytes = walk.nbytes; nbytes != 0 } {
        kernel_vector_begin();
        if enc {
            aes_ecb_encrypt_zvkned(ctx, walk.src.virt.addr, walk.dst.virt.addr, (nbytes & !(AES_BLOCK_SIZE - 1)) as usize);
        } else {
            aes_ecb_decrypt_zvkned(ctx, walk.src.virt.addr, walk.dst.virt.addr, (nbytes & !(AES_BLOCK_SIZE - 1)) as usize);
        }
        kernel_vector_end();
        err = skcipher_walk_done(&mut walk, nbytes & (AES_BLOCK_SIZE - 1));
    }
    err
}

unsafe fn riscv64_aes_ecb_encrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_ecb_crypt(req, true) }
unsafe fn riscv64_aes_ecb_decrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_ecb_crypt(req, false) }

/* AES-CBC */

unsafe fn riscv64_aes_cbc_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const crypto_aes_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    while { nbytes = walk.nbytes; nbytes != 0 } {
        kernel_vector_begin();
        if enc { aes_cbc_encrypt_zvkned(ctx, walk.src.virt.addr, walk.dst.virt.addr, (nbytes & !(AES_BLOCK_SIZE - 1)) as usize, walk.iv); }
        else { aes_cbc_decrypt_zvkned(ctx, walk.src.virt.addr, walk.dst.virt.addr, (nbytes & !(AES_BLOCK_SIZE - 1)) as usize, walk.iv); }
        kernel_vector_end();
        err = skcipher_walk_done(&mut walk, nbytes & (AES_BLOCK_SIZE - 1));
    }
    err
}
unsafe fn riscv64_aes_cbc_encrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_cbc_crypt(req, true) }
unsafe fn riscv64_aes_cbc_decrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_cbc_crypt(req, false) }

/* AES-CBC-CTS */

unsafe fn riscv64_aes_cbc_cts_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const crypto_aes_ctx;
    let mut sg_src: [scatterlist; 2] = core::mem::zeroed();
    let mut sg_dst: [scatterlist; 2] = core::mem::zeroed();
    let mut subreq: skcipher_request = core::mem::zeroed();
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut cbc_len: u32;
    if (*req).cryptlen < AES_BLOCK_SIZE { return -EINVAL; }
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    if err != 0 { return err; }
    if walk.nbytes != (*req).cryptlen {
        cbc_len = round_down((*req).cryptlen - AES_BLOCK_SIZE - 1, AES_BLOCK_SIZE);
        skcipher_walk_abort(&mut walk);
        skcipher_request_set_tfm(&mut subreq, tfm);
        skcipher_request_set_callback(&mut subreq, skcipher_request_flags(req), None, core::ptr::null_mut());
        skcipher_request_set_crypt(&mut subreq, (*req).src, (*req).dst, cbc_len, (*req).iv);
        err = riscv64_aes_cbc_crypt(&mut subreq, enc);
        if err != 0 { return err; }
        let mut src = scatterwalk_ffwd(sg_src.as_mut_ptr(), (*req).src, cbc_len);
        let mut dst = src;
        if (*req).dst != (*req).src { dst = scatterwalk_ffwd(sg_dst.as_mut_ptr(), (*req).dst, cbc_len); }
        skcipher_request_set_crypt(&mut subreq, src, dst, (*req).cryptlen - cbc_len, (*req).iv);
        err = skcipher_walk_virt(&mut walk, &mut subreq, false);
        if err != 0 { return err; }
    }
    kernel_vector_begin();
    aes_cbc_cts_crypt_zvkned(ctx, walk.src.virt.addr, walk.dst.virt.addr, walk.nbytes as usize, (*req).iv, enc);
    kernel_vector_end();
    skcipher_walk_done(&mut walk, 0)
}
unsafe fn riscv64_aes_cbc_cts_encrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_cbc_cts_crypt(req, true) }
unsafe fn riscv64_aes_cbc_cts_decrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_cbc_cts_crypt(req, false) }

/* AES-CTR */

unsafe fn riscv64_aes_ctr_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const crypto_aes_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut nbytes: u32;
    let mut p1_nbytes: u32;
    let mut ctr32 = get_unaligned_be32((*req).iv.add(12));
    let mut nblocks: u32;
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    while { nbytes = walk.nbytes; nbytes != 0 } {
        if nbytes < walk.total { nbytes = round_down(nbytes, AES_BLOCK_SIZE); nblocks = nbytes / AES_BLOCK_SIZE; }
        else { nblocks = DIV_ROUND_UP(nbytes, AES_BLOCK_SIZE); }
        ctr32 = ctr32.wrapping_add(nblocks);
        kernel_vector_begin();
        if ctr32 >= nblocks { aes_ctr32_crypt_zvkned_zvkb(ctx, walk.src.virt.addr, walk.dst.virt.addr, nbytes as usize, (*req).iv); }
        else {
            p1_nbytes = core::cmp::min(nbytes, (nblocks - ctr32) * AES_BLOCK_SIZE);
            aes_ctr32_crypt_zvkned_zvkb(ctx, walk.src.virt.addr, walk.dst.virt.addr, p1_nbytes as usize, (*req).iv);
            crypto_inc((*req).iv, 12);
            if ctr32 != 0 { aes_ctr32_crypt_zvkned_zvkb(ctx, walk.src.virt.addr.add(p1_nbytes as usize), walk.dst.virt.addr.add(p1_nbytes as usize), (nbytes - p1_nbytes) as usize, (*req).iv); }
        }
        kernel_vector_end();
        err = skcipher_walk_done(&mut walk, walk.nbytes - nbytes);
    }
    err
}

/* AES-XTS */

#[repr(C)]
struct riscv64_aes_xts_ctx { ctx1: crypto_aes_ctx, tweak_key: aes_enckey }

unsafe fn riscv64_aes_xts_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut riscv64_aes_xts_ctx;
    let err = xts_verify_key(tfm, key, keylen); if err != 0 { return err; }
    let err = riscv64_aes_setkey(&mut (*ctx).ctx1, key, keylen / 2); if err != 0 { return err; }
    aes_prepareenckey(&mut (*ctx).tweak_key, key.add((keylen / 2) as usize), keylen / 2)
}

unsafe fn riscv64_aes_xts_crypt(req: *mut skcipher_request, enc: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const riscv64_aes_xts_ctx;
    let mut tail = (*req).cryptlen % AES_BLOCK_SIZE;
    let mut sg_src: [scatterlist; 2] = core::mem::zeroed(); let mut sg_dst: [scatterlist; 2] = core::mem::zeroed();
    let mut subreq: skcipher_request = core::mem::zeroed(); let mut walk: skcipher_walk = core::mem::zeroed();
    if (*req).cryptlen < AES_BLOCK_SIZE { return -EINVAL; }
    aes_encrypt(&(*ctx).tweak_key, (*req).iv, (*req).iv);
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    if tail > 0 && walk.nbytes < walk.total {
        skcipher_walk_abort(&mut walk); skcipher_request_set_tfm(&mut subreq, tfm);
        skcipher_request_set_callback(&mut subreq, skcipher_request_flags(req), None, core::ptr::null_mut());
        skcipher_request_set_crypt(&mut subreq, (*req).src, (*req).dst, (*req).cryptlen - tail - AES_BLOCK_SIZE, (*req).iv);
        req = &mut subreq; err = skcipher_walk_virt(&mut walk, req, false);
    } else { tail = 0; }
    while walk.nbytes != 0 {
        let mut nbytes = walk.nbytes; if nbytes < walk.total { nbytes = round_down(nbytes, AES_BLOCK_SIZE); }
        kernel_vector_begin(); if enc { aes_xts_encrypt_zvkned_zvbb_zvkg(&(*ctx).ctx1, walk.src.virt.addr, walk.dst.virt.addr, nbytes as usize, (*req).iv); }
        else { aes_xts_decrypt_zvkned_zvbb_zvkg(&(*ctx).ctx1, walk.src.virt.addr, walk.dst.virt.addr, nbytes as usize, (*req).iv); }
        kernel_vector_end(); err = skcipher_walk_done(&mut walk, walk.nbytes - nbytes);
    }
    if err != 0 || tail == 0 { return err; }
    let mut src = scatterwalk_ffwd(sg_src.as_mut_ptr(), (*req).src, (*req).cryptlen); let mut dst = src;
    if (*req).dst != (*req).src { dst = scatterwalk_ffwd(sg_dst.as_mut_ptr(), (*req).dst, (*req).cryptlen); }
    skcipher_request_set_crypt(req, src, dst, AES_BLOCK_SIZE + tail, (*req).iv);
    err = skcipher_walk_virt(&mut walk, req, false); if err != 0 { return err; }
    kernel_vector_begin(); if enc { aes_xts_encrypt_zvkned_zvbb_zvkg(&(*ctx).ctx1, walk.src.virt.addr, walk.dst.virt.addr, walk.nbytes as usize, (*req).iv); }
    else { aes_xts_decrypt_zvkned_zvbb_zvkg(&(*ctx).ctx1, walk.src.virt.addr, walk.dst.virt.addr, walk.nbytes as usize, (*req).iv); }
    kernel_vector_end(); skcipher_walk_done(&mut walk, 0)
}
unsafe fn riscv64_aes_xts_encrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_xts_crypt(req, true) }
unsafe fn riscv64_aes_xts_decrypt(req: *mut skcipher_request) -> i32 { riscv64_aes_xts_crypt(req, false) }

/* Algorithm definitions */

// These descriptors preserve the C registrations; their kernel-specific structure type is supplied externally.
unsafe fn riscv64_aes_xts_supported() -> bool {
    riscv_isa_extension_available(core::ptr::null_mut(), ZVBB) &&
        riscv_isa_extension_available(core::ptr::null_mut(), ZVKG) &&
        riscv_vector_vlen() < 2048
}

unsafe fn riscv64_aes_mod_init() -> i32 {
    let mut err = -ENODEV;
    if riscv_isa_extension_available(core::ptr::null_mut(), ZVKNED) && riscv_vector_vlen() >= 128 {
        err = crypto_register_skciphers(&mut riscv64_zvkned_aes_skcipher_algs, 3);
        if err != 0 { return err; }
        if riscv_isa_extension_available(core::ptr::null_mut(), ZVKB) {
            err = crypto_register_skcipher(&mut riscv64_zvkned_zvkb_aes_skcipher_alg);
            if err != 0 { crypto_unregister_skciphers(&mut riscv64_zvkned_aes_skcipher_algs, 3); return err; }
        }
        if riscv64_aes_xts_supported() {
            err = crypto_register_skcipher(&mut riscv64_zvkned_zvbb_zvkg_aes_skcipher_alg);
            if err != 0 {
                if riscv_isa_extension_available(core::ptr::null_mut(), ZVKB) { crypto_unregister_skcipher(&mut riscv64_zvkned_zvkb_aes_skcipher_alg); }
                crypto_unregister_skciphers(&mut riscv64_zvkned_aes_skcipher_algs, 3);
            }
        }
    }
    err
}

unsafe fn riscv64_aes_mod_exit() {
    if riscv64_aes_xts_supported() { crypto_unregister_skcipher(&mut riscv64_zvkned_zvbb_zvkg_aes_skcipher_alg); }
    if riscv_isa_extension_available(core::ptr::null_mut(), ZVKB) { crypto_unregister_skcipher(&mut riscv64_zvkned_zvkb_aes_skcipher_alg); }
    crypto_unregister_skciphers(&mut riscv64_zvkned_aes_skcipher_algs, 3);
}

// module_init(riscv64_aes_mod_init); module_exit(riscv64_aes_mod_exit);
// MODULE_DESCRIPTION("AES-ECB/CBC/CTS/CTR/XTS (RISC-V accelerated)");
// MODULE_AUTHOR("Jerry Shih <jerry.shih@sifive.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("aes"); MODULE_ALIAS_CRYPTO("ecb(aes)");
// MODULE_ALIAS_CRYPTO("cbc(aes)"); MODULE_ALIAS_CRYPTO("cts(cbc(aes))");
// MODULE_ALIAS_CRYPTO("ctr(aes)"); MODULE_ALIAS_CRYPTO("xts(aes)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
