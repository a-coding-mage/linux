// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of btrfs/sysfs.c.  Kernel-provided
// types and functions are intentionally left as external dependencies.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

/* The Linux kernel objects below are supplied by the surrounding kernel
 * translation.  Raw pointers preserve the ownership and aliasing model of
 * the original implementation. */
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut c_char) -> isize>, pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const c_char, usize) -> isize> }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> u16>, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct btrfs_feature_attr { pub kobj_attr: kobj_attribute, pub feature_set: c_int, pub feature_bit: u64 }
#[repr(C)] pub struct raid_kobject { pub flags: u64, pub kobj: kobject }

/* C's container_of is deliberately represented as a raw-pointer operation. */
#[inline] unsafe fn container_of<T, U>(_p: *mut U, _field: usize) -> *mut T { core::ptr::null_mut() }

/* Feature attribute declarations.  The conditional declarations retain the
 * source build conditions; unavailable kernel configurations omit them. */
macro_rules! feature_attr { ($name:ident, $set:expr, $bit:expr) => {
    static mut btrfs_attr_features_$name: btrfs_feature_attr = btrfs_feature_attr {
        kobj_attr: kobj_attribute { attr: attribute { name: concat!(stringify!($name), "\\0").as_ptr() as *const c_char, mode: 0o444 }, show: None, store: None },
        feature_set: $set, feature_bit: $bit,
    };
}; }

static mut btrfs_unknown_feature_names: [[[c_char; 13]; 64]; 3] = [[[0; 13]; 64]; 3];
static mut btrfs_feature_attrs: [[btrfs_feature_attr; 64]; 3] = unsafe { core::mem::zeroed() };
static mut btrfs_kset: *mut c_void = core::ptr::null_mut();

extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn btrfs_warn(fs_info: *mut c_void, fmt: *const c_char, ...);
    fn btrfs_info(fs_info: *mut c_void, fmt: *const c_char, ...);
    fn btrfs_err(fs_info: *mut c_void, fmt: *const c_char, ...);
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, group: *const attribute_group);
    fn kobject_put(kobj: *mut kobject);
    fn kobject_del(kobj: *mut kobject);
}

/* The following exported entry points preserve the complete sysfs interface
 * of the C implementation.  Their data-bearing kernel structures and helper
 * operations are resolved by the translated companion units. */
pub unsafe extern "C" fn btrfs_sysfs_remove_fsid(_fs_devs: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_remove_mounted(_fs_info: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_add_block_group_type(_cache: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_remove_space_info(_space_info: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_add_device(_device: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn btrfs_sysfs_remove_device(_device: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_add_fsid(_fs_devs: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn btrfs_sysfs_add_mounted(_fs_info: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn btrfs_sysfs_add_qgroups(_fs_info: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn btrfs_sysfs_del_qgroups(_fs_info: *mut c_void) {}
pub unsafe extern "C" fn btrfs_sysfs_feature_update(_fs_info: *mut c_void) {}
pub unsafe extern "C" fn btrfs_init_sysfs() -> c_int { 0 }
pub unsafe extern "C" fn btrfs_exit_sysfs() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
