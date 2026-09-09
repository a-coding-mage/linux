// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation; external kernel and trace
// symbols are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const BENCHMARK_EVENT_STRLEN: usize = 128;
const EBUSY: c_int = 16;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn trace_benchmark_event_enabled() -> bool;
    fn tracing_is_on() -> bool;
    fn local_irq_disable();
    fn local_irq_enable();
    fn trace_clock_local() -> u64;
    fn trace_benchmark_event(s: *const c_char, last: u64);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...);
    fn msleep(msecs: c_uint);
    fn kthread_should_stop() -> bool;
    fn cond_resched_tasks_rcu_qs();
    fn kthread_run(threadfn: unsafe extern "C" fn(*mut c_void) -> c_int,
                   data: *mut c_void, name: *const c_char) -> *mut task_struct;
    fn IS_ERR(ptr: *mut task_struct) -> bool;
    fn PTR_ERR(ptr: *mut task_struct) -> c_int;
    fn kthread_stop(thread: *mut task_struct) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn div64_u64(n: u64, d: u64) -> u64;
}

static mut bm_event_thread: *mut task_struct = core::ptr::null_mut();

static mut bm_str: [c_char; BENCHMARK_EVENT_STRLEN] = {
    let mut a = [0; BENCHMARK_EVENT_STRLEN];
    a[0] = b'S' as c_char;
    a[1] = b'T' as c_char;
    a[2] = b'A' as c_char;
    a[3] = b'R' as c_char;
    a[4] = b'T' as c_char;
    a
};
static mut bm_total: u64 = 0;
static mut bm_totalsq: u64 = 0;
static mut bm_last: u64 = 0;
static mut bm_max: u64 = 0;
static mut bm_min: u64 = 0;
static mut bm_first: u64 = 0;
static mut bm_cnt: u64 = 0;
static mut bm_stddev: u64 = 0;
static mut bm_avg: c_uint = 0;
static mut bm_std: c_uint = 0;
static mut ok_to_run: bool = false;

unsafe fn do_div(n: &mut u64, d: u64) {
    *n /= d;
}

unsafe fn trace_do_benchmark() {
    let start: u64;
    let stop: u64;
    let mut delta: u64;
    let mut stddev: u64;
    let mut seed: u64;
    let mut last_seed: u64;
    let avg: c_uint;
    let mut std: c_uint = 0;

    if !trace_benchmark_event_enabled() || !tracing_is_on() { return; }

    local_irq_disable();
    start = trace_clock_local();
    trace_benchmark_event(bm_str.as_ptr(), bm_last);
    stop = trace_clock_local();
    local_irq_enable();

    bm_cnt += 1;
    delta = stop.wrapping_sub(start);

    if bm_cnt == 1 {
        bm_first = delta;
        scnprintf(bm_str.as_mut_ptr(), BENCHMARK_EVENT_STRLEN, b"first=%llu [COLD CACHED]\0".as_ptr() as *const c_char, bm_first);
        return;
    }

    bm_last = delta;
    if delta > bm_max { bm_max = delta; }
    if bm_min == 0 || delta < bm_min { bm_min = delta; }

    if bm_cnt > u32::MAX as u64 {
        scnprintf(bm_str.as_mut_ptr(), BENCHMARK_EVENT_STRLEN, b"last=%llu first=%llu max=%llu min=%llu ** avg=%u std=%d std^2=%lld\0".as_ptr() as *const c_char, bm_last, bm_first, bm_max, bm_min, bm_avg, bm_std, bm_stddev);
        return;
    }

    bm_total += delta;
    bm_totalsq = bm_totalsq.wrapping_add(delta.wrapping_mul(delta));
    if bm_cnt > 1 {
        stddev = (bm_cnt.wrapping_mul(bm_totalsq)).wrapping_sub(bm_total.wrapping_mul(bm_total));
        do_div(&mut stddev, bm_cnt as u32 as u64);
        do_div(&mut stddev, (bm_cnt as u32 - 1) as u64);
    } else { stddev = 0; }
    delta = bm_total;
    do_div(&mut delta, bm_cnt as u32 as u64);
    avg = delta as c_uint;

    if stddev > 0 {
        let mut i = 0;
        seed = avg as u64;
        loop {
            last_seed = seed;
            seed = stddev;
            if last_seed == 0 { break; }
            seed = div64_u64(seed, last_seed);
            seed += last_seed;
            do_div(&mut seed, 2);
            i += 1;
            if !(i <= 10 && last_seed != seed) { break; }
        }
        std = seed as c_uint;
    }
    scnprintf(bm_str.as_mut_ptr(), BENCHMARK_EVENT_STRLEN, b"last=%llu first=%llu max=%llu min=%llu avg=%u std=%d std^2=%lld\0".as_ptr() as *const c_char, bm_last, bm_first, bm_max, bm_min, avg, std, stddev);
    bm_std = std;
    bm_avg = avg;
    bm_stddev = stddev;
}

unsafe extern "C" fn benchmark_event_kthread(_arg: *mut c_void) -> c_int {
    msleep(100);
    while !kthread_should_stop() {
        trace_do_benchmark();
        cond_resched_tasks_rcu_qs();
    }
    0
}

pub unsafe extern "C" fn trace_benchmark_reg() -> c_int {
    if !ok_to_run { pr_warn(b"trace benchmark cannot be started via kernel command line\n\0".as_ptr() as *const c_char); return -EBUSY; }
    bm_event_thread = kthread_run(benchmark_event_kthread, core::ptr::null_mut(), b"event_benchmark\0".as_ptr() as *const c_char);
    if IS_ERR(bm_event_thread) { pr_warn(b"trace benchmark failed to create kernel thread\n\0".as_ptr() as *const c_char); return PTR_ERR(bm_event_thread); }
    0
}

pub unsafe extern "C" fn trace_benchmark_unreg() {
    if bm_event_thread.is_null() { return; }
    kthread_stop(bm_event_thread);
    bm_event_thread = core::ptr::null_mut();
    bm_str = [0; BENCHMARK_EVENT_STRLEN];
    bm_str[0] = b'S' as c_char; bm_str[1] = b'T' as c_char; bm_str[2] = b'A' as c_char;
    bm_str[3] = b'R' as c_char; bm_str[4] = b'T' as c_char;
    bm_total = 0; bm_totalsq = 0; bm_last = 0; bm_max = 0; bm_min = 0; bm_cnt = 0;
    bm_first = 0; bm_std = 0; bm_avg = 0; bm_stddev = 0;
}

unsafe extern "C" fn ok_to_run_trace_benchmark() -> c_int {
    ok_to_run = true;
    0
}

// early_initcall(ok_to_run_trace_benchmark);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
