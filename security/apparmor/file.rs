// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor mediation of files
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Linux/AppArmor kernel external dependencies - see included headers in C version
// #include <linux/tty.h>
// #include <linux/fdtable.h>
// #include <linux/file.h>
// #include <linux/fs.h>
// #include <linux/mount.h>
// #include "include/af_unix.h"
// #include "include/apparmor.h"
// #include "include/audit.h"
// #include "include/cred.h"
// #include "include/file.h"
// #include "include/match.h"
// #include "include/net.h"
// #include "include/path.h"
// #include "include/policy.h"
// #include "include/label.h"

use core::ffi::c_void;
use core::ptr::{self, null_mut};

// External type declarations from Linux kernel and AppArmor headers
pub const PERMS_CHRS_MASK: u32 = 0; // placeholder - from headers
pub const AA_MAY_GETATTR: u32 = 0; // placeholder
pub const AA_MAY_SETATTR: u32 = 0; // placeholder
pub const AA_MAY_CHMOD: u32 = 0; // placeholder
pub const AA_MAY_CHOWN: u32 = 0; // placeholder
pub const AA_MAY_LINK: u32 = 0; // placeholder
pub const AA_MAY_EXEC: u32 = 0; // placeholder
pub const AA_LINK_SUBSET: u32 = 0; // placeholder
pub const AA_X_UNSAFE: u32 = 0; // placeholder
pub const AA_AUDIT_FILE_MASK: u32 = 0; // placeholder
pub const AA_CLASS_FILE: u32 = 0; // placeholder
pub const MAY_READ: u32 = 0; // placeholder
pub const MAY_WRITE: u32 = 0; // placeholder
pub const MAY_EXEC: u32 = 0; // placeholder
pub const PATH_SOCK_COND: i32 = 0; // placeholder
pub const PATH_DELEGATE_DELETED: i32 = 0; // placeholder
pub const PATH_IS_DIR: i32 = 0; // placeholder
pub const AUDIT_APPARMOR_AUTO: i32 = 0; // placeholder
pub const AUDIT_APPARMOR_AUDIT: i32 = 0; // placeholder
pub const AUDIT_APPARMOR_KILL: i32 = 0; // placeholder
pub const AUDIT_ALL: i32 = 0; // placeholder
pub const AUDIT_QUIET_ALLOWED: i32 = 0; // placeholder
pub const AUDIT_NOQUIET: i32 = 0; // placeholder
pub const O_RDWR: i32 = 0; // placeholder
pub const IN_ATOMIC: bool = false; // placeholder

pub const OP_LINK: &str = "link"; // placeholder
pub const OP_INHERIT: &str = "inherit"; // placeholder

#[repr(C)]
pub struct audit_buffer;

#[repr(C)]
pub struct common_audit_data;

#[repr(C)]
pub struct apparmor_audit_data {
    pub subj_cred: *mut cred,
    pub request: u32,
    pub denied: u32,
    pub tags: u32,
    pub name: *const i8,
    pub fs: FileAuditData,
    pub peer: *mut aa_label,
    pub info: *const i8,
    pub error: i32,
    pub subj_label: *mut aa_label,
    pub common: common_audit_data_union,
}

#[repr(C)]
pub struct FileAuditData {
    pub target: *const i8,
    pub ouid: kuid_t,
}

#[repr(C)]
pub union common_audit_data_union {
    pub tsk: *mut task_struct,
}

pub type u32 = core::primitive::u32;
pub type i32 = core::primitive::i32;
pub type i8 = core::primitive::i8;
pub type bool = core::primitive::bool;

#[repr(C)]
pub struct kuid_t {
    pub val: u32,
}

#[repr(C)]
pub struct cred;

#[repr(C)]
pub struct aa_profile;

#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub deny: u32,
    pub audit: u32,
    pub quiet: u32,
    pub kill: u32,
    pub tag: u32,
    pub xindex: u32,
}

#[repr(C)]
pub struct aa_label;

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct path_cond {
    pub uid: kuid_t,
    pub mode: u32,
}

#[repr(C)]
pub struct inode;

#[repr(C)]
pub struct vfsmount;

#[repr(C)]
pub struct dentry;

pub type vfsuid_t = u32;

pub type aa_state_t = u32;

#[repr(C)]
pub struct aa_policydb {
    pub dfa: *mut core::ffi::c_void,
    pub perms: *mut aa_perms,
}

#[repr(C)]
pub struct aa_ruleset {
    pub file: *mut aa_policydb,
}

#[repr(C)]
pub struct aa_file_ctx {
    pub lock: spinlock_t,
    pub label: *mut aa_label,
    pub allow: u32,
}

#[repr(C)]
pub struct spinlock_t;

#[repr(C)]
pub struct file;

#[repr(C)]
pub struct socket;

#[repr(C)]
pub struct aa_sk_ctx {
    pub peer: *mut core::ffi::c_void,
    pub peer_lastupdate: *mut core::ffi::c_void,
    pub label: *mut aa_label,
}

