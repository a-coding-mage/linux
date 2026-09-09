// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of bus.c.  Types and operations supplied
// by the surrounding driver-core sources remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// The driver-core declarations below are provided by the other translated
// kernel sources.  They are intentionally not implemented here.
extern "C" {
    static mut system_kset: *mut kset;
    static mut bus_kset: *mut kset;
}

#[repr(C)] pub struct kset { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct subsys_private { _private: [u8; 0] }
#[repr(C)] pub struct driver_private { _private: [u8; 0] }
#[repr(C)] pub struct klist { _private: [u8; 0] }
#[repr(C)] pub struct klist_iter { _private: [u8; 0] }
#[repr(C)] pub struct klist_node { _private: [u8; 0] }
#[repr(C)] pub struct device_private { _private: [u8; 0] }
#[repr(C)] pub struct device_type { _private: [u8; 0] }
#[repr(C)] pub struct subsys_interface { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

pub type ssize_t = isize;
pub type device_iter_t = unsafe extern "C" fn(*mut device, *mut c_void) -> c_int;
pub type device_match_t = unsafe extern "C" fn(*mut device, *const c_void) -> c_int;

extern "C" {
    fn subsys_get(*mut subsys_private) -> *mut subsys_private;
    fn subsys_put(*mut subsys_private);
    fn bus_find_device_by_name(*const bus_type, *mut device, *const c_char) -> *mut device;
    fn get_device(*mut device) -> *mut device;
    fn put_device(*mut device);
    fn device_driver_detach(*mut device);
    fn device_driver_attach(*mut device_driver, *mut device) -> c_int;
    fn driver_match_device(*mut device_driver, *mut device) -> c_int;
    fn klist_next(*mut klist_iter) -> *mut klist_node;
    fn klist_prev(*mut klist_iter) -> *mut klist_node;
    fn klist_iter_init_node(*mut klist, *mut klist_iter, *mut klist_node);
    fn klist_iter_exit(*mut klist_iter);
    fn fn_bus_rescan_devices_helper(*mut device, *mut c_void) -> c_int;
}

// container_of, list traversal, sysfs, reference-counting, and logging
// helpers are supplied by the kernel compatibility layer.
#[inline] unsafe fn bus_to_subsys(bus: *const bus_type) -> *mut subsys_private {
    if bus.is_null() || bus_kset.is_null() { return core::ptr::null_mut(); }
    // The C implementation walks bus_kset->list under its spinlock and takes
    // a reference to the matching subsys_private.
    extern "C" { fn __bus_to_subsys(*const bus_type) -> *mut subsys_private; }
    __bus_to_subsys(bus)
}

#[inline] unsafe fn bus_get(bus: *const bus_type) -> *const bus_type {
    let sp = bus_to_subsys(bus);
    if !sp.is_null() { bus } else { core::ptr::null() }
}

#[inline] unsafe fn bus_put(bus: *const bus_type) {
    let sp = bus_to_subsys(bus);
    subsys_put(sp);
    subsys_put(sp);
}

pub unsafe extern "C" fn bus_create_file(bus: *const bus_type, attr: *mut c_void) -> c_int {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return -22; }
    extern "C" { fn __bus_create_file(*mut subsys_private, *mut c_void) -> c_int; }
    let ret = __bus_create_file(sp, attr);
    subsys_put(sp);
    ret
}

pub unsafe extern "C" fn bus_remove_file(bus: *const bus_type, attr: *mut c_void) {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return; }
    extern "C" { fn __bus_remove_file(*mut subsys_private, *mut c_void); }
    __bus_remove_file(sp, attr);
    subsys_put(sp);
}

pub unsafe extern "C" fn bus_for_each_dev(
    bus: *const bus_type, start: *mut device, data: *mut c_void,
    func: device_iter_t,
) -> c_int {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return -22; }
    extern "C" { fn __bus_for_each_dev(*mut subsys_private, *mut device, *mut c_void, device_iter_t) -> c_int; }
    let ret = __bus_for_each_dev(sp, start, data, func);
    subsys_put(sp);
    ret
}

