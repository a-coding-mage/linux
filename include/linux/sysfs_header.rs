/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of sysfs.h. External kernel types and symbols are supplied by dependencies. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct attribute {
    pub name: *const c_char,
    pub mode: umode_t,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub ignore_lockdep: bool,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub key: *mut lock_class_key,
    #[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
    pub skey: lock_class_key,
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> umode_t>,
    pub is_visible_const: Option<unsafe extern "C" fn(*mut kobject, *const attribute, c_int) -> umode_t>,
    pub is_bin_visible: Option<unsafe extern "C" fn(*mut kobject, *const bin_attribute, c_int) -> umode_t>,
    pub bin_size: Option<unsafe extern "C" fn(*mut kobject, *const bin_attribute, c_int) -> usize>,
    pub attrs: *mut *mut attribute,
    pub attrs_const: *const *const attribute,
    pub bin_attrs: *const *const bin_attribute,
}

pub const SYSFS_PREALLOC: umode_t = 0o10000;
pub const SYSFS_GROUP_INVISIBLE: umode_t = 0o20000;

#[repr(C)]
pub struct bin_attribute {
    pub attr: attribute,
    pub size: usize,
    pub private: *mut c_void,
    pub f_mapping: Option<unsafe extern "C" fn() -> *mut address_space>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut kobject, *const bin_attribute, *mut c_char, loff_t, usize) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *mut kobject, *const bin_attribute, *mut c_char, loff_t, usize) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, *mut kobject, *const bin_attribute, loff_t, c_int) -> loff_t>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut kobject, *const bin_attribute, *mut vm_area_struct) -> c_int>,
}

#[repr(C)]
pub struct sysfs_ops {
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *const c_char, usize) -> ssize_t>,
}

#[cfg(feature = "CONFIG_DEBUG_LOCK_ALLOC")]
#[macro_export]
macro_rules! sysfs_attr_init { ($attr:expr) => {{ static mut __KEY: lock_class_key = lock_class_key {}; unsafe { (*$attr).key = &mut __KEY; } }}; }
#[cfg(not(feature = "CONFIG_DEBUG_LOCK_ALLOC"))]
#[macro_export]
macro_rules! sysfs_attr_init { ($attr:expr) => {{}}; }

#[macro_export] macro_rules! DEFINE_SYSFS_GROUP_VISIBLE { ($name:ident) => { unsafe fn sysfs_group_visible_$name(kobj: *mut kobject, attr: *mut attribute, n: c_int) -> umode_t { if n == 0 && !$name##_group_visible(kobj) { SYSFS_GROUP_INVISIBLE } else { $name##_attr_visible(kobj, attr, n) } } }; }
#[macro_export] macro_rules! DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE { ($name:ident) => { unsafe fn sysfs_group_visible_$name(kobj: *mut kobject, a: *mut attribute, n: c_int) -> umode_t { if n == 0 && !$name##_group_visible(kobj) { SYSFS_GROUP_INVISIBLE } else { (*a).mode } } }; }
#[macro_export] macro_rules! SYSFS_GROUP_VISIBLE { ($fn:ident) => { sysfs_group_visible_$fn }; }

