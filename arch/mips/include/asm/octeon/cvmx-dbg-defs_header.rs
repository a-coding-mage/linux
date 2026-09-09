/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2012 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

// Original header guard and C includes are intentionally omitted.

pub const CVMX_DBG_DATA: u64 = CVMX_ADD_IO_SEG(0x00011F00000001E8u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_dbg_data_s {
    pub bits: u64,
}

impl cvmx_dbg_data_s {
    pub const DATA_SHIFT: u32 = 0;
    pub const DSEL_EXT_SHIFT: u32 = 17;
    pub const C_MUL_SHIFT: u32 = 18;
    pub const RESERVED_23_63_SHIFT: u32 = 23;

    #[inline]
    pub fn data(&self) -> u64 { (self.bits >> Self::DATA_SHIFT) & 0x1f_fff }
    #[inline]
    pub fn dsel_ext(&self) -> u64 { (self.bits >> Self::DSEL_EXT_SHIFT) & 1 }
    #[inline]
    pub fn c_mul(&self) -> u64 { (self.bits >> Self::C_MUL_SHIFT) & 0x1f }
    #[inline]
    pub fn reserved_23_63(&self) -> u64 { self.bits >> Self::RESERVED_23_63_SHIFT }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_dbg_data_cn30xx {
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_dbg_data_cn38xx {
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_dbg_data_cn58xx {
    pub bits: u64,
}

#[repr(C)]
pub union cvmx_dbg_data {
    pub u64: u64,
    pub s: cvmx_dbg_data_s,
    pub cn30xx: cvmx_dbg_data_cn30xx,
    pub cn38xx: cvmx_dbg_data_cn38xx,
    pub cn58xx: cvmx_dbg_data_cn58xx,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
