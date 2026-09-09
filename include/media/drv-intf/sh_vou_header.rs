/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SuperH Video Output Unit (VOU) driver header
 *
 * Copyright (C) 2010, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

// Dependency supplied by the Linux I2C subsystem: linux/i2c.h

/* Bus flags */
pub const SH_VOU_PCLK_FALLING: ::std::os::raw::c_int = 1 << 0;
pub const SH_VOU_HSYNC_LOW: ::std::os::raw::c_int = 1 << 1;
pub const SH_VOU_VSYNC_LOW: ::std::os::raw::c_int = 1 << 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum sh_vou_bus_fmt {
    SH_VOU_BUS_8BIT,
    SH_VOU_BUS_16BIT,
    SH_VOU_BUS_BT656,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sh_vou_pdata {
    pub bus_fmt: sh_vou_bus_fmt,
    pub i2c_adap: ::std::os::raw::c_int,
    pub board_info: *mut i2c_board_info,
    pub flags: ::std::os::raw::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
