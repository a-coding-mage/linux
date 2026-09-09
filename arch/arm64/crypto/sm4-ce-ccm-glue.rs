/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4-CCM AEAD Algorithm using ARMv8 Crypto Extensions
 * as specified in rfc8998
 * https://datatracker.ietf.org/doc/html/rfc8998
 *
 * Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// Kernel and architecture declarations are supplied by the surrounding Rust
// translation environment.

extern "C" {
    fn sm4_ce_cbcmac_update(rkey_enc: *const u32, mac: *mut u8, src: *const u8, nblocks: u32);
    fn sm4_ce_ccm_enc(rkey_enc: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nbytes: u32, mac: *mut u8);
    fn sm4_ce_ccm_dec(rkey_enc: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nbytes: u32, mac: *mut u8);
    fn sm4_ce_ccm_final(rkey_enc: *const u32, iv: *mut u8, mac: *mut u8);
}

unsafe fn ccm_setkey(tfm: *mut crypto_aead, key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm);
    if key_len != SM4_KEY_SIZE { return -EINVAL; }
    sm4_ce_expand_key(key, (*ctx).rkey_enc.as_mut_ptr(), (*ctx).rkey_dec.as_mut_ptr(), crypto_sm4_fk.as_ptr(), crypto_sm4_ck.as_ptr());
    0
}

unsafe fn ccm_setauthsize(_tfm: *mut crypto_aead, authsize: u32) -> i32 {
    if (authsize & 1) != 0 || authsize < 4 { return -EINVAL; }
    0
}

unsafe fn ccm_format_input(info: *mut u8, req: *mut aead_request, msglen: u32) -> i32 {
    let aead = crypto_aead_reqtfm(req);
    let mut l = (*req).iv[0] as u32 + 1;
    if l < 2 || l > 8 { return -EINVAL; }
    if l < 4 && (msglen >> (8 * l)) != 0 { return -EOVERFLOW; }
    memset((*req).iv.as_mut_ptr().add(SM4_BLOCK_SIZE - l as usize), 0, l as usize);
    memcpy(info, (*req).iv.as_ptr(), SM4_BLOCK_SIZE);
    let m = crypto_aead_authsize(aead);
    *info |= (((m - 2) / 2) << 3) as u8;
    if (*req).assoclen != 0 { *info |= 1 << 6; }
    if l >= 4 { l = 4; }
    let len = msglen.to_be_bytes();
    memcpy(info.add(SM4_BLOCK_SIZE - l as usize), len.as_ptr().add(4 - l as usize), l as usize);
    0
}

unsafe fn ccm_calculate_auth_mac(req: *mut aead_request, mac: *mut u8) {
    let aead = crypto_aead_reqtfm(req);
    let ctx = crypto_aead_ctx(aead);
    let mut aadlen: [u8; 6] = [0; 6];
    let mut assoclen = (*req).assoclen;
    let mut len: usize;
    if assoclen < 0xff00 {
        (assoclen as u16).to_be_bytes().iter().enumerate().for_each(|(i, b)| aadlen[i] = *b);
        len = 2;
    } else {
        0xfffeu16.to_be_bytes().iter().enumerate().for_each(|(i, b)| aadlen[i] = *b);
        aadlen[2..6].copy_from_slice(&assoclen.to_be_bytes());
        len = 6;
    }
    sm4_ce_crypt_block((*ctx).rkey_enc.as_ptr(), mac, mac);
    crypto_xor(mac, aadlen.as_ptr(), len);
    let mut walk = scatterwalk_start((*req).src);
    while assoclen != 0 {
        let orig_n = scatterwalk_next(&mut walk, assoclen);
        let mut p = walk.addr;
        let mut n = orig_n;
        while n > 0 {
            if len == SM4_BLOCK_SIZE {
                if n < SM4_BLOCK_SIZE {
                    sm4_ce_crypt_block((*ctx).rkey_enc.as_ptr(), mac, mac);
                    len = 0;
                } else {
                    let nblocks = n / SM4_BLOCK_SIZE;
                    sm4_ce_cbcmac_update((*ctx).rkey_enc.as_ptr(), mac, p, nblocks as u32);
                    p = p.add(nblocks * SM4_BLOCK_SIZE); n %= SM4_BLOCK_SIZE; continue;
                }
            }
            let l = core::cmp::min(n, SM4_BLOCK_SIZE - len);
            if l != 0 { crypto_xor(mac.add(len), p, l); len += l; p = p.add(l); n -= l; }
        }
        scatterwalk_done_src(&mut walk, orig_n); assoclen -= orig_n;
    }
}

