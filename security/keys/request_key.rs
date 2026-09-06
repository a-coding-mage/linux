// SPDX-License-Identifier: GPL-2.0-or-later
/* Request a key from userspace
 *
 * Copyright (C) 2004-2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * See Documentation/security/keys/request-key.rst
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type size_t = usize;
type key_serial_t = c_int;
type key_perm_t = c_uint;
type key_ref_t = *mut c_void;
type request_key_actor_t = unsafe extern "C" fn(*mut key, *mut c_void) -> c_int;
type bool_t = bool;

const key_negative_timeout: c_uint = 60; /* default timeout on a negative key's existence */

const KEY_FLAG_INVALIDATED: c_int = 0;
const KEY_FLAG_REVOKED: c_int = 1;
const KEY_FLAG_USER_CONSTRUCT: c_int = 2;
const PF_KTHREAD: c_uint = 0x00200000;
const TIF_NOTIFY_RESUME: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const UMH_WAIT_PROC: c_int = 1;
const ENOMEM: c_int = 12;
const ENOKEY: c_int = 126;
const EINVAL: c_int = 22;
const EINPROGRESS: c_int = 115;
const EPERM: c_int = 1;
const EAGAIN: c_int = 11;
const ERESTARTSYS: c_int = 512;
const TASK_INTERRUPTIBLE: c_uint = 1;
const TASK_UNINTERRUPTIBLE: c_uint = 2;

const KEY_POS_VIEW: key_perm_t = 0x01000000;
const KEY_POS_READ: key_perm_t = 0x02000000;
const KEY_POS_WRITE: key_perm_t = 0x04000000;
const KEY_POS_SEARCH: key_perm_t = 0x08000000;
const KEY_POS_LINK: key_perm_t = 0x10000000;
const KEY_POS_SETATTR: key_perm_t = 0x20000000;
const KEY_POS_ALL: key_perm_t = 0x3f000000;
const KEY_USR_VIEW: key_perm_t = 0x00010000;
const KEY_USR_READ: key_perm_t = 0x00020000;
const KEY_NEED_WRITE: key_perm_t = 0x04;
const KEY_NEED_LINK: key_perm_t = 0x10;
const KEY_ALLOC_QUOTA_OVERRUN: c_ulong = 0x1;
const KEY_ALLOC_IN_QUOTA: c_ulong = 0x0;

const KEYRING_SEARCH_LOOKUP_DIRECT: c_int = 0;
const KEYRING_SEARCH_DO_STATE_CHECK: c_uint = 0x0001;
const KEYRING_SEARCH_SKIP_EXPIRED: c_uint = 0x0002;
const KEYRING_SEARCH_RECURSE: c_uint = 0x0004;

const KEY_REQKEY_DEFL_DEFAULT: c_int = 0;
const KEY_REQKEY_DEFL_THREAD_KEYRING: c_int = 1;
const KEY_REQKEY_DEFL_PROCESS_KEYRING: c_int = 2;
const KEY_REQKEY_DEFL_SESSION_KEYRING: c_int = 3;
const KEY_REQKEY_DEFL_USER_KEYRING: c_int = 4;
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: c_int = 5;
const KEY_REQKEY_DEFL_GROUP_KEYRING: c_int = 6;
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: c_int = 7;

#[repr(C)]
pub struct key {
    pub serial: key_serial_t,
    pub flags: c_ulong,
    pub sem: c_void,
    pub type_: *mut key_type,
    pub index_key: keyring_index_key,
}

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub read: Option<unsafe extern "C" fn()>,
    pub update: Option<unsafe extern "C" fn()>,
    pub request_key: Option<request_key_actor_t>,
    pub match_preparse: Option<unsafe extern "C" fn(*mut key_match_data) -> c_int>,
    pub match_free: Option<unsafe extern "C" fn(*mut key_match_data)>,
}

#[repr(C)]
pub struct key_tag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_user {
    pub cons_lock: c_void,
}

#[repr(C)]
pub struct assoc_array_edit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct keyring_index_key {
    pub type_: *mut key_type,
    pub domain_tag: *mut key_tag,
    pub description: *const c_char,
    pub desc_len: size_t,
}

#[repr(C)]
pub struct key_match_data {
    pub cmp: Option<unsafe extern "C" fn(*mut key, *mut key_match_data) -> bool_t>,
    pub raw_data: *const c_void,
    pub lookup_type: c_int,
}

#[repr(C)]
pub struct keyring_search_context {
    pub index_key: keyring_index_key,
    pub cred: *const cred,
    pub match_data: key_match_data,
    pub flags: c_uint,
}

#[repr(C)]
pub struct request_key_auth {
    pub target_key: *mut key,
    pub dest_keyring: *mut key,
    pub op: *const c_char,
}

#[repr(C)]
pub struct task_struct {
    pub cached_requested_key: *mut key,
    pub flags: c_uint,
}

#[repr(C)]
pub struct subprocess_info {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct cred {
    pub fsuid: c_uint,
    pub fsgid: c_uint,
    pub jit_keyring: c_int,
    pub request_key_auth: *mut key,
    pub thread_keyring: *mut key,
    pub process_keyring: *mut key,
    pub session_keyring: *mut key,
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut init_user_ns: c_void;
    static mut key_type_keyring: key_type;
    static mut key_construction_mutex: c_void;

