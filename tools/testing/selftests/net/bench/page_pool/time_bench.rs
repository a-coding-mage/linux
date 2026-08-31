// SPDX-License-Identifier: GPL-2.0-only
/*
 * Benchmarking code execution time inside the kernel
 *
 * Copyright (C) 2014, Red Hat, Inc., Jesper Dangaard Brouer
 */
// C dependency intent:
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #include <linux/module.h>
// #include <linux/time.h>
// #include <linux/perf_event.h> /* perf_event_create_kernel_counter() */
//
// For concurrency testing
// #include <linux/completion.h>
// #include <linux/sched.h>
// #include <linux/workqueue.h>
// #include <linux/kthread.h>
//
// #include "time_bench.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type uint32_t = u32;
type uint64_t = u64;

const NULL: *mut c_void = ptr::null_mut();

static mut verbose: c_int = 1;

/** TSC (Time-Stamp Counter) based **
 * See: linux/time_bench.h
 *  tsc_start_clock() and tsc_stop_clock()
 */

/** Wall-clock based **
 */

/** PMU (Performance Monitor Unit) based **
 */
const PERF_FORMAT: uint64_t = PERF_FORMAT_GROUP
    | PERF_FORMAT_ID
    | PERF_FORMAT_TOTAL_TIME_ENABLED
    | PERF_FORMAT_TOTAL_TIME_RUNNING;

#[repr(C)]
pub struct raw_perf_event {
    pub config: uint64_t, /* event */
    pub config1: uint64_t, /* umask */
    pub save: *mut perf_event,
    pub desc: *mut c_char,
}

/* if HT is enable a maximum of 4 events (5 if one is instructions
 * retired can be specified, if HT is disabled a maximum of 8 (9 if
 * one is instructions retired) can be specified.
 *
 * From Table 19-1. Architectural Performance Events
 * Architectures Software Developer’s Manual Volume 3: System Programming
 * Guide
 */
static mut perf_events: [raw_perf_event; 2] = [
    raw_perf_event {
        config: 0x3c,
        config1: 0x00,
        save: ptr::null_mut(),
        desc: c"Unhalted CPU Cycles".as_ptr() as *mut c_char,
    },
    raw_perf_event {
        config: 0xc0,
        config1: 0x00,
        save: ptr::null_mut(),
        desc: c"Instruction Retired".as_ptr() as *mut c_char,
    },
];

const NUM_EVTS: usize = 2;

/* WARNING: PMU config is currently broken!
 */
#[no_mangle]
pub unsafe extern "C" fn time_bench_PMU_config(enable: bool_t) -> bool_t {
    let mut i: c_int;
    let mut perf_conf: perf_event_attr = core::mem::zeroed();
    let mut perf_event: *mut perf_event;
    let cpu: c_int;

    preempt_disable();
    cpu = smp_processor_id();
    pr_info(c"DEBUG: cpu:%d\n".as_ptr(), cpu);
    preempt_enable();

    memset(
        &mut perf_conf as *mut perf_event_attr as *mut c_void,
        0,
        size_of::<perf_event_attr>(),
    );
    perf_conf.type_ = PERF_TYPE_RAW;
    perf_conf.size = size_of::<perf_event_attr>() as _;
    perf_conf.read_format = PERF_FORMAT;
    perf_conf.pinned = 1;
    perf_conf.exclude_user = 1; /* No userspace events */
    perf_conf.exclude_kernel = 0; /* Only kernel events */

    i = 0;
    while i < NUM_EVTS as c_int {
        perf_conf.disabled = enable as _;
        //perf_conf.disabled = (i == 0) ? 1 : 0;
        perf_conf.config = perf_events[i as usize].config;
        perf_conf.config1 = perf_events[i as usize].config1;
        if verbose != 0 {
            pr_info(
                c"%s() enable PMU counter: %s\n".as_ptr(),
                c"time_bench_PMU_config".as_ptr(),
                perf_events[i as usize].desc,
            );
        }
        perf_event = perf_event_create_kernel_counter(
            &mut perf_conf,
            cpu,
            ptr::null_mut(), /* task */
            None,            /* overflow_handler*/
            ptr::null_mut(), /* context */
        );
        if !perf_event.is_null() {
            perf_events[i as usize].save = perf_event;
            pr_info(
                c"%s():DEBUG perf_event success\n".as_ptr(),
                c"time_bench_PMU_config".as_ptr(),
            );

            perf_event_enable(perf_event);
        } else {
            pr_info(
                c"%s():DEBUG perf_event is NULL\n".as_ptr(),
                c"time_bench_PMU_config".as_ptr(),
            );
        }
        i += 1;
    }

    true
}

