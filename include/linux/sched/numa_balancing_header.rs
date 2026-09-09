/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the interface between the scheduler and the MM that
 * implements memory access pattern based NUMA-balancing:
 *
 * The original declaration depends on <linux/sched.h>.
 */

pub const TNF_MIGRATED: i32 = 0x01;
pub const TNF_NO_GROUP: i32 = 0x02;
pub const TNF_SHARED: i32 = 0x04;
pub const TNF_FAULT_LOCAL: i32 = 0x08;
pub const TNF_MIGRATE_FAIL: i32 = 0x10;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum numa_vmaskip_reason {
    NUMAB_SKIP_UNSUITABLE,
    NUMAB_SKIP_SHARED_RO,
    NUMAB_SKIP_INACCESSIBLE,
    NUMAB_SKIP_SCAN_DELAY,
    NUMAB_SKIP_PID_INACTIVE,
    NUMAB_SKIP_IGNORE_PID,
    NUMAB_SKIP_SEQ_COMPLETED,
}

/* CONFIG_NUMA_BALANCING selects the external declarations below. */
#[cfg(CONFIG_NUMA_BALANCING)]
extern "C" {
    pub fn task_numa_fault(last_node: i32, node: i32, pages: i32, flags: i32);
    pub fn task_numa_group_id(p: *mut task_struct) -> pid_t;
    pub fn set_numabalancing_state(enabled: bool);
    pub fn task_numa_free(p: *mut task_struct, final_: bool);
    pub fn should_numa_migrate_memory(
        p: *mut task_struct,
        folio: *mut folio,
        src_nid: i32,
        dst_cpu: i32,
    ) -> bool;
}

/* When CONFIG_NUMA_BALANCING is disabled, these are the C static-inline fallbacks. */
#[cfg(not(CONFIG_NUMA_BALANCING))]
#[inline]
pub unsafe fn task_numa_fault(_last_node: i32, _node: i32, _pages: i32, _flags: i32) {}

#[cfg(not(CONFIG_NUMA_BALANCING))]
#[inline]
pub unsafe fn task_numa_group_id(_p: *mut task_struct) -> pid_t {
    0
}

#[cfg(not(CONFIG_NUMA_BALANCING))]
#[inline]
pub unsafe fn set_numabalancing_state(_enabled: bool) {}

#[cfg(not(CONFIG_NUMA_BALANCING))]
#[inline]
pub unsafe fn task_numa_free(_p: *mut task_struct, _final: bool) {}

#[cfg(not(CONFIG_NUMA_BALANCING))]
#[inline]
pub unsafe fn should_numa_migrate_memory(
    _p: *mut task_struct,
    _folio: *mut folio,
    _src_nid: i32,
    _dst_cpu: i32,
) -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
