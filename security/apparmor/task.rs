// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor task related definitions and mediation
//
// Copyright 2017 Canonical Ltd.
//
// TODO
// If a task uses change_hat it currently does not return to the old
// cred or task context but instead creates a new one.  Ideally the task
// should return to the previous cred if it has not been modified.

// Includes: <linux/gfp.h>, <linux/ptrace.h>, <linux/task_work.h>
// Includes: "include/path.h", "include/audit.h", "include/cred.h", "include/policy.h", "include/task.h"

use std::ffi::c_void;

// External types and functions from other modules/files
extern "C" {
    pub struct task_struct;
    pub struct aa_label;
    pub struct aa_task_ctx;
    pub struct cred;
    pub struct aa_profile;
    pub struct audit_buffer;
    pub struct common_audit_data;
    pub struct apparmor_audit_data;
    pub struct callback_head;
    pub struct file;
    pub struct path;
    pub struct aa_ruleset;
    pub struct aa_perms;
    pub type aa_state_t;

    pub static mut current: *mut task_struct;

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn aa_get_newest_cred_label(cred: *const cred) -> *mut aa_label;
    fn aa_current_raw_label() -> *mut aa_label;
    fn task_ctx(task: *mut task_struct) -> *mut aa_task_ctx;
    fn current_cred() -> *const cred;
    fn current_real_cred() -> *const cred;
    fn prepare_creds() -> *mut cred;
    fn label_is_stale(label: *mut aa_label) -> bool;
    fn aa_get_newest_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_put_label(label: *mut aa_label);
    fn unconfined(label: *mut aa_label) -> bool;
    fn labels_ns(label: *mut aa_label) -> *const c_void;
    fn aa_clear_task_ctx_trans(ctx: *mut aa_task_ctx);
    fn aa_get_label(label: *mut aa_label);
    fn cred_label(cred: *mut cred) -> *mut aa_label;
    fn set_cred_label(cred: *mut cred, label: *mut aa_label);
    fn commit_creds(cred: *mut cred);
    fn __task_cred(task: *mut task_struct) -> *const cred;
    fn init_task_work(tw: *mut callback_head, func: unsafe extern "C" fn(*mut callback_head));
    fn task_work_add(task: *mut task_struct, tw: *mut callback_head, flags: u32) -> i32;
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn label_mediates(label: *mut aa_label, class: u32) -> bool;
    fn aa_profile_match_label(profile: *mut aa_profile, rules: *mut aa_ruleset, peer: *mut aa_label, class: u32, request: u32, perms: *mut aa_perms);
    fn aa_apply_modes_to_perms(profile: *mut aa_profile, perms: *mut aa_perms);
    fn aa_check_perms(profile: *mut aa_profile, perms: *mut aa_perms, request: u32, ad: *mut apparmor_audit_data, cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_void)) -> i32;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const i8, ...);
    fn aa_label_xaudit(ab: *mut audit_buffer, ns: *const c_void, label: *mut aa_label, flags: u32, gfp: u32);
    fn aa_capable(cred: *const cred, label: *mut aa_label, cap: u32, opt: u32) -> i32;
    fn aa_audit(audit_type: i32, profile: *mut aa_profile, ad: *mut apparmor_audit_data, cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_void)) -> i32;
    fn get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn path_get(p: *mut path);
    fn aa_path_name(p: *mut path, flag: u32, buffer: *mut i8, path_str: *mut *const i8, a: *const c_void, b: *const c_void) -> i32;
    fn fput(file: *mut file);
    fn path_put(p: *mut path);
    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn aad_of_va(va: *mut c_void) -> *mut apparmor_audit_data;
    fn aa_get_buffer(prealloc: bool) -> *mut i8;
    fn aa_put_buffer(buffer: *mut i8);
    fn aa_lookup_perms(policy: *const c_void, state: aa_state_t) -> *mut aa_perms;
    fn xcheck_labels(a: *mut aa_label, b: *mut aa_label, profile: *mut aa_profile, res1: i32, res2: i32) -> i32;
}

