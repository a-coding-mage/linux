// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * The AEGIS-128 Authenticated-Encryption Algorithm
 *   Glue for AES-NI + SSE4.1 implementation
 *
 * Copyright (c) 2017-2018 Ondrej Mosnacek <omosnacek@gmail.com>
 * Copyright (C) 2017-2018 Red Hat, Inc. All rights reserved.
 */

// C dependencies:
// crypto/internal/aead.h, crypto/internal/skcipher.h, crypto/scatterwalk.h,
// linux/module.h, asm/fpu/api.h, asm/cpu_device_id.h

const AEGIS128_BLOCK_ALIGN: usize = 16;
const AEGIS128_BLOCK_SIZE: usize = 16;
const AEGIS128_NONCE_SIZE: usize = 16;
const AEGIS128_STATE_BLOCKS: usize = 5;
const AEGIS128_KEY_SIZE: usize = 16;
const AEGIS128_MIN_AUTH_SIZE: usize = 8;
const AEGIS128_MAX_AUTH_SIZE: usize = 16;

#[repr(C, align(16))]
pub struct aegis_block {
    pub bytes: [u8; AEGIS128_BLOCK_SIZE],
}

#[repr(C)]
pub struct aegis_state {
    pub blocks: [aegis_block; AEGIS128_STATE_BLOCKS],
}

#[repr(C)]
pub struct aegis_ctx {
    pub key: aegis_block,
}

extern "C" {
    pub fn aegis128_aesni_init(state: *mut aegis_state, key: *const aegis_block,
                               iv: *const u8);
    pub fn aegis128_aesni_ad(state: *mut aegis_state, data: *const u8, len: u32);
    pub fn aegis128_aesni_enc(state: *mut aegis_state, src: *const u8, dst: *mut u8, len: u32);
    pub fn aegis128_aesni_dec(state: *mut aegis_state, src: *const u8, dst: *mut u8, len: u32);
    pub fn aegis128_aesni_enc_tail(state: *mut aegis_state, src: *const u8, dst: *mut u8, len: u32);
    pub fn aegis128_aesni_dec_tail(state: *mut aegis_state, src: *const u8, dst: *mut u8, len: u32);
    pub fn aegis128_aesni_final(state: *mut aegis_state, tag_xor: *mut aegis_block,
                                assoclen: u32, cryptlen: u32);
}

// External kernel types and functions are supplied by the surrounding translation unit.
extern "C" {
    fn crypto_aead_ctx(aead: *mut crypto_aead) -> *mut u8;
    fn scatterwalk_start(walk: *mut scatter_walk, sg: *mut scatterlist);
    fn scatterwalk_next(walk: *mut scatter_walk, len: u32) -> u32;
    fn scatterwalk_done_src(walk: *mut scatter_walk, len: u32);
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn skcipher_walk_done(walk: *mut skcipher_walk, nbytes: u32) -> i32;
    fn skcipher_walk_aead_encrypt(walk: *mut skcipher_walk, req: *mut aead_request, atomic: bool) -> i32;
    fn skcipher_walk_aead_decrypt(walk: *mut skcipher_walk, req: *mut aead_request, atomic: bool) -> i32;
    fn crypto_aead_reqtfm(req: *mut aead_request) -> *mut crypto_aead;
    fn crypto_aead_authsize(tfm: *mut crypto_aead) -> u32;
    fn scatterwalk_map_and_copy(buf: *mut u8, sg: *mut scatterlist, start: u32, nbytes: u32, out: i32);
    fn crypto_memneq(a: *const u8, b: *const u8, len: u32) -> i32;
    fn crypto_register_aead(alg: *mut aead_alg) -> i32;
    fn crypto_unregister_aead(alg: *mut aead_alg);
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_has_xfeatures(mask: u64, state: *mut core::ffi::c_void) -> bool;
}