unsafe fn ccm_crypt(req: *mut aead_request, walk: *mut skcipher_walk, rkey_enc: *const u32, mac: *mut u8, crypt: unsafe extern "C" fn(*const u32, *mut u8, *const u8, *mut u8, u32, *mut u8)) -> i32 {
    let mut ctr0 = [0u8; SM4_BLOCK_SIZE];
    memcpy(ctr0.as_mut_ptr(), (*walk).iv, SM4_BLOCK_SIZE); crypto_inc((*walk).iv, SM4_BLOCK_SIZE);
    if (*req).assoclen != 0 { ccm_calculate_auth_mac(req, mac); }
    let mut err = 0;
    while (*walk).nbytes != 0 {
        let mut tail = (*walk).nbytes % SM4_BLOCK_SIZE;
        if (*walk).nbytes == (*walk).total { tail = 0; }
        crypt(rkey_enc, (*walk).dst.virt.addr, (*walk).src.virt.addr, (*walk).iv, (*walk).nbytes - tail, mac);
        err = skcipher_walk_done(walk, tail);
    }
    sm4_ce_ccm_final(rkey_enc, ctr0.as_mut_ptr(), mac); err
}

unsafe fn ccm_encrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req); let ctx = crypto_aead_ctx(aead); let mut mac = [0u8; SM4_BLOCK_SIZE]; let mut walk = core::mem::zeroed();
    let mut err = ccm_format_input(mac.as_mut_ptr(), req, (*req).cryptlen); if err != 0 { return err; }
    err = skcipher_walk_aead_encrypt(&mut walk, req, false); if err != 0 { return err; }
    err = ccm_crypt(req, &mut walk, (*ctx).rkey_enc.as_ptr(), mac.as_mut_ptr(), sm4_ce_ccm_enc); if err != 0 { return err; }
    scatterwalk_map_and_copy(mac.as_ptr(), (*req).dst, (*req).assoclen + (*req).cryptlen, crypto_aead_authsize(aead), 1); 0
}

unsafe fn ccm_decrypt(req: *mut aead_request) -> i32 {
    let aead = crypto_aead_reqtfm(req); let authsize = crypto_aead_authsize(aead); let ctx = crypto_aead_ctx(aead); let mut mac = [0u8; SM4_BLOCK_SIZE]; let mut authtag = [0u8; SM4_BLOCK_SIZE]; let mut walk = core::mem::zeroed();
    let mut err = ccm_format_input(mac.as_mut_ptr(), req, (*req).cryptlen - authsize); if err != 0 { return err; }
    err = skcipher_walk_aead_decrypt(&mut walk, req, false); if err != 0 { return err; }
    err = ccm_crypt(req, &mut walk, (*ctx).rkey_enc.as_ptr(), mac.as_mut_ptr(), sm4_ce_ccm_dec); if err != 0 { return err; }
    scatterwalk_map_and_copy(authtag.as_mut_ptr(), (*req).src, (*req).assoclen + (*req).cryptlen - authsize, authsize, 0);
    if crypto_memneq(authtag.as_ptr(), mac.as_ptr(), authsize) != 0 { return -EBADMSG; } 0
}

// The kernel registration, module metadata, and aead_alg initializer retain
// the corresponding C interface and are supplied by the surrounding bindings.
static mut sm4_ccm_alg: aead_alg = aead_alg { base: crypto_alg { cra_name: b"ccm(sm4)\0".as_ptr(), cra_driver_name: b"ccm-sm4-ce\0".as_ptr(), cra_priority: 400, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<sm4_ctx>(), cra_module: core::ptr::null_mut() }, ivsize: SM4_BLOCK_SIZE, chunksize: SM4_BLOCK_SIZE, maxauthsize: SM4_BLOCK_SIZE, setkey: Some(ccm_setkey), setauthsize: Some(ccm_setauthsize), encrypt: Some(ccm_encrypt), decrypt: Some(ccm_decrypt) };

unsafe fn sm4_ce_ccm_init() -> i32 { crypto_register_aead(&mut sm4_ccm_alg) }
unsafe fn sm4_ce_ccm_exit() { crypto_unregister_aead(&mut sm4_ccm_alg); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
