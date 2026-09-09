// SPDX-License-Identifier: GPL-2.0-or-later
/* rfc8009 AES Encryption with HMAC-SHA2 for Kerberos 5
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel and internal dependencies are supplied by other translation units.

static rfc8009_no_context: krb5_buffer = krb5_buffer { len: 0, data: b"\0".as_ptr() as *mut u8 };

/*
 * Calculate the key derivation function KDF-HMAC-SHA2(key, label, [context,] k)
 *
 *     KDF-HMAC-SHA2(key, label, [context,] k) = k-truncate(K1)
 *
 *     Using the appropriate one of:
 *         K1 = HMAC-SHA-256(key, 0x00000001 | label | 0x00 | k)
 *         K1 = HMAC-SHA-384(key, 0x00000001 | label | 0x00 | k)
 *         K1 = HMAC-SHA-256(key, 0x00000001 | label | 0x00 | context | k)
 *         K1 = HMAC-SHA-384(key, 0x00000001 | label | 0x00 | context | k)
 *     [rfc8009 sec 3]
 */
unsafe fn rfc8009_calc_KDF_HMAC_SHA2(krb5: *const krb5_enctype, key: *const krb5_buffer,
    label: *const krb5_buffer, context: *const krb5_buffer, k: c_uint,
    result: *mut krb5_buffer, gfp: gfp_t) -> c_int {
    let mut shash: *mut crypto_shash;
    let mut k1: krb5_buffer = core::mem::zeroed();
    let mut data: krb5_buffer = core::mem::zeroed();
    let mut desc: *mut shash_desc;
    let mut tmp: __be32;
    let bsize: usize;
    let mut buffer: *mut c_void;
    let mut p: *mut u8;
    let mut ret: c_int = -ENOMEM;

    if WARN_ON((*result).len != k / 8) { return -EINVAL; }
    shash = crypto_alloc_shash((*krb5).cksum_name, 0, 0);
    if IS_ERR(shash) { return if PTR_ERR(shash) == -ENOENT { -ENOPKG } else { PTR_ERR(shash) }; }
    ret = crypto_shash_setkey(shash, (*key).data, (*key).len);
    if ret < 0 { crypto_free_shash(shash); return ret; }
    ret = -EINVAL;
    if WARN_ON(crypto_shash_digestsize(shash) * 8 < k) { crypto_free_shash(shash); return ret; }
    ret = -ENOMEM;
    (*&mut data).len = 4 + (*label).len + 1 + (*context).len + 4;
    bsize = krb5_shash_size(shash) + krb5_digest_size(shash) + crypto_roundup(data.len);
    buffer = kzalloc(bsize, GFP_NOFS);
    if buffer.is_null() { crypto_free_shash(shash); return ret; }
    desc = buffer as *mut shash_desc;
    (*desc).tfm = shash;
    ret = crypto_shash_init(desc);
    if ret < 0 { kfree_sensitive(buffer); crypto_free_shash(shash); return ret; }
    p = buffer.add(krb5_shash_size(shash) + krb5_digest_size(shash)) as *mut u8;
    data.data = p as *mut u8;
    *(p as *mut __be32) = htonl(0x00000001); p = p.add(4);
    memcpy(p as *mut c_void, (*label).data as *const c_void, (*label).len); p = p.add((*label).len);
    *p = 0; p = p.add(1);
    memcpy(p as *mut c_void, (*context).data as *const c_void, (*context).len); p = p.add((*context).len);
    tmp = htonl(k); memcpy(p as *mut c_void, &tmp as *const _ as *const c_void, 4); p = p.add(4);
    ret = -EINVAL;
    if WARN_ON(p.offset_from(data.data) as usize != data.len) { kfree_sensitive(buffer); crypto_free_shash(shash); return ret; }
    k1.len = crypto_shash_digestsize(shash); k1.data = buffer.add(krb5_shash_size(shash)) as *mut u8;
    ret = crypto_shash_finup(desc, data.data, data.len, k1.data);
    if ret >= 0 { memcpy((*result).data as *mut c_void, k1.data as *const c_void, (*result).len); }
    kfree_sensitive(buffer); crypto_free_shash(shash); ret
}

