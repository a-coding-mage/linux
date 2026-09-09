// SPDX-License-Identifier: GPL-2.0-or-later
/* delayacct.c - per-task delay accounting
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2006
 */

// Linux kernel dependencies supplied by other translation units.

macro_rules! update_delay {
    ($d:expr, $tsk:expr, $tmp:ident, $type:ident) => {{
        $d.$type##_delay_max = $tsk.delays.$type##_delay_max;
        $d.$type##_delay_min = $tsk.delays.$type##_delay_min;
        $d.$type##_delay_max_ts.tv_sec = $tsk.delays.$type##_delay_max_ts.tv_sec;
        $d.$type##_delay_max_ts.tv_nsec = $tsk.delays.$type##_delay_max_ts.tv_nsec;
        $tmp = $d.$type##_delay_total + $tsk.delays.$type##_delay;
        $d.$type##_delay_total = if $tmp < $d.$type##_delay_total { 0 } else { $tmp };
        $d.$type##_count += $tsk.delays.$type##_count;
    }};
}

static_key_false!(delayacct_key);
static mut delayacct_on: i32 = 0; // Delay accounting turned on/off
static mut delayacct_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn set_delayacct(enabled: bool) {
    if enabled {
        static_branch_enable(&raw mut delayacct_key);
        delayacct_on = 1;
    } else {
        delayacct_on = 0;
        static_branch_disable(&raw mut delayacct_key);
    }
}

unsafe fn delayacct_setup_enable(_str: *mut c_char) -> i32 {
    delayacct_on = 1;
    1
}

