/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2007 IBM Corp
 */

// Translated from <asm/tsi108.h>; TSI108_PCI_OFFSET is supplied externally.

/* Register definitions */
pub const TSI108_PCI_P2O_BAR0: u32 = TSI108_PCI_OFFSET + 0x10;
pub const TSI108_PCI_P2O_BAR0_UPPER: u32 = TSI108_PCI_OFFSET + 0x14;
pub const TSI108_PCI_P2O_BAR2: u32 = TSI108_PCI_OFFSET + 0x18;
pub const TSI108_PCI_P2O_BAR2_UPPER: u32 = TSI108_PCI_OFFSET + 0x1c;
pub const TSI108_PCI_P2O_PAGE_SIZES: u32 = TSI108_PCI_OFFSET + 0x4c;
pub const TSI108_PCI_PFAB_BAR0: u32 = TSI108_PCI_OFFSET + 0x204;
pub const TSI108_PCI_PFAB_BAR0_UPPER: u32 = TSI108_PCI_OFFSET + 0x208;
pub const TSI108_PCI_PFAB_IO: u32 = TSI108_PCI_OFFSET + 0x20c;
pub const TSI108_PCI_PFAB_IO_UPPER: u32 = TSI108_PCI_OFFSET + 0x210;
pub const TSI108_PCI_PFAB_MEM32: u32 = TSI108_PCI_OFFSET + 0x214;
pub const TSI108_PCI_PFAB_PFM3: u32 = TSI108_PCI_OFFSET + 0x220;
pub const TSI108_PCI_PFAB_PFM4: u32 = TSI108_PCI_OFFSET + 0x230;

// Opaque types declared by external dependencies.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

extern "C" {
    pub fn tsi108_setup_pci(dev: *mut device_node, cfg_phys: u32, primary: i32) -> i32;
    pub fn tsi108_pci_int_init(node: *mut device_node);
    pub fn tsi108_irq_cascade(desc: *mut irq_desc);
    pub fn tsi108_clear_pci_cfg_error();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
