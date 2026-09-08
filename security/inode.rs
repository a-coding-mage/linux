// SPDX-License-Identifier: GPL-2.0-only
/*
 *  inode.rs - securityfs
 *
 *  Copyright (C) 2005 Greg Kroah-Hartman <gregkh@suse.de>
 *
 *  Based on fs/debugfs/inode.c which had the following copyright notice:
 *    Copyright (C) 2004 Greg Kroah-Hartman <greg@kroah.com>
 *    Copyright (C) 2004 IBM Inc.
 */

// Translated from C to Rust, originally included:
// <linux/sysfs.h>
// <linux/kobject.h>
// <linux/fs.h>
// <linux/fs_context.h>
// <linux/mount.h>
// <linux/pagemap.h>
// <linux/init.h>
// <linux/namei.h>
// <linux/security.h>
// <linux/lsm_hooks.h>
// <linux/magic.h>
// "lsm.h"

use core::ffi::c_void;
use core::ptr;

// External Linux kernel types and functions
extern "C" {
    // Types
    pub struct vfsmount;
    pub struct dentry;
    pub struct inode;
    pub struct super_block;
    pub struct fs_context;
    pub struct file_operations;
    pub struct inode_operations;
    pub struct file;
    pub struct file_system_type;
    pub struct super_operations;
    pub struct fs_context_operations;
    pub struct tree_descr;
    pub struct spinlock_t;

    // Functions
    fn kfree_const(ptr: *const c_void);
    fn free_inode_nonrcu(inode: *mut inode);
    fn simple_statfs(sb: *mut super_block, attr: *mut c_void) -> i32;
    fn simple_fill_super(
        sb: *mut super_block,
        magic: u32,
        files: *const tree_descr,
    ) -> i32;
    fn get_tree_single(
        fc: *mut fs_context,
        fill_super: extern "C" fn(*mut super_block, *mut fs_context) -> i32,
    ) -> i32;
    fn simple_pin_fs(
        type_: *mut file_system_type,
        mount: *mut *mut vfsmount,
        count: *mut i32,
    ) -> i32;
    fn simple_release_fs(mount: *mut *mut vfsmount, count: *mut i32);
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn d_inode(dentry: *const dentry) -> *mut inode;
    fn simple_start_creating(parent: *mut dentry, name: *const i8) -> *mut dentry;
    fn iput(inode: *mut inode);
    fn get_next_ino() -> u32;
    fn simple_inode_init_ts(inode: *mut inode);
    fn inc_nlink(inode: *mut inode);
    fn d_make_persistent(dentry: *mut dentry, inode: *mut inode);
    fn simple_done_creating(dentry: *mut dentry);
    fn simple_recursive_removal(
        dentry: *mut dentry,
        callback: extern "C" fn(*mut dentry),
    );
    fn kstrdup_const(s: *const i8, gfp_flags: u32) -> *const i8;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn pr_debug(fmt: *const i8, ...);
    fn simple_read_from_buffer(
        to: *mut u8,
        n: usize,
        ppos: *mut i64,
        from: *const i8,
        available: usize,
    ) -> isize;
    fn sysfs_create_mount_point(kobj: *mut c_void, name: *const i8) -> i32;
    fn sysfs_remove_mount_point(kobj: *mut c_void, name: *const i8);
    fn register_filesystem(type_: *mut file_system_type) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn strcat(dest: *mut i8, src: *const i8) -> *mut i8;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);

    pub static mut kernel_kobj: *mut c_void;
    pub static lsm_active_cnt: i32;
    pub static lsm_idlist: *const *const c_void;
}

// External constants and macros
pub const SECURITYFS_MAGIC: u32 = 0x73636673;
pub const S_IALLUGO: u32 = 0o7777;
pub const S_IFREG: u32 = 0o100000;
pub const S_IFDIR: u32 = 0o40000;
pub const S_IFLNK: u32 = 0o120000;
pub const S_IFMT: u32 = 0o170000;
pub const GFP_KERNEL: u32 = 0x10c0;
pub const THIS_MODULE: *mut c_void = ptr::null_mut();

