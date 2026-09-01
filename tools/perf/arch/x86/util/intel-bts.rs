// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel-bts.c: Intel Processor Trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 */

// Rust translation of perf/arch/x86/util/intel-bts.c.
// C include dependencies are intentionally left as external declarations.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u64 = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const UINT_MAX: c_uint = c_uint::MAX;

const fn KiB(x: size_t) -> size_t {
    x * 1024
}

const fn MiB(x: size_t) -> size_t {
    x * 1024 * 1024
}

const fn KiB_MASK(x: size_t) -> size_t {
    KiB(x) - 1
}

const fn MiB_MASK(x: size_t) -> size_t {
    MiB(x) - 1
}

const INTEL_BTS_PMU_NAME: *const c_char = b"intel_bts\0".as_ptr() as *const c_char;
const INTEL_BTS_AUXTRACE_PRIV_SIZE: size_t = 8;
const INTEL_BTS_PMU_TYPE: usize = 0;
const INTEL_BTS_TIME_SHIFT: usize = 1;
const INTEL_BTS_TIME_MULT: usize = 2;
const INTEL_BTS_TIME_ZERO: usize = 3;
const INTEL_BTS_CAP_USER_TIME_ZERO: usize = 4;
const INTEL_BTS_SNAPSHOT_MODE: usize = 5;
const PERF_AUXTRACE_INTEL_BTS: u32 = 4;
const CPU: c_int = 0;

#[repr(C)]
pub struct intel_bts_snapshot_ref {
    ref_buf: *mut c_void,
    ref_offset: size_t,
    wrapped: bool,
}

#[repr(C)]
pub struct intel_bts_recording {
    itr: auxtrace_record,
    intel_bts_pmu: *mut perf_pmu,
    evlist: *mut evlist,
    snapshot_mode: bool,
    snapshot_size: size_t,
    snapshot_ref_cnt: c_int,
    snapshot_refs: *mut intel_bts_snapshot_ref,
}

#[repr(C)]
pub struct branch {
    from: u64,
    to: u64,
    misc: u64,
}

#[repr(C)]
pub struct auxtrace_record {
    recording_options: Option<
        unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int,
    >,
    info_priv_size: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t>,
    info_fill: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            *mut perf_session,
            *mut perf_record_auxtrace_info,
            size_t,
        ) -> c_int,
    >,
    free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    snapshot_start: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    snapshot_finish: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    find_snapshot: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            c_int,
            *mut auxtrace_mmap,
            *mut u8,
            *mut u64,
            *mut u64,
        ) -> c_int,
    >,
    parse_snapshot_options:
        Option<unsafe extern "C" fn(*mut auxtrace_record, *mut record_opts, *const c_char) -> c_int>,
    reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    read_finish: Option<unsafe extern "C" fn()>,
    alignment: size_t,
}

#[repr(C)]
pub struct perf_pmu {
    type_: u32,
}

#[repr(C)]
pub struct perf_session {
    evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    type_: u32,
    priv_: [u64; 8],
}

#[repr(C)]
pub struct perf_tsc_conversion {
    time_shift: u16,
    time_mult: u32,
    time_zero: u64,
}

#[repr(C)]
pub struct evlist_core {
    nr_mmaps: c_int,
    user_requested_cpus: *const perf_cpu_map,
}

#[repr(C)]
pub struct evlist {
    core: evlist_core,
}

#[repr(C)]
pub struct perf_event_attr {
    type_: u32,
    freq: u64,
    sample_period: u64,
}

#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    core: evsel_core,
    needs_auxtrace_mmap: bool,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    auxtrace_sample_mode: bool,
    auxtrace_snapshot_mode: bool,
    auxtrace_snapshot_size: size_t,
    auxtrace_mmap_pages: size_t,
    mmap_pages: c_uint,
    full_auxtrace: bool,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap_core {
    base: *mut perf_event_mmap_page,
}

#[repr(C)]
pub struct perf_mmap {
    core: mmap_core,
}

#[repr(C)]
pub struct auxtrace_mmap {
    len: size_t,
    mask: u64,
}

unsafe extern "C" {
    static page_size: c_uint;
    static mut errno: c_int;

    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut perf_mmap;
    fn perf_read_tsc_conversion(
        pc: *mut perf_event_mmap_page,
        tc: *mut perf_tsc_conversion,
    ) -> c_int;
    fn ui__warning(fmt: *const c_char, ...);
    fn perf_event_paranoid_check(level: c_int) -> bool;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *const perf_cpu_map) -> bool;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn rdtsc() -> u64;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn zalloc(size: size_t) -> *mut c_void;
    fn evsel__disable(evsel: *mut evsel) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn auxtrace_record__read_finish();
}

unsafe fn zfree(ptrp: *mut *mut c_void) {
    if !(*ptrp).is_null() {
        free(*ptrp);
        *ptrp = ptr::null_mut();
    }
}

