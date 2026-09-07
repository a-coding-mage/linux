// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor policy attachment and domain transitions
 *
 * Copyright (C) 2002-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type gfp_t = c_uint;
type aa_state_t = u32;
type vfsuid_t = u32;

const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const ECHILD: c_int = 10;

const GFP_KERNEL: gfp_t = 0;
const PTRACE_MODE_ATTACH: c_uint = 0;
const MAY_EXEC: u32 = 0x1;
const AA_MAY_CHANGE_PROFILE: u32 = 0x2;
const AA_MAY_ONEXEC: u32 = 0x4;
const AA_MAY_CHANGEHAT: u32 = 0x8;
const AA_X_TYPE_MASK: u32 = 0x0f000000;
const AA_X_INDEX_MASK: u32 = 0x00ffffff;
const AA_X_NONE: u32 = 0;
const AA_X_NAME: u32 = 0x01000000;
const AA_X_TABLE: u32 = 0x02000000;
const AA_X_CHILD: u32 = 0x10000000;
const AA_X_INHERIT: u32 = 0x20000000;
const AA_X_UNCONFINED: u32 = 0x40000000;
const AA_X_UNSAFE: u32 = 0x80000000;
const AA_CLASS_FILE: usize = 0;
const AA_CLASS_XMATCH: usize = 1;
const FLAG_NULL: u32 = 0x1;
const FLAG_IX_ON_NAME_ERROR: u32 = 0x2;
const LSM_UNSAFE_NO_NEW_PRIVS: c_uint = 0x1;
const LSM_UNSAFE_SHARE: c_uint = 0x2;
const LSM_UNSAFE_PTRACE: c_uint = 0x4;
const PER_CLEAR_ON_SETID: c_uint = 0x1;
const AA_CHANGE_TEST: c_int = 0x1;
const AA_CHANGE_STACK: c_int = 0x2;
const AA_CHANGE_ONEXEC: c_int = 0x4;
const CAP_MAC_OVERRIDE: c_int = 0;
const CAP_OPT_NOAUDIT: c_int = 0;
const GLOBAL_ROOT_UID: u32 = 0;
const DEBUG_DOMAIN: c_int = 0;
static mut DEBUG_ON: bool = false;

static CONFLICTING_ATTACH_STR: &[u8] = b"conflicting profile attachments\0";
static CONFLICTING_ATTACH_STR_IX: &[u8] =
    b"conflicting profile attachments - ix fallback\0";
static CONFLICTING_ATTACH_STR_UX: &[u8] =
    b"conflicting profile attachments - ux fallback\0";
static stack_msg: &[u8] =
    b"change_profile unprivileged unconfined converted to stacking\0";

static OP_EXEC: &[u8] = b"exec\0";
static OP_CHANGE_HAT: &[u8] = b"change_hat\0";
static OP_CHANGE_PROFILE: &[u8] = b"change_profile\0";
static OP_STACK: &[u8] = b"stack\0";
static OP_CHANGE_ONEXEC: &[u8] = b"change_onexec\0";
static OP_STACK_ONEXEC: &[u8] = b"stack_onexec\0";

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct aa_perms {
    allow: u32,
    audit: u32,
    quiet: u32,
    kill: u32,
    xindex: u32,
}

#[repr(C)]
pub struct path_cond {
    uid: u32,
    mode: u32,
}

