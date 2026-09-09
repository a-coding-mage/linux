/* SPDX-License-Identifier: GPL-2.0 */
/* rust_binder_internal.h
 *
 * This file contains internal data structures used by Rust Binder. Mostly,
 * these are type definitions used only by binderfs or things that Rust Binder
 * define and export to binderfs.
 *
 * It does not include things exported by binderfs to Rust Binder since this
 * file is not included as input to bindgen.
 *
 * Copyright (C) 2025 Google LLC.
 */

// C header dependencies: linux/seq_file.h, uapi/linux/android/binder.h, and
// uapi/linux/android/binderfs.h.

use core::ffi::c_void;

pub const RUST_BINDERFS_SUPER_MAGIC: u32 = 0x6c6f6f71;

/*
 * The internal data types in the Rust Binder driver are opaque to C, so we use
 * void pointer typedefs for these types.
 */
pub type rust_binder_context = *mut c_void;

/**
 * struct binder_device - information about a binder device node
 * @minor:     the minor number used by this device
 * @ctx:       the Rust Context used by this device, or null for binder-control
 *
 * This is used as the private data for files directly in binderfs, but not
 * files in the binder_logs subdirectory. This struct owns a refcount on `ctx`
 * and the entry for `minor` in `binderfs_minors`. For binder-control `ctx` is
 * null.
 */
#[repr(C)]
pub struct binder_device {
    pub minor: i32,
    pub ctx: rust_binder_context,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

extern "C" {
    pub fn rust_binder_stats_show(m: *mut seq_file, unused: *mut c_void) -> i32;
    pub fn rust_binder_state_show(m: *mut seq_file, unused: *mut c_void) -> i32;
    pub fn rust_binder_transactions_show(m: *mut seq_file, unused: *mut c_void) -> i32;
    pub fn rust_binder_proc_show(m: *mut seq_file, pid: *mut c_void) -> i32;

    pub static rust_binder_fops: file_operations;
    pub fn rust_binder_new_context(name: *mut i8) -> rust_binder_context;
    pub fn rust_binder_remove_context(device: rust_binder_context);
}

/**
 * binderfs_mount_opts - mount options for binderfs
 * @max: maximum number of allocatable binderfs binder devices
 * @stats_mode: enable binder stats in binderfs.
 */
#[repr(C)]
pub struct binderfs_mount_opts {
    pub max: i32,
    pub stats_mode: i32,
}

// Types supplied by the Linux kernel headers included by the C source.
#[repr(C)]
pub struct ipc_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

pub type kuid_t = u32;
pub type kgid_t = u32;

/**
 * binderfs_info - information about a binderfs mount
 * @ipc_ns:         The ipc namespace the binderfs mount belongs to.
 * @control_dentry: This records the dentry of this binderfs mount
 *                  binder-control device.
 * @root_uid:       uid that needs to be used when a new binder device is
 *                  created.
 * @root_gid:       gid that needs to be used when a new binder device is
 *                  created.
 * @mount_opts:     The mount options in use.
 * @device_count:   The current number of allocated binder devices.
 * @proc_log_dir:   Pointer to the directory dentry containing process-specific
 *                  logs.
 */
#[repr(C)]
pub struct binderfs_info {
    pub ipc_ns: *mut ipc_namespace,
    pub control_dentry: *mut dentry,
    pub root_uid: kuid_t,
    pub root_gid: kgid_t,
    pub mount_opts: binderfs_mount_opts,
    pub device_count: i32,
    pub proc_log_dir: *mut dentry,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
