/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Copyright (c) 2004 Simtec Electronics
 * Ben Dooks <ben@simtec.co.uk>
 *
 * Header file for s3c2410 standard platform devices
 */

// The C header guard and include are intentionally omitted; required types
// are supplied by the surrounding translation unit.

#[repr(C)]
pub struct s3c24xx_uart_resources {
    pub resources: *mut resource,
    pub nr_resources: libc::c_ulong,
}

// Opaque declarations corresponding to the C types supplied by dependencies.
pub enum resource {}
pub enum platform_device {}

extern "C" {
    pub static mut s3c2410_uart_resources: [s3c24xx_uart_resources; 0];
    pub static mut s3c64xx_uart_resources: [s3c24xx_uart_resources; 0];

    pub static mut s3c24xx_uart_devs: [*mut platform_device; 0];
    pub static mut s3c24xx_uart_src: [*mut platform_device; 0];

    pub static mut s3c64xx_device_iis0: platform_device;
    pub static mut s3c64xx_device_iis1: platform_device;
    pub static mut s3c64xx_device_spi0: platform_device;

    pub static mut s3c_device_fb: platform_device;
    pub static mut s3c_device_hsmmc0: platform_device;
    pub static mut s3c_device_hsmmc1: platform_device;
    pub static mut s3c_device_hsmmc2: platform_device;
    pub static mut s3c_device_hsmmc3: platform_device;
    pub static mut s3c_device_i2c0: platform_device;
    pub static mut s3c_device_i2c1: platform_device;
    pub static mut s3c_device_ohci: platform_device;
    pub static mut s3c_device_usb_hsotg: platform_device;

    pub static mut samsung_device_keypad: platform_device;
    pub static mut samsung_device_pwm: platform_device;

    /**
     * s3c_set_platdata() - helper for setting platform data
     * @pd: The default platform data for this device.
     * @pdsize: The size of the platform data.
     * @pdev: Pointer to the device to fill in.
     *
     * This helper replaces a number of calls that copy and then set the
     * platform data of the device.
     */
    pub fn s3c_set_platdata(
        pd: *mut libc::c_void,
        pdsize: libc::size_t,
        pdev: *mut platform_device,
    ) -> *mut libc::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
