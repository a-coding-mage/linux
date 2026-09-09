/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industrial I/O configfs support
 *
 * Copyright (c) 2015 Intel Corporation
 */

// The C header guard (__IIO_CONFIGFS) has no executable Rust equivalent.

// Supplied by the configfs dependency.
#[repr(C)]
pub struct configfs_subsystem {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut iio_configfs_subsys: configfs_subsystem;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
