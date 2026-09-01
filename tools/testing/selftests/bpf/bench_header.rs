/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint, c_void};
use core::sync::atomic::Ordering;

// C dependencies removed from executable Rust:
// <stdlib.h>, <stdbool.h>, <linux/err.h>, <errno.h>, <unistd.h>,
// <bpf/bpf.h>, <bpf/libbpf.h>, <math.h>, <time.h>,
// <sys/syscall.h>, <limits.h>.

#[repr(C)]
pub struct argp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set {
    pub cpus: *mut bool,
    pub cpus_len: c_int,
    pub next_cpu: c_int,
}

#[repr(C)]
pub struct env {
    pub bench_name: *mut c_char,
    pub duration_sec: c_int,
    pub warmup_sec: c_int,
    pub verbose: bool,
    pub list: bool,
    pub affinity: bool,
    pub quiet: bool,
    pub stacktrace: bool,
    pub consumer_cnt: c_int,
    pub producer_cnt: c_int,
    pub nr_cpus: c_int,
    pub prod_cpus: cpu_set,
    pub cons_cpus: cpu_set,
}

#[repr(C)]
pub struct basic_stats {
    pub mean: f64,
    pub stddev: f64,
}

#[repr(C)]
pub struct bench_res {
    pub hits: c_long,
    pub drops: c_long,
    pub false_hits: c_long,
    pub important_hits: c_long,
    pub gp_ns: c_ulong,
    pub gp_ct: c_ulong,
    pub stime: c_uint,
    pub duration_ns: c_ulong,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *mut c_void>,
    pub consumer_thread: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(res: *mut bench_res)>,
    pub report_progress:
        Option<unsafe extern "C" fn(iter: c_int, res: *mut bench_res, delta_ns: c_long)>,
    pub report_final: Option<unsafe extern "C" fn(res: *mut bench_res, res_cnt: c_int)>,
}

#[repr(C, align(128))]
pub struct counter {
    pub value: c_long,
}

unsafe extern "C" {
    pub static mut env: env;
    pub static bench: *const bench;

    pub fn setup_libbpf();
    pub fn bench_force_done();
    pub fn hits_drops_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long);
    pub fn hits_drops_report_final(res: *mut bench_res, res_cnt: c_int);
    pub fn false_hits_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long);
    pub fn false_hits_report_final(res: *mut bench_res, res_cnt: c_int);
    pub fn ops_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long);
    pub fn ops_report_final(res: *mut bench_res, res_cnt: c_int);
    pub fn local_storage_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long);
    pub fn local_storage_report_final(res: *mut bench_res, res_cnt: c_int);
    pub fn grace_period_latency_basic_stats(
        res: *mut bench_res,
        res_cnt: c_int,
        gp_stat: *mut basic_stats,
    );
    pub fn grace_period_ticks_basic_stats(
        res: *mut bench_res,
        res_cnt: c_int,
        gp_stat: *mut basic_stats,
    );
}

#[cfg(target_pointer_width = "64")]
type AtomicCLong = core::sync::atomic::AtomicI64;
#[cfg(target_pointer_width = "32")]
type AtomicCLong = core::sync::atomic::AtomicI32;

#[inline]
pub unsafe fn atomic_inc(value: *mut c_long) {
    let atomic = unsafe { &*(value as *const AtomicCLong) };
    let _ = atomic.fetch_add(1 as _, Ordering::Relaxed) + 1;
}

#[inline]
pub unsafe fn atomic_add(value: *mut c_long, n: c_long) {
    let atomic = unsafe { &*(value as *const AtomicCLong) };
    let _ = atomic.fetch_add(n as _, Ordering::Relaxed) + n as _;
}

#[inline]
pub unsafe fn atomic_swap(value: *mut c_long, n: c_long) -> c_long {
    let atomic = unsafe { &*(value as *const AtomicCLong) };
    atomic.swap(n as _, Ordering::Relaxed) as c_long
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
