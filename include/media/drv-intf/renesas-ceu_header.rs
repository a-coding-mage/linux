// SPDX-License-Identifier: GPL-2.0
/*
 * renesas-ceu.h - Renesas CEU driver interface
 *
 * Copyright 2017-2018 Jacopo Mondi <jacopo+renesas@jmondi.org>
 */

use core::ffi::c_ulong;

pub const CEU_MAX_SUBDEVS: usize = 2;

#[repr(C)]
pub struct ceu_async_subdev {
    pub flags: c_ulong,
    pub bus_width: u8,
    pub bus_shift: u8,
    pub i2c_adapter_id: u32,
    pub i2c_address: u32,
}

#[repr(C)]
pub struct ceu_platform_data {
    pub num_subdevs: u32,
    pub subdevs: [ceu_async_subdev; CEU_MAX_SUBDEVS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
