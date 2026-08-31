// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Refactored from builtin-top.c, see that files for further copyright notes.
 */

use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
pub struct target {
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub cpu_list: *const c_char,
}

#[repr(C)]
pub struct record_opts {
    pub target: target,
    pub freq: bool,
}

#[repr(C)]
pub struct perf_top {
    pub samples: c_float,
    pub us_samples: c_float,
    pub kernel_samples: c_float,
    pub exact_samples: c_float,
    pub guest_kernel_samples: c_float,
    pub guest_us_samples: c_float,
    pub delay_secs: c_float,
    pub lost: u64,
    pub lost_total: u64,
    pub drop: u64,
    pub drop_total: u64,
    pub record_opts: record_opts,
    pub evlist: *mut evlist,
    pub sym_evsel: *mut evsel,
    pub uid_str: *const c_char,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core_data,
}

#[repr(C)]
pub struct evsel_core_data {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_period: u64,
}

unsafe extern "C" {
    static mut perf_guest: bool;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
}

macro_rules! SNPRINTF {
    ($buf:expr, $size:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        let r = snprintf($buf, $size, $fmt $(, $args)*) as usize;
        if r > $size { $size } else { r }
    }};
}

pub unsafe extern "C" fn perf_top__header_snprintf(
    top: *mut perf_top,
    bf: *mut c_char,
    size: usize,
) -> usize {
    let samples_per_sec: c_float;
    let ksamples_per_sec: c_float;
    let esamples_percent: c_float;
    let opts: *mut record_opts = &mut (*top).record_opts;
    let target: *mut target = &mut (*opts).target;
    let mut ret: usize = 0;
    let nr_cpus: c_int;

    if (*top).samples != 0.0 {
        samples_per_sec = (*top).samples / (*top).delay_secs;
        ksamples_per_sec = (*top).kernel_samples / (*top).delay_secs;
        esamples_percent = (100.0 * (*top).exact_samples) / (*top).samples;
    } else {
        samples_per_sec = 0.0;
        ksamples_per_sec = 0.0;
        esamples_percent = 0.0;
    }

    if !perf_guest {
        let mut ksamples_percent: c_float = 0.0;

        if samples_per_sec != 0.0 {
            ksamples_percent = (100.0 * ksamples_per_sec) / samples_per_sec;
        }
        ret = SNPRINTF!(
            bf,
            size,
            c"   PerfTop:%8.0f irqs/sec  kernel:%4.1f%%  exact: %4.1f%% lost: %llu/%llu drop: %llu/%llu [".as_ptr(),
            samples_per_sec as f64,
            ksamples_percent as f64,
            esamples_percent as f64,
            (*top).lost,
            (*top).lost_total,
            (*top).drop,
            (*top).drop_total
        );
    } else {
        let us_samples_per_sec: c_float = (*top).us_samples / (*top).delay_secs;
        let guest_kernel_samples_per_sec: c_float =
            (*top).guest_kernel_samples / (*top).delay_secs;
        let guest_us_samples_per_sec: c_float = (*top).guest_us_samples / (*top).delay_secs;

        ret = SNPRINTF!(
            bf,
            size,
            c"   PerfTop:%8.0f irqs/sec  kernel:%4.1f%% us:%4.1f%% guest kernel:%4.1f%% guest us:%4.1f%% exact: %4.1f%% [".as_ptr(),
            samples_per_sec as f64,
            (100.0 - (100.0 * ((samples_per_sec - ksamples_per_sec) / samples_per_sec))) as f64,
            (100.0 - (100.0 * ((samples_per_sec - us_samples_per_sec) / samples_per_sec))) as f64,
            (100.0
                - (100.0
                    * ((samples_per_sec - guest_kernel_samples_per_sec) / samples_per_sec)))
                as f64,
            (100.0 - (100.0 * ((samples_per_sec - guest_us_samples_per_sec) / samples_per_sec)))
                as f64,
            esamples_percent as f64
        );
    }

    if evlist__nr_entries((*top).evlist) == 1 {
        let first: *mut evsel = evlist__first((*top).evlist);
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c"%llu%s ".as_ptr(),
            (*first).core.attr.sample_period,
            if (*opts).freq {
                c"Hz".as_ptr()
            } else {
                c"".as_ptr()
            }
        );
    }

    ret += SNPRINTF!(
        bf.add(ret),
        size - ret,
        c"%s".as_ptr(),
        evsel__name((*top).sym_evsel)
    );

    ret += SNPRINTF!(bf.add(ret), size - ret, c"], ".as_ptr());

    if !(*target).pid.is_null() {
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c" (target_pid: %s".as_ptr(),
            (*target).pid
        );
    } else if !(*target).tid.is_null() {
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c" (target_tid: %s".as_ptr(),
            (*target).tid
        );
    } else if !(*top).uid_str.is_null() {
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c" (uid: %s".as_ptr(),
            (*top).uid_str
        );
    } else {
        ret += SNPRINTF!(bf.add(ret), size - ret, c" (all".as_ptr());
    }

    nr_cpus = perf_cpu_map__nr((*evlist__core((*top).evlist)).user_requested_cpus);
    if !(*target).cpu_list.is_null() {
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c", CPU%s: %s)".as_ptr(),
            if nr_cpus > 1 {
                c"s".as_ptr()
            } else {
                c"".as_ptr()
            },
            (*target).cpu_list
        );
    } else if !(*target).tid.is_null() {
        ret += SNPRINTF!(bf.add(ret), size - ret, c")".as_ptr());
    } else {
        ret += SNPRINTF!(
            bf.add(ret),
            size - ret,
            c", %d CPU%s)".as_ptr(),
            nr_cpus,
            if nr_cpus > 1 {
                c"s".as_ptr()
            } else {
                c"".as_ptr()
            }
        );
    }

    perf_top__reset_sample_counters(top);
    ret
}

pub unsafe extern "C" fn perf_top__reset_sample_counters(top: *mut perf_top) {
    (*top).drop = 0;
    (*top).lost = (*top).drop;
    (*top).guest_us_samples = (*top).lost as c_float;
    (*top).guest_kernel_samples = (*top).guest_us_samples;
    (*top).exact_samples = (*top).guest_kernel_samples;
    (*top).kernel_samples = (*top).exact_samples;
    (*top).us_samples = (*top).kernel_samples;
    (*top).samples = (*top).us_samples;
}
