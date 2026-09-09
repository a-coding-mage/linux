/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2017 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/* Interface to the Level 2 Cache (L2C) control, measurement, and debugging facilities. */

use core::ffi::c_int;

pub const CVMX_L2C_IDX_ADDR_SHIFT: u32 = 7;
pub const CVMX_L2C_MEMBANK_SELECT_SIZE: u32 = 4096;
pub const CVMX_L2C_TADS: u32 = 1;

/* Deprecated macros, represented as functions because their values are runtime-dependent. */
#[inline]
pub unsafe fn CVMX_L2_ASSOC() -> c_int { cvmx_l2c_get_num_assoc() }
#[inline]
pub unsafe fn CVMX_L2_SET_BITS() -> c_int { cvmx_l2c_get_set_bits() }
#[inline]
pub unsafe fn CVMX_L2_SETS() -> c_int { cvmx_l2c_get_num_sets() }
#[inline]
pub unsafe fn CVMX_L2C_IDX_MASK() -> c_int { cvmx_l2c_get_num_sets() - 1 }
#[inline]
pub unsafe fn CVMX_L2C_TAG_ADDR_ALIAS_SHIFT() -> c_int {
    CVMX_L2C_IDX_ADDR_SHIFT as c_int + cvmx_l2c_get_set_bits()
}
#[inline]
pub unsafe fn CVMX_L2C_ALIAS_MASK() -> c_int {
    CVMX_L2C_IDX_MASK() << CVMX_L2C_TAG_ADDR_ALIAS_SHIFT()
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_l2c_tag_s {
    /* C bitfields: reserved:28, V:1, D:1, L:1, U:1, addr:32. */
    pub bits: u64,
}

#[repr(C)]
pub union cvmx_l2c_tag {
    pub u64_: u64,
    pub s: cvmx_l2c_tag_s,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_l2c_event {
    CVMX_L2C_EVENT_CYCLES = 0,
    CVMX_L2C_EVENT_INSTRUCTION_MISS = 1,
    CVMX_L2C_EVENT_INSTRUCTION_HIT = 2,
    CVMX_L2C_EVENT_DATA_MISS = 3,
    CVMX_L2C_EVENT_DATA_HIT = 4,
    CVMX_L2C_EVENT_MISS = 5,
    CVMX_L2C_EVENT_HIT = 6,
    CVMX_L2C_EVENT_VICTIM_HIT = 7,
    CVMX_L2C_EVENT_INDEX_CONFLICT = 8,
    CVMX_L2C_EVENT_TAG_PROBE = 9,
    CVMX_L2C_EVENT_TAG_UPDATE = 10,
    CVMX_L2C_EVENT_TAG_COMPLETE = 11,
    CVMX_L2C_EVENT_TAG_DIRTY = 12,
    CVMX_L2C_EVENT_DATA_STORE_NOP = 13,
    CVMX_L2C_EVENT_DATA_STORE_READ = 14,
    CVMX_L2C_EVENT_DATA_STORE_WRITE = 15,
    CVMX_L2C_EVENT_FILL_DATA_VALID = 16,
    CVMX_L2C_EVENT_WRITE_REQUEST = 17,
    CVMX_L2C_EVENT_READ_REQUEST = 18,
    CVMX_L2C_EVENT_WRITE_DATA_VALID = 19,
    CVMX_L2C_EVENT_XMC_NOP = 20,
    CVMX_L2C_EVENT_XMC_LDT = 21,
    CVMX_L2C_EVENT_XMC_LDI = 22,
    CVMX_L2C_EVENT_XMC_LDD = 23,
    CVMX_L2C_EVENT_XMC_STF = 24,
    CVMX_L2C_EVENT_XMC_STT = 25,
    CVMX_L2C_EVENT_XMC_STP = 26,
    CVMX_L2C_EVENT_XMC_STC = 27,
    CVMX_L2C_EVENT_XMC_DWB = 28,
    CVMX_L2C_EVENT_XMC_PL2 = 29,
    CVMX_L2C_EVENT_XMC_PSL1 = 30,
    CVMX_L2C_EVENT_XMC_IOBLD = 31,
    CVMX_L2C_EVENT_XMC_IOBST = 32,
    CVMX_L2C_EVENT_XMC_IOBDMA = 33,
    CVMX_L2C_EVENT_XMC_IOBRSP = 34,
    CVMX_L2C_EVENT_XMC_BUS_VALID = 35,
    CVMX_L2C_EVENT_XMC_MEM_DATA = 36,
    CVMX_L2C_EVENT_XMC_REFL_DATA = 37,
    CVMX_L2C_EVENT_XMC_IOBRSP_DATA = 38,
    CVMX_L2C_EVENT_RSC_NOP = 39,
    CVMX_L2C_EVENT_RSC_STDN = 40,
    CVMX_L2C_EVENT_RSC_FILL = 41,
    CVMX_L2C_EVENT_RSC_REFL = 42,
    CVMX_L2C_EVENT_RSC_STIN = 43,
    CVMX_L2C_EVENT_RSC_SCIN = 44,
    CVMX_L2C_EVENT_RSC_SCFL = 45,
    CVMX_L2C_EVENT_RSC_SCDN = 46,
    CVMX_L2C_EVENT_RSC_DATA_VALID = 47,
    CVMX_L2C_EVENT_RSC_VALID_FILL = 48,
    CVMX_L2C_EVENT_RSC_VALID_STRSP = 49,
    CVMX_L2C_EVENT_RSC_VALID_REFL = 50,
    CVMX_L2C_EVENT_LRF_REQ = 51,
    CVMX_L2C_EVENT_DT_RD_ALLOC = 52,
    CVMX_L2C_EVENT_DT_WR_INVAL = 53,
    CVMX_L2C_EVENT_MAX = 54,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_l2c_tad_event {
    CVMX_L2C_TAD_EVENT_NONE = 0, CVMX_L2C_TAD_EVENT_TAG_HIT = 1,
    CVMX_L2C_TAD_EVENT_TAG_MISS = 2, CVMX_L2C_TAD_EVENT_TAG_NOALLOC = 3,
    CVMX_L2C_TAD_EVENT_TAG_VICTIM = 4, CVMX_L2C_TAD_EVENT_SC_FAIL = 5,
    CVMX_L2C_TAD_EVENT_SC_PASS = 6, CVMX_L2C_TAD_EVENT_LFB_VALID = 7,
    CVMX_L2C_TAD_EVENT_LFB_WAIT_LFB = 8, CVMX_L2C_TAD_EVENT_LFB_WAIT_VAB = 9,
    CVMX_L2C_TAD_EVENT_QUAD0_INDEX = 128, CVMX_L2C_TAD_EVENT_QUAD0_READ = 129,
    CVMX_L2C_TAD_EVENT_QUAD0_BANK = 130, CVMX_L2C_TAD_EVENT_QUAD0_WDAT = 131,
    CVMX_L2C_TAD_EVENT_QUAD1_INDEX = 144, CVMX_L2C_TAD_EVENT_QUAD1_READ = 145,
    CVMX_L2C_TAD_EVENT_QUAD1_BANK = 146, CVMX_L2C_TAD_EVENT_QUAD1_WDAT = 147,
    CVMX_L2C_TAD_EVENT_QUAD2_INDEX = 160, CVMX_L2C_TAD_EVENT_QUAD2_READ = 161,
    CVMX_L2C_TAD_EVENT_QUAD2_BANK = 162, CVMX_L2C_TAD_EVENT_QUAD2_WDAT = 163,
    CVMX_L2C_TAD_EVENT_QUAD3_INDEX = 176, CVMX_L2C_TAD_EVENT_QUAD3_READ = 177,
    CVMX_L2C_TAD_EVENT_QUAD3_BANK = 178, CVMX_L2C_TAD_EVENT_QUAD3_WDAT = 179,
    CVMX_L2C_TAD_EVENT_MAX = 180,
}

extern "C" {
    pub fn cvmx_l2c_config_perf(counter: u32, event: cvmx_l2c_event, clear_on_read: u32);
    pub fn cvmx_l2c_read_perf(counter: u32) -> u64;
    pub fn cvmx_l2c_get_core_way_partition(core: u32) -> c_int;
    pub fn cvmx_l2c_set_core_way_partition(core: u32, mask: u32) -> c_int;
    pub fn cvmx_l2c_get_hw_way_partition() -> c_int;
    pub fn cvmx_l2c_set_hw_way_partition(mask: u32) -> c_int;
    pub fn cvmx_l2c_lock_line(addr: u64) -> c_int;
    pub fn cvmx_l2c_lock_mem_region(start: u64, len: u64) -> c_int;
    pub fn cvmx_l2c_unlock_line(address: u64) -> c_int;
    pub fn cvmx_l2c_unlock_mem_region(start: u64, len: u64) -> c_int;
    pub fn cvmx_l2c_get_tag(association: u32, index: u32) -> cvmx_l2c_tag;
    pub fn cvmx_l2c_address_to_index(addr: u64) -> u32;
    pub fn cvmx_l2c_flush();
    pub fn cvmx_l2c_get_cache_size_bytes() -> c_int;
    pub fn cvmx_l2c_get_num_sets() -> c_int;
    pub fn cvmx_l2c_get_set_bits() -> c_int;
    pub fn cvmx_l2c_get_num_assoc() -> c_int;
    pub fn cvmx_l2c_flush_line(assoc: u32, index: u32);
}

/* Wrapper providing a deprecated old function name. */
#[deprecated]
#[inline]
pub unsafe fn cvmx_get_l2c_tag(association: u32, index: u32) -> cvmx_l2c_tag {
    cvmx_l2c_get_tag(association, index)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
