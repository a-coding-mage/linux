/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 99 Ralf Baechle
 * Copyright (C) 2000, 2002  Maciej W. Rozycki
 * Copyright (C) 1990, 1999 by Silicon Graphics, Inc.
 */

// C dependency: <spaces.h> supplies the address-space symbols used below.

/* Configure language.  The assembler-only variants have no Rust equivalent. */

/* 32-bit MIPS address spaces */

/// Returns the kernel segment base of a given address.
#[inline]
pub const fn ksegx(a: u64) -> u64 { a & 0xe0000000u64 }

/// Gives the size of each kernel segment.
pub const CSEGX_SIZE: u64 = 0x20000000;

/// Returns the physical address of a CKSEGx / XKPHYS address.
#[inline]
pub const fn cphysaddr(a: u64) -> u64 { a & 0x1fffffff }

#[inline]
pub const fn xphysaddr(a: u64) -> u64 { a & 0x0000ffffffffffff }

#[cfg(feature = "CONFIG_64BIT")]
mod config_64bit {
    /*
     * Memory segments (64bit kernel mode addresses)
     * The compatibility segments use the full 64-bit sign extended value.  Note
     * the R8000 doesn't have them so don't reference these in generic MIPS code.
     */
    pub const XKUSEG: u64 = 0x0000000000000000;
    pub const XKSSEG: u64 = 0x4000000000000000;
    pub const XKPHYS: u64 = 0x8000000000000000;
    pub const XKSEG: u64 = 0xc000000000000000;
    pub const CKSEG0: u64 = 0xffffffff80000000;
    pub const CKSEG1: u64 = 0xffffffffa0000000;
    pub const CKSSEG: u64 = 0xffffffffc0000000;
    pub const CKSEG3: u64 = 0xffffffffe0000000;

    #[inline] pub const fn ckseg0addr(a: u64) -> u64 { super::cphysaddr(a) | CKSEG0 }
    #[inline] pub const fn ckseg1addr(a: u64) -> u64 { super::cphysaddr(a) | CKSEG1 }
    // CKSEG2 is supplied by the address-space dependency, as in the C header.
    #[inline] pub const fn ckseg2addr(a: u64) -> u64 { super::cphysaddr(a) | CKSEG2 }
    #[inline] pub const fn ckseg3addr(a: u64) -> u64 { super::cphysaddr(a) | CKSEG3 }
}

#[cfg(not(feature = "CONFIG_64BIT"))]
mod config_32bit {
    /* Map an address to a certain kernel segment. */
    #[inline] pub const fn ckseg0addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG0 }
    #[inline] pub const fn ckseg1addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG1 }
    #[inline] pub const fn ckseg2addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG2 }
    #[inline] pub const fn ckseg3addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG3 }
    #[inline] pub const fn kseg0addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG0 }
    #[inline] pub const fn kseg1addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG1 }
    #[inline] pub const fn kseg2addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG2 }
    #[inline] pub const fn kseg3addr(a: u64) -> u64 { super::cphysaddr(a) | KSEG3 }

    /* Memory segments (32bit kernel mode addresses). */
    pub const KUSEG: u64 = 0x00000000;
    pub const KSEG0: u64 = 0x80000000;
    pub const KSEG1: u64 = 0xa0000000;
    pub const KSEG2: u64 = 0xc0000000;
    pub const KSEG3: u64 = 0xe0000000;
    pub const CKUSEG: u64 = 0x00000000;
    pub const CKSEG0: u64 = 0x80000000;
    pub const CKSEG1: u64 = 0xa0000000;
    pub const CKSEG2: u64 = 0xc0000000;
    pub const CKSEG3: u64 = 0xe0000000;
}

/* Cache modes for XKPHYS address conversion macros */
pub const K_CALG_COH_EXCL1_NOL2: u64 = 0;
pub const K_CALG_COH_SHRL1_NOL2: u64 = 1;
pub const K_CALG_UNCACHED: u64 = 2;
pub const K_CALG_NONCOHERENT: u64 = 3;
pub const K_CALG_COH_EXCL: u64 = 4;
pub const K_CALG_COH_SHAREABLE: u64 = 5;
pub const K_CALG_NOTUSED: u64 = 6;
pub const K_CALG_UNCACHED_ACCEL: u64 = 7;

/* 64-bit address conversions */
pub const TO_PHYS_MASK: u64 = 0x07ffffffffffffff;
pub const COMPAT_K1BASE32: u64 = 0xffffffffa0000000;

#[inline] pub const fn phys_to_xkphys(cm: u64, a: u64) -> u64 { config_64bit::XKPHYS | (cm << 59) | a }
#[inline] pub const fn phys_to_xkseg_uncached(p: u64) -> u64 { phys_to_xkphys(K_CALG_UNCACHED, p) }
#[inline] pub const fn phys_to_xkseg_cached(p: u64) -> u64 { phys_to_xkphys(K_CALG_COH_SHAREABLE, p) }
#[inline] pub const fn xkphys_to_phys(p: u64) -> u64 { p & TO_PHYS_MASK }
#[inline] pub const fn phys_to_compatk1(x: u64) -> u64 { x | COMPAT_K1BASE32 }
#[inline] pub const fn kdm_to_phys(x: u64) -> u64 { x & TO_PHYS_MASK }
// CAC_BASE is supplied by the address-space dependency, as in the C header.
#[inline] pub const fn phys_to_k0(x: u64) -> u64 { x | CAC_BASE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