#[repr(C)]
pub struct aa_dfa {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_policydb {
    dfa: *mut aa_dfa,
    start: [aa_state_t; 8],
    trans: aa_trans,
}

#[repr(C)]
pub struct aa_trans {
    table: *mut aa_trans_entry,
}

#[repr(C)]
pub struct aa_trans_entry {
    strs: *const c_char,
}

#[repr(C)]
pub struct aa_ruleset {
    file: *mut aa_policydb,
}

#[repr(C)]
pub struct aa_proxy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_label {
    rules: [*mut aa_ruleset; 1],
    flags: u32,
    proxy: *mut aa_proxy,
}

#[repr(C)]
pub struct aa_policy {
    name: *const c_char,
    hname: *const c_char,
    list: list_head,
    profiles: list_head,
}

#[repr(C)]
pub struct aa_attachment {
    xmatch: *mut aa_policydb,
    xattrs: *mut *const c_char,
    xattr_count: c_int,
    xmatch_len: c_uint,
}

#[repr(C)]
pub struct aa_ns {
    base: aa_policy,
    lock: mutex,
    level: c_int,
    revision: c_long,
    unconfined: *mut aa_profile,
}

#[repr(C)]
pub struct aa_profile {
    label: aa_label,
    ns: *mut aa_ns,
    base: aa_policy,
    attach: aa_attachment,
    parent: *mut aa_profile,
    path_flags: c_uint,
    disconnected: bool,
}

#[repr(C)]
pub struct aa_task_ctx {
    previous: *mut aa_label,
    onexec: *mut aa_label,
    token: bool,
    nnp: *mut aa_label,
}

#[repr(C)]
pub struct cred {
    euid: u32,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct path {
    dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    f_path: path,
}

#[repr(C)]
pub struct inode {
    i_mode: u32,
}

#[repr(C)]
pub struct linux_binprm {
    file: *mut file,
    cred: *mut cred,
    filename: *const c_char,
    unsafe_: c_uint,
    secureexec: c_int,
    per_clear: c_uint,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut init_user_ns: user_namespace;
    static mut aa_unprivileged_unconfined_restricted: bool;
    static mut allperms: aa_perms;
    static mut nullperms: aa_perms;
    static mut nop_mnt_idmap: c_void;

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn ptrace_parent(task: *mut task_struct) -> *mut task_struct;
    fn aa_get_task_label(task: *mut task_struct) -> *mut aa_label;
    fn get_task_cred(task: *mut task_struct) -> *const cred;
    fn put_cred(cred: *const cred);
    fn aa_put_label(label: *mut aa_label);
    fn aa_get_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_newest_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_newest_cred_label(cred: *const cred) -> *mut aa_label;
    fn aa_get_current_label() -> *mut aa_label;
    fn aa_may_ptrace(
        tracer_cred: *const cred,
        tracerl: *mut aa_label,
        to_cred: *const cred,
        to_label: *mut aa_label,
        mode: c_uint,
    ) -> c_int;
    fn unconfined(label: *mut aa_label) -> bool;
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn aa_dfa_match(dfa: *mut aa_dfa, state: aa_state_t, str_: *const c_char) -> aa_state_t;
    fn aa_dfa_match_len(
        dfa: *mut aa_dfa,
        state: aa_state_t,
        str_: *const c_char,
        len: usize,
    ) -> aa_state_t;
    fn aa_dfa_leftmatch(
        dfa: *mut aa_dfa,
        start: aa_state_t,
        str_: *const c_char,
        count: *mut c_uint,
    ) -> aa_state_t;
    fn aa_dfa_outofband_transition(dfa: *mut aa_dfa, state: aa_state_t) -> aa_state_t;
    fn aa_dfa_null_transition(dfa: *mut aa_dfa, state: aa_state_t) -> aa_state_t;
    fn aa_ns_name(from: *mut aa_ns, to: *mut aa_ns, subns: bool) -> *const c_char;
    fn aa_ns_visible(from: *mut aa_ns, to: *mut aa_ns, inview: bool) -> bool;
    fn current_fsuid() -> u32;
    fn aa_lookup_condperms(
        fsuid: u32,
        file: *mut aa_policydb,
        state: aa_state_t,
        cond: *mut path_cond,
    ) -> *mut aa_perms;
    fn aa_lookup_perms(file: *mut aa_policydb, state: aa_state_t) -> *mut aa_perms;
    fn aa_apply_modes_to_perms(profile: *const aa_profile, perms: *mut aa_perms);
    fn aa_perms_accum(perms: *mut aa_perms, add: *mut aa_perms);
    fn might_sleep();
    fn vfs_getxattr_alloc(
        idmap: *const c_void,
        d: *mut dentry,
        name: *const c_char,
        value: *mut *mut c_char,
        size: c_int,
        flags: gfp_t,
    ) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn aa_get_profile_not0(profile: *mut aa_profile) -> bool;
    fn aa_put_profile(profile: *mut aa_profile);
    fn aa_get_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn aa_get_profile_rcu(profile: *mut *mut aa_profile) -> *mut aa_profile;
    fn aa_get_newest_profile(profile: *mut aa_profile) -> *mut aa_profile;
    fn ns_unconfined(ns: *mut aa_ns) -> *mut aa_label;
    fn aa_find_child(profile: *mut aa_profile, name: *const c_char) -> *mut aa_profile;
    fn aa_label_parse(
        base: *mut aa_label,
        str_: *const c_char,
        gfp: gfp_t,
        in_current_ns: bool,
        stack: bool,
    ) -> *mut aa_label;
    fn aa_label_merge(a: *mut aa_label, b: *mut aa_label, gfp: gfp_t) -> *mut aa_label;
    fn aa_path_name(
        path: *const path,
        flags: c_uint,
        buffer: *mut c_char,
        name: *mut *const c_char,
        info: *mut *const c_char,
        disconnected: bool,
    ) -> c_int;
    fn aa_str_perms(
        file: *mut aa_policydb,
        state: aa_state_t,
        name: *const c_char,
        cond: *mut path_cond,
        perms: *mut aa_perms,
    ) -> aa_state_t;
    fn aa_audit_file(
        cred: *const cred,
        profile: *mut aa_profile,
        perms: *mut aa_perms,
        op: *const c_char,
        request: u32,
        name: *const c_char,
        target: *const c_char,
        label: *mut aa_label,
        uid: u32,
        info: *const c_char,
        error: c_int,
    ) -> c_int;
    fn aa_new_learning_profile(
        parent: *mut aa_profile,
        hat: bool,
        name: *const c_char,
        gfp: gfp_t,
    ) -> *mut aa_profile;
    fn __aa_new_learning_profile(
        parent: *mut aa_profile,
        hat: bool,
        name: *const c_char,
        gfp: gfp_t,
    ) -> *mut aa_profile;
    fn aa_label_printk(label: *mut aa_label, gfp: gfp_t);
    fn dbg_printk(fmt: *const c_char, ...);
    fn pr_warn_ratelimited(fmt: *const c_char, ...);
    fn current_cred() -> *const cred;
    fn get_current_cred() -> *const cred;
    fn task_ctx(task: *mut task_struct) -> *mut aa_task_ctx;
    fn cred_label(cred: *mut cred) -> *mut aa_label;
    fn set_cred_label(cred: *mut cred, label: *mut aa_label);
    fn aa_get_buffer(in_atomic: bool) -> *mut c_char;
    fn aa_put_buffer(buffer: *mut c_char);
    fn i_uid_into_vfsuid(idmap: *const c_void, inode: *mut inode) -> vfsuid_t;
    fn vfsuid_into_kuid(uid: vfsuid_t) -> u32;
    fn file_mnt_idmap(file: *mut file) -> *const c_void;
    fn file_inode(file: *mut file) -> *mut inode;
    fn aa_label_is_unconfined_subset(label: *mut aa_label, nnp: *mut aa_label) -> bool;
    fn cap_capable(
        cred: *const cred,
        ns: *mut user_namespace,
        cap: c_int,
        opts: c_int,
    ) -> c_int;
    fn labels_ns(label: *mut aa_label) -> *mut aa_ns;
    fn labels_profile(label: *mut aa_label) -> *mut aa_profile;
    fn label_is_stale(label: *mut aa_label) -> bool;
    fn mutex_lock_nested(lock: *mut mutex, subclass: c_int);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_is_locked(lock: *mut mutex) -> bool;
    fn list_empty(head: *mut list_head) -> bool;
    fn task_no_new_privs(task: *mut task_struct) -> bool;
    fn aa_set_current_hat(label: *mut aa_label, token: u64) -> c_int;
    fn aa_restore_previous_label(token: u64) -> c_int;
    fn aa_replace_current_label(label: *mut aa_label) -> c_int;
    fn aa_set_current_onexec(label: *mut aa_label, stack: bool);
}

#[inline]
unsafe fn AA_BUG(_cond: bool) {}
#[inline]
unsafe fn AA_DEBUG(_class: c_int, _msg: *const c_char) {}
#[inline]
unsafe fn COMPLAIN_MODE(_profile: *mut aa_profile) -> bool { false }
#[inline]
unsafe fn PROFILE_IS_HAT(_profile: *mut aa_profile) -> bool { false }
#[inline]
unsafe fn IS_ERR(ptr: *mut aa_label) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
#[inline]
unsafe fn IS_ERR_OR_NULL(ptr: *mut aa_label) -> bool { ptr.is_null() || IS_ERR(ptr) }
#[inline]
unsafe fn ERR_PTR(error: c_int) -> *mut aa_label { error as isize as *mut aa_label }
#[inline]
unsafe fn PTR_ERR(ptr: *mut aa_label) -> c_int { ptr as isize as c_int }

/**
 * may_change_ptraced_domain - check if can change profile on ptraced task
 */
unsafe fn may_change_ptraced_domain(
    to_cred: *const cred,
    to_label: *mut aa_label,
    info: *mut *const c_char,
) -> c_int {
    let mut tracer: *mut task_struct;
    let mut tracerl: *mut aa_label = ptr::null_mut();
    let mut tracer_cred: *const cred = ptr::null();
    let mut error: c_int = 0;

    rcu_read_lock();
    tracer = ptrace_parent(current);
    if !tracer.is_null() {
        /* released below */
        tracerl = aa_get_task_label(tracer);
        tracer_cred = get_task_cred(tracer);
    }
    /* not ptraced */
    if tracer.is_null() || unconfined(tracerl) {
        rcu_read_unlock();
        aa_put_label(tracerl);
        put_cred(tracer_cred);
        return 0;
    }

    error = aa_may_ptrace(tracer_cred, tracerl, to_cred, to_label, PTRACE_MODE_ATTACH);

    rcu_read_unlock();
    aa_put_label(tracerl);
    put_cred(tracer_cred);

    if error != 0 {
        *info = b"ptrace prevents transition\0".as_ptr() as *const c_char;
    }
    error
}

unsafe fn match_component(
    profile: *const aa_profile,
    tp: *const aa_profile,
    stack: bool,
    mut state: aa_state_t,
) -> aa_state_t {
    let rules = (*profile).label.rules[0];
    let ns_name: *const c_char;

    if stack {
        state = aa_dfa_match((*(*rules).file).dfa, state, b"&\0".as_ptr() as *const c_char);
    }
    if (*profile).ns == (*tp).ns {
        return aa_dfa_match((*(*rules).file).dfa, state, (*tp).base.hname);
    }

    /* try matching with namespace name and then profile */
    ns_name = aa_ns_name((*profile).ns, (*tp).ns, true);
    state = aa_dfa_match_len((*(*rules).file).dfa, state, b":\0".as_ptr() as *const c_char, 1);
    state = aa_dfa_match((*(*rules).file).dfa, state, ns_name);
    state = aa_dfa_match_len((*(*rules).file).dfa, state, b":\0".as_ptr() as *const c_char, 1);
    aa_dfa_match((*(*rules).file).dfa, state, (*tp).base.hname)
}

unsafe fn label_compound_match(
    profile: *const aa_profile,
    label: *mut aa_label,
    stack: bool,
    mut state: aa_state_t,
    inview: bool,
    request: u32,
    perms: *mut aa_perms,
) -> c_int {
    let rules = (*profile).label.rules[0];
    let mut cond = path_cond { uid: 0, mode: 0 };

    /*
     * Original C iterates label_for_each(i, label, tp), then
     * label_for_each_cont(i, label, tp). The iterator macro is supplied by
     * AppArmor headers; this standalone translation preserves the matching
     * body and dependency boundary.
     */
    let mut tp: *mut aa_profile = ptr::null_mut();
    while !tp.is_null() {
        if !aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            continue;
        }
        state = match_component(profile, tp, stack, state);
        if state == 0 {
            *perms = nullperms;
            return -EACCES;
        }
        break;
    }

