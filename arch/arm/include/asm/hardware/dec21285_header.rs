/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/hardware/dec21285.h
 *
 *  Copyright (C) 1998 Russell King
 *
 *  DC21285 registers
 */

pub const DC21285_PCI_IACK: usize = 0x79000000;
pub const DC21285_ARMCSR_BASE: usize = 0x42000000;
pub const DC21285_PCI_TYPE_0_CONFIG: usize = 0x7b000000;
pub const DC21285_PCI_TYPE_1_CONFIG: usize = 0x7a000000;
pub const DC21285_OUTBOUND_WRITE_FLUSH: usize = 0x78000000;
pub const DC21285_FLASH: usize = 0x41000000;
pub const DC21285_PCI_IO: usize = 0x7c000000;
pub const DC21285_PCI_MEM: usize = 0x80000000;

/* The non-assembly form uses ARMCSR_BASE from mach/hardware.h. */
#[macro_export]
macro_rules! DC21285_IO {
    ($x:expr) => {
        ((ARMCSR_BASE + ($x)) as *mut core::ffi::c_ulong)
    };
}

/* The footbridge is programmed to expose the system RAM at 0xe0000000.
 * The requirement is that the RAM isn't placed at bus address 0, which
 * would clash with VGA cards.
 */
pub const BUS_OFFSET: usize = 0xe0000000;

pub const CSR_PCICMD: *mut core::ffi::c_ulong = DC21285_IO!(0x0004);
pub const CSR_CLASSREV: *mut core::ffi::c_ulong = DC21285_IO!(0x0008);
pub const CSR_PCICACHELINESIZE: *mut core::ffi::c_ulong = DC21285_IO!(0x000c);
pub const CSR_PCICSRBASE: *mut core::ffi::c_ulong = DC21285_IO!(0x0010);
pub const CSR_PCICSRIOBASE: *mut core::ffi::c_ulong = DC21285_IO!(0x0014);
pub const CSR_PCISDRAMBASE: *mut core::ffi::c_ulong = DC21285_IO!(0x0018);
pub const CSR_PCIROMBASE: *mut core::ffi::c_ulong = DC21285_IO!(0x0030);
pub const CSR_MBOX0: *mut core::ffi::c_ulong = DC21285_IO!(0x0050);
pub const CSR_MBOX1: *mut core::ffi::c_ulong = DC21285_IO!(0x0054);
pub const CSR_MBOX2: *mut core::ffi::c_ulong = DC21285_IO!(0x0058);
pub const CSR_MBOX3: *mut core::ffi::c_ulong = DC21285_IO!(0x005c);
pub const CSR_DOORBELL: *mut core::ffi::c_ulong = DC21285_IO!(0x0060);
pub const CSR_DOORBELL_SETUP: *mut core::ffi::c_ulong = DC21285_IO!(0x0064);
pub const CSR_ROMWRITEREG: *mut core::ffi::c_ulong = DC21285_IO!(0x0068);
pub const CSR_CSRBASEMASK: *mut core::ffi::c_ulong = DC21285_IO!(0x00f8);
pub const CSR_CSRBASEOFFSET: *mut core::ffi::c_ulong = DC21285_IO!(0x00fc);
pub const CSR_SDRAMBASEMASK: *mut core::ffi::c_ulong = DC21285_IO!(0x0100);
pub const CSR_SDRAMBASEOFFSET: *mut core::ffi::c_ulong = DC21285_IO!(0x0104);
pub const CSR_ROMBASEMASK: *mut core::ffi::c_ulong = DC21285_IO!(0x0108);
pub const CSR_SDRAMTIMING: *mut core::ffi::c_ulong = DC21285_IO!(0x010c);
pub const CSR_SDRAMADDRSIZE0: *mut core::ffi::c_ulong = DC21285_IO!(0x0110);
pub const CSR_SDRAMADDRSIZE1: *mut core::ffi::c_ulong = DC21285_IO!(0x0114);
pub const CSR_SDRAMADDRSIZE2: *mut core::ffi::c_ulong = DC21285_IO!(0x0118);
pub const CSR_SDRAMADDRSIZE3: *mut core::ffi::c_ulong = DC21285_IO!(0x011c);
pub const CSR_I2O_INFREEHEAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0120);
pub const CSR_I2O_INPOSTTAIL: *mut core::ffi::c_ulong = DC21285_IO!(0x0124);
pub const CSR_I2O_OUTPOSTHEAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0128);
pub const CSR_I2O_OUTFREETAIL: *mut core::ffi::c_ulong = DC21285_IO!(0x012c);
pub const CSR_I2O_INFREECOUNT: *mut core::ffi::c_ulong = DC21285_IO!(0x0130);
pub const CSR_I2O_OUTPOSTCOUNT: *mut core::ffi::c_ulong = DC21285_IO!(0x0134);
pub const CSR_I2O_INPOSTCOUNT: *mut core::ffi::c_ulong = DC21285_IO!(0x0138);
pub const CSR_SA110_CNTL: *mut core::ffi::c_ulong = DC21285_IO!(0x013c);

