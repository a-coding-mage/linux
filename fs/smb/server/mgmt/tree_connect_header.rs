/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// The C header guard and include directives are omitted.  Types supplied by
// the included headers are referenced through the crate's corresponding names.

use core::ffi::c_char;

pub const TREE_NEW: i32 = 0;
pub const TREE_CONNECTED: i32 = 1;
pub const TREE_DISCONNECTED: i32 = 2;

pub struct ksmbd_share_config;
pub struct ksmbd_user;
pub struct ksmbd_conn;
pub struct ksmbd_work;

#[repr(C)]
pub struct ksmbd_tree_connect {
    pub id: i32,
    pub flags: u32,
    pub share_conf: *mut ksmbd_share_config,
    pub user: *mut ksmbd_user,
    pub list: crate::list_head,
    pub maximal_access: i32,
    pub posix_extensions: bool,
    pub refcount: crate::atomic_t,
    pub t_state: u32,
}

#[repr(C)]
pub struct ksmbd_tree_conn_status {
    pub ret: u32,
    pub tree_conn: *mut ksmbd_tree_connect,
}

#[inline]
pub unsafe fn test_tree_conn_flag(tree_conn: *mut ksmbd_tree_connect, flag: i32) -> i32 {
    (*tree_conn).flags as i32 & flag
}

pub struct ksmbd_session;

unsafe extern "C" {
    pub fn ksmbd_tree_conn_connect(
        work: *mut ksmbd_work,
        share_name: *const c_char,
    ) -> ksmbd_tree_conn_status;
    pub fn ksmbd_tree_connect_put(tcon: *mut ksmbd_tree_connect);

    pub fn ksmbd_tree_conn_disconnect(
        sess: *mut ksmbd_session,
        tree_conn: *mut ksmbd_tree_connect,
    ) -> i32;

    pub fn ksmbd_tree_conn_lookup(
        sess: *mut ksmbd_session,
        id: u32,
    ) -> *mut ksmbd_tree_connect;

    pub fn ksmbd_tree_conn_session_logoff(sess: *mut ksmbd_session) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