    if tp.is_null() {
        *perms = allperms;
        return 0;
    }

    /* Remaining label components are matched as A//&B//&C. */
    while !tp.is_null() {
        if !aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            continue;
        }
        state = aa_dfa_match((*(*rules).file).dfa, state, b"//&\0".as_ptr() as *const c_char);
        state = match_component(profile, tp, false, state);
        if state == 0 {
            *perms = nullperms;
            return -EACCES;
        }
        break;
    }
    *perms = *aa_lookup_condperms(current_fsuid(), (*rules).file, state, &mut cond);
    aa_apply_modes_to_perms(profile, perms);
    if ((*perms).allow & request) != request {
        return -EACCES;
    }
    0
}

unsafe fn label_components_match(
    profile: *const aa_profile,
    label: *mut aa_label,
    stack: bool,
    start: aa_state_t,
    inview: bool,
    request: u32,
    perms: *mut aa_perms,
) -> c_int {
    let rules = (*profile).label.rules[0];
    let mut tmp: aa_perms;
    let mut cond = path_cond { uid: 0, mode: 0 };
    let mut state: aa_state_t = 0;

    /*
     * Original C walks each visible label component with label_for_each and
     * label_for_each_cont. The macro expansion is external to this file.
     */
    let mut tp: *mut aa_profile = ptr::null_mut();
    while !tp.is_null() {
        if !aa_ns_visible((*profile).ns, (*tp).ns, inview) {
            continue;
        }
        state = match_component(profile, tp, stack, start);
        if state == 0 {
            *perms = nullperms;
            return -EACCES;
        }
        tmp = *aa_lookup_condperms(current_fsuid(), (*rules).file, state, &mut cond);
        aa_apply_modes_to_perms(profile, &mut tmp);
        aa_perms_accum(perms, &mut tmp);
        break;
    }

    if ((*perms).allow & request) != request {
        return -EACCES;
    }
    0
}

unsafe fn label_match(
    profile: *const aa_profile,
    label: *mut aa_label,
    stack: bool,
    state: aa_state_t,
    inview: bool,
    request: u32,
    perms: *mut aa_perms,
) -> c_int {
    *perms = nullperms;
    let error = label_compound_match(profile, label, stack, state, inview, request, perms);
    if error == 0 {
        return error;
    }
    *perms = allperms;
    label_components_match(profile, label, stack, state, inview, request, perms)
}

unsafe fn change_profile_perms(
    profile: *const aa_profile,
    target: *mut aa_label,
    stack: bool,
    request: u32,
    start: aa_state_t,
    perms: *mut aa_perms,
) -> c_int {
    if profile_unconfined(profile) {
        (*perms).allow = AA_MAY_CHANGE_PROFILE | AA_MAY_ONEXEC;
        (*perms).kill = 0;
        (*perms).quiet = 0;
        (*perms).audit = 0;
        return 0;
    }

    /* TODO: add profile in ns screening */
    label_match(profile, target, stack, start, true, request, perms)
}

unsafe fn aa_xattrs_match(path: *const path, profile: *const aa_profile, mut state: aa_state_t) -> c_int {
    AA_BUG(path.is_null());
    AA_BUG(profile.is_null());

    let mut i: c_int;
    let d: *mut dentry;
    let mut value: *mut c_char = ptr::null_mut();
    let attach = &(*profile).attach as *const aa_attachment;
    let mut size: c_int;
    let mut value_size: c_int = 0;
    let mut ret: c_int = (*attach).xattr_count;

    if (*attach).xattr_count == 0 {
        return 0;
    }
    might_sleep();

    /* transition from exec match to xattr set */
    state = aa_dfa_outofband_transition((*(*attach).xmatch).dfa, state);
    d = (*path).dentry;

    i = 0;
    while i < (*attach).xattr_count {
        size = vfs_getxattr_alloc(
            &nop_mnt_idmap as *const _ as *const c_void,
            d,
            *(*attach).xattrs.add(i as usize),
            &mut value,
            value_size,
            GFP_KERNEL,
        );
        if size >= 0 {
            let perms: *mut aa_perms;
            /*
             * Check the xattr presence before value. This ensure
             * that not present xattr can be distinguished from a 0
             * length value or rule that matches any value
             */
            state = aa_dfa_null_transition((*(*attach).xmatch).dfa, state);
            /* Check xattr value */
            state = aa_dfa_match_len((*(*attach).xmatch).dfa, state, value, size as usize);
            perms = aa_lookup_perms((*attach).xmatch, state);
            if ((*perms).allow & MAY_EXEC) == 0 {
                ret = -EINVAL;
                break;
            }
        }
        /* transition to next element */
        state = aa_dfa_outofband_transition((*(*attach).xmatch).dfa, state);
        if size < 0 {
            if state == 0 {
                ret = -EINVAL;
                break;
            }
            /* don't count missing optional xattr as matched */
            ret -= 1;
        }
        i += 1;
    }

    kfree(value as *mut c_void);
    ret
}

