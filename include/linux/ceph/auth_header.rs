/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
pub struct ceph_auth_client;
pub struct ceph_msg;
pub struct ceph_crypto_key;
pub struct mutex;

/*
 * Abstract interface for communicating with the authenticate module.
 * There is some handshake that takes place between us and the monitor
 * to acquire the necessary keys.  These are used to generate an
 * 'authorizer' that we use when connecting to a service (mds, osd).
 */

#[repr(C)]
pub struct ceph_authorizer {
    pub destroy: Option<unsafe extern "C" fn(*mut ceph_authorizer)>,
}

#[repr(C)]
pub struct ceph_auth_handshake {
    pub authorizer: *mut ceph_authorizer,
    pub authorizer_buf: *mut core::ffi::c_void,
    pub authorizer_buf_len: usize,
    pub authorizer_reply_buf: *mut core::ffi::c_void,
    pub authorizer_reply_buf_len: usize,
    pub sign_message: Option<unsafe extern "C" fn(*mut ceph_auth_handshake, *mut ceph_msg) -> i32>,
    pub check_message_signature:
        Option<unsafe extern "C" fn(*mut ceph_auth_handshake, *mut ceph_msg) -> i32>,
}

#[repr(C)]
pub struct ceph_auth_client_ops {
    // true if we are authenticated and can connect to services.
    pub is_authenticated: Option<unsafe extern "C" fn(*mut ceph_auth_client) -> i32>,
    // true if we should (re)authenticate, e.g., when our tickets are getting old and crusty.
    pub should_authenticate: Option<unsafe extern "C" fn(*mut ceph_auth_client) -> i32>,
    // Build requests and process replies during monitor handshake.
    pub build_request: Option<unsafe extern "C" fn(*mut ceph_auth_client, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32>,
    pub handle_reply: Option<unsafe extern "C" fn(*mut ceph_auth_client, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut i32, *mut u8, *mut i32) -> i32>,
    // Create authorizer for connecting to a service, and verify the response.
    pub create_authorizer: Option<unsafe extern "C" fn(*mut ceph_auth_client, i32, *mut ceph_auth_handshake) -> i32>,
    pub update_authorizer: Option<unsafe extern "C" fn(*mut ceph_auth_client, i32, *mut ceph_auth_handshake) -> i32>,
    pub add_authorizer_challenge: Option<unsafe extern "C" fn(*mut ceph_auth_client, *mut ceph_authorizer, *mut core::ffi::c_void, i32) -> i32>,
    pub verify_authorizer_reply: Option<unsafe extern "C" fn(*mut ceph_auth_client, *mut ceph_authorizer, *mut core::ffi::c_void, i32, *mut u8, *mut i32, *mut u8, *mut i32) -> i32>,
    pub invalidate_authorizer: Option<unsafe extern "C" fn(*mut ceph_auth_client, i32)>,
    // reset when we (re)connect to a monitor
    pub reset: Option<unsafe extern "C" fn(*mut ceph_auth_client)>,
    pub destroy: Option<unsafe extern "C" fn(*mut ceph_auth_client)>,
    pub sign_message: Option<unsafe extern "C" fn(*mut ceph_auth_handshake, *mut ceph_msg) -> i32>,
    pub check_message_signature: Option<unsafe extern "C" fn(*mut ceph_auth_handshake, *mut ceph_msg) -> i32>,
}

#[repr(C)]
pub struct ceph_auth_client {
    pub protocol: u32,
    pub private: *mut core::ffi::c_void,
    pub ops: *const ceph_auth_client_ops,
    pub negotiating: bool,
    pub name: *const core::ffi::c_char,
    pub global_id: u64,
    pub key: *const ceph_crypto_key,
    pub want_keys: core::ffi::c_uint,
    pub preferred_mode: i32,
    pub fallback_mode: i32,
    pub mutex: mutex,
}

extern "C" {
    pub fn ceph_auth_set_global_id(ac: *mut ceph_auth_client, global_id: u64);
    pub fn ceph_auth_init(name: *const core::ffi::c_char, key: *const ceph_crypto_key, con_modes: *const i32) -> *mut ceph_auth_client;
    pub fn ceph_auth_destroy(ac: *mut ceph_auth_client);
    pub fn ceph_auth_reset(ac: *mut ceph_auth_client);
    pub fn ceph_auth_build_hello(ac: *mut ceph_auth_client, buf: *mut core::ffi::c_void, len: usize) -> i32;
    pub fn ceph_handle_auth_reply(ac: *mut ceph_auth_client, buf: *mut core::ffi::c_void, len: usize, reply_buf: *mut core::ffi::c_void, reply_len: usize) -> i32;
    pub fn ceph_auth_entity_name_encode(name: *const core::ffi::c_char, p: *mut *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32;
    pub fn ceph_build_auth(ac: *mut ceph_auth_client, msg_buf: *mut core::ffi::c_void, msg_len: usize) -> i32;
    pub fn ceph_auth_is_authenticated(ac: *mut ceph_auth_client) -> i32;
    pub fn __ceph_auth_get_authorizer(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, peer_type: i32, force_new: bool, proto: *mut i32, pref_mode: *mut i32, fallb_mode: *mut i32) -> i32;
    pub fn ceph_auth_destroy_authorizer(a: *mut ceph_authorizer);
    pub fn ceph_auth_add_authorizer_challenge(ac: *mut ceph_auth_client, a: *mut ceph_authorizer, challenge_buf: *mut core::ffi::c_void, challenge_buf_len: i32) -> i32;
    pub fn ceph_auth_verify_authorizer_reply(ac: *mut ceph_auth_client, a: *mut ceph_authorizer, reply: *mut core::ffi::c_void, reply_len: i32, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32;
    pub fn ceph_auth_invalidate_authorizer(ac: *mut ceph_auth_client, peer_type: i32);
}

#[inline]
pub unsafe fn ceph_auth_sign_message(auth: *mut ceph_auth_handshake, msg: *mut ceph_msg) -> i32 {
    if let Some(f) = (*auth).sign_message { f(auth, msg) } else { 0 }
}

#[inline]
pub unsafe fn ceph_auth_check_message_signature(auth: *mut ceph_auth_handshake, msg: *mut ceph_msg) -> i32 {
    if let Some(f) = (*auth).check_message_signature { f(auth, msg) } else { 0 }
}

extern "C" {
    pub fn ceph_auth_get_request(ac: *mut ceph_auth_client, buf: *mut core::ffi::c_void, buf_len: i32) -> i32;
    pub fn ceph_auth_handle_reply_more(ac: *mut ceph_auth_client, reply: *mut core::ffi::c_void, reply_len: i32, buf: *mut core::ffi::c_void, buf_len: i32) -> i32;
    pub fn ceph_auth_handle_reply_done(ac: *mut ceph_auth_client, global_id: u64, reply: *mut core::ffi::c_void, reply_len: i32, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32;
    pub fn ceph_auth_handle_bad_method(ac: *mut ceph_auth_client, used_proto: i32, result: i32, allowed_protos: *const i32, proto_cnt: i32, allowed_modes: *const i32, mode_cnt: i32) -> bool;
    pub fn ceph_auth_get_authorizer(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, peer_type: i32, buf: *mut core::ffi::c_void, buf_len: *mut i32) -> i32;
    pub fn ceph_auth_handle_svc_reply_more(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, reply: *mut core::ffi::c_void, reply_len: i32, buf: *mut core::ffi::c_void, buf_len: *mut i32) -> i32;
    pub fn ceph_auth_handle_svc_reply_done(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, reply: *mut core::ffi::c_void, reply_len: i32, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32;
    pub fn ceph_auth_handle_bad_authorizer(ac: *mut ceph_auth_client, peer_type: i32, used_proto: i32, result: i32, allowed_protos: *const i32, proto_cnt: i32, allowed_modes: *const i32, mode_cnt: i32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