// External helper macros/functions
extern "C" {
    pub fn S_ISLNK(mode: u32) -> i32;
    pub fn S_ISDIR(mode: u32) -> i32;
    pub fn IS_ERR(ptr: *const c_void) -> i32;
    pub fn IS_ERR_OR_NULL(ptr: *const c_void) -> i32;
    pub fn ERR_PTR(error: i32) -> *mut c_void;
    pub fn PTR_ERR(ptr: *const c_void) -> i32;
    pub fn kill_anon_super(sb: *mut super_block);
    pub fn generic_file_llseek(file: *mut file, offset: i64, whence: i32) -> i64;
    pub fn simple_dir_inode_operations() -> *const inode_operations;
    pub fn simple_dir_operations() -> *const file_operations;
    pub fn simple_symlink_inode_operations() -> *const inode_operations;

    static simple_dir_inode_operations: inode_operations;
    static simple_dir_operations: file_operations;
    static simple_symlink_inode_operations: inode_operations;
}

extern "C" {
    pub fn kill_anon_super(sb: *mut super_block);
}

// Kernels constants for inode structure fields
pub const ENOMEM: i32 = -12;
pub const ENODEV: i32 = -19;

// Static variables
static mut mount: *mut vfsmount = ptr::null_mut();
static mut mount_count: i32 = 0;

// Security file inode free function
unsafe extern "C" fn securityfs_free_inode(inode_ptr: *mut inode) {
    if S_ISLNK((*inode_ptr).i_mode) != 0 {
        kfree_const((*inode_ptr).i_link as *const c_void);
    }
    free_inode_nonrcu(inode_ptr);
}

// Super operations structure
#[repr(C)]
pub struct SecurityfsSuperOps {
    statfs: extern "C" fn(*mut super_block, *mut c_void) -> i32,
    free_inode: extern "C" fn(*mut inode),
}

unsafe {
    static SECURITYFS_SUPER_OPERATIONS: super_operations = unsafe {
        // This needs to be initialized via FFI
        core::mem::zeroed()
    };
}

// Fill superblock function
unsafe extern "C" fn securityfs_fill_super(
    sb: *mut super_block,
    fc: *mut fs_context,
) -> i32 {
    let files: [tree_descr; 1] = [core::mem::zeroed()];

    let error = simple_fill_super(sb, SECURITYFS_MAGIC, files.as_ptr());
    if error != 0 {
        return error;
    }

    (*sb).s_op = &SECURITYFS_SUPER_OPERATIONS as *const super_operations;

    0
}

// Get tree function
unsafe extern "C" fn securityfs_get_tree(fc: *mut fs_context) -> i32 {
    get_tree_single(fc, securityfs_fill_super)
}

// File system type structure
#[repr(C)]
pub struct SecurityfsContextOps {
    get_tree: extern "C" fn(*mut fs_context) -> i32,
}

unsafe {
    static SECURITYFS_CONTEXT_OPS: fs_context_operations = unsafe {
        // This needs to be initialized via FFI
        core::mem::zeroed()
    };
}

// Init fs context function
unsafe extern "C" fn securityfs_init_fs_context(fc: *mut fs_context) -> i32 {
    (*fc).ops = &SECURITYFS_CONTEXT_OPS as *const fs_context_operations;
    0
}

// File system type structure
#[repr(C)]
pub struct SecurityfsType {
    pub owner: *mut c_void,
    pub name: *const i8,
    pub init_fs_context: extern "C" fn(*mut fs_context) -> i32,
    pub kill_sb: extern "C" fn(*mut super_block),
}

unsafe {
    static FS_TYPE: file_system_type = unsafe {
        // This needs to be initialized via FFI
        core::mem::zeroed()
    };
}

