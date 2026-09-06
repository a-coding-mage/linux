// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor basic permission sets definitions.
//
// Copyright 2017 Canonical Ltd.

// External dependencies:
// - linux/fs.h (for MAY_READ, MAY_WRITE, MAY_EXEC, MAY_APPEND)
// - label.h (for aa_profile, aa_label, etc.)

use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;

// Permission constants that map to external linux/fs.h constants
// Values from MAY_READ, MAY_WRITE, MAY_EXEC, MAY_APPEND defined in linux/fs.h
// Note: these are conditionally assigned to the linux/fs.h values at build time
// pub const AA_MAY_EXEC: u32 = MAY_EXEC;
// pub const AA_MAY_WRITE: u32 = MAY_WRITE;
// pub const AA_MAY_READ: u32 = MAY_READ;
// pub const AA_MAY_APPEND: u32 = MAY_APPEND;

pub const AA_MAY_CREATE: u32 = 0x0010;
pub const AA_MAY_DELETE: u32 = 0x0020;
pub const AA_MAY_OPEN: u32 = 0x0040;
pub const AA_MAY_RENAME: u32 = 0x0080;

pub const AA_MAY_SETATTR: u32 = 0x0100;
pub const AA_MAY_GETATTR: u32 = 0x0200;
pub const AA_MAY_SETCRED: u32 = 0x0400;
pub const AA_MAY_GETCRED: u32 = 0x0800;

pub const AA_MAY_CHMOD: u32 = 0x1000;
pub const AA_MAY_CHOWN: u32 = 0x2000;
pub const AA_MAY_CHGRP: u32 = 0x4000;
pub const AA_MAY_LOCK: u32 = 0x8000;

pub const AA_EXEC_MMAP: u32 = 0x00010000;
pub const AA_MAY_MPROT: u32 = 0x00020000;
pub const AA_MAY_LINK: u32 = 0x00040000;
pub const AA_MAY_SNAPSHOT: u32 = 0x00080000;

pub const AA_CONT_MATCH: u32 = 0x08000000;

pub const AA_MAY_STACK: u32 = 0x10000000;
pub const AA_MAY_ONEXEC: u32 = 0x20000000;
pub const AA_MAY_CHANGE_PROFILE: u32 = 0x40000000;
pub const AA_MAY_CHANGEHAT: u32 = 0x80000000;

pub const AA_LINK_SUBSET: u32 = AA_MAY_LOCK;

pub const AA_MAY_CREATE_SQPOLL: u32 = AA_MAY_CREATE;
pub const AA_MAY_OVERRIDE_CRED: u32 = AA_MAY_APPEND;
pub const AA_URING_PERM_MASK: u32 = AA_MAY_OVERRIDE_CRED | AA_MAY_CREATE_SQPOLL;

// Note: PERMS_CHRS_MASK and PERMS_NAMES_MASK depend on external MAY_* constants
// from linux/fs.h and cannot be defined without those values being available.
// These should be defined at build time with the proper external constant values.
// pub const PERMS_CHRS_MASK: u32 = (MAY_READ | MAY_WRITE | AA_MAY_CREATE |
//     AA_MAY_DELETE | AA_MAY_LINK | AA_MAY_LOCK | AA_MAY_EXEC | AA_EXEC_MMAP | AA_MAY_APPEND);
// pub const PERMS_NAMES_MASK: u32 = (PERMS_CHRS_MASK | AA_MAY_OPEN | AA_MAY_RENAME |
//     AA_MAY_SETATTR | AA_MAY_GETATTR | AA_MAY_SETCRED | AA_MAY_GETCRED | AA_MAY_CHMOD |
//     AA_MAY_CHOWN | AA_MAY_CHGRP | AA_MAY_MPROT | AA_MAY_SNAPSHOT | AA_MAY_STACK |
//     AA_MAY_ONEXEC | AA_MAY_CHANGE_PROFILE | AA_MAY_CHANGEHAT);

pub const AA_INDEX_MASK: u32 = 0x00ffffff;
pub const AA_INDEX_FLAG_MASK: u32 = 0xff000000;
pub const AA_INDEX_NONE: u32 = 0;

pub const ALL_PERMS_MASK: u32 = 0xffffffff;

#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub deny: u32,

    pub subtree: u32,
    pub cond: u32,

    pub kill: u32,
    pub complain: u32,
    pub prompt: u32,

    pub audit: u32,
    pub quiet: u32,
    pub hide: u32,

    pub xindex: u32,
    pub tag: u32,
    pub label: u32,
}

extern "C" {
    pub static aa_file_perm_chrs: c_char;
    pub static aa_file_perm_names: *const *const c_char;
}

extern "C" {
    pub static nullperms: aa_perms;
    pub static allperms: aa_perms;
}