#[repr(C)]
pub struct tty_struct;

#[repr(C)]
pub struct tty_file_private;

#[repr(C)]
pub struct files_struct;

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct user_namespace;

pub static mut aa_null: path = path { mnt: ptr::null_mut(), dentry: ptr::null_mut() };
pub static mut default_perms: aa_perms = aa_perms {
    allow: 0,
    deny: 0,
    audit: 0,
    quiet: 0,
    kill: 0,
    tag: 0,
    xindex: 0,
};

pub static mut init_user_ns: user_namespace = unsafe { core::mem::zeroed() };

// External function declarations from kernel
extern "C" {
    pub fn aa_perm_mask_to_str(str: *mut i8, len: usize, perm_chrs: *const i8, mask: u32);
    pub fn audit_log_format(ab: *mut audit_buffer, fmt: *const i8, ...);
    pub fn audit_log_untrustedstring(ab: *mut audit_buffer, str: *const i8);
    pub fn aa_label_xaudit(
        ab: *mut audit_buffer,
        ns: *mut core::ffi::c_void,
        label: *mut aa_label,
        flags: u32,
        gfp: u32,
    );
    pub fn aa_audit(
        audit_type: i32,
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        cb: unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void),
    ) -> i32;
    pub fn aa_path_name(
        path: *const path,
        flags: i32,
        buffer: *mut i8,
        name: *mut *const i8,
        info: *mut *const i8,
        disconnected: i32,
    ) -> i32;
    pub fn aa_dfa_match(dfa: *mut core::ffi::c_void, start: aa_state_t, name: *const i8) -> aa_state_t;
    pub fn aa_dfa_null_transition(dfa: *mut core::ffi::c_void, state: aa_state_t) -> aa_state_t;
    pub fn from_kuid(user_ns: *mut user_namespace, uid: kuid_t) -> u32;
    pub fn current_fsuid() -> kuid_t;
    pub fn uid_eq(uid1: kuid_t, uid2: kuid_t) -> bool;
    pub fn aa_get_buffer(in_atomic: bool) -> *mut i8;
    pub fn aa_put_buffer(buffer: *mut i8);
    pub fn aa_label_merge(
        old: *mut aa_label,
        new: *mut aa_label,
        gfp: u32,
    ) -> *mut aa_label;
    pub fn aa_put_label(label: *mut aa_label);
    pub fn aa_label_is_subset(label1: *mut aa_label, label2: *mut aa_label) -> bool;
    pub fn aa_get_newest_label(label: *mut aa_label) -> *mut aa_label;
    pub fn aa_sock_file_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const i8,
        request: u32,
        file: *mut file,
    ) -> i32;
    pub fn aa_map_file_to_perms(file: *mut file) -> u32;
    pub fn aa_get_newest_cred_label(cred: *const cred) -> *mut aa_label;
    pub fn get_current_tty() -> *mut tty_struct;
    pub fn tty_kref_put(tty: *mut tty_struct);
    pub fn no_tty();
    pub fn iterate_fd(
        files: *mut files_struct,
        first: u32,
        cb: unsafe extern "C" fn(*const core::ffi::c_void, *mut file, u32) -> i32,
        arg: *const core::ffi::c_void,
    ) -> u32;
    pub fn replace_fd(fd: u32, file: *mut file, flags: u32);
    pub fn dentry_open(dentry: *const path, flags: i32, cred: *const cred) -> *mut file;
    pub fn fput(file: *mut file);
    pub fn i_uid_into_vfsuid(idmap: *mut core::ffi::c_void, inode: *mut inode) -> vfsuid_t;
    pub fn mnt_idmap(mnt: *mut vfsmount) -> *mut core::ffi::c_void;
    pub fn file_mnt_idmap(file: *mut file) -> *mut core::ffi::c_void;
    pub fn file_inode(file: *mut file) -> *mut inode;
    pub fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    pub fn vfsuid_into_kuid(vfsuid: vfsuid_t) -> kuid_t;
    pub fn unconfined(label: *mut aa_label) -> bool;
    pub fn path_mediated_fs(dentry: *mut dentry) -> bool;
    pub fn file_ctx(file: *mut file) -> *mut aa_file_ctx;
    pub fn aa_sock(sk: *mut core::ffi::c_void) -> *mut aa_sk_ctx;
    pub fn lockdep_assert_in_rcu_read_lock();
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn rcu_dereference(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn rcu_dereference_protected(ptr: *mut core::ffi::c_void, cond: bool) -> *mut core::ffi::c_void;
    pub fn rcu_access_pointer(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn rcu_assign_pointer(ptr: *mut *mut core::ffi::c_void, new: *mut core::ffi::c_void);
    pub fn spin_lock(lock: *mut spinlock_t);
    pub fn spin_unlock(lock: *mut spinlock_t);
    pub fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    pub fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    pub fn labels_ns(label: *mut aa_label) -> *mut core::ffi::c_void;
    pub fn labels_profile(label: *mut aa_label) -> *mut aa_profile;
}

// Macros that need to be represented as functions or constants
pub fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data {
    sa as *mut apparmor_audit_data
}

pub unsafe fn profile_unconfined(profile: *mut aa_profile) -> bool {
    unconfined(&mut (*(profile as *mut aa_label)))
}

pub fn complain_mode(profile: *mut aa_profile) -> bool {
    // TODO: implement from profile structure
    false
}

pub fn audit_mode(profile: *mut aa_profile) -> i32 {
    // TODO: implement from profile structure
    0
}

pub unsafe fn aa_bug(cond: bool) {
    if cond {
        // TODO: trigger kernel bug/panic
    }
}

pub unsafe fn define_audit_data(data_type: i32, class: u32, op: *const i8) -> apparmor_audit_data {
    core::mem::zeroed()
}

pub unsafe fn aa_perm_mask_to_str_wrapper(s: *mut i8, len: usize, chrs: *const i8, mask: u32) {
    aa_perm_mask_to_str(s, len, chrs, mask);
}

pub fn s_isdir(mode: u32) -> bool {
    (mode & 0o170000) == 0o040000
}

pub fn s_issock(mode: u32) -> bool {
    (mode & 0o170000) == 0o140000
}

pub unsafe fn __aa_subj_label_is_cached(label: *mut aa_label, subj_label: *mut aa_label) -> bool {
    // TODO: implement from actual logic
    label == subj_label
}

pub unsafe fn fn_for_each_confined<F>(
    label: *mut aa_label,
    mut callback: F,
) -> i32
where
    F: FnMut(*mut aa_profile) -> i32,
{
    // TODO: implement iteration over confined profiles
    0
}

pub unsafe fn fn_for_each<F>(
    label: *mut aa_label,
    mut callback: F,
) -> i32
where
    F: FnMut(*mut aa_profile) -> i32,
{
    // TODO: implement iteration over profiles
    0
}

pub unsafe fn fn_for_each_not_in_set<F>(
    label: *mut aa_label,
    not_in_label: *mut aa_label,
    mut callback: F,
) -> i32
where
    F: FnMut(*mut aa_profile) -> i32,
{
    // TODO: implement iteration over profiles not in set
    0
}

pub fn last_error(e1: i32, e2: i32) -> i32 {
    if e2 != 0 { e2 } else { e1 }
}

pub extern "C" fn map_mask_to_chr_mask(mask: u32) -> u32 {
    let mut m = mask & PERMS_CHRS_MASK;

    if mask & AA_MAY_GETATTR != 0 {
        m |= MAY_READ;
    }
    if mask & (AA_MAY_SETATTR | AA_MAY_CHMOD | AA_MAY_CHOWN) != 0 {
        m |= MAY_WRITE;
    }

    m
}

/// file_audit_cb - call back for file specific audit fields
/// @ab: audit_buffer  (NOT NULL)
/// @va: audit struct to audit values of  (NOT NULL)
pub unsafe extern "C" fn file_audit_cb(ab: *mut audit_buffer, va: *mut core::ffi::c_void) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);
    let fsuid = if !(*ad).subj_cred.is_null() {
        // TODO: get fsuid from subj_cred
        current_fsuid()
    } else {
        current_fsuid()
    };
    let mut str: [i8; 10] = [0; 10];

    if (*ad).request & AA_AUDIT_FILE_MASK != 0 {
        aa_perm_mask_to_str(
            &mut str[0],
            10,
            std::ptr::null(),
            map_mask_to_chr_mask((*ad).request),
        );
        audit_log_format(ab, b" requested_mask=\"%s\"\0".as_ptr() as *const i8, &str[0]);
    }
    if (*ad).denied & AA_AUDIT_FILE_MASK != 0 {
        aa_perm_mask_to_str(
            &mut str[0],
            10,
            std::ptr::null(),
            map_mask_to_chr_mask((*ad).denied),
        );
        audit_log_format(ab, b" denied_mask=\"%s\"\0".as_ptr() as *const i8, &str[0]);
    }
    if (*ad).request & AA_AUDIT_FILE_MASK != 0 {
        audit_log_format(
            ab,
            b" fsuid=%d\0".as_ptr() as *const i8,
            from_kuid(&mut init_user_ns, fsuid),
        );
        audit_log_format(
            ab,
            b" ouid=%d\0".as_ptr() as *const i8,
            from_kuid(&mut init_user_ns, (*ad).fs.ouid),
        );
    }

    if !(*ad).peer.is_null() {
        audit_log_format(ab, b" target=\0".as_ptr() as *const i8);
        aa_label_xaudit(ab, labels_ns((*ad).subj_label), (*ad).peer, 0, 0);
    } else if !(*ad).fs.target.is_null() {
        audit_log_format(ab, b" target=\0".as_ptr() as *const i8);
        audit_log_untrustedstring(ab, (*ad).fs.target);
    }
}