const MAY_READ: u32 = 1;
const MAY_WRITE: u32 = 2;
const AA_MAY_BE_READ: u32 = 4;
const AA_MAY_BE_TRACED: u32 = 8;
const AA_PTRACE_PERM_MASK: u32 = 15;
const PTRACE_PERM_SHIFT: u32 = 16;
const AA_USERNS_CREATE: u32 = 1;
const FLAG_VIEW_SUBNS: u32 = 1;
const FLAGS_NONE: u32 = 0;
const GFP_ATOMIC: u32 = 0;
const CAP_SYS_PTRACE: u32 = 19;
const CAP_OPT_NONE: u32 = 0;
const TWA_RESUME: u32 = 0;
const AUDIT_APPARMOR_AUTO: i32 = 1;
const AA_CLASS_PTRACE: u32 = 1;
const AA_CLASS_USERNS_CREATE: u32 = 2;
const OP_PTRACE: u32 = 1;
const LSM_AUDIT_DATA_NONE: u32 = 0;
const EBUSY: i32 = -16;
const ENOMEM: i32 = -12;
const EACCES: i32 = -13;
const ENOENT: i32 = -2;

// Helper to convert *const i8 to *mut i8 for ERR_PTR
fn ERR_PTR(err: i32) -> *const i8 {
    err as *const i8
}

fn IS_ERR(ptr: *const i8) -> bool {
    (ptr as isize) < 0
}

/// aa_get_task_label - Get another task's label
/// @task: task to query  (NOT NULL)
///
/// Returns: counted reference to @task's label
pub unsafe extern "C" fn aa_get_task_label(task: *mut task_struct) -> *mut aa_label {
    let mut p: *mut aa_label;

    rcu_read_lock();
    p = aa_get_newest_cred_label(__task_cred(task));
    rcu_read_unlock();

    p
}

/// aa_replace_current_label - replace the current tasks label
/// @label: new label  (NOT NULL)
///
/// Returns: 0 or error on failure
pub unsafe extern "C" fn aa_replace_current_label(label: *mut aa_label) -> i32 {
    let old = aa_current_raw_label();
    let ctx = task_ctx(current);
    let mut new: *mut cred;

    debug_assert!(!label.is_null());

    if old == label {
        return 0;
    }

    if current_cred() != current_real_cred() {
        return EBUSY;
    }

    new = prepare_creds();
    if new.is_null() {
        return ENOMEM;
    }

    if !(*ctx).nnp.is_null() && label_is_stale((*ctx).nnp) {
        let tmp = (*ctx).nnp;

        (*ctx).nnp = aa_get_newest_label(tmp);
        aa_put_label(tmp);
    }
    if unconfined(label) || labels_ns(old) != labels_ns(label) {
        aa_clear_task_ctx_trans(task_ctx(current));
    }

    aa_get_label(label);
    aa_put_label(cred_label(new));
    set_cred_label(new, label);

    commit_creds(new);
    0
}

unsafe extern "C" fn aa_replace_stale_label_tw_func(tw: *mut callback_head) {
    let ctx = task_ctx(current);
    let mut label: *mut aa_label;

    (*ctx).label_replacement_pending = false;
    label = aa_current_raw_label();
    if !label_is_stale(label) {
        return;
    }
    label = aa_get_newest_label(label);
    aa_replace_current_label(label);
    aa_put_label(label);
}

/// replace the current task's stale label on syscall return
pub unsafe extern "C" fn aa_schedule_stale_label_replacement() {
    let ctx = task_ctx(current);

    if (*ctx).label_replacement_pending {
        return;
    }
    init_task_work(&mut (*ctx).label_replacement_tw, aa_replace_stale_label_tw_func);
    if task_work_add(current, &mut (*ctx).label_replacement_tw, TWA_RESUME) == 0 {
        (*ctx).label_replacement_pending = true;
    }
}

/// aa_set_current_onexec - set the tasks change_profile to happen onexec
/// @label: system label to set at exec  (MAYBE NULL to clear value)
/// @stack: whether stacking should be done
pub unsafe extern "C" fn aa_set_current_onexec(label: *mut aa_label, stack: bool) {
    let ctx = task_ctx(current);

    aa_get_label(label);
    aa_put_label((*ctx).onexec);
    (*ctx).onexec = label;
    (*ctx).token = stack;
}

