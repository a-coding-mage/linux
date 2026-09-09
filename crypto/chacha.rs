// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API wrappers for the ChaCha20, XChaCha20, and XChaCha12 stream ciphers
 *
 * Copyright (C) 2015 Martin Willi
 * Copyright (C) 2018 Google LLC
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct chacha_ctx {
    pub key: [u32; 8],
    pub nrounds: i32,
}

extern "C" {
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *mut chacha_ctx;
    fn crypto_skcipher_reqtfm(req: *mut skcipher_request) -> *mut crypto_skcipher;
    fn get_unaligned_le32(p: *const u8) -> u32;
    fn skcipher_walk_virt(walk: *mut skcipher_walk, req: *mut skcipher_request, maybe: bool) -> i32;
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: usize) -> i32;
    fn chacha_init(state: *mut chacha_state, key: *const u32, iv: *const u8);
    fn chacha_crypt(state: *mut chacha_state, dst: *mut u8, src: *const u8, nbytes: usize, nrounds: i32);
    fn hchacha_block(state: *mut chacha_state, out: *mut u32, nrounds: i32);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn crypto_register_skciphers(algs: *mut skcipher_alg, count: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, count: usize);
}

// These declarations correspond to types provided by the kernel crypto API.
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub iv: *const u8, _private: [u8; 0] }
#[repr(C)] pub struct skcipher_state { _private: [u8; 0] }
#[repr(C)] pub struct chacha_state { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_walk {
    pub nbytes: usize,
    pub total: usize,
    pub dst: skcipher_walk_addr,
    pub src: skcipher_walk_addr,
}
#[repr(C)] pub struct skcipher_walk_addr { pub virt: skcipher_walk_virt_addr }
#[repr(C)] pub struct skcipher_walk_virt_addr { pub addr: *mut u8 }

pub const CHACHA_KEY_SIZE: usize = 32;
pub const CHACHA_IV_SIZE: usize = 16;
pub const XCHACHA_IV_SIZE: usize = 32;
pub const CHACHA_BLOCK_SIZE: usize = 64;

unsafe fn chacha_setkey(tfm: *mut crypto_skcipher, key: *const u8, keysize: u32, nrounds: i32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm);
    if keysize as usize != CHACHA_KEY_SIZE { return -22; }
    for i in 0..8usize {
        (*ctx).key[i] = get_unaligned_le32(key.add(i * core::mem::size_of::<u32>()));
    }
    (*ctx).nrounds = nrounds;
    0
}

unsafe fn chacha20_setkey(tfm: *mut crypto_skcipher, key: *const u8, keysize: u32) -> i32 {
    chacha_setkey(tfm, key, keysize, 20)
}

unsafe fn chacha12_setkey(tfm: *mut crypto_skcipher, key: *const u8, keysize: u32) -> i32 {
    chacha_setkey(tfm, key, keysize, 12)
}

unsafe fn chacha_stream_xor(req: *mut skcipher_request, ctx: *const chacha_ctx, iv: *const u8) -> i32 {
    let mut walk = core::mem::zeroed::<skcipher_walk>();
    let mut state = core::mem::zeroed::<chacha_state>();
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    chacha_init(&mut state, (*ctx).key.as_ptr(), iv);
    while walk.nbytes > 0 {
        let mut nbytes = walk.nbytes;
        if nbytes < walk.total { nbytes = nbytes / CHACHA_BLOCK_SIZE * CHACHA_BLOCK_SIZE; }
        chacha_crypt(&mut state, walk.dst.virt.addr, walk.src.virt.addr, nbytes, (*ctx).nrounds);
        err = skcipher_walk_done(&mut walk, walk.nbytes - nbytes);
    }
    err
}

unsafe fn crypto_chacha_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    chacha_stream_xor(req, ctx, (*req).iv)
}

unsafe fn crypto_xchacha_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut subctx = core::mem::zeroed::<chacha_ctx>();
    let mut state = core::mem::zeroed::<chacha_state>();
    let mut real_iv = [0u8; 16];
    chacha_init(&mut state, (*ctx).key.as_ptr(), (*req).iv);
    hchacha_block(&mut state, subctx.key.as_mut_ptr(), (*ctx).nrounds);
    subctx.nrounds = (*ctx).nrounds;
    memcpy(real_iv.as_mut_ptr(), (*req).iv.add(24), 8);
    memcpy(real_iv.as_mut_ptr().add(8), (*req).iv.add(16), 8);
    chacha_stream_xor(req, &subctx, real_iv.as_ptr())
}

// The skcipher algorithm table and module registration are supplied by the kernel API.
extern "C" {
    static mut algs: [skcipher_alg; 3];
}

#[repr(C)] pub struct skcipher_alg { _private: [u8; 0] }

unsafe fn crypto_chacha_mod_init() -> i32 {
    crypto_register_skciphers(algs.as_mut_ptr(), 3)
}

unsafe fn crypto_chacha_mod_fini() {
    crypto_unregister_skciphers(algs.as_mut_ptr(), 3);
}

// module_init(crypto_chacha_mod_init);
// module_exit(crypto_chacha_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Martin Willi <martin@strongswan.org>");
// MODULE_DESCRIPTION("Crypto API wrappers for the ChaCha20, XChaCha20, and XChaCha12 stream ciphers");
// MODULE_ALIAS_CRYPTO("chacha20");
// MODULE_ALIAS_CRYPTO("chacha20-lib");
// MODULE_ALIAS_CRYPTO("xchacha20");
// MODULE_ALIAS_CRYPTO("xchacha20-lib");
// MODULE_ALIAS_CRYPTO("xchacha12");
// MODULE_ALIAS_CRYPTO("xchacha12-lib");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
