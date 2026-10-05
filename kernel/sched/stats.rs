// SPDX-License-Identifier: GPL-2.0
/*
 * /proc/schedstat implementation
 */

// Existing Rust continuation against native baseline
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native headers own all layouts.
// The original owner is included by build_utility.c only under SCHEDSTATS.
#![cfg(CONFIG_SCHEDSTATS)]

#[cfg(CONFIG_RUST_SCHED_STATS)]
compile_error!("SOURCE ONLY HOLD: scheduler statistics is not admitted");

use kernel::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong, c_void};
use kernel::bindings::sched_stats_native::*;

// Owner format version, exactly as defined by the native stats.c oracle.
const SCHEDSTAT_VERSION: c_int = 17;

/// Continue wait accounting with the native runqueue lock and clock current.
///
/// # Safety
/// `rq` and `stats` must be live and serialized as in stats.c; non-null `p`
/// must be the corresponding live task. No Rust references alias native state.
#[no_mangle]
pub unsafe extern "C" fn __update_stats_wait_start(rq: *mut rq, p: *mut task_struct,
                                         stats: *mut sched_statistics) {
    // SAFETY: The caller supplies native scheduler lifetime/locking guarantees.
    unsafe {
        let mut wait_start = lupos_stats_rq_clock(rq);
        let prev_wait_start = lupos_stats_wait_start_read(stats);
        if !p.is_null() && wait_start > prev_wait_start { wait_start -= prev_wait_start; }
        lupos_stats_wait_start_write(stats, wait_start);
    }
}

/// Finish wait accounting, retaining accumulated wait during migration.
///
/// # Safety
/// The same native task, statistics, clock and runqueue-lock rules as
/// `__update_stats_wait_start` apply.
#[no_mangle]
pub unsafe extern "C" fn __update_stats_wait_end(rq: *mut rq, p: *mut task_struct,
                                       stats: *mut sched_statistics) {
    // SAFETY: Native leaves retain READ_ONCE migration and tracepoint semantics.
    unsafe {
        let delta = lupos_stats_rq_clock(rq).wrapping_sub(lupos_stats_wait_start_read(stats));
        if !p.is_null() {
            if lupos_stats_task_on_rq_migrating(p) != 0 {
                lupos_stats_wait_start_write(stats, delta);
                return;
            }
            lupos_stats_trace_wait(p, delta);
        }
        lupos_stats_wait_max_write(stats, core::cmp::max(lupos_stats_wait_max_read(stats), delta));
        lupos_stats_wait_count_write(stats, lupos_stats_wait_count_read(stats).wrapping_add(1));
        lupos_stats_wait_sum_write(stats, lupos_stats_wait_sum_read(stats).wrapping_add(delta));
        lupos_stats_wait_start_write(stats, 0);
    }
}

