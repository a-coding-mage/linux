// SPDX-License-Identifier: GPL-2.0-or-later
/* Application-specific bits for GSSAPI-based RxRPC security
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: linux/net.h, linux/skbuff.h, linux/slab.h,
// linux/key-type.h, ar-internal.h and rxgk_common.h.

/*
 * Decode a default-style YFS ticket in a response and turn it into an
 * rxrpc-type key.
 */
pub unsafe fn rxgk_yfs_decode_ticket(
    conn: *mut rxrpc_connection,
    skb: *mut sk_buff,
    buffer: *mut core::ffi::c_void,
    ticket_len: u32,
    _key: *mut *mut key,
) -> i32 {
    let mut token: *mut rxrpc_key_token;
    let cred: *const cred = current_cred(); // TODO - use socket creds
    let mut key: *mut key;
    let pre_ticket_len: usize;
    let payload_len: usize;
    let klen: u32;
    let enctype: u32;
    let payload: *mut core::ffi::c_void;
    let ticket: *mut core::ffi::c_void;
    let mut t: *mut __be32;
    let mut p: *mut __be32;
    let mut q: *mut __be32;
    let mut tmp: *mut __be32;
    let mut ret: i32;

    _enter!("");

    if ticket_len < 10 * core::mem::size_of::<__be32>() as u32 {
        return rxrpc_abort_conn(conn, skb, RXGK_INCONSISTENCY, -EPROTO,
                                rxgk_abort_resp_short_yfs_tkt);
    }

    /* Get the session key length */
    tmp = buffer as *mut __be32;
    enctype = u32::from_be((*tmp.add(0)) as u32);
    klen = u32::from_be((*tmp.add(1)) as u32);

    if klen > ticket_len - 10 * core::mem::size_of::<__be32>() as u32 {
        return rxrpc_abort_conn(conn, skb, RXGK_INCONSISTENCY, -EPROTO,
                                rxgk_abort_resp_short_yfs_key);
    }

    pre_ticket_len = (5 + 14) * core::mem::size_of::<__be32>()
        + xdr_round_up(klen as usize)
        + core::mem::size_of::<__be32>();
    payload_len = pre_ticket_len + xdr_round_up(ticket_len as usize);

    payload = kzalloc(payload_len, GFP_NOFS);
    if payload.is_null() {
        return -ENOMEM;
    }

    /* Fill out the XDR form for a key payload that can be passed to add_key(). */
    ticket = (payload as *mut u8).add(pre_ticket_len) as *mut core::ffi::c_void;
    memcpy(ticket, buffer, ticket_len as usize);

    /* Fill out the form header. */
    p = payload as *mut __be32;
    *p.add(0) = htonl(0); /* Flags */
    *p.add(1) = htonl(1); /* len(cellname) */
    *p.add(2) = htonl(0x20000000); /* Cellname " " */
    *p.add(3) = htonl(1); /* #tokens */
    *p.add(4) = htonl((15 * core::mem::size_of::<__be32>()
        + xdr_round_up(klen as usize) + xdr_round_up(ticket_len as usize)) as u32);

    /* Now fill in the body. */
    t = (ticket as *mut u8).add(2 * core::mem::size_of::<__be32>()
        + xdr_round_up(klen as usize)) as *mut __be32;
    q = (payload as *mut u8).add(5 * core::mem::size_of::<__be32>()) as *mut __be32;
    *q.add(0) = htonl(RXRPC_SECURITY_YFS_RXGK);
    *q.add(1) = *t.add(1); /* begintime - msw */
    *q.add(2) = *t.add(2); /* - lsw */
    *q.add(3) = *t.add(5); /* endtime - msw */
    *q.add(4) = *t.add(6); /* - lsw */
    *q.add(5) = 0; /* level - msw */
    *q.add(6) = *t.add(0); /* - lsw */
    *q.add(7) = 0; /* lifetime - msw */
    *q.add(8) = *t.add(3); /* - lsw */
    *q.add(9) = 0; /* bytelife - msw */
    *q.add(10) = *t.add(4); /* - lsw */
    *q.add(11) = 0; /* enctype - msw */
    *q.add(12) = htonl(enctype); /* - lsw */
    *q.add(13) = htonl(klen); /* Key length */

    q = q.add(14);
    memcpy(q as *mut core::ffi::c_void,
           (ticket as *mut u8).add(2 * core::mem::size_of::<__be32>()) as *const core::ffi::c_void,
           klen as usize);
    q = q.add(xdr_round_up(klen as usize) / 4);
    *q = htonl(ticket_len);
    q = q.add(1);
    if WARN_ON!(q as usize != ticket as usize) {
        ret = -EIO;
        goto_error!(error);
    }

    q = q.add(xdr_round_up(ticket_len as usize) / 4);
    if WARN_ON!(q as usize - payload as usize != payload_len) {
        ret = -EIO;
        goto_error!(error);
    }

    key = key_alloc(&key_type_rxrpc, "x", GLOBAL_ROOT_UID, GLOBAL_ROOT_GID,
                    cred, KEY_USR_VIEW, KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut());
    if IS_ERR!(key) {
        _leave!(" = -ENOMEM [alloc %ld]", PTR_ERR!(key));
        ret = PTR_ERR!(key);
        goto_error!(error);
    }

    _debug!("key %d", key_serial(key));
    ret = key_instantiate_and_link(key, payload, payload_len, core::ptr::null_mut(), core::ptr::null_mut());
    if ret < 0 { goto_error!(error_key); }

    token = (*key).payload.data[0] as *mut rxrpc_key_token;
    (*token).no_leak_key = true;
    *_key = key;
    key = core::ptr::null_mut();
    ret = 0;
    goto_error!(error);

    // C labels error_key and error are represented by the cleanup dispatch below.
    #[allow(unreachable_code)]
    { key_put(key); }
    kfree_sensitive(payload);
    _leave!(" = %d", ret);
    ret
}

