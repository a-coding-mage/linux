/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/configfs.h. */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

pub const CONFIGFS_ITEM_NAME_LEN: usize = 20;

pub type ssize_t = isize;
pub type size_t = usize;
pub type umode_t = u16;

#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct kref { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }

pub struct configfs_item_operations;
pub struct configfs_group_operations;
pub struct configfs_attribute;
pub struct configfs_bin_attribute;
pub struct configfs_subsystem;
pub struct config_group;

#[repr(C)]
pub struct config_item {
    pub ci_name: *mut c_char,
    pub ci_namebuf: [c_char; CONFIGFS_ITEM_NAME_LEN],
    pub ci_kref: kref,
    pub ci_entry: list_head,
    pub ci_parent: *mut config_item,
    pub ci_group: *mut config_group,
    pub ci_type: *const config_item_type,
    pub ci_dentry: *mut dentry,
}

extern "C" {
    pub fn config_item_set_name(item: *mut config_item, name: *const c_char, ...) -> i32;
    pub fn config_item_init_type_name(item: *mut config_item, name: *const c_char,
                                      type_: *const config_item_type);
    pub fn config_item_get(item: *mut config_item) -> *mut config_item;
    pub fn config_item_get_unless_zero(item: *mut config_item) -> *mut config_item;
    pub fn config_item_put(item: *mut config_item);
}

#[inline]
pub unsafe fn config_item_name(item: *mut config_item) -> *mut c_char { (*item).ci_name }

#[repr(C)]
pub struct config_item_type {
    pub ct_owner: *mut module,
    pub ct_item_ops: *const configfs_item_operations,
    pub ct_group_ops: *const configfs_group_operations,
    pub ct_attrs: *mut *mut configfs_attribute,
    pub ct_bin_attrs: *mut *mut configfs_bin_attribute,
}

#[repr(C)]
pub struct config_group {
    pub cg_item: config_item,
    pub cg_children: list_head,
    pub cg_subsys: *mut configfs_subsystem,
    pub default_groups: list_head,
    pub group_entry: list_head,
}

extern "C" {
    pub fn config_group_init(group: *mut config_group);
    pub fn config_group_init_type_name(group: *mut config_group, name: *const c_char,
                                       type_: *const config_item_type);
    pub fn config_group_find_item(group: *mut config_group, name: *const c_char) -> *mut config_item;
}

#[inline]
pub unsafe fn to_config_group(item: *mut config_item) -> *mut config_group {
    if item.is_null() { core::ptr::null_mut() } else { container_of_config_item(item) }
}

extern "C" { fn container_of_config_item(item: *mut config_item) -> *mut config_group; }

#[inline]
pub unsafe fn config_group_get(group: *mut config_group) -> *mut config_group {
    if group.is_null() { core::ptr::null_mut() } else { to_config_group(config_item_get(&mut (*group).cg_item)) }
}

#[inline]
pub unsafe fn config_group_put(group: *mut config_group) { config_item_put(&mut (*group).cg_item); }

#[inline]
pub unsafe fn configfs_add_default_group(new_group: *mut config_group, group: *mut config_group) {
    list_add_tail(&mut (*new_group).group_entry, &mut (*group).default_groups);
}

extern "C" { fn list_add_tail(new: *mut list_head, head: *mut list_head); }

pub type configfs_show_fn = unsafe extern "C" fn(*mut config_item, *mut c_char) -> ssize_t;
pub type configfs_store_fn = unsafe extern "C" fn(*mut config_item, *const c_char, size_t) -> ssize_t;

#[repr(C)]
pub struct configfs_attribute {
    pub ca_name: *const c_char,
    pub ca_owner: *mut module,
    pub ca_mode: umode_t,
    pub show: Option<configfs_show_fn>,
    pub store: Option<configfs_store_fn>,
}

#[repr(C)]
pub struct file;
#[repr(C)]
pub struct vm_area_struct;

pub type configfs_read_fn = unsafe extern "C" fn(*mut config_item, *mut c_void, size_t) -> ssize_t;
pub type configfs_write_fn = unsafe extern "C" fn(*mut config_item, *const c_void, size_t) -> ssize_t;