/// Account sleep/block intervals at enqueue under native scheduler locking.
///
/// # Safety
/// The caller supplies the same native lifetime and synchronization guarantees
/// as the wait-accounting entry points; `p` may be null for a group entity.
#[no_mangle]
pub unsafe extern "C" fn __update_stats_enqueue_sleeper(rq: *mut rq, p: *mut task_struct,
                                              stats: *mut sched_statistics) {
    // SAFETY: Native accessors retain signed accumulator/bitfield types;
    // integer casts reproduce the native u64-to-int latency call conversion.
    unsafe {
        let sleep_start = lupos_stats_sleep_start_read(stats);
        let block_start = lupos_stats_block_start_read(stats);
        if sleep_start != 0 {
            let mut delta = lupos_stats_rq_clock(rq).wrapping_sub(sleep_start);
            if (delta as c_longlong) < 0 { delta = 0; }
            if delta > lupos_stats_sleep_max_read(stats) { lupos_stats_sleep_max_write(stats, delta); }
            lupos_stats_sleep_start_write(stats, 0);
            lupos_stats_sum_sleep_runtime_write(stats,
                lupos_stats_sum_sleep_runtime_read(stats).wrapping_add(delta as c_longlong));
            if !p.is_null() {
                lupos_stats_account_latency(p, (delta >> 10) as c_int, 1);
                lupos_stats_trace_sleep(p, delta);
            }
        }
        if block_start != 0 {
            let mut delta = lupos_stats_rq_clock(rq).wrapping_sub(block_start);
            if (delta as c_longlong) < 0 { delta = 0; }
            if delta > lupos_stats_block_max_read(stats) { lupos_stats_block_max_write(stats, delta); }
            lupos_stats_block_start_write(stats, 0);
            lupos_stats_sum_sleep_runtime_write(stats,
                lupos_stats_sum_sleep_runtime_read(stats).wrapping_add(delta as c_longlong));
            lupos_stats_sum_block_runtime_write(stats,
                lupos_stats_sum_block_runtime_read(stats).wrapping_add(delta as c_longlong));
            if !p.is_null() {
                if lupos_stats_task_in_iowait(p) != 0 {
                    lupos_stats_iowait_sum_write(stats, lupos_stats_iowait_sum_read(stats).wrapping_add(delta));
                    lupos_stats_iowait_count_write(stats, lupos_stats_iowait_count_read(stats).wrapping_add(1));
                    lupos_stats_trace_iowait(p, delta);
                }
                lupos_stats_trace_blocked(p, delta);
                lupos_stats_account_latency(p, (delta >> 10) as c_int, 0);
            }
        }
    }
}

/// Native seq stop callback: the original iterator holds no state to release.
///
/// # Safety
/// Called only by the native seq_file operations registered for schedstat.
#[export_name = "lupos_stats_schedstat_stop"]
pub unsafe extern "C" fn schedstat_stop(_file: *mut seq_file, _data: *mut c_void) {}

/// Emit the version-17 header or runqueue and domain records.
///
/// # Safety
/// Native seq_file supplies a live locked stream and an iterator-produced token.
/// As in stats.c, counters are sampled without an aggregate snapshot lock.
#[export_name = "lupos_stats_show_schedstat"]
pub unsafe extern "C" fn show_schedstat(seq: *mut seq_file, v: *mut c_void) -> c_int {
    // SAFETY: Format argument widths follow native headers. Domain pointers,
    // names, span bits and parent traversal stay within the RCU read section.
    unsafe {
        if v == 1 as *mut c_void {
            seq_printf(seq, b"version %d\n\0".as_ptr() as *const c_char, SCHEDSTAT_VERSION);
            seq_printf(seq, b"timestamp %lu\n\0".as_ptr() as *const c_char, lupos_stats_jiffies());
        } else {
            let cpu = (v as c_ulong).wrapping_sub(2) as c_int;
            let rq = lupos_stats_cpu_rq(cpu);
            seq_printf(seq, b"cpu%d %u 0 %u %u %u %u %llu %llu %lu\0".as_ptr() as *const c_char,
                       cpu, lupos_stats_rq_yld_count(rq), lupos_stats_rq_sched_count(rq),
                       lupos_stats_rq_sched_goidle(rq), lupos_stats_rq_ttwu_count(rq),
                       lupos_stats_rq_ttwu_local(rq), lupos_stats_rq_cpu_time(rq) as c_ulonglong,
                       lupos_stats_rq_run_delay(rq) as c_ulonglong, lupos_stats_rq_pcount(rq));
            seq_printf(seq, b"\n\0".as_ptr() as *const c_char);

            lupos_stats_rcu_read_lock();
            let mut sd = lupos_stats_domain_first(cpu);
            let mut dcount: c_int = 0;
            while !sd.is_null() {
                seq_printf(seq, b"domain%d %s %*pb\0".as_ptr() as *const c_char,
                           dcount, lupos_stats_domain_name(sd),
                           lupos_stats_nr_cpu_ids() as c_int, lupos_stats_domain_span_bits(sd));
                dcount = dcount.wrapping_add(1);
                let mut itype: c_uint = 0;
                while itype < LUPOS_STATS_CPU_MAX_IDLE_TYPES as c_uint {
                    seq_printf(seq, b" %u %u %u %u %u %u %u %u %u %u %u\0".as_ptr() as *const c_char,
                               lupos_stats_domain_lb_count(sd, itype),
                               lupos_stats_domain_lb_balanced(sd, itype),
                               lupos_stats_domain_lb_failed(sd, itype),
                               lupos_stats_domain_lb_imbalance_load(sd, itype),
                               lupos_stats_domain_lb_imbalance_util(sd, itype),
                               lupos_stats_domain_lb_imbalance_task(sd, itype),
                               lupos_stats_domain_lb_imbalance_misfit(sd, itype),
                               lupos_stats_domain_lb_gained(sd, itype),
                               lupos_stats_domain_lb_hot_gained(sd, itype),
                               lupos_stats_domain_lb_nobusyq(sd, itype),
                               lupos_stats_domain_lb_nobusyg(sd, itype));
                    itype += 1;
                }
                seq_printf(seq, b" %u %u %u %u %u %u %u %u %u %u %u %u\n\0".as_ptr() as *const c_char,
                           lupos_stats_domain_alb_count(sd), lupos_stats_domain_alb_failed(sd),
                           lupos_stats_domain_alb_pushed(sd), lupos_stats_domain_sbe_count(sd),
                           lupos_stats_domain_sbe_balanced(sd), lupos_stats_domain_sbe_pushed(sd),
                           lupos_stats_domain_sbf_count(sd), lupos_stats_domain_sbf_balanced(sd),
                           lupos_stats_domain_sbf_pushed(sd), lupos_stats_domain_ttwu_wake_remote(sd),
                           lupos_stats_domain_ttwu_move_affine(sd), lupos_stats_domain_ttwu_move_balance(sd));
                sd = lupos_stats_domain_parent(sd);
            }
            lupos_stats_rcu_read_unlock();
        }
    }
    0
}