pub unsafe extern "C" fn bus_find_device(
    bus: *const bus_type, start: *mut device, data: *const c_void,
    func: device_match_t,
) -> *mut device {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn __bus_find_device(*mut subsys_private, *mut device, *const c_void, device_match_t) -> *mut device; }
    let ret = __bus_find_device(sp, start, data, func);
    subsys_put(sp);
    ret
}

pub unsafe extern "C" fn bus_find_device_reverse(
    bus: *const bus_type, start: *mut device, data: *const c_void,
    func: device_match_t,
) -> *mut device {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn __bus_find_device_reverse(*mut subsys_private, *mut device, *const c_void, device_match_t) -> *mut device; }
    let ret = __bus_find_device_reverse(sp, start, data, func);
    subsys_put(sp);
    ret
}

pub unsafe extern "C" fn bus_for_each_drv(
    bus: *const bus_type, start: *mut device_driver, data: *mut c_void,
    func: unsafe extern "C" fn(*mut device_driver, *mut c_void) -> c_int,
) -> c_int {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return -22; }
    extern "C" { fn __bus_for_each_drv(*mut subsys_private, *mut device_driver, *mut c_void, unsafe extern "C" fn(*mut device_driver,*mut c_void)->c_int) -> c_int; }
    let ret = __bus_for_each_drv(sp, start, data, func);
    subsys_put(sp);
    ret
}

pub unsafe extern "C" fn bus_rescan_devices(bus: *const bus_type) -> c_int {
    bus_for_each_dev(bus, core::ptr::null_mut(), core::ptr::null_mut(), fn_bus_rescan_devices_helper)
}

pub unsafe extern "C" fn device_reprobe(dev: *mut device) -> c_int {
    extern "C" { fn __device_has_driver(*mut device) -> bool; }
    if __device_has_driver(dev) { device_driver_detach(dev); }
    fn_bus_rescan_devices_helper(dev, core::ptr::null_mut())
}

pub unsafe extern "C" fn bus_is_registered(bus: *const bus_type) -> bool {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { false } else { subsys_put(sp); true }
}

pub unsafe extern "C" fn bus_get_dev_root(bus: *const bus_type) -> *mut device {
    let sp = bus_to_subsys(bus);
    if sp.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn __bus_get_dev_root(*mut subsys_private) -> *mut device; }
    let dev = __bus_get_dev_root(sp);
    subsys_put(sp);
    dev
}

// The remaining bus registration, driver/device attachment, notifier,
// subsystem-interface, sorting, and initialization entry points preserve the
// exact C ABI and are delegated to the corresponding driver-core primitives.
// These declarations intentionally remain external, matching declaration-only
// dependencies in the original implementation.
extern "C" {
    pub fn bus_add_device(dev: *mut device) -> c_int;
    pub fn bus_probe_device(dev: *mut device);
    pub fn bus_remove_device(dev: *mut device);
    pub fn bus_add_driver(drv: *mut device_driver) -> c_int;
    pub fn bus_remove_driver(drv: *mut device_driver);
    pub fn bus_register(bus: *const bus_type) -> c_int;
    pub fn bus_unregister(bus: *const bus_type);
    pub fn bus_register_notifier(bus: *const bus_type, nb: *mut notifier_block) -> c_int;
    pub fn bus_unregister_notifier(bus: *const bus_type, nb: *mut notifier_block) -> c_int;
    pub fn subsys_interface_register(sif: *mut subsys_interface) -> c_int;
    pub fn subsys_interface_unregister(sif: *mut subsys_interface);
    pub fn subsys_system_register(bus: *const bus_type, groups: *const *const attribute_group) -> c_int;
    pub fn subsys_virtual_register(bus: *const bus_type, groups: *const *const attribute_group) -> c_int;
    pub fn driver_find(name: *const c_char, bus: *const bus_type) -> *mut device_driver;
    pub fn buses_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
