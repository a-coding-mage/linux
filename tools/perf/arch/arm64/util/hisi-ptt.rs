// SPDX-License-Identifier: GPL-2.0
/*
 * HiSilicon PCIe Trace and Tuning (PTT) support
 * Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
 */

// C dependencies removed from executable Rust:
// linux/kernel.h, linux/types.h, linux/bitops.h, linux/log2.h, linux/zalloc.h,
// errno.h, time.h, internal/lib.h, and perf util headers.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u64 = u64;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const UINT_MAX: c_uint = c_uint::MAX;

const HISI_PTT_AUXTRACE_PRIV_SIZE: size_t = 8;
const PERF_AUXTRACE_HISI_PTT: u32 = 8;
const TIME: c_int = 0;

const fn KiB(x: size_t) -> size_t {
    x * 1024
}

const fn MiB(x: size_t) -> size_t {
    x * 1024 * 1024
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub freq: u64,
    pub sample_period: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub needs_auxtrace_mmap: bool,
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_uint,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub type_: u32,
}

#[repr(C)]
pub struct record_opts {
    pub full_auxtrace: bool,
    pub auxtrace_mmap_pages: c_uint,
    pub mmap_pages: c_uint,
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub type_: u32,
    pub priv_: [u64; 1],
}

#[repr(C)]
pub struct auxtrace_record {
    pub recording_options: Option<
        unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int,
    >,
    pub info_priv_size:
        Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t>,
    pub info_fill: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            *mut perf_session,
            *mut perf_record_auxtrace_info,
            size_t,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    pub reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    pub read_finish: Option<unsafe extern "C" fn()>,
    pub alignment: c_uint,
}

#[repr(C)]
struct hisi_ptt_recording {
    itr: auxtrace_record,
    hisi_ptt_pmu: *mut perf_pmu,
    evlist: *mut evlist,
}

extern "C" {
    static page_size: c_ulong;

    fn perf_event_paranoid_check(paranoid: c_int) -> bool;
    fn is_power_of_2(n: size_t) -> bool;
    fn pr_err(fmt: *const c_char, ...);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn rdtsc() -> u64;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn auxtrace_record__read_finish();
}

