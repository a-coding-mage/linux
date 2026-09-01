/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Benchmarking code execution time inside the kernel
 *
 * Copyright (C) 2014, Red Hat, Inc., Jesper Dangaard Brouer
 *  for licensing details see kernel-base/COPYING
 */

/* C header guard _LINUX_TIME_BENCH_H omitted in Rust translation. */

/* Dependencies expected from surrounding kernel bindings:
 * atomic_t, completion, timespec64, task_struct, cpumask,
 * ktime_get_real_ts64(), rdmsrq_safe().
 */

/* Main structure used for recording a benchmark run */
#[repr(C)]
pub struct time_bench_record {
    pub version_abi: u32,
    pub loops: u32, /* Requested loop invocations */
    pub step: u32,  /* option for e.g. bulk invocations */

    pub flags: u32, /* Measurements types enabled */

    pub cpu: u32, /* Used when embedded in time_bench_cpu */

    /* Records */
    pub invoked_cnt: u64, /* Returned actual invocations */
    pub tsc_start: u64,
    pub tsc_stop: u64,
    pub ts_start: timespec64,
    pub ts_stop: timespec64,
    /* PMU counters for instruction and cycles
     * instructions counter including pipelined instructions
     */
    pub pmc_inst_start: u64,
    pub pmc_inst_stop: u64,
    /* CPU unhalted clock counter */
    pub pmc_clk_start: u64,
    pub pmc_clk_stop: u64,

    /* Result records */
    pub tsc_interval: u64,
    pub time_start: u64,
    pub time_stop: u64,
    pub time_interval: u64, /* in nanosec */
    pub pmc_inst: u64,
    pub pmc_clk: u64,

    /* Derived result records */
    pub tsc_cycles: u64, /* +decimal? */
    pub ns_per_call_quotient: u64,
    pub ns_per_call_decimal: u64,
    pub time_sec: u64,
    pub time_sec_remainder: u32,
    pub pmc_ipc_quotient: u64,
    pub pmc_ipc_decimal: u64, /* inst per cycle */
}

pub const TIME_BENCH_LOOP: u32 = 1u32 << 0;
pub const TIME_BENCH_TSC: u32 = 1u32 << 1;
pub const TIME_BENCH_WALLCLOCK: u32 = 1u32 << 2;
pub const TIME_BENCH_PMU: u32 = 1u32 << 3;

/* For synchronizing parallel CPUs to run concurrently */
#[repr(C)]
pub struct time_bench_sync {
    pub nr_tests_running: atomic_t,
    pub start_event: completion,
}

/* Keep track of CPUs executing our bench function.
 *
 * Embed a time_bench_record for storing info per cpu
 */
#[repr(C)]
pub struct time_bench_cpu {
    pub rec: time_bench_record,
    pub sync: *mut time_bench_sync, /* back ptr */
    pub task: *mut task_struct,
    /* "data" opaque could have been placed in time_bench_sync,
     * but to avoid any false sharing, place it per CPU
     */
    pub data: *mut core::ffi::c_void,
    /* Support masking outsome CPUs, mark if it ran */
    pub did_bench_run: bool,
    /* int cpu; // note CPU stored in time_bench_record */
    pub bench_func:
        Option<unsafe extern "C" fn(record: *mut time_bench_record, data: *mut core::ffi::c_void) -> i32>,
}

/*
 * Below TSC assembler code is not compatible with other archs, and
 * can also fail on guests if cpu-flags are not correct.
 *
 * The way TSC reading is used, many iterations, does not require as
 * high accuracy as described below (in Intel Doc #324264).
 *
 * Considering changing to use get_cycles() (#include <asm/timex.h>).
 */

/** TSC (Time-Stamp Counter) based **
 * Recommend reading, to understand details of reading TSC accurately:
 *  Intel Doc #324264, "How to Benchmark Code Execution Times on Intel"
 *
 * Consider getting exclusive ownership of CPU by using:
 *   unsigned long flags;
 *   preempt_disable();
 *   raw_local_irq_save(flags);
 *   _your_code_
 *   raw_local_irq_restore(flags);
 *   preempt_enable();
 *
 * Clobbered registers: "%rax", "%rbx", "%rcx", "%rdx"
 *  RDTSC only change "%rax" and "%rdx" but
 *  CPUID clears the high 32-bits of all (rax/rbx/rcx/rdx)
 */
#[inline(always)]
pub unsafe fn tsc_start_clock() -> u64 {
    /* See: Intel Doc #324264 */
    let hi: u32;
    let lo: u32;

    core::arch::asm!(
        "push rbx",
        "cpuid",
        "rdtsc",
        "mov {hi:e}, edx",
        "mov {lo:e}, eax",
        "pop rbx",
        hi = lateout(reg) hi,
        lo = lateout(reg) lo,
        lateout("rax") _,
        lateout("rcx") _,
        lateout("rdx") _,
    );
    /* FIXME: on 32bit use clobbered %eax + %edx */
    (lo as u64) | ((hi as u64) << 32)
}

#[inline(always)]
pub unsafe fn tsc_stop_clock() -> u64 {
    /* See: Intel Doc #324264 */
    let hi: u32;
    let lo: u32;

    core::arch::asm!(
        "rdtscp",
        "mov {hi:e}, edx",
        "mov {lo:e}, eax",
        "push rbx",
        "cpuid",
        "pop rbx",
        hi = lateout(reg) hi,
        lo = lateout(reg) lo,
        lateout("rax") _,
        lateout("rcx") _,
        lateout("rdx") _,
    );
    (lo as u64) | ((hi as u64) << 32)
}

