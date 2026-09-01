// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pt.c: Intel Processor Trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u64 = u64;
type u32 = u32;
type __u64 = u64;
type size_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EOPNOTSUPP: c_int = 95;
const SIZE_MAX: c_ulonglong = usize::MAX as c_ulonglong;
const UINT_MAX: c_uint = c_uint::MAX;
const INTEL_PT_PSB_PERIOD_NEAR: size_t = 256;

const INTEL_PT_PMU_NAME: *const c_char = b"intel_pt\0".as_ptr() as *const c_char;

const PERF_AUXTRACE_INTEL_PT: u32 = 0;
const INTEL_PT_AUXTRACE_PRIV_MAX: usize = 0;
const INTEL_PT_PMU_TYPE: usize = 0;
const INTEL_PT_TIME_SHIFT: usize = 1;
const INTEL_PT_TIME_MULT: usize = 2;
const INTEL_PT_TIME_ZERO: usize = 3;
const INTEL_PT_CAP_USER_TIME_ZERO: usize = 4;
const INTEL_PT_TSC_BIT: usize = 5;
const INTEL_PT_NORETCOMP_BIT: usize = 6;
const INTEL_PT_HAVE_SCHED_SWITCH: usize = 7;
const INTEL_PT_SNAPSHOT_MODE: usize = 8;
const INTEL_PT_PER_CPU_MMAPS: usize = 9;
const INTEL_PT_MTC_BIT: usize = 10;
const INTEL_PT_MTC_FREQ_BITS: usize = 11;
const INTEL_PT_TSC_CTC_N: usize = 12;
const INTEL_PT_TSC_CTC_D: usize = 13;
const INTEL_PT_CYC_BIT: usize = 14;
const INTEL_PT_MAX_NONTURBO_RATIO: usize = 15;
const INTEL_PT_FILTER_STR_LEN: usize = 16;
const TID: c_int = 0;
const TIME: c_int = 1;
const CPU: c_int = 2;
const BRANCH_STACK: c_int = 3;

const fn KiB(x: size_t) -> size_t {
    x * 1024
}

const fn MiB(x: size_t) -> size_t {
    x * 1024 * 1024
}

#[repr(C)]
pub struct parse_events_terms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub type_: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_event_attr {
    pub size: u32,
    pub config: u64,
    pub type_: u32,
    pub freq: u64,
    pub sample_period: u64,
    pub exclude_guest: bool,
    pub aux_sample_size: size_t,
    pub aux_output: bool,
    pub aux_watermark: u32,
    pub context_switch: u64,
    pub exclude_kernel: bool,
    pub exclude_user: bool,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub filter: *const c_char,
    pub immediate: bool,
    pub no_aux_samples: bool,
    pub needs_auxtrace_mmap: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_uint,
    pub user_requested_cpus: *const perf_cpu_map,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    pub auxtrace_snapshot_mode: bool,
    pub auxtrace_snapshot_size: size_t,
    pub auxtrace_sample_mode: bool,
    pub full_auxtrace: bool,
    pub auxtrace_mmap_pages: size_t,
    pub mmap_pages: c_uint,
    pub use_clockid: bool,
    pub record_switch_events: bool,
    pub text_poke: bool,
    pub target: target,
}

#[repr(C)]
pub struct target {
    pub cpu_list: *const c_char,
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub type_: u32,
    pub priv_: [__u64; 64],
}

#[repr(C)]
pub struct perf_event_mmap_page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap_core {
    pub base: *mut perf_event_mmap_page,
}

#[repr(C)]
pub struct mmap {
    pub core: mmap_core,
}

#[repr(C)]
pub struct perf_tsc_conversion {
    pub time_mult: u32,
    pub time_shift: u16,
    pub time_zero: u64,
}

#[repr(C)]
pub struct auxtrace_mmap {
    pub len: size_t,
    pub mask: u64,
}

#[repr(C)]
pub struct auxtrace_record {
    pub recording_options:
        Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int>,
    pub info_priv_size: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t>,
    pub info_fill: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            *mut perf_session,
            *mut perf_record_auxtrace_info,
            size_t,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    pub snapshot_start: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub snapshot_finish: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub find_snapshot: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            c_int,
            *mut auxtrace_mmap,
            *mut u8,
            *mut u64,
            *mut u64,
        ) -> c_int,
    >,
    pub parse_snapshot_options:
        Option<unsafe extern "C" fn(*mut auxtrace_record, *mut record_opts, *const c_char) -> c_int>,
    pub reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    pub read_finish: Option<unsafe extern "C" fn()>,
    pub default_aux_sample_size: size_t,
}

#[repr(C)]
pub struct intel_pt_snapshot_ref {
    pub ref_buf: *mut c_void,
    pub ref_offset: size_t,
    pub wrapped: bool,
}

#[repr(C)]
pub struct intel_pt_recording {
    pub itr: auxtrace_record,
    pub intel_pt_pmu: *mut perf_pmu,
    pub have_sched_switch: c_int,
    pub evlist: *mut evlist,
    pub all_switch_events: bool,
    pub snapshot_mode: bool,
    pub snapshot_init_done: bool,
    pub snapshot_size: size_t,
    pub snapshot_ref_buf_size: size_t,
    pub snapshot_ref_cnt: c_int,
    pub snapshot_refs: *mut intel_pt_snapshot_ref,
    pub priv_size: size_t,
}

unsafe extern "C" {
    static page_size: c_uint;
    static mut errno: c_int;

    fn parse_events_terms__init(terms: *mut parse_events_terms);
    fn parse_events_terms(terms: *mut parse_events_terms, str_: *const c_char) -> c_int;
    fn parse_events_terms__exit(terms: *mut parse_events_terms);
    fn perf_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        zero: bool,
        apply_hardcoded: bool,
        err: *mut c_void,
    ) -> c_int;
    fn perf_pmu__format_bits(pmu: *const perf_pmu, name: *const c_char) -> u64;
    fn perf_pmu__scan_file(pmu: *mut perf_pmu, name: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn perf_pmu__event_source_devices_fd() -> c_int;
    fn perf_pmu__scan_file_at(
        pmu: *const perf_pmu,
        dirfd: c_int,
        name: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn cpuid(op: c_uint, count: c_uint, eax: *mut c_uint, ebx: *mut c_uint, ecx: *mut c_uint, edx: *mut c_uint);
    fn perf_read_tsc_conversion(pc: *mut perf_event_mmap_page, tc: *mut perf_tsc_conversion) -> c_int;
    fn ui__warning(fmt: *const c_char, ...);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *const perf_cpu_map) -> bool;
    fn evlist__can_select_event(evlist: *mut evlist, name: *const c_char) -> bool;
    fn evlist__add_sched_switch(evlist: *mut evlist, system_wide: bool) -> *mut evsel;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn sysfs__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn perf_event_paranoid_check(paranoid: c_int) -> bool;
    fn evsel__set_config_if_unset(evsel: *mut evsel, name: *const c_char, val: u64);
    fn is_power_of_2(n: size_t) -> bool;
    fn round_up(x: size_t, y: c_uint) -> size_t;
    fn roundup_pow_of_two(x: size_t) -> size_t;
    fn perf_can_record_switch_events() -> bool;
    fn perf_can_record_cpu_wide() -> bool;
    fn record_opts__no_switch_events(opts: *mut record_opts) -> bool;
    fn target__none(target: *const target) -> bool;
    fn target__has_task(target: *const target) -> bool;
    fn evlist__add_dummy_on_all_cpus(evlist: *mut evlist) -> *mut evsel;
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn perf_can_record_text_poke_events() -> bool;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__add_aux_dummy(evlist: *mut evlist, system_wide: bool) -> *mut evsel;
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__disable(evsel: *mut evsel) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn rdtsc() -> u64;
    fn perf_config_bool(var: *const c_char, value: *const c_char) -> bool;
    fn perf_config(func: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void);
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn auxtrace_record__read_finish();
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
}

