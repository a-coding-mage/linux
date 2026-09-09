/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *	http://www.samsung.com
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *	http://armlinux.simtec.co.uk/
 *
 * Common Header for S3C64XX machines
 */

/* Dependency supplied externally: linux/reboot.h */

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct map_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pl08x_platform_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn s3c64xx_init_irq(vic0: u32, vic1: u32);
    pub fn s3c64xx_init_io(mach_desc: *mut map_desc, size: c_int);

    pub fn s3c64xx_set_xtal_freq(freq: c_ulong);
    pub fn s3c64xx_set_xusbxti_freq(freq: c_ulong);
}

#[cfg(feature = "CONFIG_CPU_S3C6410")]
extern "C" {
    pub fn s3c6410_init() -> c_int;
    pub fn s3c6410_init_irq();
    pub fn s3c6410_map_io();
}

/* When CONFIG_CPU_S3C6410 is disabled, the C macros expand these names to NULL. */
#[cfg(not(feature = "CONFIG_CPU_S3C6410"))]
pub const s3c6410_map_io: Option<unsafe extern "C" fn()> = None;

#[cfg(not(feature = "CONFIG_CPU_S3C6410"))]
pub const s3c6410_init: Option<unsafe extern "C" fn() -> c_int> = None;

#[cfg(feature = "CONFIG_S3C64XX_PL080")]
extern "C" {
    pub static mut s3c64xx_dma0_plat_data: pl08x_platform_data;
    pub static mut s3c64xx_dma1_plat_data: pl08x_platform_data;
}

/* Samsung HR-Timer Clock mode */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum s3c64xx_timer_mode {
    S3C64XX_PWM0,
    S3C64XX_PWM1,
    S3C64XX_PWM2,
    S3C64XX_PWM3,
    S3C64XX_PWM4,
}

extern "C" {
    pub fn s3c64xx_set_timer_source(
        event: s3c64xx_timer_mode,
        source: s3c64xx_timer_mode,
    );
    pub fn s3c64xx_timer_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
