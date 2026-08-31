// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
/*
 * Rust translation of the implementation source. C include dependencies:
 * linux/sched.h, linux/smp.h, linux/delay.h, linux/module.h,
 * linux/prandom.h, linux/ktime.h, asm/rqspinlock.h, linux/perf_event.h,
 * linux/kthread.h, linux/atomic.h, linux/slab.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const NSEC_PER_MSEC: u64 = 1_000_000;
const GFP_KERNEL: c_int = 0;
const EINVAL: c_int = 22;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub pinned: u64,
    pub disabled: u64,
}

#[repr(C)]
pub struct rqspinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct atomic64_t {
    counter: i64,
}

unsafe extern "C" {
    fn smp_processor_id() -> c_int;
    fn num_online_cpus() -> c_int;
    fn kthread_should_stop() -> bool;
    fn kthread_create(
        threadfn: unsafe extern "C" fn(*mut c_void) -> c_int,
        data: *mut c_void,
        namefmt: *const c_char,
        ...
    ) -> *mut task_struct;
    fn kthread_bind(p: *mut task_struct, cpu: c_uint);
    fn kthread_stop(k: *mut task_struct) -> c_int;
    fn wake_up_process(tsk: *mut task_struct) -> c_int;
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_ulong);
    fn cpu_relax();
    fn ktime_get_mono_fast_ns() -> u64;
    fn raw_res_spin_lock_init(lock: *mut rqspinlock_t);
    fn raw_res_spin_lock_irqsave(lock: *mut rqspinlock_t, flags: c_ulong) -> c_int;
    fn raw_res_spin_unlock_irqrestore(lock: *mut rqspinlock_t, flags: c_ulong);
    fn perf_event_create_kernel_counter(
        attr: *mut perf_event_attr,
        cpu: c_int,
        task: *mut task_struct,
        overflow_handler: unsafe extern "C" fn(*mut perf_event, *mut perf_sample_data, *mut pt_regs),
        context: *mut c_void,
    ) -> *mut perf_event;
    fn perf_event_enable(event: *mut perf_event);
    fn perf_event_release_kernel(event: *mut perf_event);
    fn kcalloc(n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic64_inc(v: *mut atomic64_t);
    fn atomic64_read(v: *const atomic64_t) -> u64;
    fn pr_err(fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn this_cpu_ptr_rqsl_cpu_hists() -> *mut rqsl_cpu_hist;
    fn per_cpu_ptr_rqsl_cpu_hists(cpu: c_int) -> *mut rqsl_cpu_hist;
}

static mut hw_attr: perf_event_attr = perf_event_attr {
    type_: PERF_TYPE_HARDWARE,
    config: PERF_COUNT_HW_CPU_CYCLES,
    size: size_of::<perf_event_attr>() as u32,
    pinned: 1,
    disabled: 1,
    sample_period: 100000,
};

static mut lock_a: rqspinlock_t = rqspinlock_t { _private: [] };
static mut lock_b: rqspinlock_t = rqspinlock_t { _private: [] };
static mut lock_c: rqspinlock_t = rqspinlock_t { _private: [] };

const RQSL_SLOW_THRESHOLD_MS: u32 = 10;
static rqsl_hist_ms: [u32; 25] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    12, 14, 16, 18, 20, 25, 30, 40, 50, 75,
    100, 150, 200, 250, 1000,
];
const RQSL_NR_HIST_BUCKETS: usize = rqsl_hist_ms.len();

#[repr(C)]
#[derive(Copy, Clone)]
enum rqsl_context {
    RQSL_CTX_NORMAL = 0,
    RQSL_CTX_NMI,
    RQSL_CTX_MAX,
}

const RQSL_CTX_NORMAL: usize = rqsl_context::RQSL_CTX_NORMAL as usize;
const RQSL_CTX_NMI: usize = rqsl_context::RQSL_CTX_NMI as usize;
const RQSL_CTX_MAX: usize = rqsl_context::RQSL_CTX_MAX as usize;

#[repr(C)]
struct rqsl_cpu_hist {
    hist: [[atomic64_t; RQSL_NR_HIST_BUCKETS]; RQSL_CTX_MAX],
    success: [atomic64_t; RQSL_CTX_MAX],
    failure: [atomic64_t; RQSL_CTX_MAX],
}

/* static DEFINE_PER_CPU(struct rqsl_cpu_hist, rqsl_cpu_hists); */

