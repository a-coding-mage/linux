/* SPDX-License-Identifier: GPL-2.0 */
//
// arch/sh/include/mach-landisk/mach/iodata_landisk.h
//
// Copyright (C) 2000  Atom Create Engineering Co., Ltd.
//
// IO-DATA LANDISK support
//
// C dependency: <linux/sh_intc.h>

/* Box specific addresses. */

pub const PA_USB: u32 = 0xa4000000; /* USB Controller M66590 */

pub const PA_ATARST: u32 = 0xb0000000; /* ATA/FATA Access Control Register */
pub const PA_LED: u32 = 0xb0000001; /* LED Control Register */
pub const PA_STATUS: u32 = 0xb0000002; /* Switch Status Register */
pub const PA_SHUTDOWN: u32 = 0xb0000003; /* Shutdown Control Register */
pub const PA_PCIPME: u32 = 0xb0000004; /* PCI PME Status Register */
pub const PA_IMASK: u32 = 0xb0000005; /* Interrupt Mask Register */
/* 2003.10.31 I-O DATA NSD NWG add. for shutdown port clear */
pub const PA_PWRINT_CLR: u32 = 0xb0000006; /* Shutdown Interrupt clear Register */

pub const PA_PIDE_OFFSET: u32 = 0x40; /* CF IDE Offset */
pub const PA_SIDE_OFFSET: u32 = 0x40; /* HDD IDE Offset */

/* evt2irq is supplied by the interrupt-controller dependency. */
unsafe extern "C" {
    pub fn evt2irq(event: u32) -> i32;
}

pub const IRQ_PCIINTA: i32 = unsafe { evt2irq(0x2a0) }; /* PCI INTA IRQ */
pub const IRQ_PCIINTB: i32 = unsafe { evt2irq(0x2c0) }; /* PCI INTB IRQ */
pub const IRQ_PCIINTC: i32 = unsafe { evt2irq(0x2e0) }; /* PCI INTC IRQ */
pub const IRQ_PCIINTD: i32 = unsafe { evt2irq(0x300) }; /* PCI INTD IRQ */
pub const IRQ_ATA: i32 = unsafe { evt2irq(0x320) }; /* ATA IRQ */
pub const IRQ_FATA: i32 = unsafe { evt2irq(0x340) }; /* FATA IRQ */
pub const IRQ_POWER: i32 = unsafe { evt2irq(0x360) }; /* Power Switch IRQ */
pub const IRQ_BUTTON: i32 = unsafe { evt2irq(0x380) }; /* USL-5P Button IRQ */
pub const IRQ_FAULT: i32 = unsafe { evt2irq(0x3a0) }; /* USL-5P Fault  IRQ */

unsafe extern "C" {
    pub fn init_landisk_IRQ();
}

// C macro dependency: __IO_PREFIX landisk
// C include dependency: <asm/io_generic.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
