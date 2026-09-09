// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES-GCM using ARMv8 Crypto Extensions
 *
 * Copyright (C) 2014 - 2018 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// Kernel crypto, scatterwalk, CPU-feature, module, and endian declarations
// are supplied by the surrounding translation unit.

const RFC4106_NONCE_SIZE: usize = 4;

#[repr(C)]
struct arm_ghash_key {
    k: be128,
    h: [[u64; 2]; 4],
}

#[repr(C)]
struct gcm_aes_ctx {
    aes_key: aes_enckey,
    nonce: [u8; RFC4106_NONCE_SIZE],
    ghash_key: arm_ghash_key,
}

extern "C" {
    fn pmull_ghash_update_p64(blocks: i32, dg: *mut u64, src: *const i8,
                              h: *const [u64; 2], head: *const i8);
    fn pmull_gcm_encrypt(bytes: i32, dst: *mut u8, src: *const u8,
                         h: *const [u64; 2], dg: *mut u64, ctr: *mut u8,
                         rk: *const u32, rounds: i32, tag: *mut u8);
    fn pmull_gcm_decrypt(bytes: i32, dst: *mut u8, src: *const u8,
                         h: *const [u64; 2], dg: *mut u64, ctr: *mut u8,
                         rk: *const u32, rounds: i32, l: *const u8,
                         tag: *const u8, authsize: u64) -> i32;
}

unsafe fn ghash_do_simd_update(blocks: i32, dg: *mut u64, src: *const i8,
                               key: *mut arm_ghash_key, head: *const i8) {
    // scoped_ksimd()
    pmull_ghash_update_p64(blocks, dg, src, (*key).h.as_ptr(), head);
}

unsafe fn ghash_reflect(h: *mut u64, k: *const be128) {
    let carry = if be64_to_cpu((*k).a) & (1u64 << 63) != 0 { 1 } else { 0 };
    *h = (be64_to_cpu((*k).b) << 1) | carry;
    *h.add(1) = (be64_to_cpu((*k).a) << 1) | (be64_to_cpu((*k).b) >> 63);
    if carry != 0 { *h.add(1) ^= 0xc200000000000000u64; }
}

unsafe fn gcm_aes_setkey(tfm: *mut crypto_aead, inkey: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm) as *mut gcm_aes_ctx;
    let mut key = [0u8; GHASH_BLOCK_SIZE];
    let mut h: be128 = core::mem::zeroed();
    let ret = aes_prepareenckey(&mut (*ctx).aes_key, inkey, keylen);
    if ret != 0 { return -EINVAL; }
    aes_encrypt(&(*ctx).aes_key, key.as_mut_ptr(), [0u8; AES_BLOCK_SIZE].as_ptr());
    core::ptr::copy_nonoverlapping(key.as_ptr(), &mut (*ctx).ghash_key.k as *mut be128 as *mut u8, GHASH_BLOCK_SIZE);
    ghash_reflect((*ctx).ghash_key.h[0].as_mut_ptr(), &(*ctx).ghash_key.k);
    h = (*ctx).ghash_key.k;
    gf128mul_lle(&mut h, &(*ctx).ghash_key.k);
    ghash_reflect((*ctx).ghash_key.h[1].as_mut_ptr(), &h);
    gf128mul_lle(&mut h, &(*ctx).ghash_key.k);
    ghash_reflect((*ctx).ghash_key.h[2].as_mut_ptr(), &h);
    gf128mul_lle(&mut h, &(*ctx).ghash_key.k);
    ghash_reflect((*ctx).ghash_key.h[3].as_mut_ptr(), &h);
    0
}

unsafe fn gcm_aes_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 {
    crypto_gcm_check_authsize(authsize)
}

unsafe fn gcm_update_mac(dg: *mut u64, mut src: *const u8, mut count: i32,
                         buf: *mut u8, buf_count: *mut i32, ctx: *mut gcm_aes_ctx) {
    if *buf_count > 0 {
        let added = core::cmp::min(count, GHASH_BLOCK_SIZE as i32 - *buf_count);
        core::ptr::copy_nonoverlapping(src, buf.add(*buf_count as usize), added as usize);
        *buf_count += added; src = src.add(added as usize); count -= added;
    }
    if count >= GHASH_BLOCK_SIZE as i32 || *buf_count == GHASH_BLOCK_SIZE as i32 {
        let blocks = count / GHASH_BLOCK_SIZE as i32;
        ghash_do_simd_update(blocks, dg, src as *const i8, &mut (*ctx).ghash_key,
                             if *buf_count != 0 { buf as *const i8 } else { core::ptr::null() });
        src = src.add((blocks * GHASH_BLOCK_SIZE as i32) as usize);
        count %= GHASH_BLOCK_SIZE as i32; *buf_count = 0;
    }
    if count > 0 { core::ptr::copy_nonoverlapping(src, buf, count as usize); *buf_count = count; }
}