/// aa_audit_file - handle the auditing of file operations
/// @subj_cred: cred of the subject
/// @profile: the profile being enforced  (NOT NULL)
/// @perms: the permissions computed for the request (NOT NULL)
/// @op: operation being mediated
/// @request: permissions requested
/// @name: name of object being mediated (MAYBE NULL)
/// @target: name of target (MAYBE NULL)
/// @tlabel: target label (MAY BE NULL)
/// @ouid: object uid
/// @info: extra information message (MAYBE NULL)
/// @error: 0 if operation allowed else failure error code
///
/// Returns: %0 or error on failure
pub unsafe extern "C" fn aa_audit_file(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    perms: *const aa_perms,
    op: *const i8,
    request: u32,
    name: *const i8,
    target: *const i8,
    tlabel: *mut aa_label,
    ouid: kuid_t,
    info: *const i8,
    error: i32,
) -> i32 {
    let mut quiet = (*perms).quiet;
    let complain = (*perms).complain;
    let mut audit_type = AUDIT_APPARMOR_AUTO;
    let mut ad: apparmor_audit_data = define_audit_data(0, AA_CLASS_FILE, op);

    ad.subj_cred = subj_cred as *mut cred;
    ad.request = request;
    ad.tags = (*perms).tag;
    ad.name = name;
    ad.fs.target = target;
    ad.peer = tlabel;
    ad.fs.ouid = ouid;
    ad.info = info;
    ad.error = error;
    ad.common.tsk = ptr::null_mut();

    if complain_mode(profile) {
        quiet |= !((*perms).allow | (*perms).deny);
    }
    if error == 0 {
        let mask = (*perms).audit;

        if audit_mode(profile) == AUDIT_ALL {
            // mask = 0xffff
        }

        ad.request &= mask;

        if ad.request == 0 {
            return 0;
        }
        audit_type = AUDIT_APPARMOR_AUDIT;
    } else {
        ad.request = ad.request & !(*perms).allow;
        aa_bug(ad.request == 0);

        if ad.request & (*perms).kill != 0 {
            audit_type = AUDIT_APPARMOR_KILL;
        }

        if audit_mode(profile) == AUDIT_QUIET_ALLOWED {
            quiet |= complain | (*perms).allow;
        }

        if (ad.request & quiet) != 0
            && audit_mode(profile) != AUDIT_NOQUIET
            && audit_mode(profile) != AUDIT_ALL
        {
            ad.request &= !quiet;
        }

        if ad.request == 0 {
            return ad.error;
        }
    }

    ad.denied = ad.request & !(*perms).allow;
    aa_audit(audit_type, profile, &mut ad, file_audit_cb)
}