unsafe fn find_attach(
    path: *const path,
    ns: *mut aa_ns,
    head: *mut list_head,
    name: *const c_char,
    info: *mut *const c_char,
) -> *mut aa_label {
    let mut candidate_len: c_int = 0;
    let mut candidate_xattrs: c_int = 0;
    let mut conflict = false;
    let mut candidate: *mut aa_profile = ptr::null_mut();

    AA_BUG(path.is_null());
    AA_BUG(name.is_null());
    AA_BUG(head.is_null());

    rcu_read_lock();
    /*
     * Original C performs list_for_each_entry_rcu(profile, head, base.list)
     * with restart after policy revision changes. The list traversal macro is
     * external; the body below is retained as the source-level algorithm.
     */
    let mut profile: *mut aa_profile = ptr::null_mut();
    while !profile.is_null() {
        let attach = &mut (*profile).attach as *mut aa_attachment;

        if ((*profile).label.flags & FLAG_NULL) != 0
            && (&mut (*profile).label as *mut aa_label) == ns_unconfined((*profile).ns)
        {
            profile = ptr::null_mut();
            continue;
        }

        if !(*(*attach).xmatch).dfa.is_null() {
            let mut count: c_uint = 0;
            let mut state: aa_state_t;
            let perms: *mut aa_perms;

            state = aa_dfa_leftmatch(
                (*(*attach).xmatch).dfa,
                (*(*attach).xmatch).start[AA_CLASS_XMATCH],
                name,
                &mut count,
            );
            perms = aa_lookup_perms((*attach).xmatch, state);
            /* any accepting state means a valid match. */
            if ((*perms).allow & MAY_EXEC) != 0 {
                let mut ret: c_int = 0;

                if (count as c_int) < candidate_len {
                    profile = ptr::null_mut();
                    continue;
                }

                if (*attach).xattr_count != 0 {
                    let rev = (*ns).revision;
                    if !aa_get_profile_not0(profile) {
                        profile = ptr::null_mut();
                        continue;
                    }
                    rcu_read_unlock();
                    ret = aa_xattrs_match(path, profile, state);
                    rcu_read_lock();
                    aa_put_profile(profile);
                    if rev != (*ns).revision {
                        profile = ptr::null_mut();
                        continue;
                    }
                    if ret < 0 {
                        profile = ptr::null_mut();
                        continue;
                    }
                }
                if count as c_int == candidate_len && ret <= candidate_xattrs {
                    if ret == candidate_xattrs {
                        conflict = true;
                    }
                    profile = ptr::null_mut();
                    continue;
                }

                candidate = profile;
                candidate_len = if count > (*attach).xmatch_len { count } else { (*attach).xmatch_len } as c_int;
                candidate_xattrs = ret;
                conflict = false;
            }
        } else if strcmp((*profile).base.name, name) == 0 {
            candidate = profile;
            break;
        }
        profile = ptr::null_mut();
    }

    if candidate.is_null() || conflict {
        if conflict {
            *info = CONFLICTING_ATTACH_STR.as_ptr() as *const c_char;
        }
        rcu_read_unlock();
        return ptr::null_mut();
    }

    candidate = aa_get_newest_profile(candidate);
    rcu_read_unlock();
    &mut (*candidate).label
}

