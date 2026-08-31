// SPDX-License-Identifier: GPL-2.0
/*
 * Arm Statistical Profiling Extensions (SPE) support
 * Copyright (c) 2017-2018, Arm Ltd.
 */

// Translated from C implementation source. External types, constants, and
// functions are supplied by the surrounding perf sources.

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type bool_ = bool;
type u64 = u64;
type __u64 = u64;

const ARM_SPE_CPU_MAGIC: u64 = 0x1010101010101010u64;

const fn KiB(x: size_t) -> size_t {
    x * 1024
}

const fn MiB(x: size_t) -> size_t {
    x * 1024 * 1024
}

extern "C" {
    static page_size: c_ulong;

    static ARM_SPE_PMU_NAME: *const c_char;

    static ARM_SPE_MAGIC: size_t;
    static ARM_SPE_CPU: size_t;
    static ARM_SPE_CPU_NR_PARAMS: size_t;
    static ARM_SPE_CPU_MIDR: size_t;
    static ARM_SPE_CPU_PMU_TYPE: size_t;
    static ARM_SPE_CAP_MIN_IVAL: size_t;
    static ARM_SPE_CAP_EVENT_FILTER: size_t;
    static ARM_SPE_AUXTRACE_PRIV_MAX: size_t;
    static ARM_SPE_CPU_PRIV_MAX: size_t;
    static ARM_SPE_HEADER_VERSION: size_t;
    static ARM_SPE_HEADER_CURRENT_VERSION: u64;
    static ARM_SPE_HEADER_SIZE: size_t;
    static ARM_SPE_PMU_TYPE_V2: size_t;
    static ARM_SPE_CPUS_NUM: size_t;
    static PERF_AUXTRACE_ARM_SPE: u32;

    static EVSEL__CONFIG_TERM_FREQ: c_int;
    static UINT_MAX: u32;
    static ULLONG_MAX: u64;
    static SIZE_MAX: c_ulonglong;
    static ENOMEM: c_int;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static CLOCK_MONOTONIC_RAW: c_int;

    static CPU: c_int;
    static DATA_SRC: c_int;
    static PHYS_ADDR: c_int;
    static TIME: c_int;

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__has_any_cpu(cpus: *const perf_cpu_map) -> bool_;
    fn perf_cpu_map__intersect(
        a: *const perf_cpu_map,
        b: *const perf_cpu_map,
    ) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__has(cpus: *const perf_cpu_map, cpu: perf_cpu) -> bool_;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *const perf_cpu_map) -> bool_;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;

    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evsel: *mut evsel) -> *mut evsel;

    fn evsel__is_aux_event(evsel: *mut evsel) -> bool_;
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__set_config_if_unset(evsel: *mut evsel, name: *const c_char, val: u64);
    fn evsel__get_config_val(evsel: *mut evsel, name: *const c_char, val: *mut u64) -> c_int;
    fn evsel__config_exists(evsel: *mut evsel, name: *const c_char) -> bool_;
    fn evsel__disable(evsel: *mut evsel) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;

    fn perf_pmu__scan_file(pmu: *const perf_pmu, path: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn get_cpuid_allow_env_override(cpu: perf_cpu) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut bool_);
    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn record_opts__no_switch_events(opts: *const record_opts) -> bool_;
    fn perf_event_paranoid_check(arg: c_int) -> bool_;
    fn round_up(x: size_t, y: c_ulong) -> size_t;
    fn roundup_pow_of_two(x: size_t) -> size_t;
    fn is_power_of_2(x: size_t) -> bool_;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn auxtrace_record__read_finish(itr: *mut auxtrace_record, idx: c_int) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

type c_long = isize;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct evsel_config_term {
    pub list: list_head,
    pub type_: c_int,
}

#[repr(C)]
pub struct perf_event_attr {
    pub freq: u64,
    pub sample_period: u64,
    pub context_switch: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub pmu: *mut perf_pmu,
    pub evlist: *mut evlist,
    pub needs_auxtrace_mmap: bool_,
    pub config_terms: list_head,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub cpus: *mut perf_cpu_map,
    pub type_: u64,
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
    pub nr_mmaps: c_int,
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub type_: u32,
    pub priv_: [u64; 0],
}

