// SPDX-License-Identifier: GPL-2.0-only
// AppArmor security module
//
// This file contains AppArmor file mediation function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// External dependencies: linux/spinlock.h, domain.h, match.h, perms.h

// Forward declarations - defined in external modules
// struct aa_policydb;
// struct aa_profile;
// struct path;

// Types from external headers (linux/spinlock.h, perms.h, etc.)
// These are assumed to be available from linked C code
type spinlock_t = u32; // Opaque kernel type
type kuid_t = u32; // Kernel uid type
type umode_t = u16; // Kernel mode type
type aa_state_t = u32; // State type
type aa_label = std::ffi::c_void; // Opaque type
type aa_policydb = std::ffi::c_void; // Opaque type
type aa_profile = std::ffi::c_void; // Opaque type
type aa_perms = std::ffi::c_void; // Opaque type
type cred = std::ffi::c_void; // Opaque type
type path = std::ffi::c_void; // Opaque type
type file = std::ffi::c_void; // Opaque type
type dentry = std::ffi::c_void; // Opaque type
type files_struct = std::ffi::c_void; // Opaque type

// Constants from external headers (assumed available)
// Values for file mode flags
const FMODE_WRITE: u32 = 0x2;
const FMODE_READ: u32 = 0x1;
const O_APPEND: i32 = 0x0800;
const O_TRUNC: i32 = 0x0200;
const O_CREAT: i32 = 0x0040;

// Permission constants (from perms.h)
const MAY_EXEC: u32 = 0x1;
const MAY_WRITE: u32 = 0x2;
const MAY_READ: u32 = 0x4;
const MAY_APPEND: u32 = 0x8;
const AA_MAY_CREATE: u32 = 0x10;
const AA_MAY_DELETE: u32 = 0x20;
const AA_MAY_GETATTR: u32 = 0x40;
const AA_MAY_SETATTR: u32 = 0x80;
const AA_MAY_CHMOD: u32 = 0x100;
const AA_MAY_CHOWN: u32 = 0x200;
const AA_MAY_LOCK: u32 = 0x400;
const AA_EXEC_MMAP: u32 = 0x800;
const AA_MAY_LINK: u32 = 0x1000;
const AA_INDEX_MASK: u32 = 0x03ffffff;
const AA_INDEX_NONE: u32 = 0x0;

// External struct: apparmor_blob_sizes
// Assumed to be available from external module
extern "C" {
    pub static apparmor_blob_sizes: AppArmorBlobSizes;
}

#[repr(C)]
pub struct AppArmorBlobSizes {
    pub lbs_file: usize,
}

/// Mask mode bits to extract only file access permission bits
#[inline]
pub const fn mask_mode_t(x: u32) -> u32 {
    x & (MAY_EXEC | MAY_WRITE | MAY_READ | MAY_APPEND)
}

/// Audit mask for file operations - all auditable permission bits
pub const AA_AUDIT_FILE_MASK: u32 = MAY_READ | MAY_WRITE | MAY_EXEC | MAY_APPEND |
                                     AA_MAY_CREATE | AA_MAY_DELETE |
                                     AA_MAY_GETATTR | AA_MAY_SETATTR |
                                     AA_MAY_CHMOD | AA_MAY_CHOWN | AA_MAY_LOCK |
                                     AA_EXEC_MMAP | AA_MAY_LINK;

/// Get the AppArmor file context from a file struct
/// # Safety
/// Requires valid file pointer and proper initialization of apparmor_blob_sizes
#[inline]
pub unsafe fn file_ctx(file: *mut file) -> *mut aa_file_ctx {
    ((*file as *mut u8).add(unsafe { apparmor_blob_sizes.lbs_file })) as *mut aa_file_ctx
}

/// struct aa_file_ctx - the AppArmor context the file was opened in
/// @lock: lock to update the ctx
/// @label: label currently cached on the ctx
/// @allow: the permission the file was opened with
#[repr(C)]
pub struct aa_file_ctx {
    pub lock: spinlock_t,
    pub label: *mut aa_label,  // __rcu annotation in C
    pub allow: u32,
}

