// SPDX-License-Identifier: GPL-2.0-or-later
/* General persistent per-UID keyrings register
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies from:
// #include <linux/user_namespace.h>
// #include <linux/cred.h>
// #include "internal.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

pub type uid_t = c_uint;
pub type key_serial_t = c_int;
pub type key_ref_t = *mut c_void;

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuid_t {
    pub val: uid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgid_t {
    pub val: c_uint,
}

#[repr(C)]
pub struct key {
    pub type_: *mut key_type,
    pub serial: key_serial_t,
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    pub keyring_sem: rw_semaphore,
    pub persistent_keyring_register: *mut key,
}

#[repr(C)]
pub struct keyring_index_key {
    pub type_: *mut key_type,
    pub description: *mut c_char,
    pub desc_len: usize,
}

pub const KEY_POS_ALL: c_uint = 0x3f000000;
pub const KEY_POS_SETATTR: c_uint = 0x01000000;
pub const KEY_USR_VIEW: c_uint = 0x00100000;
pub const KEY_USR_READ: c_uint = 0x00020000;
pub const KEY_ALLOC_NOT_IN_QUOTA: c_uint = 0x0002;
pub const KEY_NEED_LINK: c_uint = 0x04;
pub const KEY_NEED_WRITE: c_uint = 0x02;
pub const KEY_LOOKUP_CREATE: c_uint = 0x01;
pub const CAP_SETUID: c_int = 7;
pub const EINVAL: c_long = 22;
pub const EPERM: c_long = 1;
pub const ENOTDIR: c_long = 20;

pub const INVALID_GID: kgid_t = kgid_t { val: !0 };

#[inline]
pub const fn KUIDT_INIT(value: uid_t) -> kuid_t {
    kuid_t { val: value }
}

#[inline]
pub const fn KGIDT_INIT(value: c_uint) -> kgid_t {
    kgid_t { val: value }
}

unsafe extern "C" {
    pub static mut key_type_keyring: key_type;

    pub fn current_cred() -> *const cred;
    pub fn current_user_ns() -> *mut user_namespace;
    pub fn current_uid() -> kuid_t;
    pub fn current_euid() -> kuid_t;
    pub fn make_kuid(ns: *mut user_namespace, uid: uid_t) -> kuid_t;
    pub fn from_kuid(ns: *mut user_namespace, uid: kuid_t) -> uid_t;
    pub fn uid_valid(uid: kuid_t) -> bool;
    pub fn uid_eq(left: kuid_t, right: kuid_t) -> bool;
    pub fn ns_capable(ns: *mut user_namespace, cap: c_int) -> bool;

    pub fn keyring_alloc(
        description: *const c_char,
        uid: kuid_t,
        gid: kgid_t,
        cred: *const cred,
        perm: c_uint,
        flags: c_uint,
        restrict_link: *mut c_void,
        dest: *mut key,
    ) -> *mut key;
    pub fn make_key_ref(key: *mut key, possession: bool) -> key_ref_t;
    pub fn find_key_to_update(keyring_ref: key_ref_t, index_key: *mut keyring_index_key) -> key_ref_t;
    pub fn key_set_index_key(index_key: *mut keyring_index_key);
    pub fn key_task_permission(key_ref: key_ref_t, cred: *const cred, perm: c_uint) -> c_long;
    pub fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    pub fn key_link(keyring: *mut key, key: *mut key) -> c_int;
    pub fn key_set_timeout(key: *mut key, timeout: c_uint);
    pub fn key_ref_put(key_ref: key_ref_t);
    pub fn lookup_user_key(id: key_serial_t, flags: c_uint, perm: c_uint) -> key_ref_t;

    pub fn memset(dest: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    pub fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn down_read(sem: *mut rw_semaphore);
    pub fn up_read(sem: *mut rw_semaphore);
    pub fn down_write(sem: *mut rw_semaphore);
    pub fn up_write(sem: *mut rw_semaphore);
}

pub static mut persistent_keyring_expiry: c_uint = 3 * 24 * 3600; /* Expire after 3 days of non-use */

#[inline]
unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as usize) >= (usize::MAX - 4095)
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as isize as c_long
}

#[inline]
unsafe fn ERR_PTR(error: c_long) -> key_ref_t {
    error as isize as key_ref_t
}

#[inline]
unsafe fn ERR_CAST<T>(ptr: *mut T) -> key_ref_t {
    ptr as key_ref_t
}

/*
 * Create the persistent keyring register for the current user namespace.
 *
 * Called with the namespace's sem locked for writing.
 */
