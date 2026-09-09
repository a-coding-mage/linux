/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for DFL driver and device API
 *
 * Copyright (C) 2020 Intel Corporation, Inc.
 */

// Dependencies supplied by the Linux device and DFL device-id headers are
// intentionally referenced here rather than reimplemented.

/**
 * enum dfl_id_type - define the DFL FIU types
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dfl_id_type {
    FME_ID = 0,
    PORT_ID = 1,
    DFL_ID_MAX,
}

/**
 * struct dfl_device - represent an dfl device on dfl bus
 *
 * @dev: generic device interface.
 * @id: id of the dfl device.
 * @type: type of DFL FIU of the device. See enum dfl_id_type.
 * @feature_id: feature identifier local to its DFL FIU type.
 * @revision: revision of this dfl device feature.
 * @mmio_res: mmio resource of this dfl device.
 * @irqs: list of Linux IRQ numbers of this dfl device.
 * @num_irqs: number of IRQs supported by this dfl device.
 * @cdev: pointer to DFL FPGA container device this dfl device belongs to.
 * @id_entry: matched id entry in dfl driver's id table.
 * @dfh_version: version of DFH for the device
 * @param_size: size of the block parameters in bytes
 * @params: pointer to block of parameters copied memory
 */
#[repr(C)]
pub struct dfl_device {
    pub dev: device,
    pub id: core::ffi::c_int,
    pub type_: u16,
    pub feature_id: u16,
    pub revision: u8,
    pub mmio_res: resource,
    pub irqs: *mut core::ffi::c_int,
    pub num_irqs: core::ffi::c_uint,
    pub cdev: *mut dfl_fpga_cdev,
    pub id_entry: *const dfl_device_id,
    pub dfh_version: u8,
    pub param_size: core::ffi::c_uint,
    pub params: *mut core::ffi::c_void,
}

/**
 * struct dfl_driver - represent an dfl device driver
 *
 * @drv: driver model structure.
 * @id_table: pointer to table of device IDs the driver is interested in.
 *           { } member terminated.
 * @probe: mandatory callback for device binding.
 * @remove: callback for device unbinding.
 */
#[repr(C)]
pub struct dfl_driver {
    pub drv: device_driver,
    pub id_table: *const dfl_device_id,
    pub probe: Option<unsafe extern "C" fn(dfl_dev: *mut dfl_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(dfl_dev: *mut dfl_device)>,
}

// #define to_dfl_dev(d) container_of(d, struct dfl_device, dev)
// #define to_dfl_drv(d) container_of_const(d, struct dfl_driver, drv)

/*
 * use a macro to avoid include chaining to get THIS_MODULE.
 */
// #define dfl_driver_register(drv) __dfl_driver_register(drv, THIS_MODULE)
unsafe extern "C" {
    pub fn __dfl_driver_register(
        dfl_drv: *mut dfl_driver,
        owner: *mut module,
    ) -> core::ffi::c_int;
    pub fn dfl_driver_unregister(dfl_drv: *mut dfl_driver);
}

/*
 * module_dfl_driver() - Helper macro for drivers that don't do
 * anything special in module init/exit.  This eliminates a lot of
 * boilerplate.  Each module may only use this macro once, and
 * calling it replaces module_init() and module_exit().
 */
// #define module_dfl_driver(__dfl_driver) \
//     module_driver(__dfl_driver, dfl_driver_register, dfl_driver_unregister)

unsafe extern "C" {
    pub fn dfh_find_param(
        dfl_dev: *mut dfl_device,
        param_id: core::ffi::c_int,
        pcount: *mut usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
