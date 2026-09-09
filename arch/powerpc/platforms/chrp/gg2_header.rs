/*
 *  include/asm-ppc/gg2.h -- VLSI VAS96011/12 `Golden Gate 2' register definitions
 *
 *  Copyright (C) 1997 Geert Uytterhoeven
 *
 *  This file is based on the following documentation:
 *
 *	The VAS96011/12 Chipset, Data Book, Edition 1.0
 *	VLSI Technology, Inc.
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

/* Memory Map (CHRP mode) */

pub const GG2_PCI_MEM_BASE: u32 = 0xc0000000; /* Peripheral memory space */
pub const GG2_ISA_MEM_BASE: u32 = 0xf7000000; /* Peripheral memory alias */
pub const GG2_ISA_IO_BASE: u32 = 0xf8000000; /* Peripheral I/O space */
pub const GG2_PCI_CONFIG_BASE: u32 = 0xfec00000; /* PCI configuration space */
pub const GG2_INT_ACK_SPECIAL: u32 = 0xfec80000; /* Interrupt acknowledge and */
/* special PCI cycles */
pub const GG2_ROM_BASE0: u32 = 0xff000000; /* ROM bank 0 */
pub const GG2_ROM_BASE1: u32 = 0xff800000; /* ROM bank 1 */

/* GG2 specific PCI Registers */

pub static mut gg2_pci_config_base: *mut core::ffi::c_void = core::ptr::null_mut();
/* kernel virtual address */

pub const GG2_PCI_BUSNO: u32 = 0x40; /* Bus number */
pub const GG2_PCI_SUBBUSNO: u32 = 0x41; /* Subordinate bus number */
pub const GG2_PCI_DISCCTR: u32 = 0x42; /* Disconnect counter */
pub const GG2_PCI_PPC_CTRL: u32 = 0x50; /* PowerPC interface control register */
pub const GG2_PCI_ADDR_MAP: u32 = 0x5c; /* Address map */
pub const GG2_PCI_PCI_CTRL: u32 = 0x60; /* PCI interface control register */
pub const GG2_PCI_ROM_CTRL: u32 = 0x70; /* ROM interface control register */
pub const GG2_PCI_ROM_TIME: u32 = 0x74; /* ROM timing */
pub const GG2_PCI_CC_CTRL: u32 = 0x80; /* Cache controller control register */
pub const GG2_PCI_DRAM_BANK0: u32 = 0x90; /* Control register for DRAM bank #0 */
pub const GG2_PCI_DRAM_BANK1: u32 = 0x94; /* Control register for DRAM bank #1 */
pub const GG2_PCI_DRAM_BANK2: u32 = 0x98; /* Control register for DRAM bank #2 */
pub const GG2_PCI_DRAM_BANK3: u32 = 0x9c; /* Control register for DRAM bank #3 */
pub const GG2_PCI_DRAM_BANK4: u32 = 0xa0; /* Control register for DRAM bank #4 */
pub const GG2_PCI_DRAM_BANK5: u32 = 0xa4; /* Control register for DRAM bank #5 */
pub const GG2_PCI_DRAM_TIME0: u32 = 0xb0; /* Timing parameters set #0 */
pub const GG2_PCI_DRAM_TIME1: u32 = 0xb4; /* Timing parameters set #1 */
pub const GG2_PCI_DRAM_CTRL: u32 = 0xc0; /* DRAM control */
pub const GG2_PCI_ERR_CTRL: u32 = 0xd0; /* Error control register */
pub const GG2_PCI_ERR_STATUS: u32 = 0xd4; /* Error status register */
/* Cleared when read */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
