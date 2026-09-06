// SPDX-License-Identifier: GPL-2.0-only
// AppArmor security module
//
// This file contains AppArmor network mediation definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.

// Dependencies: net/sock.h, linux/path.h, apparmorfs.h, label.h, perms.h, policy.h

use std::ffi::c_char;

// Type definitions from external dependencies (assumed to be defined elsewhere)
// pub type sock = std::ffi::c_void;
// pub type cred = std::ffi::c_void;
// pub type aa_label = std::ffi::c_void;
// pub type aa_profile = std::ffi::c_void;
// pub type aa_policydb = std::ffi::c_void;
// pub type aa_perms = std::ffi::c_void;
// pub type apparmor_audit_data = std::ffi::c_void;
// pub type audit_buffer = std::ffi::c_void;
// pub type file = std::ffi::c_void;
// pub type aa_state_t = u32;

pub const AA_MAY_SEND: u32 = AA_MAY_WRITE;
pub const AA_MAY_RECEIVE: u32 = AA_MAY_READ;

pub const AA_MAY_SHUTDOWN: u32 = AA_MAY_DELETE;

pub const AA_MAY_CONNECT: u32 = AA_MAY_OPEN;
pub const AA_MAY_ACCEPT: u32 = 0x00100000;

pub const AA_MAY_BIND: u32 = 0x00200000;
pub const AA_MAY_LISTEN: u32 = 0x00400000;

pub const AA_MAY_SETOPT: u32 = 0x01000000;
pub const AA_MAY_GETOPT: u32 = 0x02000000;

pub const NET_PERMS_MASK: u32 = AA_MAY_SEND | AA_MAY_RECEIVE | AA_MAY_CREATE |
                                AA_MAY_SHUTDOWN | AA_MAY_BIND | AA_MAY_LISTEN |
                                AA_MAY_CONNECT | AA_MAY_ACCEPT | AA_MAY_SETATTR |
                                AA_MAY_GETATTR | AA_MAY_SETOPT | AA_MAY_GETOPT;

pub const NET_FS_PERMS: u32 = AA_MAY_SEND | AA_MAY_RECEIVE | AA_MAY_CREATE |
                               AA_MAY_SHUTDOWN | AA_MAY_CONNECT | AA_MAY_RENAME |
                               AA_MAY_SETATTR | AA_MAY_GETATTR | AA_MAY_CHMOD |
                               AA_MAY_CHOWN | AA_MAY_CHGRP | AA_MAY_LOCK |
                               AA_MAY_MPROT;

pub const NET_PEER_MASK: u32 = AA_MAY_SEND | AA_MAY_RECEIVE | AA_MAY_CONNECT |
                                AA_MAY_ACCEPT;

// __rcu is a Linux kernel annotation for RCU-protected pointers
// Represented as raw pointers in Rust
#[repr(C)]
pub struct aa_sk_ctx {
    pub label: *mut aa_label,      // __rcu annotated in original
    pub peer: *mut aa_label,       // __rcu annotated in original
    pub peer_lastupdate: *mut aa_label,  // __rcu annotated; ptr cmp only, no deref
}

#[inline]
pub unsafe fn aa_sock(sk: *const sock) -> *mut aa_sk_ctx {
    ((*sk).sk_security as usize + apparmor_blob_sizes.lbs_sock) as *mut aa_sk_ctx
}

// DEFINE_AUDIT_NET macro - translated as a helper function conceptually
// Original macro definition preserved in comment form:
// #define DEFINE_AUDIT_NET(NAME, OP, CRED, SK, F, T, P)
//   struct lsm_network_audit NAME ## _net = { .sk = (SK), .family = (F)};
//   DEFINE_AUDIT_DATA(NAME,
//                     ((SK) && (F) != AF_UNIX) ? LSM_AUDIT_DATA_NET : LSM_AUDIT_DATA_NONE,
//                     AA_CLASS_NET,
//                     OP);
//   NAME.common.u.net = &(NAME ## _net);
//   NAME.subj_cred = (CRED);
//   NAME.net.type = (T);
//   NAME.net.protocol = (P)
// Note: This is a complex macro with multiple side effects. Implement as needed in calling code.

// DEFINE_AUDIT_SK macro - relies on DEFINE_AUDIT_NET
// #define DEFINE_AUDIT_SK(NAME, OP, CRED, SK)
//   DEFINE_AUDIT_NET(NAME, OP, CRED, SK, (SK)->sk_family, (SK)->sk_type, (SK)->sk_protocol)
// Note: Helper macro that expands DEFINE_AUDIT_NET with socket fields. Implement as needed.

#[repr(C)]
pub struct aa_secmark {
    pub audit: u8,
    pub deny: u8,
    pub secid: u32,
    pub label: *mut c_char,
}

extern "C" {
    pub static mut aa_sfs_entry_network: aa_sfs_entry;
    pub static mut aa_sfs_entry_networkv9: aa_sfs_entry;

    pub fn aa_do_perms(
        profile: *mut aa_profile,
        policy: *mut aa_policydb,
        state: aa_state_t,
        request: u32,
        p: *const aa_perms,
        ad: *mut apparmor_audit_data,
    ) -> i32;

    // passing in state returned by XXX_mediates_AF()
    pub fn aa_match_to_prot(
        policy: *mut aa_policydb,
        state: aa_state_t,
        request: u32,
        af: u16,
        r#type: i32,
        protocol: i32,
        p: *mut *const aa_perms,
        info: *mut *const c_char,
    ) -> aa_state_t;

    pub fn audit_net_cb(ab: *mut audit_buffer, va: *mut std::ffi::c_void);

    pub fn aa_profile_af_perm(
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        request: u32,
        family: u16,
        r#type: i32,
        protocol: i32,
    ) -> i32;

    pub fn aa_af_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const c_char,
        request: u32,
        family: u16,
        r#type: i32,
        protocol: i32,
    ) -> i32;

    pub fn aa_sk_perm(op: *const c_char, request: u32, sk: *const sock) -> i32;

    pub fn aa_label_sk_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const c_char,
        request: u32,
        sk: *const sock,
    ) -> i32;

    pub fn aa_sock_file_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const c_char,
        request: u32,
        file: *mut file,
    ) -> i32;

    pub fn apparmor_secmark_check(
        label: *mut aa_label,
        op: *mut c_char,
        request: u32,
        secid: u32,
        sk: *const sock,
    ) -> i32;
}

#[inline]
pub unsafe fn aa_profile_af_sk_perm(
    profile: *mut aa_profile,
    ad: *mut apparmor_audit_data,
    request: u32,
    sk: *const sock,
) -> i32 {
    aa_profile_af_perm(
        profile,
        ad,
        request,
        (*sk).sk_family,
        (*sk).sk_type,
        (*sk).sk_protocol,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
