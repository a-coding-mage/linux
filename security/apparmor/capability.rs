// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor capability mediation functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Includes from linux/capability.h, linux/errno.h, linux/gfp.h, linux/security.h, linux/timekeeping.h
// Includes from include/apparmor.h, include/capability.h, include/cred.h, include/policy.h, include/audit.h
// Table of capability names generated from capabilities.h via capability_names.h

use core::ffi::c_int;

// External types
extern "C" {
    pub type audit_buffer;
    pub type common_audit_data;
    pub type apparmor_audit_data;
    pub type aa_profile;
    pub type aa_label;
    pub type aa_ruleset;
    pub type aa_perms;
    pub type aa_state_t;
    pub type kernel_cap_t;
    pub type cred;

    // External array from capability_names.h
    pub static capability_names: *const *const u8;

    // External constants
    pub static AA_SFS_CAPS_MASK: c_int;

    // External functions
    pub fn audit_log_format(ab: *mut audit_buffer, fmt: *const u8, ...);
    pub fn audit_log_untrustedstring(ab: *mut audit_buffer, str_: *const u8);
    pub fn get_cpu_var(var: *mut u8) -> *mut u8;
    pub fn put_cpu_var(var: *mut u8);
    pub fn ktime_get_ns() -> u64;
    pub fn get_cred(cred: *const cred) -> *const cred;
    pub fn put_cred(cred: *const cred);
    pub fn cap_raised(caps: kernel_cap_t, cap: c_int) -> bool;
    pub fn aa_dfa_next(dfa: *mut u8, state: aa_state_t, bit: c_int) -> aa_state_t;
    pub fn aa_lookup_perms(policy: *mut u8, state: aa_state_t) -> *mut aa_perms;
    pub fn aa_apply_modes_to_perms(profile: *mut aa_profile, perms: *mut aa_perms);
    pub fn aa_check_perms(
        profile: *mut aa_profile,
        perms: *const aa_perms,
        request: u32,
        ad: *mut apparmor_audit_data,
        cb: unsafe extern "C" fn(*mut audit_buffer, *mut u8),
    ) -> c_int;
    pub fn aa_audit(
        typ: c_int,
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        cb: unsafe extern "C" fn(*mut audit_buffer, *mut u8),
    ) -> c_int;
    pub fn fn_for_each_confined(
        label: *mut aa_label,
        profile: *mut *mut aa_profile,
        cb: unsafe extern "C" fn(*mut aa_profile, c_int, u32, *mut apparmor_audit_data) -> c_int,
    ) -> c_int;
    pub fn complain_error(error: c_int) -> c_int;

    // External macros - functions that check profile state
    pub fn RULE_MEDIATES(rules: *mut aa_ruleset, class: c_int) -> aa_state_t;
    pub fn AUDIT_MODE(profile: *mut aa_profile) -> c_int;
    pub fn KILL_MODE(profile: *mut aa_profile) -> bool;
    pub fn COMPLAIN_MODE(profile: *mut aa_profile) -> bool;
}

// Constants
const AUDIT_CACHE_TIMEOUT_NS: u64 = 1_000_000_000; // 1 second
const CAP_OPT_NOAUDIT: u32 = 2;
const EPERM: c_int = -1;
const AUDIT_APPARMOR_AUTO: c_int = 0;
const AUDIT_APPARMOR_AUDIT: c_int = 1;
const AUDIT_APPARMOR_KILL: c_int = 2;
const AUDIT_NOQUIET: c_int = 0;
const AUDIT_ALL: c_int = 1;
const CAP_LAST_CAP: usize = 63;

// Struct definitions inferred from usage
#[repr(C)]
pub struct aa_sfs_entry {
    pub name: *const u8,
    pub value: c_int,
}

#[repr(C)]
pub struct audit_cache {
    pub ad_subj_cred: *const cred,
    // Capabilities go from 0 to CAP_LAST_CAP
    pub ktime_ns_expiration: [u64; 64], // CAP_LAST_CAP+1
}