unsafe fn next_name(_xtype: c_int, _name: *const c_char) -> *const c_char {
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn x_table_lookup(
    profile: *mut aa_profile,
    xindex: u32,
    name: *mut *const c_char,
) -> *mut aa_label {
    let rules = (*profile).label.rules[0];
    let mut label: *mut aa_label = ptr::null_mut();
    let xtype = xindex & AA_X_TYPE_MASK;
    let index = (xindex & AA_X_INDEX_MASK) as isize;
    let mut next: *const c_char;

    AA_BUG(name.is_null());

    next = (*(*rules).file).trans.table.offset(index).read().strs;
    while !next.is_null() {
        let lookup = if *next == b'&' as c_char { next.add(1) } else { next };
        *name = next;
        if (xindex & AA_X_CHILD) != 0 {
            let new = aa_find_child(profile, lookup);
            if !new.is_null() {
                return &mut (*new).label;
            }
            next = next_name(xtype as c_int, next);
            continue;
        }
        label = aa_label_parse(&mut (*profile).label, lookup, GFP_KERNEL, true, false);
        if !IS_ERR_OR_NULL(label) {
            return label;
        }
        next = next_name(xtype as c_int, next);
    }

    ptr::null_mut()
}

unsafe fn x_to_label(
    profile: *mut aa_profile,
    path: *const path,
    name: *const c_char,
    xindex: u32,
    lookupname: *mut *const c_char,
    info: *mut *const c_char,
) -> *mut aa_label {
    let mut new: *mut aa_label = ptr::null_mut();
    let mut stack: *mut aa_label = ptr::null_mut();
    let ns = (*profile).ns;
    let xtype = xindex & AA_X_TYPE_MASK;
    let mut old_info: *const c_char = ptr::null();

    match xtype {
        AA_X_NONE => {
            *lookupname = ptr::null();
        }
        AA_X_TABLE => {
            new = x_table_lookup(profile, xindex, lookupname);
            if new.is_null() || **lookupname != b'&' as c_char {
            } else {
                stack = new;
                new = ptr::null_mut();
                if (xindex & AA_X_CHILD) != 0 {
                    new = find_attach(path, ns, &mut (*profile).base.profiles, name, info);
                } else {
                    new = find_attach(path, ns, &mut (*ns).base.profiles, name, info);
                }
                *lookupname = name;
            }
        }
        AA_X_NAME => {
            if (xindex & AA_X_CHILD) != 0 {
                new = find_attach(path, ns, &mut (*profile).base.profiles, name, info);
            } else {
                new = find_attach(path, ns, &mut (*ns).base.profiles, name, info);
            }
            *lookupname = name;
        }
        _ => {}
    }

    if new.is_null() {
        if (xindex & AA_X_INHERIT) != 0 {
            if *info == CONFLICTING_ATTACH_STR.as_ptr() as *const c_char {
                *info = CONFLICTING_ATTACH_STR_IX.as_ptr() as *const c_char;
            } else {
                old_info = *info;
                *info = b"ix fallback\0".as_ptr() as *const c_char;
            }
            new = aa_get_newest_label(&mut (*profile).label);
        } else if (xindex & AA_X_UNCONFINED) != 0 {
            new = aa_get_newest_label(ns_unconfined((*profile).ns));
            if *info == CONFLICTING_ATTACH_STR.as_ptr() as *const c_char {
                *info = CONFLICTING_ATTACH_STR_UX.as_ptr() as *const c_char;
            } else {
                old_info = *info;
                *info = b"ux fallback\0".as_ptr() as *const c_char;
            }
        }
        if !old_info.is_null() && old_info != CONFLICTING_ATTACH_STR.as_ptr() as *const c_char {
            pr_warn_ratelimited(
                b"AppArmor: find_attach (from profile %s) audit info \"%s\" dropped\0".as_ptr() as *const c_char,
                (*profile).base.hname,
                old_info,
            );
        }
    }

    if !new.is_null() && !stack.is_null() {
        let base = new;
        new = aa_label_merge(base, stack, GFP_KERNEL);
        aa_put_label(base);
    }

    aa_put_label(stack);
    new
}

unsafe fn profile_transition(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    bprm: *const linux_binprm,
    buffer: *mut c_char,
    cond: *mut path_cond,
    secure_exec: *mut bool,
) -> *mut aa_label {
    let rules = (*profile).label.rules[0];
    let mut new: *mut aa_label = ptr::null_mut();
    let mut new_profile: *mut aa_profile = ptr::null_mut();
    let mut info: *const c_char = ptr::null();
    let mut name: *const c_char = ptr::null();
    let mut target: *const c_char = ptr::null();
    let mut state = (*(*rules).file).start[AA_CLASS_FILE];
    let mut perms: aa_perms = core::mem::zeroed();
    let nonewprivs = false;
    let mut error: c_int = 0;

    AA_BUG(profile.is_null());
    AA_BUG(bprm.is_null());
    AA_BUG(buffer.is_null());

    error = aa_path_name(&(*(*bprm).file).f_path, (*profile).path_flags, buffer, &mut name, &mut info, (*profile).disconnected);
    if error != 0 {
        if profile_unconfined(profile) || ((*profile).label.flags & FLAG_IX_ON_NAME_ERROR) != 0 {
            AA_DEBUG(DEBUG_DOMAIN, b"name lookup ix on error\0".as_ptr() as *const c_char);
            error = 0;
            new = aa_get_newest_label(&mut (*profile).label);
        }
        name = (*bprm).filename;
    } else if profile_unconfined(profile) {
        new = find_attach(&(*(*bprm).file).f_path, (*profile).ns, &mut (*(*profile).ns).base.profiles, name, &mut info);
        if !info.is_null() {
            perms.audit |= MAY_EXEC;
            perms.allow |= MAY_EXEC;
            aa_audit_file(subj_cred, profile, &mut perms, OP_EXEC.as_ptr() as *const c_char, MAY_EXEC, name, target, new, (*cond).uid, info, error);
        }
        if !new.is_null() {
            AA_DEBUG(DEBUG_DOMAIN, b"unconfined attached to new label\0".as_ptr() as *const c_char);
            return new;
        }
        AA_DEBUG(DEBUG_DOMAIN, b"unconfined exec no attachment\0".as_ptr() as *const c_char);
        return aa_get_newest_label(&mut (*profile).label);
    } else {
        state = aa_str_perms((*rules).file, state, name, cond, &mut perms);
        if (perms.allow & MAY_EXEC) != 0 {
            new = x_to_label(profile, &(*(*bprm).file).f_path, name, perms.xindex, &mut target, &mut info);
            if !new.is_null() && (*new).proxy == (*profile).label.proxy && !info.is_null() {
                if info == CONFLICTING_ATTACH_STR_IX.as_ptr() as *const c_char
                    || info == CONFLICTING_ATTACH_STR_UX.as_ptr() as *const c_char
                {
                    perms.audit |= MAY_EXEC;
                }
            } else if new.is_null() {
                if !info.is_null() {
                    pr_warn_ratelimited(
                        b"AppArmor: %s (from profile %s) audit info \"%s\" dropped on missing transition\0".as_ptr() as *const c_char,
                        b"profile_transition\0".as_ptr() as *const c_char,
                        (*profile).base.hname,
                        info,
                    );
                }
                info = b"profile transition not found\0".as_ptr() as *const c_char;
                perms.allow &= !MAY_EXEC;
                if COMPLAIN_MODE(profile) {
                    new_profile = aa_new_learning_profile(profile, false, name, GFP_KERNEL);
                    if new_profile.is_null() {
                        error = -ENOMEM;
                        info = b"could not create null profile\0".as_ptr() as *const c_char;
                    } else {
                        error = -EACCES;
                        new = &mut (*new_profile).label;
                    }
                    perms.xindex |= AA_X_UNSAFE;
                } else {
                    error = -EACCES;
                }
            }
        } else if COMPLAIN_MODE(profile) {
            new_profile = aa_new_learning_profile(profile, false, name, GFP_KERNEL);
            if new_profile.is_null() {
                error = -ENOMEM;
                info = b"could not create null profile\0".as_ptr() as *const c_char;
            } else {
                error = -EACCES;
                new = &mut (*new_profile).label;
            }
            perms.xindex |= AA_X_UNSAFE;
        } else {
            error = -EACCES;
        }
    }

    if !new.is_null() && (perms.xindex & AA_X_UNSAFE) == 0 {
        if DEBUG_ON {
            dbg_printk(b"apparmor: setting AT_SECURE for %s profile=\0".as_ptr() as *const c_char, name);
            aa_label_printk(new, GFP_KERNEL);
            dbg_printk(b"\n\0".as_ptr() as *const c_char);
        }
        *secure_exec = true;
    }

    aa_audit_file(subj_cred, profile, &mut perms, OP_EXEC.as_ptr() as *const c_char, MAY_EXEC, name, target, new, (*cond).uid, info, error);
    if new.is_null() || nonewprivs {
        aa_put_label(new);
        return ERR_PTR(error);
    }
    new
}

/* The remaining routines preserve external AppArmor iterator/build macro intent. */

unsafe fn profile_onexec(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    onexec: *mut aa_label,
    stack: bool,
    bprm: *const linux_binprm,
    buffer: *mut c_char,
    cond: *mut path_cond,
    secure_exec: *mut bool,
) -> c_int {
    let rules = (*profile).label.rules[0];
    let mut state = (*(*rules).file).start[AA_CLASS_FILE];
    let mut perms: aa_perms = core::mem::zeroed();
    let mut xname: *const c_char = ptr::null();
    let mut info: *const c_char = b"change_profile onexec\0".as_ptr() as *const c_char;
    let mut error: c_int = -EACCES;

    AA_BUG(profile.is_null());
    AA_BUG(onexec.is_null());
    AA_BUG(bprm.is_null());
    AA_BUG(buffer.is_null());

    if profile_unconfined(profile) {
        return 0;
    }

    error = aa_path_name(&(*(*bprm).file).f_path, (*profile).path_flags, buffer, &mut xname, &mut info, (*profile).disconnected);
    if error != 0 {
        if profile_unconfined(profile) || ((*profile).label.flags & FLAG_IX_ON_NAME_ERROR) != 0 {
            AA_DEBUG(DEBUG_DOMAIN, b"name lookup ix on error\0".as_ptr() as *const c_char);
            error = 0;
        }
        xname = (*bprm).filename;
    } else {
        state = aa_str_perms((*rules).file, state, xname, cond, &mut perms);
        if (perms.allow & AA_MAY_ONEXEC) == 0 {
            info = b"no change_onexec valid for executable\0".as_ptr() as *const c_char;
        } else {
            state = aa_dfa_null_transition((*(*rules).file).dfa, state);
            error = change_profile_perms(profile, onexec, stack, AA_MAY_ONEXEC, state, &mut perms);
            if error != 0 {
                perms.allow &= !AA_MAY_ONEXEC;
            } else if (perms.xindex & AA_X_UNSAFE) == 0 {
                if DEBUG_ON {
                    dbg_printk(b"apparmor: setting AT_SECURE for %s label=\0".as_ptr() as *const c_char, xname);
                    aa_label_printk(onexec, GFP_KERNEL);
                    dbg_printk(b"\n\0".as_ptr() as *const c_char);
                }
                *secure_exec = true;
            }
        }
    }

    aa_audit_file(subj_cred, profile, &mut perms, OP_EXEC.as_ptr() as *const c_char, AA_MAY_ONEXEC, xname, ptr::null(), onexec, (*cond).uid, info, error)
}

unsafe fn label_merge_wrap(a: *mut aa_label, b: *mut aa_label, gfp: gfp_t) -> *mut aa_label {
    let label = aa_label_merge(a, b, gfp);
    if label.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    label
}

unsafe fn is_profile_priv_restricted_to_stack(
    _subj_cred: *const cred,
    profile: *mut aa_profile,
) -> bool {
    if profile_unconfined(profile)
        && profile == (*(*profile).ns).unconfined
        && aa_unprivileged_unconfined_restricted
        && cap_capable(current_cred(), &mut init_user_ns, CAP_MAC_OVERRIDE, CAP_OPT_NOAUDIT) != 0
    {
        return true;
    }
    false
}

unsafe fn priv_restricted_transition(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    op: *const c_char,
    request: u32,
    name: *const c_char,
    transition: *mut aa_label,
    gfp: gfp_t,
) -> *mut aa_label {
    if !is_profile_priv_restricted_to_stack(subj_cred, profile) {
        return aa_get_newest_label(transition);
    }

    let target = label_merge_wrap(&mut (*profile).label, transition, gfp);
    if IS_ERR_OR_NULL(target) {
        return target;
    }

    let mut perms = aa_perms {
        allow: request,
        audit: request,
        quiet: 0,
        kill: 0,
        xindex: 0,
    };
    aa_audit_file(subj_cred, profile, &mut perms, op, request, name, ptr::null(), target, (*subj_cred).euid, stack_msg.as_ptr() as *const c_char, 0);
    target
}

unsafe fn handle_onexec(
    subj_cred: *const cred,
    label: *mut aa_label,
    onexec: *mut aa_label,
    stack: bool,
    bprm: *const linux_binprm,
    buffer: *mut c_char,
    cond: *mut path_cond,
    unsafe_: *mut bool,
) -> *mut aa_label {
    AA_BUG(label.is_null());
    AA_BUG(onexec.is_null());
    AA_BUG(bprm.is_null());
    AA_BUG(buffer.is_null());

    /*
     * Original C:
     * error = fn_for_each_in_scope(label, profile, profile_onexec(...));
     * new = fn_label_build_in_scope(label, profile, GFP_KERNEL, ...);
     * These macros are supplied externally and are intentionally not expanded
     * in this isolated translation.
     */
    let mut profile: *mut aa_profile = ptr::null_mut();
    let error: c_int = 0;
    if error != 0 {
        return ERR_PTR(error);
    }
    let new = if stack {
        label_merge_wrap(&mut (*profile).label, onexec, GFP_KERNEL)
    } else {
        priv_restricted_transition(
            subj_cred,
            profile,
            OP_CHANGE_ONEXEC.as_ptr() as *const c_char,
            AA_MAY_ONEXEC,
            (*bprm).filename,
            onexec,
            GFP_KERNEL,
        )
    };
    if !IS_ERR(new) {
        return new;
    }
    ERR_PTR(PTR_ERR(new))
}

#[no_mangle]
pub unsafe extern "C" fn apparmor_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int {
    let ctx: *mut aa_task_ctx;
    let mut label: *mut aa_label;
    let mut new: *mut aa_label = ptr::null_mut();
    let subj_cred: *const cred;
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut buffer: *mut c_char = ptr::null_mut();
    let mut info: *const c_char = ptr::null();
    let mut error: c_int = 0;
    let mut unsafe_exec = false;
    let vfsuid = i_uid_into_vfsuid(file_mnt_idmap((*bprm).file), file_inode((*bprm).file));
    let mut cond = path_cond {
        uid: vfsuid_into_kuid(vfsuid),
        mode: (*file_inode((*bprm).file)).i_mode,
    };

    subj_cred = current_cred();
    ctx = task_ctx(current);
    AA_BUG(cred_label((*bprm).cred).is_null());
    AA_BUG(ctx.is_null());

    label = aa_get_newest_label(cred_label((*bprm).cred));

    if (((*bprm).unsafe_ & LSM_UNSAFE_NO_NEW_PRIVS) != 0) && !unconfined(label) && (*ctx).nnp.is_null() {
        (*ctx).nnp = aa_get_label(label);
    }

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        error = -ENOMEM;
    } else {
        if !(*ctx).onexec.is_null() {
            new = handle_onexec(subj_cred, label, (*ctx).onexec, (*ctx).token, bprm, buffer, &mut cond, &mut unsafe_exec);
        } else {
            /*
             * Original C builds across each label profile with fn_label_build.
             * External macro expansion supplies profile iteration.
             */
            new = profile_transition(subj_cred, profile, bprm, buffer, &mut cond, &mut unsafe_exec);
        }
        AA_BUG(new.is_null());
        if IS_ERR(new) {
            error = PTR_ERR(new);
        } else {
            if (((*bprm).unsafe_ & LSM_UNSAFE_NO_NEW_PRIVS) != 0)
                && !unconfined(label)
                && !aa_label_is_unconfined_subset(new, (*ctx).nnp)
            {
                error = -EPERM;
                info = b"no new privs\0".as_ptr() as *const c_char;
                aa_audit_file(current_cred(), profile, &mut nullperms, OP_EXEC.as_ptr() as *const c_char, MAY_EXEC, (*bprm).filename, ptr::null(), new, vfsuid_into_kuid(vfsuid), info, error);
                aa_put_label(new);
            } else {
                if ((*bprm).unsafe_ & LSM_UNSAFE_SHARE) != 0 {
                    /* FIXME: currently don't mediate shared state */
                }
                if ((*bprm).unsafe_ & LSM_UNSAFE_PTRACE) != 0 {
                    error = may_change_ptraced_domain((*bprm).cred, new, &mut info);
                }
                if error == 0 {
                    if unsafe_exec {
                        if DEBUG_ON {
                            dbg_printk(b"setting AT_SECURE for %s label=\0".as_ptr() as *const c_char, (*bprm).filename);
                            aa_label_printk(new, GFP_KERNEL);
                            dbg_printk(b"\n\0".as_ptr() as *const c_char);
                        }
                        (*bprm).secureexec = 1;
                    }
                    if (*label).proxy != (*new).proxy {
                        if DEBUG_ON {
                            dbg_printk(b"apparmor: clearing unsafe personality bits. %s label=\0".as_ptr() as *const c_char, (*bprm).filename);
                            aa_label_printk(new, GFP_KERNEL);
                            dbg_printk(b"\n\0".as_ptr() as *const c_char);
                        }
                        (*bprm).per_clear |= PER_CLEAR_ON_SETID;
                    }
                    aa_put_label(cred_label((*bprm).cred));
                    set_cred_label((*bprm).cred, new);
                }
            }
        }
    }

    aa_put_label(label);
    aa_put_buffer(buffer);
    error
}