#[repr(C)]
pub struct record_opts {
    pub auxtrace_snapshot_size: size_t,
    pub auxtrace_mmap_pages: size_t,
    pub mmap_pages: u32,
    pub auxtrace_snapshot_mode: bool_,
    pub full_auxtrace: bool_,
    pub user_freq: u32,
}

#[repr(C)]
pub struct auxtrace_mmap {
    pub len: size_t,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct auxtrace_record {
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
    pub reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    pub read_finish: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int) -> c_int>,
    pub alignment: c_int,
}

#[repr(C)]
pub struct arm_spe_recording {
    pub itr: auxtrace_record,
    pub arm_spe_pmu: *mut perf_pmu,
    pub evlist: *mut evlist,
    pub wrapped_cnt: c_int,
    pub wrapped: *mut bool_,
}

unsafe fn container_of_itr(itr: *mut auxtrace_record) -> *mut arm_spe_recording {
    itr as *mut arm_spe_recording
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/* Iterate config list to detect if the "freq" parameter is set */
unsafe extern "C" fn arm_spe_is_set_freq(evsel: *mut evsel) -> bool_ {
    let mut pos = (*evsel).config_terms.next;

    while pos != &mut (*evsel).config_terms as *mut list_head {
        let term = pos as *mut evsel_config_term;
        if (*term).type_ == EVSEL__CONFIG_TERM_FREQ {
            return true;
        }
        pos = (*pos).next;
    }

    false
}

/*
 * arm_spe_find_cpus() returns a new cpu map, and the caller should invoke
 * perf_cpu_map__put() to release the map after use.
 */
unsafe extern "C" fn arm_spe_find_cpus(evlist: *mut evlist) -> *mut perf_cpu_map {
    let event_cpus = (*evlist__core(evlist)).user_requested_cpus;
    let online_cpus = perf_cpu_map__new_online_cpus();
    let intersect_cpus: *mut perf_cpu_map;

    /* cpu map is not "any" CPU , we have specific CPUs to work with */
    if !perf_cpu_map__has_any_cpu(event_cpus) {
        intersect_cpus = perf_cpu_map__intersect(event_cpus, online_cpus);
        perf_cpu_map__put(online_cpus);
    /* Event can be "any" CPU so count all CPUs. */
    } else {
        intersect_cpus = online_cpus;
    }

    intersect_cpus
}

unsafe extern "C" fn arm_spe_info_priv_size(
    _itr: *mut auxtrace_record,
    evlist: *mut evlist,
) -> size_t {
    let cpu_map = arm_spe_find_cpus(evlist);
    let mut size: size_t;

    if cpu_map.is_null() {
        return 0;
    }

    size = ARM_SPE_AUXTRACE_PRIV_MAX + ARM_SPE_CPU_PRIV_MAX * perf_cpu_map__nr(cpu_map) as size_t;
    size *= size_of::<u64>();

    perf_cpu_map__put(cpu_map);
    size
}

unsafe extern "C" fn arm_spe_save_cpu_header(
    itr: *mut auxtrace_record,
    cpu: perf_cpu,
    data: *mut __u64,
) -> c_int {
    let sper = container_of_itr(itr);
    let mut pmu: *mut perf_pmu = ptr::null_mut();
    let mut cpuid: *mut c_char = ptr::null_mut();
    let mut val: u64;

    /* Read CPU MIDR */
    cpuid = get_cpuid_allow_env_override(cpu);
    if cpuid.is_null() {
        return -ENOMEM;
    }
    val = strtol(cpuid, ptr::null_mut(), 16) as u64;

    *data.add(ARM_SPE_MAGIC) = ARM_SPE_CPU_MAGIC;
    *data.add(ARM_SPE_CPU) = cpu.cpu as u64;
    *data.add(ARM_SPE_CPU_NR_PARAMS) = ARM_SPE_CPU_PRIV_MAX as u64 - ARM_SPE_CPU_MIDR as u64;
    *data.add(ARM_SPE_CPU_MIDR) = val;

    /* Find the associate Arm SPE PMU for the CPU */
    if perf_cpu_map__has((*(*sper).arm_spe_pmu).cpus, cpu) {
        pmu = (*sper).arm_spe_pmu;
    }

    if pmu.is_null() {
        /* No Arm SPE PMU is found */
        *data.add(ARM_SPE_CPU_PMU_TYPE) = ULLONG_MAX;
        *data.add(ARM_SPE_CAP_MIN_IVAL) = 0;
        *data.add(ARM_SPE_CAP_EVENT_FILTER) = 0;
    } else {
        *data.add(ARM_SPE_CPU_PMU_TYPE) = (*pmu).type_;

        if perf_pmu__scan_file(pmu, cstr(b"caps/min_interval\0"), cstr(b"%lu\0"), &mut val) != 1 {
            val = 0;
        }
        *data.add(ARM_SPE_CAP_MIN_IVAL) = val;

        if perf_pmu__scan_file(pmu, cstr(b"caps/event_filter\0"), cstr(b"%lx\0"), &mut val) != 1 {
            val = 0;
        }
        *data.add(ARM_SPE_CAP_EVENT_FILTER) = val;
    }

    free(cpuid as *mut c_void);
    ARM_SPE_CPU_PRIV_MAX as c_int
}

unsafe extern "C" fn arm_spe_info_fill(
    itr: *mut auxtrace_record,
    session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    priv_size: size_t,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut offset: size_t;
    let sper = container_of_itr(itr);
    let arm_spe_pmu = (*sper).arm_spe_pmu;
    let cpu_map: *mut perf_cpu_map;
    let mut cpu: perf_cpu;
    let mut data: *mut __u64;
    let priv_data = (*auxtrace_info).priv_.as_mut_ptr();

    if priv_size != arm_spe_info_priv_size(itr, (*session).evlist) {
        return -EINVAL;
    }

    if (*evlist__core((*session).evlist)).nr_mmaps == 0 {
        return -EINVAL;
    }

    cpu_map = arm_spe_find_cpus((*session).evlist);
    if cpu_map.is_null() {
        return -EINVAL;
    }

    (*auxtrace_info).type_ = PERF_AUXTRACE_ARM_SPE;
    *priv_data.add(ARM_SPE_HEADER_VERSION) = ARM_SPE_HEADER_CURRENT_VERSION;
    *priv_data.add(ARM_SPE_HEADER_SIZE) = (ARM_SPE_AUXTRACE_PRIV_MAX - ARM_SPE_HEADER_VERSION) as u64;
    *priv_data.add(ARM_SPE_PMU_TYPE_V2) = (*arm_spe_pmu).type_;
    *priv_data.add(ARM_SPE_CPUS_NUM) = perf_cpu_map__nr(cpu_map) as u64;

    offset = ARM_SPE_AUXTRACE_PRIV_MAX;
    i = 0;
    while i < perf_cpu_map__nr(cpu_map) {
        cpu = perf_cpu_map__cpu(cpu_map, i);
        debug_assert!(offset < priv_size);
        data = priv_data.add(offset);
        ret = arm_spe_save_cpu_header(itr, cpu, data);
        if ret < 0 {
            perf_cpu_map__put(cpu_map);
            return ret;
        }
        offset += ret as size_t;
        i += 1;
    }

    ret = 0;
    perf_cpu_map__put(cpu_map);
    ret
}

unsafe extern "C" fn arm_spe_snapshot_resolve_auxtrace_defaults(
    opts: *mut record_opts,
    privileged: bool_,
) {
    /*
     * The default snapshot size is the auxtrace mmap size. If neither auxtrace mmap size nor
     * snapshot size is specified, then the default is 4MiB for privileged users, 128KiB for
     * unprivileged users.
     *
     * The default auxtrace mmap size is 4MiB/page_size for privileged users, 128KiB for
     * unprivileged users. If an unprivileged user does not specify mmap pages, the mmap pages
     * will be reduced from the default 512KiB/page_size to 256KiB/page_size, otherwise the
     * user is likely to get an error as they exceed their mlock limmit.
     */

    /*
     * No size were given to '-S' or '-m,', so go with the default
     */
    if (*opts).auxtrace_snapshot_size == 0 && (*opts).auxtrace_mmap_pages == 0 {
        if privileged {
            (*opts).auxtrace_mmap_pages = MiB(4) / page_size as size_t;
        } else {
            (*opts).auxtrace_mmap_pages = KiB(128) / page_size as size_t;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (KiB(256) / page_size as size_t) as u32;
            }
        }
    } else if (*opts).auxtrace_mmap_pages == 0 && !privileged && (*opts).mmap_pages == UINT_MAX {
        (*opts).mmap_pages = (KiB(256) / page_size as size_t) as u32;
    }

    /*
     * '-m,xyz' was specified but no snapshot size, so make the snapshot size as big as the
     * auxtrace mmap area.
     */
    if (*opts).auxtrace_snapshot_size == 0 {
        (*opts).auxtrace_snapshot_size = (*opts).auxtrace_mmap_pages * page_size as size_t;
    }

    /*
     * '-Sxyz' was specified but no auxtrace mmap area, so make the auxtrace mmap area big
     * enough to fit the requested snapshot size.
     */
    if (*opts).auxtrace_mmap_pages == 0 {
        let mut sz = (*opts).auxtrace_snapshot_size;

        sz = round_up(sz, page_size) / page_size as size_t;
        (*opts).auxtrace_mmap_pages = roundup_pow_of_two(sz);
    }
}