/// securityfs_create_dentry - create a dentry in the securityfs filesystem
///
/// @name: a pointer to a string containing the name of the file to create.
/// @mode: the permission that the file should have
/// @parent: a pointer to the parent dentry for this file.  This should be a
///          directory dentry if set.  If this parameter is NULL, then the
///          file will be created in the root of the securityfs filesystem.
/// @data: a pointer to something that the caller will want to get to later
///        on.  The inode.i_private pointer will point to this value on
///        the open() call.
/// @fops: a pointer to a struct file_operations that should be used for
///        this file.
/// @iops: a point to a struct of inode_operations that should be used for
///        this file/dir
///
/// This is the basic "create a file/dir/symlink" function for
/// securityfs.  It allows for a wide range of flexibility in creating
/// a file, or a directory (if you want to create a directory, the
/// securityfs_create_dir() function is recommended to be used
/// instead).
///
/// This function returns a pointer to a dentry if it succeeds.  This
/// pointer must be passed to the securityfs_remove() function when the
/// file is to be removed (no automatic cleanup happens if your module
/// is unloaded, you are responsible here).  If an error occurs, the
/// function will return the error value (via ERR_PTR).
///
/// If securityfs is not enabled in the kernel, the value %-ENODEV is
/// returned.
unsafe extern "C" fn securityfs_create_dentry(
    name: *const i8,
    mut mode: u32,
    parent: *mut dentry,
    data: *mut c_void,
    fops: *const file_operations,
    iops: *const inode_operations,
) -> *mut dentry {
    let mut dentry_out: *mut dentry = ptr::null_mut();
    let mut dir: *mut inode = ptr::null_mut();
    let mut inode_ptr: *mut inode = ptr::null_mut();
    let mut error: i32 = 0;
    let mut pinned: bool = false;
    let mut parent_mut = parent;

    if (mode & S_IFMT) == 0 {
        mode = (mode & S_IALLUGO) | S_IFREG;
    }

    pr_debug(b"securityfs: creating file '%s'\n\0".as_ptr() as *const i8, name);

    if parent_mut.is_null() {
        error = simple_pin_fs(&FS_TYPE as *mut file_system_type, &mut mount, &mut mount_count);
        if error != 0 {
            return ERR_PTR(error) as *mut dentry;
        }
        pinned = true;
        parent_mut = (*mount).mnt_root;
    }

    inode_ptr = new_inode((*parent_mut).d_sb);
    if inode_ptr.is_null() {
        dentry_out = ERR_PTR(ENOMEM) as *mut dentry;
        // goto out
    } else {
        dir = d_inode(parent_mut);

        dentry_out = simple_start_creating(parent_mut, name);
        if IS_ERR(dentry_out as *const c_void) != 0 {
            iput(inode_ptr);
            // goto out
        } else {
            (*inode_ptr).i_ino = get_next_ino();
            (*inode_ptr).i_mode = mode;
            simple_inode_init_ts(inode_ptr);
            (*inode_ptr).i_private = data;

            if S_ISDIR(mode) != 0 {
                (*inode_ptr).i_op = &simple_dir_inode_operations;
                (*inode_ptr).i_fop = &simple_dir_operations;
                inc_nlink(inode_ptr);
                inc_nlink(dir);
            } else if S_ISLNK(mode) != 0 {
                (*inode_ptr).i_op = if !iops.is_null() {
                    iops
                } else {
                    &simple_symlink_inode_operations
                };
                (*inode_ptr).i_link = data;
            } else {
                (*inode_ptr).i_fop = fops;
            }
            d_make_persistent(dentry_out, inode_ptr);
            simple_done_creating(dentry_out);
            return dentry_out;
        }
    }

    // out:
    if pinned {
        simple_release_fs(&mut mount, &mut mount_count);
    }
    dentry_out
}

