// SPDX-License-Identifier: GPL-2.0-or-later
/* rfc6803 Camellia Encryption for Kerberos 5
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #include <linux/slab.h>
// #include "internal.h"

/*
 * Calculate the key derivation function KDF-FEEDBACK_CMAC(key, constant)
 *
 *     n = ceiling(k / 128)
 *     K(0) = zeros
 *     K(i) = CMAC(key, K(i-1) | i | constant | 0x00 | k)
 *     DR(key, constant) = k-truncate(K(1) | K(2) | ... | K(n))
 *     KDF-FEEDBACK-CMAC(key, constant) = random-to-key(DR(key, constant))
 *
 *     [rfc6803 sec 3]
 */
unsafe fn rfc6803_calc_KDF_FEEDBACK_CMAC(
    krb5: *const krb5_enctype,
    key: *const krb5_buffer,
    constant: *const krb5_buffer,
    result: *mut krb5_buffer,
    gfp: gfp_t,
) -> c_int {
    let mut shash: *mut crypto_shash;
    let mut k: krb5_buffer = core::mem::zeroed();
    let mut data: krb5_buffer = core::mem::zeroed();
    let mut desc: *mut shash_desc;
    let mut tmp: __be32;
    let (mut bsize, mut offset, mut seg): (usize, usize, usize);
    let mut buffer: *mut c_void;
    let (mut i, mut kval): (u32, u32) = (0, (*result).len * 8);
    let mut p: *mut u8;
    let mut ret: c_int = -ENOMEM;

    shash = crypto_alloc_shash((*krb5).cksum_name, 0, 0);
    if IS_ERR(shash) {
        return if PTR_ERR(shash) == -ENOENT { -ENOPKG } else { PTR_ERR(shash) };
    }
    ret = crypto_shash_setkey(shash, (*key).data, (*key).len);
    if ret < 0 { goto_error_shash!(); }

    ret = -ENOMEM;
    k.len = crypto_shash_digestsize(shash);
    data.len = k.len + 4 + (*constant).len + 1 + 4;
    bsize = krb5_shash_size(shash) + krb5_digest_size(shash)
        + crypto_roundup(k.len) + crypto_roundup(data.len);
    buffer = kzalloc(bsize, GFP_NOFS);
    if buffer.is_null() { goto_error_shash!(); }

    desc = buffer as *mut shash_desc;
    (*desc).tfm = shash;
    k.data = (buffer as *mut u8).add(krb5_shash_size(shash) + krb5_digest_size(shash)) as *mut c_void;
    data.data = (buffer as *mut u8).add(krb5_shash_size(shash) + krb5_digest_size(shash) + crypto_roundup(k.len)) as *mut c_void;

    p = (data.data as *mut u8).add(k.len + 4);
    memcpy(p as *mut c_void, (*constant).data, (*constant).len);
    p = p.add((*constant).len);
    *p = 0; p = p.add(1);
    tmp = htonl(kval); memcpy(p as *mut c_void, &tmp as *const _ as *const c_void, 4); p = p.add(4);

    ret = -EINVAL;
    if WARN_ON(p.offset_from(data.data as *mut u8) as usize != data.len) { goto_error!(); }
    offset = 0;
    loop {
        i += 1;
        p = data.data as *mut u8;
        memcpy(p as *mut c_void, k.data, k.len); p = p.add(k.len);
        *(p as *mut __be32) = htonl(i);
        ret = crypto_shash_init(desc); if ret < 0 { goto_error!(); }
        ret = crypto_shash_finup(desc, data.data, data.len, k.data); if ret < 0 { goto_error!(); }
        seg = core::cmp::min((*result).len - offset, k.len);
        memcpy((*result).data.add(offset), k.data, seg);
        offset += seg;
        if offset >= (*result).len { break; }
    }

    ret = 0;
goto_error:
    kfree_sensitive(buffer);
goto_error_shash:
    crypto_free_shash(shash);
    ret
}

