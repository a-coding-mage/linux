/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *	http://armlinux.simtec.co.uk/
 *
 * S3C64XX system register definitions
 */

// C header guard: __MACH_S3C64XX_REGS_SYS_H

// S3C_VA_SYS is supplied by the surrounding platform definitions.
#[macro_export]
macro_rules! S3C_SYSREG {
    ($x:expr) => {
        S3C_VA_SYS + ($x)
    };
}

pub const S3C64XX_AHB_CON0: usize = S3C_SYSREG!(0x100);
pub const S3C64XX_AHB_CON1: usize = S3C_SYSREG!(0x104);
pub const S3C64XX_AHB_CON2: usize = S3C_SYSREG!(0x108);

pub const S3C64XX_SDMA_SEL: usize = S3C_SYSREG!(0x110);

pub const S3C64XX_OTHERS: usize = S3C_SYSREG!(0x900);

pub const S3C64XX_OTHERS_USBMASK: usize = 1usize << 16;
pub const S3C64XX_OTHERS_SYNCMUXSEL: usize = 1usize << 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