/* Calculate the pseudo-random function, PRF(). [rfc8009 sec 5] */
unsafe fn rfc8009_calc_PRF(krb5: *const krb5_enctype, input_key: *const krb5_buffer,
    octet_string: *const krb5_buffer, result: *mut krb5_buffer, gfp: gfp_t) -> c_int {
    let prfconstant = krb5_buffer { len: 3, data: b"prf".as_ptr() as *mut u8 };
    rfc8009_calc_KDF_HMAC_SHA2(krb5, input_key, &prfconstant, octet_string, (*krb5).prf_len * 8, result, gfp)
}

/* Derive Ke. [rfc8009 sec 5] */
unsafe fn rfc8009_calc_Ke(krb5: *const krb5_enctype, base_key: *const krb5_buffer,
    usage_constant: *const krb5_buffer, result: *mut krb5_buffer, gfp: gfp_t) -> c_int {
    rfc8009_calc_KDF_HMAC_SHA2(krb5, base_key, usage_constant, &rfc8009_no_context, (*krb5).key_bytes * 8, result, gfp)
}

/* Derive Kc/Ki [rfc8009 sec 5] */
unsafe fn rfc8009_calc_Ki(krb5: *const krb5_enctype, base_key: *const krb5_buffer,
    usage_constant: *const krb5_buffer, result: *mut krb5_buffer, gfp: gfp_t) -> c_int {
    rfc8009_calc_KDF_HMAC_SHA2(krb5, base_key, usage_constant, &rfc8009_no_context, (*krb5).cksum_len * 8, result, gfp)
}

/* Apply encryption and checksumming functions to a message. */
unsafe fn rfc8009_encrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead,
    sg: *mut scatterlist, nr_sg: c_uint, sg_len: usize, data_offset: usize,
    data_len: usize, preconfounded: bool) -> ssize_t {
    let mut req: *mut aead_request; let mut bsg: [scatterlist; 2] = core::mem::zeroed();
    let mut ret: ssize_t; let mut done: ssize_t; let mut buffer: *mut c_void;
    let secure_offset = 0usize; let base_len = (*krb5).conf_len + data_len; let pad_len = 0usize;
    let secure_len = base_len + pad_len; let cksum_offset = secure_len;
    if WARN_ON(data_offset != (*krb5).conf_len) { return -EINVAL; }
    if WARN_ON(cksum_offset + (*krb5).cksum_len > sg_len) { return -EFAULT; }
    let bsize = krb5_aead_size(aead) + krb5_aead_ivsize(aead) * 2;
    buffer = kzalloc(bsize, GFP_NOFS); if buffer.is_null() { return -ENOMEM; }
    req = buffer as *mut aead_request;
    let iv = buffer.add(krb5_aead_size(aead)) as *mut u8;
    let ad = buffer.add(krb5_aead_size(aead) + krb5_aead_ivsize(aead)) as *mut u8;
    ret = -EFAULT;
    if !preconfounded { get_random_bytes(buffer, (*krb5).conf_len); done = sg_pcopy_from_buffer(sg, nr_sg, buffer, (*krb5).conf_len, secure_offset); if done != (*krb5).conf_len as ssize_t { kfree_sensitive(buffer); return ret; } }
    if pad_len != 0 { done = sg_zero_buffer(sg, nr_sg, pad_len, data_offset + data_len); if done != pad_len as ssize_t { kfree_sensitive(buffer); return ret; } }
    sg_init_table(bsg.as_mut_ptr(), 2); sg_set_buf(&mut bsg[0], ad as *mut c_void, krb5_aead_ivsize(aead)); sg_chain(bsg.as_mut_ptr(), 2, sg);
    aead_request_set_tfm(req, aead); aead_request_set_callback(req, 0, None, core::ptr::null_mut()); aead_request_set_ad(req, krb5_aead_ivsize(aead)); aead_request_set_crypt(req, bsg.as_mut_ptr(), bsg.as_mut_ptr(), secure_len, iv);
    ret = crypto_aead_encrypt(req); if ret >= 0 { ret = (secure_len + (*krb5).cksum_len) as ssize_t; }
    kfree_sensitive(buffer); ret
}

