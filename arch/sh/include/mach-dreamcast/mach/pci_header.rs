/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/dreamcast/pci.h
 *
 * Copyright (C) 2001, 2002  M. R. Brown
 * Copyright (C) 2002, 2003  Paul Mundt
 */

// Dependency equivalent of: #include <mach-dreamcast/mach/sysasic.h>

pub const GAPSPCI_REGS: u32 = 0x0100_1400;
pub const GAPSPCI_DMA_BASE: u32 = 0x0184_0000;
pub const GAPSPCI_DMA_SIZE: u32 = 32768;
pub const GAPSPCI_BBA_CONFIG: u32 = 0x0100_1600;
pub const GAPSPCI_BBA_CONFIG_SIZE: u32 = 0x2000;

pub const GAPSPCI_IRQ: u32 = HW_EVENT_EXTERNAL;

extern "C" {
    pub static mut gapspci_pci_ops: pci_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
