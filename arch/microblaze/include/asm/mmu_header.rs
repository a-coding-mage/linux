/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* Declarations from the __KERNEL__ / non-assembler configuration. */

/* Default "unsigned long" context */
pub type MmContextT = ::core::ffi::c_ulong;

/* Hardware Page Table Entry. The bit-field storage is represented explicitly
 * as the native 32-bit hardware word; field masks follow the C declaration. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PTE {
    pub bits: u32,
}

impl Pte {
    pub const V_MASK: u32 = 0x0000_0001;
    pub const VSID_MASK: u32 = 0x00ff_fffe;
    pub const H_MASK: u32 = 0x0100_0000;
    pub const API_MASK: u32 = 0x7e00_0000;
}

/* Values for PP (assumes Ks=0, Kp=1) */
pub const PP_RWXX: u32 = 0;
pub const PP_RWRX: u32 = 1;
pub const PP_RWRW: u32 = 2;
pub const PP_RXRX: u32 = 3;

/* Segment Register */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SEGREG {
    pub bits: u32,
}

extern "C" {
    pub fn _tlbie(va: ::core::ffi::c_ulong); /* invalidate a TLB entry */
    pub fn _tlbia(); /* invalidate all TLB entries */
    pub static mut tlb_skip: u32;
}

/*
 * The MicroBlaze processor has a TLB architecture identical to PPC-40x. The
 * instruction and data sides share a unified, 64-entry, semi-associative
 * TLB which is maintained totally under software control. In addition, the
 * instruction side has a hardware-managed, 2,4, or 8-entry, fully-associative
 * TLB which serves as a first level to the shared TLB. These two TLBs are
 * known as the UTLB and ITLB, respectively.
 */
pub const MICROBLAZE_TLB_SIZE: u32 = 64;

/* For cases when you want to skip some TLB entries */
pub const MICROBLAZE_TLB_SKIP: u32 = 0;

/* Use the last TLB for temporary access to LMB */
pub const MICROBLAZE_LMB_TLB_ID: u32 = 63;

/*
 * TLB entries are defined by a "high" tag portion and a "low" data
 * portion. The data portion is 32-bits.
 *
 * TLB entries are managed entirely under software control by reading,
 * writing, and searching using the MTS and MFS instructions.
 */
pub const TLB_LO: u32 = 1;
pub const TLB_HI: u32 = 0;
pub const TLB_DATA: u32 = TLB_LO;
pub const TLB_TAG: u32 = TLB_HI;

/* Tag portion */
pub const TLB_EPN_MASK: u32 = 0xffff_fc00; /* Effective Page Number */
pub const TLB_PAGESZ_MASK: u32 = 0x0000_0380;
#[inline]
pub const fn TLB_PAGESZ(x: u32) -> u32 { (x & 0x7).wrapping_shl(7) }
pub const PAGESZ_1K: u32 = 0;
pub const PAGESZ_4K: u32 = 1;
pub const PAGESZ_16K: u32 = 2;
pub const PAGESZ_64K: u32 = 3;
pub const PAGESZ_256K: u32 = 4;
pub const PAGESZ_1M: u32 = 5;
pub const PAGESZ_4M: u32 = 6;
pub const PAGESZ_16M: u32 = 7;
pub const TLB_VALID: u32 = 0x0000_0040; /* Entry is valid */

/* Data portion */
pub const TLB_RPN_MASK: u32 = 0xffff_fc00; /* Real Page Number */
pub const TLB_PERM_MASK: u32 = 0x0000_0300;
pub const TLB_EX: u32 = 0x0000_0200; /* Instruction execution allowed */
pub const TLB_WR: u32 = 0x0000_0100; /* Writes permitted */
pub const TLB_ZSEL_MASK: u32 = 0x0000_00f0;
#[inline]
pub const fn TLB_ZSEL(x: u32) -> u32 { (x & 0xf).wrapping_shl(4) }
pub const TLB_ATTR_MASK: u32 = 0x0000_000f;
pub const TLB_W: u32 = 0x0000_0008; /* Caching is write-through */
pub const TLB_I: u32 = 0x0000_0004; /* Caching is inhibited */
pub const TLB_M: u32 = 0x0000_0002; /* Memory is coherent */
pub const TLB_G: u32 = 0x0000_0001; /* Memory is guarded from prefetch */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
