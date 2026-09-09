/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2008 Zhang Le <r0bertz@gentoo.org>
 * Copyright (c) 2009 Wu Zhangjin <wuzhangjin@gmail.com>
 */

// The C header guard is omitted; Rust modules provide equivalent protection.

extern "C" {
    pub static mut loongson_pci_ops: pci_ops;
}

pub const LOONGSON_PCI_IO_START: usize = 0x00004000;

pub const LOONGSON_PCI_MEM_START: usize = 0x40000000;
pub const LOONGSON_PCI_MEM_END: usize = 0x7effffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
