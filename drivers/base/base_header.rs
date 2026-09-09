/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Core driver model functions and structures that should not be shared
 * outside of the drivers/base/ directory.
 *
 * Translated from the C header. Linux dependency types and functions are
 * supplied by other translation units.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct subsys_private {
    pub subsys: kset,
    pub devices_kset: *mut kset,
    pub interfaces: list_head,
    pub mutex: mutex,
    pub drivers_kset: *mut kset,
    pub klist_devices: klist,
    pub klist_drivers: klist,
    pub bus_notifier: blocking_notifier_head,
    pub drivers_autoprobe: u32,
    pub bus: *const bus_type,
    pub dev_root: *mut device,
    pub glue_dirs: kset,
    pub class: *const class,
    pub lock_key: lock_class_key,
}

#[inline]
pub unsafe fn subsys_get(sp: *mut subsys_private) -> *mut subsys_private {
    if !sp.is_null() { kset_get(&mut (*sp).subsys); }
    sp
}

#[inline]
pub unsafe fn subsys_put(sp: *mut subsys_private) {
    if !sp.is_null() { kset_put(&mut (*sp).subsys); }
}

pub unsafe extern "C" fn bus_to_subsys(bus: *const bus_type) -> *mut subsys_private;
pub unsafe extern "C" fn class_to_subsys(class: *const class) -> *mut subsys_private;

#[repr(C)]
pub struct driver_private {
    pub kobj: kobject,
    pub klist_devices: klist,
    pub knode_bus: klist_node,
    pub mkobj: *mut module_kobject,
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_private {
    pub klist_children: klist,
    pub knode_parent: klist_node,
    pub knode_driver: klist_node,
    pub knode_bus: klist_node,
    pub knode_class: klist_node,
    pub deferred_probe: list_head,
    pub async_driver: *const device_driver,
    pub deferred_probe_reason: *mut c_char,
    pub device: *mut device,
    pub dead: u8,
}

pub unsafe extern "C" fn devices_init() -> c_int;
pub unsafe extern "C" fn buses_init() -> c_int;
pub unsafe extern "C" fn classes_init() -> c_int;
pub unsafe extern "C" fn firmware_init() -> c_int;
#[cfg(CONFIG_SYS_HYPERVISOR)]
pub unsafe extern "C" fn hypervisor_init() -> c_int;
#[cfg(not(CONFIG_SYS_HYPERVISOR))]
#[inline] pub unsafe fn hypervisor_init() -> c_int { 0 }
pub unsafe extern "C" fn platform_bus_init() -> c_int;
pub unsafe extern "C" fn faux_bus_init() -> c_int;
pub unsafe extern "C" fn cpu_dev_init();
pub unsafe extern "C" fn container_dev_init();
#[cfg(CONFIG_AUXILIARY_BUS)]
pub unsafe extern "C" fn auxiliary_bus_init();
#[cfg(not(CONFIG_AUXILIARY_BUS))]
#[inline] pub unsafe fn auxiliary_bus_init() {}

pub unsafe extern "C" fn virtual_device_parent() -> *mut kobject;
pub unsafe extern "C" fn bus_add_device(dev: *mut device) -> c_int;
pub unsafe extern "C" fn bus_probe_device(dev: *mut device);
pub unsafe extern "C" fn bus_remove_device(dev: *mut device);
pub unsafe extern "C" fn bus_notify(dev: *mut device, value: bus_notifier_event);
pub unsafe extern "C" fn bus_is_registered(bus: *const bus_type) -> bool;
pub unsafe extern "C" fn bus_add_driver(drv: *mut device_driver) -> c_int;
pub unsafe extern "C" fn bus_remove_driver(drv: *mut device_driver);
pub unsafe extern "C" fn device_release_driver_internal(dev: *mut device, drv: *const device_driver, parent: *mut device);
pub unsafe extern "C" fn driver_detach(drv: *const device_driver);
pub unsafe extern "C" fn driver_deferred_probe_del(dev: *mut device);
pub unsafe extern "C" fn device_set_deferred_probe_reason(dev: *const device, vaf: *mut va_format);

#[inline]
pub unsafe fn driver_match_device(drv: *const device_driver, dev: *mut device) -> c_int {
    if !(*drv).bus.is_null() && (*(*drv).bus).r#match.is_some() { ((*(*drv).bus).r#match.unwrap())(dev, drv) } else { 1 }
}

#[inline]
pub unsafe fn dev_has_sync_state(dev: *mut device) -> bool {
    if dev.is_null() { return false; }
    let drv = (*dev).driver;
    if !drv.is_null() && (*drv).sync_state.is_some() { return true; }
    !(*dev).bus.is_null() && (*(*dev).bus).sync_state.is_some()
}

#[inline]
pub unsafe fn dev_sync_state(dev: *mut device) {
    if !(*dev).bus.is_null() && (*(*dev).bus).sync_state.is_some() { ((*(*dev).bus).sync_state.unwrap())(dev); }
    else if !(*dev).driver.is_null() && (*(*dev).driver).sync_state.is_some() { ((*(*dev).driver).sync_state.unwrap())(dev); }
}

pub unsafe extern "C" fn driver_add_groups(drv: *const device_driver, groups: *const *const attribute_group) -> c_int;
pub unsafe extern "C" fn driver_remove_groups(drv: *const device_driver, groups: *const *const attribute_group);
pub unsafe extern "C" fn device_driver_detach(dev: *mut device);

#[inline]
pub unsafe fn device_set_driver(dev: *mut device, drv: *const device_driver) { (*dev).driver = drv as *mut device_driver; }

pub struct devres_node;
pub type dr_node_release_t = unsafe extern "C" fn(*mut device, *mut devres_node);
pub type dr_node_free_t = unsafe extern "C" fn(*mut devres_node);

#[repr(C)]
pub struct devres_node {
    pub entry: list_head,
    pub release: Option<dr_node_release_t>,
    pub free_node: Option<dr_node_free_t>,
    pub name: *const c_char,
    pub size: usize,
}

pub unsafe extern "C" fn devres_node_init(node: *mut devres_node, release: dr_node_release_t, free_node: dr_node_free_t);
pub unsafe extern "C" fn devres_node_add(dev: *mut device, node: *mut devres_node);
pub unsafe extern "C" fn devres_node_remove(dev: *mut device, node: *mut devres_node) -> bool;
pub unsafe extern "C" fn devres_set_node_dbginfo(node: *mut devres_node, name: *const c_char, size: usize);
pub unsafe extern "C" fn devres_for_each_res(dev: *mut device, release: dr_release_t, match_: dr_match_t, match_data: *mut c_void, fn_: Option<unsafe extern "C" fn(*mut device, *mut c_void, *mut c_void)>, data: *mut c_void);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
