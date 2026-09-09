/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Elkhart Lake PSE I/O Auxiliary Device
 *
 * Copyright (c) 2025 Intel Corporation.
 *
 * Author: Raag Jadav <raag.jadav@intel.com>
 */

// Dependency equivalent of: #include <linux/ioport.h>

pub const EHL_PSE_IO_NAME: &str = "ehl_pse_io";
pub const EHL_PSE_GPIO_NAME: &str = "gpio";
pub const EHL_PSE_TIO_NAME: &str = "pps_tio";

#[repr(C)]
pub struct ehl_pse_io_data {
    pub mem: crate::resource,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
