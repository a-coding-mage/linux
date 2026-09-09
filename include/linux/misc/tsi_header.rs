/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AMD SBTSI shared data structure and auxiliary bus definitions.
 *
 * Copyright (C) 2026 Advanced Micro Devices, Inc.
 */

/* C header dependencies are supplied by other Rust translation units. */

use core::ffi::c_int;

#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i3c_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct miscdevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}

#[repr(C)]
pub union SbtsiBusDevice {
    pub client: *mut i2c_client,
    pub i3cdev: *mut i3c_device,
}

/**
 * struct sbtsi_data - driver private data for an AMD SB-TSI device
 * @client: underlying I2C client
 * @i3cdev: underlying I3C device (when using I3C bus)
 * @sbtsi_misc_dev: miscdevice exposing ioctl interface at /dev/sbtsi-<addr>
 * @lock: mutex protecting concurrent access to the device
 * @kref: reference count; keeps @sbtsi_data alive while misc fds are open
 * @dev_addr: I2C/I3C device address, used as the auxiliary device instance id
 * and name the misc device node
 * @ext_range_mode: sensor uses extended temperature range
 * @read_order: if set, decimal part must be read before integer part
 * @is_i3c: true when the device is accessed over I3C
 * @detached: set on driver unbind; open/ioctl return -ENODEV afterward
 */
#[repr(C)]
pub struct sbtsi_data {
    pub bus: SbtsiBusDevice,
    pub sbtsi_misc_dev: miscdevice,
    pub lock: mutex, /* protects concurrent access to the device */
    pub kref: kref,
    pub dev_addr: u8,
    pub ext_range_mode: bool,
    pub read_order: bool,
    pub is_i3c: bool,
    pub detached: bool,
}

/*
 * DEFINE_GUARD(sbtsi, struct sbtsi_data *, mutex_lock(&_T->lock),
 *              mutex_unlock(&_T->lock))
 *
 * The C macro defines a scoped cleanup guard which locks data->lock on entry
 * and unlocks it on scope exit.
 */

/*
 * Name of the auxiliary device published on the auxiliary bus by the core
 * driver. The full device name is "amd-sbtsi.temp-sensor.<id>", where
 * <id> is the auxiliary device instance id.
 */
pub const AMD_SBTSI_ADEV: &str = "amd-sbtsi";
pub const AMD_SBTSI_AUX_HWMON: &str = "temp-sensor";

/**
 * sbtsi_xfer - Perform a register read or write transfer on an AMD SB-TSI device.
 *
 * @data: Pointer to the sbtsi_data structure containing the device context
 * @reg: Register address to access.
 * @val: Pointer to the value to read into or write from.
 * @is_read: If true, performs a read transfer and stores the result in @val.
 *           If false, performs a write transfer using the value in @val.
 *
 * Returns 0 on success, or a negative error code on failure.
 */
unsafe extern "C" {
    pub fn sbtsi_xfer(
        data: *mut sbtsi_data,
        reg: u8,
        val: *mut u8,
        is_read: bool,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