/* Extract the token and set up a session key from the details. */
pub unsafe fn rxgk_extract_token(
    conn: *mut rxrpc_connection, skb: *mut sk_buff, token: *mut core::ffi::c_void,
    token_len: u32, _key: *mut *mut key,
) -> i32 {
    let mut krb5: *const krb5_enctype;
    let mut server_secret: *const krb5_buffer;
    let mut token_enc: *mut crypto_aead = core::ptr::null_mut();
    let mut server_key: *mut key;
    let mut ticket_len: u32;
    let mut ticket: *mut core::ffi::c_void;
    let kvno: u32;
    let enctype: u32;
    let mut ret: i32;
    let mut ec: i32 = 0;

    #[repr(C)] struct TokenContainer { kvno: __be32, enctype: __be32, token_len: __be32 }
    if token_len < core::mem::size_of::<TokenContainer>() as u32 { goto_short_packet!(); }
    let container = token as *mut TokenContainer;
    let token = (token as *mut u8).add(core::mem::size_of::<TokenContainer>()) as *mut core::ffi::c_void;
    kvno = u32::from_be((*container).kvno as u32);
    enctype = u32::from_be((*container).enctype as u32);
    ticket_len = u32::from_be((*container).token_len as u32);
    if ticket_len > xdr_round_down(token_len as usize - core::mem::size_of::<TokenContainer>()) as u32 { goto_short_packet!(); }
    _debug!("KVNO %u", kvno); _debug!("ENC %u", enctype); _debug!("TLEN %u", ticket_len);
    server_key = rxrpc_look_up_server_security(conn, skb, kvno, enctype);
    if IS_ERR!(server_key) { return PTR_ERR!(server_key); }
    down_read(&mut (*server_key).sem);
    server_secret = &(*server_key).payload.data[2] as *const _ as *const krb5_buffer;
    ret = rxgk_set_up_token_cipher(server_secret, &mut token_enc, &mut krb5, enctype, GFP_NOFS);
    up_read(&mut (*server_key).sem); key_put(server_key);
    if ret < 0 { return ret; }
    ticket = token;
    ret = rxgk_decrypt(krb5, token_enc, &mut ticket, &mut ticket_len, &mut ec);
    crypto_free_aead(token_enc); token_enc = core::ptr::null_mut();
    if ret < 0 { if ret != -ENOMEM { return rxrpc_abort_conn(conn, skb, ec, ret, rxgk_abort_resp_tok_dec); } return ret; }
    ret = (*(*conn).security).default_decode_ticket(conn, skb, ticket, ticket_len, _key);
    if ret < 0 { return ret; }
    _leave!(" = 0"); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
