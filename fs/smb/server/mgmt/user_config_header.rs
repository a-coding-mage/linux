/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependency intent: ../glob.h

#[repr(C)]
pub struct ksmbd_user {
    pub flags: u16,
    pub uid: u32,
    pub gid: u32,
    pub name: *mut std::ffi::c_char,
    pub passkey_sz: usize,
    pub passkey: *mut std::ffi::c_char,
    pub ngroups: std::ffi::c_int,
    pub sgid: *mut u32,
}

#[inline]
pub unsafe fn user_guest(user: *mut ksmbd_user) -> bool {
    ((*user).flags & (KSMBD_USER_FLAG_GUEST_ACCOUNT as u16)) != 0
}

#[inline]
pub unsafe fn set_user_flag(user: *mut ksmbd_user, flag: std::ffi::c_int) {
    (*user).flags |= flag as u16;
}

#[inline]
pub unsafe fn test_user_flag(user: *mut ksmbd_user, flag: std::ffi::c_int) -> std::ffi::c_int {
    ((*user).flags & (flag as u16)) as std::ffi::c_int
}

#[inline]
pub unsafe fn set_user_guest(_user: *mut ksmbd_user) {
}

#[inline]
pub unsafe fn user_passkey(user: *mut ksmbd_user) -> *mut std::ffi::c_char {
    (*user).passkey
}

#[inline]
pub unsafe fn user_name(user: *mut ksmbd_user) -> *mut std::ffi::c_char {
    (*user).name
}

#[inline]
pub unsafe fn user_uid(user: *mut ksmbd_user) -> u32 {
    (*user).uid
}

#[inline]
pub unsafe fn user_gid(user: *mut ksmbd_user) -> u32 {
    (*user).gid
}

extern "C" {
    pub fn ksmbd_login_user(account: *const std::ffi::c_char) -> *mut ksmbd_user;
    pub fn ksmbd_alloc_user(
        resp: *mut ksmbd_login_response,
        resp_ext: *mut ksmbd_login_response_ext,
    ) -> *mut ksmbd_user;
    pub fn ksmbd_free_user(user: *mut ksmbd_user);
    pub fn ksmbd_anonymous_user(user: *mut ksmbd_user) -> bool;
    pub fn ksmbd_compare_user(u1: *mut ksmbd_user, u2: *mut ksmbd_user) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
