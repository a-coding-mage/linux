/* SPDX-License-Identifier: GPL-2.0 */

/* External types are supplied by the corresponding translated dependencies. */
use core::ffi::c_char;

/*
 * Handle ticket for a single service.
 */
#[repr(C)]
pub struct ceph_x_ticket_handler {
    pub node: rb_node,
    pub service: core::ffi::c_uint,

    pub session_key: ceph_crypto_key,
    pub have_key: bool,

    pub secret_id: u64,
    pub ticket_blob: *mut ceph_buffer,

    pub renew_after: time64_t,
    pub expires: time64_t,
}

pub const CEPHX_AU_ENC_BUF_LEN: usize = 128; /* big enough for encrypted blob */

#[repr(align(8))]
pub struct ceph_x_authorizer_enc_buf(pub [c_char; CEPHX_AU_ENC_BUF_LEN]);

#[repr(C)]
pub struct ceph_x_authorizer {
    pub base: ceph_authorizer,
    pub session_key: ceph_crypto_key,
    pub buf: *mut ceph_buffer,
    pub service: core::ffi::c_uint,
    pub nonce: u64,
    pub secret_id: u64,
    pub enc_buf: ceph_x_authorizer_enc_buf,
}

#[repr(C)]
pub struct ceph_x_info {
    pub secret: ceph_crypto_key,

    pub starting: bool,
    pub server_challenge: u64,

    pub have_keys: core::ffi::c_uint,
    pub ticket_handlers: rb_root,

    pub auth_authorizer: ceph_x_authorizer,
}

extern "C" {
    pub fn ceph_x_init(ac: *mut ceph_auth_client) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
