// SPDX-License-Identifier: GPL-2.0
//
//	originally from arch/arm/plat-s3c24xx/devs.c
//
// Copyright (c) 2004 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//
// Base S3C24XX platform device definitions

// Dependencies supplied by the corresponding platform-device and devs headers.
use crate::platform_device;

/* uart devices */

static mut s3c24xx_uart_device0: platform_device = platform_device {
	 id: 0,
};

static mut s3c24xx_uart_device1: platform_device = platform_device {
	 id: 1,
};

static mut s3c24xx_uart_device2: platform_device = platform_device {
	 id: 2,
};

static mut s3c24xx_uart_device3: platform_device = platform_device {
	 id: 3,
};

pub static mut s3c24xx_uart_src: [*mut platform_device; 4] = [
	unsafe { &raw mut s3c24xx_uart_device0 },
	unsafe { &raw mut s3c24xx_uart_device1 },
	unsafe { &raw mut s3c24xx_uart_device2 },
	unsafe { &raw mut s3c24xx_uart_device3 },
];

pub static mut s3c24xx_uart_devs: [*mut platform_device; 4] = [
	core::ptr::null_mut(),
	core::ptr::null_mut(),
	core::ptr::null_mut(),
	core::ptr::null_mut(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
