// SPDX-License-Identifier: GPL-2.0-or-later
/* user_defined.c: user defined key type
 *
 * Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_payload {
    pub data: [*mut c_void; 4],
}

#[repr(C)]
pub struct key {
    pub payload: key_payload,
    pub description: *const c_char,
    pub datalen: u32,
    pub expiry: u64,
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub data: *const c_void,
    pub datalen: usize,
    pub quotalen: usize,
    pub expiry: u64,
    pub payload: key_payload,
}

#[repr(C)]
pub struct user_key_payload {
    pub rcu: rcu_head,
    pub datalen: usize,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub instantiate: Option<unsafe extern "C" fn()>,
    pub update: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub revoke: Option<unsafe extern "C" fn(*mut key)>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
    pub read: Option<unsafe extern "C" fn(*const key, *mut c_char, usize) -> c_long>,
    pub vet_description: Option<unsafe extern "C" fn(*const c_char) -> c_int>,
}

unsafe extern "C" {
    fn generic_key_instantiate();
    fn key_payload_reserve(key: *mut key, datalen: usize) -> c_int;
    fn key_is_positive(key: *const key) -> bool;
    fn dereference_key_locked(key: *const key) -> *mut user_key_payload;
    fn user_key_payload_locked(key: *const key) -> *mut user_key_payload;
    fn rcu_assign_keypointer(key: *mut key, payload: *mut c_void);
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn kfree_sensitive(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
}

unsafe extern "C" {
    static mut key_type_user: key_type;
    static mut key_type_logon: key_type;
}

/*
 * user defined keys take an arbitrary string as the description and an
 * arbitrary blob of data as the payload
 */
#[unsafe(export_name = "key_type_user")]
pub static mut KEY_TYPE_USER: key_type = key_type {
    name: c"user".as_ptr(),
    preparse: Some(user_preparse),
    free_preparse: Some(user_free_preparse),
    instantiate: Some(generic_key_instantiate),
    update: Some(user_update),
    revoke: Some(user_revoke),
    destroy: Some(user_destroy),
    describe: Some(user_describe),
    read: Some(user_read),
    vet_description: None,
};

/* EXPORT_SYMBOL_GPL(key_type_user); */

/*
 * This key type is essentially the same as key_type_user, but it does
 * not define a .read op. This is suitable for storing username and
 * password pairs in the keyring that you do not want to be readable
 * from userspace.
 */
#[unsafe(export_name = "key_type_logon")]
pub static mut KEY_TYPE_LOGON: key_type = key_type {
    name: c"logon".as_ptr(),
    preparse: Some(user_preparse),
    free_preparse: Some(user_free_preparse),
    instantiate: Some(generic_key_instantiate),
    update: Some(user_update),
    revoke: Some(user_revoke),
    destroy: Some(user_destroy),
    describe: Some(user_describe),
    read: None,
    vet_description: Some(logon_vet_description),
};
/* EXPORT_SYMBOL_GPL(key_type_logon); */

unsafe fn kmalloc_flex_user_key_payload_data(datalen: usize) -> *mut user_key_payload {
    unsafe extern "C" {
        fn kmalloc_flex_user_key_payload_data(datalen: usize) -> *mut user_key_payload;
    }

    unsafe { kmalloc_flex_user_key_payload_data(datalen) }
}

/*
 * Preparse a user defined key payload
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_preparse(prep: *mut key_preparsed_payload) -> c_int {
    let mut upayload: *mut user_key_payload;
    let datalen = unsafe { (*prep).datalen };

    if datalen == 0 || datalen > 32767 || unsafe { (*prep).data }.is_null() {
        return -EINVAL;
    }

    upayload = unsafe { kmalloc_flex_user_key_payload_data(datalen) };
    if upayload.is_null() {
        return -ENOMEM;
    }

    /* attach the data */
    unsafe {
        (*prep).quotalen = datalen;
        (*prep).payload.data[0] = upayload.cast();
        (*upayload).datalen = datalen;
        memcpy(
            (*upayload).data.as_mut_ptr().cast(),
            (*prep).data,
            datalen,
        );
    }
    0
}
/* EXPORT_SYMBOL_GPL(user_preparse); */

