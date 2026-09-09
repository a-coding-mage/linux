/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/* Copyright 2022 Innovative Advantage Inc. */

// Translated from the Linux kernel header.  The following names are supplied
// by the corresponding kernel/Rust dependencies:
// linux/err.h, linux/errno.h, linux/ioport.h, linux/platform_device.h,
// linux/regmap.h, and linux/types.h.

use core::ffi::c_void;

pub enum resource {}
pub enum regmap {}
pub enum regmap_config {}
pub enum platform_device {}
pub enum device {}

extern "C" {
    fn platform_get_resource(
        pdev: *mut platform_device,
        resource_type: u32,
        index: u32,
    ) -> *mut resource;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_get_regmap(dev: *mut device, name: *const i8) -> *mut regmap;
    fn err_cast<T>(ptr: *mut T) -> *mut regmap;
    fn err_ptr<T>(error: i32) -> *mut T;
    fn is_err<T>(ptr: *mut T) -> bool;
}

// IORESOURCE_MEM and IORESOURCE_REG are supplied by linux/ioport.h.
const IORESOURCE_MEM: u32 = 0x0000_0200;
const IORESOURCE_REG: u32 = 0x0000_0100;
const ENOENT: i32 = 2;

#[repr(C)]
struct PlatformDeviceLayout {
    _private: [u8; 0],
}

// The layout of platform_device and device is supplied by the kernel headers.
// These accessors preserve the source-level field references.
extern "C" {
    fn platform_device_dev(pdev: *mut platform_device) -> *mut device;
    fn device_parent(dev: *mut device) -> *mut device;
    fn resource_name(res: *mut resource) -> *const i8;
}

pub unsafe fn ocelot_regmap_from_resource_optional(
    pdev: *mut platform_device,
    index: u32,
    config: *const regmap_config,
) -> *mut regmap {
    let dev: *mut device = platform_device_dev(pdev);
    let mut res: *mut resource;
    let regs: *mut c_void;

    /*
     * Don't use _get_and_ioremap_resource() here, since that will invoke
     * prints of "invalid resource" which will simply add confusion.
     */
    res = platform_get_resource(pdev, IORESOURCE_MEM, index);
    if !res.is_null() {
        regs = devm_ioremap_resource(dev, res);
        if is_err(regs) {
            return err_cast(regs);
        }
        return devm_regmap_init_mmio(dev, regs, config);
    }

    /*
     * Fall back to using REG and getting the resource from the parent
     * device, which is possible in an MFD configuration
     */
    if !device_parent(dev).is_null() {
        res = platform_get_resource(pdev, IORESOURCE_REG, index);
        if res.is_null() {
            return core::ptr::null_mut();
        }

        return dev_get_regmap(device_parent(dev), resource_name(res));
    }

    core::ptr::null_mut()
}

pub unsafe fn ocelot_regmap_from_resource(
    pdev: *mut platform_device,
    index: u32,
    config: *const regmap_config,
) -> *mut regmap {
    let map: *mut regmap = ocelot_regmap_from_resource_optional(pdev, index, config);
    if !map.is_null() {
        map
    } else {
        err_ptr(-ENOENT)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