// External iterator helper corresponding to evlist__for_each_entry().
extern "C" {
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

unsafe fn hisi_ptt_recording_from_itr(itr: *mut auxtrace_record) -> *mut hisi_ptt_recording {
    itr as *mut hisi_ptt_recording
}

unsafe extern "C" fn hisi_ptt_info_priv_size(
    _itr: *mut auxtrace_record,
    _evlist: *mut evlist,
) -> size_t {
    HISI_PTT_AUXTRACE_PRIV_SIZE
}

unsafe extern "C" fn hisi_ptt_info_fill(
    itr: *mut auxtrace_record,
    session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    priv_size: size_t,
) -> c_int {
    let pttr = hisi_ptt_recording_from_itr(itr);
    let hisi_ptt_pmu = (*pttr).hisi_ptt_pmu;

    if priv_size != HISI_PTT_AUXTRACE_PRIV_SIZE {
        return -EINVAL;
    }

    if (*evlist__core((*session).evlist)).nr_mmaps == 0 {
        return -EINVAL;
    }

    (*auxtrace_info).type_ = PERF_AUXTRACE_HISI_PTT;
    (*auxtrace_info).priv_[0] = (*hisi_ptt_pmu).type_ as u64;

    0
}

unsafe extern "C" fn hisi_ptt_set_auxtrace_mmap_page(opts: *mut record_opts) -> c_int {
    let privileged = perf_event_paranoid_check(-1);

    if !(*opts).full_auxtrace {
        return 0;
    }

    if (*opts).full_auxtrace && (*opts).auxtrace_mmap_pages == 0 {
        if privileged {
            (*opts).auxtrace_mmap_pages = (MiB(16) / page_size as size_t) as c_uint;
        } else {
            (*opts).auxtrace_mmap_pages = (KiB(128) / page_size as size_t) as c_uint;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
            }
        }
    }

    /* Validate auxtrace_mmap_pages */
    if (*opts).auxtrace_mmap_pages != 0 {
        let sz = (*opts).auxtrace_mmap_pages as size_t * page_size as size_t;
        let min_sz = KiB(8);

        if sz < min_sz || !is_power_of_2(sz) {
            pr_err(
                b"Invalid mmap size for HISI PTT: must be at least %zuKiB and a power of 2\n\0"
                    .as_ptr() as *const c_char,
                min_sz / 1024,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn hisi_ptt_recording_options(
    itr: *mut auxtrace_record,
    evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let pttr = hisi_ptt_recording_from_itr(itr);
    let hisi_ptt_pmu = (*pttr).hisi_ptt_pmu;
    let mut evsel: *mut evsel;
    let mut hisi_ptt_evsel: *mut evsel = ptr::null_mut();
    let tracking_evsel: *mut evsel;
    let mut err: c_int;

    (*pttr).evlist = evlist;
    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if (*evsel).core.attr.type_ == (*hisi_ptt_pmu).type_ {
            if !hisi_ptt_evsel.is_null() {
                pr_err(b"There may be only one HISI_PTT_PMU_NAMEx event\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            (*evsel).core.attr.freq = 0;
            (*evsel).core.attr.sample_period = 1;
            (*evsel).needs_auxtrace_mmap = true;
            hisi_ptt_evsel = evsel;
            (*opts).full_auxtrace = true;
        }
        evsel = evlist__next(evlist, evsel);
    }

    err = hisi_ptt_set_auxtrace_mmap_page(opts);
    if err != 0 {
        return err;
    }
    /*
     * To obtain the auxtrace buffer file descriptor, the auxtrace event
     * must come first.
     */
    evlist__to_front(evlist, hisi_ptt_evsel);
    evsel__set_sample_bit(hisi_ptt_evsel, TIME);

    /* Add dummy event to keep tracking */
    err = parse_event(evlist, b"dummy:u\0".as_ptr() as *const c_char);
    if err != 0 {
        return err;
    }

    tracking_evsel = evlist__last(evlist);
    evlist__set_tracking_event(evlist, tracking_evsel);

    (*tracking_evsel).core.attr.freq = 0;
    (*tracking_evsel).core.attr.sample_period = 1;
    evsel__set_sample_bit(tracking_evsel, TIME);

    0
}

unsafe extern "C" fn hisi_ptt_reference(_itr: *mut auxtrace_record) -> u64 {
    rdtsc()
}

unsafe extern "C" fn hisi_ptt_recording_free(itr: *mut auxtrace_record) {
    let pttr = hisi_ptt_recording_from_itr(itr);

    free(pttr as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn hisi_ptt_recording_init(
    err: *mut c_int,
    hisi_ptt_pmu: *mut perf_pmu,
) -> *mut auxtrace_record {
    let pttr: *mut hisi_ptt_recording;

    if hisi_ptt_pmu.is_null() {
        *err = -ENODEV;
        return ptr::null_mut();
    }

    pttr = zalloc(size_of::<hisi_ptt_recording>()) as *mut hisi_ptt_recording;
    if pttr.is_null() {
        *err = -ENOMEM;
        return ptr::null_mut();
    }

    (*pttr).hisi_ptt_pmu = hisi_ptt_pmu;
    (*pttr).itr.recording_options = Some(hisi_ptt_recording_options);
    (*pttr).itr.info_priv_size = Some(hisi_ptt_info_priv_size);
    (*pttr).itr.info_fill = Some(hisi_ptt_info_fill);
    (*pttr).itr.free = Some(hisi_ptt_recording_free);
    (*pttr).itr.reference = Some(hisi_ptt_reference);
    (*pttr).itr.read_finish = Some(auxtrace_record__read_finish);
    (*pttr).itr.alignment = 0;

    *err = 0;
    &mut (*pttr).itr
}
