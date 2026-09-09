// SPDX-License-Identifier: GPL-2.0-only
/*
 * Establish a TLS session for a kernel socket consumer
 * using the tlshd user space handler.
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2021-2023, Oracle and/or its affiliates.
 */

// Kernel and handshake dependencies are supplied by the surrounding crate.

#[repr(C)]
struct TlsHandshakeReq {
    th_consumer_done: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, key_serial_t)>,
    th_consumer_data: *mut core::ffi::c_void,
    th_type: i32,
    th_timeout_ms: u32,
    th_auth_mode: i32,
    th_peername: *const core::ffi::c_char,
    th_keyring: key_serial_t,
    th_certificate: key_serial_t,
    th_privkey: key_serial_t,
    th_num_peerids: u32,
    th_peerid: [key_serial_t; 5],
}

unsafe fn tls_handshake_req_init(
    req: *mut handshake_req,
    args: *const tls_handshake_args,
) -> *mut TlsHandshakeReq {
    let treq = handshake_req_private(req) as *mut TlsHandshakeReq;
    (*treq).th_timeout_ms = (*args).ta_timeout_ms;
    (*treq).th_consumer_done = (*args).ta_done;
    (*treq).th_consumer_data = (*args).ta_data;
    (*treq).th_peername = (*args).ta_peername;
    (*treq).th_keyring = (*args).ta_keyring;
    (*treq).th_num_peerids = 0;
    (*treq).th_certificate = TLS_NO_CERT;
    (*treq).th_privkey = TLS_NO_PRIVKEY;
    treq
}

unsafe fn tls_handshake_remote_peerids(treq: *mut TlsHandshakeReq, info: *mut genl_info) {
    let head = nlmsg_attrdata((*info).nlhdr, GENL_HDRLEN);
    let len = nlmsg_attrlen((*info).nlhdr, GENL_HDRLEN);
    let mut rem: i32 = 0;
    let mut i: u32 = 0;
    let mut nla = head;
    while nla_for_each_attr(&mut nla, head, len, &mut rem) {
        if nla_type(nla) == HANDSHAKE_A_DONE_REMOTE_AUTH { i += 1; }
    }
    if i == 0 { return; }
    (*treq).th_num_peerids = core::cmp::min(i, (*treq).th_peerid.len() as u32);
    i = 0;
    nla = head;
    while nla_for_each_attr(&mut nla, head, len, &mut rem) {
        if nla_type(nla) == HANDSHAKE_A_DONE_REMOTE_AUTH {
            (*treq).th_peerid[i as usize] = nla_get_u32(nla);
            i += 1;
        }
        if i >= (*treq).th_num_peerids { break; }
    }
}

unsafe extern "C" fn tls_handshake_done(req: *mut handshake_req, status: i32, info: *mut genl_info) {
    let treq = handshake_req_private(req) as *mut TlsHandshakeReq;
    (*treq).th_peerid[0] = TLS_NO_PEERID;
    if !info.is_null() { tls_handshake_remote_peerids(treq, info); }
    if status == 0 { set_bit(HANDSHAKE_F_REQ_SESSION, &mut (*req).hr_flags); }
    ((*treq).th_consumer_done.unwrap())((*treq).th_consumer_data, status, (*treq).th_peerid[0]);
}

#[cfg(feature = "CONFIG_KEYS")]
unsafe fn tls_handshake_private_keyring(treq: *mut TlsHandshakeReq) -> i32 {
    if (*treq).th_keyring == TLS_NO_KEYRING { return 0; }
    let process_keyring_ref = lookup_user_key(KEY_SPEC_PROCESS_KEYRING, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
    if IS_ERR(process_keyring_ref) { return PTR_ERR(process_keyring_ref); }
    let keyring_ref = lookup_user_key((*treq).th_keyring, KEY_LOOKUP_CREATE, KEY_NEED_LINK);
    if IS_ERR(keyring_ref) { key_ref_put(process_keyring_ref); return PTR_ERR(keyring_ref); }
    let ret = key_link(key_ref_to_ptr(process_keyring_ref), key_ref_to_ptr(keyring_ref));
    key_ref_put(keyring_ref);
    key_ref_put(process_keyring_ref);
    ret
}

#[cfg(not(feature = "CONFIG_KEYS"))]
unsafe fn tls_handshake_private_keyring(_treq: *mut TlsHandshakeReq) -> i32 { 0 }

unsafe fn tls_handshake_put_peer_identity(msg: *mut sk_buff, treq: *mut TlsHandshakeReq) -> i32 {
    for i in 0..(*treq).th_num_peerids {
        if nla_put_u32(msg, HANDSHAKE_A_ACCEPT_PEER_IDENTITY, (*treq).th_peerid[i as usize]) < 0 { return -EMSGSIZE; }
    }
    0
}

unsafe fn tls_handshake_put_certificate(msg: *mut sk_buff, treq: *mut TlsHandshakeReq) -> i32 {
    if (*treq).th_certificate == TLS_NO_CERT && (*treq).th_privkey == TLS_NO_PRIVKEY { return 0; }
    let entry_attr = nla_nest_start(msg, HANDSHAKE_A_ACCEPT_CERTIFICATE);
    if entry_attr.is_null() { return -EMSGSIZE; }
    if nla_put_s32(msg, HANDSHAKE_A_X509_CERT, (*treq).th_certificate) != 0 ||
       nla_put_s32(msg, HANDSHAKE_A_X509_PRIVKEY, (*treq).th_privkey) != 0 {
        nla_nest_cancel(msg, entry_attr); return -EMSGSIZE;
    }
    nla_nest_end(msg, entry_attr); 0
}

unsafe extern "C" fn tls_handshake_accept(req: *mut handshake_req, info: *mut genl_info, fd: i32) -> i32 {
    let treq = handshake_req_private(req) as *mut TlsHandshakeReq;
    let mut ret = tls_handshake_private_keyring(treq);
    if ret < 0 { return ret; }
    let msg = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if msg.is_null() { return -ENOMEM; }
    let hdr = handshake_genl_put(msg, info);
    if hdr.is_null() { nlmsg_free(msg); return ret; }
    ret = nla_put_s32(msg, HANDSHAKE_A_ACCEPT_SOCKFD, fd);
    if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_MESSAGE_TYPE, (*treq).th_type);
    if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    if (*treq).th_peername != core::ptr::null() {
        ret = nla_put_string(msg, HANDSHAKE_A_ACCEPT_PEERNAME, (*treq).th_peername);
        if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    }
    if (*treq).th_timeout_ms != 0 { ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_TIMEOUT, (*treq).th_timeout_ms); if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; } }
    if (*treq).th_keyring != 0 { ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_KEYRING, (*treq).th_keyring); if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; } }
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_AUTH_MODE, (*treq).th_auth_mode);
    if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    ret = match (*treq).th_auth_mode {
        HANDSHAKE_AUTH_PSK => tls_handshake_put_peer_identity(msg, treq),
        HANDSHAKE_AUTH_X509 => tls_handshake_put_certificate(msg, treq),
        _ => 0,
    };
    if ret < 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    genlmsg_end(msg, hdr); genlmsg_reply(msg, info)
}

