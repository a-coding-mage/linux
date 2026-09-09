/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2004-2008 Cavium Networks
 */

// The original header guard was __OCTEON_IRQ_H__.

pub const NR_IRQS: i32 = OCTEON_IRQ_LAST;
pub const MIPS_CPU_IRQ_BASE: i32 = OCTEON_IRQ_SW0;

pub type OcteonIrq = i32;

// 1 - 8 represent the 8 MIPS standard interrupt sources.
pub const OCTEON_IRQ_SW0: i32 = 1;
pub const OCTEON_IRQ_SW1: i32 = 2;
// CIU0, CUI2, CIU4 are 3, 4, 5.
pub const OCTEON_IRQ_5: i32 = 6;
pub const OCTEON_IRQ_PERF: i32 = 7;
pub const OCTEON_IRQ_TIMER: i32 = 8;
// Sources in CIU_INTX_EN0.
pub const OCTEON_IRQ_WORKQ0: i32 = 9;
pub const OCTEON_IRQ_WDOG0: i32 = OCTEON_IRQ_WORKQ0 + 64;
pub const OCTEON_IRQ_MBOX0: i32 = OCTEON_IRQ_WDOG0 + 32;
pub const OCTEON_IRQ_MBOX1: i32 = OCTEON_IRQ_MBOX0 + 1;
pub const OCTEON_IRQ_MBOX2: i32 = OCTEON_IRQ_MBOX1 + 1;
pub const OCTEON_IRQ_MBOX3: i32 = OCTEON_IRQ_MBOX2 + 1;
pub const OCTEON_IRQ_PCI_INT0: i32 = OCTEON_IRQ_MBOX3 + 1;
pub const OCTEON_IRQ_PCI_INT1: i32 = OCTEON_IRQ_PCI_INT0 + 1;
pub const OCTEON_IRQ_PCI_INT2: i32 = OCTEON_IRQ_PCI_INT1 + 1;
pub const OCTEON_IRQ_PCI_INT3: i32 = OCTEON_IRQ_PCI_INT2 + 1;
pub const OCTEON_IRQ_PCI_MSI0: i32 = OCTEON_IRQ_PCI_INT3 + 1;
pub const OCTEON_IRQ_PCI_MSI1: i32 = OCTEON_IRQ_PCI_MSI0 + 1;
pub const OCTEON_IRQ_PCI_MSI2: i32 = OCTEON_IRQ_PCI_MSI1 + 1;
pub const OCTEON_IRQ_PCI_MSI3: i32 = OCTEON_IRQ_PCI_MSI2 + 1;
pub const OCTEON_IRQ_TWSI: i32 = OCTEON_IRQ_PCI_MSI3 + 1;
pub const OCTEON_IRQ_TWSI2: i32 = OCTEON_IRQ_TWSI + 1;
pub const OCTEON_IRQ_RML: i32 = OCTEON_IRQ_TWSI2 + 1;
pub const OCTEON_IRQ_TIMER0: i32 = OCTEON_IRQ_RML + 1;
pub const OCTEON_IRQ_TIMER1: i32 = OCTEON_IRQ_TIMER0 + 1;
pub const OCTEON_IRQ_TIMER2: i32 = OCTEON_IRQ_TIMER1 + 1;
pub const OCTEON_IRQ_TIMER3: i32 = OCTEON_IRQ_TIMER2 + 1;

#[cfg(feature = "CONFIG_PCI_MSI")]
pub const OCTEON_IRQ_MSI_BIT0: i32 = 256;

#[cfg(feature = "CONFIG_PCI_MSI")]
pub const OCTEON_IRQ_MSI_LAST: i32 = OCTEON_IRQ_MSI_BIT0 + 255;

#[cfg(feature = "CONFIG_PCI_MSI")]
pub const OCTEON_IRQ_LAST: i32 = OCTEON_IRQ_MSI_LAST + 1;

#[cfg(not(feature = "CONFIG_PCI_MSI"))]
pub const OCTEON_IRQ_LAST: i32 = 127;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
