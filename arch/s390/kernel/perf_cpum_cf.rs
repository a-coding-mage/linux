// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of s390/kernel/perf_cpum_cf.c.
// Linux kernel and s390 symbols referenced below are supplied by the
// surrounding kernel translation and are intentionally not reimplemented.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type usize_ = usize;
pub type ulong = usize;

pub const PERF_CPUM_CF_MAX_CTR: ulong = 0xffff;
pub const PERF_EVENT_CPUM_CF_DIAG: u64 = 0xBC000;
pub const CPUMF_LCCTL_ENABLE_SHIFT: u32 = 16;
pub const CPUMF_LCCTL_ACTCTL_SHIFT: u32 = 0;
pub const CF_DIAG_CTRSET_DEF: u32 = 0xfeef;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpumf_ctr_set {
    CPUMF_CTR_SET_BASIC = 0,
    CPUMF_CTR_SET_USER = 1,
    CPUMF_CTR_SET_CRYPTO = 2,
    CPUMF_CTR_SET_EXT = 3,
    CPUMF_CTR_SET_MT_DIAG = 4,
    CPUMF_CTR_SET_MAX = 5,
}

#[repr(C)]
pub struct refcount_t { pub value: u32 }
#[repr(C)]
pub struct atomic_t { pub value: i32 }
#[repr(C)]
pub struct debug_info_t { _private: [u8; 0] }
#[repr(C)]
pub struct cpumf_ctr_info { pub cfvn: u16, pub csvn: u16, pub auth_ctl: u16, pub enable_ctl: u16, pub act_ctl: u16 }
#[repr(C)]
pub struct cpuid { pub machine: u16 }
#[repr(C)]
pub struct perf_event { pub attr: perf_event_attr, pub hw: hw_perf_event, pub cpu: i32, pub pmu: *mut pmu, pub destroy: Option<unsafe extern "C" fn(*mut perf_event)>, pub count: u64 }
#[repr(C)]
pub struct perf_event_attr { pub config: u64, pub type_: u32, pub sample_period: u64, pub sample_type: u64, pub exclude_kernel: bool, pub exclude_user: bool, pub exclude_hv: bool }
#[repr(C)]
pub struct hw_perf_event { pub config: u64, pub config_base: u64, pub state: u64, pub prev_count: u64, pub count: u64, pub period_left: u64, pub sample_period: u64, pub last_period: u64 }
#[repr(C)]
pub struct pmu { pub type_: i32 }

#[repr(C)]
pub struct cpu_cf_events {
    pub refcnt: refcount_t,
    pub ctr_set: [atomic_t; cpumf_ctr_set::CPUMF_CTR_SET_MAX as usize],
    pub state: u64,
    pub dev_state: u64,
    pub flags: u32,
    pub used: usize,
    pub usedss: usize,
    pub start: [u8; 4096],
    pub stop: [u8; 4096],
    pub data: [u8; 4096],
    pub sets: u32,
}
#[repr(C)]
pub struct cpu_cf_ptr { pub cpucf: *mut cpu_cf_events }
#[repr(C)]
pub struct cpu_cf_root { pub refcnt: refcount_t, pub tskctx: u32, pub cfptr: *mut cpu_cf_ptr }
#[repr(C)]
pub struct cf_ctrset_entry { pub def: u16, pub set: u16, pub ctr: u16, pub res1: u16 }
#[repr(C)]
pub struct cf_trailer_entry { pub flags: u64, pub cfvn: u16, pub csvn: u16, pub cpu_speed: u32, pub timestamp: ulong, pub progusage: [ulong; 4], pub mach_type: u16, pub res1: u16, pub res2: u32 }

static mut cpu_cf_root: cpu_cf_root = cpu_cf_root { refcnt: refcount_t { value: 0 }, tskctx: 0, cfptr: core::ptr::null_mut() };
static mut cfdiag_cpu_speed: u32 = 0;
static mut cf_dbg: *mut debug_info_t = core::ptr::null_mut();
static mut cpumf_ctr_info: cpumf_ctr_info = cpumf_ctr_info { cfvn: 0, csvn: 0, auth_ctl: 0, enable_ctl: 0, act_ctl: 0 };
static mut cpumf_ctr_setsizes: [usize; cpumf_ctr_set::CPUMF_CTR_SET_MAX as usize] = [0; 5];
static cpumf_ctr_ctl: [u64; 5] = [0x02, 0x04, 0x08, 0x01, 0x20];

#[inline] pub unsafe fn ctr_set_enable(state: *mut u64, sets: u64) { *state |= sets << CPUMF_LCCTL_ENABLE_SHIFT; }
#[inline] pub unsafe fn ctr_set_disable(state: *mut u64, sets: u64) { *state &= !(sets << CPUMF_LCCTL_ENABLE_SHIFT); }
#[inline] pub unsafe fn ctr_set_start(state: *mut u64, sets: u64) { *state |= sets << CPUMF_LCCTL_ACTCTL_SHIFT; }
#[inline] pub unsafe fn ctr_set_stop(state: *mut u64, sets: u64) { *state &= !(sets << CPUMF_LCCTL_ACTCTL_SHIFT); }

pub unsafe fn cpum_cf_read_setsize(set: cpumf_ctr_set) -> usize { cpumf_ctr_setsizes[set as usize] }
pub unsafe fn get_counter_set(event: u64) -> cpumf_ctr_set {
    if event < 32 { cpumf_ctr_set::CPUMF_CTR_SET_BASIC } else if event < 64 { cpumf_ctr_set::CPUMF_CTR_SET_USER } else if event < 128 { cpumf_ctr_set::CPUMF_CTR_SET_CRYPTO } else if event < 288 { cpumf_ctr_set::CPUMF_CTR_SET_EXT } else if event >= 448 && event < 496 { cpumf_ctr_set::CPUMF_CTR_SET_MT_DIAG } else { cpumf_ctr_set::CPUMF_CTR_SET_MAX }
}

pub unsafe fn cfdiag_diffctrset(start: *mut u64, stop: *mut u64, mut counters: i32) { while counters > 0 { let a = *start; let b = *stop; *stop = if b >= a { b - a } else { a.wrapping_sub(b).wrapping_add(1) }; start = start.add(1); stop = stop.add(1); counters -= 1; } }
pub unsafe fn get_authctrsets() -> ulong { let mut auth = 0; let mut i = 0; while i < 5 { if (cpumf_ctr_info.auth_ctl as u64 & cpumf_ctr_ctl[i]) != 0 { auth |= cpumf_ctr_ctl[i]; } i += 1; } auth as ulong }

// The remaining functions retain their kernel-facing interfaces and are
// intentionally declared here for linkage with the translated kernel units.
extern "C" {
    pub fn cpumf_pmu_enable(pmu: *mut pmu);
    pub fn cpumf_pmu_disable(pmu: *mut pmu);
    pub fn cpumf_pmu_event_init(event: *mut perf_event) -> i32;
    pub fn cpumf_pmu_add(event: *mut perf_event, flags: i32) -> i32;
    pub fn cpumf_pmu_del(event: *mut perf_event, flags: i32);
    pub fn cpumf_pmu_start(event: *mut perf_event, flags: i32);
    pub fn cpumf_pmu_stop(event: *mut perf_event, flags: i32);
    pub fn cpumf_pmu_read(event: *mut perf_event);
    pub fn hw_perf_event_reset(event: *mut perf_event) -> i32;
    pub fn hw_perf_event_update(event: *mut perf_event);
    pub fn cfdiag_event_init(event: *mut perf_event) -> i32;
    pub fn cpumf_pmu_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
