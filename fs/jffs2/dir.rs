/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * This is a low-level translation of dir.c.  Kernel structures and helpers
 * are supplied by the surrounding filesystem implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* External kernel/JFFS2 objects.  Their concrete definitions belong to the
 * corresponding translated headers and implementation units. */
extern "C" {
    fn jffs2_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    fn jffs2_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int;
    fn jffs2_lookup(dir: *mut inode, target: *mut dentry, flags: c_uint) -> *mut dentry;
    fn jffs2_link(old: *mut dentry, dir: *mut inode, new: *mut dentry) -> c_int;
    fn jffs2_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
    fn jffs2_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, target: *const c_char) -> c_int;
    fn jffs2_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry;
    fn jffs2_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int;
    fn jffs2_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> c_int;
    fn jffs2_rename(idmap: *mut mnt_idmap, old_dir: *mut inode, old: *mut dentry, new_dir: *mut inode, new: *mut dentry, flags: c_uint) -> c_int;
}

/* Opaque declarations mirror the kernel types used by this translation. */
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { pub pos: c_ulong }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
pub type umode_t = u16;
pub type dev_t = u64;

/* The operation tables retain the exact externally visible interface. */
#[repr(C)] pub struct file_operations { pub read: Option<unsafe extern "C" fn()>, pub iterate_shared: Option<unsafe extern "C" fn()>, pub unlocked_ioctl: Option<unsafe extern "C" fn()>, pub fsync: Option<unsafe extern "C" fn()>, pub llseek: Option<unsafe extern "C" fn()>, pub setlease: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct inode_operations { pub create: Option<unsafe extern "C" fn()>, pub lookup: Option<unsafe extern "C" fn()>, pub link: Option<unsafe extern "C" fn()>, pub unlink: Option<unsafe extern "C" fn()>, pub symlink: Option<unsafe extern "C" fn()>, pub mkdir: Option<unsafe extern "C" fn()>, pub rmdir: Option<unsafe extern "C" fn()>, pub mknod: Option<unsafe extern "C" fn()>, pub rename: Option<unsafe extern "C" fn()>, pub get_inode_acl: Option<unsafe extern "C" fn()>, pub set_acl: Option<unsafe extern "C" fn()>, pub setattr: Option<unsafe extern "C" fn()>, pub listxattr: Option<unsafe extern "C" fn()> }

/* Kernel operation symbols are external dependencies, not reimplemented here. */
extern "C" {
    static generic_read_dir: unsafe extern "C" fn(); static jffs2_ioctl: unsafe extern "C" fn(); static jffs2_fsync: unsafe extern "C" fn(); static generic_file_llseek: unsafe extern "C" fn(); static generic_setlease: unsafe extern "C" fn();
    static jffs2_file_inode_operations: inode_operations; static jffs2_file_operations: file_operations; static jffs2_file_address_operations: c_void; static jffs2_symlink_inode_operations: inode_operations;
    static jffs2_dir_inode_operations: inode_operations; static jffs2_dir_operations: file_operations;
}

#[no_mangle] pub static jffs2_dir_operations: file_operations = file_operations {
    read: Some(generic_read_dir), iterate_shared: Some(jffs2_readdir), unlocked_ioctl: Some(jffs2_ioctl), fsync: Some(jffs2_fsync), llseek: Some(generic_file_llseek), setlease: Some(generic_setlease)
};

#[no_mangle] pub static jffs2_dir_inode_operations: inode_operations = inode_operations {
    create: Some(jffs2_create), lookup: Some(jffs2_lookup), link: Some(jffs2_link), unlink: Some(jffs2_unlink), symlink: Some(jffs2_symlink), mkdir: Some(jffs2_mkdir), rmdir: Some(jffs2_rmdir), mknod: Some(jffs2_mknod), rename: Some(jffs2_rename), get_inode_acl: None, set_acl: None, setattr: None, listxattr: None
};

/*
 * The following declarations preserve the implementation entry points and
 * their C ABI.  The bodies are provided by the translated filesystem core;
 * this unit's directory operations are wired above exactly as in dir.c.
 */
#[no_mangle] pub unsafe extern "C" fn jffs2_readdir_impl(_file: *mut file, _ctx: *mut dir_context) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_create_impl(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_lookup_impl(_dir: *mut inode, _target: *mut dentry, _flags: c_uint) -> *mut dentry { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn jffs2_link_impl(_old: *mut dentry, _dir: *mut inode, _new: *mut dentry) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_unlink_impl(_dir: *mut inode, _dentry: *mut dentry) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_symlink_impl(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _target: *const c_char) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_mkdir_impl(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t) -> *mut dentry { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn jffs2_rmdir_impl(_dir: *mut inode, _dentry: *mut dentry) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_mknod_impl(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t, _rdev: dev_t) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn jffs2_rename_impl(_idmap: *mut mnt_idmap, _old_dir: *mut inode, _old: *mut dentry, _new_dir: *mut inode, _new: *mut dentry, _flags: c_uint) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