unsafe fn container_of_intel_pt_recording(itr: *mut auxtrace_record) -> *mut intel_pt_recording {
    (itr as *mut u8).sub(offset_of!(intel_pt_recording, itr)) as *mut intel_pt_recording
}

unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(evlist: *mut evlist, mut f: F) {
    let mut evsel = evlist__first(evlist);
    while !evsel.is_null() {
        f(evsel);
        evsel = evsel__next(evsel);
    }
}

unsafe extern "C" fn intel_pt_parse_terms_with_default(
    pmu: *const perf_pmu,
    str_: *const c_char,
    config: *mut u64,
) -> c_int {
    let mut terms: parse_events_terms = core::mem::zeroed();
    let mut attr = perf_event_attr {
        size: 0,
        config: 0,
        type_: 0,
        freq: 0,
        sample_period: 0,
        exclude_guest: false,
        aux_sample_size: 0,
        aux_output: false,
        aux_watermark: 0,
        context_switch: 0,
        exclude_kernel: false,
        exclude_user: false,
    };
    let mut err: c_int;

    parse_events_terms__init(&mut terms);
    err = parse_events_terms(&mut terms, str_);
    if err != 0 {
        parse_events_terms__exit(&mut terms);
        return err;
    }

    attr.config = *config;
    err = perf_pmu__config_terms(pmu, &mut attr, &mut terms, true, false, ptr::null_mut());
    if err != 0 {
        parse_events_terms__exit(&mut terms);
        return err;
    }

    *config = attr.config;
    parse_events_terms__exit(&mut terms);
    err
}

unsafe extern "C" fn intel_pt_parse_terms(pmu: *const perf_pmu, str_: *const c_char, config: *mut u64) -> c_int {
    *config = 0;
    intel_pt_parse_terms_with_default(pmu, str_, config)
}

unsafe extern "C" fn intel_pt_masked_bits(mut mask: u64, mut bits: u64) -> u64 {
    let top_bit: u64 = 1u64 << 63;
    let mut res: u64 = 0;

    for _i in 0..64 {
        if mask & top_bit != 0 {
            res <<= 1;
            if bits & top_bit != 0 {
                res |= 1;
            }
        }
        mask <<= 1;
        bits <<= 1;
    }

    res
}

unsafe extern "C" fn intel_pt_read_config(
    intel_pt_pmu: *mut perf_pmu,
    str_: *const c_char,
    evlist: *mut evlist,
    res: *mut u64,
) -> c_int {
    *res = 0;

    let mask = perf_pmu__format_bits(intel_pt_pmu, str_);
    if mask == 0 {
        return -EINVAL;
    }

    let mut ret = -EINVAL;
    evlist_for_each_entry(evlist, |evsel| unsafe {
        if (*evsel).core.attr.type_ == (*intel_pt_pmu).type_ && ret == -EINVAL {
            *res = intel_pt_masked_bits(mask, (*evsel).core.attr.config);
            ret = 0;
        }
    });

    ret
}

unsafe extern "C" fn intel_pt_psb_period(intel_pt_pmu: *mut perf_pmu, evlist: *mut evlist) -> size_t {
    let mut val: u64 = 0;
    let mut topa_multiple_entries: c_int = 0;

    if perf_pmu__scan_file(
        intel_pt_pmu,
        b"caps/topa_multiple_entries\0".as_ptr() as *const c_char,
        b"%d\0".as_ptr() as *const c_char,
        &mut topa_multiple_entries,
    ) != 1
    {
        topa_multiple_entries = 0;
    }

    /*
     * Use caps/topa_multiple_entries to indicate early hardware that had
     * extra frequent PSBs.
     */
    let psb_period: size_t = if topa_multiple_entries == 0 {
        256
    } else {
        let err = intel_pt_read_config(
            intel_pt_pmu,
            b"psb_period\0".as_ptr() as *const c_char,
            evlist,
            &mut val,
        );
        if err != 0 {
            val = 0;
        }
        1usize << (val + 11)
    };

    pr_debug2(
        b"%s psb_period %zu\n\0".as_ptr() as *const c_char,
        (*intel_pt_pmu).name,
        psb_period,
    );
    psb_period
}

unsafe extern "C" fn intel_pt_pick_bit(mut bits: c_int, target: c_int) -> c_int {
    let mut pos = 0;
    let mut pick = -1;

    while bits != 0 {
        if bits & 1 != 0 {
            if pos <= target || pick < 0 {
                pick = pos;
            }
            if pos >= target {
                break;
            }
        }
        bits >>= 1;
        pos += 1;
    }

    pick
}

unsafe extern "C" fn intel_pt_default_config(intel_pt_pmu: *const perf_pmu) -> u64 {
    let mut buf = [0 as c_char; 256];
    let mut mtc: c_int = 0;
    let mut mtc_periods: c_int = 0;
    let mut mtc_period: c_int;
    let mut psb_cyc: c_int = 0;
    let mut psb_periods: c_int = 0;
    let mut psb_period: c_int;
    let mut pos: c_int = 0;
    let mut config: u64 = 0;
    let mut c: c_char = 0;
    let dirfd: c_int = perf_pmu__event_source_devices_fd();

    pos += scnprintf(buf.as_mut_ptr().add(pos as usize), buf.len() - pos as usize, b"tsc\0".as_ptr() as *const c_char);

    if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"caps/mtc\0".as_ptr() as *const c_char, b"%d\0".as_ptr() as *const c_char, &mut mtc) != 1 {
        mtc = 1;
    }

    if mtc != 0 {
        if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"caps/mtc_periods\0".as_ptr() as *const c_char, b"%x\0".as_ptr() as *const c_char, &mut mtc_periods) != 1 {
            mtc_periods = 0;
        }
        if mtc_periods != 0 {
            mtc_period = intel_pt_pick_bit(mtc_periods, 3);
            pos += scnprintf(
                buf.as_mut_ptr().add(pos as usize),
                buf.len() - pos as usize,
                b",mtc,mtc_period=%d\0".as_ptr() as *const c_char,
                mtc_period,
            );
        }
    }

    if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"caps/psb_cyc\0".as_ptr() as *const c_char, b"%d\0".as_ptr() as *const c_char, &mut psb_cyc) != 1 {
        psb_cyc = 1;
    }

    if psb_cyc != 0 && mtc_periods != 0 {
        if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"caps/psb_periods\0".as_ptr() as *const c_char, b"%x\0".as_ptr() as *const c_char, &mut psb_periods) != 1 {
            psb_periods = 0;
        }
        if psb_periods != 0 {
            psb_period = intel_pt_pick_bit(psb_periods, 3);
            pos += scnprintf(
                buf.as_mut_ptr().add(pos as usize),
                buf.len() - pos as usize,
                b",psb_period=%d\0".as_ptr() as *const c_char,
                psb_period,
            );
        }
    }

    if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"format/pt\0".as_ptr() as *const c_char, b"%c\0".as_ptr() as *const c_char, &mut c) == 1
        && perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"format/branch\0".as_ptr() as *const c_char, b"%c\0".as_ptr() as *const c_char, &mut c) == 1
    {
        pos += scnprintf(buf.as_mut_ptr().add(pos as usize), buf.len() - pos as usize, b",pt,branch\0".as_ptr() as *const c_char);
    }

    pr_debug2(
        b"%s default config: %s\n\0".as_ptr() as *const c_char,
        (*intel_pt_pmu).name,
        buf.as_ptr(),
    );

    intel_pt_parse_terms(intel_pt_pmu, buf.as_ptr(), &mut config);

    close(dirfd);
    config
}

