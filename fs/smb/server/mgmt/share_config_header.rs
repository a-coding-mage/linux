/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::c_char;

pub struct ksmbd_work;

#[repr(C)]
pub struct ksmbd_share_config {
    pub name: *mut c_char,
    pub path: *mut c_char,

    pub path_sz: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub veto_list: list_head,

    pub vfs_path: path,

    pub refcount: atomic_t,
    #[cfg(CONFIG_PROC_FS)]
    pub tree_connections: atomic_t,
    pub hlist: hlist_node,
    pub create_mask: u16,
    pub directory_mask: u16,
    pub force_create_mode: u16,
    pub force_directory_mode: u16,
    pub force_uid: u16,
    pub force_gid: u16,
}

pub const KSMBD_SHARE_INVALID_UID: u16 = u16::MAX;
pub const KSMBD_SHARE_INVALID_GID: u16 = u16::MAX;

pub unsafe fn share_config_create_mode(
    share: *mut ksmbd_share_config,
    posix_mode: umode_t,
) -> umode_t {
    let mode: umode_t = (if posix_mode != 0 {
        posix_mode
    } else {
        (!0 as umode_t)
    }) & (*share).create_mask as umode_t;

    mode | (*share).force_create_mode as umode_t
}

pub unsafe fn share_config_directory_mode(
    share: *mut ksmbd_share_config,
    posix_mode: umode_t,
) -> umode_t {
    let mode: umode_t = (if posix_mode != 0 {
        posix_mode
    } else {
        (!0 as umode_t)
    }) & (*share).directory_mask as umode_t;

    mode | (*share).force_directory_mode as umode_t
}

pub unsafe fn test_share_config_flag(
    share: *mut ksmbd_share_config,
    flag: core::ffi::c_int,
) -> core::ffi::c_int {
    ((*share).flags as core::ffi::c_int) & flag
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn ksmbd_share_tree_conn_init(share: *mut ksmbd_share_config) {
    atomic_set(&mut (*share).tree_connections, 0);
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn ksmbd_share_tree_conn_inc(share: *mut ksmbd_share_config) {
    atomic_inc(&mut (*share).tree_connections);
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn ksmbd_share_tree_conn_dec(share: *mut ksmbd_share_config) {
    atomic_dec(&mut (*share).tree_connections);
}

#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn ksmbd_share_tree_conn_init(_share: *mut ksmbd_share_config) {}

#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn ksmbd_share_tree_conn_inc(_share: *mut ksmbd_share_config) {}

#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn ksmbd_share_tree_conn_dec(_share: *mut ksmbd_share_config) {}

unsafe extern "C" {
    pub fn ksmbd_share_config_del(share: *mut ksmbd_share_config);
    pub fn __ksmbd_share_config_put(share: *mut ksmbd_share_config);
}

pub unsafe fn ksmbd_share_config_put(share: *mut ksmbd_share_config) {
    if !atomic_dec_and_test(&mut (*share).refcount) {
        return;
    }
    __ksmbd_share_config_put(share);
}

unsafe extern "C" {
    pub fn ksmbd_share_config_get(
        work: *mut ksmbd_work,
        name: *const c_char,
    ) -> *mut ksmbd_share_config;
    pub fn ksmbd_share_veto_filename(
        share: *mut ksmbd_share_config,
        filename: *const c_char,
    ) -> bool;
    pub fn create_proc_shares() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
