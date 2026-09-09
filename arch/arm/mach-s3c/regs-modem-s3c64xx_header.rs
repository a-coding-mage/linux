/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      http://armlinux.simtec.co.uk/
 *      Ben Dooks <ben@simtec.co.uk>
 *
 * S3C64XX - modem block registers
 */

/* Original C header guard: __MACH_S3C64XX_REGS_MODEM_H */

#[macro_export]
macro_rules! S3C64XX_MODEMREG {
    ($x:expr) => {
        S3C64XX_VA_MODEM + ($x)
    };
}

pub const S3C64XX_MODEM_INT2AP: usize = S3C64XX_MODEMREG!(0x0);
pub const S3C64XX_MODEM_INT2MODEM: usize = S3C64XX_MODEMREG!(0x4);
pub const S3C64XX_MODEM_MIFCON: usize = S3C64XX_MODEMREG!(0x8);
pub const S3C64XX_MODEM_MIFPCON: usize = S3C64XX_MODEMREG!(0xC);
pub const S3C64XX_MODEM_INTCLR: usize = S3C64XX_MODEMREG!(0x10);
pub const S3C64XX_MODEM_DMA_TXADDR: usize = S3C64XX_MODEMREG!(0x14);
pub const S3C64XX_MODEM_DMA_RXADDR: usize = S3C64XX_MODEMREG!(0x18);

pub const MIFPCON_INT2M_LEVEL: u32 = 1 << 4;
pub const MIFPCON_LCD_BYPASS: u32 = 1 << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
