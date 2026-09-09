// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/ocfs2/ioctl.c.  Types and helpers are supplied by
// the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* The original includes provide these declarations. */
extern "C" {
    fn ocfs2_change_file_space(filp: *mut file, cmd: u32, sr: *mut ocfs2_space_resv) -> c_long;
    fn ocfs2_group_extend(inode: *mut inode, clusters: i32) -> i32;
    fn ocfs2_group_add(inode: *mut inode, input: *mut ocfs2_new_group_input) -> i32;
    fn ocfs2_reflink_ioctl(inode: *mut inode, old_path: *const u8, new_path: *const u8, preserve: bool) -> c_long;
    fn ocfs2_ioctl_move_extents(filp: *mut file, argp: *mut c_void) -> c_long;
    fn ocfs2_trim_fs(sb: *mut super_block, range: *mut fstrim_range) -> i32;
}

type c_long = isize;

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_space_resv { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_new_group_input { _private: [u8; 0] }
#[repr(C)] pub struct fstrim_range { pub start: u64, pub len: u64, pub minlen: u64 }

const EFAULT: c_long = -14;
const EPERM: c_long = -1;
const EOPNOTSUPP: c_long = -95;
const ENOTTY: c_long = -25;
const ENOIOCTLCMD: c_long = -515;

extern "C" { fn file_inode(file: *mut file) -> *mut inode; }

/* ioctl numbers and kernel helpers are intentionally external dependencies. */
extern "C" {
    fn capable(cap: u32) -> bool;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn mnt_want_write_file(file: *mut file) -> i32;
    fn mnt_drop_write_file(file: *mut file);
}

#[repr(C)] struct reflink_arguments { old_path: u64, new_path: u64, preserve: u32 }
#[repr(C)] struct ocfs2_info { oi_count: u32, oi_requests: u64 }

extern "C" {
    static OCFS2_IOC_RESVSP: u32; static OCFS2_IOC_RESVSP64: u32;
    static OCFS2_IOC_UNRESVSP: u32; static OCFS2_IOC_UNRESVSP64: u32;
    static OCFS2_IOC_GROUP_EXTEND: u32; static OCFS2_IOC_GROUP_ADD: u32;
    static OCFS2_IOC_GROUP_ADD64: u32; static OCFS2_IOC_REFLINK: u32;
    static OCFS2_IOC_INFO: u32; static FITRIM: u32; static OCFS2_IOC_MOVE_EXT: u32;
    fn ocfs2_info_handle(inode: *mut inode, info: *mut ocfs2_info, compat: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_ioctl(filp: *mut file, cmd: u32, arg: usize) -> c_long {
    let inode = file_inode(filp);
    let argp = arg as *mut c_void;
    if cmd == OCFS2_IOC_RESVSP || cmd == OCFS2_IOC_RESVSP64 ||
       cmd == OCFS2_IOC_UNRESVSP || cmd == OCFS2_IOC_UNRESVSP64 {
        let mut sr = core::mem::MaybeUninit::<ocfs2_space_resv>::uninit();
        if copy_from_user(sr.as_mut_ptr() as *mut c_void, argp, core::mem::size_of::<ocfs2_space_resv>()) != 0 { return EFAULT; }
        return ocfs2_change_file_space(filp, cmd, sr.as_mut_ptr());
    }
    if cmd == OCFS2_IOC_GROUP_EXTEND {
        if !capable(24) { return EPERM; }
        let mut clusters = 0i32;
        if copy_from_user((&mut clusters as *mut i32).cast(), argp, 4) != 0 { return EFAULT; }
        let status = mnt_want_write_file(filp); if status != 0 { return status as c_long; }
        let status = ocfs2_group_extend(inode, clusters) as c_long; mnt_drop_write_file(filp); return status;
    }
    if cmd == OCFS2_IOC_GROUP_ADD || cmd == OCFS2_IOC_GROUP_ADD64 {
        if !capable(24) { return EPERM; }
        let mut input = core::mem::MaybeUninit::<ocfs2_new_group_input>::uninit();
        if copy_from_user(input.as_mut_ptr().cast(), argp, core::mem::size_of::<ocfs2_new_group_input>()) != 0 { return EFAULT; }
        let status = mnt_want_write_file(filp); if status != 0 { return status as c_long; }
        let status = ocfs2_group_add(inode, input.as_mut_ptr()) as c_long; mnt_drop_write_file(filp); return status;
    }
    if cmd == OCFS2_IOC_REFLINK {
        let mut args = core::mem::MaybeUninit::<reflink_arguments>::uninit();
        if copy_from_user(args.as_mut_ptr().cast(), argp, core::mem::size_of::<reflink_arguments>()) != 0 { return EFAULT; }
        let a = args.assume_init(); return ocfs2_reflink_ioctl(inode, a.old_path as *const u8, a.new_path as *const u8, a.preserve != 0);
    }
    if cmd == OCFS2_IOC_INFO {
        let mut info = core::mem::MaybeUninit::<ocfs2_info>::uninit();
        if copy_from_user(info.as_mut_ptr().cast(), argp, core::mem::size_of::<ocfs2_info>()) != 0 { return EFAULT; }
        return ocfs2_info_handle(inode, info.as_mut_ptr(), 0) as c_long;
    }
    if cmd == FITRIM { return ENOTTY; }
    if cmd == OCFS2_IOC_MOVE_EXT { return ocfs2_ioctl_move_extents(filp, argp); }
    ENOTTY
}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_compat_ioctl(file: *mut file, cmd: u32, arg: usize) -> c_long {
    if cmd == OCFS2_IOC_REFLINK || cmd == OCFS2_IOC_INFO { return ocfs2_ioctl(file, cmd, arg); }
    if cmd == OCFS2_IOC_RESVSP || cmd == OCFS2_IOC_RESVSP64 || cmd == OCFS2_IOC_UNRESVSP ||
       cmd == OCFS2_IOC_UNRESVSP64 || cmd == OCFS2_IOC_GROUP_EXTEND || cmd == OCFS2_IOC_GROUP_ADD ||
       cmd == OCFS2_IOC_GROUP_ADD64 || cmd == FITRIM || cmd == OCFS2_IOC_MOVE_EXT { return ocfs2_ioctl(file, cmd, arg); }
    ENOIOCTLCMD
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
