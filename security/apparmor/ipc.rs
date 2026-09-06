// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor ipc mediation
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2017 Canonical Ltd.
 */

// Includes from <linux/gfp.h> and relative paths are external dependencies

// From include/audit.h
extern "C" {
    struct audit_buffer;
    struct common_audit_data;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...) -> c_void;
}

// From include/capability.h, include/cred.h, include/policy.h
extern "C" {
    struct cred;
    struct aa_label;
    struct aa_profile;
    struct aa_ruleset;

    struct apparmor_audit_data {
        request: u32,
        denied: u32,
        signal: i32,
        unmappedsig: i32,
        subj_cred: *const cred,
        peer: *mut aa_label,
        subj_label: *mut aa_label,
    }

    struct aa_perms {
        // Fields not specified in source
    }

    type aa_state_t = u32;

    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn aa_dfa_next(dfa: *const c_void, state: aa_state_t, c: i32) -> aa_state_t;
    fn aa_label_match(
        profile: *const aa_profile,
        rules: *mut aa_ruleset,
        peer: *mut aa_label,
        state: aa_state_t,
        subns: bool,
        request: u32,
        perms: *mut aa_perms,
    ) -> c_void;
    fn aa_apply_modes_to_perms(profile: *const aa_profile, perms: *mut aa_perms) -> c_void;
    fn aa_check_perms(
        profile: *const aa_profile,
        perms: *const aa_perms,
        request: u32,
        ad: *mut apparmor_audit_data,
        audit_cb: extern "C" fn(*mut audit_buffer, *mut c_void) -> c_void,
    ) -> i32;
    fn aa_label_xaudit(
        ab: *mut audit_buffer,
        ns: *const c_void,
        label: *mut aa_label,
        flags: u32,
        gfp: u32,
    ) -> c_void;
    fn labels_ns(label: *mut aa_label) -> *const c_void;
}

// From include/ipc.h
const AA_SIGNAL_PERM_MASK: u32 = 0; // Value not specified in source

// From include/sig_names.h
const SIGUNKNOWN: i32 = 0; // Value not specified in source
const MAXMAPPED_SIG: i32 = 0; // Value not specified in source
const MAXMAPPED_SIGNAME: i32 = 0; // Value not specified in source
const SIGRT_BASE: i32 = 0; // Value not specified in source
const SIGRTMIN: i32 = 0; // Value not specified in source
const SIGRTMAX: i32 = 0; // Value not specified in source
const MAY_READ: u32 = 0; // Value not specified in source
const MAY_WRITE: u32 = 0; // Value not specified in source
const AA_CLASS_SIGNAL: u32 = 0; // Value not specified in source
const OP_SIGNAL: u32 = 0; // Value not specified in source
const LSM_AUDIT_DATA_NONE: u32 = 0; // Value not specified in source
const FLAGS_NONE: u32 = 0; // Value not specified in source
const GFP_ATOMIC: u32 = 0; // Value not specified in source

extern "C" {
    static sig_map: [i32; 32];
    static sig_names: [*const c_char; 32];
}

// Macro: DEFINE_AUDIT_DATA - expands to local variable declaration
// In source: DEFINE_AUDIT_DATA(ad, LSM_AUDIT_DATA_NONE, AA_CLASS_SIGNAL, OP_SIGNAL);
// Maps to local variable initialization

// Macro: RULE_MEDIATES(rules, class) - checks if rules mediate class
// Maps to checking if rules mediate the given class

// Macro: xcheck_labels - iterates through labels and applies a check
// Maps to checking labels and applying permissions

use std::ffi::c_char;
use std::ffi::c_void;

#[inline]
fn map_signal_num(sig: i32) -> i32 {
    if sig > SIGRTMAX {
        return SIGUNKNOWN;
    } else if sig >= SIGRTMIN {
        return sig - SIGRTMIN + SIGRT_BASE;
    } else if sig < MAXMAPPED_SIG {
        unsafe { sig_map[sig as usize] }
    } else {
        SIGUNKNOWN
    }
}

