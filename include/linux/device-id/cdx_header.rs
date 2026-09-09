/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/types.h> when __KERNEL__ is defined.

pub const CDX_ANY_ID: u16 = 0xFFFF;

pub const CDX_ID_F_VFIO_DRIVER_OVERRIDE: u32 = 1;

/**
 * struct cdx_device_id - CDX device identifier
 * @vendor: Vendor ID
 * @device: Device ID
 * @subvendor: Subsystem vendor ID (or CDX_ANY_ID)
 * @subdevice: Subsystem device ID (or CDX_ANY_ID)
 * @class: Device class
 *         Most drivers do not need to specify class/class_mask
 *         as vendor/device is normally sufficient.
 * @class_mask: Limit which sub-fields of the class field are compared.
 * @override_only: Match only when dev->driver_override is this driver.
 *
 * Type of entries in the "device Id" table for CDX devices supported by
 * a CDX device driver.
 */
#[repr(C)]
pub struct cdx_device_id {
    pub vendor: u16,
    pub device: u16,
    pub subvendor: u16,
    pub subdevice: u16,
    pub class: u32,
    pub class_mask: u32,
    pub override_only: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
