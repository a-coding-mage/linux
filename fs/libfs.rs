// SPDX-License-Identifier: GPL-2.0-only
// Source-level Rust translation of fs/libfs.c.
// Kernel types, constants, and helper functions are supplied by other units.

#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type loff_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type __u32 = u32;

#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct fid { _private: [u8; 0] }
#[repr(C)] pub struct delayed_call { _private: [u8; 0] }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct tree_descr { _private: [u8; 0] }
#[repr(C)] pub struct fs_context_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct inode_operations { _private: [u8; 0] }
#[repr(C)] pub struct address_space_operations { _private: [u8; 0] }
#[repr(C)] pub struct offset_ctx { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }

extern "C" {
    pub fn simple_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int;
    pub fn simple_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int;
    pub fn always_delete_dentry(dentry: *const dentry) -> c_int;
    pub fn simple_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry;
    pub fn dcache_dir_open(inode: *mut inode, file: *mut file) -> c_int;
    pub fn dcache_dir_close(inode: *mut inode, file: *mut file) -> c_int;
    pub fn dcache_dir_lseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    pub fn dcache_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    pub fn generic_read_dir(file: *mut file, buf: *mut c_char, siz: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn simple_offset_init(ctx: *mut offset_ctx);
    pub fn simple_offset_add(ctx: *mut offset_ctx, dentry: *mut dentry) -> c_int;
    pub fn simple_offset_remove(ctx: *mut offset_ctx, dentry: *mut dentry);
    pub fn simple_offset_rename(old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry);
    pub fn simple_offset_rename_exchange(old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry) -> c_int;
    pub fn simple_offset_destroy(ctx: *mut offset_ctx);
    pub fn find_next_child(parent: *mut dentry, prev: *mut dentry) -> *mut dentry;
    pub fn simple_recursive_removal(dentry: *mut dentry, callback: Option<unsafe extern "C" fn(*mut dentry)>);
    pub fn simple_remove_by_name(parent: *mut dentry, name: *const c_char, callback: Option<unsafe extern "C" fn(*mut dentry)>);
    pub fn locked_recursive_removal(dentry: *mut dentry, callback: Option<unsafe extern "C" fn(*mut dentry)>);
    pub fn init_pseudo(fc: *mut fs_context, magic: c_ulong) -> *mut c_void;
    pub fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    pub fn simple_link(old: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> c_int;
    pub fn simple_empty(dentry: *mut dentry) -> c_int;
    pub fn __simple_unlink(dir: *mut inode, dentry: *mut dentry);
    pub fn __simple_rmdir(dir: *mut inode, dentry: *mut dentry);
    pub fn simple_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
    pub fn simple_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int;
    pub fn simple_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int;
    pub fn simple_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    pub fn simple_fill_super(sb: *mut super_block, magic: c_ulong, files: *const tree_descr) -> c_int;
    pub fn simple_read_from_buffer(to: *mut c_void, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    pub fn simple_write_to_buffer(to: *mut c_void, available: size_t, ppos: *mut loff_t, from: *const c_void, count: size_t) -> ssize_t;
    pub fn memory_read_from_buffer(to: *mut c_void, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    pub fn noop_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
    pub fn noop_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    pub fn kfree_link(p: *mut c_void);
    pub fn alloc_anon_inode(sb: *mut super_block) -> *mut inode;
    pub fn simple_get_link(dentry: *mut dentry, inode: *mut inode, done: *mut delayed_call) -> *const c_char;
    pub fn make_empty_dir_inode(inode: *mut inode);
    pub fn is_empty_dir_inode(inode: *mut inode) -> bool;
    pub fn generic_ci_match(parent: *const inode, name: *const qstr, folded_name: *const qstr, de_name: *const u8, de_name_len: u32) -> c_int;
    pub fn generic_set_sb_d_ops(sb: *mut super_block);
    pub fn inode_maybe_inc_iversion(inode: *mut inode, force: bool) -> bool;
    pub fn inode_query_iversion(inode: *mut inode) -> u64;
    pub fn direct_write_fallback(iocb: *mut kiocb, iter: *mut iov_iter, direct_written: ssize_t, buffered_written: ssize_t) -> ssize_t;
    pub fn simple_inode_init_ts(inode: *mut inode) -> timespec64;
    pub fn stashed_dentry_get(stashed: *mut *mut dentry) -> *mut dentry;
    pub fn path_from_stashed(stashed: *mut *mut dentry, mnt: *mut vfsmount, data: *mut c_void, path: *mut path) -> c_int;
    pub fn stashed_dentry_prune(dentry: *mut dentry);
    pub fn simple_start_creating(parent: *mut dentry, name: *const c_char) -> *mut dentry;
    pub fn simple_done_creating(child: *mut dentry);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
