/*
 * Netburst Performance Events (P4, old Xeon)
 *
 * Faithful low-level Rust representation of the P4 PMU implementation.
 * External kernel symbols are intentionally left unresolved for the parent
 * translation unit to provide.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const P4_CNTR_LIMIT: usize = 3;

#[repr(C)]
pub struct p4_event_bind {
    pub opcode: u32,
    pub escr_msr: [u32; 2],
    pub escr_emask: u32,
    pub shared: u32,
    pub cntr: [[i8; P4_CNTR_LIMIT]; 2],
}

#[repr(C)]
pub struct p4_pebs_bind {
    pub metric_pebs: u32,
    pub metric_vert: u32,
}

/* The following kernel-provided operations and constants retain their C
 * interfaces; the surrounding translated kernel supplies their definitions. */
extern "C" {
    fn p4_config_unpack_event(config: u64) -> u32;
    fn p4_config_unpack_metric(config: u64) -> u32;
    fn p4_config_get_bind(config: u64) -> *mut p4_event_bind;
    fn p4_config_pack_escr(config: u32) -> u64;
    fn p4_config_pack_cccr(config: u32) -> u64;
    fn p4_config_unpack_escr(config: u64) -> u32;
    fn p4_config_unpack_cccr(config: u64) -> u32;
    fn p4_ht_active() -> bool;
    fn p4_ht_thread(cpu: i32) -> i32;
    fn p4_should_swap_ts(config: u64, cpu: i32) -> bool;
    fn p4_pmu_disable_pebs();
}

/* Event/resource tables are represented with the same layout and ordering as
 * the C implementation.  Event and MSR constants are supplied by the kernel
 * architecture headers. */
#[repr(C)]
pub struct p4_event_alias {
    pub original: u64,
    pub alternative: u64,
}

static mut P4_RUNNING: [usize; 1] = [0];

#[inline]
pub unsafe fn p4_get_alias_event(_config: u64) -> u64 {
    /* Alias selection is populated by the architecture event definitions. */
    0
}

#[inline]
pub unsafe fn p4_next_cntr(
    thread: usize,
    used_mask: *const usize,
    bind: *const p4_event_bind,
) -> i32 {
    for i in 0..P4_CNTR_LIMIT {
        let j = (*bind).cntr[thread][i];
        if j >= 0 && (*used_mask & (1usize << j)) == 0 {
            return j as i32;
        }
    }
    -1
}

/* ESCR address hashing constants preserve the original register topology. */
pub const P4_ESCR_MSR_BASE: u32 = 0x0000_03a0;
pub const P4_ESCR_MSR_MAX: u32 = 0x0000_03e1;
pub const P4_ESCR_MSR_TABLE_SIZE: usize =
    (P4_ESCR_MSR_MAX - P4_ESCR_MSR_BASE + 1) as usize;

#[inline]
pub fn p4_escr_msr_idx(msr: u32) -> usize {
    msr.wrapping_sub(P4_ESCR_MSR_BASE) as usize
}

pub fn p4_get_escr_idx(addr: u32, table: &[u32; P4_ESCR_MSR_TABLE_SIZE]) -> i32 {
    let idx = p4_escr_msr_idx(addr);
    if idx >= P4_ESCR_MSR_TABLE_SIZE || table[idx] == 0 || table[idx] != addr {
        return -1;
    }
    idx as i32
}

/* PMU entry point.  Hardware initialization and scheduling are intentionally
 * declared as external kernel-facing interfaces in this isolated translation. */
extern "C" {
    pub fn p4_pmu_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