static mut aa_file_perm_chrs: *const i8 = ptr::null();

pub extern "C" fn path_name(
    op: *const i8,
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    flags: i32,
    buffer: *mut i8,
    name: *mut *const i8,
    cond: *mut path_cond,
    request: u32,
) -> i32 {
    let mut info: *const i8 = ptr::null();
    let mut error;

    unsafe {
        if (*path).dentry == aa_null.dentry {
            return -libc::EACCES;
        }

        error = aa_path_name(
            path,
            flags,
            buffer,
            name,
            &mut info,
            0,
        );
        if error != 0 {
            fn_for_each_confined(label, |profile| {
                aa_audit_file(
                    subj_cred,
                    profile,
                    &default_perms,
                    op,
                    request,
                    *name,
                    ptr::null(),
                    ptr::null_mut(),
                    (*cond).uid,
                    info,
                    error,
                )
            });
            return error;
        }
    }

    0
}

/// aa_lookup_condperms - convert dfa compressed perms to internal perms
/// @subj_uid: uid to use for subject owner test
/// @rules: the aa_policydb to lookup perms for  (NOT NULL)
/// @state: state in dfa
/// @cond:  conditions to consider  (NOT NULL)
///
/// TODO: convert from dfa + state to permission entry
///
/// Returns: a pointer to a file permission set
pub extern "C" fn aa_lookup_condperms(
    subj_uid: kuid_t,
    rules: *mut aa_policydb,
    state: aa_state_t,
    cond: *mut path_cond,
) -> *mut aa_perms {
    unsafe {
        let index: u32; // TODO: ACCEPT_TABLE(rules->dfa)[state]

        if (*rules).perms.is_null() {
            return &mut default_perms;
        }

        // TODO: ACCEPT_TABLE2 check
        if false {
            if uid_eq(subj_uid, (*cond).uid) {
                return &mut *(*rules).perms.add(index as usize);
            }
            return &mut *(*rules).perms.add((index + 1) as usize);
        }

        &mut *(*rules).perms.add(index as usize)
    }
}

