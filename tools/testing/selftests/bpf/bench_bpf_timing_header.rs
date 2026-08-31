/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from the C header bench_bpf_timing.h. */
/* C includes removed: <stdbool.h>, <linux/types.h>, and "bench.h". */

#[allow(non_camel_case_types)]
pub type __u64 = u64;
#[allow(non_camel_case_types)]
pub type __u32 = u32;

pub const BENCH_NR_SAMPLES: usize = 4096;
pub const BENCH_NR_CPUS: usize = 256;

#[allow(non_camel_case_types)]
pub type bpf_bench_run_fn = Option<unsafe extern "C" fn(ctx: *mut core::ffi::c_void)>;

#[repr(C)]
pub struct bpf_bench_timing {
    pub samples: *mut [__u64; BENCH_NR_SAMPLES], /* skel->bss->timing_samples */
    pub idx: *mut __u32,                         /* skel->bss->timing_idx */
    pub timing_enabled: *mut __u32,              /* &skel->bss->timing_enabled */
    pub batch_iters_bss: *mut __u32,             /* &skel->bss->batch_iters */
    pub batch_iters: __u32,
    pub target_samples: __u32,
    pub nr_cpus: __u32,
    pub warmup_ticks: core::ffi::c_int,
    pub done: bool,
    pub machine_readable: bool,
}

#[macro_export]
macro_rules! BENCH_TIMING_INIT {
    ($t:expr, $skel:expr, $iters:expr) => {{
        (*$t).samples = (*(*$skel).bss).timing_samples;
        (*$t).idx = (*(*$skel).bss).timing_idx;
        (*$t).timing_enabled = &mut (*(*$skel).bss).timing_enabled;
        (*$t).batch_iters_bss = &mut (*(*$skel).bss).batch_iters;
        (*$t).batch_iters = $iters;
        (*$t).target_samples = 200;
        (*$t).nr_cpus = env.nr_cpus;
        (*$t).warmup_ticks = 0;
        (*$t).done = false;
        (*$t).machine_readable = false;
    }};
}

unsafe extern "C" {
    pub fn bpf_bench_timing_measure(t: *mut bpf_bench_timing, res: *mut bench_res);
    pub fn bpf_bench_timing_report(
        t: *mut bpf_bench_timing,
        name: *const core::ffi::c_char,
        desc: *const core::ffi::c_char,
    );
    pub fn bpf_bench_calibrate(
        t: *mut bpf_bench_timing,
        run_fn: bpf_bench_run_fn,
        ctx: *mut core::ffi::c_void,
    );
}
