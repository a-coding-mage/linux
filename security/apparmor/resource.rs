// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor resource mediation and attachment
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Dependencies from C headers:
// - linux/audit.h
// - linux/security.h
// - include/audit.h
// - include/cred.h
// - include/resource.h
// - include/policy.h
// - rlim_names.h (generated from resource.h)

use std::os::raw::{c_int, c_uint, c_ulong};

// ============================================================================
// External types and functions from linked code
// ============================================================================

extern "C" {
    // From linux/audit.h
    fn audit_log_format(ab: *mut AuditBuffer, fmt: *const u8, ...) -> ();

    // From include/audit.h
    fn aa_audit(
        audit_type: c_int,
        profile: *mut AaProfile,
        ad: *mut AppArmorAuditData,
        cb: unsafe extern "C" fn(*mut AuditBuffer, *mut core::ffi::c_void) -> (),
    ) -> c_int;
    fn aa_label_xaudit(
        ab: *mut AuditBuffer,
        ns: *mut core::ffi::c_void,
        label: *mut AaLabel,
        flags: c_uint,
        gfp: c_uint,
    ) -> ();
    fn aad(sa: *mut CommonAuditData) -> *mut AppArmorAuditData;

    // From include/cred.h
    fn aa_get_newest_cred_label(cred: *mut Cred) -> *mut AaLabel;
    fn __task_cred(task: *mut TaskStruct) -> *mut Cred;
    fn aa_capable(
        cred: *const Cred,
        label: *mut AaLabel,
        cap: c_uint,
        opt: c_uint,
    ) -> c_int;
    fn aa_put_label(label: *mut AaLabel) -> ();

    // From include/policy.h
    fn labels_ns(label: *mut AaLabel) -> *mut core::ffi::c_void;
    fn labels_profile(label: *mut AaLabel) -> *mut AaProfile;
    fn update_rlimit_cpu(task: *mut TaskStruct, limit: c_ulong) -> c_int;

    // Global state
    static mut current: *mut TaskStruct;
    static mut init_task: TaskStruct;

    // Generated from rlim_names.h
    static rlim_names: *const *const u8;
    static rlim_map: *const c_int;
}

// ============================================================================
// Type definitions (opaque external types)
// ============================================================================

#[repr(C)]
pub struct AuditBuffer;

#[repr(C)]
pub struct CommonAuditData;

#[repr(C)]
pub struct Cred;

#[repr(C)]
pub struct TaskStruct;

#[repr(C)]
pub struct AaLabel;

#[repr(C)]
pub struct AaProfile;

#[repr(C)]
pub struct AaRuleset;

#[repr(C)]
pub struct AaSfsEntry;

#[repr(C)]
pub struct LabelIt;

#[repr(C)]
pub struct SignalStruct;

// ============================================================================
// Type definitions (used in this file)
// ============================================================================