unsafe fn round_up(x: size_t, y: c_uint) -> size_t {
    let y = y as size_t;
    ((x + y - 1) / y) * y
}

unsafe fn is_power_of_2(x: size_t) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

unsafe fn roundup_pow_of_two(mut x: size_t) -> size_t {
    if x <= 1 {
        return 1;
    }
    x -= 1;
    let mut shift = 1;
    while shift < size_of::<size_t>() * 8 {
        x |= x >> shift;
        shift <<= 1;
    }
    x + 1
}

unsafe fn intel_bts_for_each_evsel(
    _evlist: *mut evlist,
    _cb: unsafe fn(*mut evsel, *mut c_void) -> c_int,
    _data: *mut c_void,
) -> c_int {
    // TODO: depends on evlist__for_each_entry() list iteration from perf headers.
    0
}

unsafe extern "C" fn intel_bts_info_priv_size(
    _itr: *mut auxtrace_record,
    _evlist: *mut evlist,
) -> size_t {
    INTEL_BTS_AUXTRACE_PRIV_SIZE
}

unsafe extern "C" fn intel_bts_info_fill(
    itr: *mut auxtrace_record,
    session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    priv_size: size_t,
) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let intel_bts_pmu = (*btsr).intel_bts_pmu;
    let mut tc = perf_tsc_conversion {
        time_shift: 0,
        time_mult: 0,
        time_zero: 0,
    };
    let mut cap_user_time_zero = false;
    let mut err: c_int;

    if priv_size != INTEL_BTS_AUXTRACE_PRIV_SIZE {
        return -EINVAL;
    }

    if (*evlist__core((*session).evlist)).nr_mmaps == 0 {
        return -EINVAL;
    }

    let pc = (*evlist__mmap((*session).evlist).add(0)).core.base;
    if !pc.is_null() {
        err = perf_read_tsc_conversion(pc, &mut tc);
        if err != 0 {
            if err != -EOPNOTSUPP {
                return err;
            }
        } else {
            cap_user_time_zero = tc.time_mult != 0;
        }
        if !cap_user_time_zero {
            ui__warning(c"Intel BTS: TSC not available\n".as_ptr());
        }
    }

    (*auxtrace_info).type_ = PERF_AUXTRACE_INTEL_BTS;
    (*auxtrace_info).priv_[INTEL_BTS_PMU_TYPE] = (*intel_bts_pmu).type_ as u64;
    (*auxtrace_info).priv_[INTEL_BTS_TIME_SHIFT] = tc.time_shift as u64;
    (*auxtrace_info).priv_[INTEL_BTS_TIME_MULT] = tc.time_mult as u64;
    (*auxtrace_info).priv_[INTEL_BTS_TIME_ZERO] = tc.time_zero;
    (*auxtrace_info).priv_[INTEL_BTS_CAP_USER_TIME_ZERO] = cap_user_time_zero as u64;
    (*auxtrace_info).priv_[INTEL_BTS_SNAPSHOT_MODE] = (*btsr).snapshot_mode as u64;

    0
}

