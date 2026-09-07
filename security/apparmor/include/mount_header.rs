// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor file mediation function definitions.
//
// Copyright 2017 Canonical Ltd.

// Dependencies from C includes:
// #include <linux/fs.h>
// #include <linux/path.h>
// #include "domain.h"
// #include "policy.h"

use std::ffi::{c_char, c_int, c_void, c_ulong};

// Opaque types from external headers
pub struct cred;
pub struct aa_label;
pub struct path;
pub struct vfsmount;

// mount perms
pub const AA_MAY_PIVOTROOT: i32 = 0x01;
pub const AA_MAY_MOUNT: i32 = 0x02;
pub const AA_MAY_UMOUNT: i32 = 0x04;
pub const AA_AUDIT_DATA: i32 = 0x40;
pub const AA_MNT_CONT_MATCH: i32 = 0x40;

// AA_MS_IGNORE_MASK = (MS_KERNMOUNT | MS_NOSEC | MS_ACTIVE | MS_BORN)
// Requires external constants from <linux/fs.h>.

extern "C" {
    pub fn aa_remount(
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        flags: c_ulong,
        data: *mut c_void,
    ) -> c_int;

    pub fn aa_bind_mount(
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        old_name: *const c_char,
        flags: c_ulong,
    ) -> c_int;

    pub fn aa_mount_change_type(
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        flags: c_ulong,
    ) -> c_int;

    pub fn aa_move_mount_old(
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        old_name: *const c_char,
    ) -> c_int;

    pub fn aa_move_mount(
        subj_cred: *const cred,
        label: *mut aa_label,
        from_path: *const path,
        to_path: *const path,
    ) -> c_int;

    pub fn aa_new_mount(
        subj_cred: *const cred,
        label: *mut aa_label,
        dev_name: *const c_char,
        path: *const path,
        r#type: *const c_char,
        flags: c_ulong,
        data: *mut c_void,
    ) -> c_int;

    pub fn aa_umount(
        subj_cred: *const cred,
        label: *mut aa_label,
        mnt: *mut vfsmount,
        flags: c_int,
    ) -> c_int;

    pub fn aa_pivotroot(
        subj_cred: *const cred,
        label: *mut aa_label,
        old_path: *const path,
        new_path: *const path,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
