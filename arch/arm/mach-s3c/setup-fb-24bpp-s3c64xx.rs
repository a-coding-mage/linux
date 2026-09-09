// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Base S3C64XX setup information for 24bpp LCD framebuffer

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn s3c_gpio_cfgrange_nopull(gpio: u32, nr: u32, config: u32);

    // These names correspond to the C preprocessor macros used by the source.
    fn S3C64XX_GPI(gpio: u32) -> u32;
    fn S3C64XX_GPJ(gpio: u32) -> u32;
    fn S3C_GPIO_SFN(function: u32) -> u32;
}

pub unsafe fn s3c64xx_fb_gpio_setup_24bpp() {
    s3c_gpio_cfgrange_nopull(S3C64XX_GPI(0), 16, S3C_GPIO_SFN(2));
    s3c_gpio_cfgrange_nopull(S3C64XX_GPJ(0), 12, S3C_GPIO_SFN(2));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
