// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor auditing functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Linux audit/socket and AppArmor declarations are supplied by external dependencies.

pub const audit_mode_names: [&str; 6] = [
    "normal", "quiet_denied", "quiet.allowed", "quiet", "noquiet", "all",
];

static aa_audit_type: [&str; 8] = [
    "AUDIT", "ALLOWED", "DENIED", "HINT", "STATUS", "ERROR", "KILLED", "AUTO",
];

static aa_class_names: [&str; 33] = [
    "none", "unknown", "file", "cap", "net", "rlimits", "domain", "mount",
    "unknown", "ptrace", "signal", "xmatch", "unknown", "unknown", "net", "netv9",
    "label", "posix_mqueue", "io_uring", "module", "lsm", "namespace", "io_uring",
    "unknown", "unknown", "unknown", "unknown", "unknown", "unknown", "unknown",
    "netv9_packet", "X", "dbus",
];

// Currently AppArmor auditing is fed straight into the audit framework.
// TODO: netlink interface for complain mode; user auditing and system control.

#[repr(C)]
pub struct aa_audit_rule { pub label: *mut aa_label }

extern "C" {
    pub type audit_buffer;
    pub type apparmor_audit_data;
    pub type aa_label;
    pub type aa_profile;
    pub type label_it;
    pub type audit_krule;
    pub type lsm_prop;
    static aa_g_audit_header: bool;
    static root_ns: *mut core::ffi::c_void;
    fn aad_of_va(va: *mut core::ffi::c_void) -> *mut apparmor_audit_data;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const u8, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const u8);
    fn label_isprofile(label: *mut aa_label) -> bool;
    fn labels_profile(label: *mut aa_label) -> *mut aa_profile;
    fn aa_label_xaudit(ab: *mut audit_buffer, ns: *mut core::ffi::c_void, label: *mut aa_label, flags: u32, gfp: u32);
    fn common_lsm_audit(common: *mut core::ffi::c_void, pre: unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void), cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>);
    fn send_sig_info(sig: i32, info: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> i32;
    fn aa_put_label(label: *mut aa_label);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kzalloc_obj(size: usize, gfp: u32) -> *mut core::ffi::c_void;
    fn aa_label_parse(label: *mut aa_label, rulestr: *mut u8, gfp: u32, b1: bool, b2: bool) -> *mut aa_label;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn aa_label_is_subset(label: *mut aa_label, subset: *mut aa_label) -> bool;
    fn complain_error(error: i32) -> i32;
}

const AUDIT_APPARMOR_AUDIT: i32 = 0;
const AUDIT_APPARMOR_KILL: i32 = 1;
const AUDIT_APPARMOR_ALLOWED: i32 = 2;
const AUDIT_APPARMOR_DENIED: i32 = 3;
const AUDIT_APPARMOR_AUTO: i32 = 7;
const AUDIT_SUBJ_ROLE: u32 = 1;
const AUDIT_QUIET: u32 = 3;
const AUDIT_QUIET_DENIED: u32 = 1;
const AUDIT_ALL: u32 = 5;
const Audit_equal: u32 = 0;
const Audit_not_equal: u32 = 1;
const AA_CLASS_LAST: u32 = 32;
const FLAG_VIEW_SUBNS: u32 = 1;
const GFP_ATOMIC: u32 = 0;
const LSM_AUDIT_DATA_TASK: u32 = 1;
const SEND_SIG_NOINFO: *mut core::ffi::c_void = core::ptr::null_mut();
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const ENOENT: i32 = -2;

#[repr(C)] pub struct aa_perms { pub kill: u32, pub complain: u32 }
static nullperms: aa_perms = aa_perms { kill: 0, complain: 0 };

unsafe extern "C" fn audit_pre(ab: *mut audit_buffer, va: *mut core::ffi::c_void) {
    let ad = aad_of_va(va);
    // Field access/layout is provided by the external AppArmor declarations.
    if *aa_g_audit_header { audit_log_format(ab, b"apparmor=\"%s\"\0".as_ptr(), aa_audit_type[(*ad).type_ as usize].as_ptr()); }
    if !(*ad).op.is_null() { audit_log_format(ab, b" operation=\"%s\"\0".as_ptr(), (*ad).op); }
    if (*ad).class != 0 { let n = if (*ad).class <= AA_CLASS_LAST { aa_class_names[(*ad).class as usize].as_ptr() } else { b"unknown\0".as_ptr() }; audit_log_format(ab, b" class=\"%s\"\0".as_ptr(), n); }
    if !(*ad).info.is_null() { audit_log_format(ab, b" info=\"%s\"\0".as_ptr(), (*ad).info); if (*ad).error != 0 { audit_log_format(ab, b" error=%d\0".as_ptr(), (*ad).error); } }
    if !(*ad).name.is_null() { audit_log_format(ab, b" name=\0".as_ptr()); audit_log_untrustedstring(ab, (*ad).name); }
}