#[repr(C)]
pub struct AppArmorAuditData {
    pub subj_cred: *mut Cred,
    pub rlim: RlimitInfo,
    pub peer: *mut AaLabel,
    pub info: *const u8,
    pub error: c_int,
    // Note: Additional fields may exist in the full structure
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RlimitInfo {
    pub rlim: c_uint,
    pub max: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit {
    pub rlim_cur: c_ulong,
    pub rlim_max: c_ulong,
}

// ============================================================================
// Constants
// ============================================================================

const FLAGS_NONE: c_uint = 0;
const GFP_ATOMIC: c_uint = 32;
const AUDIT_APPARMOR_AUTO: c_int = 2100;
const LSM_AUDIT_DATA_NONE: c_int = 0;
const AA_CLASS_RLIMITS: c_int = 25;
const OP_SETRLIMIT: c_int = 0;
const CAP_SYS_RESOURCE: c_uint = 25;
const CAP_OPT_NOAUDIT: c_uint = 0;
const RLIMIT_CPU: c_int = 0;
const RLIM_INFINITY: c_ulong = c_ulong::MAX;
const RLIM_NLIMITS: c_int = 16;
const EACCES: c_int = -13;

// AA_SFS_FILE_STRING and AA_SFS_RLIMIT_MASK are macro-defined strings,
// typically from rlim_names.h
const AA_SFS_FILE_STRING_NAME: &[u8] = b"mask";

// ============================================================================
// Static data
// ============================================================================

pub static AA_SFS_ENTRY_RLIMIT: &[AaSfsEntry] = &[];

// ============================================================================
// Functions
// ============================================================================

// audit callback for resource specific fields
unsafe extern "C" fn audit_cb(ab: *mut AuditBuffer, va: *mut core::ffi::c_void) {
    let sa = va as *mut CommonAuditData;
    let ad = aad(sa);

    audit_log_format(
        ab,
        b" rlimit=%s value=%lu\0" as *const u8,
        *(rlim_names as *const *const u8).add((*ad).rlim.rlim as usize),
        (*ad).rlim.max,
    );
    if !(*ad).peer.is_null() {
        audit_log_format(ab, b" peer=\0" as *const u8);
        aa_label_xaudit(
            ab,
            labels_ns((*ad).subj_label),
            (*ad).peer,
            FLAGS_NONE,
            GFP_ATOMIC,
        );
    }
}

/**
 * audit_resource - audit setting resource limit
 * @subj_cred: cred setting the resource
 * @profile: profile being enforced  (NOT NULL)
 * @resource: rlimit being auditing
 * @value: value being set
 * @peer: aa_label of the task being set
 * @info: info being auditing
 * @error: error value
 *
 * Returns: 0 or ad->error else other error code on failure
 */
unsafe fn audit_resource(
    subj_cred: *const Cred,
    profile: *mut AaProfile,
    resource: c_uint,
    value: c_ulong,
    peer: *mut AaLabel,
    info: *const u8,
    error: c_int,
) -> c_int {
    // Equivalent of: DEFINE_AUDIT_DATA(ad, LSM_AUDIT_DATA_NONE, AA_CLASS_RLIMITS, OP_SETRLIMIT);
    // This macro typically declares and zero-initializes an apparmor_audit_data struct
    let mut ad: AppArmorAuditData = core::mem::zeroed();

    ad.subj_cred = subj_cred as *mut Cred;
    ad.rlim.rlim = resource;
    ad.rlim.max = value;
    ad.peer = peer;
    ad.info = info;
    ad.error = error;

    aa_audit(AUDIT_APPARMOR_AUTO, profile, &mut ad, audit_cb)
}

/**
 * aa_map_resource - map compiled policy resource to internal #
 * @resource: flattened policy resource number
 *
 * Returns: resource # for the current architecture.
 *
 * rlimit resource can vary based on architecture, map the compiled policy
 * resource # to the internal representation for the architecture.
 */
#[no_mangle]
pub extern "C" fn aa_map_resource(resource: c_int) -> c_int {
    unsafe { *(rlim_map as *const c_int).add(resource as usize) }
}

fn profile_setrlimit(
    subj_cred: *const Cred,
    profile: *mut AaProfile,
    resource: c_uint,
    new_rlim: *const rlimit,
) -> c_int {
    unsafe {
        // Access profile->label.rules[0] requires traversing the profile structure
        // The actual field access depends on internal AaProfile layout
        // TODO: Full structure access: rules = profile->label.rules[0]
        // TODO: Check: if (rules->rlimits.mask & (1 << resource) &&
        //              new_rlim->rlim_max > rules->rlimits.limits[resource].rlim_max)

        let e: c_int = 0;

        audit_resource(
            subj_cred,
            profile,
            resource,
            (*new_rlim).rlim_max,
            core::ptr::null_mut(),
            core::ptr::null(),
            e,
        )
    }
}

/**
 * aa_task_setrlimit - test permission to set an rlimit
 * @subj_cred: cred setting the limit
 * @label: label confining the task  (NOT NULL)
 * @task: task the resource is being set on
 * @resource: the resource being set
 * @new_rlim: the new resource limit  (NOT NULL)
 *
 * Control raising the processes hard limit.
 *
 * Returns: 0 or error code if setting resource failed
 */
#[no_mangle]
pub unsafe extern "C" fn aa_task_setrlimit(
    subj_cred: *const Cred,
    label: *mut AaLabel,
    task: *mut TaskStruct,
    resource: c_uint,
    new_rlim: *mut rlimit,
) -> c_int {
    let mut profile: *mut AaProfile;
    let mut peer: *mut AaLabel;
    let mut error: c_int = 0;

    // rcu_read_lock();
    peer = aa_get_newest_cred_label(__task_cred(task));
    // rcu_read_unlock();

    // TODO: extend resource control to handle other (non current)
    // profiles.  AppArmor rules currently have the implicit assumption
    // that the task is setting the resource of a task confined with
    // the same profile or that the task setting the resource of another
    // task has CAP_SYS_RESOURCE.

    if label != peer && aa_capable(subj_cred, label, CAP_SYS_RESOURCE, CAP_OPT_NOAUDIT) != 0 {
        // fn_for_each(label, profile, ...)
        // This macro expands to iterate over profiles in the label
        // For each profile, the audit_resource call is executed
        // TODO: Expand fn_for_each macro with actual iteration over profiles
        profile = core::ptr::null_mut(); // Placeholder; actual iteration needed
        error = audit_resource(
            subj_cred,
            profile,
            resource,
            (*new_rlim).rlim_max,
            peer,
            b"cap_sys_resource\0" as *const u8,
            -EACCES,
        );
    } else {
        // label_for_each_confined(i, label, profile)
        // This macro expands to iterate over confined profiles in the label
        // For each profile, profile_setrlimit is called
        // TODO: Expand label_for_each_confined macro with actual iteration
        profile = core::ptr::null_mut(); // Placeholder; actual iteration needed
        error = profile_setrlimit(subj_cred, profile, resource, new_rlim);
    }
    aa_put_label(peer);

    error
}

/**
 * __aa_transition_rlimits - apply new profile rlimits
 * @old_l: old label on task  (NOT NULL)
 * @new_l: new label with rlimits to apply  (NOT NULL)
 */
#[no_mangle]
pub unsafe extern "C" fn __aa_transition_rlimits(old_l: *mut AaLabel, new_l: *mut AaLabel) {
    let mut mask: c_uint = 0;
    let mut rlim: *mut rlimit;
    let mut initrlim: *mut rlimit;
    let mut old: *mut AaProfile;
    let mut new: *mut AaProfile;

    old = labels_profile(old_l);
    new = labels_profile(new_l);

    // for any rlimits the profile controlled, reset the soft limit
    // to the lesser of the tasks hard limit and the init tasks soft limit

    // label_for_each_confined(i, old_l, old) { ... }
    // TODO: Expand label_for_each_confined macro iteration
    // Within iteration:
    // struct aa_ruleset *rules = old->label.rules[0];
    // if (rules->rlimits.mask) {
    //     for (j = 0, mask = 1; j < RLIM_NLIMITS; j++, mask <<= 1) {
    //         if (rules->rlimits.mask & mask) {
    //             rlim = current->signal->rlim + j;
    //             initrlim = init_task.signal->rlim + j;
    //             rlim->rlim_cur = min(rlim->rlim_max, initrlim->rlim_cur);
    //         }
    //     }
    // }

    let mut j: c_int = 0;
    mask = 1;
    while j < RLIM_NLIMITS {
        // Process rlimit entries
        // Requires access to: old->label.rules[0]->rlimits
        // and: current->signal->rlim, init_task.signal->rlim
        // TODO: Full structure access for rules and rlimit arrays
        j += 1;
        mask <<= 1;
    }

    // set any new hard limits as dictated by the new profile

    // label_for_each_confined(i, new_l, new) { ... }
    // TODO: Expand label_for_each_confined macro iteration
    // Within iteration:
    // struct aa_ruleset *rules = new->label.rules[0];
    // if (!rules->rlimits.mask)
    //     continue;
    // for (j = 0, mask = 1; j < RLIM_NLIMITS; j++, mask <<= 1) {
    //     if (!(rules->rlimits.mask & mask))
    //         continue;
    //     rlim = current->signal->rlim + j;
    //     rlim->rlim_max = min(rlim->rlim_max, rules->rlimits.limits[j].rlim_max);
    //     rlim->rlim_cur = min(rlim->rlim_cur, rlim->rlim_max);
    //     if (j == RLIMIT_CPU &&
    //         rlim->rlim_cur != RLIM_INFINITY &&
    //         IS_ENABLED(CONFIG_POSIX_TIMERS))
    //             (void) update_rlimit_cpu(current->group_leader, rlim->rlim_cur);
    // }

    let mut j: c_int = 0;
    mask = 1;
    while j < RLIM_NLIMITS {
        // Process rlimit entries for new profile
        // Requires access to: new->label.rules[0]->rlimits
        // and: current->signal->rlim
        // Conditional call to update_rlimit_cpu based on CONFIG_POSIX_TIMERS
        // TODO: Full structure access for rules, limits, and update_rlimit_cpu conditional
        j += 1;
        mask <<= 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
