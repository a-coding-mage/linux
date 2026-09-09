/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Common bits for GSSAPI-based RxRPC security.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel/RxRPC translation. */

/*
 * Per-key number context.  This is replaced when the connection is rekeyed.
 */
#[repr(C)]
pub struct rxgk_context {
    pub usage: refcount_t,
    pub key_number: c_uint, /* Rekeying number (goes in the rx header) */
    pub flags: c_ulong,
    /* RXGK_TK_NEEDS_REKEY: Set if this needs rekeying */
    pub expiry: c_ulong, /* Expiration time of this key */
    pub bytes_remaining: c_longlong, /* Remaining Tx lifetime of this key */
    pub krb5: *const krb5_enctype, /* RxGK encryption type */
    pub key: *const rxgk_key,

    /* We need up to 7 keys derived from the transport key, but we don't
     * actually need the transport key.  Each key is derived by
     * DK(TK,constant).
     */
    pub tx_enc: *mut crypto_aead, /* Transmission key */
    pub rx_enc: *mut crypto_aead, /* Reception key */
    pub tx_Kc: *mut crypto_shash, /* Transmission checksum key */
    pub rx_Kc: *mut crypto_shash, /* Reception checksum key */
    pub resp_enc: *mut crypto_aead, /* Response packet enc key */
}

pub const RXGK_TK_NEEDS_REKEY: c_ulong = 0;

#[inline]
pub const fn xdr_round_up(x: usize) -> usize {
    (x + core::mem::size_of::<u32>() - 1) & !(core::mem::size_of::<u32>() - 1)
}

#[inline]
pub const fn xdr_round_down(x: usize) -> usize {
    x & !(core::mem::size_of::<u32>() - 1)
}

#[inline]
pub const fn xdr_object_len(x: usize) -> usize {
    4 + xdr_round_up(x)
}

extern "C" {
    pub fn rxgk_yfs_decode_ticket(
        conn: *mut rxrpc_connection,
        skb: *mut sk_buff,
        ticket: *mut c_void,
        ticket_len: c_uint,
        _key: *mut *mut key,
    ) -> c_int;
    pub fn rxgk_extract_token(
        conn: *mut rxrpc_connection,
        skb: *mut sk_buff,
        token: *mut c_void,
        token_len: c_uint,
        _key: *mut *mut key,
    ) -> c_int;

    pub fn rxgk_put(gk: *mut rxgk_context);
    pub fn rxgk_generate_transport_key(
        conn: *mut rxrpc_connection,
        key: *const rxgk_key,
        key_number: c_uint,
        gfp: gfp_t,
    ) -> *mut rxgk_context;
    pub fn rxgk_set_up_token_cipher(
        server_key: *const krb5_buffer,
        token_key: *mut *mut crypto_aead,
        enctype: c_uint,
        _krb5: *mut *const krb5_enctype,
        gfp: gfp_t,
    ) -> c_int;
}

/*
 * Apply decryption and checksumming functions a flat data buffer.  The data
 * point and length are updated to reflect the actual content of the encrypted
 * region.
 */
#[inline]
pub unsafe fn rxgk_decrypt(
    krb5: *const krb5_enctype,
    aead: *mut crypto_aead,
    data: *mut *mut c_void,
    len: *mut c_uint,
    error_code: *mut c_int,
) -> c_int {
    let mut sg: [scatterlist; 1] = core::mem::zeroed();
    let mut offset: usize = 0;
    let mut data_len = *len as usize;
    sg_init_one(sg.as_mut_ptr(), *data, data_len);
    let mut ret = crypto_krb5_decrypt(krb5, aead, sg.as_mut_ptr(), 1, &mut offset, &mut data_len);
    match ret {
        0 => {
            if offset & 3 != 0 {
                *error_code = RXGK_INCONSISTENCY;
                ret = -EPROTO;
            } else {
                *data = (*data as *mut u8).add(offset) as *mut c_void;
                *len = data_len as c_uint;
            }
        }
        EBADMSG | EPROTO => *error_code = RXGK_SEALEDINCON,
        EMSGSIZE => *error_code = RXGK_PACKETSHORT,
        _ => *error_code = RXGK_INCONSISTENCY,
    }
    ret
}

/* Check the MIC on a flat buffer. */
#[inline]
pub unsafe fn rxgk_verify_mic(
    krb5: *const krb5_enctype,
    shash: *mut crypto_shash,
    metadata: *const krb5_buffer,
    data: *mut *mut c_void,
    len: *mut c_uint,
    error_code: *mut u32,
) -> c_int {
    let mut sg: [scatterlist; 1] = core::mem::zeroed();
    let mut offset: usize = 0;
    let mut data_len = *len as usize;
    sg_init_one(sg.as_mut_ptr(), *data, data_len);
    let ret = crypto_krb5_verify_mic(krb5, shash, metadata, sg.as_mut_ptr(), 1, &mut offset, &mut data_len);
    match ret {
        0 => {
            *data = (*data as *mut u8).add(offset) as *mut c_void;
            *len = data_len as c_uint;
        }
        EBADMSG | EPROTO => *error_code = RXGK_SEALEDINCON as u32,
        EMSGSIZE => *error_code = RXGK_PACKETSHORT as u32,
        _ => *error_code = RXGK_INCONSISTENCY as u32,
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
