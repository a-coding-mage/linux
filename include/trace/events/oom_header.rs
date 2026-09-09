/* SPDX-License-Identifier: GPL-2.0 */
// TRACE_SYSTEM: oom
// The C tracepoint includes and macro-generated registration are supplied by
// the surrounding kernel tracepoint infrastructure.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const PG_COUNT_TO_KB_SHIFT: u32 = PAGE_SHIFT - 10;

#[inline]
pub const fn pg_count_to_kb(x: usize) -> usize {
    x << PG_COUNT_TO_KB_SHIFT
}

#[repr(C)]
pub struct oom_score_adj_update_entry {
    pub pid: pid_t,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub oom_score_adj: i16,
}

#[repr(C)]
pub struct reclaim_retry_zone_entry {
    pub node: i32,
    pub zone_idx: i32,
    pub order: i32,
    pub reclaimable: usize,
    pub available: usize,
    pub min_wmark: usize,
    pub no_progress_loops: i32,
    pub wmark_check: bool,
}

#[repr(C)]
pub struct mark_victim_entry {
    pub pid: i32,
    // __string(comm, task->comm)
    pub comm: *mut core::ffi::c_char,
    pub total_vm: usize,
    pub anon_rss: usize,
    pub file_rss: usize,
    pub shmem_rss: usize,
    pub uid: uid_t,
    pub pgtables: usize,
    pub oom_score_adj: i16,
}

#[repr(C)]
pub struct wake_reaper_entry {
    pub pid: i32,
}

#[repr(C)]
pub struct start_task_reaping_entry {
    pub pid: i32,
}

#[repr(C)]
pub struct finish_task_reaping_entry {
    pub pid: i32,
}

#[repr(C)]
pub struct skip_task_reaping_entry {
    pub pid: i32,
}

// The following declarations preserve the C TRACE_EVENT interfaces. Their
// registration and formatted-print implementations are provided externally.
extern "C" {
    pub fn oom_score_adj_update(task: *mut task_struct);
    pub fn reclaim_retry_zone(
        zoneref: *mut zoneref,
        order: i32,
        reclaimable: usize,
        available: usize,
        min_wmark: usize,
        no_progress_loops: i32,
        wmark_check: bool,
    );
    pub fn mark_victim(task: *mut task_struct, uid: uid_t);
    pub fn wake_reaper(pid: i32);
    pub fn start_task_reaping(pid: i32);
    pub fn finish_task_reaping(pid: i32);
    pub fn skip_task_reaping(pid: i32);
}

// #ifdef CONFIG_COMPACTION
#[repr(C)]
pub struct compact_retry_entry {
    pub order: i32,
    pub priority: i32,
    pub result: i32,
    pub retries: i32,
    pub max_retries: i32,
    pub ret: bool,
}

extern "C" {
    pub fn compact_retry(
        order: i32,
        priority: i32,
        result: i32,
        retries: i32,
        max_retries: i32,
        ret: bool,
    );
}
// #endif /* CONFIG_COMPACTION */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
