/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * platform_device.h - generic, centralized driver model
 *
 * Rust translation of the source header.
 */

pub const PLATFORM_DEVID_NONE: i32 = -1;
pub const PLATFORM_DEVID_AUTO: i32 = -2;

pub enum irq_affinity {}
pub enum mfd_cell {}
pub enum property_entry {}
pub enum device_node {}
pub enum fwnode_handle {}

#[repr(C)]
pub struct platform_device {
    pub name: *const ::core::ffi::c_char,
    pub id: i32,
    pub id_auto: bool,
    pub dev: device,
    pub platform_dma_mask: u64,
    pub dma_parms: device_dma_parameters,
    pub num_resources: u32,
    pub resource: *mut resource,
    pub id_entry: *const platform_device_id,
    pub mfd_cell: *mut mfd_cell,
    pub archdata: pdev_archdata,
}

#[inline]
pub unsafe fn platform_get_device_id(pdev: *mut platform_device) -> *const platform_device_id {
    (*pdev).id_entry
}

#[inline]
pub unsafe fn dev_is_platform(dev: *mut device) -> bool {
    (*dev).bus == &platform_bus_type as *const bus_type
}

/* container_of((x), struct platform_device, dev) */
#[inline]
pub unsafe fn to_platform_device(x: *mut device) -> *mut platform_device {
    (x as *mut u8).sub(core::mem::offset_of!(platform_device, dev)) as *mut platform_device
}

unsafe extern "C" {
    pub fn platform_device_register(pdev: *mut platform_device) -> i32;
    pub fn platform_device_unregister(pdev: *mut platform_device);
    pub static platform_bus_type: bus_type;
    pub static mut platform_bus: device;
    pub fn platform_get_resource(pdev: *mut platform_device, typ: u32, num: u32) -> *mut resource;
    pub fn platform_get_mem_or_io(pdev: *mut platform_device, num: u32) -> *mut resource;
    pub fn platform_find_device_by_driver(start: *mut device, drv: *const device_driver) -> *mut device;

    /* CONFIG_HAS_IOMEM controls whether these are external functions or error stubs. */
    pub fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device, index: u32, res: *mut *mut resource,
    ) -> *mut ::core::ffi::c_void;
    pub fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut ::core::ffi::c_void;
    pub fn devm_platform_ioremap_resource_byname(
        pdev: *mut platform_device, name: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;

    pub fn platform_get_irq(pdev: *mut platform_device, num: u32) -> i32;
    pub fn platform_get_irq_optional(pdev: *mut platform_device, num: u32) -> i32;
    pub fn platform_get_irq_affinity(
        pdev: *mut platform_device, num: u32, mask: *mut *const cpumask,
    ) -> i32;
    pub fn platform_irq_count(pdev: *mut platform_device) -> i32;
    pub fn devm_platform_get_irqs_affinity(
        dev: *mut platform_device, affd: *mut irq_affinity, minvec: u32,
        maxvec: u32, irqs: *mut *mut i32,
    ) -> i32;
    pub fn platform_get_resource_byname(
        pdev: *mut platform_device, typ: u32, name: *const ::core::ffi::c_char,
    ) -> *mut resource;
    pub fn platform_get_irq_byname(pdev: *mut platform_device, name: *const ::core::ffi::c_char) -> i32;
    pub fn platform_get_irq_byname_optional(pdev: *mut platform_device, name: *const ::core::ffi::c_char) -> i32;
    pub fn platform_add_devices(pdevs: *mut *mut platform_device, num: i32) -> i32;
}

#[repr(C)]
pub struct platform_device_info {
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub of_node_reused: bool,
    pub name: *const ::core::ffi::c_char,
    pub id: i32,
    pub res: *const resource,
    pub num_res: u32,
    pub data: *const ::core::ffi::c_void,
    pub size_data: usize,
    pub dma_mask: u64,
    pub swnode: *const software_node,
    pub properties: *const property_entry,
}

unsafe extern "C" {
    pub fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
}