unsafe extern "C" fn intel_bts_recording_options(
    itr: *mut auxtrace_record,
    evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let intel_bts_pmu = (*btsr).intel_bts_pmu;
    let mut intel_bts_evsel: *mut evsel = ptr::null_mut();
    let cpus = (*evlist__core(evlist)).user_requested_cpus;
    let privileged = perf_event_paranoid_check(-1);

    if (*opts).auxtrace_sample_mode {
        pr_err(c"Intel BTS does not support AUX area sampling\n".as_ptr());
        return -EINVAL;
    }

    (*btsr).evlist = evlist;
    (*btsr).snapshot_mode = (*opts).auxtrace_snapshot_mode;

    // evlist__for_each_entry(evlist, evsel)
    // TODO: requires perf evlist iteration support from external headers.
    let _ = intel_bts_pmu;
    let _ = &mut intel_bts_evsel;

    if (*opts).auxtrace_snapshot_mode && !(*opts).full_auxtrace {
        pr_err(
            c"Snapshot mode (-S option) requires intel_bts PMU event (-e intel_bts)\n".as_ptr(),
        );
        return -EINVAL;
    }

    if !(*opts).full_auxtrace {
        return 0;
    }

    if (*opts).full_auxtrace && !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        pr_err(c"intel_bts does not support per-cpu recording\n".as_ptr());
        return -EINVAL;
    }

    /* Set default sizes for snapshot mode */
    if (*opts).auxtrace_snapshot_mode {
        if (*opts).auxtrace_snapshot_size == 0 && (*opts).auxtrace_mmap_pages == 0 {
            if privileged {
                (*opts).auxtrace_mmap_pages = MiB(4) / page_size as size_t;
            } else {
                (*opts).auxtrace_mmap_pages = KiB(128) / page_size as size_t;
                if (*opts).mmap_pages == UINT_MAX {
                    (*opts).mmap_pages = (KiB(256) / page_size as size_t) as c_uint;
                }
            }
        } else if (*opts).auxtrace_mmap_pages == 0
            && !privileged
            && (*opts).mmap_pages == UINT_MAX
        {
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
                c"Snapshot size %zu must not be greater than AUX area tracing mmap size %zu\n"
                    .as_ptr(),
                (*opts).auxtrace_snapshot_size,
                (*opts).auxtrace_mmap_pages * page_size as size_t,
            );
            return -EINVAL;
        }
        if (*opts).auxtrace_snapshot_size == 0 || (*opts).auxtrace_mmap_pages == 0 {
            pr_err(c"Failed to calculate default snapshot size and/or AUX area tracing mmap pages\n".as_ptr());
            return -EINVAL;
        }
        pr_debug2(
            c"Intel BTS snapshot size: %zu\n".as_ptr(),
            (*opts).auxtrace_snapshot_size,
        );
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

        if (*opts).auxtrace_snapshot_mode {
            min_sz = KiB(4);
        } else {
            min_sz = KiB(8);
        }

        if sz < min_sz || !is_power_of_2(sz) {
            pr_err(
                c"Invalid mmap size for Intel BTS: must be at least %zuKiB and a power of 2\n"
                    .as_ptr(),
                min_sz / 1024,
            );
            return -EINVAL;
        }
    }

    if !intel_bts_evsel.is_null() {
        /*
         * To obtain the auxtrace buffer file descriptor, the auxtrace event
         * must come first.
         */
        evlist__to_front(evlist, intel_bts_evsel);
        /*
         * In the case of per-cpu mmaps, we need the CPU on the
         * AUX event.
         */
        if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
            evsel__set_sample_bit(intel_bts_evsel, CPU);
        }
    }

    /* Add dummy event to keep tracking */
    if (*opts).full_auxtrace {
        let tracking_evsel: *mut evsel;
        let mut err: c_int;

        err = parse_event(evlist, c"dummy:u".as_ptr());
        if err != 0 {
            return err;
        }

        tracking_evsel = evlist__last(evlist);

        evlist__set_tracking_event(evlist, tracking_evsel);

        (*tracking_evsel).core.attr.freq = 0;
        (*tracking_evsel).core.attr.sample_period = 1;
    }

    0
}

unsafe extern "C" fn intel_bts_parse_snapshot_options(
    itr: *mut auxtrace_record,
    opts: *mut record_opts,
    str_: *const c_char,
) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let mut snapshot_size: c_ulonglong = 0;
    let mut endptr: *mut c_char = ptr::null_mut();

    if !str_.is_null() {
        snapshot_size = strtoull(str_, &mut endptr, 0);
        if *endptr != 0 || snapshot_size > size_t::MAX as c_ulonglong {
            return -1;
        }
    }

    (*opts).auxtrace_snapshot_mode = true;
    (*opts).auxtrace_snapshot_size = snapshot_size as size_t;

    (*btsr).snapshot_size = snapshot_size as size_t;

    0
}

unsafe extern "C" fn intel_bts_reference(_itr: *mut auxtrace_record) -> u64 {
    rdtsc()
}

unsafe fn intel_bts_alloc_snapshot_refs(
    btsr: *mut intel_bts_recording,
    idx: c_int,
) -> c_int {
    let sz: size_t = size_of::<intel_bts_snapshot_ref>();
    let cnt = (*btsr).snapshot_ref_cnt;
    let mut new_cnt = cnt * 2;
    let refs: *mut intel_bts_snapshot_ref;

    if new_cnt == 0 {
        new_cnt = 16;
    }

    while new_cnt <= idx {
        new_cnt *= 2;
    }

    refs = calloc(new_cnt as size_t, sz) as *mut intel_bts_snapshot_ref;
    if refs.is_null() {
        return -ENOMEM;
    }

    memcpy(
        refs as *mut c_void,
        (*btsr).snapshot_refs as *const c_void,
        cnt as size_t * sz,
    );

    (*btsr).snapshot_refs = refs;
    (*btsr).snapshot_ref_cnt = new_cnt;

    0
}

unsafe fn intel_bts_free_snapshot_refs(btsr: *mut intel_bts_recording) {
    let mut i: c_int = 0;

    while i < (*btsr).snapshot_ref_cnt {
        zfree(&mut (*(*btsr).snapshot_refs.add(i as usize)).ref_buf);
        i += 1;
    }
    zfree(&mut (*btsr).snapshot_refs as *mut *mut intel_bts_snapshot_ref as *mut *mut c_void);
}

