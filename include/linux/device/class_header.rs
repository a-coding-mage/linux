// SPDX-License-Identifier: GPL-2.0
/* The class-specific portions of the driver model. */
// C dependencies are supplied by other translated headers.

use core::ffi::c_char;

#[repr(C)]
pub struct class {
    pub name: *const c_char,
    pub class_groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub dev_uevent: Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> i32>,
    pub devnode: Option<unsafe extern "C" fn(*const device, *mut umode_t) -> *mut c_char>,
    pub class_release: Option<unsafe extern "C" fn(*const class)>,
    pub dev_release: Option<unsafe extern "C" fn(*mut device)>,
    pub shutdown_pre: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub ns_type: *const kobj_ns_type_operations,
    pub namespace: Option<unsafe extern "C" fn(*const device) -> *const ns_common>,
    pub get_ownership: Option<unsafe extern "C" fn(*const device, *mut kuid_t, *mut kgid_t)>,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct class_dev_iter {
    pub ki: klist_iter,
    pub type_: *const device_type,
    pub sp: *mut subsys_private,
}

extern "C" {
    pub fn class_register(class: *const class) -> i32;
    pub fn class_unregister(class: *const class);
    pub fn class_is_registered(class: *const class) -> bool;

    pub fn class_compat_register(name: *const c_char) -> *mut class_compat;
    pub fn class_compat_unregister(cls: *mut class_compat);
    pub fn class_compat_create_link(cls: *mut class_compat, dev: *mut device) -> i32;
    pub fn class_compat_remove_link(cls: *mut class_compat, dev: *mut device);

    pub fn class_dev_iter_init(
        iter: *mut class_dev_iter,
        class: *const class,
        start: *const device,
        type_: *const device_type,
    );
    pub fn class_dev_iter_next(iter: *mut class_dev_iter) -> *mut device;
    pub fn class_dev_iter_exit(iter: *mut class_dev_iter);

    pub fn class_for_each_device(
        class: *const class,
        start: *const device,
        data: *mut core::ffi::c_void,
        fn_: device_iter_t,
    ) -> i32;
    pub fn class_find_device(
        class: *const class,
        start: *const device,
        data: *const core::ffi::c_void,
        r#match: device_match_t,
    ) -> *mut device;
}

pub unsafe fn class_find_device_by_name(class: *const class, name: *const c_char) -> *mut device {
    class_find_device(class, core::ptr::null(), name.cast(), device_match_name)
}

pub unsafe fn class_find_device_by_of_node(class: *const class, np: *const device_node) -> *mut device {
    class_find_device(class, core::ptr::null(), np.cast(), device_match_of_node)
}

pub unsafe fn class_find_device_by_fwnode(class: *const class, fwnode: *const fwnode_handle) -> *mut device {
    class_find_device(class, core::ptr::null(), fwnode.cast(), device_match_fwnode)
}

pub unsafe fn class_find_device_by_devt(class: *const class, devt: dev_t) -> *mut device {
    class_find_device(class, core::ptr::null(), &devt as *const dev_t as *const core::ffi::c_void, device_match_devt)
}

#[cfg(CONFIG_ACPI)]
pub unsafe fn class_find_device_by_acpi_dev(class: *const class, adev: *const acpi_device) -> *mut device {
    class_find_device(class, core::ptr::null(), adev.cast(), device_match_acpi_dev)
}

#[cfg(not(CONFIG_ACPI))]
pub unsafe fn class_find_device_by_acpi_dev(class: *const class, _adev: *const core::ffi::c_void) -> *mut device {
    core::ptr::null_mut()
}

#[repr(C)]
pub struct class_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*const class, *const class_attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*const class, *const class_attribute, *const c_char, usize) -> isize>,
}

macro_rules! CLASS_ATTR_RW { ($name:ident) => { pub static mut class_attr_$name: class_attribute = __ATTR_RW!($name); } }
macro_rules! CLASS_ATTR_RO { ($name:ident) => { pub static mut class_attr_$name: class_attribute = __ATTR_RO!($name); } }
macro_rules! CLASS_ATTR_WO { ($name:ident) => { pub static mut class_attr_$name: class_attribute = __ATTR_WO!($name); } }

extern "C" {
    pub fn class_create_file_ns(class: *const class, attr: *const class_attribute, ns: *const ns_common) -> i32;
    pub fn class_remove_file_ns(class: *const class, attr: *const class_attribute, ns: *const ns_common);
}

pub unsafe fn class_create_file(class: *const class, attr: *const class_attribute) -> i32 {
    class_create_file_ns(class, attr, core::ptr::null())
}

pub unsafe fn class_remove_file(class: *const class, attr: *const class_attribute) {
    class_remove_file_ns(class, attr, core::ptr::null())
}

#[repr(C)]
pub struct class_attribute_string {
    pub attr: class_attribute,
    pub str_: *mut c_char,
}

macro_rules! _CLASS_ATTR_STRING { ($name:ident, $mode:expr, $str:expr) => { __ATTR!($name, $mode, show_class_attr_string, None) } }
macro_rules! CLASS_ATTR_STRING { ($name:ident, $mode:expr, $str:expr) => { pub static mut class_attr_$name: class_attribute_string = _CLASS_ATTR_STRING!($name, $mode, $str); } }

extern "C" {
    pub fn show_class_attr_string(class: *const class, attr: *const class_attribute, buf: *mut c_char) -> isize;
}

#[repr(C)]
pub struct class_interface {
    pub node: list_head,
    pub class: *const class,
    pub add_dev: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub remove_dev: Option<unsafe extern "C" fn(*mut device)>,
}

extern "C" {
    pub fn class_interface_register(interface: *mut class_interface) -> i32;
    pub fn class_interface_unregister(interface: *mut class_interface);
    pub fn class_create(name: *const c_char) -> *mut class;
    pub fn class_destroy(cls: *const class);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