#[inline]
pub unsafe fn platform_device_register_resndata(
    parent: *mut device, name: *const ::core::ffi::c_char, id: i32,
    res: *const resource, num: u32, data: *const ::core::ffi::c_void, size: usize,
) -> *mut platform_device {
    let info = platform_device_info { parent, fwnode: core::ptr::null_mut(), of_node_reused: false,
        name, id, res, num_res: num, data, size_data: size, dma_mask: 0,
        swnode: core::ptr::null(), properties: core::ptr::null() };
    platform_device_register_full(&info)
}

#[inline]
pub unsafe fn platform_device_register_simple(
    name: *const ::core::ffi::c_char, id: i32, res: *const resource, num: u32,
) -> *mut platform_device {
    platform_device_register_resndata(core::ptr::null_mut(), name, id, res, num, core::ptr::null(), 0)
}

#[inline]
pub unsafe fn platform_device_register_data(
    parent: *mut device, name: *const ::core::ffi::c_char, id: i32,
    data: *const ::core::ffi::c_void, size: usize,
) -> *mut platform_device {
    platform_device_register_resndata(parent, name, id, core::ptr::null(), 0, data, size)
}

unsafe extern "C" {
    pub fn platform_device_alloc(name: *const ::core::ffi::c_char, id: i32) -> *mut platform_device;
    pub fn platform_device_add_resources(pdev: *mut platform_device, res: *const resource, num: u32) -> i32;
    pub fn platform_device_add_data(pdev: *mut platform_device, data: *const ::core::ffi::c_void, size: usize) -> i32;
    pub fn platform_device_set_of_node(pdev: *mut platform_device, np: *mut device_node);
    pub fn platform_device_set_fwnode(pdev: *mut platform_device, fwnode: *mut fwnode_handle);
    pub fn platform_device_set_of_node_from_dev(pdev: *mut platform_device, dev2: *const device);
    pub fn platform_device_add(pdev: *mut platform_device) -> i32;
    pub fn platform_device_del(pdev: *mut platform_device);
    pub fn platform_device_put(pdev: *mut platform_device);
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut platform_device, pm_message_t) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
    pub prevent_deferred_probe: bool,
    pub driver_managed_dma: bool,
}

#[inline]
pub unsafe fn to_platform_driver(drv: *mut device_driver) -> *mut platform_driver {
    (drv as *mut u8).sub(core::mem::offset_of!(platform_driver, driver)) as *mut platform_driver
}

unsafe extern "C" {
    pub fn __platform_driver_register(drv: *mut platform_driver, module: *mut module, mod_name: *const ::core::ffi::c_char) -> i32;
    pub fn __platform_driver_unregister(drv: *mut platform_driver);
    pub fn __platform_driver_probe(
        driver: *mut platform_driver,
        probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
        module: *mut module, mod_name: *const ::core::ffi::c_char,
    ) -> i32;
    pub fn dev_get_drvdata(dev: *const device) -> *mut ::core::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
}

#[inline]
pub unsafe fn platform_get_drvdata(pdev: *const platform_device) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&(*pdev).dev)
}

#[inline]
pub unsafe fn platform_set_drvdata(pdev: *mut platform_device, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*pdev).dev, data)
}

#[inline]
pub unsafe fn is_sh_early_platform_device(_pdev: *mut platform_device) -> i32 { 0 }

unsafe extern "C" {
    pub fn __platform_create_bundle(
        driver: *mut platform_driver,
        probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
        res: *mut resource, n_res: u32, data: *const ::core::ffi::c_void, size: usize,
        module: *mut module, mod_name: *const ::core::ffi::c_char,
    ) -> *mut platform_device;
    pub fn __platform_register_drivers(
        drivers: *const *mut platform_driver, count: u32, owner: *mut module,
        mod_name: *const ::core::ffi::c_char,
    ) -> i32;
    pub fn platform_unregister_drivers(drivers: *const *mut platform_driver, count: u32);
    pub fn early_platform_cleanup();
}

/* CONFIG_SUSPEND and CONFIG_HIBERNATE_CALLBACKS provide these symbols when enabled. */
/* CONFIG_PM_SLEEP defines the aggregate USE_PLATFORM_PM_SLEEP_OPS initializer. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
