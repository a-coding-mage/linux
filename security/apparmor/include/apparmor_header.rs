// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor basic global
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2017 Canonical Ltd.
 */

// Requires: linux/types.h

// Class of mediation types in the AppArmor policy db
pub const AA_CLASS_NONE: i32 = 0;
pub const AA_CLASS_UNKNOWN: i32 = 1;
pub const AA_CLASS_FILE: i32 = 2;
pub const AA_CLASS_CAP: i32 = 3;
pub const AA_CLASS_DEPRECATED: i32 = 4;
pub const AA_CLASS_RLIMITS: i32 = 5;
pub const AA_CLASS_DOMAIN: i32 = 6;
pub const AA_CLASS_MOUNT: i32 = 7;
pub const AA_CLASS_PTRACE: i32 = 9;
pub const AA_CLASS_SIGNAL: i32 = 10;
pub const AA_CLASS_XMATCH: i32 = 11;
pub const AA_CLASS_NET: i32 = 14;
pub const AA_CLASS_NETV9: i32 = 15;
pub const AA_CLASS_LABEL: i32 = 16;
pub const AA_CLASS_POSIX_MQUEUE: i32 = 17;
pub const AA_CLASS_MODULE: i32 = 19;
pub const AA_CLASS_DISPLAY_LSM: i32 = 20;
pub const AA_CLASS_NS: i32 = 21;
pub const AA_CLASS_IO_URING: i32 = 22;

pub const AA_CLASS_NETV9_SKB: i32 = 30;
pub const AA_CLASS_X: i32 = 31;
pub const AA_CLASS_DBUS: i32 = 32;

// NOTE: if AA_CLASS_LAST > 63 need to update label->mediates
pub const AA_CLASS_LAST: i32 = AA_CLASS_DBUS;

// Control parameters settable through module/boot flags
pub type audit_mode = i32;

extern "C" {
    pub static aa_g_audit: audit_mode;
    pub static aa_g_audit_header: bool;
    pub static aa_g_debug: i32;
    pub static aa_g_hash_policy: bool;
    pub static aa_g_export_binary: bool;
    pub static aa_g_rawdata_compression_level: i32;
    pub static aa_g_lock_policy: bool;
    pub static aa_g_logsyscall: bool;
    pub static aa_g_paranoid_load: bool;
    pub static aa_g_path_max: u32;
}

// Conditional: CONFIG_SECURITY_APPARMOR_EXPORT_BINARY
// When defined: AA_MIN_CLEVEL = zstd_min_clevel()
//              AA_MAX_CLEVEL = zstd_max_clevel()
//              AA_DEFAULT_CLEVEL = ZSTD_CLEVEL_DEFAULT
#[cfg(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY")]
extern "C" {
    pub fn zstd_min_clevel() -> i32;
    pub fn zstd_max_clevel() -> i32;
    pub static ZSTD_CLEVEL_DEFAULT: i32;
}

#[cfg(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY")]
#[inline]
pub fn aa_min_clevel() -> i32 {
    unsafe { zstd_min_clevel() }
}

#[cfg(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY")]
#[inline]
pub fn aa_max_clevel() -> i32 {
    unsafe { zstd_max_clevel() }
}

#[cfg(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY")]
#[inline]
pub fn aa_default_clevel() -> i32 {
    unsafe { ZSTD_CLEVEL_DEFAULT }
}

// When CONFIG_SECURITY_APPARMOR_EXPORT_BINARY is not defined:
#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY"))]
pub const AA_MIN_CLEVEL: i32 = 0;

#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY"))]
pub const AA_MAX_CLEVEL: i32 = 0;

#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_EXPORT_BINARY"))]
pub const AA_DEFAULT_CLEVEL: i32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
