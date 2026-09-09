/* SPDX-License-Identifier: GPL-2.0 */

// This file contains both macros and data structures defined by the TDX
// architecture and Linux defined software data structures and functions.
// The two should not be mixed together for better readability.  The
// architectural definitions come first.

// TDX module SEAMCALL leaf functions
pub const TDH_VP_ENTER: u32 = 0;
pub const TDH_MNG_ADDCX: u32 = 1;
pub const TDH_MEM_PAGE_ADD: u32 = 2;
pub const TDH_MEM_SEPT_ADD: u32 = 3;
pub const TDH_VP_ADDCX: u32 = 4;
pub const TDH_MEM_PAGE_AUG: u32 = 6;
pub const TDH_MEM_RANGE_BLOCK: u32 = 7;
pub const TDH_MNG_KEY_CONFIG: u32 = 8;
pub const TDH_MNG_CREATE: u32 = 9;
pub const TDH_MNG_RD: u32 = 11;
pub const TDH_MR_EXTEND: u32 = 16;
pub const TDH_MR_FINALIZE: u32 = 17;
pub const TDH_VP_FLUSH: u32 = 18;
pub const TDH_MNG_VPFLUSHDONE: u32 = 19;
pub const TDH_VP_CREATE: u32 = 10;
pub const TDH_MNG_KEY_FREEID: u32 = 20;
pub const TDH_MNG_INIT: u32 = 21;
pub const TDH_VP_INIT: u32 = 22;
pub const TDH_PHYMEM_PAGE_RDMD: u32 = 24;
pub const TDH_VP_RD: u32 = 26;
pub const TDH_PHYMEM_PAGE_RECLAIM: u32 = 28;
pub const TDH_MEM_PAGE_REMOVE: u32 = 29;
pub const TDH_SYS_KEY_CONFIG: u32 = 31;
pub const TDH_SYS_INIT: u32 = 33;
pub const TDH_SYS_RD: u32 = 34;
pub const TDH_SYS_LP_INIT: u32 = 35;
pub const TDH_SYS_TDMR_INIT: u32 = 36;
pub const TDH_MEM_TRACK: u32 = 38;
pub const TDH_PHYMEM_CACHE_WB: u32 = 40;
pub const TDH_PHYMEM_PAGE_WBINVD: u32 = 41;
pub const TDH_VP_WR: u32 = 43;
pub const TDH_SYS_CONFIG: u32 = 45;
pub const TDH_SYS_SHUTDOWN: u32 = 52;
pub const TDH_SYS_UPDATE: u32 = 53;
pub const TDH_SYS_DISABLE: u32 = 69;

// SEAMCALL leaf:
//
// Bit 15:0\tLeaf number
// Bit 23:16\tVersion number
pub const TDX_VERSION_SHIFT: u32 = 16;

// TDX page types
pub const PT_NDA: u32 = 0x0;
pub const PT_RSVD: u32 = 0x1;

#[repr(C, packed)]
pub struct tdmr_reserved_area {
    pub offset: u64,
    pub size: u64,
}

pub const TDMR_INFO_ALIGNMENT: usize = 512;
pub const TDMR_INFO_PA_ARRAY_ALIGNMENT: usize = 512;

#[repr(C, packed, align(512))]
pub struct tdmr_info {
    pub base: u64,
    pub size: u64,
    pub pamt_1g_base: u64,
    pub pamt_1g_size: u64,
    pub pamt_2m_base: u64,
    pub pamt_2m_size: u64,
    pub pamt_4k_base: u64,
    pub pamt_4k_size: u64,
    // The actual number of reserved areas depends on the value of
    // field MD_FIELD_ID_MAX_RESERVED_PER_TDMR in the TDX module
    // global metadata.
    pub reserved_areas: [tdmr_reserved_area; 0],
}

// Do not put any hardware-defined TDX structure representations below
// this comment!
#[repr(C)]
pub struct tdx_memblock {
    pub list: list_head,
    pub start_pfn: ::core::ffi::c_ulong,
    pub end_pfn: ::core::ffi::c_ulong,
    pub nid: i32,
}

// Warn if kernel has less than TDMR_NR_WARN TDMRs after allocation
pub const TDMR_NR_WARN: i32 = 4;

#[repr(C)]
pub struct tdmr_info_list {
    pub tdmrs: *mut ::core::ffi::c_void, // Flexible array to hold 'tdmr_info's
    pub nr_consumed_tdmrs: i32, // How many 'tdmr_info's are in use

    // Metadata for finding target 'tdmr_info' and freeing @tdmrs
    pub tdmr_sz: i32, // Size of one 'tdmr_info'
    pub max_tdmrs: i32, // How many 'tdmr_info's are allocated
}

unsafe extern "C" {
    pub fn tdx_module_shutdown() -> i32;
    pub fn tdx_module_run_update() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