unsafe extern "C" fn arm_spe_pmu__sample_period(arm_spe_pmu: *const perf_pmu) -> __u64 {
    static mut SAMPLE_PERIOD: __u64 = 0;

    if SAMPLE_PERIOD != 0 {
        return SAMPLE_PERIOD;
    }

    /*
     * If kernel driver doesn't advertise a minimum,
     * use max allowable by PMSIDR_EL1.INTERVAL
     */
    if perf_pmu__scan_file(
        arm_spe_pmu,
        cstr(b"caps/min_interval\0"),
        cstr(b"%llu\0"),
        &mut SAMPLE_PERIOD,
    ) != 1 {
        pr_debug(cstr(b"arm_spe driver doesn't advertise a min. interval. Using 4096\n\0"));
        SAMPLE_PERIOD = 4096;
    }
    SAMPLE_PERIOD
}

unsafe extern "C" fn arm_spe_setup_evsel(evsel: *mut evsel, cpus: *mut perf_cpu_map) {
    let mut pa_enable_bit: u64 = 0;

    (*evsel).core.attr.freq = 0;
    (*evsel).core.attr.sample_period = arm_spe_pmu__sample_period((*evsel).pmu);
    (*evsel).needs_auxtrace_mmap = true;

    /*
     * To obtain the auxtrace buffer file descriptor, the auxtrace event
     * must come first.
     */
    evlist__to_front((*evsel).evlist, evsel);

    /*
     * In the case of per-cpu mmaps, sample CPU for AUX event;
     * also enable the timestamp tracing for samples correlation.
     */
    if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        evsel__set_sample_bit(evsel, CPU);
        evsel__set_config_if_unset(evsel, cstr(b"ts_enable\0"), 1);
    }

    /*
     * Set this only so that perf report knows that SPE generates memory info. It has no effect
     * on the opening of the event or the SPE data produced.
     */
    evsel__set_sample_bit(evsel, DATA_SRC);

    /*
     * The PHYS_ADDR flag does not affect the driver behaviour, it is used to
     * inform that the resulting output's SPE samples contain physical addresses
     * where applicable.
     */

    if evsel__get_config_val(evsel, cstr(b"pa_enable\0"), &mut pa_enable_bit) == 0 {
        if pa_enable_bit != 0 {
            evsel__set_sample_bit(evsel, PHYS_ADDR);
        }
    }
}

