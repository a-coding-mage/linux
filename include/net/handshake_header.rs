/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic netlink HANDSHAKE service.
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2023, Oracle and/or its affiliates.
 */

// C header guard: _NET_HANDSHAKE_H

pub const TLS_NO_KEYRING: i32 = 0;
pub const TLS_NO_PEERID: i32 = 0;
pub const TLS_NO_CERT: i32 = 0;
pub const TLS_NO_PRIVKEY: i32 = 0;

pub type tls_done_func_t = Option<unsafe extern "C" fn(data: *mut core::ffi::c_void,
                                                        status: core::ffi::c_int,
                                                        peerid: key_serial_t)>;

#[repr(C)]
pub struct tls_handshake_args {
    pub ta_sock: *mut socket,
    pub ta_done: tls_done_func_t,
    pub ta_data: *mut core::ffi::c_void,
    pub ta_peername: *const core::ffi::c_char,
    pub ta_timeout_ms: core::ffi::c_uint,
    pub ta_keyring: key_serial_t,
    pub ta_my_cert: key_serial_t,
    pub ta_my_privkey: key_serial_t,
    pub ta_num_peerids: core::ffi::c_uint,
    pub ta_my_peerids: [key_serial_t; 5],
}

unsafe extern "C" {
    pub fn tls_client_hello_anon(args: *const tls_handshake_args, flags: gfp_t) -> core::ffi::c_int;
    pub fn tls_client_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> core::ffi::c_int;
    pub fn tls_client_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> core::ffi::c_int;
    pub fn tls_server_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> core::ffi::c_int;
    pub fn tls_server_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> core::ffi::c_int;

    pub fn tls_handshake_cancel(sk: *mut sock) -> bool;
    pub fn tls_handshake_close(sock: *mut socket);

    pub fn tls_get_record_type(sk: *const sock, msg: *const cmsghdr) -> u8;
    pub fn tls_alert_recv(
        sk: *const sock,
        msg: *const msghdr,
        level: *mut u8,
        description: *mut u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
