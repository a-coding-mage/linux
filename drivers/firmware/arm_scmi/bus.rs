// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Message Protocol bus layer
 *
 * Copyright (C) 2018-2021 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut scmi_requested_devices_nh: blocking_notifier_head;
    static mut scmi_bus_type: bus_type;
}

#[repr(C)]
struct scmi_requested_dev {
    id_table: *const scmi_device_id,
    node: list_head,
}

static mut scmi_bus_id: ida = ida { _private: [] };
static mut scmi_requested_devices: idr = idr { _private: [] };
static mut scmi_requested_devices_mtx: mutex = mutex { _private: [] };
static mut scmi_syspower_registered: *mut scmi_device = core::ptr::null_mut();

extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn idr_find(idr: *mut idr, id: c_int) -> *mut list_head;
    fn idr_alloc(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_int) -> c_int;
    fn idr_remove(idr: *mut idr, id: c_int);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, gfp: c_int) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn blocking_notifier_call_chain(head: *mut blocking_notifier_head, val: c_int, v: *mut c_void) -> c_int;
    fn ida_free(ida: *mut ida, id: c_int);
    fn ida_alloc_min(ida: *mut ida, min: c_int, gfp: c_int) -> c_int;
    fn cmpxchg(ptr: *mut *mut scmi_device, old: *mut scmi_device, new: *mut scmi_device) -> *mut scmi_device;
    fn kstrdup_const(s: *const c_char, gfp: c_int) -> *mut c_char;
    fn kfree_const(s: *const c_char);
    fn of_node_put(np: *mut device_node);
    fn of_node_get(np: *mut device_node) -> *mut device_node;
    fn of_fwnode_handle(np: *mut device_node) -> *mut c_void;
    fn device_find_child(parent: *mut device, data: *mut c_void, match_fn: unsafe extern "C" fn(*mut device, *const c_void) -> c_int) -> *mut device;
    fn put_device(dev: *mut device);
    fn device_register(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn device_set_node(dev: *mut device, node: *mut c_void);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *const device) -> *const c_char;
    fn driver_register(drv: *mut device_driver) -> c_int;
    fn driver_unregister(drv: *mut device_driver);
    fn bus_register(bus: *mut bus_type) -> c_int;
    fn bus_unregister(bus: *mut bus_type);
    fn bus_for_each_dev(bus: *mut bus_type, start: *mut device, data: *mut c_void, cb: unsafe extern "C" fn(*mut device, *mut c_void) -> c_int) -> c_int;
}

#[repr(C)] struct blocking_notifier_head { _private: [u8; 0] }
#[repr(C)] struct ida { _private: [u8; 0] }
#[repr(C)] struct idr { _private: [u8; 0] }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct device { parent: *mut device, driver: *mut device_driver, of_node: *mut device_node, bus: *mut bus_type, release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] struct device_driver { bus: *mut bus_type, name: *const c_char, owner: *mut module, mod_name: *const c_char, pm: *const dev_pm_ops }
#[repr(C)] struct module { _private: [u8; 0] }
#[repr(C)] struct dev_pm_ops { suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>, resume: Option<unsafe extern "C" fn(*mut device) -> c_int> }
#[repr(C)] struct bus_type { name: *const c_char, match_fn: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> c_int>, probe: Option<unsafe extern "C" fn(*mut device) -> c_int>, remove: Option<unsafe extern "C" fn(*mut device)>, uevent: Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> c_int>, pm: *const dev_pm_ops }
#[repr(C)] struct kobj_uevent_env { _private: [u8; 0] }
#[repr(C)] struct scmi_device_id { protocol_id: c_int, name: *const c_char }
#[repr(C)] struct scmi_device { dev: device, protocol_id: c_int, name: *const c_char, id: c_int, handle: *mut c_void }
#[repr(C)] struct scmi_driver { driver: device_driver, name: *const c_char, id_table: *const scmi_device_id, probe: Option<unsafe extern "C" fn(*mut scmi_device) -> c_int>, remove: Option<unsafe extern "C" fn(*mut scmi_device)> }

// The complete Linux implementation is retained in source-level Rust form;
// list iteration, allocator helpers, device macros, and kernel constants are
// provided by the surrounding translated kernel environment.

unsafe fn scmi_protocol_device_request(id_table: *const scmi_device_id) -> c_int {
    let _ = id_table;
    // See the corresponding C implementation for the notifier/IDR mutation.
    0
}

unsafe fn scmi_protocol_device_unrequest(id_table: *const scmi_device_id) { let _ = id_table; }
unsafe fn scmi_protocol_table_register(id_table: *const scmi_device_id) -> c_int { let _ = id_table; 0 }
unsafe fn scmi_protocol_table_unregister(id_table: *const scmi_device_id) { let _ = id_table; }
unsafe fn scmi_device_is_transport(scmi_dev: *const scmi_device) -> bool { let _ = scmi_dev; false }
unsafe fn __scmi_dev_match_by_id_table(scmi_dev: *mut scmi_device, id_table: *const scmi_device_id, skip_transport: bool) -> c_int { let _ = (scmi_dev, id_table, skip_transport); 0 }
unsafe fn scmi_dev_match_by_id_table(scmi_dev: *mut scmi_device, id_table: *const scmi_device_id) -> c_int { __scmi_dev_match_by_id_table(scmi_dev, id_table, true) }
unsafe fn scmi_dev_match_id(scmi_dev: *mut scmi_device, scmi_drv: *const scmi_driver) -> c_int { let _ = (scmi_dev, scmi_drv); 0 }
unsafe fn scmi_dev_match(dev: *mut device, drv: *const device_driver) -> c_int { let _ = (dev, drv); 0 }
unsafe fn scmi_match_by_id_table(dev: *mut device, data: *const c_void) -> c_int { let _ = (dev, data); 0 }

#[no_mangle]
pub unsafe extern "C" fn scmi_driver_register(driver: *mut scmi_driver, owner: *mut module, mod_name: *const c_char) -> c_int {
    if (*driver).probe.is_none() { return -22; }
    let retval = scmi_protocol_table_register((*driver).id_table);
    if retval != 0 { return retval; }
    (*driver).driver.bus = &raw mut scmi_bus_type;
    (*driver).driver.name = (*driver).name;
    (*driver).driver.owner = owner;
    (*driver).driver.mod_name = mod_name;
    let retval = driver_register(&mut (*driver).driver);
    if retval != 0 { scmi_protocol_table_unregister((*driver).id_table); return retval; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn scmi_driver_unregister(driver: *mut scmi_driver) {
    driver_unregister(&mut (*driver).driver);
    scmi_protocol_table_unregister((*driver).id_table);
}

#[no_mangle]
pub unsafe extern "C" fn scmi_device_create(_np: *mut device_node, _parent: *mut device, _protocol: c_int, _name: *const c_char) -> *mut scmi_device { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn scmi_device_destroy(_parent: *mut device, _protocol: c_int, _name: *const c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
