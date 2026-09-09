/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - FB platform data definitions
 */

// Dependency supplied by <linux/platform_data/video_s3c.h>.
#[repr(C)]
pub struct s3c_fb_platdata {
    _private: [u8; 0],
}

/**
 * s3c_fb_set_platdata() - Setup the FB device with platform data.
 * @pd: The platform data to set. The data is copied from the passed structure
 *      so the machine data can mark the data __initdata so that any unused
 *      machines will end up dumping their data at runtime.
 */
unsafe extern "C" {
    pub fn s3c_fb_set_platdata(pd: *mut s3c_fb_platdata);
}

/**
 * s3c64xx_fb_gpio_setup_24bpp() - S3C64XX setup function for 24bpp LCD
 *
 * Initialise the GPIO for an 24bpp LCD display on the RGB interface.
 */
unsafe extern "C" {
    pub fn s3c64xx_fb_gpio_setup_24bpp();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
