/* SPDX-License-Identifier: GPL-2.0 */

// Declarations are conditioned on the Linux kernel build configuration
// (__KERNEL__).

#[repr(C)]
pub struct mnt_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fs_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ns_common {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut init_mnt_ns: mnt_namespace;

    pub fn copy_mnt_ns(
        flags: u64,
        old_ns: *mut mnt_namespace,
        user_ns: *mut user_namespace,
        fs: *mut fs_struct,
    ) -> *mut mnt_namespace;

    pub fn put_mnt_ns(ns: *mut mnt_namespace);

    pub fn from_mnt_ns(ns: *mut mnt_namespace) -> *mut ns_common;

    pub static proc_mounts_operations: file_operations;
    pub static proc_mountinfo_operations: file_operations;
    pub static proc_mountstats_operations: file_operations;
}

// C macro equivalent: free with put_mnt_ns when the value is neither an
// error pointer nor null. IS_ERR_OR_NULL and the cleanup mechanism are
// supplied by the kernel dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
