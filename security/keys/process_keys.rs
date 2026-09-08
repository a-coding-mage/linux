// SPDX-License-Identifier: GPL-2.0-or-later
/* Manage a process's keyrings
 *
 * Copyright (C) 2004-2005, 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

pub type bool_ = bool;
pub type uid_t = c_uint;
pub type key_perm_t = c_uint;
pub type key_serial_t = c_int;
pub type key_ref_t = *mut c_void;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callback_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_match_data {
    pub cmp: Option<unsafe extern "C" fn(*const key, *const key_match_data) -> bool>,
    pub raw_data: *const c_void,
    pub lookup_type: c_int,
}

#[repr(C)]
pub struct key_index_key {
    pub type_: *mut key_type,
    pub description: *mut c_char,
    pub desc_len: c_int,
}

#[repr(C)]
pub struct keyring_search_context {
    pub index_key: key_index_key,
    pub cred: *const cred,
    pub match_data: key_match_data,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct key_payload {
    pub data: [*mut c_void; 1],
}

#[repr(C)]
pub struct key {
    pub sem: rw_semaphore,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub flags: c_ulong,
    pub payload: key_payload,
    pub index_key: key_index_key,
    pub last_used_at: i64,
    pub serial: c_long,
}

#[repr(C)]
pub struct key_user {
    pub usage: refcount_t,
    pub cons_lock: mutex,
    pub lock: spinlock_t,
    pub nkeys: atomic_t,
    pub nikeys: atomic_t,
    pub uid: kuid_t,
}

#[repr(C)]
pub struct user_struct {
    pub uid: kuid_t,
}

#[repr(C)]
pub struct user_namespace {
    pub user_keyring_register: *mut key,
    pub keyring_sem: rw_semaphore,
    pub owner: kuid_t,
}

#[repr(C)]
pub struct group_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ucounts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub flags: c_ulong,
}

#[repr(C)]
pub struct cred {
    pub thread_keyring: *mut key,
    pub process_keyring: *mut key,
    pub session_keyring: *mut key,
    pub request_key_auth: *mut key,
    pub uid: kuid_t,
    pub euid: kuid_t,
    pub suid: kuid_t,
    pub fsuid: kuid_t,
    pub gid: kgid_t,
    pub egid: kgid_t,
    pub sgid: kgid_t,
    pub fsgid: kgid_t,
    pub user: *mut user_struct,
    pub ucounts: *mut ucounts,
    pub user_ns: *mut user_namespace,
    pub group_info: *mut group_info,
    pub securebits: c_uint,
    pub cap_inheritable: c_ulong,
    pub cap_permitted: c_ulong,
    pub cap_effective: c_ulong,
    pub cap_ambient: c_ulong,
    pub cap_bset: c_ulong,
    pub jit_keyring: c_int,
    pub rcu: callback_head,
}

#[repr(C)]
pub struct request_key_auth {
    pub cred: *const cred,
    pub dest_keyring: *mut key,
}

#[repr(C)]
pub struct kuid_t {
    pub val: c_uint,
}

#[repr(C)]
pub struct kgid_t {
    pub val: c_uint,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: atomic_t,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_need_perm {
    KEY_NEED_UNLINK = 0,
    KEY_AUTHTOKEN_OVERRIDE = 1,
    KEY_DEFER_PERM_CHECK = 2,
}

pub const EAGAIN: c_long = 11;
pub const ENOKEY: c_long = 126;
pub const EACCES: c_long = 13;
pub const EINVAL: c_long = 22;
pub const ENOMEM: c_long = 12;
pub const EKEYREVOKED: c_long = 128;
pub const ERESTARTSYS: c_long = 512;
pub const EIO: c_long = 5;

pub const KEY_POS_WRITE: key_perm_t = 0x04_000000;
pub const KEY_POS_SEARCH: key_perm_t = 0x01_000000;
pub const KEY_POS_VIEW: key_perm_t = 0x01_000000;
pub const KEY_POS_READ: key_perm_t = 0x02_000000;
pub const KEY_POS_LINK: key_perm_t = 0x08_000000;
pub const KEY_POS_SETATTR: key_perm_t = 0x20_000000;
pub const KEY_POS_ALL: key_perm_t = 0x3f_000000;
pub const KEY_USR_VIEW: key_perm_t = 0x00010000;
pub const KEY_USR_READ: key_perm_t = 0x00020000;
pub const KEY_USR_LINK: key_perm_t = 0x00080000;
pub const KEY_USR_ALL: key_perm_t = 0x003f0000;

pub const KEY_ALLOC_UID_KEYRING: c_ulong = 0x20;
pub const KEY_ALLOC_IN_QUOTA: c_ulong = 0x01;
pub const KEY_ALLOC_QUOTA_OVERRUN: c_ulong = 0x04;

pub const KEYRING_SEARCH_LOOKUP_DIRECT: c_int = 0;
pub const KEYRING_SEARCH_DO_STATE_CHECK: c_ulong = 0x0001;
pub const KEYRING_SEARCH_NO_STATE_CHECK: c_ulong = 0x0002;
pub const KEYRING_SEARCH_RECURSE: c_ulong = 0x0004;

pub const KEY_LOOKUP_CREATE: c_ulong = 0x01;
pub const KEY_LOOKUP_PARTIAL: c_ulong = 0x02;

pub const KEY_SPEC_THREAD_KEYRING: key_serial_t = -1;
pub const KEY_SPEC_PROCESS_KEYRING: key_serial_t = -2;
pub const KEY_SPEC_SESSION_KEYRING: key_serial_t = -3;
pub const KEY_SPEC_USER_KEYRING: key_serial_t = -4;
pub const KEY_SPEC_USER_SESSION_KEYRING: key_serial_t = -5;
pub const KEY_SPEC_GROUP_KEYRING: key_serial_t = -6;
pub const KEY_SPEC_REQKEY_AUTH_KEY: key_serial_t = -7;
pub const KEY_SPEC_REQUESTOR_KEYRING: key_serial_t = -8;

pub const KEY_FLAG_UID_KEYRING: c_int = 0;
pub const KEY_FLAG_REVOKED: c_int = 1;
pub const KEY_IS_UNINSTANTIATED: c_int = 0;
pub const PF_EXITING: c_ulong = 0x00000004;
pub const INVALID_GID: kgid_t = kgid_t { val: !0 };
pub const GLOBAL_ROOT_UID: kuid_t = kuid_t { val: 0 };

unsafe extern "C" {
    static mut key_type_keyring: key_type;
    static mut key_type_request_key_auth: key_type;
    static mut current: *mut task_struct;

    fn current_cred() -> *const cred;
    fn get_current_cred() -> *const cred;
    fn current_user_ns() -> *mut user_namespace;
    fn kernel_cred() -> *const cred;
    fn from_kuid(user_ns: *mut user_namespace, uid: kuid_t) -> uid_t;
    fn keyring_alloc(description: *const c_char, uid: kuid_t, gid: kgid_t, cred: *const cred,
                     perm: key_perm_t, flags: c_ulong, restrict_link: *mut c_void,
                     dest: *mut key) -> *mut key;
    fn keyring_search(keyring: key_ref_t, type_: *mut key_type, description: *const c_char,
                      recurse: bool) -> key_ref_t;
    fn keyring_search_rcu(keyring: key_ref_t, ctx: *mut keyring_search_context) -> key_ref_t;
    fn make_key_ref(key: *mut key, possession: bool) -> key_ref_t;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn key_link(keyring: *mut key, key: *mut key) -> c_int;
    fn key_put(key: *mut key);
    fn __key_get(key: *mut key);
    fn key_get(key: *mut key) -> *mut key;
    fn key_ref_put(key_ref: key_ref_t);
    fn key_default_cmp(key: *const key, match_data: *const key_match_data) -> bool;
    fn key_validate(key: *mut key) -> c_int;
    fn key_lookup(id: key_serial_t) -> *mut key;
    fn wait_for_key_construction(key: *mut key, intr: bool) -> c_int;
    fn key_read_state(key: *const key) -> c_int;
    fn key_task_permission(key_ref: key_ref_t, cred: *const cred, need_perm: key_need_perm) -> c_int;
    fn ktime_get_real_seconds() -> i64;
    fn prepare_creds() -> *mut cred;
    fn abort_creds(new: *mut cred);
    fn commit_creds(new: *mut cred) -> c_int;
    fn put_cred(cred: *const cred);
    fn find_keyring_by_name(name: *const c_char, skip_perm_check: bool) -> *mut key;
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn might_sleep();
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool;
    fn get_ucounts(ucounts: *mut ucounts) -> *mut ucounts;
    fn get_uid(user: *mut user_struct) -> *mut user_struct;
    fn get_user_ns(user_ns: *mut user_namespace) -> *mut user_namespace;
    fn get_group_info(group_info: *mut group_info) -> *mut group_info;
    fn security_transfer_creds(new: *mut cred, old: *const cred);
    fn WARN_ONCE(condition: c_int, fmt: *const c_char, ...);
}

#[inline]
unsafe fn READ_ONCE_key(p: *mut *mut key) -> *mut key {
    unsafe { core::ptr::read_volatile(p) }
}

#[inline]
unsafe fn smp_store_release_key(p: *mut *mut key, v: *mut key) {
    unsafe { core::ptr::write_volatile(p, v) }
}

#[inline]
fn ERR_PTR(err: c_long) -> key_ref_t {
    err as isize as key_ref_t
}

#[inline]
fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as isize as c_long
}

#[inline]
fn ERR_CAST<T>(ptr: *mut T) -> key_ref_t {
    ptr as key_ref_t
}

static mut key_session_mutex: mutex = mutex { _private: [] };

/* The root user's tracking struct */
#[unsafe(no_mangle)]
pub static mut root_key_user: key_user = key_user {
    usage: refcount_t { refs: atomic_t { counter: 3 } },
    cons_lock: mutex { _private: [] },
    lock: spinlock_t { _private: [] },
    nkeys: atomic_t { counter: 2 },
    nikeys: atomic_t { counter: 2 },
    uid: GLOBAL_ROOT_UID,
};