unsafe extern "C" fn arm_spe_setup_aux_buffer(opts: *mut record_opts) -> c_int {
    let privileged = perf_event_paranoid_check(-1);

    /*
     * we are in snapshot mode.
     */
    if (*opts).auxtrace_snapshot_mode {
        /*
         * Command arguments '-Sxyz' and/or '-m,xyz' are missing, so fill those in with
         * default values.
         */
        if (*opts).auxtrace_snapshot_size == 0 || (*opts).auxtrace_mmap_pages == 0 {
            arm_spe_snapshot_resolve_auxtrace_defaults(opts, privileged);
        }

        /*
         * Snapshot size can't be bigger than the auxtrace area.
         */
        if (*opts).auxtrace_snapshot_size > (*opts).auxtrace_mmap_pages * page_size as size_t {
            pr_err(
                cstr(b"Snapshot size %zu must not be greater than AUX area tracing mmap size %zu\n\0"),
                (*opts).auxtrace_snapshot_size,
                (*opts).auxtrace_mmap_pages * page_size as size_t,
            );
            return -EINVAL;
        }

        /*
         * Something went wrong somewhere - this shouldn't happen.
         */
        if (*opts).auxtrace_snapshot_size == 0 || (*opts).auxtrace_mmap_pages == 0 {
            pr_err(cstr(b"Failed to calculate default snapshot size and/or AUX area tracing mmap pages\n\0"));
            return -EINVAL;
        }

        pr_debug2(
            cstr(b"%sx snapshot size: %zu\n\0"),
            ARM_SPE_PMU_NAME,
            (*opts).auxtrace_snapshot_size,
        );
    }

    /* We are in full trace mode but '-m,xyz' wasn't specified */
    if (*opts).auxtrace_mmap_pages == 0 {
        if privileged {
            (*opts).auxtrace_mmap_pages = MiB(4) / page_size as size_t;
        } else {
            (*opts).auxtrace_mmap_pages = KiB(128) / page_size as size_t;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (KiB(256) / page_size as size_t) as u32;
            }
        }
    }

    /* Validate auxtrace_mmap_pages */
    if (*opts).auxtrace_mmap_pages != 0 {
        let sz = (*opts).auxtrace_mmap_pages * page_size as size_t;
        let min_sz = KiB(8);

        if sz < min_sz || !is_power_of_2(sz) {
            pr_err(
                cstr(b"Invalid mmap size for ARM SPE: must be at least %zuKiB and a power of 2\n\0"),
                min_sz / 1024,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn arm_spe_setup_tracking_event(
    evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let mut err: c_int;
    let tracking_evsel: *mut evsel;
    let cpus = (*evlist__core(evlist)).user_requested_cpus;

    /* Add dummy event to keep tracking */
    err = parse_event(evlist, cstr(b"dummy:u\0"));
    if err != 0 {
        return err;
    }

    tracking_evsel = evlist__last(evlist);
    evlist__set_tracking_event(evlist, tracking_evsel);

    (*tracking_evsel).core.attr.freq = 0;
    (*tracking_evsel).core.attr.sample_period = 1;

    /* In per-cpu case, always need the time of mmap events etc */
    if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        evsel__set_sample_bit(tracking_evsel, TIME);
        evsel__set_sample_bit(tracking_evsel, CPU);

        /* also track task context switch */
        if !record_opts__no_switch_events(opts) {
            (*tracking_evsel).core.attr.context_switch = 1;
        }
    }

    0
}

unsafe extern "C" fn arm_spe_recording_options(
    itr: *mut auxtrace_record,
    evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let sper = container_of_itr(itr);
    let mut evsel: *mut evsel;
    let cpus = (*evlist__core(evlist)).user_requested_cpus;
    let mut discard = false;
    let mut err: c_int;
    let mut discard_bit: u64 = 0;

    (*sper).evlist = evlist;

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if evsel__is_aux_event(evsel) {
            if !strstarts((*(*evsel).pmu).name, ARM_SPE_PMU_NAME) {
                pr_err(cstr(b"Found unexpected auxtrace event: %s\n\0"), (*(*evsel).pmu).name);
                return -EINVAL;
            }
            (*opts).full_auxtrace = true;

            if (*opts).user_freq != UINT_MAX || arm_spe_is_set_freq(evsel) {
                pr_err(
                    cstr(b"Arm SPE: Frequency is not supported. Set period with -c option or PMU parameter (-e %s/period=NUM/).\n\0"),
                    (*(*evsel).pmu).name,
                );
                return -EINVAL;
            }
        }
        evsel = evlist__next(evsel);
    }

    if !(*opts).full_auxtrace {
        return 0;
    }

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        let next = evlist__next(evsel);
        if evsel__is_aux_event(evsel) {
            arm_spe_setup_evsel(evsel, cpus);
            if evsel__config_exists(evsel, cstr(b"discard\0"))
                && evsel__get_config_val(evsel, cstr(b"discard\0"), &mut discard_bit) == 0
            {
                discard = discard_bit != 0;
            }
        }
        evsel = next;
    }

    if discard {
        return 0;
    }

    err = arm_spe_setup_aux_buffer(opts);
    if err != 0 {
        return err;
    }

    arm_spe_setup_tracking_event(evlist, opts)
}