/*
 * Free a preparse of a user defined key payload
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_free_preparse(prep: *mut key_preparsed_payload) {
    unsafe {
        kfree_sensitive((*prep).payload.data[0]);
    }
}
/* EXPORT_SYMBOL_GPL(user_free_preparse); */

unsafe extern "C" fn user_free_payload_rcu(head: *mut rcu_head) {
    let payload: *mut user_key_payload;

    payload = head.cast::<user_key_payload>();
    unsafe {
        kfree_sensitive(payload.cast());
    }
}

/*
 * update a user defined key
 * - the key's semaphore is write-locked
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_update(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    let mut zap: *mut user_key_payload = ptr::null_mut();
    let ret: c_int;

    /* check the quota and attach the new data */
    ret = unsafe { key_payload_reserve(key, (*prep).datalen) };
    if ret < 0 {
        return ret;
    }

    /* attach the new data, displacing the old */
    unsafe {
        (*key).expiry = (*prep).expiry;
        if key_is_positive(key) {
            zap = dereference_key_locked(key);
        }
        rcu_assign_keypointer(key, (*prep).payload.data[0]);
        (*prep).payload.data[0] = ptr::null_mut();

        if !zap.is_null() {
            call_rcu(&mut (*zap).rcu, user_free_payload_rcu);
        }
    }
    ret
}
/* EXPORT_SYMBOL_GPL(user_update); */

/*
 * dispose of the links from a revoked keyring
 * - called with the key sem write-locked
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_revoke(key: *mut key) {
    let upayload = unsafe { user_key_payload_locked(key) };

    /* clear the quota */
    unsafe {
        key_payload_reserve(key, 0);

        if !upayload.is_null() {
            rcu_assign_keypointer(key, ptr::null_mut());
            call_rcu(&mut (*upayload).rcu, user_free_payload_rcu);
        }
    }
}

/* EXPORT_SYMBOL(user_revoke); */

/*
 * dispose of the data dangling from the corpse of a user key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_destroy(key: *mut key) {
    let upayload = unsafe { (*key).payload.data[0] };

    unsafe {
        kfree_sensitive(upayload);
    }
}

/* EXPORT_SYMBOL_GPL(user_destroy); */

/*
 * describe the user key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_describe(key: *const key, m: *mut seq_file) {
    unsafe {
        seq_puts(m, (*key).description);
        if key_is_positive(key) {
            seq_printf(m, c": %u".as_ptr(), (*key).datalen);
        }
    }
}

/* EXPORT_SYMBOL_GPL(user_describe); */

/*
 * read the key data
 * - the key's semaphore is read-locked
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_read(
    key: *const key,
    buffer: *mut c_char,
    mut buflen: usize,
) -> c_long {
    let upayload: *const user_key_payload;
    let ret: c_long;

    upayload = unsafe { user_key_payload_locked(key) };
    ret = unsafe { (*upayload).datalen as c_long };

    /* we can return the data as is */
    if !buffer.is_null() && buflen > 0 {
        unsafe {
            if buflen > (*upayload).datalen {
                buflen = (*upayload).datalen;
            }

            memcpy(buffer.cast(), (*upayload).data.as_ptr().cast(), buflen);
        }
    }

    ret
}

/* EXPORT_SYMBOL_GPL(user_read); */

/* Vet the description for a "logon" key */
unsafe extern "C" fn logon_vet_description(desc: *const c_char) -> c_int {
    let p: *mut c_char;

    /* require a "qualified" description string */
    p = unsafe { strchr(desc, ':' as c_int) };
    if p.is_null() {
        return -EINVAL;
    }

    /* also reject description with ':' as first char */
    if ptr::addr_eq(p.cast_const(), desc) {
        return -EINVAL;
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