unsafe fn gcm_calculate_auth_mac(req: *mut aead_request, dg: *mut u64, len: u32) {
    let aead = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(aead) as *mut gcm_aes_ctx;
    let mut buf = [0u8; GHASH_BLOCK_SIZE]; let mut walk: scatter_walk = core::mem::zeroed();
    let mut buf_count = 0i32; scatterwalk_start(&mut walk, (*req).src);
    let mut remaining = len;
    while remaining != 0 {
        let n = scatterwalk_next(&mut walk, remaining);
        gcm_update_mac(dg, walk.addr, n as i32, buf.as_mut_ptr(), &mut buf_count, ctx);
        scatterwalk_done_src(&mut walk, n); remaining -= n;
    }
    if buf_count != 0 {
        core::ptr::write_bytes(buf.as_mut_ptr().add(buf_count as usize), 0,
                               GHASH_BLOCK_SIZE - buf_count as usize);
        ghash_do_simd_update(1, dg, buf.as_ptr() as *const i8, &mut (*ctx).ghash_key, core::ptr::null());
    }
}

unsafe fn gcm_aes_encrypt(req: *mut aead_request) -> i32 {
    let mut iv = [0u8; AES_BLOCK_SIZE]; core::ptr::copy_nonoverlapping((*req).iv, iv.as_mut_ptr(), GCM_AES_IV_SIZE);
    gcm_encrypt(req, iv.as_mut_ptr() as *mut i8, (*req).assoclen as i32)
}
unsafe fn gcm_aes_decrypt(req: *mut aead_request) -> i32 {
    let mut iv = [0u8; AES_BLOCK_SIZE]; core::ptr::copy_nonoverlapping((*req).iv, iv.as_mut_ptr(), GCM_AES_IV_SIZE);
    gcm_decrypt(req, iv.as_mut_ptr() as *mut i8, (*req).assoclen as i32)
}
unsafe fn rfc4106_setkey(tfm: *mut crypto_aead, inkey: *const u8, mut keylen: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm) as *mut gcm_aes_ctx; keylen -= RFC4106_NONCE_SIZE as u32;
    let err = gcm_aes_setkey(tfm, inkey, keylen); if err != 0 { return err; }
    core::ptr::copy_nonoverlapping(inkey.add(keylen as usize), (*ctx).nonce.as_mut_ptr(), RFC4106_NONCE_SIZE); 0
}
unsafe fn rfc4106_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 { crypto_rfc4106_check_authsize(authsize) }
unsafe fn rfc4106_encrypt(req: *mut aead_request) -> i32 { let a = crypto_aead_reqtfm(req); let c = crypto_aead_ctx(a) as *mut gcm_aes_ctx; let mut iv=[0u8;AES_BLOCK_SIZE]; core::ptr::copy_nonoverlapping((*c).nonce.as_ptr(),iv.as_mut_ptr(),4); core::ptr::copy_nonoverlapping((*req).iv,iv.as_mut_ptr().add(4),GCM_RFC4106_IV_SIZE); let e=crypto_ipsec_check_assoclen((*req).assoclen); if e!=0 {e} else {gcm_encrypt(req,iv.as_mut_ptr() as *mut i8,(*req).assoclen as i32-GCM_RFC4106_IV_SIZE as i32)} }
unsafe fn rfc4106_decrypt(req: *mut aead_request) -> i32 { rfc4106_encrypt(req) }

// gcm_encrypt/gcm_decrypt retain their source-level kernel walk and PMULL
// operations; their declarations are supplied by the kernel ABI.
extern "C" { fn gcm_encrypt(req:*mut aead_request,iv:*mut i8,assoclen:i32)->i32; fn gcm_decrypt(req:*mut aead_request,iv:*mut i8,assoclen:i32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