/** Generic functions **
 */

/* Calculate stats, store results in record */
#[no_mangle]
pub unsafe extern "C" fn time_bench_calc_stats(rec: *mut time_bench_record) -> bool_t {
    const NANOSEC_PER_SEC: uint64_t = 1000000000; /* 10^9 */
    let mut ns_per_call_tmp_rem: uint64_t = 0;
    let mut ns_per_call_remainder: uint32_t = 0;
    let mut pmc_ipc_tmp_rem: uint64_t = 0;
    let mut pmc_ipc_remainder: uint32_t = 0;
    let mut pmc_ipc_div: uint32_t = 0;
    let mut invoked_cnt_precision: uint32_t = 0;
    let mut invoked_cnt: uint32_t = 0; /* 32-bit due to div_u64_rem() */

    if (*rec).flags & TIME_BENCH_LOOP != 0 {
        if (*rec).invoked_cnt < 1000 {
            pr_err(
                c"ERR: need more(>1000) loops(%llu) for timing\n".as_ptr(),
                (*rec).invoked_cnt,
            );
            return false;
        }
        if (*rec).invoked_cnt > ((1u64 << 32) - 1) {
            /* div_u64_rem() can only support div with 32bit*/
            pr_err(
                c"ERR: Invoke cnt(%llu) too big overflow 32bit\n".as_ptr(),
                (*rec).invoked_cnt,
            );
            return false;
        }
        invoked_cnt = (*rec).invoked_cnt as uint32_t;
    }

    /* TSC (Time-Stamp Counter) records */
    if (*rec).flags & TIME_BENCH_TSC != 0 {
        (*rec).tsc_interval = (*rec).tsc_stop.wrapping_sub((*rec).tsc_start);
        if (*rec).tsc_interval == 0 {
            pr_err(c"ABORT: timing took ZERO TSC time\n".as_ptr());
            return false;
        }
        /* Calculate stats */
        if (*rec).flags & TIME_BENCH_LOOP != 0 {
            (*rec).tsc_cycles = (*rec).tsc_interval / invoked_cnt as uint64_t;
        } else {
            (*rec).tsc_cycles = (*rec).tsc_interval;
        }
    }

    /* Wall-clock time calc */
    if (*rec).flags & TIME_BENCH_WALLCLOCK != 0 {
        (*rec).time_start =
            (*rec).ts_start.tv_nsec as uint64_t + (NANOSEC_PER_SEC * (*rec).ts_start.tv_sec as uint64_t);
        (*rec).time_stop =
            (*rec).ts_stop.tv_nsec as uint64_t + (NANOSEC_PER_SEC * (*rec).ts_stop.tv_sec as uint64_t);
        (*rec).time_interval = (*rec).time_stop.wrapping_sub((*rec).time_start);
        if (*rec).time_interval == 0 {
            pr_err(c"ABORT: timing took ZERO wallclock time\n".as_ptr());
            return false;
        }
        /* Calculate stats */
        /*** Division in kernel it tricky ***/
        /* Orig: time_sec = (time_interval / NANOSEC_PER_SEC); */
        /* remainder only correct because NANOSEC_PER_SEC is 10^9 */
        (*rec).time_sec = div_u64_rem(
            (*rec).time_interval,
            NANOSEC_PER_SEC as uint32_t,
            &mut (*rec).time_sec_remainder,
        );
        //TODO: use existing struct timespec records instead of div?

        if (*rec).flags & TIME_BENCH_LOOP != 0 {
            /*** Division in kernel it tricky ***/
            /* Orig: ns = ((double)time_interval / invoked_cnt); */
            /* First get quotient */
            (*rec).ns_per_call_quotient = div_u64_rem(
                (*rec).time_interval,
                invoked_cnt,
                &mut ns_per_call_remainder,
            );
            /* Now get decimals .xxx precision (incorrect roundup)*/
            ns_per_call_tmp_rem = ns_per_call_remainder as uint64_t;
            invoked_cnt_precision = invoked_cnt / 1000;
            if invoked_cnt_precision > 0 {
                (*rec).ns_per_call_decimal = div_u64_rem(
                    ns_per_call_tmp_rem,
                    invoked_cnt_precision,
                    &mut ns_per_call_remainder,
                );
            }
        }
    }

    /* Performance Monitor Unit (PMU) counters */
    if (*rec).flags & TIME_BENCH_PMU != 0 {
        //FIXME: Overflow handling???
        (*rec).pmc_inst = (*rec).pmc_inst_stop.wrapping_sub((*rec).pmc_inst_start);
        (*rec).pmc_clk = (*rec).pmc_clk_stop.wrapping_sub((*rec).pmc_clk_start);

        /* Calc Instruction Per Cycle (IPC) */
        /* First get quotient */
        (*rec).pmc_ipc_quotient =
            div_u64_rem((*rec).pmc_inst, (*rec).pmc_clk as uint32_t, &mut pmc_ipc_remainder);
        /* Now get decimals .xxx precision (incorrect roundup)*/
        pmc_ipc_tmp_rem = pmc_ipc_remainder as uint64_t;
        pmc_ipc_div = ((*rec).pmc_clk / 1000) as uint32_t;
        if pmc_ipc_div > 0 {
            (*rec).pmc_ipc_decimal =
                div_u64_rem(pmc_ipc_tmp_rem, pmc_ipc_div, &mut pmc_ipc_remainder);
        }
    }

    true
}