#[inline]
pub unsafe fn aa_perms_accum_raw(accum: *mut aa_perms, addend: *const aa_perms) {
    let accum = &mut *accum;
    let addend = &*addend;

    accum.deny |= addend.deny;
    accum.allow &= addend.allow & !addend.deny;
    accum.audit |= addend.audit & addend.allow;
    accum.quiet &= addend.quiet & !addend.allow;
    accum.kill |= addend.kill & !addend.allow;
    accum.complain |= addend.complain & !addend.allow & !addend.deny;
    accum.cond |= addend.cond & !addend.allow & !addend.deny;
    accum.hide &= addend.hide & !addend.allow;
    accum.prompt |= addend.prompt & !addend.allow & !addend.deny;
    accum.subtree |= addend.subtree & !addend.deny;

    if accum.xindex == 0 {
        accum.xindex = addend.xindex;
    }
    if accum.tag == 0 {
        accum.tag = addend.tag;
    }
    if accum.label == 0 {
        accum.label = addend.label;
    }
}

#[inline]
pub unsafe fn aa_perms_accum(accum: *mut aa_perms, addend: *const aa_perms) {
    let accum = &mut *accum;
    let addend = &*addend;

    accum.deny |= addend.deny;
    accum.allow &= addend.allow & !accum.deny;
    accum.audit |= addend.audit & accum.allow;
    accum.quiet &= addend.quiet & !accum.allow;
    accum.kill |= addend.kill & !accum.allow;
    accum.complain |= addend.complain & !accum.allow & !addend.deny;
    accum.cond |= addend.cond & !accum.allow & !addend.deny;
    accum.hide &= addend.hide & !accum.allow;
    accum.prompt |= addend.prompt & !accum.allow & !addend.deny;
    accum.subtree &= addend.subtree & !accum.deny;

    if accum.xindex == 0 {
        accum.xindex = addend.xindex;
    }
    if accum.tag == 0 {
        accum.tag = addend.tag;
    }
    if accum.label == 0 {
        accum.label = addend.label;
    }
}

macro_rules! xcheck {
    ($fn1:expr, $fn2:expr) => {{
        let mut error = $fn1;
        let e = $fn2;
        if e != 0 {
            error = e;
        }
        error
    }};
}

// Note: The following macros depend on external types and macros that are not
// defined in this header. They reference:
// - struct aa_profile (external)
// - struct aa_label (external)
// - struct aa_ruleset (external)
// - fn_for_each (external macro)
//
// The macros are translated as-is with their original logic, but the actual
// compilation will require these external dependencies to be available.

macro_rules! xcheck_ns_profile_profile {
    ($p1:expr, $p2:expr, $fn:ident $(, $args:tt)*) => {{
        let mut ____e = 0i32;
        if (*$p1).ns == (*$p2).ns {
            ____e = $fn($p1, $p2, $($args),*);
        }
        ____e
    }};
}

macro_rules! xcheck_ns_profile_label {
    ($p:expr, $l:expr, $fn:ident $(, $args:tt)*) => {{
        fn_for_each!($l, __p2,
            xcheck_ns_profile_profile!($p, __p2, $fn, $($args),*))
    }};
}

macro_rules! xcheck_ns_labels {
    ($l1:expr, $l2:expr, $fn:ident $(, $args:tt)*) => {{
        fn_for_each!($l1, __p1, $fn(__p1, $l2, $($args),*))
    }};
}

macro_rules! xcheck_labels_profiles {
    ($l1:expr, $l2:expr, $fn:ident $(, $args:tt)*) => {{
        xcheck_ns_labels!($l1, $l2, xcheck_ns_profile_label, $fn, $($args),*)
    }};
}

macro_rules! xcheck_labels {
    ($l1:expr, $l2:expr, $p:expr, $fn1:ident, $fn2:ident) => {{
        xcheck!(fn_for_each!($l1, $p, $fn1), fn_for_each!($l2, $p, $fn2))
    }};
}

extern "C" {
    pub static default_perms: aa_perms;

    pub fn aa_perm_mask_to_str(
        str_: *mut c_char,
        str_size: usize,
        chrs: *const c_char,
        mask: u32,
    );

    pub fn aa_audit_perm_names(
        ab: *mut c_void,
        names: *const *const c_char,
        mask: u32,
    );

    pub fn aa_audit_perm_mask(
        ab: *mut c_void,
        mask: u32,
        chrs: *const c_char,
        chrsmask: u32,
        names: *const *const c_char,
        namesmask: u32,
    );

    pub fn aa_apply_modes_to_perms(profile: *const c_void, perms: *mut aa_perms);

    pub fn aa_profile_match_label(
        profile: *const c_void,
        rules: *mut c_void,
        label: *mut c_void,
        type_: c_int,
        request: u32,
        perms: *mut aa_perms,
    );

    pub fn aa_check_perms(
        profile: *mut c_void,
        perms: *const aa_perms,
        request: u32,
        ad: *mut c_void,
        cb: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