/// aa_str_perms - find permission that match @name
/// @file_rules: the aa_policydb to match against  (NOT NULL)
/// @start: state to start matching in
/// @name: string to match against dfa  (NOT NULL)
/// @cond: conditions to consider for permission set computation  (NOT NULL)
/// @perms: Returns - the permissions found when matching @name
///
/// Returns: the final state in @dfa when beginning @start and walking @name
pub extern "C" fn aa_str_perms(
    file_rules: *mut aa_policydb,
    start: aa_state_t,
    name: *const i8,
    cond: *mut path_cond,
    perms: *mut aa_perms,
) -> aa_state_t {
    unsafe {
        let state = aa_dfa_match((*file_rules).dfa, start, name);
        *perms = *aa_lookup_condperms(current_fsuid(), file_rules, state, cond);
        state
    }
}

pub extern "C" fn __aa_path_perm(
    op: *const i8,
    subj_cred: *const cred,
    profile: *mut aa_profile,
    name: *const i8,
    request: u32,
    cond: *mut path_cond,
    flags: i32,
    perms: *mut aa_perms,
) -> i32 {
    unsafe {
        let rules = (*profile).label.rules[0];
        let mut e = 0;

        if profile_unconfined(profile) || ((flags & PATH_SOCK_COND) != 0) {
            return 0;
        }

        aa_str_perms((*rules).file, 0, name, cond, perms); // TODO: start[AA_CLASS_FILE]
        if request & !(*perms).allow != 0 {
            e = -libc::EACCES;
        }
        aa_audit_file(
            subj_cred,
            profile,
            perms,
            op,
            request,
            name,
            ptr::null(),
            ptr::null_mut(),
            (*cond).uid,
            ptr::null(),
            e,
        )
    }
}

pub extern "C" fn profile_path_perm(
    op: *const i8,
    subj_cred: *const cred,
    profile: *mut aa_profile,
    path: *const path,
    buffer: *mut i8,
    request: u32,
    cond: *mut path_cond,
    flags: i32,
    perms: *mut aa_perms,
) -> i32 {
    unsafe {
        let mut name: *const i8 = ptr::null();
        let error;

        if profile_unconfined(profile) {
            return 0;
        }

        error = path_name(
            op,
            subj_cred,
            &mut (*profile).label,
            path,
            flags,
            buffer,
            &mut name,
            cond,
            request,
        );
        if error != 0 {
            return error;
        }
        __aa_path_perm(op, subj_cred, profile, name, request, cond, flags, perms)
    }
}

/// aa_path_perm - do permissions check & audit for @path
/// @op: operation being checked
/// @subj_cred: subject cred
/// @label: profile being enforced  (NOT NULL)
/// @path: path to check permissions of  (NOT NULL)
/// @flags: any additional path flags beyond what the profile specifies
/// @request: requested permissions
/// @cond: conditional info for this request  (NOT NULL)
///
/// Returns: %0 else error if access denied or other error
pub extern "C" fn aa_path_perm(
    op: *const i8,
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    flags: i32,
    request: u32,
    cond: *mut path_cond,
) -> i32 {
    unsafe {
        let mut perms: aa_perms = core::mem::zeroed();
        let mut buffer: *mut i8 = ptr::null_mut();
        let error;

        let flags = flags | PATH_DELEGATE_DELETED | if s_isdir((*cond).mode) { PATH_IS_DIR } else { 0 };
        buffer = aa_get_buffer(false);
        if buffer.is_null() {
            return -libc::ENOMEM;
        }
        error = fn_for_each_confined(label, |profile| {
            profile_path_perm(op, subj_cred, profile, path, buffer, request, cond, flags, &mut perms)
        });

        aa_put_buffer(buffer);
        error
    }
}

/// xindex_is_subset - helper for aa_path_link
/// @link: link permission set
/// @target: target permission set
///
/// test target x permissions are equal OR a subset of link x permissions
/// this is done as part of the subset test, where a hardlink must have
/// a subset of permissions that the target has.
///
/// Returns: true if subset else false
pub fn xindex_is_subset(link: u32, target: u32) -> bool {
    if ((link & !AA_X_UNSAFE) != (target & !AA_X_UNSAFE))
        || ((link & AA_X_UNSAFE) != 0 && (target & AA_X_UNSAFE) == 0)
    {
        return false;
    }

    true
}