pub unsafe extern "C" fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> i32 {
    if denied == 0 { AUDIT_APPARMOR_AUDIT } else if denied & (*perms).kill != 0 { AUDIT_APPARMOR_KILL } else if denied == denied & (*perms).complain { AUDIT_APPARMOR_ALLOWED } else { AUDIT_APPARMOR_DENIED }
}

// The remaining entry points retain the C ABI; their implementations are supplied by the translated companion declarations.
pub unsafe extern "C" fn aa_audit_msg(type_: i32, ad: *mut apparmor_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>) {
    (*ad).type_ = type_;
    common_lsm_audit(&mut (*ad).common as *mut _, audit_pre, cb);
}

pub unsafe extern "C" fn aa_audit_perm_error(label: *mut aa_label, request: u32, error: i32, ad: *mut apparmor_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>) -> i32 {
    let type_ = aa_select_audit_type(request, &nullperms);
    if !ad.is_null() {
        (*ad).request = request; (*ad).denied = request; (*ad).error = error;
        let mut i: label_it = core::mem::zeroed();
        label_for_each_confined(&mut i, label, |profile| { (*ad).subj_label = &mut (*profile).label; aa_audit_msg(type_, ad, cb); });
    }
    error
}

pub unsafe extern "C" fn aa_audit(type_: i32, profile: *mut aa_profile, ad: *mut apparmor_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut core::ffi::c_void)>) -> i32 {
    debug_assert!(!profile.is_null());
    let mut ty = type_;
    if ty == AUDIT_APPARMOR_AUTO {
        if (*ad).error == 0 { if AUDIT_MODE(profile) != AUDIT_ALL { return 0; } ty = AUDIT_APPARMOR_AUDIT; }
        else if COMPLAIN_MODE(profile) { ty = AUDIT_APPARMOR_ALLOWED; } else { ty = AUDIT_APPARMOR_DENIED; }
    }
    if AUDIT_MODE(profile) == AUDIT_QUIET || (ty == AUDIT_APPARMOR_DENIED && AUDIT_MODE(profile) == AUDIT_QUIET_DENIED) { return (*ad).error; }
    if KILL_MODE(profile) && ty == AUDIT_APPARMOR_DENIED { ty = AUDIT_APPARMOR_KILL; }
    (*ad).subj_label = &mut (*profile).label; aa_audit_msg(ty, ad, cb);
    if (*ad).type_ == AUDIT_APPARMOR_KILL { send_sig_info((*profile).signal, SEND_SIG_NOINFO, (*ad).common_task_or_current()); }
    if (*ad).type_ == AUDIT_APPARMOR_ALLOWED { return complain_error((*ad).error); } (*ad).error
}

pub unsafe extern "C" fn aa_audit_rule_init(field: u32, op: u32, rulestr: *mut u8, vrule: *mut *mut core::ffi::c_void, gfp: u32) -> i32 {
    if field != AUDIT_SUBJ_ROLE || (op != Audit_equal && op != Audit_not_equal) { return EINVAL; }
    let rule = kzalloc_obj(core::mem::size_of::<aa_audit_rule>(), gfp) as *mut aa_audit_rule; if rule.is_null() { return ENOMEM; }
    (*rule).label = aa_label_parse(root_ns as *mut aa_label, rulestr, gfp, true, false);
    if IS_ERR((*rule).label as *const _) { let e = PTR_ERR((*rule).label as *const _); aa_audit_rule_free(rule as *mut _); return e; }
    *vrule = rule as *mut _; 0
}
pub unsafe extern "C" fn aa_audit_rule_known(_rule: *mut audit_krule) -> i32 { 0 }
pub unsafe extern "C" fn aa_audit_rule_match(prop: *mut lsm_prop, field: u32, op: u32, vrule: *mut core::ffi::c_void) -> i32 { let rule=vrule as *mut aa_audit_rule; let label=(*prop).apparmor.label; if label.is_null(){return ENOENT;} let found=aa_label_is_subset(label,(*rule).label); if field==AUDIT_SUBJ_ROLE { if op==Audit_equal{return found as i32;} if op==Audit_not_equal{return (!found) as i32;} } 0 }

extern "C" { fn AUDIT_MODE(p:*mut aa_profile)->u32; fn COMPLAIN_MODE(p:*mut aa_profile)->bool; fn KILL_MODE(p:*mut aa_profile)->bool; fn label_for_each_confined(i:*mut label_it,l:*mut aa_label); }
pub unsafe extern "C" fn aa_audit_rule_free(vrule: *mut core::ffi::c_void) { let rule = vrule as *mut aa_audit_rule; if !rule.is_null() { if !IS_ERR((*rule).label as *const _) { aa_put_label((*rule).label); } kfree(vrule); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