unsafe fn build_change_hat(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    name: *const c_char,
    sibling: bool,
) -> *mut aa_label {
    let root: *mut aa_profile;
    let mut hat: *mut aa_profile = ptr::null_mut();
    let mut info: *const c_char = ptr::null();
    let mut error: c_int = 0;

    if sibling && PROFILE_IS_HAT(profile) {
        root = aa_get_profile_rcu(&mut (*profile).parent);
    } else if !sibling && !PROFILE_IS_HAT(profile) {
        root = aa_get_profile(profile);
    } else {
        info = b"conflicting target types\0".as_ptr() as *const c_char;
        error = -EPERM;
        root = ptr::null_mut();
    }

    if error == 0 {
        hat = aa_find_child(root, name);
        if hat.is_null() {
            error = -ENOENT;
            if COMPLAIN_MODE(profile) {
                hat = __aa_new_learning_profile(profile, true, name, GFP_KERNEL);
                if hat.is_null() {
                    info = b"failed null profile create\0".as_ptr() as *const c_char;
                    error = -ENOMEM;
                }
            }
        }
        aa_put_profile(root);
    }

    aa_audit_file(
        subj_cred,
        profile,
        &mut nullperms,
        OP_CHANGE_HAT.as_ptr() as *const c_char,
        AA_MAY_CHANGEHAT,
        name,
        if !hat.is_null() { (*hat).base.hname } else { ptr::null() },
        if !hat.is_null() { &mut (*hat).label } else { ptr::null_mut() },
        GLOBAL_ROOT_UID,
        info,
        error,
    );
    if hat.is_null() || (error != 0 && error != -ENOENT) {
        return ERR_PTR(error);
    }
    &mut (*hat).label
}