#[repr(C)]
#[derive(Copy, Clone)]
enum rqsl_mode {
    RQSL_MODE_AA = 0,
    RQSL_MODE_ABBA,
    RQSL_MODE_ABBCCA,
}

const RQSL_MODE_AA: c_int = rqsl_mode::RQSL_MODE_AA as c_int;
const RQSL_MODE_ABBA: c_int = rqsl_mode::RQSL_MODE_ABBA as c_int;
const RQSL_MODE_ABBCCA: c_int = rqsl_mode::RQSL_MODE_ABBCCA as c_int;

static mut test_mode: c_int = RQSL_MODE_AA;
/* module_param(test_mode, int, 0644); */
/* MODULE_PARM_DESC(test_mode,
 *                  "rqspinlock test mode: 0 = AA, 1 = ABBA, 2 = ABBCCA");
 */

static mut normal_delay: c_int = 20;
/* module_param(normal_delay, int, 0644); */
/* MODULE_PARM_DESC(normal_delay,
 *                  "rqspinlock critical section length for normal context (20ms default)");
 */

static mut nmi_delay: c_int = 10;
/* module_param(nmi_delay, int, 0644); */
/* MODULE_PARM_DESC(nmi_delay,
 *                  "rqspinlock critical section length for NMI context (10ms default)");
 */

static mut rqsl_evts: *mut *mut perf_event = ptr::null_mut();
static mut rqsl_nevts: c_int = 0;

static mut rqsl_threads: *mut *mut task_struct = ptr::null_mut();
static mut rqsl_nthreads: c_int = 0;
static mut rqsl_ready_cpus: atomic_t = atomic_t { counter: 0 };

static mut pause: c_int = 0;

static rqsl_mode_names: [*const c_char; 3] = [
    b"AA\0".as_ptr() as *const c_char,
    b"ABBA\0".as_ptr() as *const c_char,
    b"ABBCCA\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct rqsl_lock_pair {
    worker_lock: *mut rqspinlock_t,
    nmi_lock: *mut rqspinlock_t,
}

#[inline]
unsafe fn READ_ONCE_int(p: *const c_int) -> c_int {
    core::ptr::read_volatile(p)
}

#[inline]
unsafe fn WRITE_ONCE_int(p: *mut c_int, v: c_int) {
    core::ptr::write_volatile(p, v);
}

#[inline]
fn DIV_ROUND_UP_ULL(n: u64, d: u64) -> u64 {
    n.wrapping_add(d).wrapping_sub(1) / d
}

unsafe fn rqsl_get_lock_pair(cpu: c_int) -> rqsl_lock_pair {
    let mode = READ_ONCE_int(core::ptr::addr_of!(test_mode));

    match mode {
        RQSL_MODE_ABBA => {
            if (cpu & 1) != 0 {
                return rqsl_lock_pair {
                    worker_lock: core::ptr::addr_of_mut!(lock_b),
                    nmi_lock: core::ptr::addr_of_mut!(lock_a),
                };
            }
            return rqsl_lock_pair {
                worker_lock: core::ptr::addr_of_mut!(lock_a),
                nmi_lock: core::ptr::addr_of_mut!(lock_b),
            };
        }
        RQSL_MODE_ABBCCA => match cpu % 3 {
            0 => {
                return rqsl_lock_pair {
                    worker_lock: core::ptr::addr_of_mut!(lock_a),
                    nmi_lock: core::ptr::addr_of_mut!(lock_b),
                };
            }
            1 => {
                return rqsl_lock_pair {
                    worker_lock: core::ptr::addr_of_mut!(lock_b),
                    nmi_lock: core::ptr::addr_of_mut!(lock_c),
                };
            }
            _ => {
                return rqsl_lock_pair {
                    worker_lock: core::ptr::addr_of_mut!(lock_c),
                    nmi_lock: core::ptr::addr_of_mut!(lock_a),
                };
            }
        },
        _ => rqsl_lock_pair {
            worker_lock: core::ptr::addr_of_mut!(lock_a),
            nmi_lock: core::ptr::addr_of_mut!(lock_a),
        },
    }
}

