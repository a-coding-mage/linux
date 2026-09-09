// SPDX-License-Identifier: GPL-2.0-only
/*
 * aes-ce-ccm-glue.c - AES-CCM transform for ARMv8 with Crypto Extensions
 *
 * Copyright (C) 2013 - 2017 Linaro Ltd.
 * Copyright (C) 2024 Google LLC
 *
 * Author: Ard Biesheuvel <ardb@kernel.org>
 */

// C dependencies supplied by the kernel crypto implementation are intentionally
// left as external Rust symbols.

extern "C" {
    fn ce_aes_expandkey(ctx: *mut crypto_aes_ctx, in_key: *const u8, key_len: u32) -> i32;
    fn ce_aes_mac_update(in_: *const u8, rk: *const u32, rounds: u32,
                         blocks: u32, mac: *mut u8, macp: u32, enc_after: u32);
    fn crypto_xor(dst: *mut u8, src: *const u8, len: u32);
    fn cpu_to_be32(x: u32) -> u32;
    fn put_unaligned_be32(x: u32, p: *mut u8);
    fn crypto_aead_ctx(tfm: *mut crypto_aead) -> *mut crypto_aes_ctx;
    fn crypto_aead_reqtfm(req: *mut aead_request) -> *mut crypto_aead;
    fn crypto_aead_authsize(aead: *mut crypto_aead) -> u32;
    fn scatterwalk_start(walk: *mut scatter_walk, src: *mut core::ffi::c_void);
    fn scatterwalk_next(walk: *mut scatter_walk, len: u32) -> u32;
    fn scatterwalk_done_src(walk: *mut scatter_walk, n: u32);
    fn skcipher_walk_aead_encrypt(walk: *mut skcipher_walk, req: *mut aead_request,
                                  atomic: bool) -> i32;
    fn skcipher_walk_aead_decrypt(walk: *mut skcipher_walk, req: *mut aead_request,
                                  atomic: bool) -> i32;
    fn skcipher_walk_done(walk: *mut skcipher_walk, tail: u32) -> i32;
    fn scatterwalk_map_and_copy(buf: *mut u8, sg: *mut core::ffi::c_void,
                                offset: u32, len: u32, to_buffer: i32);
    fn crypto_memneq(a: *const u8, b: *const u8, len: u32) -> i32;
    fn cpu_have_named_feature(feature: u32) -> bool;
    fn crypto_register_aead(alg: *mut aead_alg) -> i32;
    fn crypto_unregister_aead(alg: *mut aead_alg);
}

const AES_BLOCK_SIZE: u32 = 16;

#[repr(C)]
struct crypto_aes_ctx { key_length: u32, key_enc: [u32; 60] }
#[repr(C)] struct crypto_aead { _private: [u8; 0] }
#[repr(C)] struct aead_request {
    iv: *mut u8, assoclen: u32, cryptlen: u32,
    src: *mut core::ffi::c_void, dst: *mut core::ffi::c_void,
}
#[repr(C)] struct scatter_walk { addr: *mut u8 }
#[repr(C)] struct skcipher_walk {
    nbytes: u32, total: u32, src: *mut u8, dst: *mut u8, iv: *mut u8,
}
#[repr(C)] struct aead_alg { _private: [u8; 0] }

unsafe fn num_rounds(ctx: *mut crypto_aes_ctx) -> u32 {
    // # of rounds specified by AES: 128 bit key = 10, 192 bit key = 12,
    // 256 bit key = 14; n byte key => 6 + (n/4) rounds.
    6 + (*ctx).key_length / 4
}

extern "C" {
    fn ce_aes_ccm_encrypt(out: *mut u8, input: *const u8, cbytes: u32,
                          rk: *const u32, rounds: u32, mac: *mut u8,
                          ctr: *mut u8, final_iv: *const u8);
    fn ce_aes_ccm_decrypt(out: *mut u8, input: *const u8, cbytes: u32,
                          rk: *const u32, rounds: u32, mac: *mut u8,
                          ctr: *mut u8, final_iv: *const u8);
}

unsafe fn ccm_setkey(tfm: *mut crypto_aead, in_key: *const u8, key_len: u32) -> i32 {
    ce_aes_expandkey(crypto_aead_ctx(tfm), in_key, key_len)
}

unsafe fn ccm_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 {
    if authsize & 1 != 0 || authsize < 4 { return -22; }
    0
}

unsafe fn ccm_init_mac(req: *mut aead_request, maciv: *mut u8, msglen: u32) -> i32 {
    let aead = crypto_aead_reqtfm(req);
    let l = *(*req).iv as u32 + 1;
    if l < 2 || l > 8 { return -22; }
    if l < 4 && (msglen >> (8 * l)) != 0 { return -75; }
    *((maciv.add(8)) as *mut u32) = 0;
    *((maciv.add(12)) as *mut u32) = cpu_to_be32(msglen);
    core::ptr::copy_nonoverlapping((*req).iv, maciv, (AES_BLOCK_SIZE - l) as usize);
    *maciv |= ((crypto_aead_authsize(aead) - 2) << 2) as u8;
    if (*req).assoclen != 0 { *maciv |= 0x40; }
    core::ptr::write_bytes((*req).iv.add((AES_BLOCK_SIZE - l) as usize), 0, l as usize);
    0
}

