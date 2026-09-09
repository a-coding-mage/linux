// SPDX-License-Identifier: GPL-2.0-or-later
/* RxGK transport key derivation.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/RxRPC implementation are
// intentionally referenced but not reimplemented here.

const RXGK_CLIENT_ENC_PACKET: u32 = 1026; // 0x402
const RXGK_CLIENT_MIC_PACKET: u32 = 1027; // 0x403
const RXGK_SERVER_ENC_PACKET: u32 = 1028; // 0x404
const RXGK_SERVER_MIC_PACKET: u32 = 1029; // 0x405
const RXGK_CLIENT_ENC_RESPONSE: u32 = 1030; // 0x406
const RXGK_SERVER_ENC_TOKEN: u32 = 1036; // 0x40c

#[inline]
fn round16(x: usize) -> usize {
    (x + 15) & !15
}

unsafe fn rxgk_free(gk: *mut rxgk_context) {
    if !(*gk).tx_Kc.is_null() {
        crypto_free_shash((*gk).tx_Kc);
    }
    if !(*gk).rx_Kc.is_null() {
        crypto_free_shash((*gk).rx_Kc);
    }
    if !(*gk).tx_enc.is_null() {
        crypto_free_aead((*gk).tx_enc);
    }
    if !(*gk).rx_enc.is_null() {
        crypto_free_aead((*gk).rx_enc);
    }
    if !(*gk).resp_enc.is_null() {
        crypto_free_aead((*gk).resp_enc);
    }
    kfree(gk);
}

pub unsafe fn rxgk_put(gk: *mut rxgk_context) {
    if !gk.is_null() && refcount_dec_and_test(&mut (*gk).usage) {
        rxgk_free(gk);
    }
}

/*
 * Transport key derivation function.
 *
 *      TK = random-to-key(PRF+(K0, L,
 *                         epoch || cid || start_time || key_number))
 *      [tools.ietf.org/html/draft-wilkinson-afs3-rxgk-11 sec 8.3]
 */
unsafe fn rxgk_derive_transport_key(
    conn: *mut rxrpc_connection,
    gk: *mut rxgk_context,
    rxgk: *const rxgk_key,
    tk: *mut krb5_buffer,
    gfp: gfp_t,
) -> i32 {
    let krb5 = (*gk).krb5;
    let mut conn_info = krb5_buffer { len: core::mem::size_of::<u32>() * 5, data: core::ptr::null_mut() };
    let l = (*krb5).key_bytes;
    let buffer = kzalloc(round16(conn_info.len), gfp);
    if buffer.is_null() {
        return -ENOMEM;
    }
    conn_info.data = buffer;
    let info = buffer as *mut u32;
    *info.add(0) = htonl((*conn).proto.epoch);
    *info.add(1) = htonl((*conn).proto.cid);
    *info.add(2) = htonl(((*conn).rxgk.start_time >> 32) as u32);
    *info.add(3) = htonl((*conn).rxgk.start_time as u32);
    *info.add(4) = htonl((*gk).key_number);
    let ret = crypto_krb5_calc_PRFplus(krb5, &(*rxgk).key, l, &conn_info, tk, gfp);
    kfree_sensitive(buffer);
    ret
}