pub unsafe fn delayacct_init() {
    delayacct_cache = KMEM_CACHE!(task_delay_info, SLAB_PANIC | SLAB_ACCOUNT);
    delayacct_tsk_init(&raw mut init_task);
    set_delayacct(delayacct_on != 0);
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn sysctl_delayacct(table: *const ctl_table, write: i32, buffer: *mut c_void,
                           lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    let mut state = delayacct_on;
    let mut t = *table;
    if write != 0 && !capable(CAP_SYS_ADMIN) { return -EPERM; }
    t.data = &mut state as *mut _ as *mut c_void;
    let err = proc_dointvec_minmax(&mut t, write, buffer, lenp, ppos);
    if err < 0 { return err; }
    if write != 0 { set_delayacct(state != 0); }
    err
}

#[cfg(CONFIG_SYSCTL)]
static kern_delayacct_table: [ctl_table; 1] = [ctl_table {
    procname: b"task_delayacct\0".as_ptr() as *mut c_char,
    data: core::ptr::null_mut(),
    maxlen: core::mem::size_of::<u32>(),
    mode: 0o644,
    proc_handler: Some(sysctl_delayacct),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_ONE,
}];

#[cfg(CONFIG_SYSCTL)]
unsafe fn kernel_delayacct_sysctls_init() -> i32 {
    register_sysctl_init(b"kernel\0".as_ptr() as *const c_char, &raw const kern_delayacct_table);
    0
}

pub unsafe fn __delayacct_tsk_init(tsk: *mut task_struct) {
    (*tsk).delays = kmem_cache_zalloc(delayacct_cache, GFP_KERNEL);
    if !(*tsk).delays.is_null() { raw_spin_lock_init(&mut (*(*tsk).delays).lock); }
}

/* Finish delay accounting for a statistic using its timestamps (@start),
 * accumulator (@total) and @count
 */
unsafe fn delayacct_end(lock: *mut raw_spinlock_t, start: *mut u64, total: *mut u64,
                        count: *mut u32, max: *mut u64, min: *mut u64, ts: *mut timespec64) {
    let ns = local_clock() - *start;
    let mut flags: unsigned_long = 0;
    if ns > 0 {
        raw_spin_lock_irqsave(lock, &mut flags);
        *total += ns as u64;
        *count += 1;
        if ns as u64 > *max { *max = ns as u64; ktime_get_real_ts64(ts); }
        if *min == 0 || ns as u64 < *min { *min = ns as u64; }
        raw_spin_unlock_irqrestore(lock, flags);
    }
}

pub unsafe fn __delayacct_blkio_start() { (*current).delays.blkio_start = local_clock(); }

pub unsafe fn __delayacct_blkio_end(p: *mut task_struct) {
    delayacct_end(&mut (*(*p).delays).lock, &mut (*(*p).delays).blkio_start,
        &mut (*(*p).delays).blkio_delay, &mut (*(*p).delays).blkio_count,
        &mut (*(*p).delays).blkio_delay_max, &mut (*(*p).delays).blkio_delay_min,
        &mut (*(*p).delays).blkio_delay_max_ts);
}

pub unsafe fn delayacct_add_tsk(d: *mut taskstats, tsk: *mut task_struct) -> i32 {
    let (mut utime, mut stime, mut stimescaled, mut utimescaled): (u64,u64,u64,u64) = (0,0,0,0);
    let (mut t2, mut t3, mut t1): (u64,u64,unsigned_long) = (0,0,0);
    let mut tmp: i64;
    task_cputime(tsk, &mut utime, &mut stime);
    tmp = (*d).cpu_run_real_total as i64 + utime as i64 + stime as i64;
    (*d).cpu_run_real_total = if tmp < (*d).cpu_run_real_total as i64 { 0 } else { tmp as u64 };
    task_cputime_scaled(tsk, &mut utimescaled, &mut stimescaled);
    tmp = (*d).cpu_scaled_run_real_total as i64 + utimescaled as i64 + stimescaled as i64;
    (*d).cpu_scaled_run_real_total = if tmp < (*d).cpu_scaled_run_real_total as i64 { 0 } else { tmp as u64 };
    // No locking available for sched_info (and too expensive to add one); mitigate by snapshotting values.
    t1 = (*tsk).sched_info.pcount; t2 = (*tsk).sched_info.run_delay; t3 = (*tsk).se.sum_exec_runtime;
    (*d).cpu_count += t1;
    (*d).cpu_delay_max = (*tsk).sched_info.max_run_delay; (*d).cpu_delay_min = (*tsk).sched_info.min_run_delay;
    (*d).cpu_delay_max_ts.tv_sec = (*tsk).sched_info.max_run_delay_ts.tv_sec;
    (*d).cpu_delay_max_ts.tv_nsec = (*tsk).sched_info.max_run_delay_ts.tv_nsec;
    tmp = (*d).cpu_delay_total as i64 + t2 as i64; (*d).cpu_delay_total = if tmp < (*d).cpu_delay_total as i64 { 0 } else { tmp as u64 };
    tmp = (*d).cpu_run_virtual_total as i64 + t3 as i64; (*d).cpu_run_virtual_total = if tmp < (*d).cpu_run_virtual_total as i64 { 0 } else { tmp as u64 };
    if (*tsk).delays.is_null() { return 0; }
    let mut flags: unsigned_long = 0; raw_spin_lock_irqsave(&mut (*(*tsk).delays).lock, &mut flags);
    update_delay!((*d), (*tsk), tmp, blkio); update_delay!((*d), (*tsk), tmp, swapin);
    update_delay!((*d), (*tsk), tmp, freepages); update_delay!((*d), (*tsk), tmp, thrashing);
    update_delay!((*d), (*tsk), tmp, compact); update_delay!((*d), (*tsk), tmp, wpcopy);
    update_delay!((*d), (*tsk), tmp, irq); raw_spin_unlock_irqrestore(&mut (*(*tsk).delays).lock, flags); 0
}

pub unsafe fn __delayacct_blkio_ticks(tsk: *mut task_struct) -> u64 {
    let mut ret: u64; let mut flags: unsigned_long = 0;
    raw_spin_lock_irqsave(&mut (*(*tsk).delays).lock, &mut flags);
    ret = nsec_to_clock_t((*(*tsk).delays).blkio_delay); raw_spin_unlock_irqrestore(&mut (*(*tsk).delays).lock, flags); ret
}

pub unsafe fn __delayacct_freepages_start() { (*current).delays.freepages_start = local_clock(); }
pub unsafe fn __delayacct_freepages_end() { delayacct_end(&mut (*(*current).delays).lock, &mut (*(*current).delays).freepages_start, &mut (*(*current).delays).freepages_delay, &mut (*(*current).delays).freepages_count, &mut (*(*current).delays).freepages_delay_max, &mut (*(*current).delays).freepages_delay_min, &mut (*(*current).delays).freepages_delay_max_ts); }

pub unsafe fn __delayacct_thrashing_start(in_thrashing: *mut bool) {
    *in_thrashing = (*current).in_thrashing != 0; if *in_thrashing { return; }
    (*current).in_thrashing = 1; (*current).delays.thrashing_start = local_clock();
}
pub unsafe fn __delayacct_thrashing_end(in_thrashing: *mut bool) {
    if *in_thrashing { return; } (*current).in_thrashing = 0;
    delayacct_end(&mut (*(*current).delays).lock, &mut (*(*current).delays).thrashing_start, &mut (*(*current).delays).thrashing_delay, &mut (*(*current).delays).thrashing_count, &mut (*(*current).delays).thrashing_delay_max, &mut (*(*current).delays).thrashing_delay_min, &mut (*(*current).delays).thrashing_delay_max_ts);
}

pub unsafe fn __delayacct_swapin_start() { (*current).delays.swapin_start = local_clock(); }
pub unsafe fn __delayacct_swapin_end() { delayacct_end(&mut (*(*current).delays).lock, &mut (*(*current).delays).swapin_start, &mut (*(*current).delays).swapin_delay, &mut (*(*current).delays).swapin_count, &mut (*(*current).delays).swapin_delay_max, &mut (*(*current).delays).swapin_delay_min, &mut (*(*current).delays).swapin_delay_max_ts); }
pub unsafe fn __delayacct_compact_start() { (*current).delays.compact_start = local_clock(); }
pub unsafe fn __delayacct_compact_end() { delayacct_end(&mut (*(*current).delays).lock, &mut (*(*current).delays).compact_start, &mut (*(*current).delays).compact_delay, &mut (*(*current).delays).compact_count, &mut (*(*current).delays).compact_delay_max, &mut (*(*current).delays).compact_delay_min, &mut (*(*current).delays).compact_delay_max_ts); }
pub unsafe fn __delayacct_wpcopy_start() { (*current).delays.wpcopy_start = local_clock(); }
pub unsafe fn __delayacct_wpcopy_end() { delayacct_end(&mut (*(*current).delays).lock, &mut (*(*current).delays).wpcopy_start, &mut (*(*current).delays).wpcopy_delay, &mut (*(*current).delays).wpcopy_count, &mut (*(*current).delays).wpcopy_delay_max, &mut (*(*current).delays).wpcopy_delay_min, &mut (*(*current).delays).wpcopy_delay_max_ts); }

pub unsafe fn __delayacct_irq(task: *mut task_struct, delta: u32) {
    let mut flags: unsigned_long = 0; raw_spin_lock_irqsave(&mut (*(*task).delays).lock, &mut flags);
    (*(*task).delays).irq_delay += delta as u64; (*(*task).delays).irq_count += 1;
    if delta as u64 > (*(*task).delays).irq_delay_max { (*(*task).delays).irq_delay_max = delta as u64; ktime_get_real_ts64(&mut (*(*task).delays).irq_delay_max_ts); }
    if delta != 0 && ((*(*task).delays).irq_delay_min == 0 || delta as u64 < (*(*task).delays).irq_delay_min) { (*(*task).delays).irq_delay_min = delta as u64; }
    raw_spin_unlock_irqrestore(&mut (*(*task).delays).lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
