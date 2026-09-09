/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 99 Ralf Baechle
 * Copyright (C) 2000, 2002  Maciej W. Rozycki
 * Copyright (C) 1990, 1999 by Silicon Graphics, Inc.
 */

// IP27 uses the R10000's uncached attribute feature. Attribute 3 selects
// uncached memory addressing. These definitions are hidden for 32-bit
// compilation of the compat-vdso code.
#[cfg(CONFIG_64BIT)]
pub const HSPEC_BASE: u64 = 0x9000_0000_0000_0000;
#[cfg(CONFIG_64BIT)]
pub const IO_BASE: u64 = 0x9200_0000_0000_0000;
#[cfg(CONFIG_64BIT)]
pub const MSPEC_BASE: u64 = 0x9400_0000_0000_0000;
#[cfg(CONFIG_64BIT)]
pub const UNCAC_BASE: u64 = 0x9600_0000_0000_0000;
#[cfg(CONFIG_64BIT)]
pub const CAC_BASE: u64 = 0xa800_0000_0000_0000;

macro_rules! TO_MSPEC {
    ($x:expr) => {
        MSPEC_BASE | (($x) & TO_PHYS_MASK)
    };
}

macro_rules! TO_HSPEC {
    ($x:expr) => {
        HSPEC_BASE | (($x) & TO_PHYS_MASK)
    };
}

pub const HIGHMEM_START: usize = !0usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
