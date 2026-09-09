/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/hardware/ioc.h
 *
 *  Copyright (C) Russell King
 *
 *  Use these macros to read/write the IOC.  All it does is perform the actual
 *  read/write.
 */

/* The original header excludes these accessors during assembly builds. */

/*
 * We use __raw_base variants here so that we give the compiler the
 * chance to keep IOC_BASE in a register.
 */
#[macro_export]
macro_rules! ioc_readb {
    ($off:expr) => {
        __raw_readb(IOC_BASE + ($off))
    };
}

#[macro_export]
macro_rules! ioc_writeb {
    ($val:expr, $off:expr) => {
        __raw_writeb($val, IOC_BASE + ($off))
    };
}

pub const IOC_CONTROL: u32 = 0x00;
pub const IOC_KARTTX: u32 = 0x04;
pub const IOC_KARTRX: u32 = 0x04;

pub const IOC_IRQSTATA: u32 = 0x10;
pub const IOC_IRQREQA: u32 = 0x14;
pub const IOC_IRQCLRA: u32 = 0x14;
pub const IOC_IRQMASKA: u32 = 0x18;

pub const IOC_IRQSTATB: u32 = 0x20;
pub const IOC_IRQREQB: u32 = 0x24;
pub const IOC_IRQMASKB: u32 = 0x28;

pub const IOC_FIQSTAT: u32 = 0x30;
pub const IOC_FIQREQ: u32 = 0x34;
pub const IOC_FIQMASK: u32 = 0x38;

pub const IOC_T0CNTL: u32 = 0x40;
pub const IOC_T0LTCHL: u32 = 0x40;
pub const IOC_T0CNTH: u32 = 0x44;
pub const IOC_T0LTCHH: u32 = 0x44;
pub const IOC_T0GO: u32 = 0x48;
pub const IOC_T0LATCH: u32 = 0x4c;

pub const IOC_T1CNTL: u32 = 0x50;
pub const IOC_T1LTCHL: u32 = 0x50;
pub const IOC_T1CNTH: u32 = 0x54;
pub const IOC_T1LTCHH: u32 = 0x54;
pub const IOC_T1GO: u32 = 0x58;
pub const IOC_T1LATCH: u32 = 0x5c;

pub const IOC_T2CNTL: u32 = 0x60;
pub const IOC_T2LTCHL: u32 = 0x60;
pub const IOC_T2CNTH: u32 = 0x64;
pub const IOC_T2LTCHH: u32 = 0x64;
pub const IOC_T2GO: u32 = 0x68;
pub const IOC_T2LATCH: u32 = 0x6c;

pub const IOC_T3CNTL: u32 = 0x70;
pub const IOC_T3LTCHL: u32 = 0x70;
pub const IOC_T3CNTH: u32 = 0x74;
pub const IOC_T3LTCHH: u32 = 0x74;
pub const IOC_T3GO: u32 = 0x78;
pub const IOC_T3LATCH: u32 = 0x7c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
