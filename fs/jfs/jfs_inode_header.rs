/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2001
 */

/* C forward declarations and types supplied by the surrounding kernel code. */
#[repr(C)]
pub struct fid {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_kattr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct writeback_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iattr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct address_space_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry_operations {
    _private: [u8; 0],
}

pub type umode_t = u32;
pub type loff_t = i64;
pub type sector_t = u64;

extern "C" {
    pub fn ialloc(inode: *mut inode, mode: umode_t) -> *mut inode;
    pub fn jfs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn jfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> ::std::os::raw::c_int;
    pub fn jfs_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> ::std::os::raw::c_int;
    pub fn jfs_ioctl(file: *mut file, cmd: ::std::os::raw::c_uint, arg: ::std::os::raw::c_ulong) -> ::std::os::raw::c_long;
    pub fn jfs_iget(sb: *mut super_block, ino: ::std::os::raw::c_ulong) -> *mut inode;
    pub fn jfs_commit_inode(inode: *mut inode, do_sync: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn jfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> ::std::os::raw::c_int;
    pub fn jfs_evict_inode(inode: *mut inode);
    pub fn jfs_dirty_inode(inode: *mut inode, flags: ::std::os::raw::c_int);
    pub fn jfs_truncate(inode: *mut inode);
    pub fn jfs_truncate_nolock(inode: *mut inode, length: loff_t);
    pub fn jfs_free_zero_link(inode: *mut inode);
    pub fn jfs_get_parent(dentry: *mut dentry) -> *mut dentry;
    pub fn jfs_fh_to_dentry(
        sb: *mut super_block,
        fid: *mut fid,
        fh_len: ::std::os::raw::c_int,
        fh_type: ::std::os::raw::c_int,
    ) -> *mut dentry;
    pub fn jfs_fh_to_parent(
        sb: *mut super_block,
        fid: *mut fid,
        fh_len: ::std::os::raw::c_int,
        fh_type: ::std::os::raw::c_int,
    ) -> *mut dentry;
    pub fn jfs_set_inode_flags(inode: *mut inode);
    pub fn jfs_get_block(
        inode: *mut inode,
        block: sector_t,
        bh: *mut buffer_head,
        create: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn jfs_setattr(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        attr: *mut iattr,
    ) -> ::std::os::raw::c_int;

    pub static jfs_aops: address_space_operations;
    pub static jfs_dir_inode_operations: inode_operations;
    pub static jfs_dir_operations: file_operations;
    pub static jfs_file_inode_operations: inode_operations;
    pub static jfs_file_operations: file_operations;
    pub static jfs_symlink_inode_operations: inode_operations;
    pub static jfs_fast_symlink_inode_operations: inode_operations;
    pub static jfs_ci_dentry_operations: dentry_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
