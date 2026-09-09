// SPDX-License-Identifier: GPL-2.0-or-later
/* Null security operations.
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding RxRPC implementation.

unsafe fn none_init_connection_security(
    _conn: *mut rxrpc_connection,
    _token: *mut rxrpc_key_token,
) -> i32 {
    0
}

/*
 * Allocate an appropriately sized buffer for the amount of data remaining.
 */
unsafe fn none_alloc_txbuf(
    call: *mut rxrpc_call,
    remain: usize,
    gfp: gfp_t,
) -> *mut rxrpc_txbuf {
    rxrpc_alloc_data_txbuf(call, umin(remain, RXRPC_JUMBO_DATALEN), 1, gfp)
}

unsafe fn none_secure_packet(_call: *mut rxrpc_call, txb: *mut rxrpc_txbuf) -> i32 {
    (*txb).pkt_len = (*txb).len;
    if (*txb).len == RXRPC_JUMBO_DATALEN {
        (*txb).jumboable = true;
    }
    0
}

unsafe fn none_verify_packet(_call: *mut rxrpc_call, _skb: *mut sk_buff) -> i32 {
    0
}

unsafe fn none_free_call_crypto(_call: *mut rxrpc_call) {}

unsafe fn none_validate_challenge(
    conn: *mut rxrpc_connection,
    skb: *mut sk_buff,
) -> bool {
    rxrpc_abort_conn(
        conn,
        skb,
        RX_PROTOCOL_ERROR,
        -EPROTO,
        rxrpc_eproto_rxnull_challenge,
    );
    true
}

unsafe fn none_sendmsg_respond_to_challenge(
    _challenge: *mut sk_buff,
    _msg: *mut msghdr,
) -> i32 {
    -EINVAL
}

unsafe fn none_verify_response(
    conn: *mut rxrpc_connection,
    response_skb: *mut sk_buff,
    _response: *mut core::ffi::c_void,
    _len: u32,
) -> i32 {
    rxrpc_abort_conn(
        conn,
        response_skb,
        RX_PROTOCOL_ERROR,
        -EPROTO,
        rxrpc_eproto_rxnull_response,
    )
}

unsafe fn none_clear(_conn: *mut rxrpc_connection) {}

unsafe fn none_init() -> i32 {
    0
}

unsafe fn none_exit() {}

/*
 * RxRPC Kerberos-based security
 */
#[no_mangle]
pub static rxrpc_no_security: rxrpc_security = rxrpc_security {
    name: c"none".as_ptr(),
    security_index: RXRPC_SECURITY_NONE,
    init: Some(none_init),
    exit: Some(none_exit),
    init_connection_security: Some(none_init_connection_security),
    free_call_crypto: Some(none_free_call_crypto),
    alloc_txbuf: Some(none_alloc_txbuf),
    secure_packet: Some(none_secure_packet),
    verify_packet: Some(none_verify_packet),
    validate_challenge: Some(none_validate_challenge),
    sendmsg_respond_to_challenge: Some(none_sendmsg_respond_to_challenge),
    verify_response: Some(none_verify_response),
    clear: Some(none_clear),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
