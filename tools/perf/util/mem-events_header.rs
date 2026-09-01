/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::c_char;

#[repr(C)]
pub struct perf_mem_event {
    pub supported: bool,
    pub ldlat: bool,
    pub aux_event: u32,
    pub tag: *const c_char,
    pub name: *const c_char,
    pub event_name: *const c_char,
}

pub const PERF_MEM_EVENTS__LOAD: u32 = 0;
pub const PERF_MEM_EVENTS__STORE: u32 = 1;
pub const PERF_MEM_EVENTS__LOAD_STORE: u32 = 2;
pub const PERF_MEM_EVENTS__MAX: u32 = 3;

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut perf_mem_events__loads_ldlat: u32;
    pub static mut perf_mem_events: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize];
    pub static mut perf_mem_record: [bool; PERF_MEM_EVENTS__MAX as usize];

    pub fn perf_pmu__mem_events_parse(pmu: *mut perf_pmu, str_: *const c_char) -> i32;
    pub fn perf_pmu__mem_events_init() -> i32;

    pub fn perf_pmu__mem_events_ptr(pmu: *mut perf_pmu, i: i32) -> *mut perf_mem_event;
    pub fn perf_mem_events_find_pmu() -> *mut perf_pmu;
    pub fn perf_pmu__mem_events_num_mem_pmus(pmu: *mut perf_pmu) -> i32;
    pub fn is_mem_loads_aux_event(leader: *mut evsel) -> bool;

    pub fn perf_pmu__mem_events_list(pmu: *mut perf_pmu);
    pub fn perf_mem_events__record_args(
        rec_argv: *mut *const c_char,
        argv_nr: *mut i32,
        event_name_storage_out: *mut *mut c_char,
    ) -> i32;

    pub fn perf_mem__tlb_scnprintf(
        out: *mut c_char,
        sz: usize,
        mem_info: *const mem_info,
    ) -> i32;
    pub fn perf_mem__lvl_scnprintf(
        out: *mut c_char,
        sz: usize,
        mem_info: *const mem_info,
    ) -> i32;
    pub fn perf_mem__snp_scnprintf(
        out: *mut c_char,
        sz: usize,
        mem_info: *const mem_info,
    ) -> i32;
    pub fn perf_mem__lck_scnprintf(
        out: *mut c_char,
        sz: usize,
        mem_info: *const mem_info,
    ) -> i32;
    pub fn perf_mem__blk_scnprintf(
        out: *mut c_char,
        sz: usize,
        mem_info: *const mem_info,
    ) -> i32;

    pub fn perf_script__meminfo_scnprintf(
        bf: *mut c_char,
        size: usize,
        mem_info: *const mem_info,
    ) -> i32;
}

#[repr(C)]
pub struct c2c_stats {
    pub nr_entries: u32,

    pub locks: u32,      /* count of 'lock' transactions */
    pub store: u32,      /* count of all stores in trace */
    pub st_uncache: u32, /* stores to uncacheable address */
    pub st_noadrs: u32,  /* cacheable store with no address */
    pub st_l1hit: u32,   /* count of stores that hit L1D */
    pub st_l1miss: u32,  /* count of stores that miss L1D */
    pub st_na: u32,      /* count of stores with memory level is not available */
    pub load: u32,       /* count of all loads in trace */
    pub ld_excl: u32,    /* exclusive loads, rmt/lcl DRAM - snp none/miss */
    pub ld_shared: u32,  /* shared loads, rmt/lcl DRAM - snp hit */
    pub ld_uncache: u32, /* loads to uncacheable address */
    pub ld_io: u32,      /* loads to io address */
    pub ld_miss: u32,    /* loads miss */
    pub ld_noadrs: u32,  /* cacheable load with no address */
    pub ld_fbhit: u32,   /* count of loads hitting Fill Buffer */
    pub ld_l1hit: u32,   /* count of loads that hit L1D */
    pub ld_l2hit: u32,   /* count of loads that hit L2D */
    pub ld_llchit: u32,  /* count of loads that hit LLC */
    pub lcl_hitm: u32,   /* count of loads with local HITM  */
    pub rmt_hitm: u32,   /* count of loads with remote HITM */
    pub tot_hitm: u32,   /* count of loads with local and remote HITM */
    pub lcl_peer: u32,   /* count of loads with local peer cache */
    pub rmt_peer: u32,   /* count of loads with remote peer cache */
    pub tot_peer: u32,   /* count of loads with local and remote peer cache */
    pub rmt_hit: u32,    /* count of loads with remote hit clean; */
    pub lcl_dram: u32,   /* count of loads miss to local DRAM */
    pub rmt_dram: u32,   /* count of loads miss to remote DRAM */
    pub blk_data: u32,   /* count of loads blocked by data */
    pub blk_addr: u32,   /* count of loads blocked by address conflict */
    pub nomap: u32,      /* count of load/stores with no phys addrs */
    pub noparse: u32,    /* count of unparsable data sources */
}

#[repr(C)]
pub struct hist_entry {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn c2c_decode_stats(stats: *mut c2c_stats, mi: *mut mem_info) -> i32;
    pub fn c2c_add_stats(stats: *mut c2c_stats, add: *mut c2c_stats);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_type {
    PERF_MEM_STAT_OP = 0,
    PERF_MEM_STAT_CACHE = 1,
    PERF_MEM_STAT_MEMORY = 2,
    PERF_MEM_STAT_SNOOP = 3,
    PERF_MEM_STAT_DTLB = 4,
}

pub const MEM_STAT_PRINT_LEN: usize = 7; /* 1 space + 5 digits + 1 percent sign */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_op {
    MEM_STAT_OP_LOAD = 0,
    MEM_STAT_OP_STORE = 1,
    MEM_STAT_OP_LDST = 2,
    MEM_STAT_OP_PFETCH = 3,
    MEM_STAT_OP_EXEC = 4,
    MEM_STAT_OP_OTHER = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_cache {
    MEM_STAT_CACHE_L1 = 0,
    MEM_STAT_CACHE_L2 = 1,
    MEM_STAT_CACHE_L3 = 2,
    MEM_STAT_CACHE_L4 = 3,
    MEM_STAT_CACHE_L1_BUF = 4,
    MEM_STAT_CACHE_L2_BUF = 5,
    MEM_STAT_CACHE_OTHER = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_memory {
    MEM_STAT_MEMORY_RAM = 0,
    MEM_STAT_MEMORY_MSC = 1,
    MEM_STAT_MEMORY_UNC = 2,
    MEM_STAT_MEMORY_CXL = 3,
    MEM_STAT_MEMORY_IO = 4,
    MEM_STAT_MEMORY_PMEM = 5,
    MEM_STAT_MEMORY_OTHER = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_snoop {
    MEM_STAT_SNOOP_HIT = 0,
    MEM_STAT_SNOOP_HITM = 1,
    MEM_STAT_SNOOP_MISS = 2,
    MEM_STAT_SNOOP_OTHER = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mem_stat_dtlb {
    MEM_STAT_DTLB_L1_HIT = 0,
    MEM_STAT_DTLB_L2_HIT = 1,
    MEM_STAT_DTLB_ANY_HIT = 2,
    MEM_STAT_DTLB_MISS = 3,
    MEM_STAT_DTLB_OTHER = 4,
}

unsafe extern "C" {
    pub fn mem_stat_index(mst: mem_stat_type, data_src: u64) -> i32;
    pub fn mem_stat_name(mst: mem_stat_type, idx: i32) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
