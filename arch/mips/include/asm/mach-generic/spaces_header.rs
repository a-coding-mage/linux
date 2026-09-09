/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 1999, 2000, 03, 04 Ralf Baechle
 * Copyright (C) 2000, 2002  Maciej W. Rozycki
 * Copyright (C) 1990, 1999, 2000 Silicon Graphics, Inc.
 */

// Dependency: linux/const.h
// Dependency: asm/mipsregs.h

// Build-time override: IO_SPACE_LIMIT may be supplied externally.
pub const IO_SPACE_LIMIT: usize = 0xffff;

/*
 * This gives the physical RAM offset.
 */
#[cfg(feature = "CONFIG_MIPS_AUTO_PFN_OFFSET")]
pub const PHYS_OFFSET: usize = PFN_PHYS(ARCH_PFN_OFFSET as usize);

#[cfg(not(feature = "CONFIG_MIPS_AUTO_PFN_OFFSET"))]
pub const PHYS_OFFSET: usize = 0;

#[cfg(feature = "CONFIG_32BIT")]
pub const CAC_BASE: usize = 0x8000_0000;

#[cfg(feature = "CONFIG_32BIT")]
pub const IO_BASE: usize = 0xa000_0000;

#[cfg(feature = "CONFIG_32BIT")]
pub const UNCAC_BASE: usize = 0xa000_0000;

#[cfg(feature = "CONFIG_32BIT")]
pub const MAP_BASE: usize = 0xc000_0000;

/*
 * Memory above this physical address will be considered highmem.
 */
#[cfg(feature = "CONFIG_32BIT")]
pub const HIGHMEM_START: usize = 0x2000_0000;

#[cfg(feature = "CONFIG_32BIT")]
#[inline]
pub const fn CKSEG0ADDR_OR_64BIT(x: usize) -> usize {
    CKSEG0ADDR(x)
}

#[cfg(feature = "CONFIG_32BIT")]
#[inline]
pub const fn CKSEG1ADDR_OR_64BIT(x: usize) -> usize {
    CKSEG1ADDR(x)
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn CAC_BASE() -> usize {
    PHYS_TO_XKPHYS(read_c0_config() & CONF_CM_CMASK, 0)
}

#[cfg(feature = "CONFIG_64BIT")]
pub const IO_BASE: usize = 0x9000_0000_0000_0000;

#[cfg(feature = "CONFIG_64BIT")]
pub const UNCAC_BASE: usize = 0x9000_0000_0000_0000;

#[cfg(feature = "CONFIG_64BIT")]
pub const MAP_BASE: usize = 0xc000_0000_0000_0000;

/*
 * Memory above this physical address will be considered highmem.
 * Fixme: 59 bits is a fictive number and makes assumptions about processors
 * in the distant future.  Nobody will care for a few years :-)
 */
#[cfg(feature = "CONFIG_64BIT")]
pub const HIGHMEM_START: usize = 1usize << 59;

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub fn TO_PHYS(x: usize) -> usize {
    x & TO_PHYS_MASK
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn TO_CAC(x: usize) -> usize {
    CAC_BASE() | (x & TO_PHYS_MASK)
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn TO_UNCAC(x: usize) -> usize {
    UNCAC_BASE | (x & TO_PHYS_MASK)
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn CKSEG0ADDR_OR_64BIT(x: usize) -> usize {
    TO_CAC(x)
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn CKSEG1ADDR_OR_64BIT(x: usize) -> usize {
    TO_UNCAC(x)
}

/*
 * This handles the memory map.
 */
#[cfg(feature = "CONFIG_32BIT")]
pub const PAGE_OFFSET: usize = CAC_BASE + PHYS_OFFSET;

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn PAGE_OFFSET() -> usize {
    CAC_BASE() + PHYS_OFFSET
}

pub const FIXADDR_TOP: usize = 0xfffe_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
