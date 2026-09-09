/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Congatec Board Controller driver definitions
 *
 * Copyright (C) 2024 Bootlin
 * Author: Thomas Richard <thomas.richard@bootlin.com>
 */

use core::ffi::c_void;

/// Opaque kernel device structure supplied by an external dependency.
pub enum device {}

/// Opaque kernel mutex structure supplied by an external dependency.
pub enum mutex {}

/**
 * struct cgbc_version - Board Controller device version structure
 * @feature: Board Controller feature number
 * @major:   Board Controller major revision
 * @minor:   Board Controller minor revision
 */
#[repr(C)]
pub struct cgbc_version {
    pub feature: u8,
    pub major: u8,
    pub minor: u8,
}

/**
 * struct cgbc_device_data - Internal representation of the Board Controller device
 * @io_session:       Pointer to the session IO memory
 * @io_cmd:           Pointer to the command IO memory
 * @session:          Session id returned by the Board Controller
 * @dev:              Pointer to kernel device structure
 * @version:          Board Controller version structure
 * @lock:             Board Controller mutex
 */
#[repr(C)]
pub struct cgbc_device_data {
    pub io_session: *mut c_void,
    pub io_cmd: *mut c_void,
    pub session: u8,
    pub dev: *mut device,
    pub version: cgbc_version,
    pub lock: mutex,
}

extern "C" {
    pub fn cgbc_command(
        cgbc: *mut cgbc_device_data,
        cmd: *mut c_void,
        cmd_size: u32,
        data: *mut c_void,
        data_size: u32,
        status: *mut u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