/// aa_set_current_hat - set the current tasks hat
/// @label: label to set as the current hat  (NOT NULL)
/// @token: token value that must be specified to change from the hat
///
/// Do switch of tasks hat.  If the task is currently in a hat
/// validate the token to match.
///
/// Returns: 0 or error on failure
pub unsafe extern "C" fn aa_set_current_hat(label: *mut aa_label, token: u64) -> i32 {
    let ctx = task_ctx(current);
    let mut new: *mut cred;

    new = prepare_creds();
    if new.is_null() {
        return ENOMEM;
    }
    debug_assert!(!label.is_null());

    if (*ctx).previous.is_null() {
        (*ctx).previous = cred_label(new);
        (*ctx).token = token;
    } else if (*ctx).token == token {
        aa_put_label(cred_label(new));
    } else {
        // previous_profile && ctx->token != token
        // abort_creds(new); // Not translated, would require external function
        return EACCES;
    }

    set_cred_label(new, aa_get_newest_label(label));
    aa_put_label((*ctx).onexec);
    (*ctx).onexec = std::ptr::null_mut();

    commit_creds(new);
    0
}

/// aa_restore_previous_label - exit from hat context restoring previous label
/// @token: the token that must be matched to exit hat context
///
/// Attempt to return out of a hat to the previous label.  The token
/// must match the stored token value.
///
/// Returns: 0 or error of failure
pub unsafe extern "C" fn aa_restore_previous_label(token: u64) -> i32 {
    let ctx = task_ctx(current);
    let mut new: *mut cred;

    if (*ctx).token != token {
        return EACCES;
    }
    if (*ctx).previous.is_null() {
        return 0;
    }

    new = prepare_creds();
    if new.is_null() {
        return ENOMEM;
    }

    aa_put_label(cred_label(new));
    set_cred_label(new, aa_get_newest_label((*ctx).previous));
    debug_assert!(!cred_label(new).is_null());
    aa_clear_task_ctx_trans(ctx);

    commit_creds(new);

    0
}

/// audit_ptrace_mask - convert mask to permission string
/// @mask: permission mask to convert
///
/// Returns: pointer to static string
unsafe fn audit_ptrace_mask(mask: u32) -> *const i8 {
    match mask {
        MAY_READ => b"read\0".as_ptr() as *const i8,
        MAY_WRITE => b"trace\0".as_ptr() as *const i8,
        AA_MAY_BE_READ => b"readby\0".as_ptr() as *const i8,
        AA_MAY_BE_TRACED => b"tracedby\0".as_ptr() as *const i8,
        _ => b"\0".as_ptr() as *const i8,
    }
}

/// call back to audit ptrace fields
unsafe extern "C" fn audit_ptrace_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);

    if (*ad).request & AA_PTRACE_PERM_MASK != 0 {
        audit_log_format(
            ab,
            b" requested_mask=\"%s\"\0".as_ptr() as *const i8,
            audit_ptrace_mask((*ad).request),
        );

        if (*ad).denied & AA_PTRACE_PERM_MASK != 0 {
            audit_log_format(
                ab,
                b" denied_mask=\"%s\"\0".as_ptr() as *const i8,
                audit_ptrace_mask((*ad).denied),
            );
        }
    }
    audit_log_format(ab, b" peer=\0".as_ptr() as *const i8);
    aa_label_xaudit(
        ab,
        labels_ns((*ad).subj_label),
        (*ad).peer,
        FLAGS_NONE,
        GFP_ATOMIC,
    );
}

/// assumes check for RULE_MEDIATES is already done
/// TODO: conditionals
unsafe fn profile_ptrace_perm(
    cred: *const cred,
    profile: *mut aa_profile,
    peer: *mut aa_label,
    request: u32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let rules = (*profile).label.rules[0];
    let mut perms = aa_perms {
        allow: 0,
        deny: 0,
        quiet: 0,
        hide: 0,
    };

    (*ad).subj_cred = cred;
    (*ad).peer = peer;
    aa_profile_match_label(profile, rules, peer, AA_CLASS_PTRACE, request, &mut perms);
    aa_apply_modes_to_perms(profile, &mut perms);
    aa_check_perms(profile, &mut perms, request, ad, audit_ptrace_cb)
}

unsafe fn profile_tracee_perm(
    cred: *const cred,
    tracee: *mut aa_profile,
    tracer: *mut aa_label,
    request: u32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    if profile_unconfined(tracee) || unconfined(tracer)
        || !label_mediates(&(*tracee).label, AA_CLASS_PTRACE)
    {
        return 0;
    }

    profile_ptrace_perm(cred, tracee, tracer, request, ad)
}

