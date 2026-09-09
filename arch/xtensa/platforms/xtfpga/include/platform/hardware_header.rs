/*
 * arch/xtensa/platform/xtavnet/include/platform/hardware.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006 Tensilica Inc.
 */

/*
 * This file contains the hardware configuration of the XTAVNET boards.
 */

/* The original header includes <asm/types.h>; its types are supplied externally. */

/* Default assignment of LX60 devices to external interrupts. */

/* Build-time CONFIG_XTENSA_MX condition preserved from the C header. */
#[cfg(feature = "CONFIG_XTENSA_MX")]
macro_rules! DUART16552_INTNUM { () => { XCHAL_EXTINT3_NUM }; }
#[cfg(feature = "CONFIG_XTENSA_MX")]
macro_rules! OETH_IRQ { () => { XCHAL_EXTINT4_NUM }; }
#[cfg(feature = "CONFIG_XTENSA_MX")]
macro_rules! C67X00_IRQ { () => { XCHAL_EXTINT8_NUM }; }

#[cfg(not(feature = "CONFIG_XTENSA_MX"))]
macro_rules! DUART16552_INTNUM { () => { XCHAL_EXTINT0_NUM }; }
#[cfg(not(feature = "CONFIG_XTENSA_MX"))]
macro_rules! OETH_IRQ { () => { XCHAL_EXTINT1_NUM }; }
#[cfg(not(feature = "CONFIG_XTENSA_MX"))]
macro_rules! C67X00_IRQ { () => { XCHAL_EXTINT5_NUM }; }

/*
 * Device addresses and parameters.
 */

/* UART */
macro_rules! DUART16552_PADDR { () => { XCHAL_KIO_PADDR + 0x0D050020 }; }

/* Misc. */
macro_rules! XTFPGA_FPGAREGS_VADDR { () => { IOADDR(0x0D020000) }; }
/* Clock frequency in Hz (read-only): */
macro_rules! XTFPGA_CLKFRQ_VADDR { () => { XTFPGA_FPGAREGS_VADDR!() + 0x04 }; }
/* Setting of 8 DIP switches: */
macro_rules! DIP_SWITCHES_VADDR { () => { XTFPGA_FPGAREGS_VADDR!() + 0x0C }; }
/* Software reset (write 0xdead): */
macro_rules! XTFPGA_SWRST_VADDR { () => { XTFPGA_FPGAREGS_VADDR!() + 0x10 }; }

/* OpenCores Ethernet controller: */
/* regs + RX/TX descriptors */
macro_rules! OETH_REGS_PADDR { () => { XCHAL_KIO_PADDR + 0x0D030000 }; }
macro_rules! OETH_REGS_SIZE { () => { 0x1000 }; }
macro_rules! OETH_SRAMBUFF_PADDR { () => { XCHAL_KIO_PADDR + 0x0D800000 }; }

/* 5*rx buffs + 5*tx buffs */
macro_rules! OETH_SRAMBUFF_SIZE { () => { 5 * 0x600 + 5 * 0x600 }; }

macro_rules! C67X00_PADDR { () => { XCHAL_KIO_PADDR + 0x0D0D0000 }; }
macro_rules! C67X00_SIZE { () => { 0x10 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
