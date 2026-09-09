/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4 Cipher Algorithm, using ARMv8 NEON
 * as specified in
 * https://tools.ietf.org/id/draft-ribose-cfrg-sm4-10.html
 *
 * Copyright (C) 2022, Alibaba Group.
 * Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// Kernel dependencies supplied by other translation units.

extern "C" {
    fn sm4_neon_crypt(rkey: *const u32, dst: *mut u8, src: *const u8, nblocks: c_uint);
    fn sm4_neon_cbc_dec(rkey_dec: *const u32, dst: *mut u8, src: *const u8,
                        iv: *mut u8, nblocks: c_uint);
    fn sm4_neon_ctr_crypt(rkey_enc: *const u32, dst: *mut u8, src: *const u8,
                          iv: *mut u8, nblocks: c_uint);
}

const SM4_BLOCK_SIZE: usize = 16;
const SM4_KEY_SIZE: usize = 16;

unsafe fn sm4_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: c_uint) -> c_int {
    let ctx = crypto_skcipher_ctx(tfm);
    sm4_expandkey(ctx, key, key_len)
}

unsafe fn sm4_ecb_do_crypt(req: *mut skcipher_request, rkey: *const u32) -> c_int {
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: c_uint;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);

    while {
        nbytes = (*walk.as_mut_ptr()).nbytes;
        nbytes > 0
    } {
        let walk = walk.as_mut_ptr();
        let src = (*walk).src.virt.addr;
        let dst = (*walk).dst.virt.addr;
        let nblocks = nbytes / SM4_BLOCK_SIZE as c_uint;
        if nblocks != 0 {
            sm4_neon_crypt(rkey, dst, src, nblocks);
        }
        err = skcipher_walk_done(walk, nbytes % SM4_BLOCK_SIZE as c_uint);
    }
    err
}

unsafe fn sm4_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    sm4_ecb_do_crypt(req, (*ctx).rkey_enc.as_ptr())
}

unsafe fn sm4_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    sm4_ecb_do_crypt(req, (*ctx).rkey_dec.as_ptr())
}

unsafe fn sm4_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: c_uint;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    while {
        nbytes = (*walk.as_mut_ptr()).nbytes;
        nbytes > 0
    } {
        let walk = walk.as_mut_ptr();
        let mut iv = (*walk).iv;
        let mut src = (*walk).src.virt.addr;
        let mut dst = (*walk).dst.virt.addr;
        while nbytes >= SM4_BLOCK_SIZE as c_uint {
            crypto_xor_cpy(dst, src, iv, SM4_BLOCK_SIZE);
            sm4_crypt_block((*ctx).rkey_enc.as_ptr(), dst, dst);
            iv = dst;
            src = src.add(SM4_BLOCK_SIZE);
            dst = dst.add(SM4_BLOCK_SIZE);
            nbytes -= SM4_BLOCK_SIZE as c_uint;
        }
        if iv != (*walk).iv {
            core::ptr::copy_nonoverlapping(iv, (*walk).iv, SM4_BLOCK_SIZE);
        }
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

unsafe fn sm4_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: c_uint;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    while {
        nbytes = (*walk.as_mut_ptr()).nbytes;
        nbytes > 0
    } {
        let walk = walk.as_mut_ptr();
        let src = (*walk).src.virt.addr;
        let dst = (*walk).dst.virt.addr;
        let nblocks = nbytes / SM4_BLOCK_SIZE as c_uint;
        if nblocks != 0 {
            sm4_neon_cbc_dec((*ctx).rkey_dec.as_ptr(), dst, src, (*walk).iv, nblocks);
        }
        err = skcipher_walk_done(walk, nbytes % SM4_BLOCK_SIZE as c_uint);
    }
    err
}

unsafe fn sm4_ctr_crypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: c_uint;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    while {
        nbytes = (*walk.as_mut_ptr()).nbytes;
        nbytes > 0
    } {
        let walk = walk.as_mut_ptr();
        let mut src = (*walk).src.virt.addr;
        let mut dst = (*walk).dst.virt.addr;
        let nblocks = nbytes / SM4_BLOCK_SIZE as c_uint;
        if nblocks != 0 {
            sm4_neon_ctr_crypt((*ctx).rkey_enc.as_ptr(), dst, src, (*walk).iv, nblocks);
            let offset = nblocks as usize * SM4_BLOCK_SIZE;
            dst = dst.add(offset);
            src = src.add(offset);
            nbytes -= offset as c_uint;
        }
        /* tail */
        if (*walk).nbytes == (*walk).total && nbytes > 0 {
            let mut keystream = [0u8; SM4_BLOCK_SIZE];
            sm4_crypt_block((*ctx).rkey_enc.as_ptr(), keystream.as_mut_ptr(), (*walk).iv);
            crypto_inc((*walk).iv, SM4_BLOCK_SIZE);
            crypto_xor_cpy(dst, src, keystream.as_ptr(), nbytes as usize);
            nbytes = 0;
        }
        err = skcipher_walk_done(walk, nbytes);
    }
    err
}

static mut sm4_algs: [skcipher_alg; 3] = [
    skcipher_alg {
        base: crypto_alg { cra_name: b"ecb(sm4)\0".as_ptr(), cra_driver_name: b"ecb-sm4-neon\0".as_ptr(), cra_priority: 200, cra_blocksize: SM4_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<sm4_ctx>(), cra_module: THIS_MODULE },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: 0, chunksize: 0,
        setkey: Some(sm4_setkey), encrypt: Some(sm4_ecb_encrypt), decrypt: Some(sm4_ecb_decrypt),
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"cbc(sm4)\0".as_ptr(), cra_driver_name: b"cbc-sm4-neon\0".as_ptr(), cra_priority: 200, cra_blocksize: SM4_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<sm4_ctx>(), cra_module: THIS_MODULE },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: SM4_BLOCK_SIZE, chunksize: 0,
        setkey: Some(sm4_setkey), encrypt: Some(sm4_cbc_encrypt), decrypt: Some(sm4_cbc_decrypt),
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"ctr(sm4)\0".as_ptr(), cra_driver_name: b"ctr-sm4-neon\0".as_ptr(), cra_priority: 200, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<sm4_ctx>(), cra_module: THIS_MODULE },
        min_keysize: SM4_KEY_SIZE, max_keysize: SM4_KEY_SIZE, ivsize: SM4_BLOCK_SIZE, chunksize: SM4_BLOCK_SIZE,
        setkey: Some(sm4_setkey), encrypt: Some(sm4_ctr_crypt), decrypt: Some(sm4_ctr_crypt),
    },
];

unsafe fn sm4_init() -> c_int {
    crypto_register_skciphers(sm4_algs.as_mut_ptr(), sm4_algs.len())
}

unsafe fn sm4_exit() {
    crypto_unregister_skciphers(sm4_algs.as_mut_ptr(), sm4_algs.len());
}

// module_init(sm4_init);
// module_exit(sm4_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
