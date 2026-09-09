/* SPDX-License-Identifier: GPL-2.0
 *
 *	Low-Level PCI Support for SH7780 targets
 *
 *  Dustin McIntire (dustin@sensoria.com) (c) 2001
 *  Paul Mundt (lethal@linux-sh.org) (c) 2003
 */

/* SH7780 Control Registers */
pub const PCIECR: u32 = 0xFE000008;
pub const PCIECR_ENBL: u32 = 0x01;

/* SH7780 Specific Values */
pub const SH7780_PCI_CONFIG_BASE: u32 = 0xFD000000; /* Config space base addr */
pub const SH7780_PCI_CONFIG_SIZE: u32 = 0x01000000; /* Config space size */

pub const SH7780_PCIREG_BASE: u32 = 0xFE040000; /* PCI regs base address */

/* SH7780 PCI Config Registers */
pub const SH7780_PCIIR: u32 = 0x114; /* PCI Interrupt Register */
pub const SH7780_PCIIMR: u32 = 0x118; /* PCI Interrupt Mask Register */
pub const SH7780_PCIAIR: u32 = 0x11C; /* Error Address Register */
pub const SH7780_PCICIR: u32 = 0x120; /* Error Command/Data Register */
pub const SH7780_PCIAINT: u32 = 0x130; /* Arbiter Interrupt Register */
pub const SH7780_PCIAINTM: u32 = 0x134; /* Arbiter Int. Mask Register */
pub const SH7780_PCIBMIR: u32 = 0x138; /* Error Bus Master Register */
pub const SH7780_PCIPAR: u32 = 0x1C0; /* PIO Address Register */
pub const SH7780_PCIPINT: u32 = 0x1CC; /* Power Mgmnt Int. Register */
pub const SH7780_PCIPINTM: u32 = 0x1D0; /* Power Mgmnt Mask Register */

#[inline]
pub const fn SH7780_PCIMBR(x: u32) -> u32 {
    0x1E0u32.wrapping_add(x.wrapping_mul(8))
}

#[inline]
pub const fn SH7780_PCIMBMR(x: u32) -> u32 {
    0x1E4u32.wrapping_add(x.wrapping_mul(8))
}

pub const SH7780_PCIIOBR: u32 = 0x1F8;
pub const SH7780_PCIIOBMR: u32 = 0x1FC;
pub const SH7780_PCICSCR0: u32 = 0x210; /* Cache Snoop1 Cnt. Register */
pub const SH7780_PCICSCR1: u32 = 0x214; /* Cache Snoop2 Cnt. Register */
pub const SH7780_PCICSAR0: u32 = 0x218; /* Cache Snoop1 Addr. Register */
pub const SH7780_PCICSAR1: u32 = 0x21C; /* Cache Snoop2 Addr. Register */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