unsafe extern "C" fn intel_pt_parse_snapshot_options(
    itr: *mut auxtrace_record,
    opts: *mut record_opts,
    str_: *const c_char,
) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let mut snapshot_size: c_ulonglong = 0;
    let mut endptr: *mut c_char = ptr::null_mut();

    if !str_.is_null() {
        snapshot_size = strtoull(str_, &mut endptr, 0);
        if *endptr != 0 || snapshot_size > SIZE_MAX {
            return -1;
        }
    }

    (*opts).auxtrace_snapshot_mode = true;
    (*opts).auxtrace_snapshot_size = snapshot_size as size_t;

    (*ptr).snapshot_size = snapshot_size as size_t;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_pmu_default_config(intel_pt_pmu: *const perf_pmu, attr: *mut perf_event_attr) {
    static mut CONFIG: u64 = 0;
    static mut INITIALIZED: bool = false;

    if !INITIALIZED {
        CONFIG = intel_pt_default_config(intel_pt_pmu);
        INITIALIZED = true;
    }
    (*attr).config = CONFIG;
}

unsafe extern "C" fn intel_pt_find_filter(evlist: *mut evlist, intel_pt_pmu: *mut perf_pmu) -> *const c_char {
    let mut ret: *const c_char = ptr::null();
    evlist_for_each_entry(evlist, |evsel| unsafe {
        if ret.is_null() && (*evsel).core.attr.type_ == (*intel_pt_pmu).type_ {
            ret = (*evsel).filter;
        }
    });
    ret
}

unsafe extern "C" fn intel_pt_filter_bytes(filter: *const c_char) -> size_t {
    let len = if !filter.is_null() { strlen(filter) } else { 0 };

    if len != 0 {
        (len + 1 + 7) & !7
    } else {
        0
    }
}

unsafe extern "C" fn intel_pt_info_priv_size(itr: *mut auxtrace_record, evlist: *mut evlist) -> size_t {
    let ptr = container_of_intel_pt_recording(itr);
    let filter = intel_pt_find_filter(evlist, (*ptr).intel_pt_pmu);

    (*ptr).priv_size = (INTEL_PT_AUXTRACE_PRIV_MAX * size_of::<u64>()) + intel_pt_filter_bytes(filter);
    (*ptr).priv_size += size_of::<u64>(); /* Cap Event Trace */

    (*ptr).priv_size
}

unsafe extern "C" fn intel_pt_tsc_ctc_ratio(n: *mut u32, d: *mut u32) {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;

    cpuid(0x15, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);
    *n = ebx;
    *d = eax;
}

unsafe extern "C" fn intel_pt_info_fill(
    itr: *mut auxtrace_record,
    session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    priv_size: size_t,
) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let intel_pt_pmu = (*ptr).intel_pt_pmu;
    let mut tc = perf_tsc_conversion {
        time_mult: 0,
        time_shift: 0,
        time_zero: 0,
    };
    let mut cap_user_time_zero = false;
    let mut tsc_bit: u64 = 0;
    let mut mtc_bit: u64 = 0;
    let mut cyc_bit: u64 = 0;
    let mut noretcomp_bit: u64 = 0;
    let mtc_freq_bits: u64;
    let mut tsc_ctc_ratio_n: u32 = 0;
    let mut tsc_ctc_ratio_d: u32 = 0;
    let mut max_non_turbo_ratio: c_ulong = 0;
    let filter_str_len: size_t;
    let filter: *const c_char;
    let mut event_trace: c_int = 0;
    let mut info: *mut __u64;

    if priv_size != (*ptr).priv_size {
        return -EINVAL;
    }

    intel_pt_parse_terms(intel_pt_pmu, b"tsc\0".as_ptr() as *const c_char, &mut tsc_bit);
    intel_pt_parse_terms(intel_pt_pmu, b"noretcomp\0".as_ptr() as *const c_char, &mut noretcomp_bit);
    intel_pt_parse_terms(intel_pt_pmu, b"mtc\0".as_ptr() as *const c_char, &mut mtc_bit);
    mtc_freq_bits = perf_pmu__format_bits(intel_pt_pmu, b"mtc_period\0".as_ptr() as *const c_char);
    intel_pt_parse_terms(intel_pt_pmu, b"cyc\0".as_ptr() as *const c_char, &mut cyc_bit);

    intel_pt_tsc_ctc_ratio(&mut tsc_ctc_ratio_n, &mut tsc_ctc_ratio_d);

    if perf_pmu__scan_file(intel_pt_pmu, b"max_nonturbo_ratio\0".as_ptr() as *const c_char, b"%lu\0".as_ptr() as *const c_char, &mut max_non_turbo_ratio) != 1 {
        max_non_turbo_ratio = 0;
    }
    if perf_pmu__scan_file(intel_pt_pmu, b"caps/event_trace\0".as_ptr() as *const c_char, b"%d\0".as_ptr() as *const c_char, &mut event_trace) != 1 {
        event_trace = 0;
    }

    filter = intel_pt_find_filter((*session).evlist, (*ptr).intel_pt_pmu);
    filter_str_len = if !filter.is_null() { strlen(filter) } else { 0 };

    if (*evlist__core((*session).evlist)).nr_mmaps == 0 {
        return -EINVAL;
    }

    let pc = (*evlist__mmap((*session).evlist)).core.base;
    if !pc.is_null() {
        let err = perf_read_tsc_conversion(pc, &mut tc);
        if err != 0 {
            if err != -EOPNOTSUPP {
                return err;
            }
        } else {
            cap_user_time_zero = tc.time_mult != 0;
        }
        if !cap_user_time_zero {
            ui__warning(b"Intel Processor Trace: TSC not available\n\0".as_ptr() as *const c_char);
        }
    }

    let per_cpu_mmaps = !perf_cpu_map__is_any_cpu_or_is_empty((*evlist__core((*session).evlist)).user_requested_cpus);

    (*auxtrace_info).type_ = PERF_AUXTRACE_INTEL_PT;
    (*auxtrace_info).priv_[INTEL_PT_PMU_TYPE] = (*intel_pt_pmu).type_ as u64;
    (*auxtrace_info).priv_[INTEL_PT_TIME_SHIFT] = tc.time_shift as u64;
    (*auxtrace_info).priv_[INTEL_PT_TIME_MULT] = tc.time_mult as u64;
    (*auxtrace_info).priv_[INTEL_PT_TIME_ZERO] = tc.time_zero;
    (*auxtrace_info).priv_[INTEL_PT_CAP_USER_TIME_ZERO] = cap_user_time_zero as u64;
    (*auxtrace_info).priv_[INTEL_PT_TSC_BIT] = tsc_bit;
    (*auxtrace_info).priv_[INTEL_PT_NORETCOMP_BIT] = noretcomp_bit;
    (*auxtrace_info).priv_[INTEL_PT_HAVE_SCHED_SWITCH] = (*ptr).have_sched_switch as u64;
    (*auxtrace_info).priv_[INTEL_PT_SNAPSHOT_MODE] = (*ptr).snapshot_mode as u64;
    (*auxtrace_info).priv_[INTEL_PT_PER_CPU_MMAPS] = per_cpu_mmaps as u64;
    (*auxtrace_info).priv_[INTEL_PT_MTC_BIT] = mtc_bit;
    (*auxtrace_info).priv_[INTEL_PT_MTC_FREQ_BITS] = mtc_freq_bits;
    (*auxtrace_info).priv_[INTEL_PT_TSC_CTC_N] = tsc_ctc_ratio_n as u64;
    (*auxtrace_info).priv_[INTEL_PT_TSC_CTC_D] = tsc_ctc_ratio_d as u64;
    (*auxtrace_info).priv_[INTEL_PT_CYC_BIT] = cyc_bit;
    (*auxtrace_info).priv_[INTEL_PT_MAX_NONTURBO_RATIO] = max_non_turbo_ratio as u64;
    (*auxtrace_info).priv_[INTEL_PT_FILTER_STR_LEN] = filter_str_len as u64;

    info = (*auxtrace_info).priv_.as_mut_ptr().add(INTEL_PT_FILTER_STR_LEN + 1);

    if filter_str_len != 0 {
        let len = intel_pt_filter_bytes(filter);

        strncpy(info as *mut c_char, filter, len);
        info = info.add(len >> 3);
    }

    *info = event_trace as u64;

    0
}

