// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016 - 2018 Intel Corporation. All rights reserved. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const IORESOURCE_DAX_STATIC: u32 = 1 << 0;
pub const IORESOURCE_DAX_KMEM: u32 = 1 << 1;

pub const DAX_KMEM_UNPLUGGED: i32 = -1; /* Do not create memory blocks */

extern "C" {
    pub fn alloc_dax_region(
        parent: *mut device,
        region_id: i32,
        range: *mut range,
        target_node: i32,
        align: u32,
        flags: libc::c_ulong,
    ) -> *mut dax_region;
}

#[repr(C)]
pub struct dev_dax_data {
    pub dax_region: *mut dax_region,
    pub pgmap: *mut dev_pagemap,
    pub size: resource_size_t,
    pub id: i32,
    pub memmap_on_memory: bool,
}

extern "C" {
    pub fn devm_create_dev_dax(data: *mut dev_dax_data) -> *mut dev_dax;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dax_driver_type {
    DAXDRV_KMEM_TYPE,
    DAXDRV_DEVICE_TYPE,
    DAXDRV_FSDEV_TYPE,
}

#[repr(C)]
pub struct dax_device_driver {
    pub drv: device_driver,
    pub ids: list_head,
    pub type_: dax_driver_type,
    pub probe: Option<unsafe extern "C" fn(dev: *mut dev_dax) -> i32>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut dev_dax)>,
}

// Equivalent to: container_of_const(__drv, struct dax_device_driver, drv)
#[macro_export]
macro_rules! to_dax_drv {
    ($drv:expr) => {
        container_of_const!($drv, dax_device_driver, drv)
    };
}

extern "C" {
    pub fn __dax_driver_register(
        dax_drv: *mut dax_device_driver,
        module: *mut module,
        mod_name: *const libc::c_char,
    ) -> i32;
    pub fn dax_driver_unregister(dax_drv: *mut dax_device_driver);
    pub fn kill_dev_dax(dev_dax: *mut dev_dax);
    pub fn static_dev_dax(dev_dax: *mut dev_dax) -> bool;
}

// dax_driver_register(driver) expands to __dax_driver_register(driver,
// THIS_MODULE, KBUILD_MODNAME) at build time.

#[repr(C)]
pub struct hmem_platform_device {
    pub pdev: platform_device,
    pub work: work_struct,
    pub did_probe: bool,
}

#[inline]
pub unsafe fn to_hmem_platform_device(
    pdev: *mut platform_device,
) -> *mut hmem_platform_device {
    container_of!(pdev, hmem_platform_device, pdev)
}

// CONFIG_DEV_DAX_HMEM controls whether this external operation is available.
extern "C" {
    pub fn dax_hmem_flush_work();
}

// MODULE_ALIAS_DAX_DEVICE(type) expands to MODULE_ALIAS("dax:t" __stringify(type) "*").
pub const DAX_DEVICE_MODALIAS_FMT: &str = "dax:t%d";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