static TLS_HANDSHAKE_PROTO: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD,
    hp_privsize: core::mem::size_of::<TlsHandshakeReq>(),
    hp_flags: BIT(HANDSHAKE_F_PROTO_NOTIFY),
    hp_accept: Some(tls_handshake_accept),
    hp_done: Some(tls_handshake_done),
};

pub unsafe fn tls_client_hello_anon(args: *const tls_handshake_args, flags: gfp_t) -> i32 {
    let req = handshake_req_alloc(&TLS_HANDSHAKE_PROTO, flags); if req.is_null() { return -ENOMEM; }
    let treq = tls_handshake_req_init(req, args); (*treq).th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO; (*treq).th_auth_mode = HANDSHAKE_AUTH_UNAUTH;
    handshake_req_submit((*args).ta_sock, req, flags)
}

pub unsafe fn tls_client_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> i32 {
    let req = handshake_req_alloc(&TLS_HANDSHAKE_PROTO, flags); if req.is_null() { return -ENOMEM; }
    let treq = tls_handshake_req_init(req, args); (*treq).th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO; (*treq).th_auth_mode = HANDSHAKE_AUTH_X509; (*treq).th_certificate = (*args).ta_my_cert; (*treq).th_privkey = (*args).ta_my_privkey;
    handshake_req_submit((*args).ta_sock, req, flags)
}

pub unsafe fn tls_client_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> i32 {
    if (*args).ta_num_peerids == 0 || (*args).ta_num_peerids > 5 { return -EINVAL; }
    let req = handshake_req_alloc(&TLS_HANDSHAKE_PROTO, flags); if req.is_null() { return -ENOMEM; }
    let treq = tls_handshake_req_init(req, args); (*treq).th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO; (*treq).th_auth_mode = HANDSHAKE_AUTH_PSK; (*treq).th_num_peerids = (*args).ta_num_peerids;
    for i in 0..(*args).ta_num_peerids { (*treq).th_peerid[i as usize] = (*args).ta_my_peerids[i as usize]; }
    handshake_req_submit((*args).ta_sock, req, flags)
}

pub unsafe fn tls_server_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> i32 {
    let req = handshake_req_alloc(&TLS_HANDSHAKE_PROTO, flags); if req.is_null() { return -ENOMEM; }
    let treq = tls_handshake_req_init(req, args); (*treq).th_type = HANDSHAKE_MSG_TYPE_SERVERHELLO; (*treq).th_auth_mode = HANDSHAKE_AUTH_X509; (*treq).th_certificate = (*args).ta_my_cert; (*treq).th_privkey = (*args).ta_my_privkey;
    handshake_req_submit((*args).ta_sock, req, flags)
}

pub unsafe fn tls_server_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> i32 {
    let req = handshake_req_alloc(&TLS_HANDSHAKE_PROTO, flags); if req.is_null() { return -ENOMEM; }
    let treq = tls_handshake_req_init(req, args); (*treq).th_type = HANDSHAKE_MSG_TYPE_SERVERHELLO; (*treq).th_auth_mode = HANDSHAKE_AUTH_PSK; (*treq).th_num_peerids = 1; (*treq).th_peerid[0] = (*args).ta_my_peerids[0];
    handshake_req_submit((*args).ta_sock, req, flags)
}

pub unsafe fn tls_handshake_cancel(sk: *mut sock) -> bool { handshake_req_cancel(sk) }

pub unsafe fn tls_handshake_close(sock: *mut socket) {
    let req = handshake_req_hash_lookup((*sock).sk); if req.is_null() { return; }
    if !test_and_clear_bit(HANDSHAKE_F_REQ_SESSION, &mut (*req).hr_flags) { return; }
    tls_alert_send(sock, TLS_ALERT_LEVEL_WARNING, TLS_ALERT_DESC_CLOSE_NOTIFY);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