/*
 * Get or create a user register keyring.
 */
unsafe fn get_user_register(user_ns: *mut user_namespace) -> *mut key {
    let mut reg_keyring = unsafe { READ_ONCE_key(&mut (*user_ns).user_keyring_register) };

    if !reg_keyring.is_null() {
        return reg_keyring;
    }

    unsafe { down_write(&mut (*user_ns).keyring_sem) };

    /* Make sure there's a register keyring.  It gets owned by the
     * user_namespace's owner.
     */
    reg_keyring = unsafe { (*user_ns).user_keyring_register };
    if reg_keyring.is_null() {
        reg_keyring = unsafe {
            keyring_alloc(
                c".user_reg".as_ptr(),
                (*user_ns).owner,
                INVALID_GID,
                kernel_cred(),
                KEY_POS_WRITE | KEY_POS_SEARCH | KEY_USR_VIEW | KEY_USR_READ,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if !IS_ERR(reg_keyring) {
            unsafe { smp_store_release_key(&mut (*user_ns).user_keyring_register, reg_keyring) };
        }
    }

    unsafe { up_write(&mut (*user_ns).keyring_sem) };

    /* We don't return a ref since the keyring is pinned by the user_ns */
    reg_keyring
}

/*
 * Look up the user and user session keyrings for the current process's UID,
 * creating them if they don't exist.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn look_up_user_keyrings(
    _user_keyring: *mut *mut key,
    _user_session_keyring: *mut *mut key,
) -> c_int {
    let cred = unsafe { current_cred() };
    let user_ns = unsafe { current_user_ns() };
    let mut reg_keyring: *mut key;
    let mut uid_keyring: *mut key;
    let mut session_keyring: *mut key;
    let user_keyring_perm: key_perm_t;
    let mut uid_keyring_r: key_ref_t;
    let mut session_keyring_r: key_ref_t;
    let uid = unsafe { from_kuid(user_ns, (*(*cred).user).uid) };
    let mut buf = [0 as c_char; 20];
    let mut ret: c_int;

    user_keyring_perm = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_ALL;

    reg_keyring = unsafe { get_user_register(user_ns) };
    if IS_ERR(reg_keyring) {
        return PTR_ERR(reg_keyring) as c_int;
    }

    unsafe { down_write(&mut (*user_ns).keyring_sem) };
    ret = 0;

    /* Get the user keyring.  Note that there may be one in existence
     * already as it may have been pinned by a session, but the user_struct
     * pointing to it may have been destroyed by setuid.
     */
    unsafe { snprintf(buf.as_mut_ptr(), buf.len(), c"_uid.%u".as_ptr(), uid) };
    uid_keyring_r = unsafe {
        keyring_search(make_key_ref(reg_keyring, true), &mut key_type_keyring, buf.as_ptr(), false)
    };
    if uid_keyring_r == ERR_PTR(-EAGAIN) {
        uid_keyring = unsafe {
            keyring_alloc(
                buf.as_ptr(),
                (*(*cred).user).uid,
                INVALID_GID,
                cred,
                user_keyring_perm,
                KEY_ALLOC_UID_KEYRING | KEY_ALLOC_IN_QUOTA,
                ptr::null_mut(),
                reg_keyring,
            )
        };
        if IS_ERR(uid_keyring) {
            ret = PTR_ERR(uid_keyring) as c_int;
            goto_error(user_ns, ret);
            return ret;
        }
    } else if IS_ERR(uid_keyring_r) {
        ret = PTR_ERR(uid_keyring_r) as c_int;
        goto_error(user_ns, ret);
        return ret;
    } else {
        uid_keyring = unsafe { key_ref_to_ptr(uid_keyring_r) };
    }

    /* Get a default session keyring (which might also exist already) */
    unsafe { snprintf(buf.as_mut_ptr(), buf.len(), c"_uid_ses.%u".as_ptr(), uid) };
    session_keyring_r = unsafe {
        keyring_search(make_key_ref(reg_keyring, true), &mut key_type_keyring, buf.as_ptr(), false)
    };
    if session_keyring_r == ERR_PTR(-EAGAIN) {
        session_keyring = unsafe {
            keyring_alloc(
                buf.as_ptr(),
                (*(*cred).user).uid,
                INVALID_GID,
                cred,
                user_keyring_perm,
                KEY_ALLOC_UID_KEYRING | KEY_ALLOC_IN_QUOTA,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if IS_ERR(session_keyring) {
            ret = PTR_ERR(session_keyring) as c_int;
            unsafe { key_put(uid_keyring) };
            unsafe { up_write(&mut (*user_ns).keyring_sem) };
            return ret;
        }

        /* We install a link from the user session keyring to
         * the user keyring.
         */
        ret = unsafe { key_link(session_keyring, uid_keyring) };
        if ret < 0 {
            unsafe { key_put(session_keyring) };
            unsafe { key_put(uid_keyring) };
            unsafe { up_write(&mut (*user_ns).keyring_sem) };
            return ret;
        }

        /* And only then link the user-session keyring to the
         * register.
         */
        ret = unsafe { key_link(reg_keyring, session_keyring) };
        if ret < 0 {
            unsafe { key_put(session_keyring) };
            unsafe { key_put(uid_keyring) };
            unsafe { up_write(&mut (*user_ns).keyring_sem) };
            return ret;
        }
    } else if IS_ERR(session_keyring_r) {
        ret = PTR_ERR(session_keyring_r) as c_int;
        unsafe { key_put(uid_keyring) };
        unsafe { up_write(&mut (*user_ns).keyring_sem) };
        return ret;
    } else {
        session_keyring = unsafe { key_ref_to_ptr(session_keyring_r) };
    }

    unsafe { up_write(&mut (*user_ns).keyring_sem) };

    if !_user_session_keyring.is_null() {
        unsafe { *_user_session_keyring = session_keyring };
    } else {
        unsafe { key_put(session_keyring) };
    }
    if !_user_keyring.is_null() {
        unsafe { *_user_keyring = uid_keyring };
    } else {
        unsafe { key_put(uid_keyring) };
    }
    0
}

unsafe fn goto_error(user_ns: *mut user_namespace, ret: c_int) {
    unsafe { up_write(&mut (*user_ns).keyring_sem) };
    let _ = ret;
}

/*
 * Get the user session keyring if it exists, but don't create it if it
 * doesn't.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_user_session_keyring_rcu(cred: *const cred) -> *mut key {
    let reg_keyring = unsafe { READ_ONCE_key(&mut (*(*cred).user_ns).user_keyring_register) };
    let session_keyring_r: key_ref_t;
    let mut buf = [0 as c_char; 20];

    let mut ctx = keyring_search_context {
        index_key: key_index_key {
            type_: unsafe { &mut key_type_keyring },
            description: buf.as_mut_ptr(),
            desc_len: 0,
        },
        cred,
        match_data: key_match_data {
            cmp: Some(key_default_cmp),
            raw_data: buf.as_ptr() as *const c_void,
            lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
        },
        flags: KEYRING_SEARCH_DO_STATE_CHECK,
    };

    if reg_keyring.is_null() {
        return ptr::null_mut();
    }

    ctx.index_key.desc_len = unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"_uid_ses.%u".as_ptr(),
            from_kuid((*cred).user_ns, (*(*cred).user).uid),
        )
    };

    session_keyring_r = unsafe { keyring_search_rcu(make_key_ref(reg_keyring, true), &mut ctx) };
    if IS_ERR(session_keyring_r) {
        return ptr::null_mut();
    }
    unsafe { key_ref_to_ptr(session_keyring_r) }
}

/*
 * Install a thread keyring to the given credentials struct if it didn't have
 * one already.  This is allowed to overrun the quota.
 *
 * Return: 0 if a thread keyring is now present; -errno on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn install_thread_keyring_to_cred(new: *mut cred) -> c_int {
    let keyring: *mut key;

    if unsafe { !(*new).thread_keyring.is_null() } {
        return 0;
    }

    keyring = unsafe {
        keyring_alloc(
            c"_tid".as_ptr(),
            (*new).uid,
            (*new).gid,
            new,
            KEY_POS_ALL | KEY_USR_VIEW,
            KEY_ALLOC_QUOTA_OVERRUN,
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };
    if IS_ERR(keyring) {
        return PTR_ERR(keyring) as c_int;
    }

    unsafe { (*new).thread_keyring = keyring };
    0
}

/*
 * Install a thread keyring to the current task if it didn't have one already.
 *
 * Return: 0 if a thread keyring is now present; -errno on failure.
 */
unsafe fn install_thread_keyring() -> c_int {
    let new: *mut cred;
    let ret: c_int;

    new = unsafe { prepare_creds() };
    if new.is_null() {
        return -(ENOMEM as c_int);
    }

    ret = unsafe { install_thread_keyring_to_cred(new) };
    if ret < 0 {
        unsafe { abort_creds(new) };
        return ret;
    }

    unsafe { commit_creds(new) }
}

/*
 * Install a process keyring to the given credentials struct if it didn't have
 * one already.  This is allowed to overrun the quota.
 *
 * Return: 0 if a process keyring is now present; -errno on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn install_process_keyring_to_cred(new: *mut cred) -> c_int {
    let keyring: *mut key;

    if unsafe { !(*new).process_keyring.is_null() } {
        return 0;
    }

    keyring = unsafe {
        keyring_alloc(
            c"_pid".as_ptr(),
            (*new).uid,
            (*new).gid,
            new,
            KEY_POS_ALL | KEY_USR_VIEW,
            KEY_ALLOC_QUOTA_OVERRUN,
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };
    if IS_ERR(keyring) {
        return PTR_ERR(keyring) as c_int;
    }

    unsafe { (*new).process_keyring = keyring };
    0
}

/*
 * Install a process keyring to the current task if it didn't have one already.
 *
 * Return: 0 if a process keyring is now present; -errno on failure.
 */
unsafe fn install_process_keyring() -> c_int {
    let new: *mut cred;
    let ret: c_int;

    new = unsafe { prepare_creds() };
    if new.is_null() {
        return -(ENOMEM as c_int);
    }

    ret = unsafe { install_process_keyring_to_cred(new) };
    if ret < 0 {
        unsafe { abort_creds(new) };
        return ret;
    }

    unsafe { commit_creds(new) }
}

/*
 * Install the given keyring as the session keyring of the given credentials
 * struct, replacing the existing one if any.  If the given keyring is NULL,
 * then install a new anonymous session keyring.
 * @cred can not be in use by any task yet.
 *
 * Return: 0 on success; -errno on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn install_session_keyring_to_cred(
    cred: *mut cred,
    mut keyring: *mut key,
) -> c_int {
    let mut flags: c_ulong;
    let old: *mut key;

    unsafe { might_sleep() };

    /* create an empty session keyring */
    if keyring.is_null() {
        flags = KEY_ALLOC_QUOTA_OVERRUN;
        if unsafe { !(*cred).session_keyring.is_null() } {
            flags = KEY_ALLOC_IN_QUOTA;
        }

        keyring = unsafe {
            keyring_alloc(
                c"_ses".as_ptr(),
                (*cred).uid,
                (*cred).gid,
                cred,
                KEY_POS_ALL | KEY_USR_VIEW | KEY_USR_READ,
                flags,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if IS_ERR(keyring) {
            return PTR_ERR(keyring) as c_int;
        }
    } else {
        unsafe { __key_get(keyring) };
    }

    /* install the keyring */
    old = unsafe { (*cred).session_keyring };
    unsafe { (*cred).session_keyring = keyring };

    if !old.is_null() {
        unsafe { key_put(old) };
    }

    0
}

unsafe fn install_session_keyring(keyring: *mut key) -> c_int {
    let new: *mut cred;
    let ret: c_int;

    new = unsafe { prepare_creds() };
    if new.is_null() {
        return -(ENOMEM as c_int);
    }

    ret = unsafe { install_session_keyring_to_cred(new, keyring) };
    if ret < 0 {
        unsafe { abort_creds(new) };
        return ret;
    }

    unsafe { commit_creds(new) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_fsuid_changed(new_cred: *mut cred) {
    /* update the ownership of the thread keyring */
    if unsafe { !(*new_cred).thread_keyring.is_null() } {
        unsafe { down_write(&mut (*(*new_cred).thread_keyring).sem) };
        unsafe { (*(*new_cred).thread_keyring).uid = (*new_cred).fsuid };
        unsafe { up_write(&mut (*(*new_cred).thread_keyring).sem) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_fsgid_changed(new_cred: *mut cred) {
    /* update the ownership of the thread keyring */
    if unsafe { !(*new_cred).thread_keyring.is_null() } {
        unsafe { down_write(&mut (*(*new_cred).thread_keyring).sem) };
        unsafe { (*(*new_cred).thread_keyring).gid = (*new_cred).fsgid };
        unsafe { up_write(&mut (*(*new_cred).thread_keyring).sem) };
    }
}

/*
 * Search the process keyrings attached to the supplied cred for the first
 * matching key under RCU conditions (the caller must be holding the RCU read
 * lock).
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn search_cred_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t {
    let mut user_session: *mut key;
    let mut key_ref: key_ref_t;
    let mut ret: key_ref_t;
    let mut err: key_ref_t;
    let cred = unsafe { (*ctx).cred };

    key_ref = ptr::null_mut();
    ret = ptr::null_mut();
    err = ERR_PTR(-EAGAIN);

    if unsafe { !(*cred).thread_keyring.is_null() } {
        key_ref = unsafe { keyring_search_rcu(make_key_ref((*cred).thread_keyring, true), ctx) };
        if !IS_ERR(key_ref) {
            return key_ref;
        }

        match PTR_ERR(key_ref) {
            x if x == -EAGAIN || x == -ENOKEY => ret = key_ref,
            _ => err = key_ref,
        }
    }

    if unsafe { !(*cred).process_keyring.is_null() } {
        key_ref = unsafe { keyring_search_rcu(make_key_ref((*cred).process_keyring, true), ctx) };
        if !IS_ERR(key_ref) {
            return key_ref;
        }

        match PTR_ERR(key_ref) {
            x if x == -EAGAIN => {
                if ret.is_null() {
                    ret = key_ref;
                }
            }
            x if x == -ENOKEY => ret = key_ref,
            _ => err = key_ref,
        }
    }

    if unsafe { !(*cred).session_keyring.is_null() } {
        key_ref = unsafe { keyring_search_rcu(make_key_ref((*cred).session_keyring, true), ctx) };

        if !IS_ERR(key_ref) {
            return key_ref;
        }

        match PTR_ERR(key_ref) {
            x if x == -EAGAIN => {
                if ret.is_null() {
                    ret = key_ref;
                }
            }
            x if x == -ENOKEY => ret = key_ref,
            _ => err = key_ref,
        }
    } else {
        user_session = unsafe { get_user_session_keyring_rcu(cred) };
        if !user_session.is_null() {
            key_ref = unsafe { keyring_search_rcu(make_key_ref(user_session, true), ctx) };
            unsafe { key_put(user_session) };

            if !IS_ERR(key_ref) {
                return key_ref;
            }

            match PTR_ERR(key_ref) {
                x if x == -EAGAIN => {
                    if ret.is_null() {
                        ret = key_ref;
                    }
                }
                x if x == -ENOKEY => ret = key_ref,
                _ => err = key_ref,
            }
        }
    }

    /* no key - decide on the error we're going to go for */
    if !ret.is_null() { ret } else { err }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn search_process_keyrings_rcu(
    ctx: *mut keyring_search_context,
) -> key_ref_t {
    let rka: *mut request_key_auth;
    let mut key_ref: key_ref_t;
    let mut ret: key_ref_t = ERR_PTR(-EACCES);
    let err: key_ref_t;

    key_ref = unsafe { search_cred_keyrings_rcu(ctx) };
    if !IS_ERR(key_ref) {
        return key_ref;
    }
    err = key_ref;

    if unsafe {
        !(*(*ctx).cred).request_key_auth.is_null()
            && (*ctx).cred == current_cred()
            && (*ctx).index_key.type_ != &mut key_type_request_key_auth
    } {
        let cred = unsafe { (*ctx).cred };

        if unsafe { key_validate((*cred).request_key_auth) == 0 } {
            rka = unsafe { (*(*cred).request_key_auth).payload.data[0] as *mut request_key_auth };

            //// was search_process_keyrings() [ie. recursive]
            unsafe { (*ctx).cred = (*rka).cred };
            key_ref = unsafe { search_cred_keyrings_rcu(ctx) };
            unsafe { (*ctx).cred = cred };

            if !IS_ERR(key_ref) {
                return key_ref;
            }
            ret = key_ref;
        }
    }

    /* no key - decide on the error we're going to go for */
    if err == ERR_PTR(-ENOKEY) || ret == ERR_PTR(-ENOKEY) {
        ERR_PTR(-ENOKEY)
    } else if err == ERR_PTR(-EACCES) {
        ret
    } else {
        err
    }
}

/*
 * See if the key we're looking at is the target key.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_user_key_possessed(
    key: *const key,
    match_data: *const key_match_data,
) -> bool {
    key == unsafe { (*match_data).raw_data as *const crate::key }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_user_key(
    id: key_serial_t,
    lflags: c_ulong,
    need_perm: key_need_perm,
) -> key_ref_t {
    loop {
        let mut ctx = keyring_search_context {
            index_key: key_index_key {
                type_: ptr::null_mut(),
                description: ptr::null_mut(),
                desc_len: 0,
            },
            cred: unsafe { get_current_cred() },
            match_data: key_match_data {
                cmp: Some(lookup_user_key_possessed),
                raw_data: ptr::null(),
                lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
            },
            flags: KEYRING_SEARCH_NO_STATE_CHECK | KEYRING_SEARCH_RECURSE,
        };
        let mut rka: *mut request_key_auth;
        let mut key: *mut key = ptr::null_mut();
        let mut key_ref: key_ref_t = ERR_PTR(-ENOKEY);
        let skey_ref: key_ref_t;
        let mut ret: c_int;
        let mut reget_creds = false;

        match id {
            KEY_SPEC_THREAD_KEYRING => {
                if unsafe { (*ctx.cred).thread_keyring.is_null() } {
                    if (lflags & KEY_LOOKUP_CREATE) == 0 {
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }

                    ret = unsafe { install_thread_keyring() };
                    if ret < 0 {
                        key_ref = ERR_PTR(ret as c_long);
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    reget_creds = true;
                } else {
                    key = unsafe { (*ctx.cred).thread_keyring };
                    unsafe { __key_get(key) };
                    key_ref = unsafe { make_key_ref(key, true) };
                }
            }
            KEY_SPEC_PROCESS_KEYRING => {
                if unsafe { (*ctx.cred).process_keyring.is_null() } {
                    if (lflags & KEY_LOOKUP_CREATE) == 0 {
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }

                    ret = unsafe { install_process_keyring() };
                    if ret < 0 {
                        key_ref = ERR_PTR(ret as c_long);
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    reget_creds = true;
                } else {
                    key = unsafe { (*ctx.cred).process_keyring };
                    unsafe { __key_get(key) };
                    key_ref = unsafe { make_key_ref(key, true) };
                }
            }
            KEY_SPEC_SESSION_KEYRING => {
                if unsafe { (*ctx.cred).session_keyring.is_null() } {
                    let mut user_session: *mut key = ptr::null_mut();
                    ret = unsafe { look_up_user_keyrings(ptr::null_mut(), &mut user_session) };
                    if ret < 0 {
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    if (lflags & KEY_LOOKUP_CREATE) != 0 {
                        ret = unsafe { join_session_keyring(ptr::null()) as c_int };
                    } else {
                        ret = unsafe { install_session_keyring(user_session) };
                    }

                    unsafe { key_put(user_session) };
                    if ret < 0 {
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    reget_creds = true;
                } else if unsafe {
                    test_bit(KEY_FLAG_UID_KEYRING, &(*(*ctx.cred).session_keyring).flags)
                        && (lflags & KEY_LOOKUP_CREATE) != 0
                } {
                    ret = unsafe { join_session_keyring(ptr::null()) as c_int };
                    if ret < 0 {
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    reget_creds = true;
                } else {
                    key = unsafe { (*ctx.cred).session_keyring };
                    unsafe { __key_get(key) };
                    key_ref = unsafe { make_key_ref(key, true) };
                }
            }
            KEY_SPEC_USER_KEYRING => {
                ret = unsafe { look_up_user_keyrings(&mut key, ptr::null_mut()) };
                if ret < 0 {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }
                key_ref = unsafe { make_key_ref(key, true) };
            }
            KEY_SPEC_USER_SESSION_KEYRING => {
                ret = unsafe { look_up_user_keyrings(ptr::null_mut(), &mut key) };
                if ret < 0 {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }
                key_ref = unsafe { make_key_ref(key, true) };
            }
            KEY_SPEC_GROUP_KEYRING => {
                key_ref = ERR_PTR(-EINVAL);
                unsafe { put_cred(ctx.cred) };
                return key_ref;
            }
            KEY_SPEC_REQKEY_AUTH_KEY => {
                key = unsafe { (*ctx.cred).request_key_auth };
                if key.is_null() {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }

                unsafe { __key_get(key) };
                key_ref = unsafe { make_key_ref(key, true) };
            }
            KEY_SPEC_REQUESTOR_KEYRING => {
                if unsafe { (*ctx.cred).request_key_auth.is_null() } {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }

                unsafe { down_read(&mut (*(*ctx.cred).request_key_auth).sem) };
                if unsafe { test_bit(KEY_FLAG_REVOKED, &(*(*ctx.cred).request_key_auth).flags) } {
                    key_ref = ERR_PTR(-EKEYREVOKED);
                    key = ptr::null_mut();
                } else {
                    rka = unsafe {
                        (*(*ctx.cred).request_key_auth).payload.data[0] as *mut request_key_auth
                    };
                    key = unsafe { (*rka).dest_keyring };
                    unsafe { __key_get(key) };
                }
                unsafe { up_read(&mut (*(*ctx.cred).request_key_auth).sem) };
                if key.is_null() {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }
                key_ref = unsafe { make_key_ref(key, true) };
            }
            _ => {
                key_ref = ERR_PTR(-EINVAL);
                if id < 1 {
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }

                key = unsafe { key_lookup(id) };
                if IS_ERR(key) {
                    key_ref = ERR_CAST(key);
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }

                key_ref = unsafe { make_key_ref(key, false) };

                /* check to see if we possess the key */
                unsafe { ctx.index_key = (*key).index_key };
                ctx.match_data.raw_data = key as *const c_void;
                unsafe { rcu_read_lock() };
                skey_ref = unsafe { search_process_keyrings_rcu(&mut ctx) };
                unsafe { rcu_read_unlock() };

                if !IS_ERR(skey_ref) {
                    unsafe { key_put(key) };
                    key_ref = skey_ref;
                }
            }
        }

        if reget_creds {
            unsafe { put_cred(ctx.cred) };
            continue;
        }

        if need_perm != key_need_perm::KEY_NEED_UNLINK {
            if (lflags & KEY_LOOKUP_PARTIAL) == 0 {
                ret = unsafe { wait_for_key_construction(key, true) };
                match ret {
                    x if x == -(ERESTARTSYS as c_int) => {
                        unsafe { key_ref_put(key_ref) };
                        key_ref = ERR_PTR(ret as c_long);
                        unsafe { put_cred(ctx.cred) };
                        return key_ref;
                    }
                    0 => {}
                    _ => {
                        if need_perm != key_need_perm::KEY_AUTHTOKEN_OVERRIDE
                            && need_perm != key_need_perm::KEY_DEFER_PERM_CHECK
                        {
                            unsafe { key_ref_put(key_ref) };
                            key_ref = ERR_PTR(ret as c_long);
                            unsafe { put_cred(ctx.cred) };
                            return key_ref;
                        }
                    }
                }
            } else if need_perm != key_need_perm::KEY_DEFER_PERM_CHECK {
                ret = unsafe { key_validate(key) };
                if ret < 0 {
                    unsafe { key_ref_put(key_ref) };
                    key_ref = ERR_PTR(ret as c_long);
                    unsafe { put_cred(ctx.cred) };
                    return key_ref;
                }
            }

            ret = -(EIO as c_int);
            if (lflags & KEY_LOOKUP_PARTIAL) == 0
                && unsafe { key_read_state(key) == KEY_IS_UNINSTANTIATED }
            {
                unsafe { key_ref_put(key_ref) };
                key_ref = ERR_PTR(ret as c_long);
                unsafe { put_cred(ctx.cred) };
                return key_ref;
            }
        }

        ret = unsafe { key_task_permission(key_ref, ctx.cred, need_perm) };
        if ret < 0 {
            unsafe { key_ref_put(key_ref) };
            key_ref = ERR_PTR(ret as c_long);
            unsafe { put_cred(ctx.cred) };
            return key_ref;
        }

        unsafe { (*key).last_used_at = ktime_get_real_seconds() };

        unsafe { put_cred(ctx.cred) };
        return key_ref;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn join_session_keyring(name: *const c_char) -> c_long {
    let old: *const cred;
    let new: *mut cred;
    let mut keyring: *mut key;
    let mut ret: c_long;
    let serial: c_long;

    new = unsafe { prepare_creds() };
    if new.is_null() {
        return -ENOMEM;
    }
    old = unsafe { current_cred() };

    /* if no name is provided, install an anonymous keyring */
    if name.is_null() {
        ret = unsafe { install_session_keyring_to_cred(new, ptr::null_mut()) as c_long };
        if ret < 0 {
            unsafe { abort_creds(new) };
            return ret;
        }

        serial = unsafe { (*(*new).session_keyring).serial };
        ret = unsafe { commit_creds(new) as c_long };
        if ret == 0 {
            ret = serial;
        }
        return ret;
    }

    /* allow the user to join or create a named keyring */
    unsafe { mutex_lock(&mut key_session_mutex) };

    /* look for an existing keyring of this name */
    keyring = unsafe { find_keyring_by_name(name, false) };
    if PTR_ERR(keyring) == -ENOKEY {
        /* not found - try and create a new one */
        keyring = unsafe {
            keyring_alloc(
                name,
                (*old).uid,
                (*old).gid,
                old,
                KEY_POS_ALL | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_LINK,
                KEY_ALLOC_IN_QUOTA,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if IS_ERR(keyring) {
            ret = PTR_ERR(keyring);
            unsafe { mutex_unlock(&mut key_session_mutex) };
            unsafe { abort_creds(new) };
            return ret;
        }
    } else if IS_ERR(keyring) {
        ret = PTR_ERR(keyring);
        unsafe { mutex_unlock(&mut key_session_mutex) };
        unsafe { abort_creds(new) };
        return ret;
    } else if keyring == unsafe { (*new).session_keyring } {
        ret = 0;
        unsafe { key_put(keyring) };
        unsafe { mutex_unlock(&mut key_session_mutex) };
        unsafe { abort_creds(new) };
        return ret;
    }

    /* we've got a keyring - now to install it */
    ret = unsafe { install_session_keyring_to_cred(new, keyring) as c_long };
    if ret < 0 {
        unsafe { key_put(keyring) };
        unsafe { mutex_unlock(&mut key_session_mutex) };
        unsafe { abort_creds(new) };
        return ret;
    }

    unsafe { commit_creds(new) };
    unsafe { mutex_unlock(&mut key_session_mutex) };

    ret = unsafe { (*keyring).serial };
    unsafe { key_put(keyring) };
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn key_change_session_keyring(twork: *mut callback_head) {
    let old = unsafe { current_cred() };
    let new = (twork as *mut u8).wrapping_sub(core::mem::offset_of!(cred, rcu)) as *mut cred;

    if unsafe { !current.is_null() && ((*current).flags & PF_EXITING) != 0 } {
        unsafe { put_cred(new) };
        return;
    }

    /* If get_ucounts fails more bits are needed in the refcount */
    if unsafe { get_ucounts((*old).ucounts).is_null() } {
        unsafe { WARN_ONCE(1, c"In %s get_ucounts failed\n".as_ptr(), c"key_change_session_keyring".as_ptr()) };
        unsafe { put_cred(new) };
        return;
    }

    unsafe {
        (*new).uid = (*old).uid;
        (*new).euid = (*old).euid;
        (*new).suid = (*old).suid;
        (*new).fsuid = (*old).fsuid;
        (*new).gid = (*old).gid;
        (*new).egid = (*old).egid;
        (*new).sgid = (*old).sgid;
        (*new).fsgid = (*old).fsgid;
        (*new).user = get_uid((*old).user);
        (*new).ucounts = (*old).ucounts;
        (*new).user_ns = get_user_ns((*old).user_ns);
        (*new).group_info = get_group_info((*old).group_info);

        (*new).securebits = (*old).securebits;
        (*new).cap_inheritable = (*old).cap_inheritable;
        (*new).cap_permitted = (*old).cap_permitted;
        (*new).cap_effective = (*old).cap_effective;
        (*new).cap_ambient = (*old).cap_ambient;
        (*new).cap_bset = (*old).cap_bset;

        (*new).jit_keyring = (*old).jit_keyring;
        (*new).thread_keyring = key_get((*old).thread_keyring);
        (*new).process_keyring = key_get((*old).process_keyring);

        security_transfer_creds(new, old);

        commit_creds(new);
    }
}

/*
 * Make sure that root's user and user-session keyrings exist.
 */
unsafe fn init_root_keyring() -> c_int {
    unsafe { look_up_user_keyrings(ptr::null_mut(), ptr::null_mut()) }
}

/* late_initcall(init_root_keyring); */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