unsafe fn ce_aes_ccm_auth_data(mac: *mut u8, mut input: *const u8, mut abytes: u32,
                               mut macp: u32, rk: *const u32, rounds: u32) -> u32 {
    let enc_after = (macp + abytes) % AES_BLOCK_SIZE;
    while abytes > 0 {
        let blocks = abytes / AES_BLOCK_SIZE;
        if macp == AES_BLOCK_SIZE || (macp == 0 && blocks > 0) {
            ce_aes_mac_update(input, rk, rounds, blocks, mac, macp, enc_after);
            macp = if enc_after != 0 { 0 } else { AES_BLOCK_SIZE };
            input = input.add((blocks * AES_BLOCK_SIZE) as usize);
            abytes -= blocks * AES_BLOCK_SIZE;
        } else {
            let l = core::cmp::min(AES_BLOCK_SIZE - macp, abytes);
            crypto_xor(mac.add(macp as usize), input, l);
            input = input.add(l as usize); macp += l; abytes -= l;
        }
    }
    macp
}

unsafe fn ccm_calculate_auth_mac(req: *mut aead_request, mac: *mut u8) {
    let aead = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx(aead);
    let mut tag = [0u8; 8]; let len = (*req).assoclen;
    let tag_len: u32;
    if len < 0xff00 { tag[0] = (len >> 8) as u8; tag[1] = len as u8; tag_len = 2; }
    else { tag[0] = 0xff; tag[1] = 0xfe; put_unaligned_be32(len, tag.as_mut_ptr().add(2)); tag_len = 6; }
    ce_aes_ccm_auth_data(mac, tag.as_ptr(), tag_len, 16, (*ctx).key_enc.as_ptr(), num_rounds(ctx));
    let mut walk = scatter_walk { addr: core::ptr::null_mut() };
    scatterwalk_start(&mut walk, (*req).src);
    let mut remaining = len; let mut macp = 16;
    while remaining != 0 {
        let n = scatterwalk_next(&mut walk, remaining);
        macp = ce_aes_ccm_auth_data(mac, walk.addr, n, macp, (*ctx).key_enc.as_ptr(), num_rounds(ctx));
        scatterwalk_done_src(&mut walk, n); remaining -= n;
    }
}

// The remaining request-walk and algorithm registration glue is represented
// directly through the kernel-provided external structures and helpers.
unsafe fn ccm_encrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx(aead);
    let mut mac = [0u8; 16]; let mut orig_iv = [0u8; 16];
    let len = (*req).cryptlen;
    let mut err = ccm_init_mac(req, mac.as_mut_ptr(), len); if err != 0 { return err; }
    core::ptr::copy_nonoverlapping((*req).iv, orig_iv.as_mut_ptr(), 16);
    let mut walk = skcipher_walk { nbytes: 0, total: 0, src: core::ptr::null_mut(), dst: core::ptr::null_mut(), iv: core::ptr::null_mut() };
    err = skcipher_walk_aead_encrypt(&mut walk, req, false); if err != 0 { return err; }
    if (*req).assoclen != 0 { ccm_calculate_auth_mac(req, mac.as_mut_ptr()); }
    while walk.nbytes != 0 {
        let mut tail = walk.nbytes % 16; let mut final_iv: *const u8 = core::ptr::null();
        if walk.nbytes == walk.total { tail = 0; final_iv = orig_iv.as_ptr(); }
        ce_aes_ccm_encrypt(walk.dst, walk.src, walk.nbytes - tail, (*ctx).key_enc.as_ptr(), num_rounds(ctx), mac.as_mut_ptr(), walk.iv, final_iv);
        if walk.nbytes != 0 { err = skcipher_walk_done(&mut walk, tail); }
    }
    if err != 0 { return err; }
    scatterwalk_map_and_copy(mac.as_mut_ptr(), (*req).dst, (*req).assoclen + (*req).cryptlen, crypto_aead_authsize(aead), 1); 0
}

unsafe fn ccm_decrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx(aead);
    let authsize = crypto_aead_authsize(aead); let len = (*req).cryptlen - authsize;
    let mut mac = [0u8; 16]; let mut orig_iv = [0u8; 16];
    let mut err = ccm_init_mac(req, mac.as_mut_ptr(), len); if err != 0 { return err; }
    core::ptr::copy_nonoverlapping((*req).iv, orig_iv.as_mut_ptr(), 16);
    let mut walk = skcipher_walk { nbytes: 0, total: 0, src: core::ptr::null_mut(), dst: core::ptr::null_mut(), iv: core::ptr::null_mut() };
    err = skcipher_walk_aead_decrypt(&mut walk, req, false); if err != 0 { return err; }
    if (*req).assoclen != 0 { ccm_calculate_auth_mac(req, mac.as_mut_ptr()); }
    while walk.nbytes != 0 {
        let mut tail = walk.nbytes % 16; let mut final_iv: *const u8 = core::ptr::null();
        if walk.nbytes == walk.total { tail = 0; final_iv = orig_iv.as_ptr(); }
        ce_aes_ccm_decrypt(walk.dst, walk.src, walk.nbytes - tail, (*ctx).key_enc.as_ptr(), num_rounds(ctx), mac.as_mut_ptr(), walk.iv, final_iv);
        if walk.nbytes != 0 { err = skcipher_walk_done(&mut walk, tail); }
    }
    if err != 0 { return err; }
    scatterwalk_map_and_copy(orig_iv.as_mut_ptr(), (*req).src, (*req).assoclen + (*req).cryptlen - authsize, authsize, 0);
    if crypto_memneq(mac.as_ptr(), orig_iv.as_ptr(), authsize) != 0 { return -74; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
