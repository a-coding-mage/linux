// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-2021 Intel Corporation.

/*
 * SDW Intel ACPI scan helpers
 */

// Dependencies in the original C source:
// linux/acpi.h, linux/bits.h, linux/bitfield.h, linux/device.h, linux/errno.h,
// linux/export.h, linux/module.h, linux/property.h,
// linux/soundwire/sdw_intel.h, linux/string.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const SDW_LINK_TYPE: u64 = 4; /* from Intel ACPI documentation */

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

const AE_OK: acpi_status = 0;
const AE_NOT_FOUND: acpi_status = 0x0005;
const AE_CTRL_TERMINATE: acpi_status = 0x1001;
const ACPI_TYPE_DEVICE: u32 = 0x00000001;

static mut ctrl_link_mask: c_int = 0;
// module_param_named(sdw_link_mask, ctrl_link_mask, int, 0444);
// MODULE_PARM_DESC(sdw_link_mask, "Intel link mask (one bit per link)");

static mut ctrl_addr: c_ulong = 0x40000000;
// module_param_named(sdw_ctrl_addr, ctrl_addr, ulong, 0444);
// MODULE_PARM_DESC(sdw_ctrl_addr, "Intel SoundWire Controller _ADR");

#[allow(non_camel_case_types)]
type u8 = u8;
#[allow(non_camel_case_types)]
type u32 = u32;
#[allow(non_camel_case_types)]
type u64 = u64;
#[allow(non_camel_case_types)]
type acpi_handle = *mut c_void;
#[allow(non_camel_case_types)]
type acpi_status = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    pub dev: device,
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_intel_acpi_info {
    pub handle: acpi_handle,
    pub count: u32,
    pub link_mask: u32,
}

extern "C" {
    static SDW_INTEL_QUIRK_MASK_BUS_DISABLE: u32;
    static SDW_INTEL_MAX_LINKS: u32;

    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn fwnode_get_named_child_node(
        fwnode: *mut fwnode_handle,
        childname: *const c_char,
    ) -> *mut fwnode_handle;
    fn fwnode_property_read_u32(
        fwnode: *mut fwnode_handle,
        propname: *const c_char,
        val: *mut u32,
    ) -> c_int;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);

    fn acpi_fetch_acpi_dev(handle: acpi_handle) -> *mut acpi_device;
    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut fwnode_handle;
    fn hweight32(w: u32) -> u32;
    fn acpi_get_local_u64_address(handle: acpi_handle, adr: *mut u64) -> c_int;
    fn acpi_walk_namespace(
        type_: u32,
        start_object: acpi_handle,
        max_depth: u32,
        user_function: Option<
            unsafe extern "C" fn(acpi_handle, u32, *mut c_void, *mut *mut c_void) -> acpi_status,
        >,
        descending_callback: Option<
            unsafe extern "C" fn(acpi_handle, u32, *mut c_void, *mut *mut c_void) -> acpi_status,
        >,
        context: *mut c_void,
        return_value: *mut *mut c_void,
    ) -> acpi_status;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

const fn bit(nr: u32) -> c_ulong {
    1_c_ulong << nr
}

const fn genmask(h: u32, l: u32) -> c_ulong {
    (!0_c_ulong << l) & (!0_c_ulong >> (c_ulong::BITS - 1 - h))
}

const fn field_get(mask: c_ulong, reg: u64) -> u64 {
    ((reg as c_ulong & mask) >> mask.trailing_zeros()) as u64
}

fn acpi_failure(status: acpi_status) -> bool {
    status != AE_OK && status != AE_CTRL_TERMINATE
}

unsafe fn is_link_enabled(fw_node: *mut fwnode_handle, idx: u8) -> bool {
    let mut link: *mut fwnode_handle;
    let mut name = [0 as c_char; 32];
    let mut quirk_mask: u32 = 0;

    /* Find master handle */
    snprintf(
        name.as_mut_ptr(),
        name.len(),
        b"mipi-sdw-link-%hhu-subproperties\0".as_ptr() as *const c_char,
        idx as c_int,
    );

    link = fwnode_get_named_child_node(fw_node, name.as_ptr());
    if link.is_null() {
        return false;
    }

    fwnode_property_read_u32(
        link,
        b"intel-quirk-mask\0".as_ptr() as *const c_char,
        &mut quirk_mask,
    );

    fwnode_handle_put(link);

    if quirk_mask & SDW_INTEL_QUIRK_MASK_BUS_DISABLE != 0 {
        return false;
    }

    true
}

