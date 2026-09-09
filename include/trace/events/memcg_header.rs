/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the Linux memcg trace event declarations.
//!
//! The original file is a tracepoint header.  Its tracepoint-registration
//! machinery is supplied by the surrounding kernel build; the declarations
//! below preserve the event payloads, arguments, assignments, and print
//! formats without providing implementations for those external facilities.

use core::ffi::c_void;

#[repr(C)]
pub struct mem_cgroup {
    pub css: cgroup_subsys_state,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

extern "C" {
    pub fn cgroup_id(cgrp: *mut cgroup) -> u64;
}

#[repr(C)]
pub struct memcg_rstat_stats_entry {
    pub id: u64,
    pub item: i32,
    pub val: libc_long,
}

#[repr(C)]
pub struct memcg_rstat_events_entry {
    pub id: u64,
    pub item: i32,
    pub val: libc_ulong,
}

#[repr(C)]
pub struct memcg_flush_stats_entry {
    pub id: u64,
    pub stats_updates: i64,
    pub force: bool,
    pub needs_flush: bool,
}

// The C types `long` and `unsigned long` retain their platform-dependent ABI.
pub type libc_long = isize;
pub type libc_ulong = usize;

#[inline]
pub unsafe fn memcg_rstat_stats_assign(
    entry: *mut memcg_rstat_stats_entry,
    memcg: *mut mem_cgroup,
    item: i32,
    val: libc_long,
) {
    (*entry).id = cgroup_id((*memcg).css.cgroup);
    (*entry).item = item;
    (*entry).val = val;
}

#[inline]
pub unsafe fn memcg_rstat_events_assign(
    entry: *mut memcg_rstat_events_entry,
    memcg: *mut mem_cgroup,
    item: i32,
    val: libc_ulong,
) {
    (*entry).id = cgroup_id((*memcg).css.cgroup);
    (*entry).item = item;
    (*entry).val = val;
}

#[inline]
pub unsafe fn memcg_flush_stats_assign(
    entry: *mut memcg_flush_stats_entry,
    memcg: *mut mem_cgroup,
    stats_updates: i64,
    force: bool,
    needs_flush: bool,
) {
    (*entry).id = cgroup_id((*memcg).css.cgroup);
    (*entry).stats_updates = stats_updates;
    (*entry).force = force;
    (*entry).needs_flush = needs_flush;
}

// Tracepoint event declarations corresponding to the C TRACE_EVENT machinery.
// Registration and emission are provided by the surrounding tracepoint code.
pub const MEMCG_RSTAT_STATS_PRINT: &str = "memcg_id=%llu item=%d val=%ld";
pub const MEMCG_RSTAT_EVENTS_PRINT: &str = "memcg_id=%llu item=%d val=%lu";
pub const MEMCG_FLUSH_STATS_PRINT: &str =
    "memcg_id=%llu stats_updates=%lld force=%d needs_flush=%d";

pub type ModMemcgState = memcg_rstat_stats_entry;
pub type ModMemcgLruvecState = memcg_rstat_stats_entry;
pub type CountMemcgEvents = memcg_rstat_events_entry;
pub type MemcgFlushStats = memcg_flush_stats_entry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