unsafe fn rfc6803_calc_PRF(
    krb5: *const krb5_enctype, protocol_key: *const krb5_buffer,
    octet_string: *const krb5_buffer, result: *mut krb5_buffer, gfp: gfp_t,
) -> c_int {
    static mut PRFCONSTANT: krb5_buffer = krb5_buffer { len: 3, data: b"prf\0" as *const _ as *mut c_void };
    let mut shash: *mut crypto_shash;
    let mut kp: krb5_buffer = core::mem::zeroed();
    let mut desc: *mut shash_desc;
    let bsize: usize;
    let mut buffer: *mut c_void;
    let mut ret: c_int;
    kp.len = (*krb5).prf_len;
    shash = crypto_alloc_shash((*krb5).cksum_name, 0, 0);
    if IS_ERR(shash) { return if PTR_ERR(shash) == -ENOENT { -ENOPKG } else { PTR_ERR(shash) }; }
    ret = -EINVAL;
    if (*result).len != crypto_shash_digestsize(shash) { goto_out_shash!(); }
    ret = -ENOMEM;
    bsize = krb5_shash_size(shash) + krb5_digest_size(shash) + crypto_roundup(kp.len);
    buffer = kzalloc(bsize, GFP_NOFS); if buffer.is_null() { goto_out_shash!(); }
    kp.data = (buffer as *mut u8).add(krb5_shash_size(shash) + krb5_digest_size(shash)) as *mut c_void;
    ret = rfc6803_calc_KDF_FEEDBACK_CMAC(krb5, protocol_key, &raw const PRFCONSTANT, &mut kp, gfp);
    if ret < 0 { goto_out!(); }
    ret = crypto_shash_setkey(shash, kp.data, kp.len); if ret < 0 { goto_out!(); }
    desc = buffer as *mut shash_desc; (*desc).tfm = shash;
    ret = crypto_shash_init(desc); if ret < 0 { goto_out!(); }
    ret = crypto_shash_finup(desc, (*octet_string).data, (*octet_string).len, (*result).data);
goto_out:
    kfree_sensitive(buffer);
goto_out_shash:
    crypto_free_shash(shash);
    ret
}

// The profile and enctype objects retain the C layout and external callbacks.
static rfc6803_crypto_profile: krb5_crypto_profile = krb5_crypto_profile {
    calc_PRF: Some(rfc6803_calc_PRF), calc_Kc: Some(rfc6803_calc_KDF_FEEDBACK_CMAC),
    calc_Ke: Some(rfc6803_calc_KDF_FEEDBACK_CMAC), calc_Ki: Some(rfc6803_calc_KDF_FEEDBACK_CMAC),
    derive_encrypt_keys: Some(authenc_derive_encrypt_keys), load_encrypt_keys: Some(authenc_load_encrypt_keys),
    derive_checksum_key: Some(rfc3961_derive_checksum_key), load_checksum_key: Some(rfc3961_load_checksum_key),
    encrypt: Some(krb5_aead_encrypt), decrypt: Some(krb5_aead_decrypt),
    get_mic: Some(rfc3961_get_mic), verify_mic: Some(rfc3961_verify_mic),
};

const krb5_camellia128_cts_cmac: krb5_enctype = krb5_enctype {
    etype: KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC, ctype: KRB5_CKSUMTYPE_CMAC_CAMELLIA128,
    name: b"camellia128-cts-cmac\0".as_ptr() as *const _, encrypt_name: b"krb5enc(cmac(camellia),cts(cbc(camellia)))\0".as_ptr() as *const _,
    cksum_name: b"cmac(camellia)\0".as_ptr() as *const _, hash_name: core::ptr::null(), derivation_enc: b"cts(cbc(camellia))\0".as_ptr() as *const _,
    key_bytes: 16, key_len: 16, Kc_len: 16, Ke_len: 16, Ki_len: 16, block_len: 16, conf_len: 16, cksum_len: 16, hash_len: 16, prf_len: 16,
    keyed_cksum: true, random_to_key: core::ptr::null(), profile: &rfc6803_crypto_profile,
};

const krb5_camellia256_cts_cmac: krb5_enctype = krb5_enctype {
    etype: KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC, ctype: KRB5_CKSUMTYPE_CMAC_CAMELLIA256,
    name: b"camellia256-cts-cmac\0".as_ptr() as *const _, encrypt_name: b"krb5enc(cmac(camellia),cts(cbc(camellia)))\0".as_ptr() as *const _,
    cksum_name: b"cmac(camellia)\0".as_ptr() as *const _, hash_name: core::ptr::null(), derivation_enc: b"cts(cbc(camellia))\0".as_ptr() as *const _,
    key_bytes: 32, key_len: 32, Kc_len: 32, Ke_len: 32, Ki_len: 32, block_len: 16, conf_len: 16, cksum_len: 16, hash_len: 16, prf_len: 16,
    keyed_cksum: true, random_to_key: core::ptr::null(), profile: &rfc6803_crypto_profile,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
