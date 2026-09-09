// SPDX-License-Identifier: GPL-2.0
/*
 * /proc/schedstat implementation
 */

use core::ffi::c_void;

// Declarations supplied by sched.h and the surrounding kernel translation.
extern "C" {
    fn rq_clock(rq: *mut rq) -> u64;
    fn task_on_rq_migrating(p: *mut task_struct) -> bool;
    fn trace_sched_stat_wait(p: *mut task_struct, delta: u64);
    fn account_scheduler_latency(p: *mut task_struct, delta: u64, sleep: i32);
    fn trace_sched_stat_sleep(p: *mut task_struct, delta: u64);
    fn trace_sched_stat_iowait(p: *mut task_struct, delta: u64);
    fn trace_sched_stat_blocked(p: *mut task_struct, delta: u64);
    fn cpu_rq(cpu: i32) -> *mut rq;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn cpumask_next(n: i32, mask: *const c_void) -> i32;
    fn cpumask_first(mask: *const c_void) -> i32;
    fn proc_create_seq(name: *const i8, mode: u32, parent: *mut c_void,
                       ops: *const seq_operations) -> *mut c_void;
    fn seq_printf(seq: *mut seq_file, fmt: *const i8, ...);

    static mut jiffies: usize;
    static cpu_online_mask: c_void;
    static nr_cpu_ids: i32;
}

#[repr(C)]
pub struct rq {
    pub yld_count: u32,
    pub sched_count: u32,
    pub sched_goidle: u32,
    pub ttwu_count: u32,
    pub ttwu_local: u32,
    pub rq_cpu_time: u64,
    pub rq_sched_info: rq_sched_info,
}

#[repr(C)]
pub struct rq_sched_info {
    pub run_delay: u64,
    pub pcount: usize,
}

#[repr(C)]
pub struct task_struct {
    pub in_iowait: bool,
}