pub extern "C" fn profile_path_link(
    subj_cred: *const cred,
    profile: *mut aa_profile,
    link: *const path,
    buffer: *mut i8,
    target: *const path,
    buffer2: *mut i8,
    cond: *mut path_cond,
) -> i32 {
    unsafe {
        let rules = (*profile).label.rules[0];
        let mut lname: *const i8 = ptr::null();
        let mut tname: *const i8 = ptr::null();
        let mut lperms: aa_perms = core::mem::zeroed();
        let mut perms: aa_perms = core::mem::zeroed();
        let mut info: *const i8 = ptr::null();
        let mut request = AA_MAY_LINK;
        let state: aa_state_t;
        let mut error;

        error = path_name(
            OP_LINK.as_ptr() as *const i8,
            subj_cred,
            &mut (*profile).label,
            link,
            0,
            buffer,
            &mut lname,
            cond,
            AA_MAY_LINK,
        );
        if error != 0 {
            goto_audit!(error, 412);
        }

        error = path_name(
            OP_LINK.as_ptr() as *const i8,
            subj_cred,
            &mut (*profile).label,
            target,
            0,
            buffer2,
            &mut tname,
            cond,
            AA_MAY_LINK,
        );
        if error != 0 {
            goto_audit!(error, 412);
        }

        error = -libc::EACCES;
        state = aa_str_perms((*rules).file, 0, lname, cond, &mut lperms); // TODO: start[AA_CLASS_FILE]

        if (lperms.allow & AA_MAY_LINK) == 0 {
            goto_audit!(error, 412);
        }

        state = aa_dfa_null_transition((*rules).file, state);
        aa_str_perms((*rules).file, state, tname, cond, &mut perms);

        lperms.audit = perms.audit;
        lperms.quiet = perms.quiet;
        lperms.kill = perms.kill;

        if (perms.allow & AA_MAY_LINK) == 0 {
            info = b"target restricted\0".as_ptr() as *const i8;
            lperms = perms;
            goto_audit!(error, 412);
        }

        if (perms.allow & AA_LINK_SUBSET) == 0 {
            goto_audit_done!(412);
        }

        aa_str_perms((*rules).file, 0, tname, cond, &mut perms); // TODO: start[AA_CLASS_FILE]

        request = lperms.allow & !AA_MAY_LINK;
        lperms.allow &= perms.allow | AA_MAY_LINK;

        request |= AA_AUDIT_FILE_MASK & (lperms.allow & !perms.allow);
        if (request & !lperms.allow) != 0 {
            goto_audit!(error, 412);
        } else if ((lperms.allow & MAY_EXEC) != 0) && !xindex_is_subset(lperms.xindex, perms.xindex) {
            lperms.allow &= !MAY_EXEC;
            request |= MAY_EXEC;
            info = b"link not subset of target\0".as_ptr() as *const i8;
            goto_audit!(error, 412);
        }

        error = 0;

        aa_audit_file(
            subj_cred,
            profile,
            &lperms,
            OP_LINK.as_ptr() as *const i8,
            request,
            lname,
            tname,
            ptr::null_mut(),
            (*cond).uid,
            info,
            error,
        )
    }
}

// Macros for goto-like behavior in Rust (simplified for this translation)
macro_rules! goto_audit {
    ($e:expr, $line:expr) => {
        goto_audit!($e, $line, null());
    };
}

macro_rules! goto_audit_done {
    ($line:expr) => {};
}

/// aa_path_link - Handle hard link permission check
/// @subj_cred: subject cred
/// @label: the label being enforced  (NOT NULL)
/// @old_dentry: the target dentry  (NOT NULL)
/// @new_dir: directory the new link will be created in  (NOT NULL)
/// @new_dentry: the link being created  (NOT NULL)
///
/// Handle the permission test for a link & target pair.  Permission
/// is encoded as a pair where the link permission is determined
/// first, and if allowed, the target is tested.  The target test
/// is done from the point of the link match (not start of DFA)
/// making the target permission dependent on the link permission match.
///
/// The subset test if required forces that permissions granted
/// on link are a subset of the permission granted to target.
///
/// Returns: %0 if allowed else error
pub extern "C" fn aa_path_link(
    subj_cred: *const cred,
    label: *mut aa_label,
    old_dentry: *mut dentry,
    new_dir: *const path,
    new_dentry: *mut dentry,
) -> i32 {
    unsafe {
        let link = path {
            mnt: (*new_dir).mnt,
            dentry: new_dentry,
        };
        let target = path {
            mnt: (*new_dir).mnt,
            dentry: old_dentry,
        };
        let inode = d_backing_inode(old_dentry);
        let vfsuid = i_uid_into_vfsuid(mnt_idmap(target.mnt), inode);
        let cond = path_cond {
            uid: vfsuid_into_kuid(vfsuid),
            mode: (*inode).i_mode,
        };
        let mut buffer: *mut i8 = ptr::null_mut();
        let mut buffer2: *mut i8 = ptr::null_mut();
        let mut error;

        buffer = aa_get_buffer(false);
        buffer2 = aa_get_buffer(false);
        error = -libc::ENOMEM;
        if buffer.is_null() || buffer2.is_null() {
            goto_label!(out);
        }

        error = fn_for_each_confined(label, |profile| {
            profile_path_link(subj_cred, profile, &link, buffer, &target, buffer2, &cond as *const _ as *mut _)
        });

        goto_label!(out);

        out: {
            aa_put_buffer(buffer);
            aa_put_buffer(buffer2);
            error
        }
    }
}