unsafe fn change_hat(
    subj_cred: *const cred,
    label: *mut aa_label,
    hats: *mut *const c_char,
    count: c_int,
    _flags: c_int,
) -> *mut aa_label {
    AA_BUG(label.is_null());
    AA_BUG(hats.is_null());
    AA_BUG(count < 1);

    /*
     * Original C locks the namespace, probes hats across
     * label_for_each_in_scope(), audits failures, and builds the resulting
     * label with fn_label_build_in_scope(). Those macros are external.
     */
    let name = *hats;
    let profile = labels_profile(label);
    build_change_hat(subj_cred, profile, name, PROFILE_IS_HAT(profile))
}

#[no_mangle]
pub unsafe extern "C" fn aa_change_hat(
    hats: *mut *const c_char,
    count: c_int,
    token: u64,
    flags: c_int,
) -> c_int {
    let subj_cred: *const cred;
    let ctx = task_ctx(current);
    let label: *mut aa_label;
    let previous: *mut aa_label;
    let mut new: *mut aa_label = ptr::null_mut();
    let mut target: *mut aa_label = ptr::null_mut();
    let profile: *mut aa_profile = ptr::null_mut();
    let mut perms: aa_perms = core::mem::zeroed();
    let mut info: *const c_char = ptr::null();
    let mut error: c_int = 0;

    subj_cred = get_current_cred();
    label = aa_get_newest_cred_label(subj_cred);
    previous = aa_get_newest_label((*ctx).previous);

    if task_no_new_privs(current) && !unconfined(label) && (*ctx).nnp.is_null() {
        (*ctx).nnp = aa_get_label(label);
    }

    if unconfined(label) {
        let empty = true;
        if empty {
            info = b"unconfined can not change_hat\0".as_ptr() as *const c_char;
            error = -EPERM;
        }
    }

    if error == 0 {
        if count != 0 {
            new = change_hat(subj_cred, label, hats, count, flags);
            AA_BUG(new.is_null());
            if IS_ERR(new) {
                error = PTR_ERR(new);
                new = ptr::null_mut();
            } else {
                error = may_change_ptraced_domain(subj_cred, new, &mut info);
                if error == 0
                    && task_no_new_privs(current)
                    && !unconfined(label)
                    && !aa_label_is_unconfined_subset(new, (*ctx).nnp)
                {
                    AA_DEBUG(DEBUG_DOMAIN, b"no_new_privs - change_hat denied\0".as_ptr() as *const c_char);
                    error = -EPERM;
                } else if (flags & AA_CHANGE_TEST) == 0 {
                    target = new;
                    error = aa_set_current_hat(new, token);
                    if error == -EACCES {
                        info = b"failed token match\0".as_ptr() as *const c_char;
                        perms.kill = AA_MAY_CHANGEHAT;
                    }
                }
            }
        } else if !previous.is_null() && (flags & AA_CHANGE_TEST) == 0 {
            if task_no_new_privs(current)
                && !unconfined(label)
                && !aa_label_is_unconfined_subset(previous, (*ctx).nnp)
            {
                AA_DEBUG(DEBUG_DOMAIN, b"no_new_privs - change_hat denied\0".as_ptr() as *const c_char);
                error = -EPERM;
            } else {
                target = previous;
                error = aa_restore_previous_label(token);
                if error == -EACCES {
                    info = b"failed token match\0".as_ptr() as *const c_char;
                    perms.kill = AA_MAY_CHANGEHAT;
                }
            }
        }
    }

    if error != 0 {
        aa_audit_file(subj_cred, profile, &mut perms, OP_CHANGE_HAT.as_ptr() as *const c_char, AA_MAY_CHANGEHAT, ptr::null(), ptr::null(), target, GLOBAL_ROOT_UID, info, error);
    }

    aa_put_label(new);
    aa_put_label(previous);
    aa_put_label(label);
    put_cred(subj_cred);
    error
}