#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct scatter_walk { pub addr: *mut u8 }
#[repr(C)] pub struct skcipher_walk { pub nbytes: u32, pub src: *mut walk_virt, pub dst: *mut walk_virt }
#[repr(C)] pub struct walk_virt { pub addr: *mut u8 }
#[repr(C)] pub struct crypto_aead { _private: [u8; 0] }
#[repr(C)] pub struct aead_request { pub cryptlen: u32, pub assoclen: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub iv: *const u8 }
#[repr(C)] pub struct aead_alg { _private: [u8; 0] }

unsafe fn crypto_aegis128_aesni_process_ad(state: *mut aegis_state, sg_src: *mut scatterlist, mut assoclen: u32) {
    let mut walk = scatter_walk { addr: core::ptr::null_mut() };
    let mut buf = aegis_block { bytes: [0; AEGIS128_BLOCK_SIZE] };
    let mut pos: u32 = 0;
    scatterwalk_start(&mut walk, sg_src);
    while assoclen != 0 {
        let size = scatterwalk_next(&mut walk, assoclen);
        let mut src = walk.addr;
        let mut left = size;
        if pos + size >= AEGIS128_BLOCK_SIZE as u32 {
            if pos > 0 {
                let fill = AEGIS128_BLOCK_SIZE as u32 - pos;
                core::ptr::copy_nonoverlapping(src, buf.bytes.as_mut_ptr().add(pos as usize), fill as usize);
                aegis128_aesni_ad(state, buf.bytes.as_ptr(), AEGIS128_BLOCK_SIZE as u32);
                pos = 0; left -= fill; src = src.add(fill as usize);
            }
            let full = left & !(AEGIS128_BLOCK_SIZE as u32 - 1);
            aegis128_aesni_ad(state, src, full);
            src = src.add(full as usize); left &= AEGIS128_BLOCK_SIZE as u32 - 1;
        }
        core::ptr::copy_nonoverlapping(src, buf.bytes.as_mut_ptr().add(pos as usize), left as usize);
        pos += left; assoclen -= size; scatterwalk_done_src(&mut walk, size);
    }
    if pos > 0 { for b in &mut buf.bytes[pos as usize..] { *b = 0; } aegis128_aesni_ad(state, buf.bytes.as_ptr(), AEGIS128_BLOCK_SIZE as u32); }
}

unsafe fn crypto_aegis128_aesni_process_crypt(state: *mut aegis_state, walk: *mut skcipher_walk, enc: bool) -> i32 {
    let mut err = 0;
    while (*walk).nbytes >= AEGIS128_BLOCK_SIZE as u32 {
        let n = (*walk).nbytes & !(AEGIS128_BLOCK_SIZE as u32 - 1);
        if enc { aegis128_aesni_enc(state, (*(*walk).src).addr, (*(*walk).dst).addr, n); }
        else { aegis128_aesni_dec(state, (*(*walk).src).addr, (*(*walk).dst).addr, n); }
        kernel_fpu_end(); err = skcipher_walk_done(walk, (*walk).nbytes % AEGIS128_BLOCK_SIZE as u32); kernel_fpu_begin();
    }
    if (*walk).nbytes != 0 {
        if enc { aegis128_aesni_enc_tail(state, (*(*walk).src).addr, (*(*walk).dst).addr, (*walk).nbytes); }
        else { aegis128_aesni_dec_tail(state, (*(*walk).src).addr, (*(*walk).dst).addr, (*walk).nbytes); }
        kernel_fpu_end(); err = skcipher_walk_done(walk, 0); kernel_fpu_begin();
    } err
}

unsafe fn crypto_aegis128_aesni_ctx(aead: *mut crypto_aead) -> *mut aegis_ctx {
    let p = crypto_aead_ctx(aead).add(core::mem::align_of::<aegis_ctx>() - 1);
    (p as usize & !(core::mem::align_of::<aegis_ctx>() - 1)) as *mut aegis_ctx
}