unsafe extern "C" fn intel_bts_recording_free(itr: *mut auxtrace_record) {
    let btsr = itr as *mut intel_bts_recording;

    intel_bts_free_snapshot_refs(btsr);
    free(btsr as *mut c_void);
}

unsafe extern "C" fn intel_bts_snapshot_start(itr: *mut auxtrace_record) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let evsel: *mut evsel = ptr::null_mut();

    // evlist__for_each_entry(btsr->evlist, evsel)
    // TODO: requires perf evlist iteration support from external headers.
    if !evsel.is_null() && (*evsel).core.attr.type_ == (*(*btsr).intel_bts_pmu).type_ {
        return evsel__disable(evsel);
    }
    -EINVAL
}

unsafe extern "C" fn intel_bts_snapshot_finish(itr: *mut auxtrace_record) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let evsel: *mut evsel = ptr::null_mut();

    // evlist__for_each_entry(btsr->evlist, evsel)
    // TODO: requires perf evlist iteration support from external headers.
    if !evsel.is_null() && (*evsel).core.attr.type_ == (*(*btsr).intel_bts_pmu).type_ {
        return evsel__enable(evsel);
    }
    -EINVAL
}

unsafe fn intel_bts_first_wrap(data: *mut u64, buf_size: size_t) -> bool {
    let mut i: c_int;
    let mut a: c_int;
    let b: c_int;

    b = (buf_size >> 3) as c_int;
    a = b - 512;
    if a < 0 {
        a = 0;
    }

    i = a;
    while i < b {
        if *data.add(i as usize) != 0 {
            return true;
        }
        i += 1;
    }

    false
}

unsafe extern "C" fn intel_bts_find_snapshot(
    itr: *mut auxtrace_record,
    idx: c_int,
    mm: *mut auxtrace_mmap,
    data: *mut u8,
    head: *mut u64,
    old: *mut u64,
) -> c_int {
    let btsr = itr as *mut intel_bts_recording;
    let mut wrapped: bool;
    let mut err: c_int;

    pr_debug3(
        c"%s: mmap index %d old head %zu new head %zu\n".as_ptr(),
        c"intel_bts_find_snapshot".as_ptr(),
        idx,
        *old as size_t,
        *head as size_t,
    );

    if idx >= (*btsr).snapshot_ref_cnt {
        err = intel_bts_alloc_snapshot_refs(btsr, idx);
        if err != 0 {
            pr_err(
                c"%s: failed, error %d\n".as_ptr(),
                c"intel_bts_find_snapshot".as_ptr(),
                err,
            );
            return err;
        }
    }

    wrapped = (*(*btsr).snapshot_refs.add(idx as usize)).wrapped;
    if !wrapped && intel_bts_first_wrap(data as *mut u64, (*mm).len) {
        (*(*btsr).snapshot_refs.add(idx as usize)).wrapped = true;
        wrapped = true;
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
        c"%s: wrap-around %sdetected, adjusted old head %zu adjusted new head %zu\n".as_ptr(),
        c"intel_bts_find_snapshot".as_ptr(),
        if wrapped { c"".as_ptr() } else { c"not ".as_ptr() },
        *old as size_t,
        *head as size_t,
    );

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_bts_recording_init(err: *mut c_int) -> *mut auxtrace_record {
    let intel_bts_pmu = perf_pmus__find(INTEL_BTS_PMU_NAME);
    let btsr: *mut intel_bts_recording;

    if intel_bts_pmu.is_null() {
        return ptr::null_mut();
    }

    if setenv(
        c"JITDUMP_USE_ARCH_TIMESTAMP".as_ptr(),
        c"1".as_ptr(),
        1,
    ) != 0
    {
        *err = -errno;
        return ptr::null_mut();
    }

    btsr = zalloc(size_of::<intel_bts_recording>()) as *mut intel_bts_recording;
    if btsr.is_null() {
        *err = -ENOMEM;
        return ptr::null_mut();
    }

    (*btsr).intel_bts_pmu = intel_bts_pmu;
    (*btsr).itr.recording_options = Some(intel_bts_recording_options);
    (*btsr).itr.info_priv_size = Some(intel_bts_info_priv_size);
    (*btsr).itr.info_fill = Some(intel_bts_info_fill);
    (*btsr).itr.free = Some(intel_bts_recording_free);
    (*btsr).itr.snapshot_start = Some(intel_bts_snapshot_start);
    (*btsr).itr.snapshot_finish = Some(intel_bts_snapshot_finish);
    (*btsr).itr.find_snapshot = Some(intel_bts_find_snapshot);
    (*btsr).itr.parse_snapshot_options = Some(intel_bts_parse_snapshot_options);
    (*btsr).itr.reference = Some(intel_bts_reference);
    (*btsr).itr.read_finish = Some(auxtrace_record__read_finish);
    (*btsr).itr.alignment = size_of::<branch>();
    &mut (*btsr).itr
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
