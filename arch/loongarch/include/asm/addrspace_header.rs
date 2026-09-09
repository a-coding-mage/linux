/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1996, 99 Ralf Baechle
 * Copyright (C) 2000, 2002  Maciej W. Rozycki
 * Copyright (C) 1990, 1999 by Silicon Graphics, Inc.
 */

// C headers and include guards removed; their symbols are supplied by other
// translated headers. CONFIG_32BIT/CONFIG_64BIT and __ASSEMBLER__ are build
// conditions from the original header.

/* This gives the physical RAM offset. */
#[cfg(not(feature = "__ASSEMBLER__"))]
pub const PHYS_OFFSET: usize = 0;

#[cfg(not(feature = "__ASSEMBLER__"))]
extern "C" {
    pub static mut vm_map_base: usize;
}

#[cfg(not(feature = "CONFIG_32BIT"))]
pub const IO_BASE: u64 = CSR_DMW0_BASE;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const CACHE_BASE: u64 = CSR_DMW1_BASE;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const UNCACHE_BASE: u64 = CSR_DMW0_BASE;

#[cfg(feature = "CONFIG_32BIT")]
pub const WRITECOMBINE_BASE: u64 = CSR_DMW0_BASE;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const WRITECOMBINE_BASE: u64 = CSR_DMW2_BASE;

#[cfg(feature = "CONFIG_32BIT")]
pub const DMW_PABITS: u32 = 29;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const DMW_PABITS: u32 = 48;

#[cfg(feature = "CONFIG_32BIT")]
pub const TO_PHYS_MASK: u64 = (1u64 << DMW_PABITS) - 1;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const TO_PHYS_MASK: u64 = (1u64 << DMW_PABITS) - 1;

/* Memory above this physical address will be considered highmem. */
pub const HIGHMEM_START: u64 = 1u64 << DMW_PABITS;

pub const fn TO_PHYS(x: u64) -> u64 { x & TO_PHYS_MASK }
pub const fn TO_CACHE(x: u64) -> u64 { CACHE_BASE | (x & TO_PHYS_MASK) }
pub const fn TO_UNCACHE(x: u64) -> u64 { UNCACHE_BASE | (x & TO_PHYS_MASK) }

/* This handles the memory map. */
pub const PAGE_OFFSET: u64 = CACHE_BASE + PHYS_OFFSET as u64;
pub const FIXADDR_TOP: u64 = 0xfffe0000u32 as i32 as i64 as u64;

#[cfg(feature = "CONFIG_64BIT")]
pub const fn _CONST64_(x: u64) -> u64 { x }
#[cfg(not(feature = "CONFIG_64BIT"))]
pub const fn _CONST64_(x: u64) -> u64 { x }

/* 32/64-bit LoongArch address spaces. */
#[cfg(feature = "CONFIG_32BIT")]
pub const UVRANGE: u32 = 0x00000000;
#[cfg(feature = "CONFIG_32BIT")]
pub const KPRANGE0: u32 = 0x80000000;
#[cfg(feature = "CONFIG_32BIT")]
pub const KPRANGE1: u32 = 0xa0000000;
#[cfg(feature = "CONFIG_32BIT")]
pub const KVRANGE: u32 = 0xc0000000;

#[cfg(not(feature = "CONFIG_32BIT"))]
pub const XUVRANGE: u64 = 0x0000000000000000;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const XSPRANGE: u64 = 0x4000000000000000;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const XKPRANGE: u64 = 0x8000000000000000;
#[cfg(not(feature = "CONFIG_32BIT"))]
pub const XKVRANGE: u64 = 0xc000000000000000;

/* Returns the physical address of a KPRANGEx / XKPRANGE address. */
pub const fn PHYSADDR(a: u64) -> u64 { a & TO_PHYS_MASK }

/* I/O ports map on LoongArch as described by the original header. */
pub const PCI_IOBASE: *mut core::ffi::c_void =
    (vm_map_base as *const usize as usize + (2 * PAGE_SIZE as usize)) as *mut core::ffi::c_void;
pub const PCI_IOSIZE: usize = SZ_32M;
pub const ISA_IOSIZE: usize = SZ_16K;
pub const IO_SPACE_LIMIT: usize = PCI_IOSIZE - 1;

pub const PHYS_LINK_KADDR: u64 = PHYSADDR(VMLINUX_LOAD_ADDRESS);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
