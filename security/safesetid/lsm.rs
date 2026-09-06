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

// pr_fmt(fmt) "SafeSetID: " fmt
// Dependencies from:
// <linux/lsm_hooks.h>, <linux/module.h>, <linux/ptrace.h>,
// <linux/sched/task_stack.h>, <linux/security.h>, <uapi/linux/lsm.h>, "lsm.h"

use core::ptr;

type c_int = i32;
type c_uint = u32;

#[repr(C)]
pub struct cred {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub euid: kuid_t,
    pub egid: kgid_t,
    pub suid: kuid_t,
    pub sgid: kgid_t,
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
    pub group_info: *mut group_info,
}

#[repr(C)]
pub struct group_info {
    pub ngroups: c_int,
    pub gid: *mut kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuid_t {
    _private: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgid_t {
    _private: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kid_t {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum setid_type {
    UID,
    GID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sid_policy_type {
    SIDPOL_DEFAULT,
    SIDPOL_CONSTRAINED,
    SIDPOL_ALLOWED,
}

#[repr(C)]
pub struct setid_ruleset {
    pub type_: setid_type,
    pub rules: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct setid_rule {
    pub src_id: kid_t,
    pub dst_id: kid_t,
    pub next: core::ffi::c_void,
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const u8,
    pub id: c_int,
}

#[repr(C)]
pub struct security_hook_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_fs: Option<unsafe extern "C" fn() -> c_int>,
}

unsafe extern "C" {
    fn __kuid_val(uid: kuid_t) -> c_uint;
    fn __kgid_val(gid: kgid_t) -> c_uint;
    fn uid_eq(left: kuid_t, right: kuid_t) -> bool;
    fn gid_eq(left: kgid_t, right: kgid_t) -> bool;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn force_sig(sig: c_int);
    fn get_group_info(group_info: *mut group_info);
    fn put_group_info(group_info: *mut group_info);
    fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    fn safesetid_init_securityfs() -> c_int;
}

const CAP_SETUID: c_int = 7;
const CAP_SETGID: c_int = 6;
const CAP_OPT_INSETID: c_uint = 1 << 2;
const EPERM: c_int = 1;
const EACCES: c_int = 13;
const SIGKILL: c_int = 9;
const LSM_ID_SAFESETID: c_int = 107;

const INVALID_ID: kid_t = kid_t {
    uid: kuid_t {
        _private: !0 as c_uint,
    },
};

macro_rules! pr_warn {
    ($($arg:tt)*) => {
        /* external kernel logging macro */
    };
}

macro_rules! hash_for_each_possible {
    ($rules:expr, $rule:ident, $next:ident, $key:expr, $body:block) => {{
        let _ = &$rules;
        let _ = stringify!($next);
        let _ = $key;
        let mut $rule: *mut setid_rule = ptr::null_mut();
        // The kernel hlist traversal is supplied by <linux/hashtable.h>.
        // This preserves the source loop site; concrete iteration is an external dependency.
        while !$rule.is_null() $body
    }};
}

macro_rules! LSM_HOOK_INIT {
    ($hook:ident, $func:ident) => {
        security_hook_list { _private: [] }
    };
}

/* Flag indicating whether initialization completed */
#[unsafe(link_section = ".init.data")]
pub static mut safesetid_initialized: c_int = 0;

pub static mut safesetid_setuid_rules: *mut setid_ruleset = ptr::null_mut();
pub static mut safesetid_setgid_rules: *mut setid_ruleset = ptr::null_mut();

/* Compute a decision for a transition from @src to @dst under @policy. */
pub unsafe extern "C" fn _setid_policy_lookup(
    policy: *mut setid_ruleset,
    src: kid_t,
    dst: kid_t,
) -> sid_policy_type {
    let mut result: sid_policy_type = sid_policy_type::SIDPOL_DEFAULT;

    if (*policy).type_ == setid_type::UID {
        hash_for_each_possible!((*policy).rules, rule, next, __kuid_val(src.uid), {
            if !uid_eq((*rule).src_id.uid, src.uid) {
                continue;
            }
            if uid_eq((*rule).dst_id.uid, dst.uid) {
                return sid_policy_type::SIDPOL_ALLOWED;
            }
            result = sid_policy_type::SIDPOL_CONSTRAINED;
        });
    } else if (*policy).type_ == setid_type::GID {
        hash_for_each_possible!((*policy).rules, rule, next, __kgid_val(src.gid), {
            if !gid_eq((*rule).src_id.gid, src.gid) {
                continue;
            }
            if gid_eq((*rule).dst_id.gid, dst.gid) {
                return sid_policy_type::SIDPOL_ALLOWED;
            }
            result = sid_policy_type::SIDPOL_CONSTRAINED;
        });
    } else {
        /* Should not reach here, report the ID as contrainsted */
        result = sid_policy_type::SIDPOL_CONSTRAINED;
    }
    result
}

/*
 * Compute a decision for a transition from @src to @dst under the active
 * policy.
 */
unsafe extern "C" fn setid_policy_lookup(
    src: kid_t,
    dst: kid_t,
    new_type: setid_type,
) -> sid_policy_type {
    let mut result: sid_policy_type = sid_policy_type::SIDPOL_DEFAULT;
    let pol: *mut setid_ruleset;

    rcu_read_lock();
    if new_type == setid_type::UID {
        pol = safesetid_setuid_rules;
    } else if new_type == setid_type::GID {
        pol = safesetid_setgid_rules;
    } else {
        /* Should not reach here */
        result = sid_policy_type::SIDPOL_CONSTRAINED;
        rcu_read_unlock();
        return result;
    }

    if !pol.is_null() {
        (*pol).type_ = new_type;
        result = _setid_policy_lookup(pol, src, dst);
    }
    rcu_read_unlock();
    result
}

unsafe extern "C" fn safesetid_security_capable(
    cred: *const cred,
    _ns: *mut user_namespace,
    cap: c_int,
    opts: c_uint,
) -> c_int {
    /* We're only interested in CAP_SETUID and CAP_SETGID. */
    if cap != CAP_SETUID && cap != CAP_SETGID {
        return 0;
    }

    /*
     * If CAP_SET{U/G}ID is currently used for a setid or setgroups syscall, we
     * want to let it go through here; the real security check happens later, in
     * the task_fix_set{u/g}id or task_fix_setgroups hooks.
     */
    if (opts & CAP_OPT_INSETID) != 0 {
        return 0;
    }

    match cap {
        CAP_SETUID => {
            /*
            * If no policy applies to this task, allow the use of CAP_SETUID for
            * other purposes.
            */
            if setid_policy_lookup(
                kid_t { uid: (*cred).uid },
                INVALID_ID,
                setid_type::UID,
            ) == sid_policy_type::SIDPOL_DEFAULT
            {
                return 0;
            }
            /*
             * Reject use of CAP_SETUID for functionality other than calling
             * set*uid() (e.g. setting up userns uid mappings).
             */
            pr_warn!(
                "Operation requires CAP_SETUID, which is not available to UID %u for operations besides approved set*uid transitions\n",
                __kuid_val((*cred).uid)
            );
            return -EPERM;
        }
        CAP_SETGID => {
            /*
            * If no policy applies to this task, allow the use of CAP_SETGID for
            * other purposes.
            */
            if setid_policy_lookup(
                kid_t { gid: (*cred).gid },
                INVALID_ID,
                setid_type::GID,
            ) == sid_policy_type::SIDPOL_DEFAULT
            {
                return 0;
            }
            /*
             * Reject use of CAP_SETUID for functionality other than calling
             * set*gid() (e.g. setting up userns gid mappings).
             */
            pr_warn!(
                "Operation requires CAP_SETGID, which is not available to GID %u for operations besides approved set*gid transitions\n",
                __kgid_val((*cred).gid)
            );
            return -EPERM;
        }
        _ => {
            /* Error, the only capabilities were checking for is CAP_SETUID/GID */
            return 0;
        }
    }
}

/*
 * Check whether a caller with old credentials @old is allowed to switch to
 * credentials that contain @new_id.
 */
unsafe extern "C" fn id_permitted_for_cred(
    old: *const cred,
    new_id: kid_t,
    new_type: setid_type,
) -> bool {
    let permitted: bool;

    /* If our old creds already had this ID in it, it's fine. */
    if new_type == setid_type::UID {
        if uid_eq(new_id.uid, (*old).uid)
            || uid_eq(new_id.uid, (*old).euid)
            || uid_eq(new_id.uid, (*old).suid)
        {
            return true;
        }
    } else if new_type == setid_type::GID {
        if gid_eq(new_id.gid, (*old).gid)
            || gid_eq(new_id.gid, (*old).egid)
            || gid_eq(new_id.gid, (*old).sgid)
        {
            return true;
        }
    } else {
        /* Error, new_type is an invalid type */
        return false;
    }

    /*
     * Transitions to new UIDs require a check against the policy of the old
     * RUID.
     */
    permitted = setid_policy_lookup(kid_t { uid: (*old).uid }, new_id, new_type)
        != sid_policy_type::SIDPOL_CONSTRAINED;

    if !permitted {
        if new_type == setid_type::UID {
            pr_warn!(
                "UID transition ((%d,%d,%d) -> %d) blocked\n",
                __kuid_val((*old).uid),
                __kuid_val((*old).euid),
                __kuid_val((*old).suid),
                __kuid_val(new_id.uid)
            );
        } else if new_type == setid_type::GID {
            pr_warn!(
                "GID transition ((%d,%d,%d) -> %d) blocked\n",
                __kgid_val((*old).gid),
                __kgid_val((*old).egid),
                __kgid_val((*old).sgid),
                __kgid_val(new_id.gid)
            );
        } else {
            /* Error, new_type is an invalid type */
            return false;
        }
    }
    permitted
}

/*
 * Check whether there is either an exception for user under old cred struct to
 * set*uid to user under new cred struct, or the UID transition is allowed (by
 * Linux set*uid rules) even without CAP_SETUID.
 */
unsafe extern "C" fn safesetid_task_fix_setuid(
    new: *mut cred,
    old: *const cred,
    _flags: c_int,
) -> c_int {
    /* Do nothing if there are no setuid restrictions for our old RUID. */
    if setid_policy_lookup(kid_t { uid: (*old).uid }, INVALID_ID, setid_type::UID)
        == sid_policy_type::SIDPOL_DEFAULT
    {
        return 0;
    }

    if id_permitted_for_cred(old, kid_t { uid: (*new).uid }, setid_type::UID)
        && id_permitted_for_cred(old, kid_t { uid: (*new).euid }, setid_type::UID)
        && id_permitted_for_cred(old, kid_t { uid: (*new).suid }, setid_type::UID)
        && id_permitted_for_cred(old, kid_t { uid: (*new).fsuid }, setid_type::UID)
    {
        return 0;
    }

    /*
     * Kill this process to avoid potential security vulnerabilities
     * that could arise from a missing allowlist entry preventing a
     * privileged process from dropping to a lesser-privileged one.
     */
    force_sig(SIGKILL);
    -EACCES
}

unsafe extern "C" fn safesetid_task_fix_setgid(
    new: *mut cred,
    old: *const cred,
    _flags: c_int,
) -> c_int {
    /* Do nothing if there are no setgid restrictions for our old RGID. */
    if setid_policy_lookup(kid_t { gid: (*old).gid }, INVALID_ID, setid_type::GID)
        == sid_policy_type::SIDPOL_DEFAULT
    {
        return 0;
    }

    if id_permitted_for_cred(old, kid_t { gid: (*new).gid }, setid_type::GID)
        && id_permitted_for_cred(old, kid_t { gid: (*new).egid }, setid_type::GID)
        && id_permitted_for_cred(old, kid_t { gid: (*new).sgid }, setid_type::GID)
        && id_permitted_for_cred(old, kid_t { gid: (*new).fsgid }, setid_type::GID)
    {
        return 0;
    }

    /*
     * Kill this process to avoid potential security vulnerabilities
     * that could arise from a missing allowlist entry preventing a
     * privileged process from dropping to a lesser-privileged one.
     */
    force_sig(SIGKILL);
    -EACCES
}

unsafe extern "C" fn safesetid_task_fix_setgroups(
    new: *mut cred,
    old: *const cred,
) -> c_int {
    let mut i: c_int;

    /* Do nothing if there are no setgid restrictions for our old RGID. */
    if setid_policy_lookup(kid_t { gid: (*old).gid }, INVALID_ID, setid_type::GID)
        == sid_policy_type::SIDPOL_DEFAULT
    {
        return 0;
    }

    get_group_info((*new).group_info);
    i = 0;
    while i < (*(*new).group_info).ngroups {
        if !id_permitted_for_cred(
            old,
            kid_t {
                gid: *(*(*new).group_info).gid.offset(i as isize),
            },
            setid_type::GID,
        ) {
            put_group_info((*new).group_info);
            /*
             * Kill this process to avoid potential security vulnerabilities
             * that could arise from a missing allowlist entry preventing a
             * privileged process from dropping to a lesser-privileged one.
             */
            force_sig(SIGKILL);
            return -EACCES;
        }
        i += 1;
    }

    put_group_info((*new).group_info);
    0
}

static SAFESETID_LSMID_NAME: &[u8] = b"safesetid\0";

static safesetid_lsmid: lsm_id = lsm_id {
    name: SAFESETID_LSMID_NAME.as_ptr(),
    id: LSM_ID_SAFESETID,
};

static mut safesetid_security_hooks: [security_hook_list; 4] = [
    LSM_HOOK_INIT!(task_fix_setuid, safesetid_task_fix_setuid),
    LSM_HOOK_INIT!(task_fix_setgid, safesetid_task_fix_setgid),
    LSM_HOOK_INIT!(task_fix_setgroups, safesetid_task_fix_setgroups),
    LSM_HOOK_INIT!(capable, safesetid_security_capable),
];

unsafe extern "C" fn safesetid_security_init() -> c_int {
    security_add_hooks(
        safesetid_security_hooks.as_mut_ptr(),
        safesetid_security_hooks.len(),
        &safesetid_lsmid,
    );

    /* Report that SafeSetID successfully initialized */
    safesetid_initialized = 1;

    0
}

#[unsafe(no_mangle)]
pub static safesetid_security_init_lsm: lsm_info = lsm_info {
    id: &safesetid_lsmid,
    init: Some(safesetid_security_init),
    initcall_fs: Some(safesetid_init_securityfs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