/* Apply decryption and checksumming functions to a message. */
unsafe fn rfc8009_decrypt(krb5: *const krb5_enctype, aead: *mut crypto_aead,
    sg: *mut scatterlist, nr_sg: c_uint, offset: *mut usize, len: *mut usize) -> c_int {
    if WARN_ON(*offset != 0) { return -EINVAL; }
    if *len < (*krb5).conf_len + (*krb5).cksum_len { return -EPROTO; }
    let bsize = krb5_aead_size(aead) + krb5_aead_ivsize(aead) * 2;
    let buffer = kzalloc(bsize, GFP_NOFS); if buffer.is_null() { return -ENOMEM; }
    let req = buffer as *mut aead_request; let iv = buffer.add(krb5_aead_size(aead)) as *mut u8; let ad = buffer.add(krb5_aead_size(aead) + krb5_aead_ivsize(aead)) as *mut u8;
    let mut bsg: [scatterlist; 2] = core::mem::zeroed(); sg_init_table(bsg.as_mut_ptr(), 2); sg_set_buf(&mut bsg[0], ad as *mut c_void, krb5_aead_ivsize(aead)); sg_chain(bsg.as_mut_ptr(), 2, sg);
    aead_request_set_tfm(req, aead); aead_request_set_callback(req, 0, None, core::ptr::null_mut()); aead_request_set_ad(req, krb5_aead_ivsize(aead)); aead_request_set_crypt(req, bsg.as_mut_ptr(), bsg.as_mut_ptr(), *len, iv);
    let ret = crypto_aead_decrypt(req); if ret >= 0 { *offset += (*krb5).conf_len; *len -= (*krb5).conf_len + (*krb5).cksum_len; }
    kfree_sensitive(buffer); if ret >= 0 { 0 } else { ret }
}

static rfc8009_crypto_profile: krb5_crypto_profile = krb5_crypto_profile {
    calc_PRF: rfc8009_calc_PRF, calc_Kc: rfc8009_calc_Ki, calc_Ke: rfc8009_calc_Ke, calc_Ki: rfc8009_calc_Ki,
    derive_encrypt_keys: authenc_derive_encrypt_keys, load_encrypt_keys: authenc_load_encrypt_keys,
    derive_checksum_key: rfc3961_derive_checksum_key, load_checksum_key: rfc3961_load_checksum_key,
    encrypt: rfc8009_encrypt, decrypt: rfc8009_decrypt, get_mic: rfc3961_get_mic, verify_mic: rfc3961_verify_mic,
};

const krb5_aes128_cts_hmac_sha256_128: krb5_enctype = krb5_enctype { etype: KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128, ctype: KRB5_CKSUMTYPE_HMAC_SHA256_128_AES128, name: c"aes128-cts-hmac-sha256-128", encrypt_name: c"authenc(hmac(sha256),cts(cbc(aes)))", cksum_name: c"hmac(sha256)", hash_name: c"sha256", derivation_enc: c"cts(cbc(aes))", key_bytes: 16, key_len: 16, Kc_len: 16, Ke_len: 16, Ki_len: 16, block_len: 16, conf_len: 16, cksum_len: 16, hash_len: 20, prf_len: 32, keyed_cksum: true, random_to_key: None, profile: &rfc8009_crypto_profile };
const krb5_aes256_cts_hmac_sha384_192: krb5_enctype = krb5_enctype { etype: KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192, ctype: KRB5_CKSUMTYPE_HMAC_SHA384_192_AES256, name: c"aes256-cts-hmac-sha384-192", encrypt_name: c"authenc(hmac(sha384),cts(cbc(aes)))", cksum_name: c"hmac(sha384)", hash_name: c"sha384", derivation_enc: c"cts(cbc(aes))", key_bytes: 32, key_len: 32, Kc_len: 24, Ke_len: 32, Ki_len: 24, block_len: 16, conf_len: 16, cksum_len: 24, hash_len: 20, prf_len: 48, keyed_cksum: true, random_to_key: None, profile: &rfc8009_crypto_profile };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