unsafe extern "C" fn arm_spe_parse_snapshot_options(
    _itr: *mut auxtrace_record,
    opts: *mut record_opts,
    str_: *const c_char,
) -> c_int {
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

    0
}

unsafe extern "C" fn arm_spe_snapshot_start(itr: *mut auxtrace_record) -> c_int {
    let ptr = container_of_itr(itr);
    let mut evsel: *mut evsel;
    let mut ret: c_int = -EINVAL;

    evsel = evlist__first((*ptr).evlist);
    while !evsel.is_null() {
        if evsel__is_aux_event(evsel) {
            ret = evsel__disable(evsel);
            if ret < 0 {
                return ret;
            }
        }
        evsel = evlist__next(evsel);
    }
    ret
}

unsafe extern "C" fn arm_spe_snapshot_finish(itr: *mut auxtrace_record) -> c_int {
    let ptr = container_of_itr(itr);
    let mut evsel: *mut evsel;
    let mut ret: c_int = -EINVAL;

    evsel = evlist__first((*ptr).evlist);
    while !evsel.is_null() {
        if evsel__is_aux_event(evsel) {
            ret = evsel__enable(evsel);
            if ret < 0 {
                return ret;
            }
        }
        evsel = evlist__next(evsel);
    }
    ret
}

unsafe extern "C" fn arm_spe_alloc_wrapped_array(ptr: *mut arm_spe_recording, idx: c_int) -> c_int {
    let mut wrapped: *mut bool_;
    let cnt = (*ptr).wrapped_cnt;
    let new_cnt: c_int;
    let mut i: c_int;

    /*
     * No need to allocate, so return early.
     */
    if idx < cnt {
        return 0;
    }

    /*
     * Make ptr->wrapped as big as idx.
     */
    new_cnt = idx + 1;

    /*
     * Free'ed in arm_spe_recording_free().
     */
    wrapped = reallocarray((*ptr).wrapped as *mut c_void, new_cnt as size_t, size_of::<bool_>())
        as *mut bool_;
    if wrapped.is_null() {
        return -ENOMEM;
    }

    /*
     * init new allocated values.
     */
    i = cnt;
    while i < new_cnt {
        *wrapped.add(i as size_t) = false;
        i += 1;
    }

    (*ptr).wrapped_cnt = new_cnt;
    (*ptr).wrapped = wrapped;

    0
}

