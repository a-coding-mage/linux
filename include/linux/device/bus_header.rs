// SPDX-License-Identifier: GPL-2.0
/*
 * bus.h - the bus-specific portions of the driver model
 *
 * Rust translation of the source header.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct bus_type {
    pub name: *const ::std::os::raw::c_char,
    pub dev_name: *const ::std::os::raw::c_char,
    pub bus_groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub drv_groups: *const *const attribute_group,

    pub match_: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> ::std::os::raw::c_int>,
    pub uevent: Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> ::std::os::raw::c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub sync_state: Option<unsafe extern "C" fn(*mut device)>,
    pub remove: Option<unsafe extern "C" fn(*mut device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut device)>,
    pub irq_get_affinity: Option<unsafe extern "C" fn(*mut device, ::std::os::raw::c_uint) -> *const cpumask>,

    pub online: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub offline: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device, pm_message_t) -> ::std::os::raw::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub num_vf: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub dma_configure: Option<unsafe extern "C" fn(*mut device) -> ::std::os::raw::c_int>,
    pub dma_cleanup: Option<unsafe extern "C" fn(*mut device)>,
    pub pm: *const dev_pm_ops,
    pub driver_override: bool,
    pub need_parent_lock: bool,
}

extern "C" {
    pub fn bus_register(bus: *const bus_type) -> ::std::os::raw::c_int;
    pub fn bus_unregister(bus: *const bus_type);
    pub fn bus_rescan_devices(bus: *const bus_type) -> ::std::os::raw::c_int;
}

#[repr(C)]
pub struct bus_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*const bus_type, *mut ::std::os::raw::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*const bus_type, *const ::std::os::raw::c_char, usize) -> isize>,
}

#[macro_export]
macro_rules! BUS_ATTR_RW { ($name:ident) => { bus_attribute { attr: __ATTR_RW!($name), show: None, store: None } }; }
#[macro_export]
macro_rules! BUS_ATTR_RO { ($name:ident) => { bus_attribute { attr: __ATTR_RO!($name), show: None, store: None } }; }
#[macro_export]
macro_rules! BUS_ATTR_WO { ($name:ident) => { bus_attribute { attr: __ATTR_WO!($name), show: None, store: None } }; }

extern "C" {
    pub fn bus_create_file(bus: *const bus_type, attr: *mut bus_attribute) -> ::std::os::raw::c_int;
    pub fn bus_remove_file(bus: *const bus_type, attr: *mut bus_attribute);
}

pub type device_match_t = unsafe extern "C" fn(*mut device, *const ::std::ffi::c_void) -> ::std::os::raw::c_int;

extern "C" {
    pub fn device_match_name(dev: *mut device, name: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_type(dev: *mut device, type_: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_of_node(dev: *mut device, np: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_fwnode(dev: *mut device, fwnode: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_devt(dev: *mut device, pdevt: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_acpi_dev(dev: *mut device, adev: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_acpi_handle(dev: *mut device, handle: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn device_match_any(dev: *mut device, unused: *const ::std::ffi::c_void) -> ::std::os::raw::c_int;
}

pub type device_iter_t = unsafe extern "C" fn(*mut device, *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;

extern "C" {
    pub fn bus_for_each_dev(bus: *const bus_type, start: *mut device, data: *mut ::std::ffi::c_void, fn_: device_iter_t) -> ::std::os::raw::c_int;
    pub fn bus_find_device(bus: *const bus_type, start: *mut device, data: *const ::std::ffi::c_void, r#match: device_match_t) -> *mut device;
    pub fn bus_find_device_reverse(bus: *const bus_type, start: *mut device, data: *const ::std::ffi::c_void, r#match: device_match_t) -> *mut device;
}

pub unsafe fn bus_find_device_by_name(bus: *const bus_type, start: *mut device, name: *const ::std::os::raw::c_char) -> *mut device {
    bus_find_device(bus, start, name.cast(), device_match_name)
}

pub unsafe fn bus_find_device_by_of_node(bus: *const bus_type, np: *const device_node) -> *mut device {
    bus_find_device(bus, ::std::ptr::null_mut(), np.cast(), device_match_of_node)
}

pub unsafe fn bus_find_device_by_fwnode(bus: *const bus_type, fwnode: *const fwnode_handle) -> *mut device {
    bus_find_device(bus, ::std::ptr::null_mut(), fwnode.cast(), device_match_fwnode)
}

pub unsafe fn bus_find_device_by_devt(bus: *const bus_type, devt: dev_t) -> *mut device {
    bus_find_device(bus, ::std::ptr::null_mut(), &devt as *const dev_t as *const _, device_match_devt)
}

pub unsafe fn bus_find_next_device(bus: *const bus_type, cur: *mut device) -> *mut device {
    bus_find_device(bus, cur, ::std::ptr::null(), device_match_any)
}

extern "C" {
    pub fn bus_find_device_by_acpi_dev(bus: *const bus_type, adev: *const acpi_device) -> *mut device;
    pub fn bus_for_each_drv(bus: *const bus_type, start: *mut device_driver, data: *mut ::std::ffi::c_void, fn_: unsafe extern "C" fn(*mut device_driver, *mut ::std::ffi::c_void) -> ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn bus_sort_breadthfirst(bus: *const bus_type, compare: unsafe extern "C" fn(*const device, *const device) -> ::std::os::raw::c_int);
    pub fn bus_register_notifier(bus: *const bus_type, nb: *mut notifier_block) -> ::std::os::raw::c_int;
    pub fn bus_unregister_notifier(bus: *const bus_type, nb: *mut notifier_block) -> ::std::os::raw::c_int;
    pub fn bus_get_kset(bus: *const bus_type) -> *mut kset;
    pub fn bus_get_dev_root(bus: *const bus_type) -> *mut device;
}

#[repr(C)]
pub enum bus_notifier_event {
    BUS_NOTIFY_ADD_DEVICE,
    BUS_NOTIFY_DEL_DEVICE,
    BUS_NOTIFY_REMOVED_DEVICE,
    BUS_NOTIFY_BIND_DRIVER,
    BUS_NOTIFY_BOUND_DRIVER,
    BUS_NOTIFY_UNBIND_DRIVER,
    BUS_NOTIFY_UNBOUND_DRIVER,
    BUS_NOTIFY_DRIVER_NOT_BOUND,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