// Per-CPU static variable for audit cache
// DEFINE_PER_CPU(struct audit_cache, audit_cache)
// Note: Direct per-CPU implementation requires runtime support not available in this file.
// Declaration placeholder for external per-CPU mechanism.
thread_local! {
    static AUDIT_CACHE: core::cell::RefCell<audit_cache> = core::cell::RefCell::new(audit_cache {
        ad_subj_cred: core::ptr::null(),
        ktime_ns_expiration: [0; 64],
    });
}

pub static mut aa_sfs_entry_caps: [aa_sfs_entry; 3] = [
    aa_sfs_entry {
        name: b"mask\0".as_ptr(),
        value: AA_SFS_CAPS_MASK,
    },
    aa_sfs_entry {
        name: b"extended\0".as_ptr(),
        value: 1,
    },
    aa_sfs_entry {
        name: core::ptr::null(),
        value: 0,
    },
];

/**
 * audit_cb - call back for capability components of audit struct
 * @ab: audit buffer   (NOT NULL)
 * @va: audit struct to audit data from  (NOT NULL)
 */
unsafe extern "C" fn audit_cb(ab: *mut audit_buffer, va: *mut u8) {
    let sa = va as *const common_audit_data;

    audit_log_format(ab, b" capname=\0".as_ptr());
    audit_log_untrustedstring(ab, *capability_names.add((*sa).u.cap as usize));
}

/**
 * audit_caps - audit a capability
 * @ad: audit data
 * @profile: profile being tested for confinement (NOT NULL)
 * @cap: capability tested
 * @error: error code returned by test
 *
 * Do auditing of capability and handle, audit/complain/kill modes switching
 * and duplicate message elimination.
 *
 * Returns: 0 or ad->error on success,  error code on failure
 */
unsafe fn audit_caps(
    ad: *mut apparmor_audit_data,
    profile: *mut aa_profile,
    cap: c_int,
    error: c_int,
) -> c_int {
    let rules = (*(*profile).label.rules[0]) as *mut aa_ruleset;
    let mut typ = AUDIT_APPARMOR_AUTO;

    (*ad).error = error;

    if error == 0 {
        // test if auditing is being forced
        if (AUDIT_MODE(profile) != AUDIT_ALL) && !cap_raised((*rules).caps.audit, cap) {
            return 0;
        }
        typ = AUDIT_APPARMOR_AUDIT;
    } else if KILL_MODE(profile) || cap_raised((*rules).caps.kill, cap) {
        typ = AUDIT_APPARMOR_KILL;
    } else if cap_raised((*rules).caps.quiet, cap)
        && AUDIT_MODE(profile) != AUDIT_NOQUIET
        && AUDIT_MODE(profile) != AUDIT_ALL
    {
        // quiet auditing
        return error;
    }

    // Do simple duplicate message elimination
    let ent = &mut *(get_cpu_var(core::ptr::null_mut::<u8>()) as *mut audit_cache);
    // If the capability was never raised the timestamp check would also catch that
    if (*ad).subj_cred == ent.ad_subj_cred
        && ktime_get_ns() <= ent.ktime_ns_expiration[cap as usize]
    {
        put_cpu_var(core::ptr::null_mut::<u8>());
        if COMPLAIN_MODE(profile) {
            return complain_error(error);
        }
        return error;
    } else {
        put_cred(ent.ad_subj_cred);
        ent.ad_subj_cred = get_cred((*ad).subj_cred);
        ent.ktime_ns_expiration[cap as usize] = ktime_get_ns() + AUDIT_CACHE_TIMEOUT_NS;
    }
    put_cpu_var(core::ptr::null_mut::<u8>());

    aa_audit(typ, profile, ad, audit_cb)
}

/**
 * profile_capable - test if profile allows use of capability @cap
 * @profile: profile being enforced    (NOT NULL, NOT unconfined)
 * @cap: capability to test if allowed
 * @opts: CAP_OPT_NOAUDIT bit determines whether audit record is generated
 * @ad: audit data (NOT NULL)
 *
 * Returns: 0 if allowed else -EPERM
 */
