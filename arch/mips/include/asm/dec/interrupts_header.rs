/*
 * Miscellaneous definitions used to initialise the interrupt vector table
 * with the machine-specific interrupt routines.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997 by Paul M. Antoine.
 * reworked 1998 by Harald Koerfgen.
 * Copyright (C) 2001, 2002, 2003  Maciej W. Rozycki
 */

use core::ffi::c_void;

/* The following names are supplied by the corresponding IRQ/MIPS headers. */

pub const DEC_IRQ_CASCADE: i32 = 0; /* cascade from CSR or I/O ASIC */

/* Ordinary interrupts */
pub const DEC_IRQ_AB_RECV: i32 = 1; /* ACCESS.bus receive */
pub const DEC_IRQ_AB_XMIT: i32 = 2; /* ACCESS.bus transmit */
pub const DEC_IRQ_DZ11: i32 = 3; /* DZ11 (DC7085) serial */
pub const DEC_IRQ_ASC: i32 = 4; /* ASC (NCR53C94) SCSI */
pub const DEC_IRQ_FLOPPY: i32 = 5; /* 82077 FDC */
pub const DEC_IRQ_FPU: i32 = 6; /* R3k FPU */
pub const DEC_IRQ_HALT: i32 = 7; /* HALT button or from ACCESS.Bus */
pub const DEC_IRQ_ISDN: i32 = 8; /* Am79C30A ISDN */
pub const DEC_IRQ_LANCE: i32 = 9; /* LANCE (Am7990) Ethernet */
pub const DEC_IRQ_BUS: i32 = 10; /* memory, I/O bus read/write errors */
pub const DEC_IRQ_PSU: i32 = 11; /* power supply unit warning */
pub const DEC_IRQ_RTC: i32 = 12; /* DS1287 RTC */
pub const DEC_IRQ_SCC0: i32 = 13; /* SCC (Z85C30) serial #0 */
pub const DEC_IRQ_SCC1: i32 = 14; /* SCC (Z85C30) serial #1 */
pub const DEC_IRQ_SII: i32 = 15; /* SII (DC7061) SCSI */
pub const DEC_IRQ_TC0: i32 = 16; /* TURBOchannel slot #0 */
pub const DEC_IRQ_TC1: i32 = 17; /* TURBOchannel slot #1 */
pub const DEC_IRQ_TC2: i32 = 18; /* TURBOchannel slot #2 */
pub const DEC_IRQ_TIMER: i32 = 19; /* ARC periodic timer */
pub const DEC_IRQ_VIDEO: i32 = 20; /* framebuffer */

/* I/O ASIC DMA interrupts */
pub const DEC_IRQ_ASC_MERR: i32 = 21;
pub const DEC_IRQ_ASC_ERR: i32 = 22;
pub const DEC_IRQ_ASC_DMA: i32 = 23;
pub const DEC_IRQ_FLOPPY_ERR: i32 = 24;
pub const DEC_IRQ_ISDN_ERR: i32 = 25;
pub const DEC_IRQ_ISDN_RXDMA: i32 = 26;
pub const DEC_IRQ_ISDN_TXDMA: i32 = 27;
pub const DEC_IRQ_LANCE_MERR: i32 = 28;
pub const DEC_IRQ_SCC0A_RXERR: i32 = 29;
pub const DEC_IRQ_SCC0A_RXDMA: i32 = 30;
pub const DEC_IRQ_SCC0A_TXERR: i32 = 31;
pub const DEC_IRQ_SCC0A_TXDMA: i32 = 32;
pub const DEC_IRQ_AB_RXERR: i32 = 33;
pub const DEC_IRQ_AB_RXDMA: i32 = 34;
pub const DEC_IRQ_AB_TXERR: i32 = 35;
pub const DEC_IRQ_AB_TXDMA: i32 = 36;
pub const DEC_IRQ_SCC1A_RXERR: i32 = 37;
pub const DEC_IRQ_SCC1A_RXDMA: i32 = 38;
pub const DEC_IRQ_SCC1A_TXERR: i32 = 39;
pub const DEC_IRQ_SCC1A_TXDMA: i32 = 40;

/* TC5 & TC6 are virtual slots for KN02's onboard devices */
pub const DEC_IRQ_TC5: i32 = DEC_IRQ_ASC;
pub const DEC_IRQ_TC6: i32 = DEC_IRQ_LANCE;
pub const DEC_NR_INTS: usize = 41;

pub const DEC_MAX_CPU_INTS: usize = 6;
pub const DEC_MAX_ASIC_INTS: usize = 9;

pub const DEC_CPU_INR_FPU: i32 = 7;
pub const DEC_CPU_INR_SW1: i32 = 1;
pub const DEC_CPU_INR_SW0: i32 = 0;

pub const DEC_CPU_IRQ_BASE: i32 = MIPS_CPU_IRQ_BASE;

#[inline]
pub const fn DEC_CPU_IRQ_NR(n: i32) -> i32 { n + DEC_CPU_IRQ_BASE }
#[inline]
pub const fn DEC_CPU_IRQ_MASK(n: i32) -> i32 { 1 << (n + CAUSEB_IP) }
pub const DEC_CPU_IRQ_ALL: i32 = 0xff << CAUSEB_IP;

#[repr(C)]
pub union int_ptr {
    pub i: i32,
    pub p: *mut c_void,
}

extern "C" {
    pub static mut dec_interrupt: [i32; DEC_NR_INTS];
    pub static mut cpu_mask_nr_tbl: [[int_ptr; 2]; DEC_MAX_CPU_INTS];
    pub static mut asic_mask_nr_tbl: [[int_ptr; 2]; DEC_MAX_ASIC_INTS];
    pub static mut cpu_fpu_mask: i32;

    pub fn kn02_io_int();
    pub fn kn02xa_io_int();
    pub fn kn03_io_int();
    pub fn asic_dma_int();
    pub fn asic_all_int();
    pub fn kn02_all_int();
    pub fn cpu_all_int();

    pub fn dec_intr_unimplemented();
    pub fn asic_intr_unimplemented();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