    fn get_request_key_auth(authkey: *mut key) -> *mut request_key_auth;
    fn key_get(key: *mut key) -> *mut key;
    fn key_put(key: *mut key);
    fn key_negate_and_link(key: *mut key, timeout: c_uint, payload: *const c_void, authkey: *mut key) -> c_int;
    fn key_revoke(key: *mut key) -> c_int;
    fn install_session_keyring_to_cred(cred: *mut cred, keyring: *mut key) -> c_int;
    fn call_usermodehelper_setup(
        path: *const c_char,
        argv: *mut *mut c_char,
        envp: *mut *mut c_char,
        gfp_mask: c_uint,
        init: Option<unsafe extern "C" fn(*mut subprocess_info, *mut cred) -> c_int>,
        cleanup: Option<unsafe extern "C" fn(*mut subprocess_info)>,
        data: *mut c_void,
    ) -> *mut subprocess_info;
    fn call_usermodehelper_exec(info: *mut subprocess_info, wait: c_int) -> c_int;
    fn current_cred() -> *const cred;
    fn get_current_cred() -> *const cred;
    fn put_cred(cred: *const cred);
    fn look_up_user_keyrings(user: *mut *mut key, user_session: *mut *mut key) -> c_int;
    fn keyring_alloc(
        desc: *const c_char,
        fsuid: c_uint,
        fsgid: c_uint,
        cred: *const cred,
        perm: key_perm_t,
        flags: c_ulong,
        restrict_link: *const c_void,
        dest: *const c_void,
    ) -> *mut key;
    fn key_link(keyring: *mut key, key: *mut key) -> c_int;
    fn from_kuid(ns: *mut c_void, kuid: c_uint) -> c_uint;
    fn from_kgid(ns: *mut c_void, kgid: c_uint) -> c_uint;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn key_validate(key: *mut key) -> c_int;
    fn request_key_auth_new(
        target: *mut key,
        op: *const c_char,
        callout_info: *const c_void,
        callout_len: size_t,
        dest_keyring: *mut key,
    ) -> *mut key;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool_t;
    fn down_read(sem: *mut c_void);
    fn up_read(sem: *mut c_void);
    fn key_permission(key_ref: key_ref_t, perm: key_perm_t) -> c_int;
    fn make_key_ref(key: *mut key, possession: c_int) -> key_ref_t;
    fn key_serial(key: *mut key) -> key_serial_t;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn key_alloc(
        type_: *mut key_type,
        desc: *const c_char,
        fsuid: c_uint,
        fsgid: c_uint,
        cred: *const cred,
        perm: key_perm_t,
        flags: c_ulong,
        restrict_link: *const c_void,
    ) -> *mut key;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn __key_link_lock(keyring: *mut key, index_key: *const keyring_index_key) -> c_int;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn search_process_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
    fn __key_link_begin(keyring: *mut key, index_key: *const keyring_index_key, edit: *mut *mut assoc_array_edit) -> c_int;
    fn __key_link(keyring: *mut key, key: *mut key, edit: *mut *mut assoc_array_edit);
    fn __key_link_end(keyring: *mut key, index_key: *const keyring_index_key, edit: *mut assoc_array_edit);
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn __key_link_check_live_key(keyring: *mut key, key: *mut key) -> c_int;
    fn key_user_lookup(uid: c_uint) -> *mut key_user;
    fn current_fsuid() -> c_uint;
    fn key_user_put(user: *mut key_user);
    fn strlen(s: *const c_char) -> size_t;
    fn key_default_cmp(key: *mut key, match_data: *mut key_match_data) -> bool_t;
    fn key_task_permission(key_ref: key_ref_t, cred: *const cred, perm: key_perm_t) -> c_int;
    fn key_ref_put(key_ref: key_ref_t);
    fn wait_on_bit(word: *mut c_ulong, bit: c_int, mode: c_uint) -> c_int;
    fn key_read_state(key: *mut key) -> c_int;
    fn set_tsk_thread_flag(task: *mut task_struct, flag: c_int);
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_long {
    ptr as c_long
}

unsafe fn ERR_PTR<T>(err: c_long) -> *mut T {
    err as *mut T
}

unsafe fn ERR_CAST<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}

unsafe fn WARN_ON(condition: bool) -> bool {
    condition
}

unsafe fn kenter(_fmt: *const c_char) {}
unsafe fn kleave(_fmt: *const c_char) {}
unsafe fn kdebug(_fmt: *const c_char) {}

unsafe fn check_cached_key(ctx: *mut keyring_search_context) -> *mut key {
    /*
     * CONFIG_KEYS_REQUEST_CACHE:
     * struct key *key = current->cached_requested_key;
     */
    let key = if !current.is_null() {
        (*current).cached_requested_key
    } else {
        ptr::null_mut()
    };

    if !key.is_null()
        && ((*ctx).match_data.cmp.unwrap())(key, &mut (*ctx).match_data)
        && ((*key).flags
            & ((1_u64 << KEY_FLAG_INVALIDATED) as c_ulong
                | (1_u64 << KEY_FLAG_REVOKED) as c_ulong))
            == 0
    {
        return key_get(key);
    }
    ptr::null_mut()
}

unsafe fn cache_requested_key(key: *mut key) {
    /*
     * CONFIG_KEYS_REQUEST_CACHE:
     * Do not cache key if it is a kernel thread.
     */
    let t = current;
    if !t.is_null() && ((*t).flags & PF_KTHREAD) == 0 {
        key_put((*t).cached_requested_key);
        (*t).cached_requested_key = key_get(key);
        set_tsk_thread_flag(t, TIF_NOTIFY_RESUME);
    }
}

/**
 * complete_request_key - Complete the construction of a key.
 * @authkey: The authorisation key.
 * @error: The success or failute of the construction.
 *
 * Complete the attempt to construct a key.  The key will be negated
 * if an error is indicated.  The authorisation key will be revoked
 * unconditionally.
 */
#[no_mangle]
pub unsafe extern "C" fn complete_request_key(authkey: *mut key, error: c_int) {
    let rka = get_request_key_auth(authkey);
    let key = (*rka).target_key;

    kenter(c"%d{%d},%d".as_ptr());

    if error < 0 {
        key_negate_and_link(key, key_negative_timeout, ptr::null(), authkey);
    } else {
        key_revoke(authkey);
    }
}

/*
 * Initialise a usermode helper that is going to have a specific session
 * keyring.
 *
 * This is called in context of freshly forked kthread before kernel_execve(),
 * so we can simply install the desired session_keyring at this point.
 */
unsafe extern "C" fn umh_keys_init(info: *mut subprocess_info, cred: *mut cred) -> c_int {
    let keyring = (*info).data as *mut key;

    install_session_keyring_to_cred(cred, keyring)
}

/*
 * Clean up a usermode helper with session keyring.
 */
unsafe extern "C" fn umh_keys_cleanup(info: *mut subprocess_info) {
    let keyring = (*info).data as *mut key;
    key_put(keyring);
}

/*
 * Call a usermode helper with a specific session keyring.
 */
unsafe fn call_usermodehelper_keys(
    path: *const c_char,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
    session_keyring: *mut key,
    wait: c_int,
) -> c_int {
    let info = call_usermodehelper_setup(
        path,
        argv,
        envp,
        GFP_KERNEL,
        Some(umh_keys_init),
        Some(umh_keys_cleanup),
        session_keyring as *mut c_void,
    );
    if info.is_null() {
        return -ENOMEM;
    }

    key_get(session_keyring);
    call_usermodehelper_exec(info, wait)
}

/*
 * Request userspace finish the construction of a key
 * - execute "/sbin/request-key <op> <key> <uid> <gid> <keyring> <keyring> <keyring>"
 */
unsafe extern "C" fn call_sbin_request_key(authkey: *mut key, aux: *mut c_void) -> c_int {
    static request_key_path: &[u8] = b"/sbin/request-key\0";
    let rka = get_request_key_auth(authkey);
    let mut credp = current_cred();
    let mut prkey: key_serial_t;
    let sskey: key_serial_t;
    let keyp = (*rka).target_key;
    let mut keyring: *mut key;
    let mut session: *mut key;
    let mut user_session: *mut key = ptr::null_mut();
    let mut argv: [*mut c_char; 9] = [ptr::null_mut(); 9];
    let mut envp: [*mut c_char; 3] = [ptr::null_mut(); 3];
    let mut uid_str: [c_char; 12] = [0; 12];
    let mut gid_str: [c_char; 12] = [0; 12];
    let mut key_str: [c_char; 12] = [0; 12];
    let mut keyring_str: [[c_char; 12]; 3] = [[0; 12]; 3];
    let mut desc: [c_char; 20] = [0; 20];
    let mut ret: c_int;
    let mut i: usize;

    let _ = aux;
    kenter(c"{%d},{%d},%s".as_ptr());

    ret = look_up_user_keyrings(ptr::null_mut(), &mut user_session);
    if ret < 0 {
        complete_request_key(authkey, ret);
        kleave(c" = %d".as_ptr());
        return ret;
    }

    /* allocate a new session keyring */
    sprintf(desc.as_mut_ptr(), c"_req.%u".as_ptr(), (*keyp).serial as c_uint);

    credp = get_current_cred();
    keyring = keyring_alloc(
        desc.as_ptr(),
        (*credp).fsuid,
        (*credp).fsgid,
        credp,
        KEY_POS_ALL | KEY_USR_VIEW | KEY_USR_READ,
        KEY_ALLOC_QUOTA_OVERRUN,
        ptr::null(),
        ptr::null(),
    );
    put_cred(credp);
    if IS_ERR(keyring) {
        ret = PTR_ERR(keyring) as c_int;
        key_put(user_session);
        complete_request_key(authkey, ret);
        kleave(c" = %d".as_ptr());
        return ret;
    }

    /* attach the auth key to the session keyring */
    ret = key_link(keyring, authkey);
    if ret < 0 {
        key_put(keyring);
        key_put(user_session);
        complete_request_key(authkey, ret);
        kleave(c" = %d".as_ptr());
        return ret;
    }

    /* record the UID and GID */
    sprintf(uid_str.as_mut_ptr(), c"%d".as_ptr(), from_kuid(&raw mut init_user_ns, (*credp).fsuid));
    sprintf(gid_str.as_mut_ptr(), c"%d".as_ptr(), from_kgid(&raw mut init_user_ns, (*credp).fsgid));

    /* we say which key is under construction */
    sprintf(key_str.as_mut_ptr(), c"%d".as_ptr(), (*keyp).serial);

    /* we specify the process's default keyrings */
    sprintf(
        keyring_str[0].as_mut_ptr(),
        c"%d".as_ptr(),
        if !(*credp).thread_keyring.is_null() {
            (*(*credp).thread_keyring).serial
        } else {
            0
        },
    );

    prkey = 0;
    if !(*credp).process_keyring.is_null() {
        prkey = (*(*credp).process_keyring).serial;
    }
    sprintf(keyring_str[1].as_mut_ptr(), c"%d".as_ptr(), prkey);

    session = (*credp).session_keyring;
    if session.is_null() {
        session = user_session;
    }
    sskey = (*session).serial;

    sprintf(keyring_str[2].as_mut_ptr(), c"%d".as_ptr(), sskey);

    /* set up a minimal environment */
    i = 0;
    envp[i] = c"HOME=/".as_ptr() as *mut c_char;
    i += 1;
    envp[i] = c"PATH=/sbin:/bin:/usr/sbin:/usr/bin".as_ptr() as *mut c_char;
    i += 1;
    envp[i] = ptr::null_mut();

    /* set up the argument list */
    i = 0;
    argv[i] = request_key_path.as_ptr() as *mut c_char;
    i += 1;
    argv[i] = (*rka).op as *mut c_char;
    i += 1;
    argv[i] = key_str.as_mut_ptr();
    i += 1;
    argv[i] = uid_str.as_mut_ptr();
    i += 1;
    argv[i] = gid_str.as_mut_ptr();
    i += 1;
    argv[i] = keyring_str[0].as_mut_ptr();
    i += 1;
    argv[i] = keyring_str[1].as_mut_ptr();
    i += 1;
    argv[i] = keyring_str[2].as_mut_ptr();
    i += 1;
    argv[i] = ptr::null_mut();

    /* do it */
    ret = call_usermodehelper_keys(
        request_key_path.as_ptr() as *const c_char,
        argv.as_mut_ptr(),
        envp.as_mut_ptr(),
        keyring,
        UMH_WAIT_PROC,
    );
    kdebug(c"usermode -> 0x%x".as_ptr());
    if ret >= 0 {
        /* ret is the exit/wait code */
        if test_bit(KEY_FLAG_USER_CONSTRUCT, &(*keyp).flags) || key_validate(keyp) < 0 {
            ret = -ENOKEY;
        } else {
            /* ignore any errors from userspace if the key was
             * instantiated */
            ret = 0;
        }
    }

    key_put(keyring);
    key_put(user_session);
    complete_request_key(authkey, ret);
    kleave(c" = %d".as_ptr());
    ret
}

/*
 * Call out to userspace for key construction.
 *
 * Program failure is ignored in favour of key status.
 */
unsafe fn construct_key(
    keyp: *mut key,
    callout_info: *const c_void,
    callout_len: size_t,
    aux: *mut c_void,
    dest_keyring: *mut key,
) -> c_int {
    let mut actor: request_key_actor_t;
    let authkey: *mut key;
    let ret: c_int;

    kenter(c"%d,%p,%zu,%p".as_ptr());

    /* allocate an authorisation key */
    authkey = request_key_auth_new(keyp, c"create".as_ptr(), callout_info, callout_len, dest_keyring);
    if IS_ERR(authkey) {
        return PTR_ERR(authkey) as c_int;
    }

    /* Make the call */
    actor = call_sbin_request_key;
    if let Some(request_key) = (*(*keyp).type_).request_key {
        actor = request_key;
    }

    ret = actor(authkey, aux);

    /* check that the actor called complete_request_key() prior to
     * returning an error */
    WARN_ON(ret < 0 && !test_bit(KEY_FLAG_INVALIDATED, &(*authkey).flags));

    key_put(authkey);
    kleave(c" = %d".as_ptr());
    ret
}

/*
 * Get the appropriate destination keyring for the request.
 *
 * The keyring selected is returned with an extra reference upon it which the
 * caller must release.
 */
unsafe fn construct_get_dest_keyring(_dest_keyring: *mut *mut key) -> c_int {
    let mut rka: *mut request_key_auth;
    let credp = current_cred();
    let mut dest_keyring = *_dest_keyring;
    let mut authkey: *mut key;
    let mut ret: c_int;

    kenter(c"%p".as_ptr());

    /* find the appropriate keyring */
    if !dest_keyring.is_null() {
        /* the caller supplied one */
        key_get(dest_keyring);
    } else {
        let mut do_perm_check = true;

        /* use a default keyring; falling through the cases until we
         * find one that we actually have */
        if (*credp).jit_keyring == KEY_REQKEY_DEFL_DEFAULT
            || (*credp).jit_keyring == KEY_REQKEY_DEFL_REQUESTOR_KEYRING
        {
            if !(*credp).request_key_auth.is_null() {
                authkey = (*credp).request_key_auth;
                down_read(&mut (*authkey).sem);
                rka = get_request_key_auth(authkey);
                if !test_bit(KEY_FLAG_REVOKED, &(*authkey).flags) {
                    dest_keyring = key_get((*rka).dest_keyring);
                }
                up_read(&mut (*authkey).sem);
                if !dest_keyring.is_null() {
                    do_perm_check = false;
                }
            }
        }

        if dest_keyring.is_null()
            && ((*credp).jit_keyring == KEY_REQKEY_DEFL_DEFAULT
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_REQUESTOR_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_THREAD_KEYRING)
        {
            dest_keyring = key_get((*credp).thread_keyring);
        }

        if dest_keyring.is_null()
            && ((*credp).jit_keyring == KEY_REQKEY_DEFL_DEFAULT
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_REQUESTOR_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_THREAD_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_PROCESS_KEYRING)
        {
            dest_keyring = key_get((*credp).process_keyring);
        }

        if dest_keyring.is_null()
            && ((*credp).jit_keyring == KEY_REQKEY_DEFL_DEFAULT
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_REQUESTOR_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_THREAD_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_PROCESS_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_SESSION_KEYRING)
        {
            dest_keyring = key_get((*credp).session_keyring);
        }

        if dest_keyring.is_null()
            && ((*credp).jit_keyring == KEY_REQKEY_DEFL_DEFAULT
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_REQUESTOR_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_THREAD_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_PROCESS_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_SESSION_KEYRING
                || (*credp).jit_keyring == KEY_REQKEY_DEFL_USER_SESSION_KEYRING)
        {
            ret = look_up_user_keyrings(ptr::null_mut(), &mut dest_keyring);
            if ret < 0 {
                return ret;
            }
        } else if (*credp).jit_keyring == KEY_REQKEY_DEFL_USER_KEYRING {
            ret = look_up_user_keyrings(&mut dest_keyring, ptr::null_mut());
            if ret < 0 {
                return ret;
            }
        } else if (*credp).jit_keyring == KEY_REQKEY_DEFL_GROUP_KEYRING {
            return -EINVAL;
        } else if dest_keyring.is_null() {
            return -EINVAL;
        }

        /*
         * Require Write permission on the keyring.  This is essential
         * because the default keyring may be the session keyring, and
         * joining a keyring only requires Search permission.
         *
         * However, this check is skipped for the "requestor keyring" so
         * that /sbin/request-key can itself use request_key() to add
         * keys to the original requestor's destination keyring.
         */
        if !dest_keyring.is_null() && do_perm_check {
            ret = key_permission(make_key_ref(dest_keyring, 1), KEY_NEED_WRITE);
            if ret != 0 {
                key_put(dest_keyring);
                return ret;
            }
        }
    }

    *_dest_keyring = dest_keyring;
    kleave(c" [dk %d]".as_ptr());
    0
}

/*
 * Allocate a new key in under-construction state and attempt to link it in to
 * the requested keyring.
 *
 * May return a key that's already under construction instead if there was a
 * race between two thread calling request_key().
 */
unsafe fn construct_alloc_key(
    ctx: *mut keyring_search_context,
    dest_keyring: *mut key,
    flags: c_ulong,
    user: *mut key_user,
    _key: *mut *mut key,
) -> c_int {
    let mut edit: *mut assoc_array_edit = ptr::null_mut();
    let mut keyp: *mut key;
    let mut perm: key_perm_t;
    let key_ref: key_ref_t;
    let mut ret: c_int;

    kenter(c"%s,%s,,,".as_ptr());

    *_key = ptr::null_mut();
    mutex_lock(&mut (*user).cons_lock);

    perm = KEY_POS_VIEW | KEY_POS_SEARCH | KEY_POS_LINK | KEY_POS_SETATTR;
    perm |= KEY_USR_VIEW;
    if (*(*ctx).index_key.type_).read.is_some() {
        perm |= KEY_POS_READ;
    }
    if (*ctx).index_key.type_ == &raw mut key_type_keyring || (*(*ctx).index_key.type_).update.is_some() {
        perm |= KEY_POS_WRITE;
    }

    keyp = key_alloc(
        (*ctx).index_key.type_,
        (*ctx).index_key.description,
        (*(*ctx).cred).fsuid,
        (*(*ctx).cred).fsgid,
        (*ctx).cred,
        perm,
        flags,
        ptr::null(),
    );
    if IS_ERR(keyp) {
        mutex_unlock(&mut (*user).cons_lock);
        kleave(c" = %ld".as_ptr());
        return PTR_ERR(keyp) as c_int;
    }

    set_bit(KEY_FLAG_USER_CONSTRUCT, &mut (*keyp).flags);

    if !dest_keyring.is_null() {
        ret = __key_link_lock(dest_keyring, &(*keyp).index_key);
        if ret < 0 {
            mutex_unlock(&mut (*user).cons_lock);
            key_put(keyp);
            kleave(c" = %d [prelink]".as_ptr());
            return ret;
        }
    }

    /*
     * Attach the key to the destination keyring under lock, but we do need
     * to do another check just in case someone beat us to it whilst we
     * waited for locks.
     *
     * The caller might specify a comparison function which looks for keys
     * that do not exactly match but are still equivalent from the caller's
     * perspective. The __key_link_begin() operation must be done only after
     * an actual key is determined.
     */
    mutex_lock(&raw mut key_construction_mutex);

    rcu_read_lock();
    key_ref = search_process_keyrings_rcu(ctx);
    rcu_read_unlock();
    if !IS_ERR(key_ref) {
        key_put(keyp);
        mutex_unlock(&raw mut key_construction_mutex);
        keyp = key_ref_to_ptr(key_ref);
        if !dest_keyring.is_null() {
            ret = __key_link_begin(dest_keyring, &(*keyp).index_key, &mut edit);
            if ret < 0 {
                __key_link_end(dest_keyring, &(*keyp).index_key, edit);
                mutex_unlock(&mut (*user).cons_lock);
                key_put(keyp);
                kleave(c" = %d [prelink]".as_ptr());
                return ret;
            }
            ret = __key_link_check_live_key(dest_keyring, keyp);
            if ret == 0 {
                __key_link(dest_keyring, keyp, &mut edit);
            }
            __key_link_end(dest_keyring, &(*keyp).index_key, edit);
            if ret < 0 {
                mutex_unlock(&mut (*user).cons_lock);
                key_put(keyp);
                kleave(c" = %d [linkcheck]".as_ptr());
                return ret;
            }
        }
        mutex_unlock(&mut (*user).cons_lock);
        *_key = keyp;
        kleave(c" = -EINPROGRESS [%d]".as_ptr());
        return -EINPROGRESS;
    }

    if !dest_keyring.is_null() {
        ret = __key_link_begin(dest_keyring, &(*keyp).index_key, &mut edit);
        if ret < 0 {
            mutex_unlock(&raw mut key_construction_mutex);
            __key_link_end(dest_keyring, &(*keyp).index_key, edit);
            mutex_unlock(&mut (*user).cons_lock);
            key_put(keyp);
            kleave(c" = %d [prelink]".as_ptr());
            return ret;
        }
        __key_link(dest_keyring, keyp, &mut edit);
    }

    mutex_unlock(&raw mut key_construction_mutex);
    if !dest_keyring.is_null() {
        __key_link_end(dest_keyring, &(*keyp).index_key, edit);
    }
    mutex_unlock(&mut (*user).cons_lock);
    *_key = keyp;
    kleave(c" = 0 [%d]".as_ptr());
    0
}

/*
 * Commence key construction.
 */
unsafe fn construct_key_and_link(
    ctx: *mut keyring_search_context,
    callout_info: *const c_char,
    callout_len: size_t,
    aux: *mut c_void,
    mut dest_keyring: *mut key,
    flags: c_ulong,
) -> *mut key {
    let user: *mut key_user;
    let mut keyp: *mut key = ptr::null_mut();
    let mut ret: c_int;

    kenter(c"".as_ptr());

    if (*ctx).index_key.type_ == &raw mut key_type_keyring {
        return ERR_PTR(-(EPERM as c_long));
    }

    ret = construct_get_dest_keyring(&mut dest_keyring);
    if ret != 0 {
        kleave(c" = %d".as_ptr());
        return ERR_PTR(ret as c_long);
    }

    user = key_user_lookup(current_fsuid());
    if user.is_null() {
        ret = -ENOMEM;
        key_put(dest_keyring);
        kleave(c" = %d".as_ptr());
        return ERR_PTR(ret as c_long);
    }

    ret = construct_alloc_key(ctx, dest_keyring, flags, user, &mut keyp);
    key_user_put(user);

    if ret == 0 {
        ret = construct_key(keyp, callout_info as *const c_void, callout_len, aux, dest_keyring);
        if ret < 0 {
            kdebug(c"cons failed".as_ptr());
            key_negate_and_link(keyp, key_negative_timeout, ptr::null(), ptr::null_mut());
            key_put(keyp);
            key_put(dest_keyring);
            kleave(c" = %d".as_ptr());
            return ERR_PTR(ret as c_long);
        }
    } else if ret == -EINPROGRESS {
        ret = 0;
    } else {
        key_put(dest_keyring);
        kleave(c" = %d".as_ptr());
        return ERR_PTR(ret as c_long);
    }

    key_put(dest_keyring);
    kleave(c" = key %d".as_ptr());
    keyp
}

/**
 * request_key_and_link - Request a key and cache it in a keyring.
 * @type: The type of key we want.
 * @description: The searchable description of the key.
 * @domain_tag: The domain in which the key operates.
 * @callout_info: The data to pass to the instantiation upcall (or NULL).
 * @callout_len: The length of callout_info.
 * @aux: Auxiliary data for the upcall.
 * @dest_keyring: Where to cache the key.
 * @flags: Flags to key_alloc().
 *
 * A key matching the specified criteria (type, description, domain_tag) is
 * searched for in the process's keyrings and returned with its usage count
 * incremented if found.  Otherwise, if callout_info is not NULL, a key will be
 * allocated and some service (probably in userspace) will be asked to
 * instantiate it.
 *
 * If successfully found or created, the key will be linked to the destination
 * keyring if one is provided.
 *
 * Returns a pointer to the key if successful; -EACCES, -ENOKEY, -EKEYREVOKED
 * or -EKEYEXPIRED if an inaccessible, negative, revoked or expired key was
 * found; -ENOKEY if no key was found and no @callout_info was given; -EDQUOT
 * if insufficient key quota was available to create a new key; or -ENOMEM if
 * insufficient memory was available.
 *
 * If the returned key was created, then it may still be under construction,
 * and wait_for_key_construction() should be used to wait for that to complete.
 */
#[no_mangle]
pub unsafe extern "C" fn request_key_and_link(
    type_: *mut key_type,
    description: *const c_char,
    domain_tag: *mut key_tag,
    callout_info: *const c_void,
    callout_len: size_t,
    aux: *mut c_void,
    dest_keyring: *mut key,
    flags: c_ulong,
) -> *mut key {
    let mut ctx = keyring_search_context {
        index_key: keyring_index_key {
            type_,
            domain_tag,
            description,
            desc_len: strlen(description),
        },
        cred: current_cred(),
        match_data: key_match_data {
            cmp: Some(key_default_cmp),
            raw_data: description as *const c_void,
            lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
        },
        flags: KEYRING_SEARCH_DO_STATE_CHECK | KEYRING_SEARCH_SKIP_EXPIRED | KEYRING_SEARCH_RECURSE,
    };
    let mut keyp: *mut key;
    let key_ref: key_ref_t;
    let mut ret: c_int;

    kenter(c"%s,%s,%p,%zu,%p,%p,%lx".as_ptr());

    if let Some(match_preparse) = (*type_).match_preparse {
        ret = match_preparse(&mut ctx.match_data);
        if ret < 0 {
            keyp = ERR_PTR(ret as c_long);
            kleave(c" = %p".as_ptr());
            return keyp;
        }
    }

    keyp = check_cached_key(&mut ctx);
    if !keyp.is_null() {
        if let Some(match_free) = (*type_).match_free {
            match_free(&mut ctx.match_data);
        }
        kleave(c" = %p".as_ptr());
        return keyp;
    }

    /* search all the process keyrings for a key */
    rcu_read_lock();
    key_ref = search_process_keyrings_rcu(&mut ctx);
    rcu_read_unlock();

    if !IS_ERR(key_ref) {
        if !dest_keyring.is_null() {
            ret = key_task_permission(key_ref, current_cred(), KEY_NEED_LINK);
            if ret < 0 {
                key_ref_put(key_ref);
                keyp = ERR_PTR(ret as c_long);
                if let Some(match_free) = (*type_).match_free {
                    match_free(&mut ctx.match_data);
                }
                kleave(c" = %p".as_ptr());
                return keyp;
            }
        }

        keyp = key_ref_to_ptr(key_ref);
        if !dest_keyring.is_null() {
            ret = key_link(dest_keyring, keyp);
            if ret < 0 {
                key_put(keyp);
                keyp = ERR_PTR(ret as c_long);
                if let Some(match_free) = (*type_).match_free {
                    match_free(&mut ctx.match_data);
                }
                kleave(c" = %p".as_ptr());
                return keyp;
            }
        }

        /* Only cache the key on immediate success */
        cache_requested_key(keyp);
    } else if PTR_ERR(key_ref) != -(EAGAIN as c_long) {
        keyp = ERR_CAST(key_ref);
    } else {
        /* the search failed, but the keyrings were searchable, so we
         * should consult userspace if we can */
        keyp = ERR_PTR(-(ENOKEY as c_long));
        if !callout_info.is_null() {
            keyp = construct_key_and_link(
                &mut ctx,
                callout_info as *const c_char,
                callout_len,
                aux,
                dest_keyring,
                flags,
            );
        }
    }

    if let Some(match_free) = (*type_).match_free {
        match_free(&mut ctx.match_data);
    }
    kleave(c" = %p".as_ptr());
    keyp
}

/**
 * wait_for_key_construction - Wait for construction of a key to complete
 * @key: The key being waited for.
 * @intr: Whether to wait interruptibly.
 *
 * Wait for a key to finish being constructed.
 *
 * Returns 0 if successful; -ERESTARTSYS if the wait was interrupted; -ENOKEY
 * if the key was negated; or -EKEYREVOKED or -EKEYEXPIRED if the key was
 * revoked or expired.
 */
#[no_mangle]
pub unsafe extern "C" fn wait_for_key_construction(keyp: *mut key, intr: bool_t) -> c_int {
    let mut ret: c_int;

    ret = wait_on_bit(
        &mut (*keyp).flags,
        KEY_FLAG_USER_CONSTRUCT,
        if intr { TASK_INTERRUPTIBLE } else { TASK_UNINTERRUPTIBLE },
    );
    if ret != 0 {
        return -ERESTARTSYS;
    }
    ret = key_read_state(keyp);
    if ret < 0 {
        return ret;
    }
    key_validate(keyp)
}

/**
 * request_key_tag - Request a key and wait for construction
 * @type: Type of key.
 * @description: The searchable description of the key.
 * @domain_tag: The domain in which the key operates.
 * @callout_info: The data to pass to the instantiation upcall (or NULL).
 *
 * As for request_key_and_link() except that it does not add the returned key
 * to a keyring if found, new keys are always allocated in the user's quota,
 * the callout_info must be a NUL-terminated string and no auxiliary data can
 * be passed.
 *
 * Furthermore, it then works as wait_for_key_construction() to wait for the
 * completion of keys undergoing construction with a non-interruptible wait.
 */
#[no_mangle]
pub unsafe extern "C" fn request_key_tag(
    type_: *mut key_type,
    description: *const c_char,
    domain_tag: *mut key_tag,
    callout_info: *const c_char,
) -> *mut key {
    let mut keyp: *mut key;
    let mut callout_len: size_t = 0;
    let ret: c_int;

    if !callout_info.is_null() {
        callout_len = strlen(callout_info);
    }
    keyp = request_key_and_link(
        type_,
        description,
        domain_tag,
        callout_info as *const c_void,
        callout_len,
        ptr::null_mut(),
        ptr::null_mut(),
        KEY_ALLOC_IN_QUOTA,
    );
    if !IS_ERR(keyp) {
        ret = wait_for_key_construction(keyp, false);
        if ret < 0 {
            key_put(keyp);
            return ERR_PTR(ret as c_long);
        }
    }
    keyp
}

/**
 * request_key_with_auxdata - Request a key with auxiliary data for the upcaller
 * @type: The type of key we want.
 * @description: The searchable description of the key.
 * @domain_tag: The domain in which the key operates.
 * @callout_info: The data to pass to the instantiation upcall (or NULL).
 * @callout_len: The length of callout_info.
 * @aux: Auxiliary data for the upcall.
 *
 * As for request_key_and_link() except that it does not add the returned key
 * to a keyring if found and new keys are always allocated in the user's quota.
 *
 * Furthermore, it then works as wait_for_key_construction() to wait for the
 * completion of keys undergoing construction with a non-interruptible wait.
 */
#[no_mangle]
pub unsafe extern "C" fn request_key_with_auxdata(
    type_: *mut key_type,
    description: *const c_char,
    domain_tag: *mut key_tag,
    callout_info: *const c_void,
    callout_len: size_t,
    aux: *mut c_void,
) -> *mut key {
    let keyp: *mut key;
    let ret: c_int;

    keyp = request_key_and_link(
        type_,
        description,
        domain_tag,
        callout_info,
        callout_len,
        aux,
        ptr::null_mut(),
        KEY_ALLOC_IN_QUOTA,
    );
    if !IS_ERR(keyp) {
        ret = wait_for_key_construction(keyp, false);
        if ret < 0 {
            key_put(keyp);
            return ERR_PTR(ret as c_long);
        }
    }
    keyp
}

/**
 * request_key_rcu - Request key from RCU-read-locked context
 * @type: The type of key we want.
 * @description: The name of the key we want.
 * @domain_tag: The domain in which the key operates.
 *
 * Request a key from a context that we may not sleep in (such as RCU-mode
 * pathwalk).  Keys under construction are ignored.
 *
 * Return a pointer to the found key if successful, -ENOKEY if we couldn't find
 * a key or some other error if the key found was unsuitable or inaccessible.
 */
#[no_mangle]
pub unsafe extern "C" fn request_key_rcu(
    type_: *mut key_type,
    description: *const c_char,
    domain_tag: *mut key_tag,
) -> *mut key {
    let mut ctx = keyring_search_context {
        index_key: keyring_index_key {
            type_,
            domain_tag,
            description,
            desc_len: strlen(description),
        },
        cred: current_cred(),
        match_data: key_match_data {
            cmp: Some(key_default_cmp),
            raw_data: description as *const c_void,
            lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
        },
        flags: KEYRING_SEARCH_DO_STATE_CHECK | KEYRING_SEARCH_SKIP_EXPIRED,
    };
    let mut keyp: *mut key;
    let key_ref: key_ref_t;

    kenter(c"%s,%s".as_ptr());

    keyp = check_cached_key(&mut ctx);
    if !keyp.is_null() {
        return keyp;
    }

    /* search all the process keyrings for a key */
    key_ref = search_process_keyrings_rcu(&mut ctx);
    if IS_ERR(key_ref) {
        keyp = ERR_CAST(key_ref);
        if PTR_ERR(key_ref) == -(EAGAIN as c_long) {
            keyp = ERR_PTR(-(ENOKEY as c_long));
        }
    } else {
        keyp = key_ref_to_ptr(key_ref);
        cache_requested_key(keyp);
    }

    kleave(c" = %p".as_ptr());
    keyp
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
