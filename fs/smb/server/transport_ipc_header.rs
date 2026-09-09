/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependency intent from the original header: <linux/wait.h>.

use core::ffi::{c_char, c_int, c_void};

pub const KSMBD_IPC_MAX_PAYLOAD: usize = 4096;

#[repr(C)]
pub struct ksmbd_login_response {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_login_response_ext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_share_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_tree_connect {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_tree_connect_response {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_share_config_response {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_spnego_authen_response {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_rpc_command {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ksmbd_ipc_login_request(account: *const c_char) -> *mut ksmbd_login_response;
    pub fn ksmbd_ipc_login_request_ext(account: *const c_char) -> *mut ksmbd_login_response_ext;

    pub fn ksmbd_ipc_tree_connect_request(
        sess: *mut ksmbd_session,
        share: *mut ksmbd_share_config,
        tree_conn: *mut ksmbd_tree_connect,
        peer_addr: *mut sockaddr,
    ) -> *mut ksmbd_tree_connect_response;
    pub fn ksmbd_ipc_tree_disconnect_request(
        session_id: u64,
        connect_id: u64,
    ) -> c_int;
    pub fn ksmbd_ipc_logout_request(account: *const c_char, flags: c_int) -> c_int;
    pub fn ksmbd_ipc_share_config_request(
        name: *const c_char,
    ) -> *mut ksmbd_share_config_response;
    pub fn ksmbd_ipc_spnego_authen_request(
        spnego_blob: *const c_char,
        blob_len: c_int,
    ) -> *mut ksmbd_spnego_authen_response;
    pub fn ksmbd_ipc_id_alloc() -> c_int;
    pub fn ksmbd_rpc_id_free(handle: c_int);
    pub fn ksmbd_rpc_open(sess: *mut ksmbd_session, handle: c_int) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_close(sess: *mut ksmbd_session, handle: c_int) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_write(
        sess: *mut ksmbd_session,
        handle: c_int,
        payload: *mut c_void,
        payload_sz: usize,
    ) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_read(sess: *mut ksmbd_session, handle: c_int) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_rpc_ioctl(
        sess: *mut ksmbd_session,
        handle: c_int,
        payload: *mut c_void,
        payload_sz: usize,
    ) -> *mut ksmbd_rpc_command;
    pub fn ksmbd_ipc_release();
    pub fn ksmbd_ipc_soft_reset();
    pub fn ksmbd_ipc_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
