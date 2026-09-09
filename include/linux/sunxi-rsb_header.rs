/*
 * Allwinner Reduced Serial Bus Driver
 *
 * Copyright (c) 2015 Chen-Yu Tsai
 *
 * Author: Chen-Yu Tsai <wens@csie.org>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/device.h, linux/regmap.h, and linux/types.h

pub struct sunxi_rsb;

/**
 * struct sunxi_rsb_device - Basic representation of an RSB device
 * @dev:      Driver model representation of the device.
 * @ctrl:     RSB controller managing the bus hosting this device.
 * @rtaddr:   This device's runtime address
 * @hwaddr:   This device's hardware address
 */
#[repr(C)]
pub struct sunxi_rsb_device {
    pub dev: device,
    pub rsb: *mut sunxi_rsb,
    pub irq: i32,
    pub rtaddr: u8,
    pub hwaddr: u16,
}

#[inline]
pub unsafe fn to_sunxi_rsb_device(d: *mut device) -> *mut sunxi_rsb_device {
    container_of!(d, sunxi_rsb_device, dev)
}

#[inline]
pub unsafe fn sunxi_rsb_device_get_drvdata(
    rdev: *const sunxi_rsb_device,
) -> *mut core::ffi::c_void {
    dev_get_drvdata!(&(*rdev).dev)
}

#[inline]
pub unsafe fn sunxi_rsb_device_set_drvdata(
    rdev: *mut sunxi_rsb_device,
    data: *mut core::ffi::c_void,
) {
    dev_set_drvdata!(&mut (*rdev).dev, data);
}

/**
 * struct sunxi_rsb_driver - RSB slave device driver
 * @driver:   RSB device drivers should initialize name and owner field of
 *            this structure.
 * @probe:    binds this driver to a RSB device.
 * @remove:   unbinds this driver from the RSB device.
 */
#[repr(C)]
pub struct sunxi_rsb_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sunxi_rsb_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut sunxi_rsb_device)>,
}

#[inline]
pub unsafe fn to_sunxi_rsb_driver(d: *mut device_driver) -> *mut sunxi_rsb_driver {
    container_of!(d, sunxi_rsb_driver, driver)
}

pub unsafe extern "C" fn sunxi_rsb_driver_register(
    rdrv: *mut sunxi_rsb_driver,
) -> i32;

/**
 * sunxi_rsb_driver_unregister() - unregister an RSB client driver
 * @rdrv: the driver to unregister
 */
#[inline]
pub unsafe fn sunxi_rsb_driver_unregister(rdrv: *mut sunxi_rsb_driver) {
    if !rdrv.is_null() {
        driver_unregister!(&mut (*rdrv).driver);
    }
}

macro_rules! module_sunxi_rsb_driver {
    ($sunxi_rsb_driver:expr) => {
        module_driver!($sunxi_rsb_driver, sunxi_rsb_driver_register,
                       sunxi_rsb_driver_unregister)
    };
}

pub unsafe extern "C" fn __devm_regmap_init_sunxi_rsb(
    rdev: *mut sunxi_rsb_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap;

/**
 * devm_regmap_init_sunxi_rsb(): Initialise managed register map
 *
 * @rdev: Device that will be interacted with
 * @config: Configuration for register map
 *
 * The return value will be an ERR_PTR() on error or a valid pointer
 * to a struct regmap.  The regmap will be automatically freed by the
 * device management code.
 */
macro_rules! devm_regmap_init_sunxi_rsb {
    ($rdev:expr, $config:expr) => {
        __regmap_lockdep_wrapper!(__devm_regmap_init_sunxi_rsb,
                                  stringify!($config), $rdev, $config)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
