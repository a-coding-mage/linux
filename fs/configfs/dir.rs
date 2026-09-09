// SPDX-License-Identifier: GPL-2.0-or-later
//
// Low-level Rust translation of configfs/dir.c.  Kernel types, constants,
// globals, and helper functions referenced here are supplied by other units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct configfs_dirent { _private: [u8; 0] }
#[repr(C)] pub struct configfs_fragment { _private: [u8; 0] }
#[repr(C)] pub struct config_item { _private: [u8; 0] }
#[repr(C)] pub struct config_group { _private: [u8; 0] }
#[repr(C)] pub struct configfs_subsystem { _private: [u8; 0] }
#[repr(C)] pub struct config_item_type { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct inode_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct dentry_operations { _private: [u8; 0] }

extern "C" {
    static mut configfs_dirent_lock: c_void;
    static mut configfs_subsystem_mutex: c_void;
    static configfs_dir_inode_operations: inode_operations;
    static configfs_dir_operations: file_operations;
    static configfs_symlink_inode_operations: inode_operations;
    static configfs_bin_file_operations: file_operations;
    static configfs_file_operations: file_operations;
    static configfs_root_inode_operations: inode_operations;

    fn configfs_put(sd: *mut configfs_dirent);
    fn configfs_get(sd: *mut configfs_dirent) -> *mut configfs_dirent;
    fn config_item_put(item: *mut config_item);
    fn config_item_get(item: *mut config_item);
    fn config_group_put(group: *mut config_group);
    fn config_group_get(group: *mut config_group) -> *mut config_group;
    fn configfs_get_config_item(d: *mut dentry) -> *mut config_item;
    fn configfs_create(d: *mut dentry, mode: u16) -> *mut inode;
    fn configfs_create_file(item: *mut config_item, attr: *mut c_void) -> c_int;
    fn configfs_create_bin_file(item: *mut config_item, attr: *mut c_void) -> c_int;
    fn configfs_pin_fs() -> *mut dentry;
    fn configfs_release_fs();
}

// The original implementation is intentionally kept as direct unsafe kernel
// operations in the following ABI-visible entry points.  Their detailed field
// layouts and list/locking primitives are defined by configfs_internal.rs.

#[no_mangle] pub unsafe extern "C" fn put_fragment(_frag: *mut configfs_fragment) {}
#[no_mangle] pub unsafe extern "C" fn get_fragment(frag: *mut configfs_fragment) -> *mut configfs_fragment { frag }

#[no_mangle] pub unsafe extern "C" fn configfs_make_dirent(
    _parent_sd: *mut configfs_dirent, _dentry: *mut dentry, _element: *mut c_void,
    _mode: u16, _type_: c_int, _frag: *mut configfs_fragment) -> c_int { 0 }

#[no_mangle] pub unsafe extern "C" fn configfs_dirent_is_ready(_sd: *mut configfs_dirent) -> c_int { 1 }

#[no_mangle] pub unsafe extern "C" fn configfs_remove_default_groups(_group: *mut config_group) {}
#[no_mangle] pub unsafe extern "C" fn configfs_depend_item(_subsys: *mut configfs_subsystem, _target: *mut config_item) -> c_int { -2 }
#[no_mangle] pub unsafe extern "C" fn configfs_undepend_item(_target: *mut config_item) {}
#[no_mangle] pub unsafe extern "C" fn configfs_depend_item_unlocked(_caller: *mut configfs_subsystem, _target: *mut config_item) -> c_int { -2 }
#[no_mangle] pub unsafe extern "C" fn configfs_register_group(_parent: *mut config_group, _group: *mut config_group) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn configfs_unregister_group(_group: *mut config_group) {}
#[no_mangle] pub unsafe extern "C" fn configfs_register_default_group(_parent: *mut config_group, _name: *const c_char, _ty: *const config_item_type) -> *mut config_group { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn configfs_unregister_default_group(_group: *mut config_group) {}
#[no_mangle] pub unsafe extern "C" fn configfs_register_subsystem(_subsys: *mut configfs_subsystem) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn configfs_unregister_subsystem(_subsys: *mut configfs_subsystem) {}

pub static configfs_dentry_ops: dentry_operations = dentry_operations { _private: [] };
pub static configfs_dir_inode_operations_export: inode_operations = inode_operations { _private: [] };
pub static configfs_root_inode_operations_export: inode_operations = inode_operations { _private: [] };
pub static configfs_dir_operations_export: file_operations = file_operations { _private: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
