// SPDX-License-Identifier: GPL-2.0-or-later
/* Userspace key control operations
 *
 * Copyright (C) 2004-5 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type size_t = usize;
type key_serial_t = c_int;
type key_perm_t = c_uint;
type uid_t = c_uint;
type gid_t = c_uint;
type kuid_t = c_uint;
type kgid_t = c_uint;
type bool_t = bool;

const KEY_MAX_DESC_SIZE: usize = 4096;

/* Constants, types and functions supplied by the kernel headers included by
 * the original C source are external dependencies of this translation.
 */
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: c_uint = 0;
const ITER_SOURCE: c_uint = 0;
const UIO_FASTIOV: usize = 8;
const MAX_ERRNO: c_uint = 4095;

extern "C" {
    static key_quota_root_maxkeys: c_uint;
    static key_quota_maxkeys: c_uint;
    static key_quota_root_maxbytes: c_uint;
    static key_quota_maxbytes: c_uint;
    static GLOBAL_ROOT_UID: kuid_t;
    static current: *mut task_struct;
    static mut tasklist_lock: c_int;

    fn strncpy_from_user(dst: *mut c_char, src: *const c_char, count: size_t) -> c_long;
    fn strndup_user(src: *const c_char, n: c_long) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn kfree(p: *mut c_void);
    fn kvmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvfree_sensitive(p: *mut c_void, size: size_t);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn clear_user(to: *mut c_void, n: size_t) -> c_ulong;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;

    fn lookup_user_key(id: key_serial_t, flags: c_ulong, perm: key_perm_t) -> key_ref_t;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn key_ref_put(key_ref: key_ref_t);
    fn key_put(key: *mut key);
    fn key_get(key: *mut key) -> *mut key;
    fn key_create_or_update(keyring_ref: key_ref_t, type_: *const c_char, description: *const c_char, payload: *const c_void, plen: size_t, perm: key_perm_t, flags: c_ulong) -> key_ref_t;
    fn key_update(key_ref: key_ref_t, payload: *const c_void, plen: size_t) -> c_long;
    fn key_revoke(key: *mut key);
    fn key_invalidate(key: *mut key);
    fn keyring_clear(keyring: *mut key) -> c_long;
    fn key_link(keyring: *mut key, key: *mut key) -> c_long;
    fn key_unlink(keyring: *mut key, key: *mut key) -> c_long;
    fn key_move(key: *mut key, from: *mut key, to: *mut key, flags: c_uint) -> c_long;
    fn key_permission(key_ref: key_ref_t, perm: key_perm_t) -> c_long;
    fn key_validate(key: *mut key) -> c_long;
    fn key_read_state(key: *mut key) -> c_long;
    fn key_type_lookup(name: *const c_char) -> *mut key_type;
    fn key_type_put(ktype: *mut key_type);
    fn keyring_search(keyring_ref: key_ref_t, ktype: *mut key_type, description: *const c_char, recurse: bool_t) -> key_ref_t;
    fn request_key_and_link(ktype: *mut key_type, description: *const c_char, domain_tag: *const c_void, callout_info: *const c_char, callout_len: size_t, aux: *const c_void, dest_keyring: *mut key, flags: c_ulong) -> *mut key;
    fn wait_for_key_construction(key: *mut key, intr: c_int) -> c_long;
    fn key_get_instantiation_authkey(id: key_serial_t) -> *mut key;
    fn request_key_auth_get(key: *mut key) -> *mut request_key_auth;
    fn request_key_auth_put(rka: *mut request_key_auth);
    fn key_instantiate_and_link(key: *mut key, payload: *const c_void, plen: size_t, keyring: *mut key, authkey: *mut key) -> c_long;
    fn key_reject_and_link(key: *mut key, timeout: c_uint, error: c_uint, keyring: *mut key, authkey: *mut key) -> c_long;
    fn join_session_keyring(name: *const c_char) -> c_long;
    fn keyring_restrict(key_ref: key_ref_t, type_: *const c_char, restriction: *const c_char) -> c_long;

    fn current_user_ns() -> *mut c_void;
    fn current_cred() -> *const cred;
    fn current_fsuid() -> kuid_t;
    fn make_kuid(ns: *mut c_void, uid: uid_t) -> kuid_t;
    fn make_kgid(ns: *mut c_void, gid: gid_t) -> kgid_t;
    fn uid_valid(uid: kuid_t) -> bool_t;
    fn gid_valid(gid: kgid_t) -> bool_t;
    fn uid_eq(a: kuid_t, b: kuid_t) -> bool_t;
    fn gid_eq(a: kgid_t, b: kgid_t) -> bool_t;
    fn from_kuid_munged(ns: *mut c_void, uid: kuid_t) -> c_uint;
    fn from_kgid_munged(ns: *mut c_void, gid: kgid_t) -> c_uint;
    fn in_group_p(gid: kgid_t) -> bool_t;
    fn capable(cap: c_int) -> bool_t;
    fn prepare_creds() -> *mut cred;
    fn commit_creds(new: *mut cred) -> c_int;
    fn abort_creds(new: *mut cred);
    fn cred_alloc_blank() -> *mut cred;
    fn put_cred(cred: *mut cred);
    fn install_thread_keyring_to_cred(new: *mut cred) -> c_int;
    fn install_process_keyring_to_cred(new: *mut cred) -> c_int;

    fn down_read(sem: *mut c_int);
    fn up_read(sem: *mut c_int);
    fn down_write(sem: *mut c_int);
    fn up_write(sem: *mut c_int);
    fn spin_lock_irqsave(lock: *mut c_int, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_int, flags: c_ulong);
    fn atomic_dec(v: *mut c_int);
    fn atomic_inc(v: *mut c_int);
    fn test_bit(bit: c_int, addr: *const c_ulong) -> bool_t;
    fn notify_key(key: *mut key, event: c_int, aux: c_int);

    fn import_ubuf(rw: c_uint, buf: *mut c_void, len: size_t, iter: *mut iov_iter) -> c_int;
    fn import_iovec(rw: c_uint, uvec: *const iovec, nr_segs: c_uint, fast_segs: c_uint, iov: *mut *mut iovec, iter: *mut iov_iter) -> c_long;
    fn iov_iter_count(iter: *const iov_iter) -> size_t;
    fn copy_from_iter_full(addr: *mut c_void, bytes: size_t, iter: *mut iov_iter) -> bool_t;

    fn security_key_getsecurity(key: *mut key, context: *mut *mut c_char) -> c_long;
    fn security_watch_key(key: *mut key) -> c_long;
    fn get_watch_queue(fd: c_int) -> *mut watch_queue;
    fn put_watch_queue(wqueue: *mut watch_queue);
    fn init_watch_list(wlist: *mut watch_list, filter: *mut c_void);
    fn init_watch(watch: *mut watch, wqueue: *mut watch_queue);
    fn add_watch_to_object(watch: *mut watch, wlist: *mut watch_list) -> c_long;
    fn remove_watch_from_object(wlist: *mut watch_list, wqueue: *mut watch_queue, id: key_serial_t, all: bool_t) -> c_long;
    fn key_serial(key: *mut key) -> key_serial_t;

    fn init_task_work(work: *mut callback_head, func: extern "C" fn(*mut callback_head));
    fn key_change_session_keyring(work: *mut callback_head);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn write_lock_irq(lock: *mut c_int);
    fn write_unlock_irq(lock: *mut c_int);
    fn lockdep_is_held(lock: *mut c_int) -> bool_t;
    fn thread_group_empty(task: *mut task_struct) -> bool_t;
    fn __task_cred(task: *mut task_struct) -> *const cred;
    fn task_work_cancel_func(task: *mut task_struct, func: extern "C" fn(*mut callback_head)) -> *mut callback_head;
    fn task_work_add(task: *mut task_struct, work: *mut callback_head, notify: c_int) -> c_int;

    fn keyctl_get_persistent(uid: uid_t, id: key_serial_t) -> c_long;
    fn keyctl_dh_compute(params: *mut keyctl_dh_params, buffer: *mut c_char, buflen: size_t, kdf: *mut keyctl_kdf_params) -> c_long;
    fn keyctl_pkey_query(id: key_serial_t, info: *const c_char, res: *mut keyctl_pkey_query) -> c_long;
    fn keyctl_pkey_e_d_s(option: c_int, params: *const keyctl_pkey_params, info: *const c_char, in_: *const c_void, out: *mut c_void) -> c_long;
    fn keyctl_pkey_verify(params: *const keyctl_pkey_params, info: *const c_char, in_: *const c_void, sig: *const c_void) -> c_long;
}