fn rqsl_hist_bucket_idx(delta_ms: u32) -> u32 {
    let mut i: usize = 0;

    while i < RQSL_NR_HIST_BUCKETS {
        if delta_ms <= rqsl_hist_ms[i] {
            return i as u32;
        }
        i += 1;
    }

    (RQSL_NR_HIST_BUCKETS - 1) as u32
}

unsafe fn rqsl_record_lock_result(delta_ns: u64, ctx: rqsl_context, ret: c_int) {
    let hist = this_cpu_ptr_rqsl_cpu_hists();
    let delta_ms = DIV_ROUND_UP_ULL(delta_ns, NSEC_PER_MSEC) as u32;
    let bucket = rqsl_hist_bucket_idx(delta_ms);
    let buckets = (*hist).hist[ctx as usize].as_mut_ptr();

    atomic64_inc(buckets.add(bucket as usize));
    if ret == 0 {
        atomic64_inc(core::ptr::addr_of_mut!((*hist).success[ctx as usize]));
    } else {
        atomic64_inc(core::ptr::addr_of_mut!((*hist).failure[ctx as usize]));
    }
}

unsafe extern "C" fn rqspinlock_worker_fn(_arg: *mut c_void) -> c_int {
    let cpu = smp_processor_id();
    let mut flags: c_ulong = 0;
    let mut start_ns: u64;
    let mut ret: c_int;

    if cpu != 0 {
        atomic_inc(core::ptr::addr_of_mut!(rqsl_ready_cpus));

        while !kthread_should_stop() {
            let locks = rqsl_get_lock_pair(cpu);
            let worker_lock = locks.worker_lock;

            if READ_ONCE_int(core::ptr::addr_of!(pause)) != 0 {
                msleep(1000);
                continue;
            }
            start_ns = ktime_get_mono_fast_ns();
            ret = raw_res_spin_lock_irqsave(worker_lock, flags);
            rqsl_record_lock_result(
                ktime_get_mono_fast_ns().wrapping_sub(start_ns),
                rqsl_context::RQSL_CTX_NORMAL,
                ret,
            );
            mdelay(normal_delay as c_ulong);
            if ret == 0 {
                raw_res_spin_unlock_irqrestore(worker_lock, flags);
            }
            cpu_relax();
        }
        return 0;
    }

    while !kthread_should_stop() {
        let expected = if rqsl_nthreads > 0 { rqsl_nthreads - 1 } else { 0 };
        let ready = atomic_read(core::ptr::addr_of!(rqsl_ready_cpus));

        if ready == expected && READ_ONCE_int(core::ptr::addr_of!(pause)) == 0 {
            let mut i: c_int = 0;
            while i < rqsl_nevts {
                perf_event_enable(*rqsl_evts.add(i as usize));
                i += 1;
            }
            pr_err(b"Waiting 5 secs to pause the test\n\0".as_ptr() as *const c_char);
            msleep(1000 * 5);
            WRITE_ONCE_int(core::ptr::addr_of_mut!(pause), 1);
            pr_err(b"Paused the test\n\0".as_ptr() as *const c_char);
        } else {
            msleep(1000);
            cpu_relax();
        }
    }
    0
}

unsafe extern "C" fn nmi_cb(
    _event: *mut perf_event,
    _data: *mut perf_sample_data,
    _regs: *mut pt_regs,
) {
    let mut locks: rqsl_lock_pair;
    let cpu = smp_processor_id();
    let mut flags: c_ulong = 0;
    let start_ns: u64;
    let ret: c_int;

    if cpu == 0 || READ_ONCE_int(core::ptr::addr_of!(pause)) != 0 {
        return;
    }

    locks = rqsl_get_lock_pair(cpu);
    start_ns = ktime_get_mono_fast_ns();
    ret = raw_res_spin_lock_irqsave(locks.nmi_lock, flags);
    rqsl_record_lock_result(
        ktime_get_mono_fast_ns().wrapping_sub(start_ns),
        rqsl_context::RQSL_CTX_NMI,
        ret,
    );

    mdelay(nmi_delay as c_ulong);

    if ret == 0 {
        raw_res_spin_unlock_irqrestore(locks.nmi_lock, flags);
    }
}

unsafe fn free_rqsl_threads() {
    let mut i: c_int = 0;

    if !rqsl_threads.is_null() {
        while i < num_online_cpus() {
            if !(*rqsl_threads.add(i as usize)).is_null() {
                kthread_stop(*rqsl_threads.add(i as usize));
            }
            i += 1;
        }
        kfree(rqsl_threads as *mut c_void);
    }
}

