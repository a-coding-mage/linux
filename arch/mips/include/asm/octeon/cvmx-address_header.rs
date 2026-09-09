/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2009 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 2, as published by
 * the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT. See the GNU General Public License for more details.
 ***********************license end**************************************/

//! Typedefs and defines for working with Octeon physical addresses.

#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_mips_xkseg_space_t {
    CVMX_MIPS_XKSEG_SPACE_KSEG0 = 0,
    CVMX_MIPS_XKSEG_SPACE_KSEG1 = 1,
    CVMX_MIPS_XKSEG_SPACE_SSEG = 2,
    CVMX_MIPS_XKSEG_SPACE_KSEG3 = 3,
}

/* decodes <14:13> of a kseg3 window address */
#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_add_win_dec_t {
    CVMX_ADD_WIN_SCR = 0,
    /* see cvmx_add_win_dma_dec_t for further decode */
    CVMX_ADD_WIN_DMA = 1,
    CVMX_ADD_WIN_UNUSED = 2,
    CVMX_ADD_WIN_UNUSED2 = 3,
}

/* decode within DMA space */
#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_add_win_dma_dec_t {
    /* Add store data to the write buffer entry, allocating it if necessary. */
    CVMX_ADD_WIN_DMA_ADD = 0,
    /* send out the write buffer entry to DRAM */
    CVMX_ADD_WIN_DMA_SENDMEM = 1,
    /* send out the write buffer entry as an IOBDMA command */
    CVMX_ADD_WIN_DMA_SENDDMA = 2,
    /* send out the write buffer entry as an IO write */
    CVMX_ADD_WIN_DMA_SENDIO = 3,
    /* send out a single-tick command on the NCB bus */
    CVMX_ADD_WIN_DMA_SENDSINGLE = 4,
}

/*
 * Physical Address Decode. The hardware interprets the address as a
 * 64-bit value; the C source uses endian-dependent bit-fields. Rust has
 * no native bit-field syntax, so each view retains the exact 64-bit storage
 * and documents the corresponding fields through accessors below.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_addr_bits_t {
    pub value: u64,
}

#[repr(C)]
pub union cvmx_addr_t {
    pub u64: u64,
    pub sva: cvmx_addr_bits_t,
    pub suseg: cvmx_addr_bits_t,
    pub sxkseg: cvmx_addr_bits_t,
    pub sxkphys: cvmx_addr_bits_t,
    pub sphys: cvmx_addr_bits_t,
    pub smem: cvmx_addr_bits_t,
    pub sio: cvmx_addr_bits_t,
    pub sscr: cvmx_addr_bits_t,
    pub sdma: cvmx_addr_bits_t,
    pub sfilldidspace: cvmx_addr_bits_t,
}

/* These macros are used by 32 bit applications. */
pub const CVMX_MIPS32_SPACE_KSEG0: i32 = 1;

#[inline]
pub const fn CVMX_ADD_SEG32(segment: i32, add: i32) -> i32 {
    ((segment << 31) | add)
}

/* Currently all IOs are performed using XKPHYS addressing. */
pub const CVMX_MIPS_SPACE_XKPHYS: u64 = 2;
pub const CVMX_IO_SEG: u64 = CVMX_MIPS_SPACE_XKPHYS;

#[inline]
pub const fn CVMX_ADD_SEG(segment: u64, add: u64) -> u64 {
    (segment << 62) | add
}

#[inline]
pub const fn CVMX_ADD_IO_SEG(add: u64) -> u64 {
    CVMX_ADD_SEG(CVMX_IO_SEG, add)
}

#[inline]
pub const fn CVMX_ADDR_DIDSPACE(did: u64) -> u64 {
    (CVMX_IO_SEG << 22) | (1u64 << 8) | did
}

#[inline]
pub const fn CVMX_ADDR_DID(did: u64) -> u64 {
    CVMX_ADDR_DIDSPACE(did) << 40
}

#[inline]
pub const fn CVMX_FULL_DID(did: u64, subdid: u64) -> u64 {
    (did << 3) | subdid
}

/* from include/ncb_rsl_id.v */
pub const CVMX_OCT_DID_MIS: u64 = 0;
pub const CVMX_OCT_DID_GMX0: u64 = 1;
pub const CVMX_OCT_DID_GMX1: u64 = 2;
pub const CVMX_OCT_DID_PCI: u64 = 3;
pub const CVMX_OCT_DID_KEY: u64 = 4;
pub const CVMX_OCT_DID_FPA: u64 = 5;
pub const CVMX_OCT_DID_DFA: u64 = 6;
pub const CVMX_OCT_DID_ZIP: u64 = 7;
pub const CVMX_OCT_DID_RNG: u64 = 8;
pub const CVMX_OCT_DID_IPD: u64 = 9;
pub const CVMX_OCT_DID_PKT: u64 = 10;
pub const CVMX_OCT_DID_TIM: u64 = 11;
pub const CVMX_OCT_DID_TAG: u64 = 12;
pub const CVMX_OCT_DID_L2C: u64 = 16;
pub const CVMX_OCT_DID_LMC: u64 = 17;
pub const CVMX_OCT_DID_SPX0: u64 = 18;
pub const CVMX_OCT_DID_SPX1: u64 = 19;
pub const CVMX_OCT_DID_PIP: u64 = 20;
pub const CVMX_OCT_DID_ASX0: u64 = 22;
pub const CVMX_OCT_DID_ASX1: u64 = 23;
pub const CVMX_OCT_DID_IOB: u64 = 30;

pub const CVMX_OCT_DID_PKT_SEND: u64 = CVMX_FULL_DID(CVMX_OCT_DID_PKT, 2);
pub const CVMX_OCT_DID_TAG_SWTAG: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 0);
pub const CVMX_OCT_DID_TAG_TAG1: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 1);
pub const CVMX_OCT_DID_TAG_TAG2: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 2);
pub const CVMX_OCT_DID_TAG_TAG3: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 3);
pub const CVMX_OCT_DID_TAG_NULL_RD: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 4);
pub const CVMX_OCT_DID_TAG_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TAG, 7);
pub const CVMX_OCT_DID_FAU_FAI: u64 = CVMX_FULL_DID(CVMX_OCT_DID_IOB, 0);
pub const CVMX_OCT_DID_TIM_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_TIM, 0);
pub const CVMX_OCT_DID_KEY_RW: u64 = CVMX_FULL_DID(CVMX_OCT_DID_KEY, 0);
pub const CVMX_OCT_DID_PCI_6: u64 = CVMX_FULL_DID(CVMX_OCT_DID_PCI, 6);
pub const CVMX_OCT_DID_MIS_BOO: u64 = CVMX_FULL_DID(CVMX_OCT_DID_MIS, 0);
pub const CVMX_OCT_DID_PCI_RML: u64 = CVMX_FULL_DID(CVMX_OCT_DID_PCI, 0);
pub const CVMX_OCT_DID_IPD_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_IPD, 7);
pub const CVMX_OCT_DID_DFA_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_DFA, 7);
pub const CVMX_OCT_DID_MIS_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_MIS, 7);
pub const CVMX_OCT_DID_ZIP_CSR: u64 = CVMX_FULL_DID(CVMX_OCT_DID_ZIP, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