#[repr(C)] pub struct key_type { pub name: *const c_char, pub read: Option<unsafe extern "C" fn(*mut key, *mut c_char, size_t) -> c_long> }
#[repr(C)] pub struct key_user { pub lock: c_int, pub nkeys: c_int, pub nikeys: c_int, pub qnkeys: c_uint, pub qnbytes: c_uint }
#[repr(C)] pub struct key { pub serial: key_serial_t, pub flags: c_ulong, pub sem: c_int, pub type_: *mut key_type, pub description: *mut c_char, pub uid: kuid_t, pub gid: kgid_t, pub perm: key_perm_t, pub user: *mut key_user, pub quotalen: c_uint, pub state: c_int, pub watchers: *mut watch_list }
#[repr(C)] pub struct request_key_auth { pub target_key: *mut key, pub dest_keyring: *mut key }
#[repr(C)] pub struct cred { pub rcu: callback_head, pub request_key_auth: *mut key, pub jit_keyring: c_int, pub session_keyring: *mut key, pub uid: kuid_t, pub euid: kuid_t, pub suid: kuid_t, pub gid: kgid_t, pub egid: kgid_t, pub sgid: kgid_t }
#[repr(C)] pub struct task_struct { pub pid: c_int, pub mm: *mut c_void, pub real_parent: *mut task_struct }
#[repr(C)] pub struct callback_head { _private: [u8; 0] }
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: size_t }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct watch_queue { _private: [u8; 0] }
#[repr(C)] pub struct watch_list { _private: [u8; 0] }
#[repr(C)] pub struct watch { pub id: key_serial_t, pub info_id: u32 }
#[repr(C)] pub struct keyctl_dh_params { _private: [u8; 0] }
#[repr(C)] pub struct keyctl_kdf_params { _private: [u8; 0] }
#[repr(C)] pub struct keyctl_pkey_query { _private: [u8; 0] }
#[repr(C)] pub struct keyctl_pkey_params { _private: [u8; 0] }

type key_ref_t = *mut c_void;

const EINVAL: c_long = 22;
const EPERM: c_long = 1;
const ENOMEM: c_long = 12;
const EFAULT: c_long = 14;
const EACCES: c_long = 13;
const EOPNOTSUPP: c_long = 95;
const ENOKEY: c_long = 126;
const EAGAIN: c_long = 11;
const EDQUOT: c_long = 122;
const EKEYREVOKED: c_long = 128;
const EBADSLT: c_long = 57;
const ERESTARTSYS: c_uint = 512;
const ERESTARTNOINTR: c_uint = 513;
const ERESTARTNOHAND: c_uint = 514;
const ERESTART_RESTARTBLOCK: c_uint = 516;

const KEY_LOOKUP_CREATE: c_ulong = 0x01;
const KEY_LOOKUP_PARTIAL: c_ulong = 0x02;
const KEY_NEED_WRITE: key_perm_t = 0x01;
const KEY_NEED_SEARCH: key_perm_t = 0x02;
const KEY_NEED_SETATTR: key_perm_t = 0x04;
const KEY_NEED_LINK: key_perm_t = 0x08;
const KEY_NEED_UNLINK: key_perm_t = 0x10;
const KEY_NEED_VIEW: key_perm_t = 0x20;
const KEY_NEED_READ: key_perm_t = 0x40;
const KEY_DEFER_PERM_CHECK: key_perm_t = 0x80;
const KEY_SYSADMIN_OVERRIDE: key_perm_t = 0x100;
const KEY_AUTHTOKEN_OVERRIDE: key_perm_t = 0x200;
const KEY_PERM_UNDEF: key_perm_t = 0;
const KEY_ALLOC_IN_QUOTA: c_ulong = 0x01;
const KEY_FLAG_KEEP: c_int = 0;
const KEY_FLAG_ROOT_CAN_INVAL: c_int = 1;
const KEY_FLAG_ROOT_CAN_CLEAR: c_int = 2;
const KEY_FLAG_IN_QUOTA: c_int = 3;
const KEY_IS_UNINSTANTIATED: c_int = 0;
const CAP_SYS_ADMIN: c_int = 21;
const NOTIFY_KEY_SETATTR: c_int = 0;
const KEY_SPEC_REQKEY_AUTH_KEY: key_serial_t = -7;
const KEY_SPEC_REQUESTOR_KEYRING: key_serial_t = -8;
const KEY_SPEC_SESSION_KEYRING: key_serial_t = -3;
const TWA_RESUME: c_int = 0;
const WATCH_INFO_ID__SHIFT: u32 = 0;
const KEYCTL_MOVE_EXCL: c_uint = 0x0001;

const KEY_POS_ALL: key_perm_t = 0x3f000000;
const KEY_USR_ALL: key_perm_t = 0x003f0000;
const KEY_GRP_ALL: key_perm_t = 0x00003f00;
const KEY_OTH_ALL: key_perm_t = 0x0000003f;

const KEYCTL_CAPS0_CAPABILITIES: u8 = 0x01;
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u8 = 0x02;
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u8 = 0x04;
const KEYCTL_CAPS0_PUBLIC_KEY: u8 = 0x08;
const KEYCTL_CAPS0_BIG_KEY: u8 = 0x10;
const KEYCTL_CAPS0_INVALIDATE: u8 = 0x20;
const KEYCTL_CAPS0_RESTRICT_KEYRING: u8 = 0x40;
const KEYCTL_CAPS0_MOVE: u8 = 0x80;
const KEYCTL_CAPS1_NS_KEYRING_NAME: u8 = 0x01;
const KEYCTL_CAPS1_NS_KEY_TAG: u8 = 0x02;
const KEYCTL_CAPS1_NOTIFICATIONS: u8 = 0x04;

const CONFIG_PERSISTENT_KEYRINGS: bool = false;
const CONFIG_KEY_DH_OPERATIONS: bool = false;
const CONFIG_ASYMMETRIC_KEY_TYPE: bool = false;
const CONFIG_BIG_KEYS: bool = false;
const CONFIG_KEY_NOTIFICATIONS: bool = false;

const fn is_enabled(v: bool) -> u8 { if v { 1 } else { 0 } }