unsafe fn profile_capable(
    profile: *mut aa_profile,
    cap: c_int,
    opts: u32,
    ad: *mut apparmor_audit_data,
) -> c_int {
    let rules = (*(*profile).label.rules[0]) as *mut aa_ruleset;
    let state: aa_state_t;
    let error: c_int;

    state = RULE_MEDIATES(rules, 0); // AA_CLASS_CAP
    if state != 0 {
        let mut perms: aa_perms = core::mem::zeroed();
        let request: u32;

        // caps broken into 256 x 32 bit permission chunks
        let next_state = aa_dfa_next((*(*rules).policy).dfa, state, cap >> 5);
        request = 1 << (cap & 0x1f);
        perms = *aa_lookup_perms((*rules).policy, next_state);
        aa_apply_modes_to_perms(profile, &mut perms);

        if (opts & CAP_OPT_NOAUDIT) != 0 {
            if (perms.complain & request) != 0 {
                (*ad).info = b"optional: no audit\0".as_ptr();
            } else {
                ad = core::ptr::null_mut();
            }
        }
        return aa_check_perms(profile, &perms, request, ad, audit_cb);
    }

    // fallback to old caps mediation that doesn't support conditionals
    if cap_raised((*rules).caps.allow, cap) && !cap_raised((*rules).caps.denied, cap) {
        error = 0;
    } else {
        error = EPERM;
    }

    if (opts & CAP_OPT_NOAUDIT) != 0 {
        if !COMPLAIN_MODE(profile) {
            return error;
        }
        // audit the cap request in complain mode but note that it
        // should be optional.
        (*ad).info = b"optional: no audit\0".as_ptr();
    }

    audit_caps(ad, profile, cap, error)
}

/**
 * aa_capable - test permission to use capability
 * @subj_cred: cred we are testing capability against
 * @label: label being tested for capability (NOT NULL)
 * @cap: capability to be tested
 * @opts: CAP_OPT_NOAUDIT bit determines whether audit record is generated
 *
 * Look up capability in profile capability set.
 *
 * Returns: 0 on success, or else an error code.
 */
#[no_mangle]
pub extern "C" fn aa_capable(
    subj_cred: *const cred,
    label: *mut aa_label,
    cap: c_int,
    opts: u32,
) -> c_int {
    unsafe {
        let mut profile: *mut aa_profile = core::ptr::null_mut();
        let mut error: c_int = 0;
        // DEFINE_AUDIT_DATA(ad, LSM_AUDIT_DATA_CAP, AA_CLASS_CAP, OP_CAPABLE)
        let mut ad: apparmor_audit_data = core::mem::zeroed();

        (*(&mut ad as *mut apparmor_audit_data)).subj_cred = subj_cred;
        (*(&mut ad as *mut apparmor_audit_data)).common.u.cap = cap;
        error = fn_for_each_confined(label, &mut profile, profile_capable_callback);

        error
    }
}

unsafe extern "C" fn profile_capable_callback(
    profile: *mut aa_profile,
    cap: c_int,
    opts: u32,
    ad: *mut apparmor_audit_data,
) -> c_int {
    profile_capable(profile, cap, opts, ad)
}

#[no_mangle]
pub extern "C" fn aa_profile_capget(profile: *const aa_profile) -> kernel_cap_t {
    unsafe {
        let rules = (*(*profile).label.rules[0]) as *mut aa_ruleset;
        let state: aa_state_t;

        state = RULE_MEDIATES(rules, 0); // AA_CLASS_CAP
        if state != 0 {
            let mut caps: kernel_cap_t = core::mem::zeroed(); // CAP_EMPTY_SET
            let mut i: usize = 0;

            // caps broken into up to 256, 32 bit permission chunks
            while i < (CAP_LAST_CAP >> 5) {
                let mut perms: aa_perms = core::mem::zeroed();
                let tmp: aa_state_t;

                tmp = aa_dfa_next((*(*rules).policy).dfa, state, i as c_int);
                perms = *aa_lookup_perms((*rules).policy, tmp);
                aa_apply_modes_to_perms(profile as *mut aa_profile, &mut perms);
                caps.val |= (perms.allow as u64) << (i * 5);
                caps.val |= (perms.complain as u64) << (i * 5);
                i += 1;
            }
            return caps;
        }

        // fallback to old caps
        if COMPLAIN_MODE(profile as *mut aa_profile) {
            return CAP_FULL_SET;
        }

        (*rules).caps.allow
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
