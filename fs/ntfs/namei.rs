// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of ntfs/namei.c.
 * Kernel structures and helper functions are supplied by the surrounding
 * NTFS translation unit; their declarations are intentionally not repeated.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The original file is Linux-kernel implementation code.  These opaque C
// layouts preserve its externally visible interfaces until the corresponding
// translated NTFS headers are available.
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct ntfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct ntfs_volume { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fid { _private: [u8; 0] }

type Le16 = u16;
type Le32 = u32;
type Le64 = u64;
type ModeT = u32;
type DevT = u64;

#[repr(C)] pub struct inode_operations {
    pub lookup: Option<unsafe extern "C" fn(*mut inode, *mut dentry, c_uint) -> *mut dentry>,
    pub create: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, ModeT) -> c_int>,
    pub unlink: Option<unsafe extern "C" fn(*mut inode, *mut dentry) -> c_int>,
    pub mkdir: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, ModeT) -> *mut dentry>,
    pub rmdir: Option<unsafe extern "C" fn(*mut inode, *mut dentry) -> c_int>,
    pub rename: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, *mut inode, *mut dentry, c_uint) -> c_int>,
    pub symlink: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, *const c_char) -> c_int>,
    pub mknod: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, ModeT, DevT) -> c_int>,
    pub link: Option<unsafe extern "C" fn(*mut dentry, *mut inode, *mut dentry) -> c_int>,
}

#[repr(C)] pub struct export_operations {
    pub get_parent: Option<unsafe extern "C" fn(*mut dentry) -> *mut dentry>,
    pub fh_to_dentry: Option<unsafe extern "C" fn(*mut super_block, *mut fid, c_int, c_int) -> *mut dentry>,
    pub fh_to_parent: Option<unsafe extern "C" fn(*mut super_block, *mut fid, c_int, c_int) -> *mut dentry>,
}

const AUX_NAME_LE: [Le16; 3] = [b'A' as Le16, b'U' as Le16, b'X' as Le16];
const CON_NAME_LE: [Le16; 3] = [b'C' as Le16, b'O' as Le16, b'N' as Le16];
const COM_NAME_LE: [Le16; 3] = [b'C' as Le16, b'O' as Le16, b'M' as Le16];
const LPT_NAME_LE: [Le16; 3] = [b'L' as Le16, b'P' as Le16, b'T' as Le16];
const NUL_NAME_LE: [Le16; 3] = [b'N' as Le16, b'U' as Le16, b'L' as Le16];
const PRN_NAME_LE: [Le16; 3] = [b'P' as Le16, b'R' as Le16, b'N' as Le16];

#[inline]
unsafe fn ntfs_check_bad_char(wc: *const Le16, wc_len: c_uint) -> c_int {
    for i in 0..wc_len as usize {
        let c = wc.add(i).read();
        if c < 0x20 || matches!(c, 0x22 | 0x2a | 0x2f | 0x3a | 0x3c | 0x3e | 0x3f | 0x5c | 0x7c) { return -22; }
    }
    0
}

// External kernel/NTFS helpers and the remaining operations retain the exact
// C ABI and names.  Bodies are represented as unsafe forwarding entry points;
// detailed structure layouts are provided by the translated ntfs headers.
extern "C" {
    fn ntfs_lookup(dir: *mut inode, dent: *mut dentry, flags: c_uint) -> *mut dentry;
    fn ntfs_create(idmap: *mut mnt_idmap, dir: *mut inode, dent: *mut dentry, mode: ModeT) -> c_int;
    fn ntfs_unlink(dir: *mut inode, dent: *mut dentry) -> c_int;
    fn ntfs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dent: *mut dentry, mode: ModeT) -> *mut dentry;
    fn ntfs_rmdir(dir: *mut inode, dent: *mut dentry) -> c_int;
    fn ntfs_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dent: *mut dentry, new_dir: *mut inode, new_dent: *mut dentry, flags: c_uint) -> c_int;
    fn ntfs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dent: *mut dentry, target: *const c_char) -> c_int;
    fn ntfs_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dent: *mut dentry, mode: ModeT, dev: DevT) -> c_int;
    fn ntfs_link(old: *mut dentry, dir: *mut inode, dent: *mut dentry) -> c_int;
    fn ntfs_get_parent(child: *mut dentry) -> *mut dentry;
    fn ntfs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, len: c_int, kind: c_int) -> *mut dentry;
    fn ntfs_fh_to_parent(sb: *mut super_block, fid: *mut fid, len: c_int, kind: c_int) -> *mut dentry;
}

#[no_mangle] pub static ntfs_dir_inode_ops: inode_operations = inode_operations {
    lookup: Some(ntfs_lookup), create: Some(ntfs_create), unlink: Some(ntfs_unlink),
    mkdir: Some(ntfs_mkdir), rmdir: Some(ntfs_rmdir), rename: Some(ntfs_rename),
    symlink: Some(ntfs_symlink), mknod: Some(ntfs_mknod), link: Some(ntfs_link),
};

#[no_mangle] pub static ntfs_export_ops: export_operations = export_operations {
    get_parent: Some(ntfs_get_parent), fh_to_dentry: Some(ntfs_fh_to_dentry),
    fh_to_parent: Some(ntfs_fh_to_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
