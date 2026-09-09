/* SPDX-License-Identifier: GPL-2.0-or-later */
/* RxRPC kernel service interface definitions
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct key {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rxrpc_call {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rxrpc_peer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct krb5_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub enum rxrpc_interruptibility {
    RXRPC_INTERRUPTIBLE,     /* Call is interruptible */
    RXRPC_PREINTERRUPTIBLE,  /* Call can be cancelled whilst waiting for a slot */
    RXRPC_UNINTERRUPTIBLE,   /* Call should not be interruptible at all */
}

#[repr(C)]
pub enum rxrpc_oob_type {
    RXRPC_OOB_CHALLENGE,     /* Security challenge for a connection */
}

extern "C" {
    // Debug ID counter for tracing.
    pub static mut rxrpc_debug_id: atomic_t;
}

#[repr(C)]
pub struct rxrpc_kernel_ops {
    pub notify_new_call: Option<unsafe extern "C" fn(
        sk: *mut sock,
        call: *mut rxrpc_call,
        user_call_ID: ::core::ffi::c_ulong,
    )>,
    pub discard_new_call: Option<unsafe extern "C" fn(
        call: *mut rxrpc_call,
        user_call_ID: ::core::ffi::c_ulong,
    )>,
    pub user_attach_call: Option<unsafe extern "C" fn(
        call: *mut rxrpc_call,
        user_call_ID: ::core::ffi::c_ulong,
    )>,
    pub notify_oob: Option<unsafe extern "C" fn(
        sk: *mut sock,
        oob: *mut sk_buff,
    )>,
}

pub type rxrpc_notify_rx_t = unsafe extern "C" fn(
    *mut sock,
    *mut rxrpc_call,
    ::core::ffi::c_ulong,
);
pub type rxrpc_notify_end_tx_t = unsafe extern "C" fn(
    *mut sock,
    *mut rxrpc_call,
    ::core::ffi::c_ulong,
);

extern "C" {
    pub fn rxrpc_kernel_set_notifications(sock: *mut socket, app_ops: *const rxrpc_kernel_ops);
    pub fn rxrpc_kernel_begin_call(
        sock: *mut socket,
        peer: *mut rxrpc_peer,
        key: *mut key,
        user_call_ID: ::core::ffi::c_ulong,
        tx_total_len: s64,
        hard_timeout: u32,
        gfp: gfp_t,
        notify_rx: rxrpc_notify_rx_t,
        service_id: u16,
        upgrade: bool,
        interruptibility: rxrpc_interruptibility,
        debug_id: ::core::ffi::c_uint,
    ) -> *mut rxrpc_call;
    pub fn rxrpc_kernel_send_data(
        sock: *mut socket,
        call: *mut rxrpc_call,
        msg: *mut msghdr,
        len: usize,
        notify_end_tx: rxrpc_notify_end_tx_t,
    ) -> ::core::ffi::c_int;
    pub fn rxrpc_kernel_recv_data(
        sock: *mut socket,
        call: *mut rxrpc_call,
        iter: *mut iov_iter,
        len: *mut usize,
        want_more: bool,
        abort_code: *mut u32,
        service_id: *mut u16,
    ) -> ::core::ffi::c_int;
    pub fn rxrpc_kernel_abort_call(
        sock: *mut socket,
        call: *mut rxrpc_call,
        abort_code: u32,
        error: ::core::ffi::c_int,
        why: rxrpc_abort_reason,
    ) -> bool;
    pub fn rxrpc_kernel_shutdown_call(sock: *mut socket, call: *mut rxrpc_call);
    pub fn rxrpc_kernel_put_call(sock: *mut socket, call: *mut rxrpc_call);
    pub fn rxrpc_kernel_lookup_peer(sock: *mut socket, srx: *mut sockaddr_rxrpc, gfp: gfp_t) -> *mut rxrpc_peer;
    pub fn rxrpc_kernel_put_peer(peer: *mut rxrpc_peer);
    pub fn rxrpc_kernel_get_peer(peer: *mut rxrpc_peer) -> *mut rxrpc_peer;
    pub fn rxrpc_kernel_get_call_peer(sock: *mut socket, call: *mut rxrpc_call) -> *mut rxrpc_peer;
    pub fn rxrpc_kernel_remote_srx(peer: *const rxrpc_peer) -> *const sockaddr_rxrpc;
    pub fn rxrpc_kernel_remote_addr(peer: *const rxrpc_peer) -> *const sockaddr;
    pub fn rxrpc_kernel_set_peer_data(peer: *mut rxrpc_peer, app_data: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn rxrpc_kernel_get_peer_data(peer: *const rxrpc_peer) -> ::core::ffi::c_ulong;
    pub fn rxrpc_kernel_get_srtt(peer: *const rxrpc_peer) -> ::core::ffi::c_uint;
    pub fn rxrpc_kernel_charge_accept(sock: *mut socket, notify_rx: rxrpc_notify_rx_t, user_call_ID: ::core::ffi::c_ulong, gfp: gfp_t, debug_id: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rxrpc_kernel_set_tx_length(sock: *mut socket, call: *mut rxrpc_call, len: s64);
    pub fn rxrpc_kernel_check_life(sock: *const socket, call: *const rxrpc_call) -> bool;
    pub fn rxrpc_sock_set_min_security_level(sk: *mut sock, val: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rxrpc_sock_set_security_keyring(sk: *mut sock, key: *mut key) -> ::core::ffi::c_int;
    pub fn rxrpc_sock_set_manage_response(sk: *mut sock, set: bool) -> ::core::ffi::c_int;
    pub fn rxrpc_kernel_query_oob(oob: *mut sk_buff, peer: *mut *mut rxrpc_peer, peer_appdata: *mut ::core::ffi::c_ulong) -> rxrpc_oob_type;
    pub fn rxrpc_kernel_dequeue_oob(sock: *mut socket, type_: *mut rxrpc_oob_type) -> *mut sk_buff;
    pub fn rxrpc_kernel_free_oob(oob: *mut sk_buff);
    pub fn rxrpc_kernel_query_challenge(challenge: *mut sk_buff, peer: *mut *mut rxrpc_peer, peer_appdata: *mut ::core::ffi::c_ulong, service_id: *mut u16, security_index: *mut u8);
    pub fn rxrpc_kernel_reject_challenge(challenge: *mut sk_buff, abort_code: u32, error: ::core::ffi::c_int, why: rxrpc_abort_reason) -> ::core::ffi::c_int;
    pub fn rxkad_kernel_respond_to_challenge(challenge: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn rxgk_kernel_query_challenge(challenge: *mut sk_buff) -> u32;
    pub fn rxgk_kernel_respond_to_challenge(challenge: *mut sk_buff, appdata: *mut krb5_buffer) -> ::core::ffi::c_int;
    pub fn rxrpc_kernel_query_call_security(call: *mut rxrpc_call, service_id: *mut u16, enctype: *mut u32) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
