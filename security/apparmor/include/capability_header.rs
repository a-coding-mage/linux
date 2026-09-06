// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor capability mediation definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2013 Canonical Ltd.

// Depends on: <linux/capability.h>, <linux/sched.h>, apparmorfs.h

// Forward declarations
pub struct aa_label;
pub struct aa_profile;
pub struct cred;
pub struct aa_sfs_entry;

// aa_caps - confinement data for capabilities
// allow: capabilities mask
// audit: caps that are to be audited
// denied: caps that are explicitly denied
// quiet: caps that should not be audited
// kill: caps that when requested will result in the task being killed
// extended: caps that are subject finer grained mediation
#[repr(C)]
pub struct aa_caps {
    pub allow: kernel_cap_t,
    pub audit: kernel_cap_t,
    pub denied: kernel_cap_t,
    pub quiet: kernel_cap_t,
    pub kill: kernel_cap_t,
    pub extended: kernel_cap_t,
}

extern "C" {
    pub static aa_sfs_entry_caps: [aa_sfs_entry; 0];

    pub fn aa_profile_capget(profile: *const aa_profile) -> kernel_cap_t;

    pub fn aa_capable(
        subj_cred: *const cred,
        label: *mut aa_label,
        cap: i32,
        opts: u32,
    ) -> i32;
}

#[inline]
pub fn aa_free_cap_rules(_caps: *mut aa_caps) {
    // NOP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
