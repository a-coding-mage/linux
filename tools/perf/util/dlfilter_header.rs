/* SPDX-License-Identifier: GPL-2.0 */
/*
 * dlfilter.h: Interface to perf script --dlfilter shared object
 * Copyright (c) 2021, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

/* Forward declarations from included perf headers. */
#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

/* Original C declaration is: union perf_event; */
#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_dlfilter_fns {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_dlfilter_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_dlfilter_al {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlfilter {
    pub file: *mut c_char,
    pub handle: *mut c_void,
    pub data: *mut c_void,
    pub session: *mut perf_session,
    pub ctx_valid: bool,
    pub in_start: bool,
    pub in_stop: bool,
    pub dlargc: c_int,
    pub dlargv: *mut *mut c_char,

    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub evsel: *mut evsel,
    pub machine: *mut machine,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
    pub d_sample: *mut perf_dlfilter_sample,
    pub d_ip_al: *mut perf_dlfilter_al,
    pub d_addr_al: *mut perf_dlfilter_al,

    pub start: Option<unsafe extern "C" fn(data: *mut *mut c_void, ctx: *mut c_void) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(data: *mut c_void, ctx: *mut c_void) -> c_int>,

    pub filter_event: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            sample: *const perf_dlfilter_sample,
            ctx: *mut c_void,
        ) -> c_int,
    >,
    pub filter_event_early: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            sample: *const perf_dlfilter_sample,
            ctx: *mut c_void,
        ) -> c_int,
    >,

    pub fns: *mut perf_dlfilter_fns,
}

unsafe extern "C" {
    pub fn dlfilter__new(
        file: *const c_char,
        dlargc: c_int,
        dlargv: *mut *mut c_char,
    ) -> *mut dlfilter;

    pub fn dlfilter__start(d: *mut dlfilter, session: *mut perf_session) -> c_int;

    pub fn dlfilter__do_filter_event(
        d: *mut dlfilter,
        event: *mut perf_event,
        sample: *mut perf_sample,
        evsel: *mut evsel,
        machine: *mut machine,
        al: *mut addr_location,
        addr_al: *mut addr_location,
        early: bool,
    ) -> c_int;

    pub fn dlfilter__cleanup(d: *mut dlfilter);

    pub fn list_available_dlfilters(
        opt: *const option,
        s: *const c_char,
        unset: c_int,
    ) -> c_int;

    pub fn get_filter_desc(
        dirname: *const c_char,
        name: *const c_char,
        desc: *mut *mut c_char,
        long_desc: *mut *mut c_char,
    ) -> bool;
}

pub unsafe fn dlfilter__filter_event(
    d: *mut dlfilter,
    event: *mut perf_event,
    sample: *mut perf_sample,
    evsel: *mut evsel,
    machine: *mut machine,
    al: *mut addr_location,
    addr_al: *mut addr_location,
) -> c_int {
    if d.is_null() || unsafe { (*d).filter_event.is_none() } {
        return 0;
    }
    unsafe { dlfilter__do_filter_event(d, event, sample, evsel, machine, al, addr_al, false) }
}

pub unsafe fn dlfilter__filter_event_early(
    d: *mut dlfilter,
    event: *mut perf_event,
    sample: *mut perf_sample,
    evsel: *mut evsel,
    machine: *mut machine,
    al: *mut addr_location,
    addr_al: *mut addr_location,
) -> c_int {
    if d.is_null() || unsafe { (*d).filter_event_early.is_none() } {
        return 0;
    }
    unsafe { dlfilter__do_filter_event(d, event, sample, evsel, machine, al, addr_al, true) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
