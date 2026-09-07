// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor security domain transition function definitions.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

pub const AA_CHANGE_NOFLAGS: u32 = 0;
pub const AA_CHANGE_TEST: u32 = 1;
pub const AA_CHANGE_CHILD: u32 = 2;
pub const AA_CHANGE_ONEXEC: u32 = 4;
pub const AA_CHANGE_STACK: u32 = 8;

// Opaque types from external headers (linux/binfmts.h and label.h)
#[repr(C)]
pub struct aa_label {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct aa_profile {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct linux_binprm {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn x_table_lookup(
        profile: *mut aa_profile,
        xindex: u32,
        name: *mut *const i8,
    ) -> *mut aa_label;

    pub fn apparmor_bprm_creds_for_exec(bprm: *mut linux_binprm) -> i32;

    pub fn aa_change_hat(
        hats: *const *const i8,
        count: i32,
        token: u64,
        flags: i32,
    ) -> i32;

    pub fn aa_change_profile(fqname: *const i8, flags: i32) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
