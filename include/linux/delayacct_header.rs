/* SPDX-License-Identifier: GPL-2.0-or-later */
/* delayacct.h - per-task delay accounting */

/* Dependencies supplied by the surrounding kernel translation unit. */

#[cfg(CONFIG_TASK_DELAY_ACCT)]
#[repr(C)]
pub struct task_delay_info {
    pub lock: raw_spinlock_t,
    pub blkio_start: u64,
    pub blkio_delay_max: u64,
    pub blkio_delay_min: u64,
    pub blkio_delay: u64, /* wait for sync block io completion */
    pub swapin_start: u64,
    pub swapin_delay_max: u64,
    pub swapin_delay_min: u64,
    pub swapin_delay: u64, /* wait for swapin */
    pub blkio_count: u32, /* total count of the number of sync block io operations performed */
    pub swapin_count: u32, /* total count of swapin */
    pub freepages_start: u64,
    pub freepages_delay_max: u64,
    pub freepages_delay_min: u64,
    pub freepages_delay: u64, /* wait for memory reclaim */
    pub thrashing_start: u64,
    pub thrashing_delay_max: u64,
    pub thrashing_delay_min: u64,
    pub thrashing_delay: u64, /* wait for thrashing page */
    pub compact_start: u64,
    pub compact_delay_max: u64,
    pub compact_delay_min: u64,
    pub compact_delay: u64, /* wait for memory compact */
    pub wpcopy_start: u64,
    pub wpcopy_delay_max: u64,
    pub wpcopy_delay_min: u64,
    pub wpcopy_delay: u64, /* wait for write-protect copy */
    pub irq_delay_max: u64,
    pub irq_delay_min: u64,
    pub irq_delay: u64, /* wait for IRQ/SOFTIRQ */
    pub freepages_count: u32, /* total count of memory reclaim */
    pub thrashing_count: u32, /* total count of thrash waits */
    pub compact_count: u32, /* total count of memory compact */
    pub wpcopy_count: u32, /* total count of write-protect copy */
    pub irq_count: u32, /* total count of IRQ/SOFTIRQ */
    pub blkio_delay_max_ts: timespec64,
    pub swapin_delay_max_ts: timespec64,
    pub freepages_delay_max_ts: timespec64,
    pub thrashing_delay_max_ts: timespec64,
    pub compact_delay_max_ts: timespec64,
    pub wpcopy_delay_max_ts: timespec64,
    pub irq_delay_max_ts: timespec64,
}

#[cfg(CONFIG_TASK_DELAY_ACCT)]
extern "C" {
    pub static mut delayacct_key: static_key_false;
    pub static mut delayacct_on: i32;
    pub static mut delayacct_cache: *mut kmem_cache;
    pub fn delayacct_init();
    pub fn __delayacct_tsk_init(tsk: *mut task_struct);
    pub fn __delayacct_tsk_exit(tsk: *mut task_struct);
    pub fn __delayacct_blkio_start();
    pub fn __delayacct_blkio_end(tsk: *mut task_struct);
    pub fn delayacct_add_tsk(d: *mut taskstats, tsk: *mut task_struct) -> i32;
    pub fn __delayacct_blkio_ticks(tsk: *mut task_struct) -> __u64;
    pub fn __delayacct_freepages_start();
    pub fn __delayacct_freepages_end();
    pub fn __delayacct_thrashing_start(in_thrashing: *mut bool);
    pub fn __delayacct_thrashing_end(in_thrashing: *mut bool);
    pub fn __delayacct_swapin_start();
    pub fn __delayacct_swapin_end();
    pub fn __delayacct_compact_start();
    pub fn __delayacct_compact_end();
    pub fn __delayacct_wpcopy_start();
    pub fn __delayacct_wpcopy_end();
    pub fn __delayacct_irq(task: *mut task_struct, delta: u32);
}

/* CONFIG_TASK_DELAY_ACCT wrappers retain the kernel's static-key and delays checks. */
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_tsk_init(tsk: *mut task_struct) {
    (*tsk).delays = core::ptr::null_mut();
    if delayacct_on != 0 { __delayacct_tsk_init(tsk); }
}
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_tsk_free(tsk: *mut task_struct) {
    if !(*tsk).delays.is_null() { kmem_cache_free(delayacct_cache, (*tsk).delays); }
    (*tsk).delays = core::ptr::null_mut();
}

#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_blkio_start() { if static_branch_unlikely(&delayacct_key) && !(*current).delays.is_null() { __delayacct_blkio_start(); } }
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_blkio_end(p: *mut task_struct) { if static_branch_unlikely(&delayacct_key) && !(*p).delays.is_null() { __delayacct_blkio_end(p); } }
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_blkio_ticks(tsk: *mut task_struct) -> __u64 { if !(*tsk).delays.is_null() { __delayacct_blkio_ticks(tsk) } else { 0 } }

#[cfg(CONFIG_TASK_DELAY_ACCT)]
macro_rules! delayacct_current_wrapper { ($name:ident, $inner:ident) => { pub unsafe fn $name() { if static_branch_unlikely(&delayacct_key) && !(*current).delays.is_null() { $inner(); } } }; }
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_freepages_start, __delayacct_freepages_start);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_freepages_end, __delayacct_freepages_end);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_swapin_start, __delayacct_swapin_start);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_swapin_end, __delayacct_swapin_end);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_compact_start, __delayacct_compact_start);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_compact_end, __delayacct_compact_end);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_wpcopy_start, __delayacct_wpcopy_start);
#[cfg(CONFIG_TASK_DELAY_ACCT)] delayacct_current_wrapper!(delayacct_wpcopy_end, __delayacct_wpcopy_end);

#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_thrashing_start(p: *mut bool) { if static_branch_unlikely(&delayacct_key) && !(*current).delays.is_null() { __delayacct_thrashing_start(p); } }
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_thrashing_end(p: *mut bool) { if static_branch_unlikely(&delayacct_key) && !(*current).delays.is_null() { __delayacct_thrashing_end(p); } }
#[cfg(CONFIG_TASK_DELAY_ACCT)]
pub unsafe fn delayacct_irq(task: *mut task_struct, delta: u32) { if static_branch_unlikely(&delayacct_key) && !(*task).delays.is_null() { __delayacct_irq(task, delta); } }

#[cfg(not(CONFIG_TASK_DELAY_ACCT))]
pub unsafe fn delayacct_init() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_tsk_init(_: *mut task_struct) {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_tsk_free(_: *mut task_struct) {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_blkio_start() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_blkio_end(_: *mut task_struct) {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_add_tsk(_: *mut taskstats, _: *mut task_struct) -> i32 { 0 }
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_blkio_ticks(_: *mut task_struct) -> __u64 { 0 }
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_is_task_waiting_on_io(_: *mut task_struct) -> i32 { 0 }
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_freepages_start() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_freepages_end() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_thrashing_start(_: *mut bool) {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_thrashing_end(_: *mut bool) {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_swapin_start() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_swapin_end() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_compact_start() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_compact_end() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_wpcopy_start() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_wpcopy_end() {}
#[cfg(not(CONFIG_TASK_DELAY_ACCT))] pub unsafe fn delayacct_irq(_: *mut task_struct, _: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