unsafe extern "C" fn arm_spe_buffer_has_wrapped(
    buffer: *mut u8,
    buffer_size: size_t,
    head: u64,
) -> bool_ {
    let mut i: u64;
    let mut watermark: u64;
    let buf = buffer as *mut u64;
    let mut buf_size = buffer_size;

    /*
     * Defensively handle the case where head might be continually increasing - if its value is
     * equal or greater than the size of the ring buffer, then we can safely determine it has
     * wrapped around. Otherwise, continue to detect if head might have wrapped.
     */
    if head >= buffer_size as u64 {
        return true;
    }

    /*
     * We want to look the very last 512 byte (chosen arbitrarily) in the ring buffer.
     */
    watermark = buf_size as u64 - 512;

    /*
     * The value of head is somewhere within the size of the ring buffer. This can be that there
     * hasn't been enough data to fill the ring buffer yet or the trace time was so long that
     * head has numerically wrapped around.  To find we need to check if we have data at the
     * very end of the ring buffer.  We can reliably do this because mmap'ed pages are zeroed
     * out and there is a fresh mapping with every new session.
     */

    /*
     * head is less than 512 byte from the end of the ring buffer.
     */
    if head > watermark {
        watermark = head;
    }

    /*
     * Speed things up by using 64 bit transactions (see "u64 *buf" above)
     */
    watermark /= size_of::<u64>() as u64;
    buf_size /= size_of::<u64>();

    /*
     * If we find trace data at the end of the ring buffer, head has been there and has
     * numerically wrapped around at least once.
     */
    i = watermark;
    while i < buf_size as u64 {
        if *buf.add(i as size_t) != 0 {
            return true;
        }
        i += 1;
    }

    false
}

