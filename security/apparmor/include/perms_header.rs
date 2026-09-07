// SPDX-License-Identifier: GPL-2.0-only
// AppArmor security module: basic permission sets definitions.

use std::ffi::{c_char, c_int, c_void};

// MAY_EXEC, MAY_WRITE, MAY_READ, and MAY_APPEND are supplied by linux/fs.h.
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
pub const AA_EXEC_MMAP: u32 = 0x0001_0000;
pub const AA_MAY_MPROT: u32 = 0x0002_0000;
pub const AA_MAY_LINK: u32 = 0x0004_0000;
pub const AA_MAY_SNAPSHOT: u32 = 0x0008_0000;
// AA_MAY_DELEGATE is an intentionally valueless C preprocessor marker.
pub const AA_CONT_MATCH: u32 = 0x0800_0000;
pub const AA_MAY_STACK: u32 = 0x1000_0000;
pub const AA_MAY_ONEXEC: u32 = 0x2000_0000;
pub const AA_MAY_CHANGE_PROFILE: u32 = 0x4000_0000;
pub const AA_MAY_CHANGEHAT: u32 = 0x8000_0000;
pub const AA_LINK_SUBSET: u32 = AA_MAY_LOCK;
pub const AA_MAY_CREATE_SQPOLL: u32 = AA_MAY_CREATE;
// AA_MAY_APPEND and the following masks depend on linux/fs.h values.
// pub const AA_MAY_OVERRIDE_CRED: u32 = AA_MAY_APPEND;
// pub const AA_URING_PERM_MASK: u32 = AA_MAY_OVERRIDE_CRED | AA_MAY_CREATE_SQPOLL;
// pub const PERMS_CHRS_MASK: u32 = MAY_READ | MAY_WRITE | AA_MAY_CREATE | AA_MAY_DELETE |
//     AA_MAY_LINK | AA_MAY_LOCK | MAY_EXEC | AA_EXEC_MMAP | MAY_APPEND;
// pub const PERMS_NAMES_MASK: u32 = PERMS_CHRS_MASK | AA_MAY_OPEN | AA_MAY_RENAME |
//     AA_MAY_SETATTR | AA_MAY_GETATTR | AA_MAY_SETCRED | AA_MAY_GETCRED | AA_MAY_CHMOD |
//     AA_MAY_CHOWN | AA_MAY_CHGRP | AA_MAY_MPROT | AA_MAY_SNAPSHOT | AA_MAY_STACK |
//     AA_MAY_ONEXEC | AA_MAY_CHANGE_PROFILE | AA_MAY_CHANGEHAT;

#[repr(C)]
pub struct aa_perms {
    pub allow: u32, pub deny: u32, pub subtree: u32, pub cond: u32,
    pub kill: u32, pub complain: u32, pub prompt: u32,
    pub audit: u32, pub quiet: u32, pub hide: u32,
    pub xindex: u32, pub tag: u32, pub label: u32,
}

pub const AA_INDEX_MASK: u32 = 0x00ff_ffff;
pub const AA_INDEX_FLAG_MASK: u32 = 0xff00_0000;
pub const AA_INDEX_NONE: u32 = 0;
pub const ALL_PERMS_MASK: u32 = 0xffff_ffff;

extern "C" {
    pub static aa_file_perm_chrs: c_char;
    pub static aa_file_perm_names: *const *const c_char;
    pub static nullperms: aa_perms;
    pub static allperms: aa_perms;
    pub static default_perms: aa_perms;
}

#[inline]
pub unsafe fn aa_perms_accum_raw(accum: *mut aa_perms, addend: *const aa_perms) {
    let a = &mut *accum; let b = &*addend;
    a.deny |= b.deny; a.allow &= b.allow & !b.deny; a.audit |= b.audit & b.allow;
    a.quiet &= b.quiet & !b.allow; a.kill |= b.kill & !b.allow;
    a.complain |= b.complain & !b.allow & !b.deny; a.cond |= b.cond & !b.allow & !b.deny;
    a.hide &= b.hide & !b.allow; a.prompt |= b.prompt & !b.allow & !b.deny;
    a.subtree |= b.subtree & !b.deny;
    if a.xindex == 0 { a.xindex = b.xindex; } if a.tag == 0 { a.tag = b.tag; }
    if a.label == 0 { a.label = b.label; }
}

#[inline]
pub unsafe fn aa_perms_accum(accum: *mut aa_perms, addend: *const aa_perms) {
    let a = &mut *accum; let b = &*addend;
    a.deny |= b.deny; a.allow &= b.allow & !a.deny; a.audit |= b.audit & a.allow;
    a.quiet &= b.quiet & !a.allow; a.kill |= b.kill & !a.allow;
    a.complain |= b.complain & !a.allow & !a.deny; a.cond |= b.cond & !a.allow & !a.deny;
    a.hide &= b.hide & !a.allow; a.prompt |= b.prompt & !a.allow & !a.deny;
    a.subtree &= b.subtree & !a.deny;
    if a.xindex == 0 { a.xindex = b.xindex; } if a.tag == 0 { a.tag = b.tag; }
    if a.label == 0 { a.label = b.label; }
}

macro_rules! xcheck { ($fn1:expr, $fn2:expr) => {{ let mut error = $fn1; let e = $fn2; if e != 0 { error = e; } error }}; }
// Cross-check macros depend on aa_profile, aa_label, and fn_for_each from other headers.
macro_rules! xcheck_ns_profile_profile { ($p1:expr, $p2:expr, $fn:ident $(, $args:tt)*) => {{ let mut e = 0i32; if (*$p1).ns == (*$p2).ns { e = $fn($p1, $p2, $($args),*); } e }}; }
macro_rules! xcheck_ns_profile_label { ($p:expr, $l:expr, $fn:ident $(, $args:tt)*) => {{ fn_for_each!($l, __p2, xcheck_ns_profile_profile!($p, __p2, $fn, $($args),*)) }}; }
macro_rules! xcheck_ns_labels { ($l1:expr, $l2:expr, $fn:ident $(, $args:tt)*) => {{ fn_for_each!($l1, __p1, $fn(__p1, $l2, $($args),*)) }}; }
macro_rules! xcheck_labels_profiles { ($l1:expr, $l2:expr, $fn:ident $(, $args:tt)*) => {{ xcheck_ns_labels!($l1, $l2, xcheck_ns_profile_label, $fn, $($args),*) }}; }
macro_rules! xcheck_labels { ($l1:expr, $l2:expr, $p:expr, $fn1:ident, $fn2:ident) => {{ xcheck!(fn_for_each!($l1, $p, $fn1), fn_for_each!($l2, $p, $fn2)) }}; }

extern "C" {
    pub fn aa_perm_mask_to_str(str_: *mut c_char, str_size: usize, chrs: *const c_char, mask: u32);
    pub fn aa_audit_perm_names(ab: *mut c_void, names: *const *const c_char, mask: u32);
    pub fn aa_audit_perm_mask(ab: *mut c_void, mask: u32, chrs: *const c_char, chrsmask: u32, names: *const *const c_char, namesmask: u32);
    pub fn aa_apply_modes_to_perms(profile: *const c_void, perms: *mut aa_perms);
    pub fn aa_profile_match_label(profile: *const c_void, rules: *mut c_void, label: *mut c_void, type_: c_int, request: u32, perms: *mut aa_perms);
    pub fn aa_check_perms(profile: *mut c_void, perms: *const aa_perms, request: u32, ad: *mut c_void, cb: Option<extern "C" fn(*mut c_void, *mut c_void)>) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
