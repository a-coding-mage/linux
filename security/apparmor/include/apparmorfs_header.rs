// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor filesystem definitions.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

// Requires: <linux/init.h>, <linux/types.h>

use std::ffi::c_char;
use std::os::raw::c_int;

// External type aliases for Linux kernel types
// These will be provided by actual kernel headers
pub type path = crate::linux::path;
pub type dentry = crate::linux::dentry;
pub type file_operations = crate::linux::file_operations;
pub type umode_t = u16;

extern "C" {
    pub static aa_null: path;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aa_sfs_type {
    AA_SFS_TYPE_BOOLEAN = 0,
    AA_SFS_TYPE_STRING = 1,
    AA_SFS_TYPE_U64 = 2,
    AA_SFS_TYPE_FOPS = 3,
    AA_SFS_TYPE_DIR = 4,
}

// Forward declarations
pub struct aa_sfs_entry;
pub struct aa_profile;
pub struct aa_ns;
pub struct aa_loaddata;

#[repr(C)]
pub union AaSfsEntryValue {
    pub boolean: bool,
    pub string: *mut c_char,
    pub u64: std::os::raw::c_ulong,
    pub files: *mut aa_sfs_entry,
}

#[repr(C)]
pub struct aa_sfs_entry {
    pub name: *const c_char,
    pub dentry: *mut dentry,
    pub mode: umode_t,
    pub v_type: aa_sfs_type,
    pub v: AaSfsEntryValue,
    pub file_ops: *const file_operations,
}

extern "C" {
    pub static aa_sfs_seq_file_ops: file_operations;
}

// Helper macros for creating aa_sfs_entry instances
#[macro_export]
macro_rules! AA_SFS_FILE_BOOLEAN {
    ($name:expr, $value:expr) => {
        $crate::aa_sfs_entry {
            name: $name,
            mode: 0o444,
            v_type: $crate::aa_sfs_type::AA_SFS_TYPE_BOOLEAN,
            v: $crate::AaSfsEntryValue { boolean: $value },
            file_ops: &$crate::aa_sfs_seq_file_ops,
            dentry: core::ptr::null_mut(),
        }
    };
}

#[macro_export]
macro_rules! AA_SFS_FILE_STRING {
    ($name:expr, $value:expr) => {
        $crate::aa_sfs_entry {
            name: $name,
            mode: 0o444,
            v_type: $crate::aa_sfs_type::AA_SFS_TYPE_STRING,
            v: $crate::AaSfsEntryValue { string: $value },
            file_ops: &$crate::aa_sfs_seq_file_ops,
            dentry: core::ptr::null_mut(),
        }
    };
}

#[macro_export]
macro_rules! AA_SFS_FILE_U64 {
    ($name:expr, $value:expr) => {
        $crate::aa_sfs_entry {
            name: $name,
            mode: 0o444,
            v_type: $crate::aa_sfs_type::AA_SFS_TYPE_U64,
            v: $crate::AaSfsEntryValue { u64: $value },
            file_ops: &$crate::aa_sfs_seq_file_ops,
            dentry: core::ptr::null_mut(),
        }
    };
}

#[macro_export]
macro_rules! AA_SFS_FILE_FOPS {
    ($name:expr, $mode:expr, $fops:expr) => {
        $crate::aa_sfs_entry {
            name: $name,
            v_type: $crate::aa_sfs_type::AA_SFS_TYPE_FOPS,
            mode: $mode,
            file_ops: $fops,
            dentry: core::ptr::null_mut(),
            v: $crate::AaSfsEntryValue {
                files: core::ptr::null_mut(),
            },
        }
    };
}

#[macro_export]
macro_rules! AA_SFS_DIR {
    ($name:expr, $value:expr) => {
        $crate::aa_sfs_entry {
            name: $name,
            v_type: $crate::aa_sfs_type::AA_SFS_TYPE_DIR,
            v: $crate::AaSfsEntryValue { files: $value },
            mode: 0,
            dentry: core::ptr::null_mut(),
            file_ops: core::ptr::null(),
        }
    };
}

extern "C" {
    pub fn aa_destroy_aafs();
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aafs_ns_type {
    AAFS_NS_DIR = 0,
    AAFS_NS_PROFS = 1,
    AAFS_NS_NS = 2,
    AAFS_NS_RAW_DATA = 3,
    AAFS_NS_LOAD = 4,
    AAFS_NS_REPLACE = 5,
    AAFS_NS_REMOVE = 6,
    AAFS_NS_REVISION = 7,
    AAFS_NS_COUNT = 8,
    AAFS_NS_MAX_COUNT = 9,
    AAFS_NS_SIZE = 10,
    AAFS_NS_MAX_SIZE = 11,
    AAFS_NS_OWNER = 12,
    AAFS_NS_SIZEOF = 13,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum aafs_prof_type {
    AAFS_PROF_DIR = 0,
    AAFS_PROF_PROFS = 1,
    AAFS_PROF_NAME = 2,
    AAFS_PROF_MODE = 3,
    AAFS_PROF_ATTACH = 4,
    AAFS_PROF_HASH = 5,
    AAFS_PROF_RAW_DATA = 6,
    AAFS_PROF_RAW_HASH = 7,
    AAFS_PROF_RAW_ABI = 8,
    AAFS_PROF_SIZEOF = 9,
}

// Accessor macros for namespace dentry array
#[macro_export]
macro_rules! ns_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_DIR as usize]
    };
}