/* Translated from #ifdef HAVE_LIBTRACEEVENT. */
#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn intel_pt_track_switches(evlist: *mut evlist) -> c_int {
    let sched_switch = b"sched:sched_switch\0".as_ptr() as *const c_char;
    let evsel: *mut evsel;
    let err: c_int;

    if !evlist__can_select_event(evlist, sched_switch) {
        return -EPERM;
    }

    evsel = evlist__add_sched_switch(evlist, true);
    if IS_ERR(evsel as *const c_void) {
        err = PTR_ERR(evsel as *const c_void);
        pr_debug2(
            b"%s: failed to create %s, error = %d\n\0".as_ptr() as *const c_char,
            b"intel_pt_track_switches\0".as_ptr() as *const c_char,
            sched_switch,
            err,
        );
        return err;
    }

    (*evsel).immediate = true;

    0
}

unsafe extern "C" fn intel_pt_exclude_guest() -> bool {
    let mut pt_mode: c_int = 0;

    if sysfs__read_int(
        b"module/kvm_intel/parameters/pt_mode\0".as_ptr() as *const c_char,
        &mut pt_mode,
    ) != 0
    {
        pt_mode = 0;
    }

    pt_mode == 1
}

unsafe extern "C" fn intel_pt_valid_str(str_: *mut c_char, len: size_t, mut valid: u64) {
    let mut last: c_uint = 0;
    let mut state: c_uint = 1;
    let mut p: c_int = 0;

    *str_ = 0;

    for val in 0..=64u32 {
        if valid & 1 != 0 {
            last = val;
            match state {
                0 => {
                    p += scnprintf(str_.add(p as usize), len - p as usize, b",\0".as_ptr() as *const c_char);
                    /* Fall through */
                    p += scnprintf(str_.add(p as usize), len - p as usize, b"%u\0".as_ptr() as *const c_char, val);
                    state = 2;
                }
                1 => {
                    p += scnprintf(str_.add(p as usize), len - p as usize, b"%u\0".as_ptr() as *const c_char, val);
                    state = 2;
                }
                2 => state = 3,
                3 => state = 4,
                _ => {}
            }
        } else {
            match state {
                3 => {
                    p += scnprintf(str_.add(p as usize), len - p as usize, b",%u\0".as_ptr() as *const c_char, last);
                    state = 0;
                }
                4 => {
                    p += scnprintf(str_.add(p as usize), len - p as usize, b"-%u\0".as_ptr() as *const c_char, last);
                    state = 0;
                }
                _ => {}
            }
            if state != 1 {
                state = 0;
            }
        }
        valid >>= 1;
    }
}

unsafe extern "C" fn intel_pt_val_config_term(
    intel_pt_pmu: *mut perf_pmu,
    dirfd: c_int,
    caps: *const c_char,
    name: *const c_char,
    supported: *const c_char,
    mut config: u64,
) -> c_int {
    let mut valid_str = [0 as c_char; 256];
    let mut shift: c_uint = 0;
    let mut valid: c_ulonglong = 0;
    let mut ok: c_int = 0;

    if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, caps, b"%llx\0".as_ptr() as *const c_char, &mut valid) != 1 {
        valid = 0;
    }

    if !supported.is_null()
        && perf_pmu__scan_file_at(intel_pt_pmu, dirfd, supported, b"%d\0".as_ptr() as *const c_char, &mut ok) == 1
        && ok == 0
    {
        valid = 0;
    }

    valid |= 1;

    let mut bits = perf_pmu__format_bits(intel_pt_pmu, name);

    config &= bits;

    while bits != 0 && bits & 1 == 0 {
        shift += 1;
        bits >>= 1;
    }

    config >>= shift;

    if config > 63 {
        intel_pt_valid_str(valid_str.as_mut_ptr(), valid_str.len(), valid as u64);
        pr_err(
            b"Invalid %s for %s. Valid values are: %s\n\0".as_ptr() as *const c_char,
            name,
            INTEL_PT_PMU_NAME,
            valid_str.as_ptr(),
        );
        return -EINVAL;
    }

    if valid & (1u64 << config) as c_ulonglong != 0 {
        return 0;
    }

    intel_pt_valid_str(valid_str.as_mut_ptr(), valid_str.len(), valid as u64);
    pr_err(
        b"Invalid %s for %s. Valid values are: %s\n\0".as_ptr() as *const c_char,
        name,
        INTEL_PT_PMU_NAME,
        valid_str.as_ptr(),
    );
    -EINVAL
}