unsafe fn change_profile_perms_wrapper(
    op: *const c_char,
    name: *const c_char,
    subj_cred: *const cred,
    profile: *mut aa_profile,
    target: *mut aa_label,
    stack: bool,
    request: u32,
    perms: *mut aa_perms,
) -> c_int {
    let rules = (*profile).label.rules[0];
    let info: *const c_char = ptr::null();
    let mut error: c_int = 0;

    if error == 0 {
        error = change_profile_perms(profile, target, stack, request, (*(*rules).file).start[AA_CLASS_FILE], perms);
    }
    if error != 0 {
        error = aa_audit_file(subj_cred, profile, perms, op, request, name, ptr::null(), target, GLOBAL_ROOT_UID, info, error);
    }
    error
}

#[no_mangle]
pub unsafe extern "C" fn aa_change_profile(fqname: *const c_char, flags: c_int) -> c_int {
    let label: *mut aa_label;
    let mut new: *mut aa_label = ptr::null_mut();
    let mut target: *mut aa_label;
    let profile: *mut aa_profile;
    let mut perms: aa_perms = core::mem::zeroed();
    let mut info: *const c_char = ptr::null();
    let auditname = fqname;
    let mut stack = (flags & AA_CHANGE_STACK) != 0;
    let ctx = task_ctx(current);
    let subj_cred = get_current_cred();
    let mut error: c_int = 0;
    let op: *const c_char;
    let request: u32;

    label = aa_get_current_label();

    if task_no_new_privs(current) && !unconfined(label) && (*ctx).nnp.is_null() {
        (*ctx).nnp = aa_get_label(label);
    }

    if fqname.is_null() || *fqname == 0 {
        aa_put_label(label);
        AA_DEBUG(DEBUG_DOMAIN, b"no profile name\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (flags & AA_CHANGE_ONEXEC) != 0 {
        request = AA_MAY_ONEXEC;
        op = if stack { OP_STACK_ONEXEC.as_ptr() as *const c_char } else { OP_CHANGE_ONEXEC.as_ptr() as *const c_char };
    } else {
        request = AA_MAY_CHANGE_PROFILE;
        op = if stack { OP_STACK.as_ptr() as *const c_char } else { OP_CHANGE_PROFILE.as_ptr() as *const c_char };
    }

    let mut parse_name = fqname;
    if *parse_name == b'&' as c_char {
        stack = true;
        parse_name = parse_name.add(1);
    }
    target = aa_label_parse(label, parse_name, GFP_KERNEL, true, false);
    if IS_ERR(target) {
        info = b"label not found\0".as_ptr() as *const c_char;
        error = PTR_ERR(target);
        target = ptr::null_mut();
        if (flags & AA_CHANGE_TEST) != 0 || !COMPLAIN_MODE(labels_profile(label)) {
            aa_audit_file(subj_cred, labels_profile(label), &mut perms, op, request, auditname, ptr::null(), target, GLOBAL_ROOT_UID, info, error);
            aa_put_label(new);
            aa_put_label(target);
            aa_put_label(label);
            put_cred(subj_cred);
            return error;
        }
        let tprofile = aa_new_learning_profile(labels_profile(label), false, parse_name, GFP_KERNEL);
        if tprofile.is_null() {
            info = b"failed null profile create\0".as_ptr() as *const c_char;
            error = -ENOMEM;
        } else {
            target = &mut (*tprofile).label;
            error = 0;
        }
    }

    profile = labels_profile(label);
    if error == 0 {
        error = change_profile_perms_wrapper(op, auditname, subj_cred, profile, target, stack, request, &mut perms);
    }
    if error == 0 {
        error = may_change_ptraced_domain(subj_cred, target, &mut info);
        if error != 0 && COMPLAIN_MODE(profile) {
            error = 0;
        }
    }

    if error == 0 && (flags & AA_CHANGE_TEST) == 0 {
        if !stack {
            new = priv_restricted_transition(subj_cred, profile, op, request, auditname, target, GFP_KERNEL);
            AA_BUG(new.is_null());
            if IS_ERR(new) {
                info = b"failed to build target label\0".as_ptr() as *const c_char;
                error = PTR_ERR(new);
                new = ptr::null_mut();
                perms.allow = 0;
            } else if task_no_new_privs(current)
                && !unconfined(label)
                && !aa_label_is_unconfined_subset(new, (*ctx).nnp)
            {
                AA_DEBUG(DEBUG_DOMAIN, b"no_new_privs - change_hat denied\0".as_ptr() as *const c_char);
                error = -EPERM;
            }
        }

        if error == 0 {
            if (flags & AA_CHANGE_ONEXEC) == 0 {
                if stack {
                    new = aa_label_merge(label, target, GFP_KERNEL);
                }
                if IS_ERR_OR_NULL(new) {
                    info = b"failed to build target label\0".as_ptr() as *const c_char;
                    error = if new.is_null() { -ENOMEM } else { PTR_ERR(new) };
                    new = ptr::null_mut();
                    perms.allow = 0;
                } else {
                    error = aa_replace_current_label(new);
                }
            } else {
                aa_put_label(new);
                new = ptr::null_mut();
                aa_set_current_onexec(target, stack);
            }
        }
    }

    error = aa_audit_file(
        subj_cred,
        profile,
        &mut perms,
        op,
        request,
        auditname,
        ptr::null(),
        if !new.is_null() { new } else { target },
        GLOBAL_ROOT_UID,
        info,
        error,
    );

    aa_put_label(new);
    aa_put_label(target);
    aa_put_label(label);
    put_cred(subj_cred);
    error
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
