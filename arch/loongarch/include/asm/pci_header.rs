/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header guard: _ASM_PCI_H

// Dependencies supplied by the surrounding kernel translation:
// linux/ioport.h, linux/list.h, linux/types.h, and asm/io.h

pub const PCIBIOS_MIN_IO: u32 = 0x4000;
pub const PCIBIOS_MIN_MEM: u32 = 0x20000000;
pub const PCIBIOS_MIN_CARDBUS_IO: u32 = 0x4000;

// C feature marker: HAVE_PCI_MMAP
pub const HAVE_PCI_MMAP: bool = true;

pub const PCIBIOS_ASSIGN_ALL_BUSSES: i32 = 0;

extern "C" {
    pub fn mcfg_addr_init(node: core::ffi::c_int) -> phys_addr_t;
}

// Generic PCI declarations supplied by asm-generic/pci.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
