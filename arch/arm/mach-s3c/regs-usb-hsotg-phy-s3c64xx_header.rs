/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      http://armlinux.simtec.co.uk/
 *      Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - USB2.0 Highspeed/OtG device PHY registers
 */

/* Note, this is a separate header file as some of the clock framework
 * needs to touch this if the clk_48m is used as the USB OHCI or other
 * peripheral source.
 */

/* S3C64XX_PA_USB_HSPHY */

macro_rules! S3C_HSOTG_PHYREG {
    ($x:expr) => {
        ($x) + S3C_VA_USB_HSPHY
    };
}

pub const S3C_PHYPWR: usize = S3C_HSOTG_PHYREG!(0x00);
pub const S3C_PHYPWR_NORMAL_MASK: usize = 0x19 << 0;
pub const S3C_PHYPWR_OTG_DISABLE: usize = 1 << 4;
pub const S3C_PHYPWR_ANALOG_POWERDOWN: usize = 1 << 3;
pub const SRC_PHYPWR_FORCE_SUSPEND: usize = 1 << 1;

pub const S3C_PHYCLK: usize = S3C_HSOTG_PHYREG!(0x04);
pub const S3C_PHYCLK_MODE_USB11: usize = 1 << 6;
pub const S3C_PHYCLK_EXT_OSC: usize = 1 << 5;
pub const S3C_PHYCLK_CLK_FORCE: usize = 1 << 4;
pub const S3C_PHYCLK_ID_PULL: usize = 1 << 2;
pub const S3C_PHYCLK_CLKSEL_MASK: usize = 0x3 << 0;
pub const S3C_PHYCLK_CLKSEL_SHIFT: usize = 0;
pub const S3C_PHYCLK_CLKSEL_48M: usize = 0x0 << 0;
pub const S3C_PHYCLK_CLKSEL_12M: usize = 0x2 << 0;
pub const S3C_PHYCLK_CLKSEL_24M: usize = 0x3 << 0;

pub const S3C_RSTCON: usize = S3C_HSOTG_PHYREG!(0x08);
pub const S3C_RSTCON_PHYCLK: usize = 1 << 2;
pub const S3C_RSTCON_HCLK: usize = 1 << 1;
pub const S3C_RSTCON_PHY: usize = 1 << 0;

pub const S3C_PHYTUNE: usize = S3C_HSOTG_PHYREG!(0x20);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