#[macro_export]
macro_rules! ns_subns_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_NS as usize]
    };
}

#[macro_export]
macro_rules! ns_subprofs_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_PROFS as usize]
    };
}

#[macro_export]
macro_rules! ns_subdata_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_RAW_DATA as usize]
    };
}

#[macro_export]
macro_rules! ns_subload {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_LOAD as usize]
    };
}

#[macro_export]
macro_rules! ns_subreplace {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_REPLACE as usize]
    };
}

#[macro_export]
macro_rules! ns_subremove {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_REMOVE as usize]
    };
}

#[macro_export]
macro_rules! ns_subrevision {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_ns_type::AAFS_NS_REVISION as usize]
    };
}

// Accessor macros for profile dentry array
#[macro_export]
macro_rules! prof_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_prof_type::AAFS_PROF_DIR as usize]
    };
}

#[macro_export]
macro_rules! prof_child_dir {
    ($x:expr) => {
        (*$x).dents[$crate::aafs_prof_type::AAFS_PROF_PROFS as usize]
    };
}

extern "C" {
    pub fn aa_create_aafs() -> c_int;

    pub fn __aa_bump_ns_revision(ns: *mut aa_ns);

    pub fn __aafs_profile_rmdir(profile: *mut aa_profile);

    pub fn __aafs_profile_migrate_dents(old: *mut aa_profile, new: *mut aa_profile);

    pub fn __aafs_profile_mkdir(
        profile: *mut aa_profile,
        parent: *mut dentry,
    ) -> c_int;

    pub fn __aafs_ns_rmdir(ns: *mut aa_ns);

    pub fn __aafs_ns_mkdir(
        ns: *mut aa_ns,
        parent: *mut dentry,
        name: *const c_char,
        dent: *mut dentry,
    ) -> c_int;
}

#[cfg(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY")]
extern "C" {
    pub fn __aa_fs_remove_rawdata(rawdata: *mut aa_loaddata);

    pub fn __aa_fs_create_rawdata(ns: *mut aa_ns, rawdata: *mut aa_loaddata) -> c_int;

    pub fn __aa_remove_rawdata_symlink_dents(profile: *mut aa_profile);

    pub fn __aa_create_rawdata_symlink_dents(profile: *mut aa_profile) -> c_int;
}

#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY"))]
pub mod _stubs {
    use super::*;

    pub fn __aa_fs_remove_rawdata(_rawdata: *mut aa_loaddata) {
        // empty stub
    }

    pub fn __aa_fs_create_rawdata(_ns: *mut aa_ns, _rawdata: *mut aa_loaddata) -> c_int {
        0
    }

    pub fn __aa_remove_rawdata_symlink_dents(_profile: *mut aa_profile) {
        // empty stub
    }

    pub fn __aa_create_rawdata_symlink_dents(_profile: *mut aa_profile) -> c_int {
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
