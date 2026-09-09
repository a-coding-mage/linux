/* SPDX-License-Identifier: GPL-2.0
 *
 * CDX bus public interface
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here.

pub const MAX_CDX_DEV_RESOURCES: usize = 4;
pub const CDX_CONTROLLER_ID_SHIFT: u32 = 4;
pub const CDX_BUS_NUM_MASK: u32 = 0xF;

/* Forward declaration for CDX controller */
pub const CDX_DEV_MSI_CONF: i32 = 0;
pub const CDX_DEV_BUS_MASTER_CONF: i32 = 1;
pub const CDX_DEV_RESET_CONF: i32 = 2;
pub const CDX_DEV_MSI_ENABLE: i32 = 3;

#[repr(C)]
pub struct cdx_msi_config {
    pub addr: u64,
    pub data: u32,
    pub msi_index: u16,
}

#[repr(C)]
pub union cdx_device_config_data {
    pub msi: cdx_msi_config,
    pub bus_master_enable: bool,
    pub msi_enable: bool,
}

#[repr(C)]
pub struct cdx_device_config {
    pub type_: u8,
    pub data: cdx_device_config_data,
}

pub type cdx_bus_enable_cb = unsafe extern "C" fn(*mut cdx_controller, u8) -> i32;
pub type cdx_bus_disable_cb = unsafe extern "C" fn(*mut cdx_controller, u8) -> i32;
pub type cdx_scan_cb = unsafe extern "C" fn(*mut cdx_controller) -> i32;
pub type cdx_dev_configure_cb = unsafe extern "C" fn(
    *mut cdx_controller,
    u8,
    u8,
    *mut cdx_device_config,
) -> i32;

#[repr(C)]
pub struct cdx_ops {
    pub bus_enable: Option<cdx_bus_enable_cb>,
    pub bus_disable: Option<cdx_bus_disable_cb>,
    pub scan: Option<cdx_scan_cb>,
    pub dev_configure: Option<cdx_dev_configure_cb>,
}

#[repr(C)]
pub struct cdx_controller {
    pub dev: *mut device,
    pub priv_: *mut core::ffi::c_void,
    pub msi_domain: *mut irq_domain,
    pub id: u32,
    pub controller_registered: bool,
    pub ops: *mut cdx_ops,
}

#[repr(C)]
pub struct cdx_device {
    pub dev: device,
    pub cdx: *mut cdx_controller,
    pub vendor: u16,
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub class: u32,
    pub revision: u8,
    pub bus_num: u8,
    pub dev_num: u8,
    pub res: [resource; MAX_CDX_DEV_RESOURCES],
    pub res_attr: [*mut bin_attribute; MAX_CDX_DEV_RESOURCES],
    pub debugfs_dir: *mut dentry,
    pub res_count: u8,
    pub dma_mask: u64,
    pub flags: u16,
    pub req_id: u32,
    pub is_bus: bool,
    pub enabled: bool,
    pub msi_dev_id: u32,
    pub num_msi: u32,
    pub irqchip_lock: mutex,
    pub msi_write_pending: bool,
}

#[macro_export]
macro_rules! CDX_DEVICE {
    ($vend:expr, $dev:expr) => {
        cdx_device_id { vendor: $vend, device: $dev, subvendor: CDX_ANY_ID, subdevice: CDX_ANY_ID }
    };
}

#[macro_export]
macro_rules! CDX_DEVICE_DRIVER_OVERRIDE {
    ($vend:expr, $dev:expr, $driver_override:expr) => {
        cdx_device_id { vendor: $vend, device: $dev, subvendor: CDX_ANY_ID, subdevice: CDX_ANY_ID, override_only: $driver_override }
    };
}

#[macro_export]
macro_rules! to_cdx_device { ($dev:expr) => { container_of!($dev, cdx_device, dev) }; }
#[macro_export]
macro_rules! cdx_resource_start { ($dev:expr, $num:expr) => { ($dev).res[$num].start }; }
#[macro_export]
macro_rules! cdx_resource_end { ($dev:expr, $num:expr) => { ($dev).res[$num].end }; }
#[macro_export]
macro_rules! cdx_resource_flags { ($dev:expr, $num:expr) => { ($dev).res[$num].flags }; }
#[macro_export]
macro_rules! cdx_resource_len {
    ($dev:expr, $num:expr) => {{
        let start = cdx_resource_start!($dev, $num);
        let end = cdx_resource_end!($dev, $num);
        if start == 0 && end == start { 0 } else { end - start + 1 }
    }};
}

#[repr(C)]
pub struct cdx_driver {
    pub driver: device_driver,
    pub match_id_table: *const cdx_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut cdx_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut cdx_device) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut cdx_device)>,
    pub reset_prepare: Option<unsafe extern "C" fn(*mut cdx_device)>,
    pub reset_done: Option<unsafe extern "C" fn(*mut cdx_device)>,
    pub driver_managed_dma: bool,
}

#[macro_export]
macro_rules! to_cdx_driver { ($drv:expr) => { container_of_const!($drv, cdx_driver, driver) }; }

extern "C" {
    pub fn __cdx_driver_register(cdx_driver: *mut cdx_driver, owner: *mut module) -> i32;
    pub fn cdx_driver_unregister(cdx_driver: *mut cdx_driver);
    pub static cdx_bus_type: bus_type;
    pub fn cdx_dev_reset(dev: *mut device) -> i32;
    pub fn cdx_set_master(cdx_dev: *mut cdx_device) -> i32;
    pub fn cdx_clear_master(cdx_dev: *mut cdx_device) -> i32;
    pub fn cdx_enable_msi(cdx_dev: *mut cdx_device) -> i32;
    pub fn cdx_disable_msi(cdx_dev: *mut cdx_device);
}

#[macro_export]
macro_rules! cdx_driver_register { ($drv:expr) => { __cdx_driver_register($drv, THIS_MODULE) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