unsafe fn crypto_aegis128_aesni_setkey(aead: *mut crypto_aead, key: *const u8, keylen: u32) -> i32 {
    if keylen != AEGIS128_KEY_SIZE as u32 { return -22; }
    core::ptr::copy_nonoverlapping(key, (*crypto_aegis128_aesni_ctx(aead)).key.bytes.as_mut_ptr(), AEGIS128_KEY_SIZE); 0
}
unsafe fn crypto_aegis128_aesni_setauthsize(_: *mut crypto_aead, authsize: u32) -> i32 {
    if authsize > AEGIS128_MAX_AUTH_SIZE as u32 || authsize < AEGIS128_MIN_AUTH_SIZE as u32 { -22 } else { 0 }
}
unsafe fn crypto_aegis128_aesni_crypt(req: *mut aead_request, tag: *mut aegis_block, cryptlen: u32, enc: bool) -> i32 {
    let tfm = crypto_aead_reqtfm(req); let ctx = crypto_aegis128_aesni_ctx(tfm); let mut walk = skcipher_walk { nbytes: 0, src: core::ptr::null_mut(), dst: core::ptr::null_mut() }; let mut state = core::mem::zeroed();
    let mut err = if enc { skcipher_walk_aead_encrypt(&mut walk, req, false) } else { skcipher_walk_aead_decrypt(&mut walk, req, false) }; if err != 0 { return err; }
    kernel_fpu_begin(); aegis128_aesni_init(&mut state, &(*ctx).key, (*req).iv); crypto_aegis128_aesni_process_ad(&mut state, (*req).src, (*req).assoclen); err = crypto_aegis128_aesni_process_crypt(&mut state, &mut walk, enc); if err == 0 { aegis128_aesni_final(&mut state, tag, (*req).assoclen, cryptlen); } kernel_fpu_end(); err
}
unsafe fn crypto_aegis128_aesni_encrypt(req: *mut aead_request) -> i32 { let tfm = crypto_aead_reqtfm(req); let mut tag = aegis_block { bytes: [0; 16] }; let n = (*req).cryptlen; let a = crypto_aead_authsize(tfm); let e = crypto_aegis128_aesni_crypt(req, &mut tag, n, true); if e != 0 { e } else { scatterwalk_map_and_copy(tag.bytes.as_mut_ptr(), (*req).dst, (*req).assoclen + n, a, 1); 0 } }
unsafe fn crypto_aegis128_aesni_decrypt(req: *mut aead_request) -> i32 { let tfm = crypto_aead_reqtfm(req); let a = crypto_aead_authsize(tfm); let n = (*req).cryptlen - a; let mut tag = core::mem::zeroed(); scatterwalk_map_and_copy(tag.bytes.as_mut_ptr(), (*req).src, (*req).assoclen + n, a, 0); let e = crypto_aegis128_aesni_crypt(req, &mut tag, n, false); if e != 0 { e } else { if crypto_memneq(tag.bytes.as_ptr(), [0u8; 16].as_ptr(), a) != 0 { -74 } else { 0 } } }

// struct aead_alg crypto_aegis128_aesni_alg: .setkey, .setauthsize, .encrypt,
// .decrypt, .ivsize = 16, .maxauthsize = 16, .chunksize = 16,
// .cra_blocksize = 1, .cra_ctxsize = sizeof(struct aegis_ctx) + alignof(struct aegis_ctx),
// .cra_priority = 400, .cra_name = "aegis128", .cra_driver_name = "aegis128-aesni".
// The kernel registration object and CPU feature checks remain external build integration.
// module_init(crypto_aegis128_aesni_module_init);
// module_exit(crypto_aegis128_aesni_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ondrej Mosnacek <omosnacek@gmail.com>");
// MODULE_DESCRIPTION("AEGIS-128 AEAD algorithm -- AESNI+SSE4.1 implementation");
// MODULE_ALIAS_CRYPTO("aegis128");
// MODULE_ALIAS_CRYPTO("aegis128-aesni");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