/* Generic function for invoking a loop function and calculating
 * execution time stats.  The function being called/timed is assumed
 * to perform a tight loop, and update the timing record struct.
 */
#[no_mangle]
pub unsafe extern "C" fn time_bench_loop(
    loops: uint32_t,
    step: c_int,
    txt: *mut c_char,
    data: *mut c_void,
    func: Option<unsafe extern "C" fn(record: *mut time_bench_record, data: *mut c_void) -> c_int>,
) -> bool_t {
    let mut rec: time_bench_record = core::mem::zeroed();

    /* Setup record */
    memset(
        &mut rec as *mut time_bench_record as *mut c_void,
        0,
        size_of_val(&rec),
    ); /* zero func might not update all */
    rec.version_abi = 1;
    rec.loops = loops;
    rec.step = step;
    rec.flags = TIME_BENCH_LOOP | TIME_BENCH_TSC | TIME_BENCH_WALLCLOCK;

    /*** Loop function being timed ***/
    if func.unwrap()(&mut rec, data) == 0 {
        pr_err(c"ABORT: function being timed failed\n".as_ptr());
        return false;
    }

    if rec.invoked_cnt < loops as uint64_t {
        pr_warn(
            c"WARNING: Invoke count(%llu) smaller than loops(%d)\n".as_ptr(),
            rec.invoked_cnt,
            loops,
        );
    }

    /* Calculate stats */
    time_bench_calc_stats(&mut rec);

    pr_info(
        c"Type:%s Per elem: %llu cycles(tsc) %llu.%03llu ns (step:%d) - (measurement period time:%llu.%09u sec time_interval:%llu) - (invoke count:%llu tsc_interval:%llu)\n".as_ptr(),
        txt,
        rec.tsc_cycles,
        rec.ns_per_call_quotient,
        rec.ns_per_call_decimal,
        rec.step,
        rec.time_sec,
        rec.time_sec_remainder,
        rec.time_interval,
        rec.invoked_cnt,
        rec.tsc_interval,
    );
    if rec.flags & TIME_BENCH_PMU != 0 {
        pr_info(
            c"Type:%s PMU inst/clock%llu/%llu = %llu.%03llu IPC (inst per cycle)\n".as_ptr(),
            txt,
            rec.pmc_inst,
            rec.pmc_clk,
            rec.pmc_ipc_quotient,
            rec.pmc_ipc_decimal,
        );
    }
    true
}