#[cfg(feature = "CONFIG_SYSFS")]
extern "C" {
    pub fn sysfs_create_dir_ns(kobj: *mut kobject, ns: *const ns_common) -> c_int;
    pub fn sysfs_remove_dir(kobj: *mut kobject);
    pub fn sysfs_rename_dir_ns(kobj: *mut kobject, new_name: *const c_char, new_ns: *const ns_common) -> c_int;
    pub fn sysfs_move_dir_ns(kobj: *mut kobject, new_parent_kobj: *mut kobject, new_ns: *const ns_common) -> c_int;
    pub fn sysfs_create_mount_point(parent_kobj: *mut kobject, name: *const c_char) -> c_int;
    pub fn sysfs_remove_mount_point(parent_kobj: *mut kobject, name: *const c_char);
    pub fn sysfs_create_file_ns(kobj: *mut kobject, attr: *const attribute, ns: *const ns_common) -> c_int;
    pub fn sysfs_create_files(kobj: *mut kobject, attr: *const *const attribute) -> c_int;
    pub fn sysfs_chmod_file(kobj: *mut kobject, attr: *const attribute, mode: umode_t) -> c_int;
    pub fn sysfs_break_active_protection(kobj: *mut kobject, attr: *const attribute) -> *mut kernfs_node;
    pub fn sysfs_unbreak_active_protection(kn: *mut kernfs_node);
    pub fn sysfs_remove_file_ns(kobj: *mut kobject, attr: *const attribute, ns: *const ns_common);
    pub fn sysfs_remove_file_self(kobj: *mut kobject, attr: *const attribute) -> bool;
    pub fn sysfs_remove_files(kobj: *mut kobject, attr: *const *const attribute);
    pub fn sysfs_create_bin_file(kobj: *mut kobject, attr: *const bin_attribute) -> c_int;
    pub fn sysfs_remove_bin_file(kobj: *mut kobject, attr: *const bin_attribute);
    pub fn sysfs_create_link(kobj: *mut kobject, target: *mut kobject, name: *const c_char) -> c_int;
    pub fn sysfs_create_link_nowarn(kobj: *mut kobject, target: *mut kobject, name: *const c_char) -> c_int;
    pub fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
    pub fn sysfs_rename_link_ns(kobj: *mut kobject, target: *mut kobject, old_name: *const c_char, new_name: *const c_char, new_ns: *const ns_common) -> c_int;
    pub fn sysfs_delete_link(dir: *mut kobject, targ: *mut kobject, name: *const c_char);
    pub fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    pub fn sysfs_create_groups(kobj: *mut kobject, groups: *const *const attribute_group) -> c_int;
    pub fn sysfs_update_groups(kobj: *mut kobject, groups: *const *const attribute_group) -> c_int;
    pub fn sysfs_update_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    pub fn sysfs_remove_group(kobj: *mut kobject, grp: *const attribute_group);
    pub fn sysfs_remove_groups(kobj: *mut kobject, groups: *const *const attribute_group);
    pub fn sysfs_add_file_to_group(kobj: *mut kobject, attr: *const attribute, group: *const c_char) -> c_int;
    pub fn sysfs_remove_file_from_group(kobj: *mut kobject, attr: *const attribute, group: *const c_char);
    pub fn sysfs_merge_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    pub fn sysfs_unmerge_group(kobj: *mut kobject, grp: *const attribute_group);
    pub fn sysfs_add_link_to_group(kobj: *mut kobject, group_name: *const c_char, target: *mut kobject, link_name: *const c_char) -> c_int;
    pub fn sysfs_remove_link_from_group(kobj: *mut kobject, group_name: *const c_char, link_name: *const c_char);
    pub fn compat_only_sysfs_link_entry_to_kobj(kobj: *mut kobject, target_kobj: *mut kobject, target_name: *const c_char, symlink_name: *const c_char) -> c_int;
    pub fn sysfs_notify(kobj: *mut kobject, dir: *const c_char, attr: *const c_char);
    pub fn sysfs_init() -> c_int;
    pub fn sysfs_enable_ns(kn: *mut kernfs_node);
    pub fn sysfs_file_change_owner(kobj: *mut kobject, name: *const c_char, kuid: kuid_t, kgid: kgid_t) -> c_int;
    pub fn sysfs_change_owner(kobj: *mut kobject, kuid: kuid_t, kgid: kgid_t) -> c_int;
    pub fn sysfs_link_change_owner(kobj: *mut kobject, targ: *mut kobject, name: *const c_char, kuid: kuid_t, kgid: kgid_t) -> c_int;
    pub fn sysfs_groups_change_owner(kobj: *mut kobject, groups: *const *const attribute_group, kuid: kuid_t, kgid: kgid_t) -> c_int;
    pub fn sysfs_group_change_owner(kobj: *mut kobject, groups: *const attribute_group, kuid: kuid_t, kgid: kgid_t) -> c_int;
    pub fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    pub fn sysfs_emit_at(buf: *mut c_char, at: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn sysfs_bin_attr_simple_read(file: *mut file, kobj: *mut kobject, attr: *const bin_attribute, buf: *mut c_char, off: loff_t, count: usize) -> ssize_t;
}

#[cfg(not(feature = "CONFIG_SYSFS"))]
macro_rules! sysfs_stub { ($($name:ident($($arg:ident : $ty:ty),*) -> $ret:ty),* $(,)?) => { $(unsafe fn $name($($arg:$ty),*) -> $ret { 0 as $ret })* }; }

pub unsafe fn sysfs_create_file(kobj: *mut kobject, attr: *const attribute) -> c_int { sysfs_create_file_ns(kobj, attr, core::ptr::null()) }
pub unsafe fn sysfs_remove_file(kobj: *mut kobject, attr: *const attribute) { sysfs_remove_file_ns(kobj, attr, core::ptr::null()) }
pub unsafe fn sysfs_rename_link(kobj: *mut kobject, target: *mut kobject, old_name: *const c_char, new_name: *const c_char) -> c_int { sysfs_rename_link_ns(kobj, target, old_name, new_name, core::ptr::null()) }

pub unsafe fn sysfs_notify_dirent(kn: *mut kernfs_node) { kernfs_notify(kn); }
pub unsafe fn sysfs_get_dirent(parent: *mut kernfs_node, name: *const c_char) -> *mut kernfs_node { kernfs_find_and_get(parent, name) }
pub unsafe fn sysfs_get(kn: *mut kernfs_node) -> *mut kernfs_node { kernfs_get(kn); kn }
pub unsafe fn sysfs_put(kn: *mut kernfs_node) { kernfs_put(kn); }

/* Permissions on a sysfs file. */
#[macro_export]
macro_rules! VERIFY_OCTAL_PERMISSIONS { ($perms:expr) => { $perms }; }

/* External kernel declarations. */
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct ns_common { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
pub type umode_t = u16; pub type ssize_t = isize; pub type loff_t = i64; pub type kuid_t = u32; pub type kgid_t = u32;
extern "C" { fn kernfs_notify(kn: *mut kernfs_node); fn kernfs_find_and_get(parent: *mut kernfs_node, name: *const c_char) -> *mut kernfs_node; fn kernfs_get(kn: *mut kernfs_node); fn kernfs_put(kn: *mut kernfs_node); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
