// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor ipc mediation
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2017 Canonical Ltd.
 */

use std::ffi::{c_char, c_void};

extern "C" {
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn aa_dfa_next(dfa: *const c_void, state: aa_state_t, c: i32) -> aa_state_t;
    fn aa_label_match(profile: *const aa_profile, rules: *mut aa_ruleset,
                      peer: *mut aa_label, state: aa_state_t, subns: bool,
                      request: u32, perms: *mut aa_perms);
    fn aa_apply_modes_to_perms(profile: *const aa_profile, perms: *mut aa_perms);
    fn aa_check_perms(profile: *const aa_profile, perms: *const aa_perms,
                      request: u32, ad: *mut apparmor_audit_data,
                      audit_cb: extern "C" fn(*mut audit_buffer, *mut c_void) -> ());
    fn aa_label_xaudit(ab: *mut audit_buffer, ns: *const c_void,
                       label: *mut aa_label, flags: u32, gfp: u32);
    fn labels_ns(label: *mut aa_label) -> *const c_void;
    fn rule_mediates(rules: *mut aa_ruleset, class: u32) -> aa_state_t;
    fn xcheck_labels(sender: *mut aa_label, target: *mut aa_label,
                     first: i32, second: i32) -> i32;
    static sig_map: [i32; 32];
    static sig_names: [*const c_char; 32];
}

#[repr(C)] pub struct audit_buffer;
#[repr(C)] pub struct common_audit_data;
#[repr(C)] pub struct cred;
#[repr(C)] pub struct aa_label { pub rules: [*mut aa_ruleset; 1] }
#[repr(C)] pub struct aa_profile { pub label: aa_label }
#[repr(C)] pub struct aa_ruleset { pub policy: *mut aa_policy }
#[repr(C)] pub struct aa_policy { pub dfa: *const c_void }
#[repr(C)] pub struct aa_perms { _private: [u8; 0] }
#[repr(C)] pub struct apparmor_audit_data {
    pub request: u32, pub denied: u32, pub signal: i32, pub unmappedsig: i32,
    pub subj_cred: *const cred, pub peer: *mut aa_label, pub subj_label: *mut aa_label,
}
pub type aa_state_t = u32;

const AA_SIGNAL_PERM_MASK: u32 = 0;
const SIGUNKNOWN: i32 = 0;
const MAXMAPPED_SIG: i32 = 0;
const MAXMAPPED_SIGNAME: i32 = 0;
const SIGRT_BASE: i32 = 0;
const SIGRTMIN: i32 = 0;
const SIGRTMAX: i32 = 0;
const MAY_READ: u32 = 0;
const MAY_WRITE: u32 = 0;
const AA_CLASS_SIGNAL: u32 = 0;
const OP_SIGNAL: u32 = 0;
const LSM_AUDIT_DATA_NONE: u32 = 0;
const FLAGS_NONE: u32 = 0;
const GFP_ATOMIC: u32 = 0;

#[inline]
fn map_signal_num(sig: i32) -> i32 {
    if sig > SIGRTMAX { SIGUNKNOWN }
    else if sig >= SIGRTMIN { sig - SIGRTMIN + SIGRT_BASE }
    else if sig < MAXMAPPED_SIG { unsafe { sig_map[sig as usize] } }
    else { SIGUNKNOWN }
}

fn audit_signal_mask(mask: u32) -> &'static [u8] {
    if mask & MAY_READ != 0 { b"receive\0" }
    else if mask & MAY_WRITE != 0 { b"send\0" }
    else { b"\0" }
}

extern "C" fn audit_signal_cb(ab: *mut audit_buffer, va: *mut c_void) {
    unsafe {
        let ad = aad(va as *mut common_audit_data);
        if (*ad).request & AA_SIGNAL_PERM_MASK != 0 {
            audit_log_format(ab, b" requested_mask=\"%s\"\0".as_ptr() as *const c_char,
                              audit_signal_mask((*ad).request).as_ptr());
            if (*ad).denied & AA_SIGNAL_PERM_MASK != 0 {
                audit_log_format(ab, b" denied_mask=\"%s\"\0".as_ptr() as *const c_char,
                                  audit_signal_mask((*ad).denied).as_ptr());
            }
        }
        if (*ad).signal == SIGUNKNOWN {
            audit_log_format(ab, b"signal=unknown(%d)\0".as_ptr() as *const c_char, (*ad).unmappedsig);
        } else if (*ad).signal < MAXMAPPED_SIGNAME {
            audit_log_format(ab, b" signal=%s\0".as_ptr() as *const c_char,
                              sig_names[(*ad).signal as usize]);
        } else {
            audit_log_format(ab, b" signal=rtmin+%d\0".as_ptr() as *const c_char,
                              (*ad).signal - SIGRT_BASE);
        }
        audit_log_format(ab, b" peer=\0".as_ptr() as *const c_char);
        aa_label_xaudit(ab, labels_ns((*ad).subj_label), (*ad).peer, FLAGS_NONE, GFP_ATOMIC);
    }
}

unsafe fn profile_signal_perm(cred: *const cred, profile: *mut aa_profile,
                              peer: *mut aa_label, request: u32,
                              ad: *mut apparmor_audit_data) -> i32 {
    if profile_unconfined(profile) { return 0; }
    let rules = (*profile).label.rules[0];
    (*ad).subj_cred = cred;
    (*ad).peer = peer;
    let mut state = rule_mediates(rules, AA_CLASS_SIGNAL);
    if state == 0 { return 0; }
    state = aa_dfa_next((*(*rules).policy).dfa, state, (*ad).signal);
    let mut perms = std::mem::zeroed::<aa_perms>();
    aa_label_match(profile, rules, peer, state, false, request, &mut perms);
    aa_apply_modes_to_perms(profile, &mut perms);
    aa_check_perms(profile, &perms, request, ad, audit_signal_cb)
}

pub unsafe extern "C" fn aa_may_signal(subj_cred: *const cred, sender: *mut aa_label,
                                        target_cred: *const cred, target: *mut aa_label,
                                        sig: i32) -> i32 {
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    ad.signal = map_signal_num(sig);
    ad.unmappedsig = sig;
    let profile = (*sender).rules[0] as *mut aa_profile;
    let target_profile = (*target).rules[0] as *mut aa_profile;
    xcheck_labels(sender, target,
        profile_signal_perm(subj_cred, profile, target, MAY_WRITE, &mut ad),
        profile_signal_perm(target_cred, target_profile, sender, MAY_READ, &mut ad))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