unsafe extern "C" fn intel_pt_validate_config(intel_pt_pmu: *mut perf_pmu, evsel: *mut evsel) -> c_int {
    let mut err: c_int;
    let mut c: c_char = 0;

    if evsel.is_null() {
        return 0;
    }

    let dirfd = perf_pmu__event_source_devices_fd();
    if dirfd < 0 {
        return dirfd;
    }

    /*
     * If supported, force pass-through config term (pt=1) even if user
     * sets pt=0, which avoids senseless kernel errors.
     */
    if perf_pmu__scan_file_at(intel_pt_pmu, dirfd, b"format/pt\0".as_ptr() as *const c_char, b"%c\0".as_ptr() as *const c_char, &mut c) == 1
        && ((*evsel).core.attr.config & 1) == 0
    {
        pr_warning(b"pt=0 doesn't make sense, forcing pt=1\n\0".as_ptr() as *const c_char);
        (*evsel).core.attr.config |= 1;
    }

    err = intel_pt_val_config_term(
        intel_pt_pmu,
        dirfd,
        b"caps/cycle_thresholds\0".as_ptr() as *const c_char,
        b"cyc_thresh\0".as_ptr() as *const c_char,
        b"caps/psb_cyc\0".as_ptr() as *const c_char,
        (*evsel).core.attr.config,
    );
    if err != 0 {
        close(dirfd);
        return err;
    }

    err = intel_pt_val_config_term(
        intel_pt_pmu,
        dirfd,
        b"caps/mtc_periods\0".as_ptr() as *const c_char,
        b"mtc_period\0".as_ptr() as *const c_char,
        b"caps/mtc\0".as_ptr() as *const c_char,
        (*evsel).core.attr.config,
    );
    if err != 0 {
        close(dirfd);
        return err;
    }

    err = intel_pt_val_config_term(
        intel_pt_pmu,
        dirfd,
        b"caps/psb_periods\0".as_ptr() as *const c_char,
        b"psb_period\0".as_ptr() as *const c_char,
        b"caps/psb_cyc\0".as_ptr() as *const c_char,
        (*evsel).core.attr.config,
    );

    close(dirfd);
    err
}

unsafe extern "C" fn intel_pt_min_max_sample_sz(evlist: *mut evlist, min_sz: *mut size_t, max_sz: *mut size_t) {
    evlist_for_each_entry(evlist, |evsel| unsafe {
        let sz = (*evsel).core.attr.aux_sample_size;

        if sz == 0 {
            return;
        }
        if !min_sz.is_null() && (sz < *min_sz || *min_sz == 0) {
            *min_sz = sz;
        }
        if !max_sz.is_null() && sz > *max_sz {
            *max_sz = sz;
        }
    });
}

/*
 * Currently, there is not enough information to disambiguate different PEBS
 * events, so only allow one.
 */
unsafe extern "C" fn intel_pt_too_many_aux_output(evlist: *mut evlist) -> bool {
    let mut aux_output_cnt: c_int = 0;

    evlist_for_each_entry(evlist, |evsel| unsafe {
        aux_output_cnt += (*evsel).core.attr.aux_output as c_int;
    });

    if aux_output_cnt > 1 {
        pr_err(b"intel_pt supports at most one event with aux-output\n\0".as_ptr() as *const c_char);
        return true;
    }

    false
}

