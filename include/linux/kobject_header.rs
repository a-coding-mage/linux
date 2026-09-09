// SPDX-License-Identifier: GPL-2.0
/* Translated from kobject.h. */

pub const UEVENT_HELPER_PATH_LEN: usize = 256;
pub const UEVENT_NUM_ENVP: usize = 64;
pub const UEVENT_BUFFER_SIZE: usize = 2048;

// CONFIG_UEVENT_HELPER controls this declaration in the original header.
#[cfg(feature = "CONFIG_UEVENT_HELPER")]
unsafe extern "C" {
    pub static mut uevent_helper: [core::ffi::c_char; UEVENT_HELPER_PATH_LEN];
}

unsafe extern "C" {
    pub static mut uevent_seqnum: atomic64_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kobject_action {
    KOBJ_ADD,
    KOBJ_REMOVE,
    KOBJ_CHANGE,
    KOBJ_MOVE,
    KOBJ_ONLINE,
    KOBJ_OFFLINE,
    KOBJ_BIND,
    KOBJ_UNBIND,
}

#[repr(C)]
pub struct kobject {
    pub name: *const core::ffi::c_char,
    pub entry: list_head,
    pub parent: *mut kobject,
    pub kset: *mut kset,
    pub ktype: *const kobj_type,
    pub sd: *mut kernfs_node,
    pub kref: kref,
    pub state_initialized: u32,
    pub state_in_sysfs: u32,
    pub state_add_uevent_sent: u32,
    pub state_remove_uevent_sent: u32,
    pub uevent_suppress: u32,
    #[cfg(feature = "CONFIG_DEBUG_KOBJECT_RELEASE")]
    pub release: delayed_work,
}

unsafe extern "C" {
    pub fn kobject_set_name(kobj: *mut kobject, name: *const core::ffi::c_char, ...) -> i32;
    pub fn kobject_set_name_vargs(kobj: *mut kobject, fmt: *const core::ffi::c_char, vargs: va_list) -> i32;
}

#[inline]
pub unsafe fn kobject_name(kobj: *const kobject) -> *const core::ffi::c_char {
    (*kobj).name
}

unsafe extern "C" {
    pub fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type);
    pub fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn kobject_init_and_add(kobj: *mut kobject, ktype: *const kobj_type,
                                parent: *mut kobject, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn kobject_del(kobj: *mut kobject);
    pub fn kobject_create_and_add(name: *const core::ffi::c_char, parent: *mut kobject) -> *mut kobject;
    pub fn kobject_rename(kobj: *mut kobject, new_name: *const core::ffi::c_char) -> i32;
    pub fn kobject_move(kobj: *mut kobject, new_parent: *mut kobject) -> i32;
    pub fn kobject_get(kobj: *mut kobject) -> *mut kobject;
    pub fn kobject_get_unless_zero(kobj: *mut kobject) -> *mut kobject;
    pub fn kobject_put(kobj: *mut kobject);
    pub fn kobject_namespace(kobj: *const kobject) -> *const ns_common;
    pub fn kobject_get_ownership(kobj: *const kobject, uid: *mut kuid_t, gid: *mut kgid_t);
    pub fn kobject_get_path(kobj: *const kobject, flag: gfp_t) -> *mut core::ffi::c_char;
}

#[repr(C)]
pub struct kobj_type {
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
    pub sysfs_ops: *const sysfs_ops,
    pub default_groups: *const *const attribute_group,
    pub child_ns_type: Option<unsafe extern "C" fn(*const kobject) -> *const kobj_ns_type_operations>,
    pub namespace: Option<unsafe extern "C" fn(*const kobject) -> *const ns_common>,
    pub get_ownership: Option<unsafe extern "C" fn(*const kobject, *mut kuid_t, *mut kgid_t)>,
}

#[repr(C)]
pub struct kobj_uevent_env {
    pub argv: [*mut core::ffi::c_char; 3],
    pub envp: [*mut core::ffi::c_char; UEVENT_NUM_ENVP],
    pub envp_idx: i32,
    pub buf: [core::ffi::c_char; UEVENT_BUFFER_SIZE],
    pub buflen: i32,
}

#[repr(C)]
pub struct kset_uevent_ops {
    pub filter: Option<unsafe extern "C" fn(*const kobject) -> i32>,
    pub name: Option<unsafe extern "C" fn(*const kobject) -> *const core::ffi::c_char>,
    pub uevent: Option<unsafe extern "C" fn(*const kobject, *mut kobj_uevent_env) -> i32>,
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    // __SYSFS_FUNCTION_ALTERNATIVE selects the const/non-const callback form at build time.
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut core::ffi::c_char) -> ssize_t>,
    pub show_const: Option<unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *mut core::ffi::c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const core::ffi::c_char, usize) -> ssize_t>,
    pub store_const: Option<unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *const core::ffi::c_char, usize) -> ssize_t>,
}

pub type __kobj_show_handler_const = unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *mut core::ffi::c_char) -> ssize_t;
pub type __kobj_store_handler_const = unsafe extern "C" fn(*mut kobject, *const kobj_attribute, *const core::ffi::c_char, usize) -> ssize_t;

// __KOBJ_ATTR* macros retain their C initializer behavior and build-time CFI selection.

unsafe extern "C" {
    pub static kobj_sysfs_ops: sysfs_ops;
}

#[repr(C)]
pub struct kset {
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub kobj: kobject,
    pub uevent_ops: *const kset_uevent_ops,
}

unsafe extern "C" {
    pub fn kset_init(kset: *mut kset);
    pub fn kset_register(kset: *mut kset) -> i32;
    pub fn kset_unregister(kset: *mut kset);
    pub fn kset_create_and_add(name: *const core::ffi::c_char, u: *const kset_uevent_ops,
                               parent_kobj: *mut kobject) -> *mut kset;
}

#[inline]
pub unsafe fn to_kset(kobj: *mut kobject) -> *mut kset {
    if !kobj.is_null() { container_of!(kobj, kset, kobj) } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn kset_get(k: *mut kset) -> *mut kset {
    if !k.is_null() { to_kset(kobject_get(&mut (*k).kobj)) } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn kset_put(k: *mut kset) { kobject_put(&mut (*k).kobj); }

#[inline]
pub unsafe fn get_ktype(kobj: *const kobject) -> *const kobj_type { (*kobj).ktype }

unsafe extern "C" {
    pub fn kset_find_obj(kset: *mut kset, name: *const core::ffi::c_char) -> *mut kobject;
    pub static mut kernel_kobj: *mut kobject;
    pub static mut mm_kobj: *mut kobject;
    pub static mut hypervisor_kobj: *mut kobject;
    pub static mut power_kobj: *mut kobject;
    pub static mut firmware_kobj: *mut kobject;
    pub fn kobject_uevent(kobj: *mut kobject, action: kobject_action) -> i32;
    pub fn kobject_uevent_env(kobj: *mut kobject, action: kobject_action, envp: *mut *mut core::ffi::c_char) -> i32;
    pub fn kobject_synth_uevent(kobj: *mut kobject, buf: *const core::ffi::c_char, count: usize) -> i32;
    pub fn add_uevent_var(env: *mut kobj_uevent_env, format: *const core::ffi::c_char, ...) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