/** Wall-clock based **
 *
 * use: getnstimeofday()
 *  getnstimeofday(&rec->ts_start);
 *  getnstimeofday(&rec->ts_stop);
 *
 * API changed see: Documentation/core-api/timekeeping.rst
 *  https://www.kernel.org/doc/html/latest/core-api/timekeeping.html#c.getnstimeofday
 *
 * We should instead use: ktime_get_real_ts64() is a direct
 *  replacement, but consider using monotonic time (ktime_get_ts64())
 *  and/or a ktime_t based interface (ktime_get()/ktime_get_real()).
 */

/** PMU (Performance Monitor Unit) based **
 *
 * Needed for calculating: Instructions Per Cycle (IPC)
 * - The IPC number tell how efficient the CPU pipelining were
 */
/* lookup: perf_event_create_kernel_counter() */

unsafe extern "C" {
    pub fn time_bench_PMU_config(enable: bool) -> bool;
}

/* Raw reading via rdpmc() using fixed counters
 *
 * From: https://github.com/andikleen/simple-pmu
 */
pub const FIXED_SELECT: u32 = 1u32 << 30; /* == 0x40000000 */
pub const FIXED_INST_RETIRED_ANY: u32 = 0;
pub const FIXED_CPU_CLK_UNHALTED_CORE: u32 = 1;
pub const FIXED_CPU_CLK_UNHALTED_REF: u32 = 2;

#[inline(always)]
pub unsafe fn p_rdpmc(in_: u32) -> u64 {
    let d: u32;
    let a: u32;

    core::arch::asm!(
        "rdpmc",
        lateout("edx") d,
        lateout("eax") a,
        in("ecx") in_,
        options(nostack, preserves_flags),
    );
    ((d as u64) << 32) | (a as u64)
}

/* These PMU counter needs to be enabled, but I don't have the
 * configure code implemented.  My current hack is running:
 *  sudo perf stat -e cycles:k -e instructions:k insmod lib/ring_queue_test.ko
 */
/* Reading all pipelined instruction */
#[inline(always)]
pub unsafe fn pmc_inst() -> u64 {
    p_rdpmc(FIXED_SELECT | FIXED_INST_RETIRED_ANY)
}

/* Reading CPU clock cycles */
#[inline(always)]
pub unsafe fn pmc_clk() -> u64 {
    p_rdpmc(FIXED_SELECT | FIXED_CPU_CLK_UNHALTED_CORE)
}

/* Raw reading via MSR rdmsr() is likely wrong
 * FIXME: How can I know which raw MSR registers are conf for what?
 */
pub const MSR_IA32_PCM0: u32 = 0x400000C1; /* PERFCTR0 */
pub const MSR_IA32_PCM1: u32 = 0x400000C2; /* PERFCTR1 */
pub const MSR_IA32_PCM2: u32 = 0x400000C3;

#[inline]
pub unsafe fn msr_inst(msr_result: *mut u64) -> u64 {
    rdmsrq_safe(MSR_IA32_PCM0, msr_result)
}

/** Generic functions **
 */
unsafe extern "C" {
    pub fn time_bench_loop(
        loops: u32,
        step: i32,
        txt: *mut core::ffi::c_char,
        data: *mut core::ffi::c_void,
        func: Option<
            unsafe extern "C" fn(rec: *mut time_bench_record, data: *mut core::ffi::c_void) -> i32,
        >,
    ) -> bool;
    pub fn time_bench_calc_stats(rec: *mut time_bench_record) -> bool;

    pub fn time_bench_run_concurrent(
        loops: u32,
        step: i32,
        data: *mut core::ffi::c_void,
        mask: *const cpumask, /* Support masking outsome CPUs*/
        sync: *mut time_bench_sync,
        cpu_tasks: *mut time_bench_cpu,
        func: Option<
            unsafe extern "C" fn(record: *mut time_bench_record, data: *mut core::ffi::c_void) -> i32,
        >,
    );
    pub fn time_bench_print_stats_cpumask(
        desc: *const core::ffi::c_char,
        cpu_tasks: *mut time_bench_cpu,
        mask: *const cpumask,
    );
}

/* FIXME: use rec->flags to select measurement, should be MACRO */
#[inline(always)]
pub unsafe fn time_bench_start(rec: *mut time_bench_record) {
    /* getnstimeofday(&rec->ts_start); */
    ktime_get_real_ts64(core::ptr::addr_of_mut!((*rec).ts_start));
    if ((*rec).flags & TIME_BENCH_PMU) != 0 {
        (*rec).pmc_inst_start = pmc_inst();
        (*rec).pmc_clk_start = pmc_clk();
    }
    (*rec).tsc_start = tsc_start_clock();
}

#[inline(always)]
pub unsafe fn time_bench_stop(rec: *mut time_bench_record, invoked_cnt: u64) {
    (*rec).tsc_stop = tsc_stop_clock();
    if ((*rec).flags & TIME_BENCH_PMU) != 0 {
        (*rec).pmc_inst_stop = pmc_inst();
        (*rec).pmc_clk_stop = pmc_clk();
    }
    /* getnstimeofday(&rec->ts_stop); */
    ktime_get_real_ts64(core::ptr::addr_of_mut!((*rec).ts_stop));
    (*rec).invoked_cnt = invoked_cnt;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