unsafe fn key_create_persistent_register(ns: *mut user_namespace) -> c_int {
    let reg: *mut key = unsafe {
        keyring_alloc(
            c".persistent_register".as_ptr(),
            KUIDT_INIT(0),
            KGIDT_INIT(0),
            current_cred(),
            (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
            KEY_ALLOC_NOT_IN_QUOTA,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    if unsafe { IS_ERR(reg as *const c_void) } {
        return unsafe { PTR_ERR(reg) as c_int };
    }

    unsafe {
        (*ns).persistent_keyring_register = reg;
    }
    0
}

/*
 * Create the persistent keyring for the specified user.
 *
 * Called with the namespace's sem locked for writing.
 */
unsafe fn key_create_persistent(
    ns: *mut user_namespace,
    uid: kuid_t,
    index_key: *mut keyring_index_key,
) -> key_ref_t {
    let persistent: *mut key;
    let reg_ref: key_ref_t;
    let persistent_ref: key_ref_t;

    if unsafe { (*ns).persistent_keyring_register.is_null() } {
        let err: c_long = unsafe { key_create_persistent_register(ns) as c_long };
        if err < 0 {
            return unsafe { ERR_PTR(err) };
        }
    } else {
        reg_ref = unsafe { make_key_ref((*ns).persistent_keyring_register, true) };
        persistent_ref = unsafe { find_key_to_update(reg_ref, index_key) };
        if !persistent_ref.is_null() {
            return persistent_ref;
        }
    }

    persistent = unsafe {
        keyring_alloc(
            (*index_key).description,
            uid,
            INVALID_GID,
            current_cred(),
            (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
            KEY_ALLOC_NOT_IN_QUOTA,
            core::ptr::null_mut(),
            (*ns).persistent_keyring_register,
        )
    };
    if unsafe { IS_ERR(persistent as *const c_void) } {
        return unsafe { ERR_CAST(persistent) };
    }

    unsafe { make_key_ref(persistent, true) }
}

/*
 * Get the persistent keyring for a specific UID and link it to the nominated
 * keyring.
 */
unsafe fn key_get_persistent(ns: *mut user_namespace, uid: kuid_t, dest_ref: key_ref_t) -> c_long {
    let mut index_key: keyring_index_key = unsafe { core::mem::zeroed() };
    let persistent: *mut key;
    let reg_ref: key_ref_t;
    let mut persistent_ref: key_ref_t;
    let mut buf = [0 as c_char; 32];
    let mut ret: c_long;

    /* Look in the register if it exists */
    unsafe {
        memset(
            &mut index_key as *mut keyring_index_key as *mut c_void,
            0,
            core::mem::size_of::<keyring_index_key>(),
        );
    }
    index_key.type_ = unsafe { &raw mut key_type_keyring };
    index_key.description = buf.as_mut_ptr();
    index_key.desc_len =
        unsafe { sprintf(buf.as_mut_ptr(), c"_persistent.%u".as_ptr(), from_kuid(ns, uid)) } as usize;
    unsafe {
        key_set_index_key(&mut index_key);
    }

    if unsafe { !(*ns).persistent_keyring_register.is_null() } {
        reg_ref = unsafe { make_key_ref((*ns).persistent_keyring_register, true) };
        unsafe {
            down_read(&mut (*ns).keyring_sem);
        }
        persistent_ref = unsafe { find_key_to_update(reg_ref, &mut index_key) };
        unsafe {
            up_read(&mut (*ns).keyring_sem);
        }

        if !persistent_ref.is_null() {
            /* found */
        } else {
            /* It wasn't in the register, so we'll need to create it.  We might
             * also need to create the register.
             */
            unsafe {
                down_write(&mut (*ns).keyring_sem);
            }
            persistent_ref = unsafe { key_create_persistent(ns, uid, &mut index_key) };
            unsafe {
                up_write(&mut (*ns).keyring_sem);
            }
            if unsafe { IS_ERR(persistent_ref as *const c_void) } {
                return unsafe { PTR_ERR(persistent_ref) };
            }
        }
    } else {
        /* It wasn't in the register, so we'll need to create it.  We might
         * also need to create the register.
         */
        unsafe {
            down_write(&mut (*ns).keyring_sem);
        }
        persistent_ref = unsafe { key_create_persistent(ns, uid, &mut index_key) };
        unsafe {
            up_write(&mut (*ns).keyring_sem);
        }
        if unsafe { IS_ERR(persistent_ref as *const c_void) } {
            return unsafe { PTR_ERR(persistent_ref) };
        }
    }

    ret = unsafe { key_task_permission(persistent_ref, current_cred(), KEY_NEED_LINK) };
    if ret == 0 {
        persistent = unsafe { key_ref_to_ptr(persistent_ref) };
        ret = unsafe { key_link(key_ref_to_ptr(dest_ref), persistent) as c_long };
        if ret == 0 {
            unsafe {
                key_set_timeout(persistent, persistent_keyring_expiry);
            }
            ret = unsafe { (*persistent).serial as c_long };
        }
    }

    unsafe {
        key_ref_put(persistent_ref);
    }
    ret
}

/*
 * Get the persistent keyring for a specific UID and link it to the nominated
 * keyring.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn keyctl_get_persistent(_uid: uid_t, destid: key_serial_t) -> c_long {
    let ns: *mut user_namespace = unsafe { current_user_ns() };
    let dest_ref: key_ref_t;
    let uid: kuid_t;
    let ret: c_long;

    /* -1 indicates the current user */
    if _uid == -1i32 as uid_t {
        uid = unsafe { current_uid() };
    } else {
        uid = unsafe { make_kuid(ns, _uid) };
        if !unsafe { uid_valid(uid) } {
            return -EINVAL;
        }

        /* You can only see your own persistent cache if you're not
         * sufficiently privileged.
         */
        if !unsafe { uid_eq(uid, current_uid()) }
            && !unsafe { uid_eq(uid, current_euid()) }
            && !unsafe { ns_capable(ns, CAP_SETUID) }
        {
            return -EPERM;
        }
    }

    /* There must be a destination keyring */
    dest_ref = unsafe { lookup_user_key(destid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE) };
    if unsafe { IS_ERR(dest_ref as *const c_void) } {
        return unsafe { PTR_ERR(dest_ref) };
    }
    if unsafe { (*key_ref_to_ptr(dest_ref)).type_ } != unsafe { &raw mut key_type_keyring } {
        ret = -ENOTDIR;
    } else {
        ret = unsafe { key_get_persistent(ns, uid, dest_ref) };
    }

    unsafe {
        key_ref_put(dest_ref);
    }
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
