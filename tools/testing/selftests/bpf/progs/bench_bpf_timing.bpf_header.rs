// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

// C dependencies in the original header:
// <stdbool.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf_may_goto.h>

pub const BENCH_NR_SAMPLES: usize = 4096;
pub const BENCH_NR_CPUS: usize = 256;
pub const BENCH_CPU_MASK: u32 = (BENCH_NR_CPUS as u32) - 1;

pub static mut timing_samples: [[u64; BENCH_NR_SAMPLES]; BENCH_NR_CPUS] =
    [[0; BENCH_NR_SAMPLES]; BENCH_NR_CPUS];
pub static mut timing_idx: [u32; BENCH_NR_CPUS] = [0; BENCH_NR_CPUS];

pub static mut batch_iters: u32 = 0;
pub static mut timing_enabled: u32 = 0;

extern "C" {
    pub fn bpf_get_smp_processor_id() -> u64;
    pub fn bpf_ktime_get_ns() -> u64;
}

#[inline(always)]
pub unsafe fn bench_record_sample(elapsed_ns: u64) {
    let cpu: u32;
    let idx: u32;

    if core::ptr::read_volatile(core::ptr::addr_of!(timing_enabled)) == 0 {
        return;
    }

    cpu = (bpf_get_smp_processor_id() as u32) & BENCH_CPU_MASK;
    idx = timing_idx[cpu as usize];

    if idx >= BENCH_NR_SAMPLES as u32 {
        return;
    }

    timing_samples[cpu as usize][idx as usize] = elapsed_ns;
    timing_idx[cpu as usize] = idx + 1;
}

/*
 * @body:  expression to time; return value (int) stored in __bench_result.
 * @reset: undo body's side-effects so each iteration starts identically.
 *         May reference __bench_result.  Use ({}) for empty reset.
 *
 * Runs batch_iters timed iterations, then one untimed iteration whose
 * return value the macro evaluates to (for validation).
 */
macro_rules! BENCH_BPF_LOOP {
    ($body:expr, $reset:block) => {{
        let __bench_start: u64 = unsafe { bpf_ktime_get_ns() };
        let mut __bench_i: u32;
        let mut __bench_result: i32;

        __bench_i = 0;
        while __bench_i < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(batch_iters)) }
            && can_loop
        {
            __bench_result = $body;
            $reset
            __bench_i += 1;
        }

        unsafe {
            bench_record_sample(bpf_ktime_get_ns().wrapping_sub(__bench_start));
        }

        __bench_result = $body;
        __bench_result
    }};
}

pub(crate) use BENCH_BPF_LOOP;
