// SPDX-License-Identifier: GPL-2.0-only
/*
 * auxtrace.c: AUX area tracing support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int};

const EINVAL: c_int = 22;
const INTEL_PT_PMU_NAME: *const c_char = b"intel_pt\0".as_ptr() as *const c_char;
const INTEL_BTS_PMU_NAME: *const c_char = b"intel_bts\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct auxtrace_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub type_: u32,
}

#[repr(C)]
pub struct perf_cpu {
    /* External definition supplied by perf CPU map support. */
    _private: c_int,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist_core {
    pub all_cpus: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn perf_cpu_map__min(cpus: *mut core::ffi::c_void) -> perf_cpu;
    fn get_cpuid(buffer: *mut c_char, sz: usize, cpu: perf_cpu) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn intel_pt_recording_init(err: *mut c_int) -> *mut auxtrace_record;
    fn intel_bts_recording_init(err: *mut c_int) -> *mut auxtrace_record;
    fn pr_err(fmt: *const c_char, ...);

    /*
     * Rust translation of evlist__for_each_entry(evlist, evsel) depends on
     * the external evlist iteration implementation.
     */
    fn evlist__first_entry(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next_entry(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

unsafe fn auxtrace_record__init_intel(
    evlist: *mut evlist,
    err: *mut c_int,
) -> *mut auxtrace_record {
    let intel_pt_pmu: *mut perf_pmu;
    let intel_bts_pmu: *mut perf_pmu;
    let mut evsel: *mut evsel;
    let mut found_pt = false;
    let mut found_bts = false;

    intel_pt_pmu = unsafe { perf_pmus__find(INTEL_PT_PMU_NAME) };
    intel_bts_pmu = unsafe { perf_pmus__find(INTEL_BTS_PMU_NAME) };

    evsel = unsafe { evlist__first_entry(evlist) };
    while !evsel.is_null() {
        if !intel_pt_pmu.is_null()
            && unsafe { (*evsel).core.attr.type_ == (*intel_pt_pmu).type_ }
        {
            found_pt = true;
        }
        if !intel_bts_pmu.is_null()
            && unsafe { (*evsel).core.attr.type_ == (*intel_bts_pmu).type_ }
        {
            found_bts = true;
        }
        evsel = unsafe { evlist__next_entry(evlist, evsel) };
    }

    if found_pt && found_bts {
        unsafe {
            pr_err(b"intel_pt and intel_bts may not be used together\n\0".as_ptr() as *const c_char);
            *err = -EINVAL;
        }
        return core::ptr::null_mut();
    }

    if found_pt {
        return unsafe { intel_pt_recording_init(err) };
    }

    if found_bts {
        return unsafe { intel_bts_recording_init(err) };
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn auxtrace_record__init(
    evlist: *mut evlist,
    err: *mut c_int,
) -> *mut auxtrace_record {
    let mut buffer = [0 as c_char; 64];
    let cpu: perf_cpu =
        unsafe { perf_cpu_map__min((*evlist__core(evlist)).all_cpus) };
    let ret: c_int;

    unsafe {
        *err = 0;
    }

    ret = unsafe { get_cpuid(buffer.as_mut_ptr(), buffer.len(), cpu) };
    if ret != 0 {
        unsafe {
            *err = ret;
        }
        return core::ptr::null_mut();
    }

    if unsafe { strncmp(buffer.as_ptr(), b"GenuineIntel,\0".as_ptr() as *const c_char, 13) } == 0 {
        return unsafe { auxtrace_record__init_intel(evlist, err) };
    }

    core::ptr::null_mut()
}
