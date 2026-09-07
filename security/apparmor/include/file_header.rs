// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor file mediation function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// External dependencies: linux/spinlock.h, domain.h, match.h, perms.h

// Forward declarations supplied by external modules:
// struct aa_policydb;
// struct aa_profile;
// struct path;

#[inline]
pub const fn mask_mode_t(x: u32) -> u32 {
    x & (MAY_EXEC | MAY_WRITE | MAY_READ | MAY_APPEND)
}

pub const AA_AUDIT_FILE_MASK: u32 = MAY_READ | MAY_WRITE | MAY_EXEC | MAY_APPEND |
    AA_MAY_CREATE | AA_MAY_DELETE |
    AA_MAY_GETATTR | AA_MAY_SETATTR |
    AA_MAY_CHMOD | AA_MAY_CHOWN | AA_MAY_LOCK |
    AA_EXEC_MMAP | AA_MAY_LINK;

#[inline]
pub unsafe fn file_ctx(file: *mut file) -> *mut aa_file_ctx {
    (*file).f_security.add(apparmor_blob_sizes.lbs_file) as *mut aa_file_ctx
}

// struct aa_file_ctx - the AppArmor context the file was opened in
// @lock: lock to update the ctx
// @label: label currently cached on the ctx
// @perms: the permission the file was opened with
#[repr(C)]
pub struct aa_file_ctx {
    pub lock: spinlock_t,
    pub label: *mut aa_label,
    pub allow: u32,
}

// The xindex is broken into 3 parts:
// - index - an index into either the exec name table or the variable table
// - exec type - which determines how the executable name and index are used
// - flags - which modify how the destination name is applied

pub const AA_X_INDEX_MASK: u32 = AA_INDEX_MASK;

pub const AA_X_TYPE_MASK: u32 = 0x0c000000;
pub const AA_X_NONE: u32 = AA_INDEX_NONE;
pub const AA_X_NAME: u32 = 0x04000000; // use executable name px
pub const AA_X_TABLE: u32 = 0x08000000; // use a specified name ->n#

pub const AA_X_UNSAFE: u32 = 0x10000000;
pub const AA_X_CHILD: u32 = 0x20000000;
pub const AA_X_INHERIT: u32 = 0x40000000;
pub const AA_X_UNCONFINED: u32 = 0x80000000;

// need to make conditional which ones are being set
#[repr(C)]
pub struct path_cond {
    pub uid: kuid_t,
    pub mode: umode_t,
}

#[inline]
pub const fn COMBINED_PERM_MASK(x: aa_perms) -> u32 {
    x.allow | x.audit | x.quiet | x.kill
}

extern "C" {
    pub fn aa_audit_file(
        cred: *const cred,
        profile: *mut aa_profile,
        perms: *const aa_perms,
        op: *const i8,
        request: u32,
        name: *const i8,
        target: *const i8,
        tlabel: *mut aa_label,
        ouid: kuid_t,
        info: *const i8,
        error: i32,
    ) -> i32;

    pub fn aa_lookup_condperms(
        subj_uid: kuid_t,
        file_rules: *mut aa_policydb,
        state: aa_state_t,
        cond: *mut path_cond,
    ) -> *mut aa_perms;

    pub fn aa_str_perms(
        file_rules: *mut aa_policydb,
        start: aa_state_t,
        name: *const i8,
        cond: *mut path_cond,
        perms: *mut aa_perms,
    ) -> aa_state_t;

    pub fn __aa_path_perm(
        op: *const i8,
        subj_cred: *const cred,
        profile: *mut aa_profile,
        name: *const i8,
        request: u32,
        cond: *mut path_cond,
        flags: i32,
        perms: *mut aa_perms,
    ) -> i32;

    pub fn aa_path_perm(
        op: *const i8,
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        flags: i32,
        request: u32,
        cond: *mut path_cond,
    ) -> i32;

    pub fn aa_path_link(
        subj_cred: *const cred,
        label: *mut aa_label,
        old_dentry: *mut dentry,
        new_dir: *const path,
        new_dentry: *mut dentry,
    ) -> i32;

    pub fn aa_file_perm(
        op: *const i8,
        subj_cred: *const cred,
        label: *mut aa_label,
        file: *mut file,
        request: u32,
        in_atomic: bool,
    ) -> i32;

    pub fn aa_inherit_files(cred: *const cred, files: *mut files_struct);
}

// aa_map_file_to_perms - map file flags to AppArmor permissions
// @file: open file to map flags to AppArmor permissions
//
// Returns: apparmor permission set for the file
#[inline]
pub unsafe fn aa_map_file_to_perms(file: *mut file) -> u32 {
    let flags = (*file).f_flags;
    let mut perms = 0u32;

    if (*file).f_mode & FMODE_WRITE != 0 {
        perms |= MAY_WRITE;
    }
    if (*file).f_mode & FMODE_READ != 0 {
        perms |= MAY_READ;
    }

    if (flags & O_APPEND) != 0 && (perms & MAY_WRITE) != 0 {
        perms = (perms & !MAY_WRITE) | MAY_APPEND;
    }
    // trunc implies write permission
    if (flags & O_TRUNC) != 0 {
        perms |= MAY_WRITE;
    }
    if (flags & O_CREAT) != 0 {
        perms |= AA_MAY_CREATE;
    }

    perms
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