/* Set up the ciphers for the usage keys. */
unsafe fn rxgk_set_up_ciphers(
    conn: *mut rxrpc_connection,
    gk: *mut rxgk_context,
    rxgk: *const rxgk_key,
    gfp: gfp_t,
) -> i32 {
    let krb5 = (*gk).krb5;
    let mut tk = krb5_buffer { len: (*krb5).key_bytes, data: core::ptr::null_mut() };
    let buffer = kzalloc((*krb5).key_bytes, gfp);
    if buffer.is_null() { return -ENOMEM; }
    tk.data = buffer;
    let mut ret = rxgk_derive_transport_key(conn, gk, rxgk, &mut tk, gfp);
    if ret < 0 { kfree_sensitive(buffer); return ret; }
    let service = rxrpc_conn_is_service(conn);
    let mut aead = crypto_krb5_prepare_encryption(krb5, &tk, RXGK_CLIENT_ENC_RESPONSE, gfp);
    if IS_ERR(aead) { ret = PTR_ERR(aead); kfree_sensitive(buffer); return ret; }
    (*gk).resp_enc = aead;
    if crypto_aead_blocksize((*gk).resp_enc) != (*krb5).block_len || crypto_aead_authsize((*gk).resp_enc) != (*krb5).cksum_len {
        pr_notice("algo inconsistent with krb5 table %u!=%u or %u!=%u\n", crypto_aead_blocksize((*gk).resp_enc), (*krb5).block_len, crypto_aead_authsize((*gk).resp_enc), (*krb5).cksum_len);
        kfree_sensitive(buffer); return -EINVAL;
    }
    if service {
        match (*conn).security_level {
            RXRPC_SECURITY_AUTH => {
                let mut shash = crypto_krb5_prepare_checksum(krb5, &tk, RXGK_SERVER_MIC_PACKET, gfp);
                if IS_ERR(shash) { ret = PTR_ERR(shash); kfree_sensitive(buffer); return ret; }
                (*gk).tx_Kc = shash;
                shash = crypto_krb5_prepare_checksum(krb5, &tk, RXGK_CLIENT_MIC_PACKET, gfp);
                if IS_ERR(shash) { ret = PTR_ERR(shash); kfree_sensitive(buffer); return ret; }
                (*gk).rx_Kc = shash;
            }
            RXRPC_SECURITY_ENCRYPT => {
                aead = crypto_krb5_prepare_encryption(krb5, &tk, RXGK_SERVER_ENC_PACKET, gfp);
                if IS_ERR(aead) { ret = PTR_ERR(aead); kfree_sensitive(buffer); return ret; }
                (*gk).tx_enc = aead;
                aead = crypto_krb5_prepare_encryption(krb5, &tk, RXGK_CLIENT_ENC_PACKET, gfp);
                if IS_ERR(aead) { ret = PTR_ERR(aead); kfree_sensitive(buffer); return ret; }
                (*gk).rx_enc = aead;
            }
            _ => {}
        }
    } else {
        match (*conn).security_level {
            RXRPC_SECURITY_AUTH => {
                let mut shash = crypto_krb5_prepare_checksum(krb5, &tk, RXGK_CLIENT_MIC_PACKET, gfp);
                if IS_ERR(shash) { ret = PTR_ERR(shash); kfree_sensitive(buffer); return ret; }
                (*gk).tx_Kc = shash;
                shash = crypto_krb5_prepare_checksum(krb5, &tk, RXGK_SERVER_MIC_PACKET, gfp);
                if IS_ERR(shash) { ret = PTR_ERR(shash); kfree_sensitive(buffer); return ret; }
                (*gk).rx_Kc = shash;
            }
            RXRPC_SECURITY_ENCRYPT => {
                aead = crypto_krb5_prepare_encryption(krb5, &tk, RXGK_CLIENT_ENC_PACKET, gfp);
                if IS_ERR(aead) { ret = PTR_ERR(aead); kfree_sensitive(buffer); return ret; }
                (*gk).tx_enc = aead;
                aead = crypto_krb5_prepare_encryption(krb5, &tk, RXGK_SERVER_ENC_PACKET, gfp);
                if IS_ERR(aead) { ret = PTR_ERR(aead); kfree_sensitive(buffer); return ret; }
                (*gk).rx_enc = aead;
            }
            _ => {}
        }
    }
    kfree_sensitive(buffer);
    0
}

pub unsafe fn rxgk_generate_transport_key(conn: *mut rxrpc_connection, key: *const rxgk_key, key_number: u32, gfp: gfp_t) -> *mut rxgk_context {
    let gk = kzalloc_obj::<rxgk_context>();
    if gk.is_null() { return ERR_PTR(-ENOMEM); }
    refcount_set(&mut (*gk).usage, 1);
    (*gk).key = key;
    (*gk).key_number = key_number;
    (*gk).krb5 = crypto_krb5_find_enctype((*key).enctype);
    if (*gk).krb5.is_null() { rxgk_put(gk); return ERR_PTR(-ENOPKG); }
    let ret = rxgk_set_up_ciphers(conn, gk, key, gfp);
    if ret != 0 { rxgk_put(gk); return ERR_PTR(ret); }
    (*gk).bytes_remaining = match (*key).bytelife {
        0 | 63 => LLONG_MAX,
        1..=62 => 1i64 << (*key).bytelife,
        n => n as i64,
    };
    let lifetime = if (*key).lifetime != 0 {
        core::cmp::min((*key).lifetime, INT_MAX as u64 / HZ as u64) * HZ as u64
    } else { MAX_JIFFY_OFFSET };
    (*gk).expiry = jiffies + lifetime;
    gk
}

pub unsafe fn rxgk_set_up_token_cipher(server_key: *const krb5_buffer, token_aead: *mut *mut crypto_aead, enctype: u32, krb5_out: *mut *const krb5_enctype, gfp: gfp_t) -> i32 {
    let krb5 = crypto_krb5_find_enctype(enctype);
    if krb5.is_null() { return -ENOPKG; }
    let aead = crypto_krb5_prepare_encryption(krb5, server_key, RXGK_SERVER_ENC_TOKEN, gfp);
    if IS_ERR(aead) { return PTR_ERR(aead); }
    *krb5_out = krb5;
    *token_aead = aead;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