unsafe fn sdw_intel_scan_controller(info: *mut sdw_intel_acpi_info) -> c_int {
    let adev = acpi_fetch_acpi_dev((*info).handle);
    let fwnode: *mut fwnode_handle;
    let mut list: c_ulong;
    let mut i: u32;
    let mut count: u32 = 0;
    let mut tmp: u32 = 0;
    let mut ret: c_int;

    if adev.is_null() {
        return -EINVAL;
    }

    fwnode = acpi_fwnode_handle(adev);

    /*
     * Found controller, find links supported
     *
     * In theory we could check the number of links supported in
     * hardware, but in that step we cannot assume SoundWire IP is
     * powered.
     *
     * In addition, if the BIOS doesn't even provide this
     * 'master-count' property then all the inits based on link
     * masks will fail as well.
     *
     * We will check the hardware capabilities in the startup() step
     */
    ret = fwnode_property_read_u32(
        fwnode,
        b"mipi-sdw-manager-list\0".as_ptr() as *const c_char,
        &mut tmp,
    );
    if ret != 0 {
        ret = fwnode_property_read_u32(
            fwnode,
            b"mipi-sdw-master-count\0".as_ptr() as *const c_char,
            &mut count,
        );
        if ret != 0 {
            dev_err(
                &mut (*adev).dev,
                b"Failed to read mipi-sdw-master-count: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
        list = genmask(count - 1, 0);
    } else {
        list = tmp as c_ulong;
        count = hweight32(list as u32);
    }

    /* Check count is within bounds */
    if count > SDW_INTEL_MAX_LINKS {
        dev_err(
            &mut (*adev).dev,
            b"Link count %d exceeds max %d\n\0".as_ptr() as *const c_char,
            count,
            SDW_INTEL_MAX_LINKS,
        );
        return -EINVAL;
    }

    if count == 0 {
        dev_warn(
            &mut (*adev).dev,
            b"No SoundWire links detected\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    dev_dbg(
        &mut (*adev).dev,
        b"ACPI reports %d SDW Link devices\n\0".as_ptr() as *const c_char,
        count,
    );

    (*info).count = count;
    (*info).link_mask = 0;

    i = 0;
    while i < SDW_INTEL_MAX_LINKS {
        if list & bit(i) != 0 {
            if ctrl_link_mask != 0 && (ctrl_link_mask as c_ulong & bit(i)) == 0 {
                dev_dbg(
                    &mut (*adev).dev,
                    b"Link %d masked, will not be enabled\n\0".as_ptr() as *const c_char,
                    i,
                );
                i += 1;
                continue;
            }

            if !is_link_enabled(fwnode, i as u8) {
                dev_dbg(
                    &mut (*adev).dev,
                    b"Link %d not selected in firmware\n\0".as_ptr() as *const c_char,
                    i,
                );
                i += 1;
                continue;
            }

            (*info).link_mask |= bit(i) as u32;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn sdw_intel_acpi_cb(
    handle: acpi_handle,
    _level: u32,
    cdata: *mut c_void,
    _return_value: *mut *mut c_void,
) -> acpi_status {
    let info = cdata as *mut sdw_intel_acpi_info;
    let mut adr: u64 = 0;
    let ret: c_int;

    ret = acpi_get_local_u64_address(handle, &mut adr);
    if ret < 0 {
        return AE_OK; /* keep going */
    }

    if acpi_fetch_acpi_dev(handle).is_null() {
        pr_err(
            b"%s: Couldn't find ACPI handle\n\0".as_ptr() as *const c_char,
            b"sdw_intel_acpi_cb\0".as_ptr() as *const c_char,
        );
        return AE_NOT_FOUND;
    }

    /*
     * On some Intel platforms, multiple children of the HDAS
     * device can be found, but only one of them is the SoundWire
     * controller. The SNDW device is always exposed with
     * Name(_ADR, 0x40000000), with bits 31..28 representing the
     * SoundWire link so filter accordingly
     */
    if field_get(genmask(31, 28), adr) != SDW_LINK_TYPE {
        return AE_OK; /* keep going */
    }

    if adr != ctrl_addr as u64 {
        return AE_OK; /* keep going */
    }

    /* found the correct SoundWire controller */
    (*info).handle = handle;

    /* device found, stop namespace walk */
    AE_CTRL_TERMINATE
}

/**
 * sdw_intel_acpi_scan() - SoundWire Intel init routine
 * @parent_handle: ACPI parent handle
 * @info: description of what firmware/DSDT tables expose
 *
 * This scans the namespace and queries firmware to figure out which
 * links to enable. A follow-up use of sdw_intel_probe() and
 * sdw_intel_startup() is required for creation of devices and bus
 * startup
 */
#[no_mangle]
pub unsafe extern "C" fn sdw_intel_acpi_scan(
    parent_handle: acpi_handle,
    info: *mut sdw_intel_acpi_info,
) -> c_int {
    let status: acpi_status;

    (*info).handle = ptr::null_mut();
    /*
     * In the HDAS ACPI scope, 'SNDW' may be either the child of
     * 'HDAS' or the grandchild of 'HDAS'. So let's go through
     * the ACPI from 'HDAS' at max depth of 2 to find the 'SNDW'
     * device.
     */
    status = acpi_walk_namespace(
        ACPI_TYPE_DEVICE,
        parent_handle,
        2,
        Some(sdw_intel_acpi_cb),
        None,
        info as *mut c_void,
        ptr::null_mut(),
    );
    if acpi_failure(status) || (*info).handle.is_null() {
        return -ENODEV;
    }

    sdw_intel_scan_controller(info)
}
// EXPORT_SYMBOL_NS(sdw_intel_acpi_scan, "SND_INTEL_SOUNDWIRE_ACPI");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("Intel Soundwire ACPI helpers");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