macro_rules! goto_label {
    ($label:ident) => {
        // Simplified for translation; actual control flow needs adjustment
    };
}

pub unsafe extern "C" fn update_file_ctx(fctx: *mut aa_file_ctx, label: *mut aa_label, request: u32) {
    let mut old;

    spin_lock(&mut (*fctx).lock);
    old = rcu_dereference_protected(
        (*fctx).label as *mut core::ffi::c_void,
        true, // TODO: lockdep_is_held(&fctx->lock)
    ) as *mut aa_label;
    let l = aa_label_merge(old, label, 0); // GFP_ATOMIC
    if !l.is_null() {
        if l != old {
            rcu_assign_pointer(
                &mut (*fctx).label as *mut _ as *mut *mut core::ffi::c_void,
                l as *mut core::ffi::c_void,
            );
            aa_put_label(old);
        } else {
            aa_put_label(l);
        }
        (*fctx).allow |= request;
    }
    spin_unlock(&mut (*fctx).lock);
}

pub extern "C" fn __file_path_perm(
    op: *const i8,
    subj_cred: *const cred,
    label: *mut aa_label,
    flabel: *mut aa_label,
    file: *mut file,
    request: u32,
    denied: u32,
    in_atomic: bool,
) -> i32 {
    unsafe {
        let mut perms: aa_perms = core::mem::zeroed();
        let vfsuid = i_uid_into_vfsuid(file_mnt_idmap(file), file_inode(file));
        let cond = path_cond {
            uid: vfsuid_into_kuid(vfsuid),
            mode: (*file_inode(file)).i_mode,
        };
        let buffer: *mut i8;
        let flags;
        let error;

        if denied == 0 && aa_label_is_subset(flabel, label) {
            return 0;
        }

        flags = PATH_DELEGATE_DELETED | if s_isdir(cond.mode) { PATH_IS_DIR } else { 0 };
        buffer = aa_get_buffer(in_atomic);
        if buffer.is_null() {
            return -libc::ENOMEM;
        }

        error = fn_for_each_not_in_set(flabel, label, |profile| {
            profile_path_perm(op, subj_cred, profile, &(*file).f_path, buffer, request, &cond as *const _ as *mut _, flags, &mut perms)
        });

        if denied != 0 && error == 0 {
            if label == flabel {
                error = fn_for_each(label, |profile| {
                    profile_path_perm(
                        op,
                        subj_cred,
                        profile,
                        &(*file).f_path,
                        buffer,
                        request,
                        &cond as *const _ as *mut _,
                        flags,
                        &mut perms,
                    )
                });
            } else {
                error = fn_for_each_not_in_set(label, flabel, |profile| {
                    profile_path_perm(
                        op,
                        subj_cred,
                        profile,
                        &(*file).f_path,
                        buffer,
                        request,
                        &cond as *const _ as *mut _,
                        flags,
                        &mut perms,
                    )
                });
            }
        }
        if error == 0 {
            update_file_ctx(file_ctx(file), label, request);
        }

        aa_put_buffer(buffer);

        error
    }
}

pub extern "C" fn __file_sock_perm(
    op: *const i8,
    subj_cred: *const cred,
    label: *mut aa_label,
    flabel: *mut aa_label,
    file: *mut file,
    request: u32,
    denied: u32,
) -> i32 {
    unsafe {
        let error;

        if denied == 0 && aa_label_is_subset(flabel, label) {
            return 0;
        }

        error = aa_sock_file_perm(subj_cred, label, op, request, file);
        if denied != 0 {
            let sock_error = aa_sock_file_perm(subj_cred, flabel, op, request, file);
            return last_error(error, sock_error);
        }
        if error == 0 {
            update_file_ctx(file_ctx(file), label, request);
        }

        error
    }
}

pub unsafe extern "C" fn __file_is_delegated(obj_label: *mut aa_label) -> bool {
    unconfined(obj_label)
}

pub unsafe extern "C" fn __is_unix_file(file: *mut file) -> bool {
    let sock = (*file).private_data as *mut socket;

    lockdep_assert_in_rcu_read_lock();

    if !s_issock((*file_inode(file)).i_mode) {
        return false;
    }
    if sock.is_null() || (*sock).sk.is_null() {
        return false;
    }
    // TODO: check sock->sk->sk_family == PF_UNIX
    true
}

pub unsafe extern "C" fn __unix_needs_revalidation(
    file: *mut file,
    label: *mut aa_label,
    request: u32,
) -> bool {
    let sock = (*file).private_data as *mut socket;

    aa_bug(!__is_unix_file(file));
    lockdep_assert_in_rcu_read_lock();

    let skctx = aa_sock((*sock).sk);

    if rcu_access_pointer((*skctx).peer as *mut core::ffi::c_void)
        != rcu_access_pointer((*skctx).peer_lastupdate as *mut core::ffi::c_void)
    {
        return true;
    }

    !__aa_subj_label_is_cached(
        rcu_dereference((*skctx).label as *mut core::ffi::c_void) as *mut aa_label,
        label,
    )
}

