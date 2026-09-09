// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding kernel/ksmbd translation.

extern "C" {
    fn ksmbd_ipc_login_request(account: *const libc::c_char) -> *mut ksmbd_login_response;
    fn ksmbd_ipc_login_request_ext(
        account: *const libc::c_char,
    ) -> *mut ksmbd_login_response_ext;
    fn ksmbd_ipc_logout_request(name: *mut libc::c_char, flags: u32);
    fn kvfree_sensitive(ptr: *mut libc::c_void, size: usize);
    fn kvfree(ptr: *mut libc::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut libc::c_void;
    fn kmalloc_obj<T>(flags: u32) -> *mut T;
    fn kstrdup(s: *const libc::c_char, flags: u32) -> *mut libc::c_char;
    fn kmemdup(src: *const libc::c_void, size: usize, flags: u32) -> *mut libc::c_void;
    fn memcpy(dst: *mut libc::c_void, src: *const libc::c_void, size: usize) -> *mut libc::c_void;
    fn kfree(ptr: *mut libc::c_void);
    fn kfree_sensitive(ptr: *mut libc::c_void);
    fn strcmp(a: *const libc::c_char, b: *const libc::c_char) -> libc::c_int;
    fn memcmp(a: *const libc::c_void, b: *const libc::c_void, size: usize) -> libc::c_int;
}

pub unsafe fn ksmbd_login_user(account: *const libc::c_char) -> *mut ksmbd_user {
    let resp: *mut ksmbd_login_response;
    let mut resp_ext: *mut ksmbd_login_response_ext = core::ptr::null_mut();
    let mut user: *mut ksmbd_user = core::ptr::null_mut();

    resp = ksmbd_ipc_login_request(account);
    if resp.is_null() {
        return core::ptr::null_mut();
    }

    if (*resp).status & KSMBD_USER_FLAG_OK == 0 {
        kvfree_sensitive(resp.cast(), core::mem::size_of::<ksmbd_login_response>());
        return user;
    }

    if (*resp).status & KSMBD_USER_FLAG_EXTENSION != 0 {
        resp_ext = ksmbd_ipc_login_request_ext(account);
    }

    user = ksmbd_alloc_user(resp, resp_ext);
    kvfree(resp_ext.cast());
    kvfree_sensitive(resp.cast(), core::mem::size_of::<ksmbd_login_response>());
    user
}

pub unsafe fn ksmbd_alloc_user(
    resp: *mut ksmbd_login_response,
    resp_ext: *mut ksmbd_login_response_ext,
) -> *mut ksmbd_user {
    let user: *mut ksmbd_user;

    /* Reject oversized hashes rather than trust the response length. */
    if (*resp).hash_sz as usize > core::mem::size_of_val(&(*resp).hash) {
        return core::ptr::null_mut();
    }

    user = kmalloc_obj::<ksmbd_user>(KSMBD_DEFAULT_GFP);
    if user.is_null() {
        return core::ptr::null_mut();
    }

    (*user).name = kstrdup((*resp).account.as_ptr(), KSMBD_DEFAULT_GFP);
    (*user).flags = (*resp).status;
    (*user).gid = (*resp).gid;
    (*user).uid = (*resp).uid;
    (*user).passkey_sz = (*resp).hash_sz;
    (*user).passkey = kmalloc((*resp).hash_sz as usize, KSMBD_DEFAULT_GFP).cast();
    if !(*user).passkey.is_null() {
        memcpy(
            (*user).passkey.cast(),
            (*resp).hash.as_ptr().cast(),
            (*resp).hash_sz as usize,
        );
    }

    (*user).ngroups = 0;
    (*user).sgid = core::ptr::null_mut();

    if (*user).name.is_null() || (*user).passkey.is_null() {
        kfree((*user).name.cast());
        kfree_sensitive((*user).passkey.cast());
        kfree(user.cast());
        return core::ptr::null_mut();
    }

    if !resp_ext.is_null() {
        (*user).sgid = kmemdup(
            (*resp_ext).__payload.as_ptr().cast(),
            (*resp_ext).ngroups as usize * core::mem::size_of::<libc::gid_t>(),
            KSMBD_DEFAULT_GFP,
        ).cast();
        if (*user).sgid.is_null() {
            kfree((*user).name.cast());
            kfree_sensitive((*user).passkey.cast());
            kfree(user.cast());
            return core::ptr::null_mut();
        }

        (*user).ngroups = (*resp_ext).ngroups;
        ksmbd_debug(SMB, "supplementary groups : %d\n", (*user).ngroups);
    }

    user
}

pub unsafe fn ksmbd_free_user(user: *mut ksmbd_user) {
    ksmbd_ipc_logout_request((*user).name, (*user).flags);
    kfree((*user).sgid.cast());
    kfree((*user).name.cast());
    kfree_sensitive((*user).passkey.cast());
    kfree(user.cast());
}

pub unsafe fn ksmbd_anonymous_user(user: *mut ksmbd_user) -> bool {
    *(*user).name == 0
}

pub unsafe fn ksmbd_compare_user(u1: *mut ksmbd_user, u2: *mut ksmbd_user) -> bool {
    if strcmp((*u1).name, (*u2).name) != 0 {
        return false;
    }
    if memcmp((*u1).passkey.cast(), (*u2).passkey.cast(), (*u1).passkey_sz as usize) != 0 {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