/// securityfs_create_file - create a file in the securityfs filesystem
///
/// @name: a pointer to a string containing the name of the file to create.
/// @mode: the permission that the file should have
/// @parent: a pointer to the parent dentry for this file.  This should be a
///          directory dentry if set.  If this parameter is NULL, then the
///          file will be created in the root of the securityfs filesystem.
/// @data: a pointer to something that the caller will want to get to later
///        on.  The inode.i_private pointer will point to this value on
///        the open() call.
/// @fops: a pointer to a struct file_operations that should be used for
///        this file.
///
/// This function creates a file in securityfs with the given @name.
///
/// This function returns a pointer to a dentry if it succeeds.  This
/// pointer must be passed to the securityfs_remove() function when the file is
/// to be removed (no automatic cleanup happens if your module is unloaded,
/// you are responsible here).  If an error occurs, the function will return
/// the error value (via ERR_PTR).
///
/// If securityfs is not enabled in the kernel, the value %-ENODEV is
/// returned.
#[no_mangle]
pub unsafe extern "C" fn securityfs_create_file(
    name: *const i8,
    mode: u32,
    parent: *mut dentry,
    data: *mut c_void,
    fops: *const file_operations,
) -> *mut dentry {
    securityfs_create_dentry(name, mode, parent, data, fops, ptr::null())
}

// EXPORT_SYMBOL_GPL(securityfs_create_file);

/// securityfs_create_dir - create a directory in the securityfs filesystem
///
/// @name: a pointer to a string containing the name of the directory to
///        create.
/// @parent: a pointer to the parent dentry for this file.  This should be a
///          directory dentry if set.  If this parameter is NULL, then the
///          directory will be created in the root of the securityfs filesystem.
///
/// This function creates a directory in securityfs with the given @name.
///
/// This function returns a pointer to a dentry if it succeeds.  This
/// pointer must be passed to the securityfs_remove() function when the file is
/// to be removed (no automatic cleanup happens if your module is unloaded,
/// you are responsible here).  If an error occurs, the function will return
/// the error value (via ERR_PTR).
///
/// If securityfs is not enabled in the kernel, the value %-ENODEV is
/// returned.
#[no_mangle]
pub unsafe extern "C" fn securityfs_create_dir(
    name: *const i8,
    parent: *mut dentry,
) -> *mut dentry {
    securityfs_create_file(name, S_IFDIR | 0o755, parent, ptr::null_mut(), ptr::null())
}

// EXPORT_SYMBOL_GPL(securityfs_create_dir);

/// securityfs_create_symlink - create a symlink in the securityfs filesystem
///
/// @name: a pointer to a string containing the name of the symlink to
///        create.
/// @parent: a pointer to the parent dentry for the symlink.  This should be a
///          directory dentry if set.  If this parameter is NULL, then the
///          directory will be created in the root of the securityfs filesystem.
/// @target: a pointer to a string containing the name of the symlink's target.
///          If this parameter is NULL, then the @iops parameter needs to be
///          setup to handle .readlink and .get_link inode_operations.
/// @iops: a pointer to the struct inode_operations to use for the symlink. If
///        this parameter is NULL, then the default simple_symlink_inode
///        operations will be used.
///
/// This function creates a symlink in securityfs with the given @name.
///
/// This function returns a pointer to a dentry if it succeeds.  This
/// pointer must be passed to the securityfs_remove() function when the file is
/// to be removed (no automatic cleanup happens if your module is unloaded,
/// you are responsible here).  If an error occurs, the function will return
/// the error value (via ERR_PTR).
///
/// If securityfs is not enabled in the kernel, the value %-ENODEV is
/// returned.
#[no_mangle]
pub unsafe extern "C" fn securityfs_create_symlink(
    name: *const i8,
    parent: *mut dentry,
    target: *const i8,
    iops: *const inode_operations,
) -> *mut dentry {
    let mut link: *const i8 = ptr::null();
    let mut dent: *mut dentry;

    if !target.is_null() {
        link = kstrdup_const(target, GFP_KERNEL);
        if link.is_null() {
            return ERR_PTR(ENOMEM) as *mut dentry;
        }
    }

    dent = securityfs_create_dentry(
        name,
        S_IFLNK | 0o444,
        parent,
        link as *mut c_void,
        ptr::null(),
        iops,
    );

    if IS_ERR(dent as *const c_void) != 0 {
        kfree_const(link as *const c_void);
    }

    dent
}

// EXPORT_SYMBOL_GPL(securityfs_create_symlink);

unsafe extern "C" fn remove_one(victim: *mut dentry) {
    if (*victim).d_parent == (*(*victim).d_sb).s_root {
        simple_release_fs(&mut mount, &mut mount_count);
    }
}

