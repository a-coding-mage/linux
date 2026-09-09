/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * configfs_internal.h - Internal stuff for configfs
 *
 * Based on sysfs:
 *   sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 */

// C preprocessor: pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.
// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct configfs_fragment {
    pub frag_count: atomic_t,
    pub frag_sem: rw_semaphore,
    pub frag_dead: bool,
}

extern "C" {
    pub fn put_fragment(fragment: *mut configfs_fragment);
    pub fn get_fragment(fragment: *mut configfs_fragment) -> *mut configfs_fragment;
}

#[repr(C)]
pub struct configfs_dirent {
    pub s_count: atomic_t,
    pub s_dependent_count: core::ffi::c_int,
    pub s_sibling: list_head,
    pub s_children: list_head,
    pub s_links: core::ffi::c_int,
    pub s_element: *mut core::ffi::c_void,
    pub s_type: core::ffi::c_int,
    pub s_mode: umode_t,
    pub s_dentry: *mut dentry,
    pub s_iattr: *mut iattr,
    // Present when CONFIG_LOCKDEP is enabled.
    #[cfg(CONFIG_LOCKDEP)]
    pub s_depth: core::ffi::c_int,
    pub s_frag: *mut configfs_fragment,
}

pub const CONFIGFS_ROOT: core::ffi::c_int = 0x0001;
pub const CONFIGFS_DIR: core::ffi::c_int = 0x0002;
pub const CONFIGFS_ITEM_ATTR: core::ffi::c_int = 0x0004;
pub const CONFIGFS_ITEM_BIN_ATTR: core::ffi::c_int = 0x0008;
pub const CONFIGFS_ITEM_LINK: core::ffi::c_int = 0x0020;
pub const CONFIGFS_USET_DIR: core::ffi::c_int = 0x0040;
pub const CONFIGFS_USET_DEFAULT: core::ffi::c_int = 0x0080;
pub const CONFIGFS_USET_DROPPING: core::ffi::c_int = 0x0100;
pub const CONFIGFS_USET_IN_MKDIR: core::ffi::c_int = 0x0200;
pub const CONFIGFS_USET_CREATING: core::ffi::c_int = 0x0400;
pub const CONFIGFS_NOT_PINNED: core::ffi::c_int = CONFIGFS_ITEM_ATTR | CONFIGFS_ITEM_BIN_ATTR;
pub const CONFIGFS_PINNED: core::ffi::c_int = CONFIGFS_ROOT | CONFIGFS_DIR | CONFIGFS_ITEM_LINK;

extern "C" {
    pub static mut configfs_symlink_mutex: mutex;
    pub static mut configfs_dirent_lock: spinlock_t;
    pub static mut configfs_dir_cachep: *mut kmem_cache;

    pub fn configfs_is_root(item: *mut config_item) -> core::ffi::c_int;
    pub fn configfs_new_inode(mode: umode_t, dirent: *mut configfs_dirent, sb: *mut super_block) -> *mut inode;
    pub fn configfs_create(dentry: *mut dentry, mode: umode_t) -> *mut inode;
    pub fn configfs_create_file(item: *mut config_item, attr: *const configfs_attribute) -> core::ffi::c_int;
    pub fn configfs_create_bin_file(item: *mut config_item, attr: *const configfs_bin_attribute) -> core::ffi::c_int;
    pub fn configfs_make_dirent(dirent: *mut configfs_dirent, dentry: *mut dentry, element: *mut core::ffi::c_void, mode: umode_t, ty: core::ffi::c_int, frag: *mut configfs_fragment) -> core::ffi::c_int;
    pub fn configfs_dirent_is_ready(dirent: *mut configfs_dirent) -> core::ffi::c_int;
    pub fn configfs_get_name(sd: *mut configfs_dirent) -> *const u8;
    pub fn configfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> core::ffi::c_int;
    pub fn configfs_pin_fs() -> *mut dentry;
    pub fn configfs_release_fs();
    pub static configfs_dir_operations: file_operations;
    pub static configfs_file_operations: file_operations;
    pub static configfs_bin_file_operations: file_operations;
    pub static configfs_dir_inode_operations: inode_operations;
    pub static configfs_root_inode_operations: inode_operations;
    pub static configfs_symlink_inode_operations: inode_operations;
    pub static configfs_dentry_ops: dentry_operations;
    pub fn configfs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn configfs_unlink(dir: *mut inode, dentry: *mut dentry) -> core::ffi::c_int;
    pub fn configfs_create_link(target: *mut configfs_dirent, parent: *mut dentry, dentry: *mut dentry, body: *mut core::ffi::c_char) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn to_item(dentry: *mut dentry) -> *mut config_item {
    (*(dentry)).d_fsdata as *mut configfs_dirent as *mut config_item
}

#[inline]
pub unsafe fn to_attr(dentry: *mut dentry) -> *mut configfs_attribute {
    (*(dentry)).d_fsdata as *mut configfs_dirent as *mut configfs_attribute
}

#[inline]
pub unsafe fn to_bin_attr(dentry: *mut dentry) -> *mut configfs_bin_attribute {
    let attr = to_attr(dentry);
    container_of!(attr, configfs_bin_attribute, cb_attr)
}

#[inline]
pub unsafe fn configfs_get_config_item(dentry: *mut dentry) -> *mut config_item {
    let mut item: *mut config_item = core::ptr::null_mut();
    spin_lock(&mut (*dentry).d_lock);
    if !d_unhashed(dentry) {
        let sd = (*dentry).d_fsdata as *mut configfs_dirent;
        item = config_item_get((*sd).s_element as *mut config_item);
    }
    spin_unlock(&mut (*dentry).d_lock);
    item
}

#[inline]
pub unsafe fn release_configfs_dirent(sd: *mut configfs_dirent) {
    if (*sd).s_type & CONFIGFS_ROOT == 0 {
        kfree((*sd).s_iattr as *mut core::ffi::c_void);
        put_fragment((*sd).s_frag);
        kmem_cache_free(configfs_dir_cachep, sd as *mut core::ffi::c_void);
    }
}

#[inline]
pub unsafe fn configfs_get(sd: *mut configfs_dirent) -> *mut configfs_dirent {
    if !sd.is_null() {
        WARN_ON(atomic_read(&mut (*sd).s_count) == 0);
        atomic_inc(&mut (*sd).s_count);
    }
    sd
}

#[inline]
pub unsafe fn configfs_put(sd: *mut configfs_dirent) {
    WARN_ON(atomic_read(&mut (*sd).s_count) == 0);
    if atomic_dec_and_test(&mut (*sd).s_count) {
        release_configfs_dirent(sd);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
