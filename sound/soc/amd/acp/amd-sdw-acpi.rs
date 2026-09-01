// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*
 * SDW AMD ACPI scan helper function
 */

// C dependencies:
// linux/acpi.h
// linux/bits.h
// linux/bitfield.h
// linux/device.h
// linux/errno.h
// linux/export.h
// linux/module.h
// linux/property.h
// linux/soundwire/sdw_amd.h
// linux/string.h

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn acpi_fetch_acpi_dev(handle: *mut c_void) -> *mut acpi_device;
    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut fwnode_handle;
    fn fwnode_property_read_u32_array(
        fwnode: *mut fwnode_handle,
        propname: *const c_char,
        val: *mut u32,
        nval: usize,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn hweight32(w: c_uint) -> c_uint;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    pub dev: device,
}

#[repr(C)]
pub struct sdw_amd_acpi_info {
    pub handle: *mut c_void,
    pub count: u8,
    pub link_mask: u32,
}

const EINVAL: c_int = 22;

#[no_mangle]
pub unsafe extern "C" fn amd_sdw_scan_controller(info: *mut sdw_amd_acpi_info) -> c_int {
    let adev: *mut acpi_device = acpi_fetch_acpi_dev((*info).handle);
    let mut sdw_bitmap: u32 = 0;
    let mut count: u8 = 0;
    let ret: c_int;

    if adev.is_null() {
        return -EINVAL;
    }

    /* Found controller, find links supported */
    ret = fwnode_property_read_u32_array(
        acpi_fwnode_handle(adev),
        b"mipi-sdw-manager-list\0".as_ptr() as *const c_char,
        &mut sdw_bitmap,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*adev).dev,
            b"Failed to read mipi-sdw-manager-list: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return -EINVAL;
    }
    count = hweight32(sdw_bitmap) as u8;
    /* Check count is within bounds */
    if count > (*info).count {
        dev_err(
            &mut (*adev).dev,
            b"Manager count %d exceeds max %d\n\0".as_ptr() as *const c_char,
            count as c_int,
            (*info).count as c_int,
        );
        return -EINVAL;
    }

    if count == 0 {
        dev_dbg(
            &mut (*adev).dev,
            b"No SoundWire Managers detected\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    dev_dbg(
        &mut (*adev).dev,
        b"ACPI reports %d SoundWire Manager devices\n\0".as_ptr() as *const c_char,
        count as c_int,
    );
    (*info).link_mask = sdw_bitmap;
    0
}

// EXPORT_SYMBOL_NS(amd_sdw_scan_controller, "SND_AMD_SOUNDWIRE_ACPI");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("AMD SoundWire ACPI helpers");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
