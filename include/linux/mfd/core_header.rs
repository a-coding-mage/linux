/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * drivers/mfd/mfd-core.h
 *
 * core MFD support
 * Copyright (c) 2006 Ian Molton
 * Copyright (c) 2007 Dmitry Baryshkov
 */

// C dependency: <linux/platform_device.h>

#[macro_export]
macro_rules! MFD_RES_SIZE {
    ($arr:expr) => {
        core::mem::size_of_val(&$arr) / core::mem::size_of::<$crate::resource>()
    };
}

#[macro_export]
macro_rules! MFD_CELL_ALL {
    ($name:expr, $res:expr, $pdata:expr, $pdsize:expr, $id:expr, $compat:expr, $of_reg:expr, $use_of_reg:expr, $match_:expr) => {
        $crate::mfd_cell {
            name: $name,
            resources: $res,
            num_resources: MFD_RES_SIZE!($res),
            platform_data: $pdata,
            pdata_size: $pdsize,
            of_compatible: $compat,
            of_reg: $of_reg,
            use_of_reg: $use_of_reg,
            acpi_match: $match_,
            id: $id,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

#[macro_export]
macro_rules! MFD_CELL_OF_REG {
    ($name:expr, $res:expr, $pdata:expr, $pdsize:expr, $id:expr, $compat:expr, $of_reg:expr) => {
        MFD_CELL_ALL!($name, $res, $pdata, $pdsize, $id, $compat, $of_reg, true, core::ptr::null())
    };
}

#[macro_export]
macro_rules! MFD_CELL_OF {
    ($name:expr, $res:expr, $pdata:expr, $pdsize:expr, $id:expr, $compat:expr) => {
        MFD_CELL_ALL!($name, $res, $pdata, $pdsize, $id, $compat, 0, false, core::ptr::null())
    };
}

#[macro_export]
macro_rules! MFD_CELL_ACPI {
    ($name:expr, $res:expr, $pdata:expr, $pdsize:expr, $id:expr, $match_:expr) => {
        MFD_CELL_ALL!($name, $res, $pdata, $pdsize, $id, core::ptr::null(), 0, false, $match_)
    };
}

#[macro_export]
macro_rules! MFD_CELL_BASIC {
    ($name:expr, $res:expr, $pdata:expr, $pdsize:expr, $id:expr) => {
        MFD_CELL_ALL!($name, $res, $pdata, $pdsize, $id, core::ptr::null(), 0, false, core::ptr::null())
    };
}

#[macro_export]
macro_rules! MFD_CELL_RES {
    ($name:expr, $res:expr) => {
        MFD_CELL_ALL!($name, $res, core::ptr::null(), 0, 0, core::ptr::null(), 0, false, core::ptr::null())
    };
}

#[macro_export]
macro_rules! MFD_CELL_NAME {
    ($name:expr) => {
        MFD_CELL_ALL!($name, core::ptr::null(), core::ptr::null(), 0, 0, core::ptr::null(), 0, false, core::ptr::null())
    };
}

pub const MFD_DEP_LEVEL_NORMAL: i32 = 0;
pub const MFD_DEP_LEVEL_HIGH: i32 = 1;

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

// Matches ACPI PNP id, either _HID or _CID, or ACPI _ADR
#[repr(C)]
pub struct mfd_cell_acpi_match {
    pub pnpid: *const core::ffi::c_char,
    pub adr: u64,
}

#[repr(C)]
pub struct mfd_cell {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub level: i32,
    pub suspend: Option<unsafe extern "C" fn(dev: *mut platform_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut platform_device) -> i32>,
    pub platform_data: *const core::ffi::c_void,
    pub pdata_size: usize,
    pub acpi_match: *const mfd_cell_acpi_match,
    pub swnode: *const software_node,
    pub of_compatible: *const core::ffi::c_char,
    pub of_reg: u64,
    pub use_of_reg: bool,
    pub num_resources: i32,
    pub resources: *const resource,
    pub ignore_resource_conflicts: bool,
    pub pm_runtime_no_callbacks: bool,
    pub num_parent_supplies: i32,
    pub parent_supplies: *const *const core::ffi::c_char,
}

// C dependency types supplied by other files.
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub mfd_cell: *const mfd_cell }
#[repr(C)] pub struct resource { _private: [u8; 0] }

#[inline]
pub unsafe fn mfd_get_cell(pdev: *mut platform_device) -> *const mfd_cell {
    (*pdev).mfd_cell
}

extern "C" {
    pub fn mfd_add_devices(parent: *mut device, id: i32, cells: *const mfd_cell,
                            n_devs: i32, mem_base: *mut resource, irq_base: i32,
                            irq_domain: *mut irq_domain) -> i32;
}

pub const PLATFORM_DEVID_AUTO: i32 = -1;

#[inline]
pub unsafe fn mfd_add_hotplug_devices(parent: *mut device, cells: *const mfd_cell,
                                      n_devs: i32) -> i32 {
    mfd_add_devices(parent, PLATFORM_DEVID_AUTO, cells, n_devs,
                     core::ptr::null_mut(), 0, core::ptr::null_mut())
}

extern "C" {
    pub fn mfd_remove_devices(parent: *mut device);
    pub fn mfd_remove_devices_late(parent: *mut device);
    pub fn devm_mfd_add_devices(dev: *mut device, id: i32, cells: *const mfd_cell,
                                n_devs: i32, mem_base: *mut resource, irq_base: i32,
                                irq_domain: *mut irq_domain) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