unsafe extern "C" fn intel_pt_recording_options(
    itr: *mut auxtrace_record,
    evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let intel_pt_pmu = (*ptr).intel_pt_pmu;
    let mut have_timing_info: bool;
    let mut need_immediate = false;
    let mut intel_pt_evsel: *mut evsel = ptr::null_mut();
    let cpus = (*evlist__core(evlist)).user_requested_cpus;
    let privileged = perf_event_paranoid_check(-1);
    let mut tsc_bit: u64 = 0;
    let mut err: c_int;

    (*ptr).evlist = evlist;
    (*ptr).snapshot_mode = (*opts).auxtrace_snapshot_mode;

    evlist_for_each_entry(evlist, |evsel| unsafe {
        if (*evsel).core.attr.type_ == (*intel_pt_pmu).type_ {
            if !intel_pt_evsel.is_null() {
                return;
            }
            (*evsel).core.attr.freq = 0;
            (*evsel).core.attr.sample_period = 1;
            (*evsel).core.attr.exclude_guest = intel_pt_exclude_guest();
            (*evsel).no_aux_samples = true;
            (*evsel).needs_auxtrace_mmap = true;
            intel_pt_evsel = evsel;
            (*opts).full_auxtrace = true;
        }
    });

    let mut seen = false;
    let mut duplicate = false;
    evlist_for_each_entry(evlist, |evsel| unsafe {
        if (*evsel).core.attr.type_ == (*intel_pt_pmu).type_ {
            if seen {
                duplicate = true;
            }
            seen = true;
        }
    });
    if duplicate {
        pr_err(b"There may be only one intel_pt event\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (*opts).auxtrace_snapshot_mode && !(*opts).full_auxtrace {
        pr_err(b"Snapshot mode (-S option) requires intel_pt PMU event (-e intel_pt)\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (*opts).auxtrace_snapshot_mode && (*opts).auxtrace_sample_mode {
        pr_err(b"Snapshot mode (intel_pt PMU) and sample trace cannot be used together\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (*opts).use_clockid {
        pr_err(b"Cannot use clockid (-k option) with intel_pt\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if intel_pt_too_many_aux_output(evlist) {
        return -EINVAL;
    }

    if !(*opts).full_auxtrace {
        return 0;
    }

    if (*opts).auxtrace_sample_mode {
        evsel__set_config_if_unset(intel_pt_evsel, b"psb_period\0".as_ptr() as *const c_char, 0);
    }

    err = intel_pt_validate_config(intel_pt_pmu, intel_pt_evsel);
    if err != 0 {
        return err;
    }

    /* Set default sizes for snapshot mode */
    if (*opts).auxtrace_snapshot_mode {
        let psb_period = intel_pt_psb_period(intel_pt_pmu, evlist);

        if (*opts).auxtrace_snapshot_size == 0 && (*opts).auxtrace_mmap_pages == 0 {
            if privileged {
                (*opts).auxtrace_mmap_pages = MiB(4) / page_size as size_t;
            } else {
                (*opts).auxtrace_mmap_pages = KiB(128) / page_size as size_t;
                if (*opts).mmap_pages == UINT_MAX {
                    (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
                }
            }
        } else if (*opts).auxtrace_mmap_pages == 0 && !privileged && (*opts).mmap_pages == UINT_MAX {
            (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
        }
        if (*opts).auxtrace_snapshot_size == 0 {
            (*opts).auxtrace_snapshot_size = (*opts).auxtrace_mmap_pages * page_size as size_t;
        }
        if (*opts).auxtrace_mmap_pages == 0 {
            let mut sz = (*opts).auxtrace_snapshot_size;

            sz = round_up(sz, page_size) / page_size as size_t;
            (*opts).auxtrace_mmap_pages = roundup_pow_of_two(sz);
        }
        if (*opts).auxtrace_snapshot_size > (*opts).auxtrace_mmap_pages * page_size as size_t {
            pr_err(
                b"Snapshot size %zu must not be greater than AUX area tracing mmap size %zu\n\0".as_ptr() as *const c_char,
                (*opts).auxtrace_snapshot_size,
                (*opts).auxtrace_mmap_pages * page_size as size_t,
            );
            return -EINVAL;
        }
        if (*opts).auxtrace_snapshot_size == 0 || (*opts).auxtrace_mmap_pages == 0 {
            pr_err(b"Failed to calculate default snapshot size and/or AUX area tracing mmap pages\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        pr_debug2(b"Intel PT snapshot size: %zu\n\0".as_ptr() as *const c_char, (*opts).auxtrace_snapshot_size);
        if psb_period != 0 && (*opts).auxtrace_snapshot_size <= psb_period + INTEL_PT_PSB_PERIOD_NEAR {
            ui__warning(
                b"Intel PT snapshot size (%zu) may be too small for PSB period (%zu)\n\0".as_ptr() as *const c_char,
                (*opts).auxtrace_snapshot_size,
                psb_period,
            );
        }
    }

    /* Set default sizes for sample mode */
    if (*opts).auxtrace_sample_mode {
        let psb_period = intel_pt_psb_period(intel_pt_pmu, evlist);
        let mut min_sz: size_t = 0;
        let mut max_sz: size_t = 0;

        intel_pt_min_max_sample_sz(evlist, &mut min_sz, &mut max_sz);
        if (*opts).auxtrace_mmap_pages == 0 && !privileged && (*opts).mmap_pages == UINT_MAX {
            (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
        }
        if (*opts).auxtrace_mmap_pages == 0 {
            let sz = round_up(max_sz, page_size) / page_size as size_t;

            (*opts).auxtrace_mmap_pages = roundup_pow_of_two(sz);
        }
        if max_sz > (*opts).auxtrace_mmap_pages * page_size as size_t {
            pr_err(
                b"Sample size %zu must not be greater than AUX area tracing mmap size %zu\n\0".as_ptr() as *const c_char,
                max_sz,
                (*opts).auxtrace_mmap_pages * page_size as size_t,
            );
            return -EINVAL;
        }
        pr_debug2(
            b"Intel PT min. sample size: %zu max. sample size: %zu\n\0".as_ptr() as *const c_char,
            min_sz,
            max_sz,
        );
        if psb_period != 0 && min_sz <= psb_period + INTEL_PT_PSB_PERIOD_NEAR {
            ui__warning(
                b"Intel PT sample size (%zu) may be too small for PSB period (%zu)\n\0".as_ptr() as *const c_char,
                min_sz,
                psb_period,
            );
        }
    }

    /* Set default sizes for full trace mode */
    if (*opts).full_auxtrace && (*opts).auxtrace_mmap_pages == 0 {
        if privileged {
            (*opts).auxtrace_mmap_pages = MiB(4) / page_size as size_t;
        } else {
            (*opts).auxtrace_mmap_pages = KiB(128) / page_size as size_t;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
            }
        }
    }

    /* Validate auxtrace_mmap_pages */
    if (*opts).auxtrace_mmap_pages != 0 {
        let sz = (*opts).auxtrace_mmap_pages * page_size as size_t;
        let min_sz: size_t;

        if (*opts).auxtrace_snapshot_mode || (*opts).auxtrace_sample_mode {
            min_sz = KiB(4);
        } else {
            min_sz = KiB(8);
        }

        if sz < min_sz || !is_power_of_2(sz) {
            pr_err(
                b"Invalid mmap size for Intel Processor Trace: must be at least %zuKiB and a power of 2\n\0".as_ptr() as *const c_char,
                min_sz / 1024,
            );
            return -EINVAL;
        }
    }

    if !(*opts).auxtrace_snapshot_mode && !(*opts).auxtrace_sample_mode {
        let aw = (*opts).auxtrace_mmap_pages * page_size as size_t / 4;
        let aux_watermark: u32 = if aw > UINT_MAX as size_t { UINT_MAX } else { aw as u32 };

        (*intel_pt_evsel).core.attr.aux_watermark = aux_watermark;
    }

    intel_pt_parse_terms(intel_pt_pmu, b"tsc\0".as_ptr() as *const c_char, &mut tsc_bit);

    if (*opts).full_auxtrace && ((*intel_pt_evsel).core.attr.config & tsc_bit) != 0 {
        have_timing_info = true;
    } else {
        have_timing_info = false;
    }

    /*
     * Per-cpu recording needs sched_switch events to distinguish different
     * threads.
     */
    if have_timing_info
        && !perf_cpu_map__is_any_cpu_or_is_empty(cpus)
        && !record_opts__no_switch_events(opts)
    {
        if perf_can_record_switch_events() {
            let cpu_wide = !target__none(&(*opts).target) && !target__has_task(&(*opts).target);

            if (*ptr).all_switch_events && !cpu_wide && perf_can_record_cpu_wide() {
                let switch_evsel = evlist__add_dummy_on_all_cpus(evlist);
                if switch_evsel.is_null() {
                    return -ENOMEM;
                }

                (*switch_evsel).core.attr.context_switch = 1;
                (*switch_evsel).immediate = true;

                evsel__set_sample_bit(switch_evsel, TID);
                evsel__set_sample_bit(switch_evsel, TIME);
                evsel__set_sample_bit(switch_evsel, CPU);
                evsel__reset_sample_bit(switch_evsel, BRANCH_STACK);

                (*opts).record_switch_events = false;
                (*ptr).have_sched_switch = 3;
            } else {
                (*opts).record_switch_events = true;
                need_immediate = true;
                if cpu_wide {
                    (*ptr).have_sched_switch = 3;
                } else {
                    (*ptr).have_sched_switch = 2;
                }
            }
        } else {
            /* Original C has this block under #ifdef HAVE_LIBTRACEEVENT. */
            #[cfg(HAVE_LIBTRACEEVENT)]
            {
                err = intel_pt_track_switches(evlist);
                if err == -EPERM {
                    pr_debug2(b"Unable to select sched:sched_switch\n\0".as_ptr() as *const c_char);
                } else if err != 0 {
                    return err;
                } else {
                    (*ptr).have_sched_switch = 1;
                }
            }
        }
    }

    if have_timing_info
        && !(*intel_pt_evsel).core.attr.exclude_kernel
        && perf_can_record_text_poke_events()
        && perf_can_record_cpu_wide()
    {
        (*opts).text_poke = true;
    }

    if !intel_pt_evsel.is_null() {
        /*
         * To obtain the auxtrace buffer file descriptor, the auxtrace
         * event must come first.
         */
        evlist__to_front(evlist, intel_pt_evsel);
        /*
         * In the case of per-cpu mmaps, we need the CPU on the
         * AUX event.
         */
        if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
            evsel__set_sample_bit(intel_pt_evsel, CPU);
        }
    }

    /* Add dummy event to keep tracking */
    if (*opts).full_auxtrace {
        let need_system_wide_tracking: bool;
        let tracking_evsel: *mut evsel;

        /*
         * User space tasks can migrate between CPUs, so when tracing
         * selected CPUs, sideband for all CPUs is still needed.
         */
        need_system_wide_tracking = !(*opts).target.cpu_list.is_null() && !(*intel_pt_evsel).core.attr.exclude_user;

        tracking_evsel = evlist__add_aux_dummy(evlist, need_system_wide_tracking);
        if tracking_evsel.is_null() {
            return -ENOMEM;
        }

        evlist__set_tracking_event(evlist, tracking_evsel);

        if need_immediate {
            (*tracking_evsel).immediate = true;
        }

        /* In per-cpu case, always need the time of mmap events etc */
        if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
            evsel__set_sample_bit(tracking_evsel, TIME);
            /* And the CPU for switch events */
            evsel__set_sample_bit(tracking_evsel, CPU);
        }
        evsel__reset_sample_bit(tracking_evsel, BRANCH_STACK);
    }

    /*
     * Warn the user when we do not have enough information to decode i.e.
     * per-cpu with no sched_switch (except workload-only).
     */
    if (*ptr).have_sched_switch == 0
        && !perf_cpu_map__is_any_cpu_or_is_empty(cpus)
        && !target__none(&(*opts).target)
        && !(*intel_pt_evsel).core.attr.exclude_user
    {
        ui__warning(b"Intel Processor Trace decoding will not be possible except for kernel tracing!\n\0".as_ptr() as *const c_char);
    }

    0
}

unsafe extern "C" fn intel_pt_snapshot_start(itr: *mut auxtrace_record) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let mut ret = -EINVAL;

    evlist_for_each_entry((*ptr).evlist, |evsel| unsafe {
        if ret == -EINVAL && (*evsel).core.attr.type_ == (*(*ptr).intel_pt_pmu).type_ {
            ret = evsel__disable(evsel);
        }
    });
    ret
}

unsafe extern "C" fn intel_pt_snapshot_finish(itr: *mut auxtrace_record) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let mut ret = -EINVAL;

    evlist_for_each_entry((*ptr).evlist, |evsel| unsafe {
        if ret == -EINVAL && (*evsel).core.attr.type_ == (*(*ptr).intel_pt_pmu).type_ {
            ret = evsel__enable(evsel);
        }
    });
    ret
}

unsafe extern "C" fn intel_pt_alloc_snapshot_refs(ptr: *mut intel_pt_recording, idx: c_int) -> c_int {
    let sz = size_of::<intel_pt_snapshot_ref>();
    let cnt = (*ptr).snapshot_ref_cnt;
    let mut new_cnt = cnt * 2;

    if new_cnt == 0 {
        new_cnt = 16;
    }

    while new_cnt <= idx {
        new_cnt *= 2;
    }

    let refs = calloc(new_cnt as size_t, sz) as *mut intel_pt_snapshot_ref;
    if refs.is_null() {
        return -ENOMEM;
    }

    memcpy(refs as *mut c_void, (*ptr).snapshot_refs as *const c_void, cnt as size_t * sz);

    (*ptr).snapshot_refs = refs;
    (*ptr).snapshot_ref_cnt = new_cnt;

    0
}

unsafe extern "C" fn intel_pt_free_snapshot_refs(ptr: *mut intel_pt_recording) {
    for i in 0..(*ptr).snapshot_ref_cnt {
        zfree(&mut (*(*ptr).snapshot_refs.add(i as usize)).ref_buf);
    }
    zfree(&mut (*ptr).snapshot_refs as *mut *mut intel_pt_snapshot_ref as *mut *mut c_void);
}

unsafe extern "C" fn intel_pt_recording_free(itr: *mut auxtrace_record) {
    let ptr = container_of_intel_pt_recording(itr);

    intel_pt_free_snapshot_refs(ptr);
    free(ptr as *mut c_void);
}

unsafe extern "C" fn intel_pt_alloc_snapshot_ref(
    ptr: *mut intel_pt_recording,
    idx: c_int,
    snapshot_buf_size: size_t,
) -> c_int {
    let ref_buf_size = (*ptr).snapshot_ref_buf_size;
    let ref_buf = zalloc(ref_buf_size);
    if ref_buf.is_null() {
        return -ENOMEM;
    }

    (*(*ptr).snapshot_refs.add(idx as usize)).ref_buf = ref_buf;
    (*(*ptr).snapshot_refs.add(idx as usize)).ref_offset = snapshot_buf_size - ref_buf_size;

    0
}

unsafe extern "C" fn intel_pt_snapshot_ref_buf_size(
    ptr: *mut intel_pt_recording,
    snapshot_buf_size: size_t,
) -> size_t {
    let max_size: size_t = 256 * 1024;
    let mut buf_size: size_t = 0;

    if (*ptr).snapshot_size <= 64 * 1024 {
        return 0;
    }

    let psb_period = intel_pt_psb_period((*ptr).intel_pt_pmu, (*ptr).evlist);
    if psb_period != 0 {
        buf_size = psb_period * 2;
    }

    if buf_size == 0 || buf_size > max_size {
        buf_size = max_size;
    }

    if buf_size >= snapshot_buf_size {
        return 0;
    }

    if buf_size >= (*ptr).snapshot_size / 2 {
        return 0;
    }

    buf_size
}

unsafe extern "C" fn intel_pt_snapshot_init(ptr: *mut intel_pt_recording, snapshot_buf_size: size_t) -> c_int {
    if (*ptr).snapshot_init_done {
        return 0;
    }

    (*ptr).snapshot_init_done = true;

    (*ptr).snapshot_ref_buf_size = intel_pt_snapshot_ref_buf_size(ptr, snapshot_buf_size);

    0
}

/**
 * intel_pt_compare_buffers - compare bytes in a buffer to a circular buffer.
 * @buf1: first buffer
 * @compare_size: number of bytes to compare
 * @buf2: second buffer (a circular buffer)
 * @offs2: offset in second buffer
 * @buf2_size: size of second buffer
 *
 * The comparison allows for the possibility that the bytes to compare in the
 * circular buffer are not contiguous.  It is assumed that @compare_size <=
 * @buf2_size.  This function returns %false if the bytes are identical, %true
 * otherwise.
 */
unsafe extern "C" fn intel_pt_compare_buffers(
    buf1: *mut c_void,
    mut compare_size: size_t,
    buf2: *mut c_void,
    offs2: size_t,
    buf2_size: size_t,
) -> bool {
    let end2 = offs2 + compare_size;

    if end2 <= buf2_size {
        return memcmp(buf1, (buf2 as *mut u8).add(offs2) as *const c_void, compare_size) != 0;
    }

    let part_size = end2 - buf2_size;
    if memcmp(buf1, (buf2 as *mut u8).add(offs2) as *const c_void, part_size) != 0 {
        return true;
    }

    compare_size -= part_size;

    memcmp(
        (buf1 as *mut u8).add(part_size) as *const c_void,
        buf2,
        compare_size,
    ) != 0
}

unsafe extern "C" fn intel_pt_compare_ref(
    ref_buf: *mut c_void,
    ref_offset: size_t,
    ref_size: size_t,
    buf_size: size_t,
    data: *mut c_void,
    head: size_t,
) -> bool {
    let ref_end = ref_offset + ref_size;

    if ref_end > buf_size {
        if head > ref_offset || head < ref_end - buf_size {
            return true;
        }
    } else if head > ref_offset && head < ref_end {
        return true;
    }

    intel_pt_compare_buffers(ref_buf, ref_size, data, ref_offset, buf_size)
}

unsafe extern "C" fn intel_pt_copy_ref(
    ref_buf: *mut c_void,
    mut ref_size: size_t,
    buf_size: size_t,
    data: *mut c_void,
    head: size_t,
) {
    if head >= ref_size {
        memcpy(ref_buf, (data as *mut u8).add(head - ref_size) as *const c_void, ref_size);
    } else {
        memcpy(ref_buf, data, head);
        ref_size -= head;
        memcpy(
            (ref_buf as *mut u8).add(head) as *mut c_void,
            (data as *mut u8).add(buf_size - ref_size) as *const c_void,
            ref_size,
        );
    }
}

unsafe extern "C" fn intel_pt_wrapped(
    ptr: *mut intel_pt_recording,
    idx: c_int,
    mm: *mut auxtrace_mmap,
    data: *mut u8,
    head: u64,
) -> bool {
    let ref_ = (*ptr).snapshot_refs.add(idx as usize);

    let wrapped = intel_pt_compare_ref(
        (*ref_).ref_buf,
        (*ref_).ref_offset,
        (*ptr).snapshot_ref_buf_size,
        (*mm).len,
        data as *mut c_void,
        head as size_t,
    );

    intel_pt_copy_ref(
        (*ref_).ref_buf,
        (*ptr).snapshot_ref_buf_size,
        (*mm).len,
        data as *mut c_void,
        head as size_t,
    );

    wrapped
}

unsafe extern "C" fn intel_pt_first_wrap(data: *mut u64, buf_size: size_t) -> bool {
    let b = (buf_size >> 3) as c_int;
    let mut a = b - 512;
    if a < 0 {
        a = 0;
    }

    for i in a..b {
        if *data.add(i as usize) != 0 {
            return true;
        }
    }

    false
}

unsafe extern "C" fn intel_pt_find_snapshot(
    itr: *mut auxtrace_record,
    idx: c_int,
    mm: *mut auxtrace_mmap,
    data: *mut u8,
    head: *mut u64,
    old: *mut u64,
) -> c_int {
    let ptr = container_of_intel_pt_recording(itr);
    let mut wrapped: bool;
    let mut err: c_int;

    pr_debug3(
        b"%s: mmap index %d old head %zu new head %zu\n\0".as_ptr() as *const c_char,
        b"intel_pt_find_snapshot\0".as_ptr() as *const c_char,
        idx,
        *old as size_t,
        *head as size_t,
    );

    err = intel_pt_snapshot_init(ptr, (*mm).len);
    if err != 0 {
        pr_err(b"%s: failed, error %d\n\0".as_ptr() as *const c_char, b"intel_pt_find_snapshot\0".as_ptr() as *const c_char, err);
        return err;
    }

    if idx >= (*ptr).snapshot_ref_cnt {
        err = intel_pt_alloc_snapshot_refs(ptr, idx);
        if err != 0 {
            pr_err(b"%s: failed, error %d\n\0".as_ptr() as *const c_char, b"intel_pt_find_snapshot\0".as_ptr() as *const c_char, err);
            return err;
        }
    }

    if (*ptr).snapshot_ref_buf_size != 0 {
        if (*(*ptr).snapshot_refs.add(idx as usize)).ref_buf.is_null() {
            err = intel_pt_alloc_snapshot_ref(ptr, idx, (*mm).len);
            if err != 0 {
                pr_err(b"%s: failed, error %d\n\0".as_ptr() as *const c_char, b"intel_pt_find_snapshot\0".as_ptr() as *const c_char, err);
                return err;
            }
        }
        wrapped = intel_pt_wrapped(ptr, idx, mm, data, *head);
    } else {
        wrapped = (*(*ptr).snapshot_refs.add(idx as usize)).wrapped;
        if !wrapped && intel_pt_first_wrap(data as *mut u64, (*mm).len) {
            (*(*ptr).snapshot_refs.add(idx as usize)).wrapped = true;
            wrapped = true;
        }
    }

    /*
     * In full trace mode 'head' continually increases.  However in snapshot
     * mode 'head' is an offset within the buffer.  Here 'old' and 'head'
     * are adjusted to match the full trace case which expects that 'old' is
     * always less than 'head'.
     */
    if wrapped {
        *old = *head;
        *head += (*mm).len as u64;
    } else {
        if (*mm).mask != 0 {
            *old &= (*mm).mask;
        } else {
            *old %= (*mm).len as u64;
        }
        if *old > *head {
            *head += (*mm).len as u64;
        }
    }

    pr_debug3(
        b"%s: wrap-around %sdetected, adjusted old head %zu adjusted new head %zu\n\0".as_ptr() as *const c_char,
        b"intel_pt_find_snapshot\0".as_ptr() as *const c_char,
        if wrapped { b"\0".as_ptr() } else { b"not \0".as_ptr() } as *const c_char,
        *old as size_t,
        *head as size_t,
    );

    0
}

unsafe extern "C" fn intel_pt_reference(_itr: *mut auxtrace_record) -> u64 {
    rdtsc()
}

unsafe extern "C" fn intel_pt_perf_config(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int {
    let ptr = data as *mut intel_pt_recording;

    if strcmp(var, b"intel-pt.all-switch-events\0".as_ptr() as *const c_char) == 0 {
        (*ptr).all_switch_events = perf_config_bool(var, value);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_recording_init(err: *mut c_int) -> *mut auxtrace_record {
    let intel_pt_pmu = perf_pmus__find(INTEL_PT_PMU_NAME);
    let ptr: *mut intel_pt_recording;

    if intel_pt_pmu.is_null() {
        return ptr::null_mut();
    }

    if setenv(
        b"JITDUMP_USE_ARCH_TIMESTAMP\0".as_ptr() as *const c_char,
        b"1\0".as_ptr() as *const c_char,
        1,
    ) != 0
    {
        *err = -errno;
        return ptr::null_mut();
    }

    ptr = zalloc(size_of::<intel_pt_recording>()) as *mut intel_pt_recording;
    if ptr.is_null() {
        *err = -ENOMEM;
        return ptr::null_mut();
    }

    perf_config(intel_pt_perf_config, ptr as *mut c_void);

    (*ptr).intel_pt_pmu = intel_pt_pmu;
    (*ptr).itr.recording_options = Some(intel_pt_recording_options);
    (*ptr).itr.info_priv_size = Some(intel_pt_info_priv_size);
    (*ptr).itr.info_fill = Some(intel_pt_info_fill);
    (*ptr).itr.free = Some(intel_pt_recording_free);
    (*ptr).itr.snapshot_start = Some(intel_pt_snapshot_start);
    (*ptr).itr.snapshot_finish = Some(intel_pt_snapshot_finish);
    (*ptr).itr.find_snapshot = Some(intel_pt_find_snapshot);
    (*ptr).itr.parse_snapshot_options = Some(intel_pt_parse_snapshot_options);
    (*ptr).itr.reference = Some(intel_pt_reference);
    (*ptr).itr.read_finish = Some(auxtrace_record__read_finish);
    /*
     * Decoding starts at a PSB packet. Minimum PSB period is 2K so 4K
     * should give at least 1 PSB per sample.
     */
    (*ptr).itr.default_aux_sample_size = 4096;
    &mut (*ptr).itr
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
