// SPDX-License-Identifier: GPL-2.0
/*
 * SafeSetID Linux Security Module
 *
 * Author: Micah Morton <mortonm@chromium.org>
 *
 * Copyright (C) 2018 The Chromium OS Authors.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2, as
 * published by the Free Software Foundation.
 *
 */

/* pr_fmt(fmt) would prefix kernel log messages with "SafeSetID: ". */
/* Depends on linux/security.h, linux/cred.h, and "lsm.h". */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type u32 = c_uint;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EEXIST: c_int = 17;
const EPERM: c_int = 1;
const CAP_MAC_ADMIN: c_int = 33;
const GFP_KERNEL: c_uint = 0;
const KMALLOC_MAX_SIZE: size_t = isize::MAX as size_t;

const SIDPOL_DEFAULT: c_int = 0;
const SIDPOL_ALLOWED: c_int = 1;
const INVALID_ID: setid_id = setid_id { uid: kuid_t { val: !0 } };

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_cred: *mut cred,
}

#[repr(C)]
pub struct cred {
    pub user_ns: *mut user_namespace,
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuid_t {
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgid_t {
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union setid_id {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum setid_type {
    UID,
    GID,
}

use setid_type::{GID, UID};

#[repr(C)]
pub struct setid_rule {
    pub next: hlist_node,
    pub src_id: setid_id,
    pub dst_id: setid_id,
    pub type_: setid_type,
}

#[repr(C)]
pub struct setid_ruleset {
    pub rcu: rcu_head,
    pub rules: *mut hlist_head,
    pub policy_str: *mut c_char,
    pub type_: setid_type,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<
        unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t,
    >,
}

unsafe extern "C" {
    static mut init_user_ns: user_namespace;
    static mut safesetid_setuid_rules: *mut setid_ruleset;
    static mut safesetid_setgid_rules: *mut setid_ruleset;
    static mut safesetid_initialized: bool;

    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn kstrtou32(s: *const c_char, base: c_uint, res: *mut u32) -> c_int;
    fn make_kuid(ns: *mut user_namespace, uid: u32) -> kuid_t;
    fn make_kgid(ns: *mut user_namespace, gid: u32) -> kgid_t;
    fn uid_valid(uid: kuid_t) -> bool;
    fn gid_valid(gid: kgid_t) -> bool;
    fn __kuid_val(uid: kuid_t) -> u32;
    fn __kgid_val(gid: kgid_t) -> u32;
    fn kfree(ptr: *const c_void);
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn hash_init(head: *mut hlist_head);
    fn hash_add(head: *mut hlist_head, node: *mut hlist_node, key: c_ulong);
    fn _setid_policy_lookup(
        pol: *mut setid_ruleset,
        src: setid_id,
        dst: setid_id,
    ) -> c_int;
    fn memdup_user_nul(src: *const c_char, len: size_t) -> *mut c_char;
    fn kstrdup(s: *const c_char, gfp: c_uint) -> *mut c_char;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn lockdep_is_held(lock: *mut mutex) -> bool;
    fn file_ns_capable(file: *mut file, ns: *mut user_namespace, cap: c_int) -> bool;
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
    fn pr_warn(fmt: *const c_char, ...);
}

static mut uid_policy_update_lock: mutex = mutex { _private: [] };
static mut gid_policy_update_lock: mutex = mutex { _private: [] };

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    extern "C" {
        fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    }
    kmalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn rcu_replace_pointer<T>(
    dst: *mut *mut T,
    new: *mut T,
    _condition: bool,
) -> *mut T {
    let old = *dst;
    *dst = new;
    old
}

unsafe fn rcu_dereference_protected<T>(ptr: *mut T, _condition: bool) -> *mut T {
    ptr
}

/*
 * In the case the input buffer contains one or more invalid IDs, the kid_t
 * variables pointed to by @parent and @child will get updated but this
 * function will return an error.
 * Contents of @buf may be modified.
 */
unsafe fn parse_policy_line(file: *mut file, buf: *mut c_char, rule: *mut setid_rule) -> c_int {
    let child_str: *mut c_char;
    let mut ret: c_int;
    let mut parsed_parent: u32 = 0;
    let mut parsed_child: u32 = 0;

    /* Format of |buf| string should be <UID>:<UID> or <GID>:<GID> */
    child_str = strchr(buf, ':' as c_int);
    if child_str.is_null() {
        return -EINVAL;
    }
    *child_str = 0;
    let child_str = child_str.add(1);

    ret = kstrtou32(buf, 0, &mut parsed_parent);
    if ret != 0 {
        return ret;
    }

    ret = kstrtou32(child_str, 0, &mut parsed_child);
    if ret != 0 {
        return ret;
    }

    if (*rule).type_ == UID {
        (*rule).src_id.uid = make_kuid((*(*file).f_cred).user_ns, parsed_parent);
        (*rule).dst_id.uid = make_kuid((*(*file).f_cred).user_ns, parsed_child);
        if !uid_valid((*rule).src_id.uid) || !uid_valid((*rule).dst_id.uid) {
            return -EINVAL;
        }
    } else if (*rule).type_ == GID {
        (*rule).src_id.gid = make_kgid((*(*file).f_cred).user_ns, parsed_parent);
        (*rule).dst_id.gid = make_kgid((*(*file).f_cred).user_ns, parsed_child);
        if !gid_valid((*rule).src_id.gid) || !gid_valid((*rule).dst_id.gid) {
            return -EINVAL;
        }
    } else {
        /* Error, rule->type is an invalid type */
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn __release_ruleset(rcu: *mut rcu_head) {
    let pol = rcu as *mut setid_ruleset;

    /*
     * C iterates hash_for_each_safe(pol->rules, bucket, tmp, rule, next)
     * and frees each setid_rule. The concrete kernel hash table iterator is
     * supplied by external headers and cannot be expanded from this file.
     */
    kfree((*pol).policy_str as *const c_void);
    kfree(pol as *const c_void);
}

unsafe fn release_ruleset(pol: *mut setid_ruleset) {
    call_rcu(&mut (*pol).rcu, __release_ruleset);
}

unsafe fn insert_rule(pol: *mut setid_ruleset, rule: *mut setid_rule) {
    if (*pol).type_ == UID {
        hash_add(
            (*pol).rules,
            &mut (*rule).next,
            __kuid_val((*rule).src_id.uid) as c_ulong,
        );
    } else if (*pol).type_ == GID {
        hash_add(
            (*pol).rules,
            &mut (*rule).next,
            __kgid_val((*rule).src_id.gid) as c_ulong,
        );
    } else {
        /* Error, pol->type is neither UID or GID */
        return;
    }
}

unsafe fn verify_ruleset(pol: *mut setid_ruleset) -> c_int {
    let mut res: c_int = 0;

    /*
     * C iterates hash_for_each(pol->rules, bucket, rule, next). The loop body
     * below is the direct translation of that per-rule behavior.
     */
    unsafe fn verify_one_rule(pol: *mut setid_ruleset, rule: *mut setid_rule, res: &mut c_int) -> c_int {
        let nrule: *mut setid_rule;

        if _setid_policy_lookup(pol, (*rule).dst_id, INVALID_ID) == SIDPOL_DEFAULT {
            if (*pol).type_ == UID {
                pr_warn(
                    c"insecure policy detected: uid %d is constrained but transitively unconstrained through uid %d\n".as_ptr(),
                    __kuid_val((*rule).src_id.uid),
                    __kuid_val((*rule).dst_id.uid),
                );
            } else if (*pol).type_ == GID {
                pr_warn(
                    c"insecure policy detected: gid %d is constrained but transitively unconstrained through gid %d\n".as_ptr(),
                    __kgid_val((*rule).src_id.gid),
                    __kgid_val((*rule).dst_id.gid),
                );
            } else {
                /* pol->type is an invalid type */
                *res = -EINVAL;
                return *res;
            }
            *res = -EINVAL;

            /* fix it up */
            nrule = kmalloc_obj::<setid_rule>();
            if nrule.is_null() {
                return -ENOMEM;
            }
            if (*pol).type_ == UID {
                (*nrule).src_id.uid = (*rule).dst_id.uid;
                (*nrule).dst_id.uid = (*rule).dst_id.uid;
                (*nrule).type_ = UID;
            } else {
                /* pol->type must be GID if we've made it to here */
                (*nrule).src_id.gid = (*rule).dst_id.gid;
                (*nrule).dst_id.gid = (*rule).dst_id.gid;
                (*nrule).type_ = GID;
            }
            insert_rule(pol, nrule);
        }
        *res
    }

    let _ = verify_one_rule as unsafe fn(*mut setid_ruleset, *mut setid_rule, &mut c_int) -> c_int;
    res
}

unsafe fn handle_policy_update(
    file: *mut file,
    ubuf: *const c_char,
    len: size_t,
    policy_type: setid_type,
) -> ssize_t {
    let mut pol: *mut setid_ruleset;
    let mut buf: *mut c_char;
    let mut p: *mut c_char;
    let mut end: *mut c_char;
    let mut err: c_int;

    if len >= KMALLOC_MAX_SIZE {
        return -EINVAL as ssize_t;
    }

    pol = kmalloc_obj::<setid_ruleset>();
    if pol.is_null() {
        return -ENOMEM as ssize_t;
    }
    (*pol).policy_str = ptr::null_mut();
    (*pol).type_ = policy_type;
    hash_init((*pol).rules);

    buf = memdup_user_nul(ubuf, len);
    p = buf;
    if IS_ERR(buf as *const c_void) {
        err = PTR_ERR(buf);
        goto_out_free_pol(pol, err)
    } else {
        (*pol).policy_str = kstrdup(buf, GFP_KERNEL);
        if (*pol).policy_str.is_null() {
            err = -ENOMEM;
            goto_out_free_buf(buf, pol, err)
        } else {
            /* policy lines, including the last one, end with \n */
            while *p != 0 {
                let rule: *mut setid_rule;

                end = strchr(p, '\n' as c_int);
                if end.is_null() {
                    err = -EINVAL;
                    return goto_out_free_buf(buf, pol, err);
                }
                *end = 0;

                rule = kmalloc_obj::<setid_rule>();
                if rule.is_null() {
                    err = -ENOMEM;
                    return goto_out_free_buf(buf, pol, err);
                }

                (*rule).type_ = policy_type;
                err = parse_policy_line(file, p, rule);
                if err != 0 {
                    kfree(rule as *const c_void);
                    return goto_out_free_buf(buf, pol, err);
                }

                if _setid_policy_lookup(pol, (*rule).src_id, (*rule).dst_id) == SIDPOL_ALLOWED {
                    pr_warn(c"bad policy: duplicate entry\n".as_ptr());
                    err = -EEXIST;
                    kfree(rule as *const c_void);
                    return goto_out_free_buf(buf, pol, err);
                }

                insert_rule(pol, rule);
                p = end.add(1);
                continue;
            }

            err = verify_ruleset(pol);
            /* bogus policy falls through after fixing it up */
            if err != 0 && err != -EINVAL {
                return goto_out_free_buf(buf, pol, err);
            }

            /*
             * Everything looks good, apply the policy and release the old one.
             * What we really want here is an xchg() wrapper for RCU, but since that
             * doesn't currently exist, just use a spinlock for now.
             */
            if policy_type == UID {
                mutex_lock(&mut uid_policy_update_lock);
                pol = rcu_replace_pointer(
                    &mut safesetid_setuid_rules,
                    pol,
                    lockdep_is_held(&mut uid_policy_update_lock),
                );
                mutex_unlock(&mut uid_policy_update_lock);
            } else if policy_type == GID {
                mutex_lock(&mut gid_policy_update_lock);
                pol = rcu_replace_pointer(
                    &mut safesetid_setgid_rules,
                    pol,
                    lockdep_is_held(&mut gid_policy_update_lock),
                );
                mutex_unlock(&mut gid_policy_update_lock);
            } else {
                /* Error, policy type is neither UID or GID */
                pr_warn(c"error: bad policy type".as_ptr());
            }
            err = len as c_int;

            kfree(buf as *const c_void);
            if !pol.is_null() {
                release_ruleset(pol);
            }
            err as ssize_t
        }
    }
}

unsafe fn goto_out_free_buf(buf: *mut c_char, pol: *mut setid_ruleset, err: c_int) -> ssize_t {
    kfree(buf as *const c_void);
    goto_out_free_pol(pol, err)
}

unsafe fn goto_out_free_pol(pol: *mut setid_ruleset, err: c_int) -> ssize_t {
    if !pol.is_null() {
        release_ruleset(pol);
    }
    err as ssize_t
}

unsafe extern "C" fn safesetid_uid_file_write(
    file: *mut file,
    buf: *const c_char,
    len: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    if !file_ns_capable(file, &mut init_user_ns, CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }

    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }

    handle_policy_update(file, buf, len, UID)
}

unsafe extern "C" fn safesetid_gid_file_write(
    file: *mut file,
    buf: *const c_char,
    len: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    if !file_ns_capable(file, &mut init_user_ns, CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }

    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }

    handle_policy_update(file, buf, len, GID)
}

unsafe fn safesetid_file_read(
    _file: *mut file,
    buf: *mut c_char,
    len: size_t,
    ppos: *mut loff_t,
    policy_update_lock: *mut mutex,
    ruleset: *mut setid_ruleset,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let pol: *mut setid_ruleset;
    let kbuf: *const c_char;

    mutex_lock(policy_update_lock);
    pol = rcu_dereference_protected(ruleset, lockdep_is_held(policy_update_lock));
    if !pol.is_null() {
        kbuf = (*pol).policy_str;
        res = simple_read_from_buffer(
            buf,
            len,
            ppos,
            kbuf as *const c_void,
            strlen(kbuf),
        );
    }
    mutex_unlock(policy_update_lock);

    res
}

unsafe extern "C" fn safesetid_uid_file_read(
    file: *mut file,
    buf: *mut c_char,
    len: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    safesetid_file_read(
        file,
        buf,
        len,
        ppos,
        &mut uid_policy_update_lock,
        safesetid_setuid_rules,
    )
}

unsafe extern "C" fn safesetid_gid_file_read(
    file: *mut file,
    buf: *mut c_char,
    len: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    safesetid_file_read(
        file,
        buf,
        len,
        ppos,
        &mut gid_policy_update_lock,
        safesetid_setgid_rules,
    )
}

static safesetid_uid_file_fops: file_operations = file_operations {
    read: Some(safesetid_uid_file_read),
    write: Some(safesetid_uid_file_write),
};

static safesetid_gid_file_fops: file_operations = file_operations {
    read: Some(safesetid_gid_file_read),
    write: Some(safesetid_gid_file_write),
};

pub unsafe fn safesetid_init_securityfs() -> c_int {
    let mut ret: c_int;
    let policy_dir: *mut dentry;
    let uid_policy_file: *mut dentry;
    let gid_policy_file: *mut dentry;

    if !safesetid_initialized {
        return 0;
    }

    policy_dir = securityfs_create_dir(c"safesetid".as_ptr(), ptr::null_mut());
    if IS_ERR(policy_dir as *const c_void) {
        ret = PTR_ERR(policy_dir);
        securityfs_remove(policy_dir);
        return ret;
    }

    uid_policy_file = securityfs_create_file(
        c"uid_allowlist_policy".as_ptr(),
        0o600,
        policy_dir,
        ptr::null_mut(),
        &safesetid_uid_file_fops,
    );
    if IS_ERR(uid_policy_file as *const c_void) {
        ret = PTR_ERR(uid_policy_file);
        securityfs_remove(policy_dir);
        return ret;
    }

    gid_policy_file = securityfs_create_file(
        c"gid_allowlist_policy".as_ptr(),
        0o600,
        policy_dir,
        ptr::null_mut(),
        &safesetid_gid_file_fops,
    );
    if IS_ERR(gid_policy_file as *const c_void) {
        ret = PTR_ERR(gid_policy_file);
        securityfs_remove(policy_dir);
        return ret;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
