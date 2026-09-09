/* SPDX-License-Identifier: GPL-2.0 */
/* taskstats_kern.h - kernel header for per-task statistics interface
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2006
 *           (C) Balbir Singh,   IBM Corp. 2006
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/taskstats.h, linux/sched/signal.h, and linux/slab.h

#[cfg(CONFIG_TASKSTATS)]
extern "C" {
    pub static mut taskstats_cache: *mut kmem_cache;

    pub fn taskstats_exit(tsk: *mut task_struct, group_dead: i32);
    pub fn taskstats_init_early();
}

#[cfg(CONFIG_TASKSTATS)]
#[inline]
pub unsafe fn taskstats_tgid_free(sig: *mut signal_struct) {
    if !(*sig).stats.is_null() {
        kmem_cache_free(taskstats_cache, (*sig).stats);
    }
}

#[cfg(not(CONFIG_TASKSTATS))]
#[inline]
pub unsafe fn taskstats_exit(_tsk: *mut task_struct, _group_dead: i32) {}

#[cfg(not(CONFIG_TASKSTATS))]
#[inline]
pub unsafe fn taskstats_tgid_free(_sig: *mut signal_struct) {}

#[cfg(not(CONFIG_TASKSTATS))]
#[inline]
pub unsafe fn taskstats_init_early() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
