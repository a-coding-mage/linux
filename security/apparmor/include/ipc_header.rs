// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor ipc mediation function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.

// Requires: linux/sched.h types (external dependency)

use std::os::raw::c_int;

pub const SIGUNKNOWN: c_int = 0;
pub const MAXMAPPED_SIG: c_int = 35;

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aa_label {
    _private: [u8; 0],
}

extern "C" {
    pub fn aa_may_signal(
        subj_cred: *const cred,
        sender: *mut aa_label,
        target_cred: *const cred,
        target: *mut aa_label,
        sig: c_int,
    ) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