// The xindex is broken into 3 parts:
// - index - an index into either the exec name table or the variable table
// - exec type - which determines how the executable name and index are used
// - flags - which modify how the destination name is applied

pub const AA_X_INDEX_MASK: u32 = AA_INDEX_MASK;

pub const AA_X_TYPE_MASK: u32 = 0x0c000000;
pub const AA_X_NONE: u32 = AA_INDEX_NONE;
pub const AA_X_NAME: u32 = 0x04000000;  // use executable name px
pub const AA_X_TABLE: u32 = 0x08000000; // use a specified name ->n#

pub const AA_X_UNSAFE: u32 = 0x10000000;
pub const AA_X_CHILD: u32 = 0x20000000;
pub const AA_X_INHERIT: u32 = 0x40000000;
pub const AA_X_UNCONFINED: u32 = 0x80000000;

/// struct path_cond - conditional context for path operations
#[repr(C)]
pub struct path_cond {
    pub uid: kuid_t,
    pub mode: umode_t,
}

/// Combined permission mask from all permission categories
#[inline]
pub fn combined_perm_mask(x: &aa_perms) -> u32 {
    // Note: This assumes aa_perms has allow, audit, quiet, kill fields
    // The exact field access requires the full aa_perms definition
    unsafe {
        let ptr = x as *const aa_perms as *const u32;
        let allow = *ptr;
        let audit = *ptr.add(1);
        let quiet = *ptr.add(2);
        let kill = *ptr.add(3);
        allow | audit | quiet | kill
    }
}

extern "C" {
    /// Audit file operation
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

    /// Lookup conditional permissions based on uid and path condition
    pub fn aa_lookup_condperms(
        subj_uid: kuid_t,
        file_rules: *mut aa_policydb,
        state: aa_state_t,
        cond: *mut path_cond,
    ) -> *mut aa_perms;

    /// Lookup string permission state
    pub fn aa_str_perms(
        file_rules: *mut aa_policydb,
        start: aa_state_t,
        name: *const i8,
        cond: *mut path_cond,
        perms: *mut aa_perms,
    ) -> aa_state_t;

    /// Check path permission (internal)
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

    /// Check path permission
    pub fn aa_path_perm(
        op: *const i8,
        subj_cred: *const cred,
        label: *mut aa_label,
        path: *const path,
        flags: i32,
        request: u32,
        cond: *mut path_cond,
    ) -> i32;

    /// Check path link permission
    pub fn aa_path_link(
        subj_cred: *const cred,
        label: *mut aa_label,
        old_dentry: *mut dentry,
        new_dir: *const path,
        new_dentry: *mut dentry,
    ) -> i32;

    /// Check file permission
    pub fn aa_file_perm(
        op: *const i8,
        subj_cred: *const cred,
        label: *mut aa_label,
        file: *mut file,
        request: u32,
        in_atomic: bool,
    ) -> i32;

    /// Inherit files for credential
    pub fn aa_inherit_files(cred: *const cred, files: *mut files_struct);
}

/// aa_map_file_to_perms - map file flags to AppArmor permissions
/// Maps the file open flags and mode to the corresponding AppArmor permission bits
///
/// # Arguments
/// * `file` - open file to map flags to AppArmor permissions
///
/// # Returns
/// apparmor permission set for the file
///
/// # Safety
/// Requires valid file pointer
#[inline]
pub unsafe fn aa_map_file_to_perms(file: *const file) -> u32 {
    let flags = (*(file as *const std::ffi::c_void as *const FileFFI)).f_flags;
    let f_mode = (*(file as *const std::ffi::c_void as *const FileFFI)).f_mode;
    let mut perms: u32 = 0;

    if f_mode & FMODE_WRITE != 0 {
        perms |= MAY_WRITE;
    }
    if f_mode & FMODE_READ != 0 {
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

// Minimal file struct representation for accessing f_flags and f_mode
#[repr(C)]
struct FileFFI {
    f_flags: i32,
    f_mode: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