#[repr(C)]
pub struct file_path {
    pub dentry: *mut dentry,
}

/// aa_file_perm - do permission revalidation check & audit for @file
/// @op: operation being checked
/// @subj_cred: subject cred
/// @label: label being enforced   (NOT NULL)
/// @file: file to revalidate access permissions on  (NOT NULL)
/// @request: requested permissions
/// @in_atomic: whether allocations need to be done in atomic context
///
/// Returns: %0 if access allowed else error
pub extern "C" fn aa_file_perm(
    op: *const i8,
    subj_cred: *const cred,
    label: *mut aa_label,
    file: *mut file,
    request: u32,
    in_atomic: bool,
) -> i32 {
    unsafe {
        let fctx: *mut aa_file_ctx;
        let mut flabel: *mut aa_label;
        let denied: u32;
        let mut error = 0;

        aa_bug(label.is_null());
        aa_bug(file.is_null());

        if (*file).f_path.dentry == aa_null.dentry {
            return -libc::EACCES;
        }

        fctx = file_ctx(file);

        rcu_read_lock();
        flabel = rcu_dereference((*fctx).label as *mut core::ffi::c_void) as *mut aa_label;
        aa_bug(flabel.is_null());

        denied = request & !(*fctx).allow;
        if unconfined(label)
            || __file_is_delegated(flabel)
            || (!denied == 0 && __is_unix_file(file) && !__unix_needs_revalidation(file, label, request))
            || (!denied == 0 && __aa_subj_label_is_cached(label, flabel))
        {
            rcu_read_unlock();
            return error; // goto done
        }

        flabel = aa_get_newest_label(flabel);
        rcu_read_unlock();

        if path_mediated_fs((*file).f_path.dentry) {
            error = __file_path_perm(op, subj_cred, label, flabel, file, request, denied, in_atomic);
        } else if s_issock((*file_inode(file)).i_mode) {
            error = __file_sock_perm(op, subj_cred, label, flabel, file, request, denied);
        }
        aa_put_label(flabel);

        error
    }
}

pub unsafe extern "C" fn revalidate_tty(subj_cred: *const cred, label: *mut aa_label) {
    let tty: *mut tty_struct;
    let mut drop_tty = 0;

    tty = get_current_tty();
    if tty.is_null() {
        return;
    }

    spin_lock(&mut (*tty).files_lock as *mut spinlock_t);
    if !(*tty).tty_files_empty() {
        let file_priv: *mut tty_file_private;
        let file: *mut file;
        file_priv = (*tty).tty_files_first();
        file = (*file_priv).file;

        if aa_file_perm(
            OP_INHERIT.as_ptr() as *const i8,
            subj_cred,
            label,
            file,
            MAY_READ | MAY_WRITE,
            true,
        ) != 0
        {
            drop_tty = 1;
        }
    }
    spin_unlock(&mut (*tty).files_lock as *mut spinlock_t);
    tty_kref_put(tty);

    if drop_tty != 0 {
        no_tty();
    }
}

pub struct cred_label {
    pub cred: *const cred,
    pub label: *mut aa_label,
}

pub extern "C" fn match_file(p: *const core::ffi::c_void, file: *mut file, fd: u32) -> i32 {
    unsafe {
        let cl = p as *const cred_label;

        if aa_file_perm(
            OP_INHERIT.as_ptr() as *const i8,
            (*cl).cred,
            (*cl).label,
            file,
            aa_map_file_to_perms(file),
            true,
        ) != 0
        {
            return (fd + 1) as i32;
        }
        0
    }
}

/// based on selinux's flush_unauthorized_files
pub extern "C" fn aa_inherit_files(cred: *const cred, files: *mut files_struct) {
    unsafe {
        let label = aa_get_newest_cred_label(cred);
        let cl = cred_label {
            cred,
            label,
        };
        let mut devnull: *mut file = ptr::null_mut();
        let mut n: u32;

        revalidate_tty(cred, label);

        n = iterate_fd(files, 0, match_file, &cl as *const _ as *const core::ffi::c_void);
        if n == 0 {
            goto_label!(out);
        }

        devnull = dentry_open(&aa_null, O_RDWR, cred);
        if is_err(devnull as *mut core::ffi::c_void) {
            devnull = ptr::null_mut();
        }

        loop {
            replace_fd(n - 1, devnull, 0);
            n = iterate_fd(files, n, match_file, &cl as *const _ as *const core::ffi::c_void);
            if n == 0 {
                break;
            }
        }

        if !devnull.is_null() {
            fput(devnull);
        }

        out: {
            aa_put_label(label);
        }
    }
}

// Note: Various macro expansions and kernel abstractions are simplified.
// See include files and kernel source for complete definitions.



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
