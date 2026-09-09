/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding kernel/SMB translation.

pub const CIFDS_SESSION_FLAG_SMB2: ::core::ffi::c_int = 1 << 1;
pub const PREAUTH_HASHVALUE_SIZE: usize = 64;

#[repr(C)]
pub struct ksmbd_file_table;

#[repr(C)]
pub struct channel {
    pub sess_key: [::core::ffi::c_char; CIFS_KEY_SIZE],
    pub smb3signingkey: [u8; SMB3_SIGN_KEY_SIZE],
    pub conn: *mut ksmbd_conn,
}

#[repr(C)]
pub struct preauth_session {
    pub Preauth_HashValue: [u8; PREAUTH_HASHVALUE_SIZE],
    pub id: u64,
    pub preauth_entry: list_head,
}

#[repr(C)]
pub struct ksmbd_session {
    pub id: u64,
    pub dialect: u16,
    pub ClientGUID: [::core::ffi::c_char; SMB2_CLIENT_GUID_SIZE],
    pub user: *mut ksmbd_user,
    pub sequence_number: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub sign: bool,
    pub enc: bool,
    pub state: ::core::ffi::c_int,
    pub Preauth_HashValue: *mut u8,
    pub sess_key: [::core::ffi::c_char; CIFS_KEY_SIZE],
    pub kerberos_expiry: u64,
    pub hlist: hlist_node,
    pub chann_lock: rw_semaphore,
    pub ksmbd_chann_list: xarray,
    pub tree_conns: xarray,
    pub tree_conn_ida: ida,
    pub rpc_handle_list: xarray,
    pub smb3encryptionkey: [u8; SMB3_ENC_DEC_KEY_SIZE],
    pub smb3decryptionkey: [u8; SMB3_ENC_DEC_KEY_SIZE],
    pub smb3signingkey: [u8; SMB3_SIGN_KEY_SIZE],
    pub file_table: ksmbd_file_table,
    pub last_active: ::core::ffi::c_ulong,
    pub tree_conns_lock: rw_semaphore,
    #[cfg(CONFIG_PROC_FS)]
    pub proc_entry: *mut proc_dir_entry,
    pub refcnt: atomic_t,
    pub rpc_lock: rw_semaphore,
}

#[inline]
pub unsafe fn test_session_flag(sess: *mut ksmbd_session, bit: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (*sess).flags & bit as ::core::ffi::c_uint as ::core::ffi::c_int
}

#[inline]
pub unsafe fn set_session_flag(sess: *mut ksmbd_session, bit: ::core::ffi::c_int) {
    (*sess).flags |= bit as ::core::ffi::c_uint;
}

#[inline]
pub unsafe fn clear_session_flag(sess: *mut ksmbd_session, bit: ::core::ffi::c_int) {
    (*sess).flags &= !(bit as ::core::ffi::c_uint);
}

extern "C" {
    pub fn ksmbd_smb2_session_create() -> *mut ksmbd_session;
    pub fn ksmbd_session_destroy(sess: *mut ksmbd_session);
    pub fn ksmbd_session_lookup_slowpath(id: u64) -> *mut ksmbd_session;
    pub fn ksmbd_session_lookup(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session;
    pub fn is_ksmbd_session_in_connection(conn: *mut ksmbd_conn, id: u64) -> bool;
    pub fn ksmbd_session_register(conn: *mut ksmbd_conn, sess: *mut ksmbd_session) -> ::core::ffi::c_int;
    pub fn ksmbd_sessions_deregister(conn: *mut ksmbd_conn);
    pub fn __session_lookup(id: u64) -> *mut ksmbd_session;
    pub fn ksmbd_session_lookup_all(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session;
    pub fn ksmbd_session_lookup_all_states(conn: *mut ksmbd_conn, id: u64) -> *mut ksmbd_session;
    pub fn destroy_previous_session(conn: *mut ksmbd_conn, user: *mut ksmbd_user, id: u64);
    pub fn ksmbd_preauth_session_alloc(conn: *mut ksmbd_conn, sess_id: u64) -> *mut preauth_session;
    pub fn ksmbd_preauth_session_destroy(conn: *mut ksmbd_conn);
    pub fn ksmbd_preauth_session_lookup(conn: *mut ksmbd_conn, id: u64) -> *mut preauth_session;
    pub fn ksmbd_acquire_tree_conn_id(sess: *mut ksmbd_session) -> ::core::ffi::c_int;
    pub fn ksmbd_release_tree_conn_id(sess: *mut ksmbd_session, id: ::core::ffi::c_int);
    pub fn ksmbd_session_rpc_open(sess: *mut ksmbd_session, rpc_name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn ksmbd_session_rpc_close(sess: *mut ksmbd_session, id: ::core::ffi::c_int);
    pub fn ksmbd_session_rpc_method(sess: *mut ksmbd_session, id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ksmbd_user_session_get(sess: *mut ksmbd_session);
    pub fn ksmbd_user_session_put(sess: *mut ksmbd_session);
    pub fn create_proc_sessions() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
