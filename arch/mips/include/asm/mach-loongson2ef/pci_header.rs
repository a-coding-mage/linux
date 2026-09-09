/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) 2008 Zhang Le <r0bertz@gentoo.org> */
/* Copyright (c) 2009 Wu Zhangjin <wuzhangjin@gmail.com> */

// C header guard: __ASM_MACH_LOONGSON2EF_PCI_H_
extern "C" { pub static mut loongson_pci_ops: pci_ops; }

/* this is an offset from mips_io_port_base */
pub const LOONGSON_PCI_IO_START: u64 = 0x00004000u64;

// CONFIG_CPU_SUPPORTS_ADDRWINCFG
#[cfg(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG")]
pub const LOONGSON_CPU_MEM_SRC: u64 = 0x40000000u64; /* 1G */
#[cfg(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG")]
pub const LOONGSON_PCI_MEM_DST: u64 = LOONGSON_CPU_MEM_SRC;
#[cfg(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG")]
pub const LOONGSON_PCI_MEM_START: u64 = LOONGSON_PCI_MEM_DST;
#[cfg(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG")]
pub const LOONGSON_PCI_MEM_END: u64 = 0x80000000u64 - 1; /* 2G */
#[cfg(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG")]
pub const MMAP_CPUTOPCI_SIZE: u64 = LOONGSON_PCI_MEM_END - LOONGSON_PCI_MEM_START + 1;

/*
 * we use address window2 to map cpu address space to pci space
 * window2: cpu [1G, 2G] -> pci [1G, 2G]
 * why not use window 0 & 1? because they are used by cpu when booting.
 * window0: cpu [0, 256M] -> ddr [0, 256M]
 * window1: cpu [256M, 512M] -> pci [256M, 512M]
 */
/* the smallest LOONGSON_CPU_MEM_SRC can be 512M */

// !CONFIG_CPU_SUPPORTS_ADDRWINCFG (loongson2f/32bit & loongson2e)
#[cfg(not(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG"))]
pub const LOONGSON_PCI_MEM_START: u64 = LOONGSON_PCILO1_BASE;
#[cfg(not(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG"))]
pub const LOONGSON_PCI_MEM_END: u64 = LOONGSON_PCILO1_BASE + 0x04000000u64 * 2;
/* this pci memory space is mapped by pcimap in pci.c */
/* this is an offset from mips_io_port_base */
#[cfg(not(feature = "CONFIG_CPU_SUPPORTS_ADDRWINCFG"))]
pub const LOONGSON_PCI_IO_START: u64 = 0x00004000u64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