unsafe fn free_rqsl_evts() {
    let mut i: c_int = 0;

    if !rqsl_evts.is_null() {
        while i < rqsl_nevts {
            if !(*rqsl_evts.add(i as usize)).is_null() {
                perf_event_release_kernel(*rqsl_evts.add(i as usize));
            }
            i += 1;
        }
        kfree(rqsl_evts as *mut c_void);
    }
}

unsafe fn bpf_test_rqspinlock_init() -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let ncpus = num_online_cpus();

    if test_mode < RQSL_MODE_AA || test_mode > RQSL_MODE_ABBCCA {
        pr_err(
            b"Invalid mode %d\n\0".as_ptr() as *const c_char,
            test_mode,
        );
        return -EINVAL;
    }

    pr_err(
        b"Mode = %s\n\0".as_ptr() as *const c_char,
        rqsl_mode_names[test_mode as usize],
    );

    if ncpus < test_mode + 2 {
        return -ENOTSUPP;
    }

    raw_res_spin_lock_init(core::ptr::addr_of_mut!(lock_a));
    raw_res_spin_lock_init(core::ptr::addr_of_mut!(lock_b));
    raw_res_spin_lock_init(core::ptr::addr_of_mut!(lock_c));

    rqsl_evts = kcalloc(
        (ncpus - 1) as usize,
        size_of::<*mut perf_event>(),
        GFP_KERNEL,
    ) as *mut *mut perf_event;
    if rqsl_evts.is_null() {
        return -ENOMEM;
    }
    rqsl_nevts = ncpus - 1;

    i = 1;
    while i < ncpus {
        let e: *mut perf_event;

        e = perf_event_create_kernel_counter(
            core::ptr::addr_of_mut!(hw_attr),
            i,
            ptr::null_mut(),
            nmi_cb,
            ptr::null_mut(),
        );
        if IS_ERR(e as *const c_void) {
            ret = PTR_ERR(e as *const c_void);
            free_rqsl_evts();
            return ret;
        }
        *rqsl_evts.add((i - 1) as usize) = e;
        i += 1;
    }

    rqsl_threads = kcalloc(
        ncpus as usize,
        size_of::<*mut task_struct>(),
        GFP_KERNEL,
    ) as *mut *mut task_struct;
    if rqsl_threads.is_null() {
        ret = -ENOMEM;
        free_rqsl_evts();
        return ret;
    }
    rqsl_nthreads = ncpus;

    i = 0;
    while i < num_online_cpus() {
        let t: *mut task_struct;

        t = kthread_create(
            rqspinlock_worker_fn,
            ptr::null_mut(),
            b"rqsl_w/%d\0".as_ptr() as *const c_char,
            i,
        );
        if IS_ERR(t as *const c_void) {
            ret = PTR_ERR(t as *const c_void);
            free_rqsl_threads();
            free_rqsl_evts();
            return ret;
        }
        kthread_bind(t, i as c_uint);
        *rqsl_threads.add(i as usize) = t;
        wake_up_process(t);
        i += 1;
    }
    0
}

/* module_init(bpf_test_rqspinlock_init); */