/* Function getting invoked by kthread */
unsafe extern "C" fn invoke_test_on_cpu_func(private: *mut c_void) -> c_int {
    let cpu: *mut time_bench_cpu = private as *mut time_bench_cpu;
    let sync: *mut time_bench_sync = (*cpu).sync;
    let mut newmask: cpumask_t = CPU_MASK_NONE;
    let data: *mut c_void = (*cpu).data;

    /* Restrict CPU */
    cpumask_set_cpu((*cpu).rec.cpu, &mut newmask);
    set_cpus_allowed_ptr(current, &mut newmask);

    /* Synchronize start of concurrency test */
    atomic_inc(&mut (*sync).nr_tests_running);
    wait_for_completion(&mut (*sync).start_event);

    /* Start benchmark function */
    if ((*cpu).bench_func).unwrap()(&mut (*cpu).rec, data) == 0 {
        pr_err(
            c"ERROR: function being timed failed on CPU:%d(%d)\n".as_ptr(),
            (*cpu).rec.cpu,
            smp_processor_id(),
        );
    } else if verbose != 0 {
        pr_info(
            c"SUCCESS: ran on CPU:%d(%d)\n".as_ptr(),
            (*cpu).rec.cpu,
            smp_processor_id(),
        );
    }
    (*cpu).did_bench_run = true;

    /* End test */
    atomic_dec(&mut (*sync).nr_tests_running);
    /*  Wait for kthread_stop() telling us to stop */
    while !kthread_should_stop() {
        set_current_state(TASK_INTERRUPTIBLE);
        schedule();
    }
    __set_current_state(TASK_RUNNING);
    0
}