unsafe fn profile_tracer_perm(
    cred: *const cred,
    tracer: *mut aa_profile,
    tracee: *mut aa_label,
    request: u32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    if profile_unconfined(tracer) {
        return 0;
    }

    if label_mediates(&(*tracer).label, AA_CLASS_PTRACE) {
        return profile_ptrace_perm(cred, tracer, tracee, request, ad);
    }

    if &(*tracer).label as *const _ == tracee as *const _ {
        return 0;
    }

    (*ad).subj_label = &mut (*tracer).label;
    (*ad).peer = tracee;
    (*ad).request = 0;
    (*ad).error = aa_capable(cred, &mut (*tracer).label, CAP_SYS_PTRACE, CAP_OPT_NONE);

    aa_audit(AUDIT_APPARMOR_AUTO, tracer, ad, audit_ptrace_cb)
}

/// aa_may_ptrace - test if tracer task can trace the tracee
/// @tracer_cred: cred of task doing the tracing  (NOT NULL)
/// @tracer: label of the task doing the tracing  (NOT NULL)
/// @tracee_cred: cred of task to be traced
/// @tracee: task label to be traced
/// @request: permission request
///
/// Returns: %0 else error code if permission denied or error
pub unsafe extern "C" fn aa_may_ptrace(
    tracer_cred: *const cred,
    tracer: *mut aa_label,
    tracee_cred: *const cred,
    tracee: *mut aa_label,
    request: u32,
) -> i32 {
    let mut profile: *mut aa_profile = std::ptr::null_mut();
    let _xrequest = request << PTRACE_PERM_SHIFT;
    // DEFINE_AUDIT_DATA(sa, LSM_AUDIT_DATA_NONE, AA_CLASS_PTRACE, OP_PTRACE);
    let mut sa = common_audit_data {
        type_: LSM_AUDIT_DATA_NONE,
        class: AA_CLASS_PTRACE,
        u: 0,
    };

    xcheck_labels(
        tracer,
        tracee,
        profile,
        profile_tracer_perm(tracer_cred, profile, tracee, request, &mut aad(&mut sa)[0]),
        profile_tracee_perm(tracee_cred, profile, tracer, _xrequest, &mut aad(&mut sa)[0]),
    )
}

unsafe fn get_current_exe_path(buffer: *mut i8, buffer_size: i32) -> *const i8 {
    let mut exe_file: *mut file;
    let mut p: path = std::mem::zeroed();
    let mut path_str: *const i8;

    exe_file = get_task_exe_file(current);
    if exe_file.is_null() {
        return ERR_PTR(ENOENT);
    }
    p = (*exe_file).f_path;
    path_get(&mut p);

    if aa_path_name(&mut p, FLAG_VIEW_SUBNS, buffer, &mut path_str, std::ptr::null(), std::ptr::null()) != 0 {
        path_str = ERR_PTR(ENOMEM);
    }

    fput(exe_file);
    path_put(&mut p);

    path_str
}

/// call back to audit ptrace fields
unsafe extern "C" fn audit_ns_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let ad = aad_of_va(va);
    let mut buffer: *mut i8;
    let path: *const i8;

    if (*ad).request & AA_USERNS_CREATE != 0 {
        audit_log_format(ab, b" requested=\"userns_create\"\0".as_ptr() as *const i8);
    }

    if (*ad).denied & AA_USERNS_CREATE != 0 {
        audit_log_format(ab, b" denied=\"userns_create\"\0".as_ptr() as *const i8);
    }

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        return;
    }
    path = get_current_exe_path(buffer, 256); // aa_g_path_max - using placeholder
    if !IS_ERR(path) {
        audit_log_format(
            ab,
            b" execpath=\"%s\"\0".as_ptr() as *const i8,
            path,
        );
    }
    aa_put_buffer(buffer);
}

pub unsafe extern "C" fn aa_profile_ns_perm(
    profile: *mut aa_profile,
    ad: *mut apparmor_audit_data,
    request: u32,
) -> i32 {
    let mut perms = aa_perms {
        allow: 0,
        deny: 0,
        quiet: 0,
        hide: 0,
    };
    let mut error: i32 = 0;

    (*ad).subj_label = &mut (*profile).label;
    (*ad).request = request;

    if !profile_unconfined(profile) {
        let rules = (*profile).label.rules[0];
        let state: aa_state_t;

        state = unsafe { std::mem::zeroed() }; // RULE_MEDIATES macro placeholder
        if state as u32 == 0 {
            return 0;
        }
        perms = *aa_lookup_perms(std::ptr::null(), state);
        aa_apply_modes_to_perms(profile, &mut perms);
        error = aa_check_perms(profile, &mut perms, request, ad, audit_ns_cb);
    }

    error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