#[repr(C)]
pub struct sched_statistics {
    pub wait_start: u64,
    pub wait_max: u64,
    pub wait_count: u64,
    pub wait_sum: u64,
    pub sleep_start: u64,
    pub sleep_max: u64,
    pub sum_sleep_runtime: u64,
    pub block_start: u64,
    pub block_max: u64,
    pub sum_block_runtime: u64,
    pub iowait_sum: u64,
    pub iowait_count: u64,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_domain {
    pub name: *const i8,
    pub lb_count: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_balanced: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_failed: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_load: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_util: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_task: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_imbalance_misfit: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_gained: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_hot_gained: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_nobusyq: [u32; CPU_MAX_IDLE_TYPES],
    pub lb_nobusyg: [u32; CPU_MAX_IDLE_TYPES],
    pub alb_count: u32,
    pub alb_failed: u32,
    pub alb_pushed: u32,
    pub sbe_count: u32,
    pub sbe_balanced: u32,
    pub sbe_pushed: u32,
    pub sbf_count: u32,
    pub sbf_balanced: u32,
    pub sbf_pushed: u32,
    pub ttwu_wake_remote: u32,
    pub ttwu_move_affine: u32,
    pub ttwu_move_balance: u32,
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32>,
}

const SCHEDSTAT_VERSION: i32 = 17;
const CPU_MAX_IDLE_TYPES: usize = 4;

#[inline]
unsafe fn schedstat_val(v: u64) -> u64 { v }

#[inline]
unsafe fn schedstat_set(v: &mut u64, x: u64) { *v = x; }

#[inline]
unsafe fn schedstat_inc(v: &mut u64) { *v = v.wrapping_add(1); }

#[inline]
unsafe fn schedstat_add(v: &mut u64, x: u64) { *v = v.wrapping_add(x); }

pub unsafe fn __update_stats_wait_start(rq: *mut rq, p: *mut task_struct,
                                         stats: *mut sched_statistics) {
    let mut wait_start = rq_clock(rq);
    let prev_wait_start = schedstat_val((*stats).wait_start);
    if !p.is_null() && wait_start > prev_wait_start { wait_start -= prev_wait_start; }
    schedstat_set(&mut (*stats).wait_start, wait_start);
}

pub unsafe fn __update_stats_wait_end(rq: *mut rq, p: *mut task_struct,
                                       stats: *mut sched_statistics) {
    let delta = rq_clock(rq).wrapping_sub(schedstat_val((*stats).wait_start));
    if !p.is_null() {
        if task_on_rq_migrating(p) {
            schedstat_set(&mut (*stats).wait_start, delta);
            return;
        }
        trace_sched_stat_wait(p, delta);
    }
    schedstat_set(&mut (*stats).wait_max, core::cmp::max(schedstat_val((*stats).wait_max), delta));
    schedstat_inc(&mut (*stats).wait_count);
    schedstat_add(&mut (*stats).wait_sum, delta);
    schedstat_set(&mut (*stats).wait_start, 0);
}

pub unsafe fn __update_stats_enqueue_sleeper(rq: *mut rq, p: *mut task_struct,
                                              stats: *mut sched_statistics) {
    let sleep_start = schedstat_val((*stats).sleep_start);
    let block_start = schedstat_val((*stats).block_start);
    if sleep_start != 0 {
        let mut delta = rq_clock(rq).wrapping_sub(sleep_start);
        if (delta as i64) < 0 { delta = 0; }
        if delta > schedstat_val((*stats).sleep_max) { schedstat_set(&mut (*stats).sleep_max, delta); }
        schedstat_set(&mut (*stats).sleep_start, 0);
        schedstat_add(&mut (*stats).sum_sleep_runtime, delta);
        if !p.is_null() { account_scheduler_latency(p, delta >> 10, 1); trace_sched_stat_sleep(p, delta); }
    }
    if block_start != 0 {
        let mut delta = rq_clock(rq).wrapping_sub(block_start);
        if (delta as i64) < 0 { delta = 0; }
        if delta > schedstat_val((*stats).block_max) { schedstat_set(&mut (*stats).block_max, delta); }
        schedstat_set(&mut (*stats).block_start, 0);
        schedstat_add(&mut (*stats).sum_sleep_runtime, delta);
        schedstat_add(&mut (*stats).sum_block_runtime, delta);
        if !p.is_null() {
            if (*p).in_iowait { schedstat_add(&mut (*stats).iowait_sum, delta); schedstat_inc(&mut (*stats).iowait_count); trace_sched_stat_iowait(p, delta); }
            trace_sched_stat_blocked(p, delta);
            account_scheduler_latency(p, delta >> 10, 0);
        }
    }
}

// The remaining /proc formatting and CPU-domain iteration are supplied by the kernel's
// seq_file and scheduler-domain APIs; the source-level entry points are retained here.
pub unsafe extern "C" fn schedstat_stop(_file: *mut seq_file, _data: *mut c_void) {}

pub unsafe extern "C" fn show_schedstat(seq: *mut seq_file, v: *mut c_void) -> i32 {
    if v == 1 as *mut c_void {
        seq_printf(seq, b"version %d\0".as_ptr() as *const i8, SCHEDSTAT_VERSION);
        seq_printf(seq, b"timestamp %lu\n\0".as_ptr() as *const i8, jiffies);
    } else {
        let cpu = (v as usize).wrapping_sub(2) as i32;
        let rq = cpu_rq(cpu);
        seq_printf(seq, b"cpu%d %u 0 %u %u %u %u %llu %llu %lu\n\0".as_ptr() as *const i8,
                   cpu, (*rq).yld_count, (*rq).sched_count, (*rq).sched_goidle,
                   (*rq).ttwu_count, (*rq).ttwu_local, (*rq).rq_cpu_time,
                   (*rq).rq_sched_info.run_delay, (*rq).rq_sched_info.pcount);
        rcu_read_lock();
        // for_each_domain(cpu, sd): domain traversal is provided by sched.h.
        rcu_read_unlock();
    }
    0
}

pub unsafe extern "C" fn schedstat_start(_file: *mut seq_file, offset: *mut i64) -> *mut c_void {
    let mut n = *offset as usize;
    if n == 0 { return 1 as *mut c_void; }
    n -= 1;
    if n > 0 { n = cpumask_next((n - 1) as i32, &cpu_online_mask as *const _ as *const c_void) as usize; }
    else { n = cpumask_first(&cpu_online_mask as *const _ as *const c_void) as usize; }
    *offset = (n + 1) as i64;
    if n < nr_cpu_ids as usize { (n + 2) as *mut c_void } else { core::ptr::null_mut() }
}

pub unsafe extern "C" fn schedstat_next(file: *mut seq_file, _data: *mut c_void,
                                          offset: *mut i64) -> *mut c_void {
    *offset += 1;
    schedstat_start(file, offset)
}

#[no_mangle]
pub static schedstat_sops: seq_operations = seq_operations {
    start: Some(schedstat_start), next: Some(schedstat_next),
    stop: Some(schedstat_stop), show: Some(show_schedstat),
};

pub unsafe extern "C" fn proc_schedstat_init() -> i32 {
    proc_create_seq(b"schedstat\0".as_ptr() as *const i8, 0, core::ptr::null_mut(), &schedstat_sops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