#[repr(C)]
struct sum {
    tsc_cycles: uint64_t,
    records: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn time_bench_print_stats_cpumask(
    desc: *const c_char,
    cpu_tasks: *mut time_bench_cpu,
    mask: *const cpumask,
) {
    let mut average: uint64_t = 0;
    let mut cpu: c_int = 0;
    let mut step: c_int = 0;
    let mut sum = sum {
        tsc_cycles: 0,
        records: 0,
    };

    /* Get stats */
    for_each_cpu_begin(&mut cpu, mask);
    while for_each_cpu_cond(cpu, mask) {
        let c: *mut time_bench_cpu = cpu_tasks.offset(cpu as isize);
        let rec: *mut time_bench_record = &mut (*c).rec;

        /* Calculate stats */
        time_bench_calc_stats(rec);

        pr_info(
            c"Type:%s CPU(%d) %llu cycles(tsc) %llu.%03llu ns (step:%d) - (measurement period time:%llu.%09u sec time_interval:%llu) - (invoke count:%llu tsc_interval:%llu)\n".as_ptr(),
            desc,
            cpu,
            (*rec).tsc_cycles,
            (*rec).ns_per_call_quotient,
            (*rec).ns_per_call_decimal,
            (*rec).step,
            (*rec).time_sec,
            (*rec).time_sec_remainder,
            (*rec).time_interval,
            (*rec).invoked_cnt,
            (*rec).tsc_interval,
        );

        /* Collect average */
        sum.records += 1;
        sum.tsc_cycles = sum.tsc_cycles.wrapping_add((*rec).tsc_cycles);
        step = (*rec).step;

        for_each_cpu_next(&mut cpu, mask);
    }

    if sum.records != 0 {
        /* avoid div-by-zero */
        average = sum.tsc_cycles / sum.records as uint64_t;
    }
    pr_info(
        c"Sum Type:%s Average: %llu cycles(tsc) CPUs:%d step:%d\n".as_ptr(),
        desc,
        average,
        sum.records,
        step,
    );
}

#[no_mangle]
pub unsafe extern "C" fn time_bench_run_concurrent(
    loops: uint32_t,
    step: c_int,
    data: *mut c_void,
    mask: *const cpumask, /* Support masking outsome CPUs*/
    sync: *mut time_bench_sync,
    cpu_tasks: *mut time_bench_cpu,
    func: Option<unsafe extern "C" fn(record: *mut time_bench_record, data: *mut c_void) -> c_int>,
) {
    let mut cpu: c_int = 0;
    let mut running: c_int = 0;

    if verbose != 0 {
        // DEBUG
        pr_warn(
            c"%s() Started on CPU:%d\n".as_ptr(),
            c"time_bench_run_concurrent".as_ptr(),
            smp_processor_id(),
        );
    }

    /* Reset sync conditions */
    atomic_set(&mut (*sync).nr_tests_running, 0);
    init_completion(&mut (*sync).start_event);

    /* Spawn off jobs on all CPUs */
    for_each_cpu_begin(&mut cpu, mask);
    while for_each_cpu_cond(cpu, mask) {
        let c: *mut time_bench_cpu = cpu_tasks.offset(cpu as isize);

        running += 1;
        (*c).sync = sync; /* Send sync variable along */
        (*c).data = data; /* Send opaque along */

        /* Init benchmark record */
        memset(
            &mut (*c).rec as *mut time_bench_record as *mut c_void,
            0,
            size_of::<time_bench_record>(),
        );
        (*c).rec.version_abi = 1;
        (*c).rec.loops = loops;
        (*c).rec.step = step;
        (*c).rec.flags = TIME_BENCH_LOOP | TIME_BENCH_TSC | TIME_BENCH_WALLCLOCK;
        (*c).rec.cpu = cpu;
        (*c).bench_func = func;
        (*c).task = kthread_run(
            Some(invoke_test_on_cpu_func),
            c as *mut c_void,
            c"time_bench%d".as_ptr(),
            cpu,
        );
        if IS_ERR((*c).task as *const c_void) {
            pr_err(
                c"%s(): Failed to start test func\n".as_ptr(),
                c"time_bench_run_concurrent".as_ptr(),
            );
            return; /* Argh, what about cleanup?! */
        }

        for_each_cpu_next(&mut cpu, mask);
    }

    /* Wait until all processes are running */
    while atomic_read(&mut (*sync).nr_tests_running) < running {
        set_current_state(TASK_UNINTERRUPTIBLE);
        schedule_timeout(10);
    }
    /* Kick off all CPU concurrently on completion event */
    complete_all(&mut (*sync).start_event);

    /* Wait for CPUs to finish */
    while atomic_read(&mut (*sync).nr_tests_running) != 0 {
        set_current_state(TASK_UNINTERRUPTIBLE);
        schedule_timeout(10);
    }

    /* Stop the kthreads */
    for_each_cpu_begin(&mut cpu, mask);
    while for_each_cpu_cond(cpu, mask) {
        let c: *mut time_bench_cpu = cpu_tasks.offset(cpu as isize);

        kthread_stop((*c).task);

        for_each_cpu_next(&mut cpu, mask);
    }

    if verbose != 0 {
        // DEBUG - happens often, finish on another CPU
        pr_warn(
            c"%s() Finished on CPU:%d\n".as_ptr(),
            c"time_bench_run_concurrent".as_ptr(),
            smp_processor_id(),
        );
    }
}

extern "C" {
    static mut current: *mut task_struct;

    static CPU_MASK_NONE: cpumask_t;

    static PERF_FORMAT_GROUP: uint64_t;
    static PERF_FORMAT_ID: uint64_t;
    static PERF_FORMAT_TOTAL_TIME_ENABLED: uint64_t;
    static PERF_FORMAT_TOTAL_TIME_RUNNING: uint64_t;
    static PERF_TYPE_RAW: uint32_t;

    static TIME_BENCH_LOOP: uint32_t;
    static TIME_BENCH_TSC: uint32_t;
    static TIME_BENCH_WALLCLOCK: uint32_t;
    static TIME_BENCH_PMU: uint32_t;

    static TASK_INTERRUPTIBLE: c_int;
    static TASK_RUNNING: c_int;
    static TASK_UNINTERRUPTIBLE: c_int;

    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);