pub const SA110_CNTL_INITCMPLETE: usize = 1 << 0;
pub const SA110_CNTL_ASSERTSERR: usize = 1 << 1;
pub const SA110_CNTL_RXSERR: usize = 1 << 3;
pub const SA110_CNTL_SA110DRAMPARITY: usize = 1 << 4;
pub const SA110_CNTL_PCISDRAMPARITY: usize = 1 << 5;
pub const SA110_CNTL_DMASDRAMPARITY: usize = 1 << 6;
pub const SA110_CNTL_DISCARDTIMER: usize = 1 << 8;
pub const SA110_CNTL_PCINRESET: usize = 1 << 9;
pub const SA110_CNTL_I2O_256: usize = 0 << 10;
pub const SA110_CNTL_I20_512: usize = 1 << 10;
pub const SA110_CNTL_I2O_1024: usize = 2 << 10;
pub const SA110_CNTL_I2O_2048: usize = 3 << 10;
pub const SA110_CNTL_I2O_4096: usize = 4 << 10;
pub const SA110_CNTL_I2O_8192: usize = 5 << 10;
pub const SA110_CNTL_I2O_16384: usize = 6 << 10;
pub const SA110_CNTL_I2O_32768: usize = 7 << 10;
pub const SA110_CNTL_WATCHDOG: usize = 1 << 13;
pub const SA110_CNTL_ROMWIDTH_UNDEF: usize = 0 << 14;
pub const SA110_CNTL_ROMWIDTH_16: usize = 1 << 14;
pub const SA110_CNTL_ROMWIDTH_32: usize = 2 << 14;
pub const SA110_CNTL_ROMWIDTH_8: usize = 3 << 14;
#[inline]
pub const fn SA110_CNTL_ROMACCESSTIME(x: usize) -> usize { x << 16 }
#[inline]
pub const fn SA110_CNTL_ROMBURSTTIME(x: usize) -> usize { x << 20 }
#[inline]
pub const fn SA110_CNTL_ROMTRISTATETIME(x: usize) -> usize { x << 24 }
#[inline]
pub const fn SA110_CNTL_XCSDIR(x: usize) -> usize { x << 28 }
pub const SA110_CNTL_PCICFN: usize = 1 << 31;

