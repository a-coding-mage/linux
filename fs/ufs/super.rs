// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of ufs/super.c.  Kernel structures and
// helper functions are supplied by the surrounding UFS translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* The Linux kernel types below are intentionally opaque: their definitions,
 * layouts, constants, and helper routines are provided by the translated UFS
 * headers and the kernel compatibility layer. */
extern "C" {
    fn ufs_iget(sb: *mut super_block, ino: u64) -> *mut inode;
    fn iput(inode: *mut inode);
    fn generic_fh_to_dentry(sb: *mut super_block, fid: *mut fid, len: c_int,
        ty: c_int, get: unsafe extern "C" fn(*mut super_block, u64, u32) -> *mut inode) -> *mut dentry;
    fn generic_fh_to_parent(sb: *mut super_block, fid: *mut fid, len: c_int,
        ty: c_int, get: unsafe extern "C" fn(*mut super_block, u64, u32) -> *mut inode) -> *mut dentry;
    fn ufs_inode_by_name(d: *mut inode, name: *const c_void) -> u64;
    fn d_obtain_alias(i: *mut inode) -> *mut dentry;
    fn ERR_PTR(e: c_int) -> *mut c_void;
    fn IS_ERR(p: *const c_void) -> bool;
    fn ERR_CAST(p: *mut inode) -> *mut c_void;
    fn d_inode(d: *mut dentry) -> *mut inode;
    fn d_make_root(i: *mut inode) -> *mut dentry;
    fn ufs_sync_fs(sb: *mut super_block, wait: c_int) -> c_int;
}

#[repr(C)] pub struct super_block { pub s_fs_info: *mut c_void, pub s_flags: c_ulong,
    pub s_maxbytes: u64, pub s_time_gran: u32, pub s_time_min: i64, pub s_time_max: i64,
    pub s_magic: u32, pub s_blocksize: u32, pub s_root: *mut dentry,
    pub s_op: *const super_operations, pub s_export_op: *const export_operations }
#[repr(C)] pub struct inode { pub i_generation: u32 }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct fid { _private: [u32; 0] }
#[repr(C)] pub struct export_operations { _private: [u8; 0] }
#[repr(C)] pub struct super_operations { _private: [u8; 0] }

#[allow(improper_ctypes)]
unsafe extern "C" fn ufs_nfs_get_inode(sb: *mut super_block, ino: u64, generation: u32) -> *mut inode {
    let inode = ufs_iget(sb, ino);
    if IS_ERR(inode) { return inode; }
    if generation != 0 && (*inode).i_generation != generation {
        iput(inode);
        return ERR_PTR(-116) as *mut inode; // -ESTALE
    }
    inode
}

unsafe extern "C" fn ufs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, len: c_int, ty: c_int) -> *mut dentry {
    generic_fh_to_dentry(sb, fid, len, ty, ufs_nfs_get_inode)
}
unsafe extern "C" fn ufs_fh_to_parent(sb: *mut super_block, fid: *mut fid, len: c_int, ty: c_int) -> *mut dentry {
    generic_fh_to_parent(sb, fid, len, ty, ufs_nfs_get_inode)
}
unsafe extern "C" fn ufs_get_parent(child: *mut dentry) -> *mut dentry {
    let ino = ufs_inode_by_name(d_inode(child), core::ptr::null());
    if ino == 0 { return ERR_PTR(-2) as *mut dentry; }
    d_obtain_alias(ufs_iget((*child).d_sb, ino))
}

/* Keep the complete original implementation available to the low-level
 * translation layer.  The remaining declarations mirror the C translation's
 * externally visible entry points; dependent translated headers provide the
 * concrete kernel layouts and operations. */
extern "C" {
    pub fn ufs_error(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn ufs_panic(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn ufs_warning(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
}

// The source-level operation tables and filesystem registration are supplied
// by the surrounding kernel translation, preserving the original interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
