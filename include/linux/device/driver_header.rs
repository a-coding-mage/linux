// SPDX-License-Identifier: GPL-2.0
/* The driver-specific portions of the driver model. */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust names.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum probe_type {
    PROBE_DEFAULT_STRATEGY,
    PROBE_PREFER_ASYNCHRONOUS,
    PROBE_FORCE_SYNCHRONOUS,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::std::os::raw::c_char,
    pub bus: *const bus_type,
    pub owner: *mut module,
    pub mod_name: *const ::std::os::raw::c_char,
    pub suppress_bind_attrs: bool,
    pub probe_type: probe_type,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
    pub probe: Option<unsafe extern "C" fn(dev: *mut device) -> ::std::os::raw::c_int>,
    pub sync_state: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut device) -> ::std::os::raw::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device, state: pm_message_t) -> ::std::os::raw::c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> ::std::os::raw::c_int>,
    pub groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub pm: *const dev_pm_ops,
    pub coredump: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub p: *mut driver_private,
    pub p_cb: device_driver_p_cb,
}

#[repr(C)]
pub struct device_driver_p_cb {
    // Called after remove() but before devres entries are released.
    // This is a Rust only callback.
    pub post_unbind_rust: Option<unsafe extern "C" fn(dev: *mut device)>,
}

#[repr(C)]
pub struct driver_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(driver: *mut device_driver, buf: *mut ::std::os::raw::c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(driver: *mut device_driver, buf: *const ::std::os::raw::c_char, count: usize) -> ssize_t>,
}

#[macro_export]
macro_rules! DRIVER_ATTR_RW {
    ($name:ident) => { driver_attribute { attr: __ATTR_RW!($name) } };
}
#[macro_export]
macro_rules! DRIVER_ATTR_RO {
    ($name:ident) => { driver_attribute { attr: __ATTR_RO!($name) } };
}
#[macro_export]
macro_rules! DRIVER_ATTR_WO {
    ($name:ident) => { driver_attribute { attr: __ATTR_WO!($name) } };
}

extern "C" {
    pub fn driver_register(drv: *mut device_driver) -> ::std::os::raw::c_int;
    pub fn driver_unregister(drv: *mut device_driver);
    pub fn driver_find(name: *const ::std::os::raw::c_char, bus: *const bus_type) -> *mut device_driver;
    pub fn driver_probe_done() -> bool;
    pub fn wait_for_device_probe();
    pub fn wait_for_init_devices_probe();
    pub fn driver_create_file(driver: *const device_driver, attr: *const driver_attribute) -> ::std::os::raw::c_int;
    pub fn driver_remove_file(driver: *const device_driver, attr: *const driver_attribute);
    pub fn driver_for_each_device(drv: *mut device_driver, start: *mut device, data: *mut ::std::ffi::c_void, fn_: device_iter_t) -> ::std::os::raw::c_int;
    pub fn driver_find_device(drv: *const device_driver, start: *mut device, data: *const ::std::ffi::c_void, mat: device_match_t) -> *mut device;
    pub fn driver_deferred_probe_add(dev: *mut device);
    pub fn driver_deferred_probe_check_state(dev: *mut device) -> ::std::os::raw::c_int;
    pub fn driver_init();
}

#[inline]
pub unsafe fn driver_find_device_by_name(drv: *const device_driver, name: *const ::std::os::raw::c_char) -> *mut device {
    driver_find_device(drv, core::ptr::null_mut(), name.cast(), device_match_name)
}
#[inline]
pub unsafe fn driver_find_device_by_of_node(drv: *const device_driver, np: *const device_node) -> *mut device {
    driver_find_device(drv, core::ptr::null_mut(), np.cast(), device_match_of_node)
}
#[inline]
pub unsafe fn driver_find_device_by_fwnode(drv: *mut device_driver, fwnode: *const fwnode_handle) -> *mut device {
    driver_find_device(drv, core::ptr::null_mut(), fwnode.cast(), device_match_fwnode)
}
#[inline]
pub unsafe fn driver_find_device_by_devt(drv: *const device_driver, devt: dev_t) -> *mut device {
    driver_find_device(drv, core::ptr::null_mut(), &devt as *const dev_t as *const _, device_match_devt)
}
#[inline]
pub unsafe fn driver_find_next_device(drv: *const device_driver, start: *mut device) -> *mut device {
    driver_find_device(drv, start, core::ptr::null(), device_match_any)
}

#[cfg(feature = "CONFIG_ACPI")]
#[inline]
pub unsafe fn driver_find_device_by_acpi_dev(drv: *const device_driver, adev: *const acpi_device) -> *mut device {
    driver_find_device(drv, core::ptr::null_mut(), adev.cast(), device_match_acpi_dev)
}
#[cfg(not(feature = "CONFIG_ACPI"))]
#[inline]
pub unsafe fn driver_find_device_by_acpi_dev(drv: *const device_driver, adev: *const ::std::ffi::c_void) -> *mut device {
    let _ = (drv, adev);
    core::ptr::null_mut()
}

// module_driver and builtin_driver are build-system registration helpers;
// their expansion is supplied by the surrounding kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
