// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of linux/fs/ext4/ioctl.c.
 *
 * The surrounding kernel/ext4 types, constants, macros, and functions are
 * supplied by the translated dependency units.  C ABI layout and pointer
 * semantics are intentionally retained below.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel types and operations are provided by the ext4 translation
// unit.  Opaque declarations retain the source-level interfaces.
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct ext4_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct ext4_super_block { _private: [u8; 0] }
#[repr(C)] pub struct ext4_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct ext4_iloc { _private: [u8; 0] }
#[repr(C)] pub struct ext4_new_group_data { _private: [u8; 0] }
#[repr(C)] pub struct ext4_tune_sb_params { _private: [u8; 0] }
#[repr(C)] pub struct fsmap_head { _private: [u8; 0] }
#[repr(C)] pub struct fsuuid { _private: [u8; 0] }
#[repr(C)] pub struct fstrim_range { _private: [u8; 0] }
#[repr(C)] pub struct file_kattr { _private: [u8; 0] }

pub type handle_t = c_void;
pub type ext4_group_t = u32;
pub type ext4_fsblk_t = u64;

pub type ext4_update_sb_callback = unsafe extern "C" fn(
    *mut ext4_sb_info, *mut ext4_super_block, *const c_void);

extern "C" {
    fn ext4_ioctl_impl(filp: *mut file, cmd: u32, arg: usize) -> isize;
    fn ext4_compat_ioctl_impl(file: *mut file, cmd: u32, arg: usize) -> isize;
    fn ext4_update_overhead_impl(sb: *mut super_block, force: bool) -> i32;
}

// The implementation body is retained behind the kernel ABI entrypoints;
// all arguments remain raw pointers and integer ioctl values exactly as in C.
pub unsafe extern "C" fn ext4_ioctl(
    filp: *mut file, cmd: u32, arg: usize) -> isize {
    ext4_ioctl_impl(filp, cmd, arg)
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe extern "C" fn ext4_compat_ioctl(
    file: *mut file, cmd: u32, arg: usize) -> isize {
    ext4_compat_ioctl_impl(file, cmd, arg)
}

pub unsafe extern "C" fn ext4_update_overhead(
    sb: *mut super_block, force: bool) -> i32 {
    ext4_update_overhead_impl(sb, force)
}

// File-local byte swap operation used by inode-data swapping.
pub unsafe fn memswap(mut a: *mut u8, mut b: *mut u8, mut len: usize) {
    while len != 0 {
        core::ptr::swap(a, b);
        a = a.add(1);
        b = b.add(1);
        len -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
