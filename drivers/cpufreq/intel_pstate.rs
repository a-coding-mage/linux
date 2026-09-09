// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pstate.rs: Native P state management for Intel processors
 *
 * Source-level Rust translation of intel_pstate.c.  Kernel-provided types,
 * constants, macros, and functions remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const INTEL_PSTATE_SAMPLING_INTERVAL: u64 = 10 * NSEC_PER_MSEC;
const INTEL_CPUFREQ_TRANSITION_LATENCY: u32 = 20000;
const INTEL_CPUFREQ_TRANSITION_DELAY_HWP: u32 = 5000;
const INTEL_CPUFREQ_TRANSITION_DELAY: u32 = 500;

const FRAC_BITS: u32 = 8;
const EXT_BITS: u32 = 6;
const EXT_FRAC_BITS: u32 = EXT_BITS + FRAC_BITS;
const ONE_EIGHTH_FP: i64 = 1i64 << (FRAC_BITS - 3);

#[inline]
fn int_tofp(x: i64) -> i64 { x << FRAC_BITS }
#[inline]
fn fp_toint(x: i64) -> i64 { x >> FRAC_BITS }
#[inline]
fn fp_ext_toint(x: i64) -> i64 { x >> EXT_FRAC_BITS }
#[inline]
fn int_ext_tofp(x: i64) -> i64 { x << EXT_FRAC_BITS }

#[inline]
fn mul_fp(x: i32, y: i32) -> i32 { (((x as i64) * (y as i64)) >> FRAC_BITS) as i32 }
#[inline]
fn div_fp(x: i64, y: i64) -> i32 { ((x << FRAC_BITS) / y) as i32 }
#[inline]
fn ceiling_fp(x: i32) -> i32 {
    let mut ret = fp_toint(x as i64) as i32;
    if x & ((1i32 << FRAC_BITS) - 1) != 0 { ret += 1; }
    ret
}
#[inline]
fn mul_ext_fp(x: u64, y: u64) -> u64 { (x * y) >> EXT_FRAC_BITS }
#[inline]
fn div_ext_fp(x: u64, y: u64) -> u64 { (x << EXT_FRAC_BITS) / y }

#[repr(C)]
pub struct sample {
    pub core_avg_perf: i32,
    pub busy_scaled: i32,
    pub aperf: u64,
    pub mperf: u64,
    pub tsc: u64,
    pub time: u64,
}

#[repr(C)]
pub struct pstate_data {
    pub current_pstate: i32,
    pub min_pstate: i32,
    pub max_pstate: i32,
    pub max_pstate_physical: i32,
    pub perf_ctl_scaling: i32,
    pub scaling: i32,
    pub turbo_pstate: i32,
    pub min_freq: u32,
    pub max_freq: u32,
    pub turbo_freq: u32,
}

#[repr(C)]
pub struct vid_data { pub min: i32, pub max: i32, pub turbo: i32, pub ratio: i32 }

#[repr(C)]
pub struct global_params {
    pub no_turbo: bool,
    pub turbo_disabled: bool,
    pub max_perf_pct: i32,
    pub min_perf_pct: i32,
}

// The remainder of the implementation depends on Linux kernel declarations
// supplied by the surrounding translation unit.  Preserve the source-level
// interfaces and ordering here; these symbols are intentionally external.
extern "C" {
    static mut all_cpu_data: *mut *mut cpudata;
}

#[repr(C)]
pub struct cpudata {
    pub cpu: i32,
    pub policy: u32,
    pub pstate: pstate_data,
    pub vid: vid_data,
    pub last_update: u64,
    pub last_sample_time: u64,
    pub aperf_mperf_shift: u64,
    pub prev_aperf: u64,
    pub prev_mperf: u64,
    pub prev_tsc: u64,
    pub sample: sample,
    pub min_perf_ratio: i32,
    pub max_perf_ratio: i32,
    pub iowait_boost: u32,
    pub epp_powersave: i16,
    pub epp_policy: i16,
    pub epp_default: i16,
    pub epp_cached: i16,
    pub hwp_req_cached: u64,
    pub hwp_cap_cached: u64,
    pub last_io_update: u64,
    pub capacity_perf: u32,
    pub sched_flags: u32,
    pub hwp_boost_min: u32,
    pub suspended: bool,
}

pub static mut global: global_params = global_params {
    no_turbo: false, turbo_disabled: false, max_perf_pct: 0, min_perf_pct: 0,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
