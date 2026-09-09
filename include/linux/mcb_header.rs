/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MEN Chameleon Bus.
 *
 * Copyright (C) 2014 MEN Mikroelektronik GmbH (www.men.de)
 * Author: Johannes Thumshirn <johannes.thumshirn@men.de>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const CHAMELEON_FILENAME_LEN: usize = 12;

pub struct mcb_driver;
pub struct mcb_device;

#[repr(C)]
pub struct mcb_bus {
    pub dev: device,
    pub carrier: *mut device,
    pub bus_nr: core::ffi::c_int,
    pub revision: u8,
    pub model: core::ffi::c_char,
    pub minor: u8,
    pub name: [core::ffi::c_char; CHAMELEON_FILENAME_LEN + 1],
    pub get_irq: Option<unsafe extern "C" fn(dev: *mut mcb_device) -> core::ffi::c_int>,
}

#[inline]
pub unsafe fn to_mcb_bus(dev: *mut device) -> *mut mcb_bus {
    container_of!(dev, mcb_bus, dev)
}

#[repr(C)]
pub struct mcb_device {
    pub dev: device,
    pub bus: *mut mcb_bus,
    pub driver: *mut mcb_driver,
    pub id: u16,
    pub inst: core::ffi::c_int,
    pub group: core::ffi::c_int,
    pub var: core::ffi::c_int,
    pub bar: core::ffi::c_int,
    pub rev: core::ffi::c_int,
    pub irq: resource,
    pub mem: resource,
    pub dma_dev: *mut device,
}

#[inline]
pub unsafe fn to_mcb_device(dev: *const device) -> *const mcb_device {
    container_of_const!(dev, mcb_device, dev)
}

#[repr(C)]
pub struct mcb_driver {
    pub driver: device_driver,
    pub id_table: *const mcb_device_id,
    pub probe: Option<unsafe extern "C" fn(
        mdev: *mut mcb_device,
        id: *const mcb_device_id,
    ) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(mdev: *mut mcb_device)>,
    pub shutdown: Option<unsafe extern "C" fn(mdev: *mut mcb_device)>,
}

#[inline]
pub unsafe fn to_mcb_driver(drv: *const device_driver) -> *const mcb_driver {
    container_of_const!(drv, mcb_driver, driver)
}

#[inline]
pub unsafe fn mcb_get_drvdata(dev: *mut mcb_device) -> *mut core::ffi::c_void {
    dev_get_drvdata(core::ptr::addr_of_mut!((*dev).dev))
}

#[inline]
pub unsafe fn mcb_set_drvdata(dev: *mut mcb_device, data: *mut core::ffi::c_void) {
    dev_set_drvdata(core::ptr::addr_of_mut!((*dev).dev), data);
}

unsafe extern "C" {
    pub fn __mcb_register_driver(
        drv: *mut mcb_driver,
        owner: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn mcb_unregister_driver(driver: *mut mcb_driver);
    pub fn mcb_bus_add_devices(bus: *const mcb_bus);
    pub fn mcb_device_register(bus: *mut mcb_bus, dev: *mut mcb_device) -> core::ffi::c_int;
    pub fn mcb_alloc_bus(carrier: *mut device) -> *mut mcb_bus;
    pub fn mcb_bus_get(bus: *mut mcb_bus) -> *mut mcb_bus;
    pub fn mcb_bus_put(bus: *mut mcb_bus);
    pub fn mcb_alloc_dev(bus: *mut mcb_bus) -> *mut mcb_device;
    pub fn mcb_free_dev(dev: *mut mcb_device);
    pub fn mcb_release_bus(bus: *mut mcb_bus);
    pub fn mcb_request_mem(
        dev: *mut mcb_device,
        name: *const core::ffi::c_char,
    ) -> *mut resource;
    pub fn mcb_release_mem(mem: *mut resource);
    pub fn mcb_get_irq(dev: *mut mcb_device) -> core::ffi::c_int;
    pub fn mcb_get_resource(dev: *mut mcb_device, type_: u32) -> *mut resource;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
