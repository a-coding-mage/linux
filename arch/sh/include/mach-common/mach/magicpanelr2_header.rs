/* SPDX-License-Identifier: GPL-2.0
 *
 *  include/asm-sh/magicpanelr2.h
 *
 *  Copyright (C) 2007  Markus Brunner, Mark Jonas
 *
 *  I/O addresses and bitmasks for Magic Panel Release 2 board
 */

// C header guard omitted.
// `__IO_PREFIX mpr2` and the dependency on <asm/io_generic.h> are supplied by
// the surrounding translation unit.

macro_rules! SETBITS_OUTB {
    ($mask:expr, $reg:expr) => {
        __raw_writeb(__raw_readb($reg) | $mask, $reg)
    };
}
macro_rules! SETBITS_OUTW {
    ($mask:expr, $reg:expr) => {
        __raw_writew(__raw_readw($reg) | $mask, $reg)
    };
}
macro_rules! SETBITS_OUTL {
    ($mask:expr, $reg:expr) => {
        __raw_writel(__raw_readl($reg) | $mask, $reg)
    };
}
macro_rules! CLRBITS_OUTB {
    ($mask:expr, $reg:expr) => {
        __raw_writeb(__raw_readb($reg) & !$mask, $reg)
    };
}
macro_rules! CLRBITS_OUTW {
    ($mask:expr, $reg:expr) => {
        __raw_writew(__raw_readw($reg) & !$mask, $reg)
    };
}
macro_rules! CLRBITS_OUTL {
    ($mask:expr, $reg:expr) => {
        __raw_writel(__raw_readl($reg) & !$mask, $reg)
    };
}

pub const PA_LED: usize = PORT_PADR; /* LED */

/* BSC */
pub const CMNCR: u32 = 0xA4FD0000;
pub const CS0BCR: u32 = 0xA4FD0004;
pub const CS2BCR: u32 = 0xA4FD0008;
pub const CS3BCR: u32 = 0xA4FD000C;
pub const CS4BCR: u32 = 0xA4FD0010;
pub const CS5ABCR: u32 = 0xA4FD0014;
pub const CS5BBCR: u32 = 0xA4FD0018;
pub const CS6ABCR: u32 = 0xA4FD001C;
pub const CS6BBCR: u32 = 0xA4FD0020;
pub const CS0WCR: u32 = 0xA4FD0024;
pub const CS2WCR: u32 = 0xA4FD0028;
pub const CS3WCR: u32 = 0xA4FD002C;
pub const CS4WCR: u32 = 0xA4FD0030;
pub const CS5AWCR: u32 = 0xA4FD0034;
pub const CS5BWCR: u32 = 0xA4FD0038;
pub const CS6AWCR: u32 = 0xA4FD003C;
pub const CS6BWCR: u32 = 0xA4FD0040;

/* usb */
pub const PORT_UTRCTL: u32 = 0xA405012C;
pub const PORT_UCLKCR_W: u32 = 0xA40A0008;

pub const INTC_ICR0: u32 = 0xA414FEE0;
pub const INTC_ICR1: u32 = 0xA4140010;
pub const INTC_ICR2: u32 = 0xA4140012;

/* MTD */
pub const MPR2_MTD_BOOTLOADER_SIZE: u32 = 0x00060000;
pub const MPR2_MTD_KERNEL_SIZE: u32 = 0x00200000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
