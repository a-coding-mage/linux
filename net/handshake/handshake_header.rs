/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic netlink handshake service
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2023, Oracle and/or its affiliates.
 */

/* Per-net namespace context */
#[repr(C)]
pub struct handshake_net {
    pub hn_lock: spinlock_t, /* protects next 3 fields */
    pub hn_pending: ::kernel::ffi::c_int,
    pub hn_pending_max: ::kernel::ffi::c_int,
    pub hn_requests: list_head,

    pub hn_flags: ::kernel::ffi::c_ulong,
}

pub const HANDSHAKE_F_NET_DRAINING: hn_flags_bits = 0;
pub type hn_flags_bits = ::kernel::ffi::c_uint;

pub struct file;
pub struct handshake_proto;

/* One handshake request */
#[repr(C)]
pub struct handshake_req {
    pub hr_list: list_head,
    pub hr_rhash: rhash_head,
    pub hr_flags: ::kernel::ffi::c_ulong,
    pub hr_proto: *const handshake_proto,
    pub hr_file: *mut file,
    pub hr_sk: *mut sock,
    pub hr_odestruct: Option<unsafe extern "C" fn(sk: *mut sock)>,

    /* Always the last field */
    pub hr_priv: [::kernel::ffi::c_char; 0],
}

pub const HANDSHAKE_F_REQ_COMPLETED: hr_flags_bits = 0;
pub const HANDSHAKE_F_REQ_SESSION: hr_flags_bits = 1;
pub type hr_flags_bits = ::kernel::ffi::c_uint;

pub struct genl_info;

/* Invariants for all handshake requests for one transport layer
 * security protocol
 */
#[repr(C)]
pub struct handshake_proto {
    pub hp_handler_class: ::kernel::ffi::c_int,
    pub hp_privsize: usize,
    pub hp_flags: ::kernel::ffi::c_ulong,

    pub hp_accept: Option<unsafe extern "C" fn(
        req: *mut handshake_req,
        info: *mut genl_info,
        fd: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int>,
    pub hp_done: Option<unsafe extern "C" fn(
        req: *mut handshake_req,
        status: ::kernel::ffi::c_int,
        info: *mut genl_info,
    )>,
    pub hp_destroy: Option<unsafe extern "C" fn(req: *mut handshake_req)>,
}

pub const HANDSHAKE_F_PROTO_NOTIFY: hp_flags_bits = 0;
pub type hp_flags_bits = ::kernel::ffi::c_uint;

/* alert.c */
extern "C" {
    pub fn tls_alert_send(
        sock: *mut socket,
        level: u8,
        description: u8,
    ) -> ::kernel::ffi::c_int;
}

/* netlink.c */
extern "C" {
    pub fn handshake_genl_notify(
        net: *mut net,
        proto: *const handshake_proto,
        flags: gfp_t,
    ) -> ::kernel::ffi::c_int;
    pub fn handshake_genl_put(
        msg: *mut sk_buff,
        info: *mut genl_info,
    ) -> *mut nlmsghdr;
    pub fn handshake_pernet(net: *mut net) -> *mut handshake_net;
}

/* request.c */
extern "C" {
    pub fn handshake_req_alloc(
        proto: *const handshake_proto,
        flags: gfp_t,
    ) -> *mut handshake_req;
    pub fn handshake_req_hash_init() -> ::kernel::ffi::c_int;
    pub fn handshake_req_hash_destroy();
    pub fn handshake_req_private(req: *mut handshake_req) -> *mut ::kernel::ffi::c_void;
    pub fn handshake_req_hash_lookup(sk: *mut sock) -> *mut handshake_req;
    pub fn handshake_req_next(
        hn: *mut handshake_net,
        class: ::kernel::ffi::c_int,
    ) -> *mut handshake_req;
    pub fn handshake_req_submit(
        sock: *mut socket,
        req: *mut handshake_req,
        flags: gfp_t,
    ) -> ::kernel::ffi::c_int;
    pub fn handshake_complete(
        req: *mut handshake_req,
        status: ::kernel::ffi::c_int,
        info: *mut genl_info,
    );
    pub fn handshake_req_cancel(sk: *mut sock) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