unsafe fn rqsl_print_histograms() {
    let mut cpu: c_int = 0;
    let mut i: c_int;

    pr_err(b"rqspinlock acquisition latency histogram (ms):\n\0".as_ptr() as *const c_char);

    while cpu < num_online_cpus() {
        let hist = per_cpu_ptr_rqsl_cpu_hists(cpu);
        let mut norm_counts: [u64; RQSL_NR_HIST_BUCKETS] = [0; RQSL_NR_HIST_BUCKETS];
        let mut nmi_counts: [u64; RQSL_NR_HIST_BUCKETS] = [0; RQSL_NR_HIST_BUCKETS];
        let mut total_counts: [u64; RQSL_NR_HIST_BUCKETS] = [0; RQSL_NR_HIST_BUCKETS];
        let norm_success: u64;
        let nmi_success: u64;
        let success_total: u64;
        let norm_failure: u64;
        let nmi_failure: u64;
        let failure_total: u64;
        let mut norm_total: u64 = 0;
        let mut nmi_total: u64 = 0;
        let mut total: u64 = 0;
        let mut has_slow = false;

        i = 0;
        while i < RQSL_NR_HIST_BUCKETS as c_int {
            norm_counts[i as usize] = atomic64_read(core::ptr::addr_of!(
                (*hist).hist[RQSL_CTX_NORMAL][i as usize]
            ));
            nmi_counts[i as usize] = atomic64_read(core::ptr::addr_of!(
                (*hist).hist[RQSL_CTX_NMI][i as usize]
            ));
            total_counts[i as usize] =
                norm_counts[i as usize].wrapping_add(nmi_counts[i as usize]);
            norm_total = norm_total.wrapping_add(norm_counts[i as usize]);
            nmi_total = nmi_total.wrapping_add(nmi_counts[i as usize]);
            total = total.wrapping_add(total_counts[i as usize]);
            if rqsl_hist_ms[i as usize] > RQSL_SLOW_THRESHOLD_MS
                && total_counts[i as usize] != 0
            {
                has_slow = true;
            }
            i += 1;
        }

        norm_success = atomic64_read(core::ptr::addr_of!((*hist).success[RQSL_CTX_NORMAL]));
        nmi_success = atomic64_read(core::ptr::addr_of!((*hist).success[RQSL_CTX_NMI]));
        norm_failure = atomic64_read(core::ptr::addr_of!((*hist).failure[RQSL_CTX_NORMAL]));
        nmi_failure = atomic64_read(core::ptr::addr_of!((*hist).failure[RQSL_CTX_NMI]));
        success_total = norm_success.wrapping_add(nmi_success);
        failure_total = norm_failure.wrapping_add(nmi_failure);

        if total == 0 {
            cpu += 1;
            continue;
        }

        if !has_slow {
            pr_err(
                b" cpu%d: total %llu (normal %llu, nmi %llu) | success %llu (normal %llu, nmi %llu) | failure %llu (normal %llu, nmi %llu), all within 0-%ums\n\0"
                    .as_ptr() as *const c_char,
                cpu,
                total,
                norm_total,
                nmi_total,
                success_total,
                norm_success,
                nmi_success,
                failure_total,
                norm_failure,
                nmi_failure,
                RQSL_SLOW_THRESHOLD_MS,
            );
            cpu += 1;
            continue;
        }

        pr_err(
            b" cpu%d: total %llu (normal %llu, nmi %llu) | success %llu (normal %llu, nmi %llu) | failure %llu (normal %llu, nmi %llu)\n\0"
                .as_ptr() as *const c_char,
            cpu,
            total,
            norm_total,
            nmi_total,
            success_total,
            norm_success,
            nmi_success,
            failure_total,
            norm_failure,
            nmi_failure,
        );
        i = 0;
        while i < RQSL_NR_HIST_BUCKETS as c_int {
            let start_ms: u32;

            if total_counts[i as usize] == 0 {
                i += 1;
                continue;
            }

            start_ms = if i == 0 {
                0
            } else {
                rqsl_hist_ms[(i - 1) as usize].wrapping_add(1)
            };
            if i == RQSL_NR_HIST_BUCKETS as c_int - 1 {
                pr_err(
                    b"   >= %ums: total %llu (normal %llu, nmi %llu)\n\0"
                        .as_ptr() as *const c_char,
                    start_ms,
                    total_counts[i as usize],
                    norm_counts[i as usize],
                    nmi_counts[i as usize],
                );
            } else {
                pr_err(
                    b"   %u-%ums: total %llu (normal %llu, nmi %llu)\n\0"
                        .as_ptr() as *const c_char,
                    start_ms,
                    rqsl_hist_ms[i as usize],
                    total_counts[i as usize],
                    norm_counts[i as usize],
                    nmi_counts[i as usize],
                );
            }
            i += 1;
        }
        cpu += 1;
    }
}

unsafe fn bpf_test_rqspinlock_exit() {
    WRITE_ONCE_int(core::ptr::addr_of_mut!(pause), 1);
    free_rqsl_threads();
    free_rqsl_evts();
    rqsl_print_histograms();
}

/* module_exit(bpf_test_rqspinlock_exit); */

/* MODULE_AUTHOR("Kumar Kartikeya Dwivedi"); */
/* MODULE_DESCRIPTION("BPF rqspinlock stress test module"); */
/* MODULE_LICENSE("GPL"); */