    fn preempt_disable();
    fn preempt_enable();
    fn smp_processor_id() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn div_u64_rem(dividend: uint64_t, divisor: uint32_t, remainder: *mut uint32_t) -> uint64_t;

    fn perf_event_create_kernel_counter(
        attr: *mut perf_event_attr,
        cpu: c_int,
        task: *mut task_struct,
        overflow_handler: Option<unsafe extern "C" fn()>,
        context: *mut c_void,
    ) -> *mut perf_event;
    fn perf_event_enable(event: *mut perf_event);

    fn cpumask_set_cpu(cpu: c_int, dstp: *mut cpumask_t);
    fn set_cpus_allowed_ptr(p: *mut task_struct, new_mask: *const cpumask_t) -> c_int;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn wait_for_completion(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn complete_all(x: *mut completion);
    fn kthread_should_stop() -> bool_t;
    fn set_current_state(state_value: c_int);
    fn __set_current_state(state_value: c_int);
    fn schedule();
    fn schedule_timeout(timeout: i64) -> i64;
    fn kthread_run(
        threadfn: Option<unsafe extern "C" fn(data: *mut c_void) -> c_int>,
        data: *mut c_void,
        namefmt: *const c_char,
        ...
    ) -> *mut task_struct;
    fn kthread_stop(k: *mut task_struct) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_t;

    // Rust placeholders for the C for_each_cpu(cpu, mask) macro iteration.
    fn for_each_cpu_begin(cpu: *mut c_int, mask: *const cpumask);
    fn for_each_cpu_cond(cpu: c_int, mask: *const cpumask) -> bool_t;
    fn for_each_cpu_next(cpu: *mut c_int, mask: *const cpumask);
}

extern "C" {
    pub type perf_event;
    pub type task_struct;
    pub type cpumask;
    pub type cpumask_t;
    pub type atomic_t;
    pub type completion;
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: uint32_t,
    pub size: uint32_t,
    pub config: uint64_t,
    pub read_format: uint64_t,
    pub disabled: uint64_t,
    pub pinned: uint64_t,
    pub exclude_user: uint64_t,
    pub exclude_kernel: uint64_t,
    pub config1: uint64_t,
}

#[repr(C)]
pub struct time_bench_record {
    pub version_abi: uint32_t,
    pub loops: uint32_t,
    pub step: c_int,
    pub flags: uint32_t,
    pub cpu: c_int,
    pub invoked_cnt: uint64_t,
    pub tsc_start: uint64_t,
    pub tsc_stop: uint64_t,
    pub tsc_interval: uint64_t,
    pub tsc_cycles: uint64_t,
    pub ts_start: timespec64,
    pub ts_stop: timespec64,
    pub time_start: uint64_t,
    pub time_stop: uint64_t,
    pub time_interval: uint64_t,
    pub time_sec: uint64_t,
    pub time_sec_remainder: uint32_t,
    pub ns_per_call_quotient: uint64_t,
    pub ns_per_call_decimal: uint64_t,
    pub pmc_inst_start: uint64_t,
    pub pmc_inst_stop: uint64_t,
    pub pmc_inst: uint64_t,
    pub pmc_clk_start: uint64_t,
    pub pmc_clk_stop: uint64_t,
    pub pmc_clk: uint64_t,
    pub pmc_ipc_quotient: uint64_t,
    pub pmc_ipc_decimal: uint64_t,
}

#[repr(C)]
pub struct time_bench_sync {
    pub nr_tests_running: atomic_t,
    pub start_event: completion,
}

#[repr(C)]
pub struct time_bench_cpu {
    pub rec: time_bench_record,
    pub sync: *mut time_bench_sync,
    pub data: *mut c_void,
    pub bench_func:
        Option<unsafe extern "C" fn(record: *mut time_bench_record, data: *mut c_void) -> c_int>,
    pub task: *mut task_struct,
    pub did_bench_run: bool_t,
}

fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}