unsafe extern "C" fn arm_spe_find_snapshot(
    itr: *mut auxtrace_record,
    idx: c_int,
    mm: *mut auxtrace_mmap,
    data: *mut u8,
    head: *mut u64,
    old: *mut u64,
) -> c_int {
    let mut err: c_int;
    let mut wrapped: bool_;
    let ptr = container_of_itr(itr);

    /*
     * Allocate memory to keep track of wrapping if this is the first
     * time we deal with this *mm.
     */
    if idx >= (*ptr).wrapped_cnt {
        err = arm_spe_alloc_wrapped_array(ptr, idx);
        if err != 0 {
            return err;
        }
    }

    /*
     * Check to see if *head has wrapped around.  If it hasn't only the
     * amount of data between *head and *old is snapshot'ed to avoid
     * bloating the perf.data file with zeros.  But as soon as *head has
     * wrapped around the entire size of the AUX ring buffer it taken.
     */
    wrapped = *(*ptr).wrapped.add(idx as size_t);
    if !wrapped && arm_spe_buffer_has_wrapped(data, (*mm).len, *head) {
        wrapped = true;
        *(*ptr).wrapped.add(idx as size_t) = true;
    }

    pr_debug3(
        cstr(b"%s: mmap index %d old head %zu new head %zu size %zu\n\0"),
        cstr(b"arm_spe_find_snapshot\0"),
        idx,
        *old as size_t,
        *head as size_t,
        (*mm).len,
    );

    /*
     * No wrap has occurred, we can just use *head and *old.
     */
    if !wrapped {
        return 0;
    }

    /*
     * *head has wrapped around - adjust *head and *old to pickup the
     * entire content of the AUX buffer.
     */
    if *head >= (*mm).len as u64 {
        *old = *head - (*mm).len as u64;
    } else {
        *head += (*mm).len as u64;
        *old = *head - (*mm).len as u64;
    }

    0
}

unsafe extern "C" fn arm_spe_reference(_itr: *mut auxtrace_record) -> u64 {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };

    clock_gettime(CLOCK_MONOTONIC_RAW, &mut ts);

    (ts.tv_sec as u64) ^ (ts.tv_nsec as u64)
}

unsafe extern "C" fn arm_spe_recording_free(itr: *mut auxtrace_record) {
    let sper = container_of_itr(itr);

    zfree(&mut (*sper).wrapped);
    free(sper as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_recording_init(
    err: *mut c_int,
    arm_spe_pmu: *mut perf_pmu,
) -> *mut auxtrace_record {
    let sper: *mut arm_spe_recording;

    if arm_spe_pmu.is_null() {
        *err = -ENODEV;
        return ptr::null_mut();
    }

    sper = zalloc(size_of::<arm_spe_recording>()) as *mut arm_spe_recording;
    if sper.is_null() {
        *err = -ENOMEM;
        return ptr::null_mut();
    }

    (*sper).arm_spe_pmu = arm_spe_pmu;
    (*sper).itr.snapshot_start = Some(arm_spe_snapshot_start);
    (*sper).itr.snapshot_finish = Some(arm_spe_snapshot_finish);
    (*sper).itr.find_snapshot = Some(arm_spe_find_snapshot);
    (*sper).itr.parse_snapshot_options = Some(arm_spe_parse_snapshot_options);
    (*sper).itr.recording_options = Some(arm_spe_recording_options);
    (*sper).itr.info_priv_size = Some(arm_spe_info_priv_size);
    (*sper).itr.info_fill = Some(arm_spe_info_fill);
    (*sper).itr.free = Some(arm_spe_recording_free);
    (*sper).itr.reference = Some(arm_spe_reference);
    (*sper).itr.read_finish = Some(auxtrace_record__read_finish);
    (*sper).itr.alignment = 0;

    *err = 0;
    &mut (*sper).itr
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_pmu_default_config(
    arm_spe_pmu: *const perf_pmu,
    attr: *mut perf_event_attr,
) {
    (*attr).sample_period = arm_spe_pmu__sample_period(arm_spe_pmu);
}