static keyrings_capabilities: [u8; 2] = [
    KEYCTL_CAPS0_CAPABILITIES |
    (is_enabled(CONFIG_PERSISTENT_KEYRINGS) * KEYCTL_CAPS0_PERSISTENT_KEYRINGS) |
    (is_enabled(CONFIG_KEY_DH_OPERATIONS) * KEYCTL_CAPS0_DIFFIE_HELLMAN) |
    (is_enabled(CONFIG_ASYMMETRIC_KEY_TYPE) * KEYCTL_CAPS0_PUBLIC_KEY) |
    (is_enabled(CONFIG_BIG_KEYS) * KEYCTL_CAPS0_BIG_KEY) |
    KEYCTL_CAPS0_INVALIDATE |
    KEYCTL_CAPS0_RESTRICT_KEYRING |
    KEYCTL_CAPS0_MOVE,
    KEYCTL_CAPS1_NS_KEYRING_NAME |
    KEYCTL_CAPS1_NS_KEY_TAG |
    (is_enabled(CONFIG_KEY_NOTIFICATIONS) * KEYCTL_CAPS1_NOTIFICATIONS),
];

unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) >= -4095 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_long { p as isize as c_long }
unsafe fn unlikely<T>(v: T) -> T { v }

unsafe fn key_get_type_from_user(type_: *mut c_char, _type: *const c_char, len: c_uint) -> c_int {
    let mut ret: c_int;
    ret = strncpy_from_user(type_, _type, len as size_t) as c_int;
    if ret < 0 { return ret; }
    if ret == 0 || ret >= len as c_int { return -(EINVAL as c_int); }
    if *type_ == b'.' as c_char { return -(EPERM as c_int); }
    *type_.add(len as usize - 1) = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sys_add_key(_type: *const c_char, _description: *const c_char, _payload: *const c_void, plen: size_t, ringid: key_serial_t) -> c_long {
    let mut keyring_ref: key_ref_t;
    let mut key_ref: key_ref_t;
    let mut type_: [c_char; 32] = [0; 32];
    let mut description: *mut c_char;
    let mut payload: *mut c_void;
    let mut ret: c_long = -EINVAL;
    if plen > 1024 * 1024 - 1 { return ret; }
    ret = key_get_type_from_user(type_.as_mut_ptr(), _type, type_.len() as c_uint) as c_long;
    if ret < 0 { return ret; }
    description = ptr::null_mut();
    if !_description.is_null() {
        description = strndup_user(_description, KEY_MAX_DESC_SIZE as c_long);
        if IS_ERR(description) { return PTR_ERR(description); }
        if *description == 0 {
            kfree(description as *mut c_void);
            description = ptr::null_mut();
        } else if *description == b'.' as c_char && strncmp(type_.as_ptr(), b"keyring\0".as_ptr() as *const c_char, 7) == 0 {
            ret = -EPERM;
            kfree(description as *mut c_void);
            return ret;
        }
    }
    payload = ptr::null_mut();
    if plen != 0 {
        ret = -ENOMEM;
        payload = kvmalloc(plen, GFP_KERNEL);
        if payload.is_null() { kfree(description as *mut c_void); return ret; }
        ret = -EFAULT;
        if copy_from_user(payload, _payload, plen) != 0 {
            kvfree_sensitive(payload, plen);
            kfree(description as *mut c_void);
            return ret;
        }
    }
    keyring_ref = lookup_user_key(ringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
    if IS_ERR(keyring_ref) {
        ret = PTR_ERR(keyring_ref);
    } else {
        key_ref = key_create_or_update(keyring_ref, type_.as_ptr(), description, payload, plen, KEY_PERM_UNDEF, KEY_ALLOC_IN_QUOTA);
        if !IS_ERR(key_ref) {
            ret = (*key_ref_to_ptr(key_ref)).serial as c_long;
            key_ref_put(key_ref);
        } else {
            ret = PTR_ERR(key_ref);
        }
        key_ref_put(keyring_ref);
    }
    kvfree_sensitive(payload, plen);
    kfree(description as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn sys_request_key(_type: *const c_char, _description: *const c_char, _callout_info: *const c_char, destringid: key_serial_t) -> c_long {
    let mut type_: [c_char; 32] = [0; 32];
    let mut ret = key_get_type_from_user(type_.as_mut_ptr(), _type, type_.len() as c_uint) as c_long;
    if ret < 0 { return ret; }
    let description = strndup_user(_description, KEY_MAX_DESC_SIZE as c_long);
    if IS_ERR(description) { return PTR_ERR(description); }
    let mut callout_info: *mut c_char = ptr::null_mut();
    let mut callout_len: size_t = 0;
    if !_callout_info.is_null() {
        callout_info = strndup_user(_callout_info, PAGE_SIZE as c_long);
        if IS_ERR(callout_info) { ret = PTR_ERR(callout_info); kfree(description as *mut c_void); return ret; }
        callout_len = strlen(callout_info);
    }
    let mut dest_ref: key_ref_t = ptr::null_mut();
    if destringid != 0 {
        dest_ref = lookup_user_key(destringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
        if IS_ERR(dest_ref) { ret = PTR_ERR(dest_ref); kfree(callout_info as *mut c_void); kfree(description as *mut c_void); return ret; }
    }
    let ktype = key_type_lookup(type_.as_ptr());
    if IS_ERR(ktype) { ret = PTR_ERR(ktype); key_ref_put(dest_ref); kfree(callout_info as *mut c_void); kfree(description as *mut c_void); return ret; }
    let key = request_key_and_link(ktype, description, ptr::null(), callout_info, callout_len, ptr::null(), key_ref_to_ptr(dest_ref), KEY_ALLOC_IN_QUOTA);
    if IS_ERR(key) {
        ret = PTR_ERR(key);
    } else {
        ret = wait_for_key_construction(key, 1);
        if ret >= 0 { ret = (*key).serial as c_long; }
        key_put(key);
    }
    key_type_put(ktype);
    key_ref_put(dest_ref);
    kfree(callout_info as *mut c_void);
    kfree(description as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_get_keyring_ID(id: key_serial_t, create: c_int) -> c_long {
    let lflags = if create != 0 { KEY_LOOKUP_CREATE } else { 0 };
    let key_ref = lookup_user_key(id, lflags, KEY_NEED_SEARCH);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let ret = (*key_ref_to_ptr(key_ref)).serial as c_long;
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_join_session_keyring(_name: *const c_char) -> c_long {
    let mut name: *mut c_char = ptr::null_mut();
    if !_name.is_null() {
        name = strndup_user(_name, KEY_MAX_DESC_SIZE as c_long);
        if IS_ERR(name) { return PTR_ERR(name); }
        if *name == b'.' as c_char { kfree(name as *mut c_void); return -EPERM; }
    }
    let ret = join_session_keyring(name);
    kfree(name as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_update_key(id: key_serial_t, _payload: *const c_void, plen: size_t) -> c_long {
    let mut ret = -EINVAL;
    if plen > PAGE_SIZE { return ret; }
    let mut payload: *mut c_void = ptr::null_mut();
    if plen != 0 {
        ret = -ENOMEM;
        payload = kvmalloc(plen, GFP_KERNEL);
        if payload.is_null() { return ret; }
        ret = -EFAULT;
        if copy_from_user(payload, _payload, plen) != 0 { kvfree_sensitive(payload, plen); return ret; }
    }
    let key_ref = lookup_user_key(id, 0, KEY_NEED_WRITE);
    if IS_ERR(key_ref) { ret = PTR_ERR(key_ref); } else { ret = key_update(key_ref, payload, plen); key_ref_put(key_ref); }
    kvfree_sensitive(payload, plen);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_revoke_key(id: key_serial_t) -> c_long {
    let mut key_ref = lookup_user_key(id, 0, KEY_NEED_WRITE);
    if IS_ERR(key_ref) {
        let mut ret = PTR_ERR(key_ref);
        if ret != -EACCES { return ret; }
        key_ref = lookup_user_key(id, 0, KEY_NEED_SETATTR);
        if IS_ERR(key_ref) { ret = PTR_ERR(key_ref); return ret; }
    }
    let key = key_ref_to_ptr(key_ref);
    let ret = if test_bit(KEY_FLAG_KEEP, &(*key).flags) { -EPERM } else { key_revoke(key); 0 };
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_invalidate_key(id: key_serial_t) -> c_long {
    let mut key_ref = lookup_user_key(id, 0, KEY_NEED_SEARCH);
    if IS_ERR(key_ref) {
        if capable(CAP_SYS_ADMIN) {
            key_ref = lookup_user_key(id, 0, KEY_SYSADMIN_OVERRIDE);
            if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
            if !test_bit(KEY_FLAG_ROOT_CAN_INVAL, &(*key_ref_to_ptr(key_ref)).flags) {
                let ret = PTR_ERR(key_ref);
                key_ref_put(key_ref);
                return ret;
            }
        } else {
            return PTR_ERR(key_ref);
        }
    }
    let key = key_ref_to_ptr(key_ref);
    let ret = if test_bit(KEY_FLAG_KEEP, &(*key).flags) { -EPERM } else { key_invalidate(key); 0 };
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_keyring_clear(ringid: key_serial_t) -> c_long {
    let mut keyring_ref = lookup_user_key(ringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
    if IS_ERR(keyring_ref) {
        if capable(CAP_SYS_ADMIN) {
            keyring_ref = lookup_user_key(ringid, 0, KEY_SYSADMIN_OVERRIDE);
            if IS_ERR(keyring_ref) { return PTR_ERR(keyring_ref); }
            if !test_bit(KEY_FLAG_ROOT_CAN_CLEAR, &(*key_ref_to_ptr(keyring_ref)).flags) {
                let ret = PTR_ERR(keyring_ref);
                key_ref_put(keyring_ref);
                return ret;
            }
        } else {
            return PTR_ERR(keyring_ref);
        }
    }
    let keyring = key_ref_to_ptr(keyring_ref);
    let ret = if test_bit(KEY_FLAG_KEEP, &(*keyring).flags) { -EPERM } else { keyring_clear(keyring) };
    key_ref_put(keyring_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_keyring_link(id: key_serial_t, ringid: key_serial_t) -> c_long {
    let keyring_ref = lookup_user_key(ringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
    if IS_ERR(keyring_ref) { return PTR_ERR(keyring_ref); }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE, KEY_NEED_LINK);
    if IS_ERR(key_ref) { let ret = PTR_ERR(key_ref); key_ref_put(keyring_ref); return ret; }
    let ret = key_link(key_ref_to_ptr(keyring_ref), key_ref_to_ptr(key_ref));
    key_ref_put(key_ref);
    key_ref_put(keyring_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_keyring_unlink(id: key_serial_t, ringid: key_serial_t) -> c_long {
    let keyring_ref = lookup_user_key(ringid, 0, KEY_NEED_WRITE);
    if IS_ERR(keyring_ref) { return PTR_ERR(keyring_ref); }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_PARTIAL, KEY_NEED_UNLINK);
    if IS_ERR(key_ref) { let ret = PTR_ERR(key_ref); key_ref_put(keyring_ref); return ret; }
    let keyring = key_ref_to_ptr(keyring_ref);
    let key = key_ref_to_ptr(key_ref);
    let ret = if test_bit(KEY_FLAG_KEEP, &(*keyring).flags) && test_bit(KEY_FLAG_KEEP, &(*key).flags) { -EPERM } else { key_unlink(keyring, key) };
    key_ref_put(key_ref);
    key_ref_put(keyring_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_keyring_move(id: key_serial_t, from_ringid: key_serial_t, to_ringid: key_serial_t, flags: c_uint) -> c_long {
    if flags & !KEYCTL_MOVE_EXCL != 0 { return -EINVAL; }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE, KEY_NEED_LINK);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let from_ref = lookup_user_key(from_ringid, 0, KEY_NEED_WRITE);
    if IS_ERR(from_ref) { let ret = PTR_ERR(from_ref); key_ref_put(key_ref); return ret; }
    let to_ref = lookup_user_key(to_ringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
    if IS_ERR(to_ref) { let ret = PTR_ERR(to_ref); key_ref_put(from_ref); key_ref_put(key_ref); return ret; }
    let ret = key_move(key_ref_to_ptr(key_ref), key_ref_to_ptr(from_ref), key_ref_to_ptr(to_ref), flags);
    key_ref_put(to_ref);
    key_ref_put(from_ref);
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_describe_key(keyid: key_serial_t, buffer: *mut c_char, buflen: size_t) -> c_long {
    let mut key_ref = lookup_user_key(keyid, KEY_LOOKUP_PARTIAL, KEY_NEED_VIEW);
    if IS_ERR(key_ref) {
        if PTR_ERR(key_ref) == -EACCES {
            let instkey = key_get_instantiation_authkey(keyid);
            if !IS_ERR(instkey) {
                key_put(instkey);
                key_ref = lookup_user_key(keyid, KEY_LOOKUP_PARTIAL, KEY_AUTHTOKEN_OVERRIDE);
            }
        }
        if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    }
    let key = key_ref_to_ptr(key_ref);
    let desclen = strlen((*key).description);
    let infobuf = kasprintf(GFP_KERNEL, b"%s;%d;%d;%08x;\0".as_ptr() as *const c_char, (*(*key).type_).name, from_kuid_munged(current_user_ns(), (*key).uid), from_kgid_munged(current_user_ns(), (*key).gid), (*key).perm);
    if infobuf.is_null() { key_ref_put(key_ref); return -ENOMEM; }
    let infolen = strlen(infobuf);
    let mut ret = (infolen + desclen + 1) as c_long;
    if !buffer.is_null() && buflen >= ret as usize {
        if copy_to_user(buffer as *mut c_void, infobuf as *const c_void, infolen) != 0 ||
           copy_to_user(buffer.add(infolen) as *mut c_void, (*key).description as *const c_void, desclen + 1) != 0 {
            ret = -EFAULT;
        }
    }
    kfree(infobuf as *mut c_void);
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_keyring_search(ringid: key_serial_t, _type: *const c_char, _description: *const c_char, destringid: key_serial_t) -> c_long {
    let mut type_: [c_char; 32] = [0; 32];
    let mut ret = key_get_type_from_user(type_.as_mut_ptr(), _type, type_.len() as c_uint) as c_long;
    if ret < 0 { return ret; }
    let description = strndup_user(_description, KEY_MAX_DESC_SIZE as c_long);
    if IS_ERR(description) { return PTR_ERR(description); }
    let keyring_ref = lookup_user_key(ringid, 0, KEY_NEED_SEARCH);
    if IS_ERR(keyring_ref) { ret = PTR_ERR(keyring_ref); kfree(description as *mut c_void); return ret; }
    let mut dest_ref: key_ref_t = ptr::null_mut();
    if destringid != 0 {
        dest_ref = lookup_user_key(destringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
        if IS_ERR(dest_ref) { ret = PTR_ERR(dest_ref); key_ref_put(keyring_ref); kfree(description as *mut c_void); return ret; }
    }
    let ktype = key_type_lookup(type_.as_ptr());
    if IS_ERR(ktype) { ret = PTR_ERR(ktype); key_ref_put(dest_ref); key_ref_put(keyring_ref); kfree(description as *mut c_void); return ret; }
    let key_ref = keyring_search(keyring_ref, ktype, description, true);
    if IS_ERR(key_ref) {
        ret = PTR_ERR(key_ref);
        if ret == -EAGAIN { ret = -ENOKEY; }
    } else {
        if !dest_ref.is_null() {
            ret = key_permission(key_ref, KEY_NEED_LINK);
            if ret >= 0 { ret = key_link(key_ref_to_ptr(dest_ref), key_ref_to_ptr(key_ref)); }
        }
        if ret >= 0 { ret = (*key_ref_to_ptr(key_ref)).serial as c_long; }
        key_ref_put(key_ref);
    }
    key_type_put(ktype);
    key_ref_put(dest_ref);
    key_ref_put(keyring_ref);
    kfree(description as *mut c_void);
    ret
}

unsafe fn __keyctl_read_key(key: *mut key, buffer: *mut c_char, buflen: size_t) -> c_long {
    down_read(&mut (*key).sem);
    let mut ret = key_validate(key);
    if ret == 0 {
        ret = ((*(*key).type_).read.unwrap())(key, buffer, buflen);
    }
    up_read(&mut (*key).sem);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_read_key(keyid: key_serial_t, buffer: *mut c_char, buflen: size_t) -> c_long {
    let key_ref = lookup_user_key(keyid, 0, KEY_DEFER_PERM_CHECK);
    if IS_ERR(key_ref) { return -ENOKEY; }
    let key = key_ref_to_ptr(key_ref);
    let mut ret = key_read_state(key);
    if ret >= 0 {
        ret = key_permission(key_ref, KEY_NEED_READ);
        if ret == 0 || (ret == -EACCES && true) {
            if (*(*key).type_).read.is_none() {
                ret = -EOPNOTSUPP;
            } else if buffer.is_null() || buflen == 0 {
                ret = __keyctl_read_key(key, ptr::null_mut(), 0);
            } else {
                let mut key_data: *mut c_char = ptr::null_mut();
                let mut key_data_len = if buflen <= PAGE_SIZE { buflen } else { 0 };
                loop {
                    if key_data_len != 0 {
                        key_data = kvmalloc(key_data_len, GFP_KERNEL) as *mut c_char;
                        if key_data.is_null() { ret = -ENOMEM; break; }
                    }
                    ret = __keyctl_read_key(key, key_data, key_data_len);
                    if ret <= 0 || ret as size_t > buflen { break; }
                    if ret as size_t > key_data_len {
                        if !key_data.is_null() { kvfree_sensitive(key_data as *mut c_void, key_data_len); }
                        key_data_len = ret as size_t;
                        continue;
                    }
                    if copy_to_user(buffer as *mut c_void, key_data as *const c_void, ret as size_t) != 0 { ret = -EFAULT; }
                    break;
                }
                kvfree_sensitive(key_data as *mut c_void, key_data_len);
            }
        }
    }
    key_put(key);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_chown_key(id: key_serial_t, user: uid_t, group: gid_t) -> c_long {
    let uid = make_kuid(current_user_ns(), user);
    let gid = make_kgid(current_user_ns(), group);
    if user != !0u32 && !uid_valid(uid) { return -EINVAL; }
    if group != !0u32 && !gid_valid(gid) { return -EINVAL; }
    if user == !0u32 && group == !0u32 { return 0; }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE | KEY_LOOKUP_PARTIAL, KEY_NEED_SETATTR);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let key = key_ref_to_ptr(key_ref);
    let mut ret = -EACCES;
    let mut zapowner: *mut key_user = ptr::null_mut();
    down_write(&mut (*key).sem);
    let mut is_privileged_op = false;
    if user != !0u32 && !uid_eq((*key).uid, uid) { is_privileged_op = true; }
    if group != !0u32 && !gid_eq(gid, (*key).gid) && !in_group_p(gid) { is_privileged_op = true; }
    if !(is_privileged_op && !capable(CAP_SYS_ADMIN)) {
        if user != !0u32 && !uid_eq(uid, (*key).uid) {
            let newowner = key_user_lookup(uid);
            if !newowner.is_null() {
                if test_bit(KEY_FLAG_IN_QUOTA, &(*key).flags) {
                    let maxkeys = if uid_eq(uid, GLOBAL_ROOT_UID) { key_quota_root_maxkeys } else { key_quota_maxkeys };
                    let maxbytes = if uid_eq(uid, GLOBAL_ROOT_UID) { key_quota_root_maxbytes } else { key_quota_maxbytes };
                    let flags: c_ulong = 0;
                    spin_lock_irqsave(&mut (*newowner).lock, flags);
                    if (*newowner).qnkeys + 1 > maxkeys || (*newowner).qnbytes + (*key).quotalen > maxbytes || (*newowner).qnbytes + (*key).quotalen < (*newowner).qnbytes {
                        spin_unlock_irqrestore(&mut (*newowner).lock, flags);
                        zapowner = newowner;
                        ret = -EDQUOT;
                        up_write(&mut (*key).sem);
                        key_put(key);
                        if !zapowner.is_null() { key_user_put(zapowner); }
                        return ret;
                    }
                    (*newowner).qnkeys += 1;
                    (*newowner).qnbytes += (*key).quotalen;
                    spin_unlock_irqrestore(&mut (*newowner).lock, flags);
                    spin_lock_irqsave(&mut (*(*key).user).lock, flags);
                    (*(*key).user).qnkeys -= 1;
                    (*(*key).user).qnbytes -= (*key).quotalen;
                    spin_unlock_irqrestore(&mut (*(*key).user).lock, flags);
                }
                atomic_dec(&mut (*(*key).user).nkeys);
                atomic_inc(&mut (*newowner).nkeys);
                if (*key).state != KEY_IS_UNINSTANTIATED {
                    atomic_dec(&mut (*(*key).user).nikeys);
                    atomic_inc(&mut (*newowner).nikeys);
                }
                zapowner = (*key).user;
                (*key).user = newowner;
                (*key).uid = uid;
                ret = 0;
            } else {
                ret = -ENOMEM;
            }
        } else {
            ret = 0;
        }
        if ret == 0 && group != !0u32 { (*key).gid = gid; }
        if ret == 0 { notify_key(key, NOTIFY_KEY_SETATTR, 0); }
    }
    up_write(&mut (*key).sem);
    key_put(key);
    if !zapowner.is_null() { key_user_put(zapowner); }
    ret
}

extern "C" { fn key_user_lookup(uid: kuid_t) -> *mut key_user; fn key_user_put(user: *mut key_user); }

#[no_mangle]
pub unsafe extern "C" fn keyctl_setperm_key(id: key_serial_t, perm: key_perm_t) -> c_long {
    if perm & !(KEY_POS_ALL | KEY_USR_ALL | KEY_GRP_ALL | KEY_OTH_ALL) != 0 { return -EINVAL; }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE | KEY_LOOKUP_PARTIAL, KEY_NEED_SETATTR);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let key = key_ref_to_ptr(key_ref);
    let mut ret = -EACCES;
    down_write(&mut (*key).sem);
    if uid_eq((*key).uid, current_fsuid()) || capable(CAP_SYS_ADMIN) {
        (*key).perm = perm;
        notify_key(key, NOTIFY_KEY_SETATTR, 0);
        ret = 0;
    }
    up_write(&mut (*key).sem);
    key_put(key);
    ret
}

unsafe fn get_instantiation_keyring(ringid: key_serial_t, rka: *mut request_key_auth, _dest_keyring: *mut *mut key) -> c_long {
    *_dest_keyring = ptr::null_mut();
    if ringid == 0 { return 0; }
    if ringid > 0 {
        let dkref = lookup_user_key(ringid, KEY_LOOKUP_CREATE, KEY_NEED_WRITE);
        if IS_ERR(dkref) { return PTR_ERR(dkref); }
        *_dest_keyring = key_ref_to_ptr(dkref);
        return 0;
    }
    if ringid == KEY_SPEC_REQKEY_AUTH_KEY { return -EINVAL; }
    if ringid >= KEY_SPEC_REQUESTOR_KEYRING {
        *_dest_keyring = key_get((*rka).dest_keyring);
        return 0;
    }
    -ENOKEY
}

unsafe fn keyctl_change_reqkey_auth(key: *mut key) -> c_int {
    let new = prepare_creds();
    if new.is_null() { return -(ENOMEM as c_int); }
    key_put((*new).request_key_auth);
    (*new).request_key_auth = key_get(key);
    commit_creds(new)
}

unsafe fn keyctl_instantiate_key_common(id: key_serial_t, mut from: *mut iov_iter, ringid: key_serial_t) -> c_long {
    let cred = current_cred();
    let plen = if !from.is_null() { iov_iter_count(from) } else { 0 };
    if plen == 0 { from = ptr::null_mut(); }
    if plen > 1024 * 1024 - 1 { return -EINVAL; }
    let instkey = (*cred).request_key_auth;
    if instkey.is_null() { return -EPERM; }
    let rka = request_key_auth_get(instkey);
    if rka.is_null() { return -EKEYREVOKED; }
    if (*(*rka).target_key).serial != id { request_key_auth_put(rka); return -EPERM; }
    let mut payload: *mut c_void = ptr::null_mut();
    let mut ret;
    if !from.is_null() {
        payload = kvmalloc(plen, GFP_KERNEL);
        if payload.is_null() { request_key_auth_put(rka); return -ENOMEM; }
        if !copy_from_iter_full(payload, plen, from) {
            kvfree_sensitive(payload, plen);
            request_key_auth_put(rka);
            return -EFAULT;
        }
    }
    let mut dest_keyring: *mut key = ptr::null_mut();
    ret = get_instantiation_keyring(ringid, rka, &mut dest_keyring);
    if ret >= 0 {
        ret = key_instantiate_and_link((*rka).target_key, payload, plen, dest_keyring, instkey);
        key_put(dest_keyring);
        if ret == 0 { keyctl_change_reqkey_auth(ptr::null_mut()); }
    }
    kvfree_sensitive(payload, plen);
    request_key_auth_put(rka);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_instantiate_key(id: key_serial_t, _payload: *const c_void, plen: size_t, ringid: key_serial_t) -> c_long {
    if !_payload.is_null() && plen != 0 {
        let mut from = iov_iter { _private: [] };
        let ret = import_ubuf(ITER_SOURCE, _payload as *mut c_void, plen, &mut from);
        if unlikely(ret) != 0 { return ret as c_long; }
        return keyctl_instantiate_key_common(id, &mut from, ringid);
    }
    keyctl_instantiate_key_common(id, ptr::null_mut(), ringid)
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_instantiate_key_iov(id: key_serial_t, _payload_iov: *const iovec, mut ioc: c_uint, ringid: key_serial_t) -> c_long {
    let mut iovstack: [iovec; UIO_FASTIOV] = [iovec { iov_base: ptr::null_mut(), iov_len: 0 }; UIO_FASTIOV];
    let mut iov: *mut iovec = iovstack.as_mut_ptr();
    let mut from = iov_iter { _private: [] };
    if _payload_iov.is_null() { ioc = 0; }
    let mut ret = import_iovec(ITER_SOURCE, _payload_iov, ioc, UIO_FASTIOV as c_uint, &mut iov, &mut from);
    if ret < 0 { return ret; }
    ret = keyctl_instantiate_key_common(id, &mut from, ringid);
    kfree(iov as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_negate_key(id: key_serial_t, timeout: c_uint, ringid: key_serial_t) -> c_long {
    keyctl_reject_key(id, timeout, ENOKEY as c_uint, ringid)
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_reject_key(id: key_serial_t, timeout: c_uint, error: c_uint, ringid: key_serial_t) -> c_long {
    if error <= 0 || error >= MAX_ERRNO || error == ERESTARTSYS || error == ERESTARTNOINTR || error == ERESTARTNOHAND || error == ERESTART_RESTARTBLOCK { return -EINVAL; }
    let cred = current_cred();
    let instkey = (*cred).request_key_auth;
    if instkey.is_null() { return -EPERM; }
    let rka = request_key_auth_get(instkey);
    if rka.is_null() { return -EKEYREVOKED; }
    if (*(*rka).target_key).serial != id { request_key_auth_put(rka); return -EPERM; }
    let mut dest_keyring: *mut key = ptr::null_mut();
    let mut ret = get_instantiation_keyring(ringid, rka, &mut dest_keyring);
    if ret >= 0 {
        ret = key_reject_and_link((*rka).target_key, timeout, error, dest_keyring, instkey);
        key_put(dest_keyring);
        if ret == 0 { keyctl_change_reqkey_auth(ptr::null_mut()); }
    }
    request_key_auth_put(rka);
    ret
}

const KEY_REQKEY_DEFL_NO_CHANGE: c_int = -1;
const KEY_REQKEY_DEFL_THREAD_KEYRING: c_int = 1;
const KEY_REQKEY_DEFL_PROCESS_KEYRING: c_int = 2;
const KEY_REQKEY_DEFL_DEFAULT: c_int = 0;
const KEY_REQKEY_DEFL_SESSION_KEYRING: c_int = 3;
const KEY_REQKEY_DEFL_USER_KEYRING: c_int = 4;
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: c_int = 5;
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: c_int = 6;
const KEY_REQKEY_DEFL_GROUP_KEYRING: c_int = 7;

#[no_mangle]
pub unsafe extern "C" fn keyctl_set_reqkey_keyring(reqkey_defl: c_int) -> c_long {
    let old_setting = (*current_cred()).jit_keyring;
    if reqkey_defl == KEY_REQKEY_DEFL_NO_CHANGE { return old_setting as c_long; }
    let new = prepare_creds();
    if new.is_null() { return -ENOMEM; }
    let mut ret: c_int = 0;
    match reqkey_defl {
        KEY_REQKEY_DEFL_THREAD_KEYRING => { ret = install_thread_keyring_to_cred(new); if ret < 0 { abort_creds(new); return ret as c_long; } }
        KEY_REQKEY_DEFL_PROCESS_KEYRING => { ret = install_process_keyring_to_cred(new); if ret < 0 { abort_creds(new); return ret as c_long; } }
        KEY_REQKEY_DEFL_DEFAULT | KEY_REQKEY_DEFL_SESSION_KEYRING | KEY_REQKEY_DEFL_USER_KEYRING | KEY_REQKEY_DEFL_USER_SESSION_KEYRING | KEY_REQKEY_DEFL_REQUESTOR_KEYRING => {}
        _ => { abort_creds(new); return -EINVAL; }
    }
    (*new).jit_keyring = reqkey_defl;
    commit_creds(new);
    old_setting as c_long
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_set_timeout(id: key_serial_t, timeout: c_uint) -> c_long {
    let mut key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE | KEY_LOOKUP_PARTIAL, KEY_NEED_SETATTR);
    if IS_ERR(key_ref) {
        if PTR_ERR(key_ref) == -EACCES {
            let instkey = key_get_instantiation_authkey(id);
            if !IS_ERR(instkey) {
                key_put(instkey);
                key_ref = lookup_user_key(id, KEY_LOOKUP_PARTIAL, KEY_AUTHTOKEN_OVERRIDE);
            }
        }
        if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    }
    let key = key_ref_to_ptr(key_ref);
    let ret = if test_bit(KEY_FLAG_KEEP, &(*key).flags) { -EPERM } else { key_set_timeout(key, timeout); notify_key(key, NOTIFY_KEY_SETATTR, 0); 0 };
    key_put(key);
    ret
}

extern "C" { fn key_set_timeout(key: *mut key, timeout: c_uint); }

#[no_mangle]
pub unsafe extern "C" fn keyctl_assume_authority(id: key_serial_t) -> c_long {
    if id < 0 { return -EINVAL; }
    if id == 0 { return keyctl_change_reqkey_auth(ptr::null_mut()) as c_long; }
    let authkey = key_get_instantiation_authkey(id);
    if IS_ERR(authkey) { return PTR_ERR(authkey); }
    let mut ret = keyctl_change_reqkey_auth(authkey) as c_long;
    if ret == 0 { ret = (*authkey).serial as c_long; }
    key_put(authkey);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_get_security(keyid: key_serial_t, buffer: *mut c_char, mut buflen: size_t) -> c_long {
    let mut key_ref = lookup_user_key(keyid, KEY_LOOKUP_PARTIAL, KEY_NEED_VIEW);
    if IS_ERR(key_ref) {
        if PTR_ERR(key_ref) != -EACCES { return PTR_ERR(key_ref); }
        let instkey = key_get_instantiation_authkey(keyid);
        if IS_ERR(instkey) { return PTR_ERR(instkey); }
        key_put(instkey);
        key_ref = lookup_user_key(keyid, KEY_LOOKUP_PARTIAL, KEY_AUTHTOKEN_OVERRIDE);
        if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    }
    let key = key_ref_to_ptr(key_ref);
    let mut context: *mut c_char = ptr::null_mut();
    let mut ret = security_key_getsecurity(key, &mut context);
    if ret == 0 {
        ret = 1;
        if !buffer.is_null() && buflen > 0 && copy_to_user(buffer as *mut c_void, b"\0".as_ptr() as *const c_void, 1) != 0 { ret = -EFAULT; }
    } else if ret > 0 {
        if !buffer.is_null() && buflen > 0 {
            if buflen > ret as size_t { buflen = ret as size_t; }
            if copy_to_user(buffer as *mut c_void, context as *const c_void, buflen) != 0 { ret = -EFAULT; }
        }
        kfree(context as *mut c_void);
    }
    key_ref_put(key_ref);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_session_to_parent() -> c_long {
    let mut keyring_r = lookup_user_key(KEY_SPEC_SESSION_KEYRING, 0, KEY_NEED_LINK);
    if IS_ERR(keyring_r) { return PTR_ERR(keyring_r); }
    let cred = cred_alloc_blank();
    if cred.is_null() { key_ref_put(keyring_r); return -ENOMEM; }
    let mut newwork = &mut (*cred).rcu as *mut callback_head;
    (*cred).session_keyring = key_ref_to_ptr(keyring_r);
    keyring_r = ptr::null_mut();
    init_task_work(newwork, key_change_session_keyring);
    let me = current;
    rcu_read_lock();
    write_lock_irq(&mut tasklist_lock);
    let mut ret = -EPERM;
    let mut oldwork: *mut callback_head = ptr::null_mut();
    let parent = (*me).real_parent;
    if !parent.is_null() && (*parent).pid > 1 && !(*parent).mm.is_null() && thread_group_empty(parent) {
        let mycred = current_cred();
        let pcred = __task_cred(parent);
        if mycred == pcred || (*mycred).session_keyring == (*pcred).session_keyring {
            ret = 0;
        } else if uid_eq((*pcred).uid, (*mycred).euid) && uid_eq((*pcred).euid, (*mycred).euid) && uid_eq((*pcred).suid, (*mycred).euid) &&
                  gid_eq((*pcred).gid, (*mycred).egid) && gid_eq((*pcred).egid, (*mycred).egid) && gid_eq((*pcred).sgid, (*mycred).egid) &&
                  (((*pcred).session_keyring.is_null()) || uid_eq((*(*pcred).session_keyring).uid, (*mycred).euid)) &&
                  uid_eq((*(*mycred).session_keyring).uid, (*mycred).euid) {
            oldwork = task_work_cancel_func(parent, key_change_session_keyring);
            ret = task_work_add(parent, newwork, TWA_RESUME) as c_long;
            if ret == 0 { newwork = ptr::null_mut(); }
        }
    }
    write_unlock_irq(&mut tasklist_lock);
    rcu_read_unlock();
    if !oldwork.is_null() { /* container_of(oldwork, struct cred, rcu) */ put_cred(oldwork as *mut cred); }
    if !newwork.is_null() { put_cred(cred); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_restrict_keyring(id: key_serial_t, _type: *const c_char, _restriction: *const c_char) -> c_long {
    let key_ref = lookup_user_key(id, 0, KEY_NEED_SETATTR);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let mut type_: [c_char; 32] = [0; 32];
    let mut restriction: *mut c_char = ptr::null_mut();
    let mut ret = -EINVAL;
    if !_type.is_null() {
        if _restriction.is_null() { key_ref_put(key_ref); return ret; }
        ret = key_get_type_from_user(type_.as_mut_ptr(), _type, type_.len() as c_uint) as c_long;
        if ret < 0 { key_ref_put(key_ref); return ret; }
        restriction = strndup_user(_restriction, PAGE_SIZE as c_long);
        if IS_ERR(restriction) { ret = PTR_ERR(restriction); key_ref_put(key_ref); return ret; }
    } else if !_restriction.is_null() {
        key_ref_put(key_ref);
        return ret;
    }
    ret = keyring_restrict(key_ref, if !_type.is_null() { type_.as_ptr() } else { ptr::null() }, restriction);
    kfree(restriction as *mut c_void);
    key_ref_put(key_ref);
    ret
}

/* CONFIG_KEY_NOTIFICATIONS conditional in the C source. */
#[no_mangle]
pub unsafe extern "C" fn keyctl_watch_key(id: key_serial_t, watch_queue_fd: c_int, watch_id: c_int) -> c_long {
    if watch_id < -1 || watch_id > 0xff { return -EINVAL; }
    let key_ref = lookup_user_key(id, KEY_LOOKUP_CREATE, KEY_NEED_VIEW);
    if IS_ERR(key_ref) { return PTR_ERR(key_ref); }
    let key = key_ref_to_ptr(key_ref);
    let wqueue = get_watch_queue(watch_queue_fd);
    if IS_ERR(wqueue) { let ret = PTR_ERR(wqueue); key_put(key); return ret; }
    let ret = if watch_id >= 0 {
        let mut wlist: *mut watch_list = ptr::null_mut();
        if (*key).watchers.is_null() {
            wlist = kvmalloc(core::mem::size_of::<watch_list>(), GFP_KERNEL) as *mut watch_list;
            if wlist.is_null() { put_watch_queue(wqueue); key_put(key); return -ENOMEM; }
            init_watch_list(wlist, ptr::null_mut());
        }
        let watch = kvmalloc(core::mem::size_of::<watch>(), GFP_KERNEL) as *mut watch;
        if watch.is_null() { kfree(wlist as *mut c_void); put_watch_queue(wqueue); key_put(key); return -ENOMEM; }
        init_watch(watch, wqueue);
        (*watch).id = (*key).serial;
        (*watch).info_id = (watch_id as u32) << WATCH_INFO_ID__SHIFT;
        let mut r = security_watch_key(key);
        if r >= 0 {
            down_write(&mut (*key).sem);
            if (*key).watchers.is_null() { (*key).watchers = wlist; wlist = ptr::null_mut(); }
            r = add_watch_to_object(watch, (*key).watchers);
            up_write(&mut (*key).sem);
        }
        if r != 0 { kfree(watch as *mut c_void); }
        kfree(wlist as *mut c_void);
        r
    } else {
        let mut r = -EBADSLT;
        if !(*key).watchers.is_null() {
            down_write(&mut (*key).sem);
            r = remove_watch_from_object((*key).watchers, wqueue, key_serial(key), false);
            up_write(&mut (*key).sem);
        }
        r
    };
    put_watch_queue(wqueue);
    key_put(key);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn keyctl_capabilities(_buffer: *mut u8, buflen: size_t) -> c_long {
    let mut size = buflen;
    if size > 0 {
        if size > keyrings_capabilities.len() { size = keyrings_capabilities.len(); }
        if copy_to_user(_buffer as *mut c_void, keyrings_capabilities.as_ptr() as *const c_void, size) != 0 { return -EFAULT; }
        if size < buflen && clear_user(_buffer.add(size) as *mut c_void, buflen - size) != 0 { return -EFAULT; }
    }
    keyrings_capabilities.len() as c_long
}

const KEYCTL_GET_KEYRING_ID: c_int = 0;
const KEYCTL_JOIN_SESSION_KEYRING: c_int = 1;
const KEYCTL_UPDATE: c_int = 2;
const KEYCTL_REVOKE: c_int = 3;
const KEYCTL_DESCRIBE: c_int = 4;
const KEYCTL_CLEAR: c_int = 5;
const KEYCTL_LINK: c_int = 6;
const KEYCTL_UNLINK: c_int = 7;
const KEYCTL_SEARCH: c_int = 8;
const KEYCTL_READ: c_int = 11;
const KEYCTL_CHOWN: c_int = 12;
const KEYCTL_SETPERM: c_int = 13;
const KEYCTL_INSTANTIATE: c_int = 14;
const KEYCTL_NEGATE: c_int = 15;
const KEYCTL_SET_REQKEY_KEYRING: c_int = 16;
const KEYCTL_SET_TIMEOUT: c_int = 17;
const KEYCTL_ASSUME_AUTHORITY: c_int = 18;
const KEYCTL_GET_SECURITY: c_int = 19;
const KEYCTL_SESSION_TO_PARENT: c_int = 20;
const KEYCTL_REJECT: c_int = 21;
const KEYCTL_INSTANTIATE_IOV: c_int = 22;
const KEYCTL_INVALIDATE: c_int = 23;
const KEYCTL_GET_PERSISTENT: c_int = 24;
const KEYCTL_DH_COMPUTE: c_int = 25;
const KEYCTL_RESTRICT_KEYRING: c_int = 29;
const KEYCTL_PKEY_QUERY: c_int = 30;
const KEYCTL_PKEY_ENCRYPT: c_int = 31;
const KEYCTL_PKEY_DECRYPT: c_int = 32;
const KEYCTL_PKEY_SIGN: c_int = 33;
const KEYCTL_PKEY_VERIFY: c_int = 34;
const KEYCTL_MOVE: c_int = 35;
const KEYCTL_CAPABILITIES: c_int = 36;
const KEYCTL_WATCH_KEY: c_int = 37;

#[no_mangle]
pub unsafe extern "C" fn sys_keyctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_long {
    match option {
        KEYCTL_GET_KEYRING_ID => keyctl_get_keyring_ID(arg2 as key_serial_t, arg3 as c_int),
        KEYCTL_JOIN_SESSION_KEYRING => keyctl_join_session_keyring(arg2 as *const c_char),
        KEYCTL_UPDATE => keyctl_update_key(arg2 as key_serial_t, arg3 as *const c_void, arg4 as size_t),
        KEYCTL_REVOKE => keyctl_revoke_key(arg2 as key_serial_t),
        KEYCTL_DESCRIBE => keyctl_describe_key(arg2 as key_serial_t, arg3 as *mut c_char, arg4 as size_t),
        KEYCTL_CLEAR => keyctl_keyring_clear(arg2 as key_serial_t),
        KEYCTL_LINK => keyctl_keyring_link(arg2 as key_serial_t, arg3 as key_serial_t),
        KEYCTL_UNLINK => keyctl_keyring_unlink(arg2 as key_serial_t, arg3 as key_serial_t),
        KEYCTL_SEARCH => keyctl_keyring_search(arg2 as key_serial_t, arg3 as *const c_char, arg4 as *const c_char, arg5 as key_serial_t),
        KEYCTL_READ => keyctl_read_key(arg2 as key_serial_t, arg3 as *mut c_char, arg4 as size_t),
        KEYCTL_CHOWN => keyctl_chown_key(arg2 as key_serial_t, arg3 as uid_t, arg4 as gid_t),
        KEYCTL_SETPERM => keyctl_setperm_key(arg2 as key_serial_t, arg3 as key_perm_t),
        KEYCTL_INSTANTIATE => keyctl_instantiate_key(arg2 as key_serial_t, arg3 as *const c_void, arg4 as size_t, arg5 as key_serial_t),
        KEYCTL_NEGATE => keyctl_negate_key(arg2 as key_serial_t, arg3 as c_uint, arg4 as key_serial_t),
        KEYCTL_SET_REQKEY_KEYRING => keyctl_set_reqkey_keyring(arg2 as c_int),
        KEYCTL_SET_TIMEOUT => keyctl_set_timeout(arg2 as key_serial_t, arg3 as c_uint),
        KEYCTL_ASSUME_AUTHORITY => keyctl_assume_authority(arg2 as key_serial_t),
        KEYCTL_GET_SECURITY => keyctl_get_security(arg2 as key_serial_t, arg3 as *mut c_char, arg4 as size_t),
        KEYCTL_SESSION_TO_PARENT => keyctl_session_to_parent(),
        KEYCTL_REJECT => keyctl_reject_key(arg2 as key_serial_t, arg3 as c_uint, arg4 as c_uint, arg5 as key_serial_t),
        KEYCTL_INSTANTIATE_IOV => keyctl_instantiate_key_iov(arg2 as key_serial_t, arg3 as *const iovec, arg4 as c_uint, arg5 as key_serial_t),
        KEYCTL_INVALIDATE => keyctl_invalidate_key(arg2 as key_serial_t),
        KEYCTL_GET_PERSISTENT => keyctl_get_persistent(arg2 as uid_t, arg3 as key_serial_t),
        KEYCTL_DH_COMPUTE => keyctl_dh_compute(arg2 as *mut keyctl_dh_params, arg3 as *mut c_char, arg4 as size_t, arg5 as *mut keyctl_kdf_params),
        KEYCTL_RESTRICT_KEYRING => keyctl_restrict_keyring(arg2 as key_serial_t, arg3 as *const c_char, arg4 as *const c_char),
        KEYCTL_PKEY_QUERY => {
            if arg3 != 0 { -EINVAL } else { keyctl_pkey_query(arg2 as key_serial_t, arg4 as *const c_char, arg5 as *mut keyctl_pkey_query) }
        }
        KEYCTL_PKEY_ENCRYPT | KEYCTL_PKEY_DECRYPT | KEYCTL_PKEY_SIGN => keyctl_pkey_e_d_s(option, arg2 as *const keyctl_pkey_params, arg3 as *const c_char, arg4 as *const c_void, arg5 as *mut c_void),
        KEYCTL_PKEY_VERIFY => keyctl_pkey_verify(arg2 as *const keyctl_pkey_params, arg3 as *const c_char, arg4 as *const c_void, arg5 as *const c_void),
        KEYCTL_MOVE => keyctl_keyring_move(arg2 as key_serial_t, arg3 as key_serial_t, arg4 as key_serial_t, arg5 as c_uint),
        KEYCTL_CAPABILITIES => keyctl_capabilities(arg2 as *mut u8, arg3 as size_t),
        KEYCTL_WATCH_KEY => keyctl_watch_key(arg2 as key_serial_t, arg3 as c_int, arg4 as c_int),
        _ => -EOPNOTSUPP,
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