/// securityfs_remove - removes a file or directory from the securityfs filesystem
///
/// @dentry: a pointer to a the dentry of the file or directory to be removed.
///
/// This function removes a file or directory in securityfs that was previously
/// created with a call to another securityfs function (like
/// securityfs_create_file() or variants thereof.)
///
/// This function is required to be called in order for the file to be
/// removed. No automatic cleanup of files will happen when a module is
/// removed; you are responsible here.
///
/// AV: when applied to directory it will take all children out; no need to call
/// it for descendents if ancestor is getting killed.
#[no_mangle]
pub unsafe extern "C" fn securityfs_remove(dentry: *mut dentry) {
    if IS_ERR_OR_NULL(dentry as *const c_void) != 0 {
        return;
    }

    simple_pin_fs(&FS_TYPE as *mut file_system_type, &mut mount, &mut mount_count);
    simple_recursive_removal(dentry, remove_one);
    simple_release_fs(&mut mount, &mut mount_count);
}

// EXPORT_SYMBOL_GPL(securityfs_remove);

// CONFIG_SECURITY conditional section
#[cfg(feature = "CONFIG_SECURITY")]
pub mod lsm_security {
    use super::*;

    pub static mut lsm_dentry: *mut dentry = ptr::null_mut();

    unsafe extern "C" fn lsm_read(
        filp: *mut file,
        buf: *mut u8,
        count: usize,
        ppos: *mut i64,
    ) -> isize {
        static mut str: *mut i8 = ptr::null_mut();
        static mut len: usize = 0;
        static mut lock: spinlock_t = unsafe { core::mem::zeroed() };

        if str.is_null() || len == 0 {
            let mut str_tmp: *mut i8;
            let mut len_tmp: usize = 0;
            let mut i: i32;

            i = 0;
            while i < lsm_active_cnt {
                let name_ptr = *(lsm_idlist.add(i as usize)) as *const i8;
                len_tmp += strlen(name_ptr) + 1;
                i += 1;
            }

            str_tmp = kmalloc(len_tmp, GFP_KERNEL) as *mut i8;
            if str_tmp.is_null() {
                return ENOMEM as isize;
            }
            *str_tmp = b'\0' as i8;

            i = 0;
            while i < lsm_active_cnt {
                if i > 0 {
                    strcat(str_tmp, b",\0".as_ptr() as *const i8);
                }
                let name_ptr = *(lsm_idlist.add(i as usize)) as *const i8;
                strcat(str_tmp, name_ptr);
                i += 1;
            }

            spin_lock(&mut lock);
            if str.is_null() {
                str = str_tmp;
                len = len_tmp - 1;
            } else {
                kfree(str_tmp as *mut c_void);
            }
            spin_unlock(&mut lock);
        }

        simple_read_from_buffer(buf, count, ppos, str, len)
    }

    #[repr(C)]
    pub struct LsmOps {
        pub read: extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize,
        pub llseek: extern "C" fn(*mut file, i64, i32) -> i64,
    }

    unsafe {
        pub static LSM_OPS: file_operations = unsafe {
            // This needs to be initialized via FFI
            core::mem::zeroed()
        };
    }
}

/// securityfs_init - initialize the security filesystem
#[no_mangle]
pub unsafe extern "C" fn securityfs_init() -> i32 {
    let mut retval: i32;

    retval = sysfs_create_mount_point(kernel_kobj, b"security\0".as_ptr() as *const i8);
    if retval != 0 {
        return retval;
    }

    retval = register_filesystem(&FS_TYPE as *mut file_system_type);
    if retval != 0 {
        sysfs_remove_mount_point(kernel_kobj, b"security\0".as_ptr() as *const i8);
        return retval;
    }

    #[cfg(feature = "CONFIG_SECURITY")]
    {
        lsm_security::lsm_dentry = securityfs_create_file(
            b"lsm\0".as_ptr() as *const i8,
            0o444,
            ptr::null_mut(),
            ptr::null_mut(),
            &lsm_security::LSM_OPS as *const file_operations,
        );
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
