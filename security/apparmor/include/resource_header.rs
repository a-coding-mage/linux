// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor resource limits function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// External dependencies from <linux/resource.h>, <linux/sched.h>, and "apparmorfs.h"
use std::ffi::c_uint;

// Forward declarations of external types
// struct aa_profile (defined elsewhere)
#[repr(C)]
pub struct aa_profile;

#[repr(C)]
pub struct cred;
#[repr(C)]
pub struct aa_label;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct aa_sfs_entry;
#[repr(C)]
pub struct rlimit;

// struct aa_rlimit - rlimit settings for the profile
// @mask: which hard limits to set
// @limits: rlimit values that override task limits
//
// AppArmor rlimits are used to set confined task rlimits.  Only the
// limits specified in @mask will be controlled by apparmor.
#[repr(C)]
pub struct aa_rlimit {
    pub mask: c_uint,
    pub limits: [rlimit; RLIM_NLIMITS],
}

extern "C" {
    pub static aa_sfs_entry_rlimit: [aa_sfs_entry; 0];

    pub fn aa_map_resource(resource: i32) -> i32;

    pub fn aa_task_setrlimit(
        subj_cred: *const cred,
        label: *mut aa_label,
        task: *mut task_struct,
        resource: c_uint,
        new_rlim: *mut rlimit,
    ) -> i32;

    pub fn __aa_transition_rlimits(old: *mut aa_label, new: *mut aa_label);
}

#[inline]
pub fn aa_free_rlimit_rules(_rlims: *mut aa_rlimit) {
    // NOP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