/// Native online-CPU iterator, with token 1 for the header and CPU + 2 otherwise.
///
/// # Safety
/// Native seq_file supplies an exclusively writable native loff_t position.
#[export_name = "lupos_stats_schedstat_start"]
pub unsafe extern "C" fn schedstat_start(_file: *mut seq_file, offset: *mut loff_t) -> *mut c_void {
    // SAFETY: Leaves use the actual online mask, including hotplug holes and
    // configured nr_cpu_ids representation. Tokens are never dereferenced.
    unsafe {
        let mut n = *offset as c_ulong;
        if n == 0 { return 1 as *mut c_void; }
        n -= 1;
        if n > 0 { n = lupos_stats_next_online_cpu((n - 1) as c_int) as c_ulong; }
        else { n = lupos_stats_first_online_cpu() as c_ulong; }
        *offset = n.wrapping_add(1) as loff_t;
        if n < lupos_stats_nr_cpu_ids() as c_ulong {
            n.wrapping_add(2) as *mut c_void
        } else {
            core::ptr::null_mut()
        }
    }
}

/// Advance the native seq iterator through the same hotplug-aware start path.
///
/// # Safety
/// `offset` and `file` must meet the native seq_file callback contract.
#[export_name = "lupos_stats_schedstat_next"]
pub unsafe extern "C" fn schedstat_next(file: *mut seq_file, _data: *mut c_void,
                                          offset: *mut loff_t) -> *mut c_void {
    // SAFETY: The native seq framework owns the callback and position lifetime.
    unsafe {
        *offset = (*offset).wrapping_add(1);
        schedstat_start(file, offset)
    }
}

/// Register the native seq table; native subsys_initcall retains init placement.
///
/// # Safety
/// Called once by the native subsystem-init registration wrapper.
#[export_name = "lupos_stats_proc_schedstat_init"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn proc_schedstat_init() -> c_int {
    // SAFETY: The leaf holds a static native seq_operations table, applies the
    // proc_create_seq macro, and preserves the original ignored allocation result.
    unsafe { lupos_stats_proc_create_seq() };
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