/// audit_signal_mask - convert mask to permission string
/// @mask: permission mask to convert
///
/// Returns: pointer to static string
fn audit_signal_mask(mask: u32) -> &'static str {
    if (mask & MAY_READ) != 0 {
        "receive"
    } else if (mask & MAY_WRITE) != 0 {
        "send"
    } else {
        ""
    }
}

/// audit_signal_cb() - call back for signal specific audit fields
/// @ab: audit_buffer  (NOT NULL)
/// @va: audit struct to audit values of  (NOT NULL)
extern "C" fn audit_signal_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let sa = va as *mut common_audit_data;
    let ad = unsafe { aad(sa) };

    unsafe {
        if (*ad).request & AA_SIGNAL_PERM_MASK != 0 {
            audit_log_format(
                ab,
                b" requested_mask=\"%s\"\0".as_ptr() as *const c_char,
                audit_signal_mask((*ad).request).as_ptr(),
            );
            if (*ad).denied & AA_SIGNAL_PERM_MASK != 0 {
                audit_log_format(
                    ab,
                    b" denied_mask=\"%s\"\0".as_ptr() as *const c_char,
                    audit_signal_mask((*ad).denied).as_ptr(),
                );
            }
        }
        if (*ad).signal == SIGUNKNOWN {
            audit_log_format(
                ab,
                b"signal=unknown(%d)\0".as_ptr() as *const c_char,
                (*ad).unmappedsig,
            );
        } else if (*ad).signal < MAXMAPPED_SIGNAME {
            audit_log_format(
                ab,
                b" signal=%s\0".as_ptr() as *const c_char,
                sig_names[(*ad).signal as usize],
            );
        } else {
            audit_log_format(
                ab,
                b" signal=rtmin+%d\0".as_ptr() as *const c_char,
                (*ad).signal - SIGRT_BASE,
            );
        }
        audit_log_format(ab, b" peer=\0".as_ptr() as *const c_char);
        aa_label_xaudit(
            ab,
            labels_ns((*ad).subj_label),
            (*ad).peer,
            FLAGS_NONE,
            GFP_ATOMIC,
        );
    }
}

fn profile_signal_perm(
    cred: *const cred,
    profile: *mut aa_profile,
    peer: *mut aa_label,
    request: u32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let rules: *mut aa_ruleset;
    let mut perms: aa_perms;
    let mut state: aa_state_t;

    unsafe {
        rules = (*profile).label.rules[0];

        if profile_unconfined(profile) {
            return 0;
        }

        (*ad).subj_cred = cred;
        (*ad).peer = peer;
        // TODO: secondary cache check <profile, profile, perm>
        state = if RULE_MEDIATES_SIGNAL(rules) { 1 } else { 0 };
        if state == 0 {
            return 0;
        }
        state = aa_dfa_next((*(*profile).label.rules[0]).policy_dfa, state, (*ad).signal);
        aa_label_match(profile, rules, peer, state, false, request, &mut perms);
        aa_apply_modes_to_perms(profile, &mut perms);
        return aa_check_perms(profile, &perms, request, ad, audit_signal_cb);
    }
}

// Helper function to simulate RULE_MEDIATES macro
#[inline]
fn RULE_MEDIATES_SIGNAL(rules: *mut aa_ruleset) -> bool {
    // Implementation depends on external structures
    // This is a placeholder that returns true when rules mediate AA_CLASS_SIGNAL
    true
}

#[no_mangle]
pub extern "C" fn aa_may_signal(
    subj_cred: *const cred,
    sender: *mut aa_label,
    target_cred: *const cred,
    target: *mut aa_label,
    sig: i32,
) -> i32 {
    let mut ad = unsafe {
        let mut audit_data: apparmor_audit_data = std::mem::zeroed();
        audit_data
    };

    unsafe {
        ad.signal = map_signal_num(sig);
        ad.unmappedsig = sig;
    }

    // Simulate xcheck_labels macro behavior:
    // Check permissions in both directions and combine results
    let check1 = profile_signal_perm(subj_cred, std::ptr::null_mut(), target, MAY_WRITE, &mut ad);
    let check2 = profile_signal_perm(target_cred, std::ptr::null_mut(), sender, MAY_READ, &mut ad);

    // Return combined result (typically OR for permission checks)
    if check1 != 0 { check1 } else { check2 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
