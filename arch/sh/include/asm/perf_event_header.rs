/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header __ASM_SH_PERF_EVENT_H.

pub struct hw_perf_event {
    _private: [u8; 0],
}

pub const MAX_HWEVENTS: u32 = 2;

#[repr(C)]
pub struct sh_pmu {
    pub name: *const ::core::ffi::c_char,
    pub num_events: u32,
    pub disable_all: Option<unsafe extern "C" fn()>,
    pub enable_all: Option<unsafe extern "C" fn()>,
    pub enable: Option<unsafe extern "C" fn(*mut hw_perf_event, i32)>,
    pub disable: Option<unsafe extern "C" fn(*mut hw_perf_event, i32)>,
    pub read: Option<unsafe extern "C" fn(i32) -> u64>,
    pub event_map: Option<unsafe extern "C" fn(i32) -> i32>,
    pub max_events: u32,
    pub raw_event_mask: ::core::ffi::c_ulong,
    pub cache_events: *const [[[i32; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX],
}

/* arch/sh/kernel/perf_event.c */
unsafe extern "C" {
    pub fn register_sh_pmu(pmu: *mut sh_pmu) -> i32;
    pub fn reserve_pmc_hardware() -> i32;
    pub fn release_pmc_hardware();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