pub const CSR_PCIADDR_EXTN: *mut core::ffi::c_ulong = DC21285_IO!(0x0140);
pub const CSR_PREFETCHMEMRANGE: *mut core::ffi::c_ulong = DC21285_IO!(0x0144);
pub const CSR_XBUS_CYCLE: *mut core::ffi::c_ulong = DC21285_IO!(0x0148);
pub const CSR_XBUS_IOSTROBE: *mut core::ffi::c_ulong = DC21285_IO!(0x014c);
pub const CSR_DOORBELL_PCI: *mut core::ffi::c_ulong = DC21285_IO!(0x0150);
pub const CSR_DOORBELL_SA110: *mut core::ffi::c_ulong = DC21285_IO!(0x0154);
pub const CSR_UARTDR: *mut core::ffi::c_ulong = DC21285_IO!(0x0160);
pub const CSR_RXSTAT: *mut core::ffi::c_ulong = DC21285_IO!(0x0164);
pub const CSR_H_UBRLCR: *mut core::ffi::c_ulong = DC21285_IO!(0x0168);
pub const CSR_M_UBRLCR: *mut core::ffi::c_ulong = DC21285_IO!(0x016c);
pub const CSR_L_UBRLCR: *mut core::ffi::c_ulong = DC21285_IO!(0x0170);
pub const CSR_UARTCON: *mut core::ffi::c_ulong = DC21285_IO!(0x0174);
pub const CSR_UARTFLG: *mut core::ffi::c_ulong = DC21285_IO!(0x0178);
pub const CSR_IRQ_STATUS: *mut core::ffi::c_ulong = DC21285_IO!(0x0180);
pub const CSR_IRQ_RAWSTATUS: *mut core::ffi::c_ulong = DC21285_IO!(0x0184);
pub const CSR_IRQ_ENABLE: *mut core::ffi::c_ulong = DC21285_IO!(0x0188);
pub const CSR_IRQ_DISABLE: *mut core::ffi::c_ulong = DC21285_IO!(0x018c);
pub const CSR_IRQ_SOFT: *mut core::ffi::c_ulong = DC21285_IO!(0x0190);
pub const CSR_FIQ_STATUS: *mut core::ffi::c_ulong = DC21285_IO!(0x0280);
pub const CSR_FIQ_RAWSTATUS: *mut core::ffi::c_ulong = DC21285_IO!(0x0284);
pub const CSR_FIQ_ENABLE: *mut core::ffi::c_ulong = DC21285_IO!(0x0288);
pub const CSR_FIQ_DISABLE: *mut core::ffi::c_ulong = DC21285_IO!(0x028c);
pub const CSR_FIQ_SOFT: *mut core::ffi::c_ulong = DC21285_IO!(0x0290);

pub const CSR_TIMER1_LOAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0300);
pub const CSR_TIMER1_VALUE: *mut core::ffi::c_ulong = DC21285_IO!(0x0304);
pub const CSR_TIMER1_CNTL: *mut core::ffi::c_ulong = DC21285_IO!(0x0308);
pub const CSR_TIMER1_CLR: *mut core::ffi::c_ulong = DC21285_IO!(0x030c);
pub const CSR_TIMER2_LOAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0320);
pub const CSR_TIMER2_VALUE: *mut core::ffi::c_ulong = DC21285_IO!(0x0324);
pub const CSR_TIMER2_CNTL: *mut core::ffi::c_ulong = DC21285_IO!(0x0328);
pub const CSR_TIMER2_CLR: *mut core::ffi::c_ulong = DC21285_IO!(0x032c);
pub const CSR_TIMER3_LOAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0340);
pub const CSR_TIMER3_VALUE: *mut core::ffi::c_ulong = DC21285_IO!(0x0344);
pub const CSR_TIMER3_CNTL: *mut core::ffi::c_ulong = DC21285_IO!(0x0348);
pub const CSR_TIMER3_CLR: *mut core::ffi::c_ulong = DC21285_IO!(0x034c);
pub const CSR_TIMER4_LOAD: *mut core::ffi::c_ulong = DC21285_IO!(0x0360);
pub const CSR_TIMER4_VALUE: *mut core::ffi::c_ulong = DC21285_IO!(0x0364);
pub const CSR_TIMER4_CNTL: *mut core::ffi::c_ulong = DC21285_IO!(0x0368);
pub const CSR_TIMER4_CLR: *mut core::ffi::c_ulong = DC21285_IO!(0x036c);

pub const TIMER_CNTL_ENABLE: usize = 1 << 7;
pub const TIMER_CNTL_AUTORELOAD: usize = 1 << 6;
pub const TIMER_CNTL_DIV1: usize = 0;
pub const TIMER_CNTL_DIV16: usize = 1 << 2;
pub const TIMER_CNTL_DIV256: usize = 2 << 2;
pub const TIMER_CNTL_CNTEXT: usize = 3 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