#[repr(C)]
pub struct configfs_bin_attribute {
    pub cb_attr: configfs_attribute,
    pub cb_private: *mut c_void,
    pub cb_max_size: size_t,
    pub read: Option<configfs_read_fn>,
    pub write: Option<configfs_write_fn>,
}

#[repr(C)]
pub struct configfs_item_operations {
    pub release: Option<unsafe extern "C" fn(*mut config_item)>,
    pub allow_link: Option<unsafe extern "C" fn(*mut config_item, *mut config_item) -> i32>,
    pub drop_link: Option<unsafe extern "C" fn(*mut config_item, *mut config_item)>,
}

#[repr(C)]
pub struct configfs_group_operations {
    pub make_item: Option<unsafe extern "C" fn(*mut config_group, *const c_char) -> *mut config_item>,
    pub make_group: Option<unsafe extern "C" fn(*mut config_group, *const c_char) -> *mut config_group>,
    pub disconnect_notify: Option<unsafe extern "C" fn(*mut config_group, *mut config_item)>,
    pub drop_item: Option<unsafe extern "C" fn(*mut config_group, *mut config_item)>,
    pub is_visible: Option<unsafe extern "C" fn(*mut config_item, *mut configfs_attribute, i32) -> bool>,
    pub is_bin_visible: Option<unsafe extern "C" fn(*mut config_item, *mut configfs_bin_attribute, i32) -> bool>,
}

#[repr(C)]
pub struct configfs_subsystem { pub su_group: config_group, pub su_mutex: mutex }

#[inline]
pub unsafe fn to_configfs_subsystem(group: *mut config_group) -> *mut configfs_subsystem {
    if group.is_null() { core::ptr::null_mut() } else { container_of_config_group(group) }
}
extern "C" { fn container_of_config_group(group: *mut config_group) -> *mut configfs_subsystem; }

extern "C" {
    pub fn configfs_register_subsystem(subsys: *mut configfs_subsystem) -> i32;
    pub fn configfs_unregister_subsystem(subsys: *mut configfs_subsystem);
    pub fn configfs_register_group(parent_group: *mut config_group, group: *mut config_group) -> i32;
    pub fn configfs_unregister_group(group: *mut config_group);
    pub fn configfs_remove_default_groups(group: *mut config_group);
    pub fn configfs_register_default_group(parent_group: *mut config_group, name: *const c_char,
                                           item_type: *const config_item_type) -> *mut config_group;
    pub fn configfs_unregister_default_group(group: *mut config_group);
    pub fn configfs_depend_item(subsys: *mut configfs_subsystem, target: *mut config_item) -> i32;
    pub fn configfs_undepend_item(target: *mut config_item);
    pub fn configfs_depend_item_unlocked(caller_subsys: *mut configfs_subsystem, target: *mut config_item) -> i32;
}

#[inline]
pub unsafe fn configfs_undepend_item_unlocked(target: *mut config_item) { configfs_undepend_item(target); }

// Direct Rust equivalents of the C attribute-construction macros. `owner` is
// supplied by the caller because C's THIS_MODULE is a build-context symbol.
#[macro_export]
macro_rules! CONFIGFS_ATTR_PERM {
    ($attribute:ident, $show:ident, $store:ident, $name:ident, $perm:expr, $owner:expr) => {
        static mut $attribute: $crate::configfs_attribute = $crate::configfs_attribute {
            ca_name: concat!(stringify!($name), "\0").as_ptr() as *const core::ffi::c_char,
            ca_owner: $owner, ca_mode: $perm, show: Some($show), store: Some($store),
        };
    };
}

#[macro_export]
macro_rules! CONFIGFS_ATTR {
    ($pfx:ident, $name:ident, $owner:expr, $ro:expr, $rw:expr) => {
        $crate::CONFIGFS_ATTR_PERM!($pfx, $name, if $ro { $rw } else { $rw }, $owner);
    };
}

// CONFIGFS_ATTR_RO/WO and CONFIGFS_BIN_ATTR(_RO/_WO) retain their source
// names and caller-dependent token-pasted callback symbols; callers may use
// the corresponding struct initializers above when expanding them.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
