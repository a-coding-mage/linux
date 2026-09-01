// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// Translated from perf/arch/arm/util/cs-etm.c.
// C include dependencies are expected to be supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type size_t = usize;
type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const BUFSIZ: usize = 8192;
const PATH_MAX: usize = 4096;
const UINT_MAX: u32 = u32::MAX;
const SIZE_MAX: c_ulonglong = usize::MAX as c_ulonglong;

const fn genmask(h: u32, l: u32) -> u64 {
    ((!0u64) << l) & ((!0u64) >> (63 - h))
}

const fn bmval(x: u64, l: u32, h: u32) -> u64 {
    (x & genmask(h, l)) >> l
}

const fn kib(x: usize) -> usize {
    x * 1024
}

const fn mib(x: usize) -> usize {
    x * 1024 * 1024
}

unsafe extern "C" {
    static page_size: usize;
    static mut errno: c_int;

    static CORESIGHT_ETM_PMU_NAME: *const c_char;
    static __perf_cs_ete_magic: u64;
    static __perf_cs_etmv4_magic: u64;
    static __perf_cs_etmv3_magic: u64;

    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn rand() -> c_int;
    fn free(ptr: *mut c_void);
    fn zalloc(size: usize) -> *mut c_void;
    fn assert(expr: bool);

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;

    fn perf_event_paranoid_check(level: c_int) -> bool;
    fn perf_can_record_switch_events() -> bool;
    fn record_opts__no_switch_events(opts: *mut record_opts) -> bool;

    fn evsel__get_config_val(evsel: *mut evsel, name: *const c_char, val: *mut u64) -> c_int;
    fn evsel__set_config_if_unset(evsel: *mut evsel, name: *const c_char, val: u64);
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__disable(evsel: *mut evsel) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;

    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;

    fn perf_cpu_map__has_any_cpu(map: *mut perf_cpu_map) -> bool;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__intersect(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__is_any_cpu_or_is_empty(map: *mut perf_cpu_map) -> bool;
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> u64;
    fn perf_cpu_map__has(map: *mut perf_cpu_map, cpu: perf_cpu) -> bool;
    fn perf_cpu_map__for_each_cpu_next(map: *mut perf_cpu_map, idx: *mut c_uint, cpu: *mut perf_cpu) -> bool;
    fn perf_cpu_map__for_each_cpu_skip_any_next(map: *mut perf_cpu_map, idx: *mut c_uint, cpu: *mut perf_cpu) -> bool;

    fn perf_pmu__scan_file(pmu: *mut perf_pmu, path: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn perf_pmu__file_exists(pmu: *mut perf_pmu, path: *const c_char) -> bool;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;

    fn roundup_pow_of_two(x: usize) -> usize;
    fn auxtrace_record__read_finish(itr: *mut auxtrace_record, idx: c_int) -> c_int;
}

#[repr(C)]
pub struct auxtrace_record {
    parse_snapshot_options: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut record_opts, *const c_char) -> c_int>,
    recording_options: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int>,
    info_priv_size: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t>,
    info_fill: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut perf_session, *mut perf_record_auxtrace_info, size_t) -> c_int>,
    snapshot_start: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    snapshot_finish: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    read_finish: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int) -> c_int>,
}

#[repr(C)]
pub struct cs_etm_recording {
    itr: auxtrace_record,
    cs_etm_pmu: *mut perf_pmu,
    evlist: *mut evlist,
    snapshot_mode: bool,
    snapshot_size: size_t,
}

#[repr(C)]
pub struct perf_pmu {
    type_: u32,
}

#[repr(C)]
pub struct perf_cpu {
    cpu: c_int,
}

#[repr(C)]
pub struct perf_event_attr {
    type_: u32,
    sample_period: u64,
    freq: u64,
}

#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    core: evsel_core,
    evlist: *mut evlist,
    config_terms: list_head,
    needs_auxtrace_mmap: bool,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct evsel_config_term {
    list: list_head,
    type_: c_int,
    val: evsel_config_term_val,
}

#[repr(C)]
pub union evsel_config_term_val {
    str_: *mut c_char,
}

#[repr(C)]
pub struct evlist_core {
    user_requested_cpus: *mut perf_cpu_map,
    nr_mmaps: c_uint,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    auxtrace_snapshot_mode: bool,
    auxtrace_snapshot_size: size_t,
    auxtrace_mmap_pages: size_t,
    mmap_pages: u32,
    record_switch_events: bool,
    full_auxtrace: bool,
    use_clockid: bool,
    sample_time_set: bool,
}

#[repr(C)]
pub struct perf_session {
    evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    type_: u32,
    priv_: [u64; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

const CS_ETM_PRIV_MAX: usize = 4;
const CS_ETM_ETMCCER: usize = 2;
const CS_ETM_ETMIDR: usize = 3;
const CS_ETM_ETMCR: usize = 0;
const CS_ETM_ETMTRACEIDR: usize = 1;

const CS_ETMV4_TRCIDR0: usize = 2;
const CS_ETMV4_TRCIDR1: usize = 3;
const CS_ETMV4_TRCIDR2: usize = 4;
const CS_ETMV4_TRCIDR8: usize = 5;
const CS_ETMV4_TRCAUTHSTATUS: usize = 6;
const CS_ETMV4_TS_SOURCE: usize = 7;
const CS_ETMV4_TRCCONFIGR: usize = 0;
const CS_ETMV4_TRCTRACEIDR: usize = 1;
const CS_ETMV4_PRIV_MAX: usize = 8;

const CS_ETE_TRCIDR0: usize = 2;
const CS_ETE_TRCIDR1: usize = 3;
const CS_ETE_TRCIDR2: usize = 4;
const CS_ETE_TRCIDR8: usize = 5;
const CS_ETE_TRCAUTHSTATUS: usize = 6;
const CS_ETE_TRCDEVARCH: usize = 7;
const CS_ETE_TS_SOURCE: usize = 8;
const CS_ETE_TRCCONFIGR: usize = 0;
const CS_ETE_TRCTRACEIDR: usize = 1;
const CS_ETE_PRIV_MAX: usize = 9;

const CS_ETM_HEADER_SIZE: usize = 4;
const CS_ETE_PRIV_SIZE: usize = CS_ETE_PRIV_MAX;
const CS_ETMV4_PRIV_SIZE: usize = CS_ETMV4_PRIV_MAX;
const CS_ETMV3_PRIV_SIZE: usize = CS_ETM_PRIV_MAX;
const CS_ETM_COMMON_BLK_MAX_V1: u32 = 2;
const CS_ETM_MAGIC: usize = 0;
const CS_ETM_CPU: usize = 1;
const CS_ETM_NR_TRC_PARAMS: usize = 2;
const CS_HEADER_VERSION: usize = 0;
const CS_HEADER_CURRENT_VERSION: u64 = 1;
const CS_PMU_TYPE_CPUS: usize = 1;
const CS_ETM_SNAPSHOT: usize = 2;
const PERF_AUXTRACE_CS_ETM: u32 = 3;

const ETMCR_CYC_ACC: u64 = 1 << 12;
const ETMCR_TIMESTAMP_EN: u64 = 1 << 28;
const ETMCR_RETURN_STACK: u64 = 1 << 29;
const TRCCONFIGR_CCI: u64 = 1 << 4;
const TRCCONFIGR_CID: u64 = 1 << 6;
const TRCCONFIGR_TS: u64 = 1 << 11;
const TRCCONFIGR_RS: u64 = 1 << 12;
const TRCCONFIGR_VMID: u64 = 1 << 7;
const TRCCONFIGR_VMIDOPT: u64 = 1 << 15;
const TRCCONFIGR_BB: u64 = 1 << 3;
const EVSEL__CONFIG_TERM_DRV_CFG: c_int = 1;
const CPU: c_int = 0;
const TIME: c_int = 1;

static metadata_etmv3_ro: [*const c_char; CS_ETM_PRIV_MAX] = [
    ptr::null(),
    ptr::null(),
    b"mgmt/etmccer\0".as_ptr() as *const c_char,
    b"mgmt/etmidr\0".as_ptr() as *const c_char,
];

static metadata_etmv4_ro: [*const c_char; 8] = [
    ptr::null(),
    ptr::null(),
    b"trcidr/trcidr0\0".as_ptr() as *const c_char,
    b"trcidr/trcidr1\0".as_ptr() as *const c_char,
    b"trcidr/trcidr2\0".as_ptr() as *const c_char,
    b"trcidr/trcidr8\0".as_ptr() as *const c_char,
    b"mgmt/trcauthstatus\0".as_ptr() as *const c_char,
    b"ts_source\0".as_ptr() as *const c_char,
];

static metadata_ete_ro: [*const c_char; 9] = [
    ptr::null(),
    ptr::null(),
    b"trcidr/trcidr0\0".as_ptr() as *const c_char,
    b"trcidr/trcidr1\0".as_ptr() as *const c_char,
    b"trcidr/trcidr2\0".as_ptr() as *const c_char,
    b"trcidr/trcidr8\0".as_ptr() as *const c_char,
    b"mgmt/trcauthstatus\0".as_ptr() as *const c_char,
    b"mgmt/trcdevarch\0".as_ptr() as *const c_char,
    b"ts_source\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cs_etm_version {
    CS_NOT_PRESENT,
    CS_ETMV3,
    CS_ETMV4,
    CS_ETE,
}

unsafe fn cs_etm_ptr(itr: *mut auxtrace_record) -> *mut cs_etm_recording {
    itr as *mut cs_etm_recording
}

unsafe fn round_up(sz: usize, align: usize) -> usize {
    ((sz + align - 1) / align) * align
}

unsafe fn coresight_legacy_cpu_trace_id(cpu: c_int) -> u64 {
    cpu as u64 + 1
}

unsafe fn evlist_for_each_entry(_evlist: *mut evlist, _f: impl FnMut(*mut evsel) -> bool) {
    /* External list traversal macro from C; supplied by surrounding bindings. */
}

unsafe fn config_terms_for_each(_head: *mut list_head, _f: impl FnMut(*mut evsel_config_term) -> bool) {
    /* External list traversal macro from C; supplied by surrounding bindings. */
}

unsafe extern "C" fn cs_etm_get_version(cs_etm_pmu: *mut perf_pmu, cpu: perf_cpu) -> cs_etm_version {
    if cs_etm_is_ete(cs_etm_pmu, cpu) {
        cs_etm_version::CS_ETE
    } else if cs_etm_pmu_path_exists(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR0]) {
        cs_etm_version::CS_ETMV4
    } else if cs_etm_pmu_path_exists(cs_etm_pmu, cpu, metadata_etmv3_ro[CS_ETM_ETMCCER]) {
        cs_etm_version::CS_ETMV3
    } else {
        cs_etm_version::CS_NOT_PRESENT
    }
}

unsafe extern "C" fn cs_etm_validate_context_id(cs_etm_pmu: *mut perf_pmu, evsel: *mut evsel, cpu: perf_cpu) -> c_int {
    let mut err: c_int;
    let mut ctxt: u64 = 0;
    let mut ctxt1: u64 = 0;
    let mut ctxt2: u64 = 0;
    let mut trcidr2: u64 = 0;

    evsel__get_config_val(evsel, b"contextid\0".as_ptr() as *const c_char, &mut ctxt);
    evsel__get_config_val(evsel, b"contextid1\0".as_ptr() as *const c_char, &mut ctxt1);
    evsel__get_config_val(evsel, b"contextid2\0".as_ptr() as *const c_char, &mut ctxt2);

    if ctxt == 0 && ctxt1 == 0 && ctxt2 == 0 {
        return 0;
    }

    /* Not supported in etmv3 */
    if cs_etm_get_version(cs_etm_pmu, cpu) == cs_etm_version::CS_ETMV3 {
        pr_err(b"%s: contextid not supported in ETMv3, disable with %s/contextid=0/\n\0".as_ptr() as *const c_char,
               CORESIGHT_ETM_PMU_NAME, CORESIGHT_ETM_PMU_NAME);
        return -EINVAL;
    }

    /* Get a handle on TRCIDR2 */
    err = cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR2], &mut trcidr2);
    if err != 0 {
        return err;
    }

    if ctxt1 != 0 {
        if bmval(trcidr2, 5, 9) != 0x4 {
            pr_err(b"%s: CONTEXTIDR_EL1 isn't supported, disable with %s/contextid1=0/\n\0".as_ptr() as *const c_char,
                   CORESIGHT_ETM_PMU_NAME, CORESIGHT_ETM_PMU_NAME);
            return -EINVAL;
        }
    }

    if ctxt2 != 0 {
        if bmval(trcidr2, 29, 30) == 0 || bmval(trcidr2, 10, 14) < 4 {
            pr_err(b"%s: CONTEXTIDR_EL2 isn't supported, disable with %s/contextid2=0/\n\0".as_ptr() as *const c_char,
                   CORESIGHT_ETM_PMU_NAME, CORESIGHT_ETM_PMU_NAME);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn cs_etm_validate_timestamp(cs_etm_pmu: *mut perf_pmu, evsel: *mut evsel, cpu: perf_cpu) -> c_int {
    let mut err: c_int;
    let mut val: u64 = 0;
    let mut trcidr0: u64 = 0;

    evsel__get_config_val(evsel, b"timestamp\0".as_ptr() as *const c_char, &mut val);
    if val == 0 {
        return 0;
    }

    if cs_etm_get_version(cs_etm_pmu, cpu) == cs_etm_version::CS_ETMV3 {
        pr_err(b"%s: timestamp not supported in ETMv3, disable with %s/timestamp=0/\n\0".as_ptr() as *const c_char,
               CORESIGHT_ETM_PMU_NAME, CORESIGHT_ETM_PMU_NAME);
        return -EINVAL;
    }

    /* Get a handle on TRCIRD0 */
    err = cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR0], &mut trcidr0);
    if err != 0 {
        return err;
    }

    trcidr0 &= genmask(28, 24);
    if trcidr0 == 0 {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn cs_etm_get_pmu(itr: *mut auxtrace_record) -> *mut perf_pmu {
    let ptr = cs_etm_ptr(itr);
    (*ptr).cs_etm_pmu
}

unsafe extern "C" fn cs_etm_validate_config(cs_etm_pmu: *mut perf_pmu, evsel: *mut evsel) -> c_int {
    let mut idx: c_uint = 0;
    let mut err: c_int = 0;
    let event_cpus = (*evlist__core((*evsel).evlist)).user_requested_cpus;
    let intersect_cpus: *mut perf_cpu_map;
    let mut cpu = perf_cpu { cpu: 0 };

    if !perf_cpu_map__has_any_cpu(event_cpus) {
        let online_cpus = perf_cpu_map__new_online_cpus();
        intersect_cpus = perf_cpu_map__intersect(event_cpus, online_cpus);
        perf_cpu_map__put(online_cpus);
    } else {
        intersect_cpus = perf_cpu_map__new_online_cpus();
    }

    while perf_cpu_map__for_each_cpu_skip_any_next(intersect_cpus, &mut idx, &mut cpu) {
        if cs_etm_get_version(cs_etm_pmu, cpu) == cs_etm_version::CS_NOT_PRESENT {
            pr_err(b"%s: Not found on CPU %d. Check hardware and firmware support and that all Coresight drivers are loaded\n\0".as_ptr() as *const c_char,
                   CORESIGHT_ETM_PMU_NAME, cpu.cpu);
            return -EINVAL;
        }
        err = cs_etm_validate_context_id(cs_etm_pmu, evsel, cpu);
        if err != 0 {
            break;
        }

        err = cs_etm_validate_timestamp(cs_etm_pmu, evsel, cpu);
        if err != 0 {
            break;
        }
    }

    perf_cpu_map__put(intersect_cpus);
    err
}

unsafe extern "C" fn cs_etm_parse_snapshot_options(itr: *mut auxtrace_record, opts: *mut record_opts, str_: *const c_char) -> c_int {
    let ptr = cs_etm_ptr(itr);
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

unsafe extern "C" fn cs_etm_set_sink_attr(pmu: *mut perf_pmu, evsel: *mut evsel) -> c_int {
    let mut msg = [0 as c_char; BUFSIZ];
    let mut path = [0 as c_char; PATH_MAX];
    let mut ret: c_int = 0;

    config_terms_for_each(&mut (*evsel).config_terms, |term| {
        if (*term).type_ != EVSEL__CONFIG_TERM_DRV_CFG {
            return true;
        }

        let sink = (*term).val.str_;
        snprintf(path.as_mut_ptr(), PATH_MAX, b"sinks/%s\0".as_ptr() as *const c_char, sink);

        let mut hash: u32 = 0;
        ret = perf_pmu__scan_file(pmu, path.as_ptr(), b"%x\0".as_ptr() as *const c_char, &mut hash);
        if ret != 1 {
            if errno == ENOENT {
                pr_err(b"Couldn't find sink \"%s\" on event %s\nMissing kernel or device support?\n\nHint: An appropriate sink will be picked automatically if one isn't specified.\n\0".as_ptr() as *const c_char,
                       sink, evsel__name(evsel));
            } else {
                pr_err(b"Failed to set sink \"%s\" on event %s with %d (%s)\n\0".as_ptr() as *const c_char,
                       sink, evsel__name(evsel), errno, str_error_r(errno, msg.as_mut_ptr(), msg.len()));
            }
            return false;
        }

        evsel__set_config_if_unset(evsel, b"sinkid\0".as_ptr() as *const c_char, hash as u64);
        ret = 0;
        false
    });

    ret
}

unsafe extern "C" fn cs_etm_get_evsel(evlist: *mut evlist, cs_etm_pmu: *mut perf_pmu) -> *mut evsel {
    let mut found: *mut evsel = ptr::null_mut();
    evlist_for_each_entry(evlist, |evsel| {
        if (*evsel).core.attr.type_ == (*cs_etm_pmu).type_ {
            found = evsel;
            false
        } else {
            true
        }
    });
    found
}

unsafe extern "C" fn cs_etm_recording_options(itr: *mut auxtrace_record, evlist: *mut evlist, opts: *mut record_opts) -> c_int {
    let mut ret: c_int;
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    let mut evsel: *mut evsel = ptr::null_mut();
    let mut cs_etm_evsel: *mut evsel = ptr::null_mut();
    let cpus = (*evlist__core(evlist)).user_requested_cpus;
    let privileged = perf_event_paranoid_check(-1);
    let mut err: c_int = 0;

    evlist_for_each_entry(evlist, |e| {
        if (*e).core.attr.type_ == (*cs_etm_pmu).type_ {
            if !cs_etm_evsel.is_null() {
                pr_err(b"There may be only one %s event\n\0".as_ptr() as *const c_char, CORESIGHT_ETM_PMU_NAME);
                err = -EINVAL;
                return false;
            }
            cs_etm_evsel = e;
        }
        true
    });
    if err != 0 {
        return err;
    }

    if cs_etm_evsel.is_null() {
        return 0;
    }

    (*ptr).evlist = evlist;
    (*ptr).snapshot_mode = (*opts).auxtrace_snapshot_mode;

    if !record_opts__no_switch_events(opts) && perf_can_record_switch_events() {
        (*opts).record_switch_events = true;
    }

    (*cs_etm_evsel).needs_auxtrace_mmap = true;
    (*opts).full_auxtrace = true;

    ret = cs_etm_set_sink_attr(cs_etm_pmu, cs_etm_evsel);
    if ret != 0 {
        return ret;
    }

    if (*opts).use_clockid {
        pr_err(b"Cannot use clockid (-k option) with %s\n\0".as_ptr() as *const c_char, CORESIGHT_ETM_PMU_NAME);
        return -EINVAL;
    }

    if (*opts).auxtrace_snapshot_mode {
        if (*opts).auxtrace_snapshot_size == 0 && (*opts).auxtrace_mmap_pages == 0 {
            if privileged {
                (*opts).auxtrace_mmap_pages = mib(4) / page_size;
            } else {
                (*opts).auxtrace_mmap_pages = kib(128) / page_size;
                if (*opts).mmap_pages == UINT_MAX {
                    (*opts).mmap_pages = (kib(256) / page_size) as u32;
                }
            }
        } else if (*opts).auxtrace_mmap_pages == 0 && !privileged && (*opts).mmap_pages == UINT_MAX {
            (*opts).mmap_pages = (kib(256) / page_size) as u32;
        }

        if (*opts).auxtrace_snapshot_size == 0 {
            (*opts).auxtrace_snapshot_size = (*opts).auxtrace_mmap_pages * page_size;
        }

        if (*opts).auxtrace_mmap_pages == 0 {
            let mut sz = (*opts).auxtrace_snapshot_size;
            sz = round_up(sz, page_size) / page_size;
            (*opts).auxtrace_mmap_pages = roundup_pow_of_two(sz);
        }

        if (*opts).auxtrace_snapshot_size > (*opts).auxtrace_mmap_pages * page_size {
            pr_err(b"Snapshot size %zu must not be greater than AUX area tracing mmap size %zu\n\0".as_ptr() as *const c_char,
                   (*opts).auxtrace_snapshot_size, (*opts).auxtrace_mmap_pages * page_size);
            return -EINVAL;
        }

        if (*opts).auxtrace_snapshot_size == 0 || (*opts).auxtrace_mmap_pages == 0 {
            pr_err(b"Failed to calculate default snapshot size and/or AUX area tracing mmap pages\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*opts).auxtrace_mmap_pages == 0 {
        if privileged {
            (*opts).auxtrace_mmap_pages = mib(4) / page_size;
        } else {
            (*opts).auxtrace_mmap_pages = kib(128) / page_size;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (kib(256) / page_size) as u32;
            }
        }
    }

    if (*opts).auxtrace_snapshot_mode {
        pr_debug2(b"%s snapshot size: %zu\n\0".as_ptr() as *const c_char, CORESIGHT_ETM_PMU_NAME, (*opts).auxtrace_snapshot_size);
    }

    evlist__to_front(evlist, cs_etm_evsel);
    evsel__set_sample_bit(cs_etm_evsel, CPU);

    if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        evsel__set_config_if_unset(cs_etm_evsel, b"timestamp\0".as_ptr() as *const c_char, 1);
        evsel__set_config_if_unset(cs_etm_evsel, b"contextid\0".as_ptr() as *const c_char, 1);
    }

    if (*opts).sample_time_set {
        evsel__set_config_if_unset(cs_etm_evsel, b"timestamp\0".as_ptr() as *const c_char, 1);
    }

    err = parse_event(evlist, b"dummy:u\0".as_ptr() as *const c_char);
    if err != 0 {
        return err;
    }
    evsel = evlist__last(evlist);
    evlist__set_tracking_event(evlist, evsel);
    (*evsel).core.attr.freq = 0;
    (*evsel).core.attr.sample_period = 1;

    if !perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        evsel__set_sample_bit(evsel, TIME);
    }

    cs_etm_validate_config(cs_etm_pmu, cs_etm_evsel)
}

unsafe extern "C" fn cs_etm_synth_etmcr(itr: *mut auxtrace_record) -> u64 {
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    let evsel = cs_etm_get_evsel((*ptr).evlist, cs_etm_pmu);
    let mut etmcr: u64 = 0;
    let mut val: u64 = 0;

    if evsel.is_null() {
        return 0;
    }

    if evsel__get_config_val(evsel, b"cycacc\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 {
        etmcr |= ETMCR_CYC_ACC;
    }
    if evsel__get_config_val(evsel, b"timestamp\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 {
        etmcr |= ETMCR_TIMESTAMP_EN;
    }
    if evsel__get_config_val(evsel, b"retstack\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 {
        etmcr |= ETMCR_RETURN_STACK;
    }

    etmcr
}

unsafe extern "C" fn cs_etmv4_synth_trcconfigr(itr: *mut auxtrace_record) -> u64 {
    let mut trcconfigr: u64 = 0;
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    let evsel = cs_etm_get_evsel((*ptr).evlist, cs_etm_pmu);
    let mut val: u64 = 0;

    if evsel.is_null() {
        return 0;
    }

    if evsel__get_config_val(evsel, b"cycacc\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_CCI; }
    if evsel__get_config_val(evsel, b"contextid1\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_CID; }
    if evsel__get_config_val(evsel, b"timestamp\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_TS; }
    if evsel__get_config_val(evsel, b"retstack\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_RS; }
    if evsel__get_config_val(evsel, b"contextid2\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_VMID | TRCCONFIGR_VMIDOPT; }
    if evsel__get_config_val(evsel, b"branch_broadcast\0".as_ptr() as *const c_char, &mut val) == 0 && val != 0 { trcconfigr |= TRCCONFIGR_BB; }

    trcconfigr
}

unsafe extern "C" fn cs_etm_info_priv_size(itr: *mut auxtrace_record, evlist: *mut evlist) -> size_t {
    let mut idx: c_uint = 0;
    let mut etmv3: usize = 0;
    let mut etmv4: usize = 0;
    let mut ete: usize = 0;
    let event_cpus = (*evlist__core(evlist)).user_requested_cpus;
    let intersect_cpus: *mut perf_cpu_map;
    let mut cpu = perf_cpu { cpu: 0 };
    let cs_etm_pmu = cs_etm_get_pmu(itr);

    if !perf_cpu_map__has_any_cpu(event_cpus) {
        let online_cpus = perf_cpu_map__new_online_cpus();
        intersect_cpus = perf_cpu_map__intersect(event_cpus, online_cpus);
        perf_cpu_map__put(online_cpus);
    } else {
        intersect_cpus = perf_cpu_map__new_online_cpus();
    }

    while perf_cpu_map__for_each_cpu_skip_any_next(intersect_cpus, &mut idx, &mut cpu) {
        let v = cs_etm_get_version(cs_etm_pmu, cpu);
        ete += (v == cs_etm_version::CS_ETE) as usize;
        etmv4 += (v == cs_etm_version::CS_ETMV4) as usize;
        etmv3 += (v == cs_etm_version::CS_ETMV3) as usize;
    }
    perf_cpu_map__put(intersect_cpus);

    CS_ETM_HEADER_SIZE + (ete * CS_ETE_PRIV_SIZE) + (etmv4 * CS_ETMV4_PRIV_SIZE) + (etmv3 * CS_ETMV3_PRIV_SIZE)
}

unsafe extern "C" fn cs_etm_get_ro(pmu: *mut perf_pmu, cpu: perf_cpu, path_in: *const c_char, val: *mut u64) -> c_int {
    let mut pmu_path = [0 as c_char; PATH_MAX];
    snprintf(pmu_path.as_mut_ptr(), PATH_MAX, b"cpu%d/%s\0".as_ptr() as *const c_char, cpu.cpu, path_in);
    let scan = perf_pmu__scan_file(pmu, pmu_path.as_ptr(), b"%llx\0".as_ptr() as *const c_char, val);
    if scan != 1 {
        pr_err(b"%s: error reading: %s\n\0".as_ptr() as *const c_char, b"cs_etm_get_ro\0".as_ptr() as *const c_char, pmu_path.as_ptr());
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn cs_etm_get_ro_signed(pmu: *mut perf_pmu, cpu: perf_cpu, path_in: *const c_char, out_val: *mut u64) -> c_int {
    let mut pmu_path = [0 as c_char; PATH_MAX];
    let mut val: c_int = 0;
    snprintf(pmu_path.as_mut_ptr(), PATH_MAX, b"cpu%d/%s\0".as_ptr() as *const c_char, cpu.cpu, path_in);
    let scan = perf_pmu__scan_file(pmu, pmu_path.as_ptr(), b"%d\0".as_ptr() as *const c_char, &mut val);
    if scan != 1 {
        pr_err(b"%s: error reading: %s\n\0".as_ptr() as *const c_char, b"cs_etm_get_ro_signed\0".as_ptr() as *const c_char, pmu_path.as_ptr());
        return -EINVAL;
    }
    *out_val = val as u64;
    0
}

unsafe extern "C" fn cs_etm_pmu_path_exists(pmu: *mut perf_pmu, cpu: perf_cpu, path_in: *const c_char) -> bool {
    let mut pmu_path = [0 as c_char; PATH_MAX];
    snprintf(pmu_path.as_mut_ptr(), PATH_MAX, b"cpu%d/%s\0".as_ptr() as *const c_char, cpu.cpu, path_in);
    perf_pmu__file_exists(pmu, pmu_path.as_ptr())
}

const TRCDEVARCH_ARCHPART_SHIFT: u32 = 0;
const TRCDEVARCH_ARCHPART_MASK: u64 = genmask(11, 0);
fn TRCDEVARCH_ARCHPART(x: u64) -> u64 { (x & TRCDEVARCH_ARCHPART_MASK) >> TRCDEVARCH_ARCHPART_SHIFT }
const TRCDEVARCH_ARCHVER_SHIFT: u32 = 12;
const TRCDEVARCH_ARCHVER_MASK: u64 = genmask(15, 12);
fn TRCDEVARCH_ARCHVER(x: u64) -> u64 { (x & TRCDEVARCH_ARCHVER_MASK) >> TRCDEVARCH_ARCHVER_SHIFT }

unsafe extern "C" fn cs_etm_is_ete(cs_etm_pmu: *mut perf_pmu, cpu: perf_cpu) -> bool {
    let mut trcdevarch: u64 = 0;
    if !cs_etm_pmu_path_exists(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCDEVARCH]) {
        return false;
    }
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCDEVARCH], &mut trcdevarch);
    TRCDEVARCH_ARCHVER(trcdevarch) == 5 && TRCDEVARCH_ARCHPART(trcdevarch) == 0xA13
}

unsafe extern "C" fn cs_etm_get_legacy_trace_id(cpu: perf_cpu) -> u64 {
    coresight_legacy_cpu_trace_id(cpu.cpu % 48)
}

unsafe extern "C" fn cs_etm_save_etmv4_header(data: *mut u64, itr: *mut auxtrace_record, cpu: perf_cpu) {
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    *data.add(CS_ETMV4_TRCCONFIGR) = cs_etmv4_synth_trcconfigr(itr);
    *data.add(CS_ETMV4_TRCTRACEIDR) = cs_etm_get_legacy_trace_id(cpu);
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR0], data.add(CS_ETMV4_TRCIDR0));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR1], data.add(CS_ETMV4_TRCIDR1));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR2], data.add(CS_ETMV4_TRCIDR2));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCIDR8], data.add(CS_ETMV4_TRCIDR8));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TRCAUTHSTATUS], data.add(CS_ETMV4_TRCAUTHSTATUS));
    if !cs_etm_pmu_path_exists(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TS_SOURCE]) ||
        cs_etm_get_ro_signed(cs_etm_pmu, cpu, metadata_etmv4_ro[CS_ETMV4_TS_SOURCE], data.add(CS_ETMV4_TS_SOURCE)) != 0 {
        pr_debug3(b"[%03d] pmu file 'ts_source' not found. Fallback to safe value (-1)\n\0".as_ptr() as *const c_char, cpu.cpu);
        *data.add(CS_ETMV4_TS_SOURCE) = (-1i64) as u64;
    }
}

unsafe extern "C" fn cs_etm_save_ete_header(data: *mut u64, itr: *mut auxtrace_record, cpu: perf_cpu) {
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    *data.add(CS_ETE_TRCCONFIGR) = cs_etmv4_synth_trcconfigr(itr);
    *data.add(CS_ETE_TRCTRACEIDR) = cs_etm_get_legacy_trace_id(cpu);
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCIDR0], data.add(CS_ETE_TRCIDR0));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCIDR1], data.add(CS_ETE_TRCIDR1));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCIDR2], data.add(CS_ETE_TRCIDR2));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCIDR8], data.add(CS_ETE_TRCIDR8));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCAUTHSTATUS], data.add(CS_ETE_TRCAUTHSTATUS));
    cs_etm_get_ro(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TRCDEVARCH], data.add(CS_ETE_TRCDEVARCH));
    if !cs_etm_pmu_path_exists(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TS_SOURCE]) ||
        cs_etm_get_ro_signed(cs_etm_pmu, cpu, metadata_ete_ro[CS_ETE_TS_SOURCE], data.add(CS_ETE_TS_SOURCE)) != 0 {
        pr_debug3(b"[%03d] pmu file 'ts_source' not found. Fallback to safe value (-1)\n\0".as_ptr() as *const c_char, cpu.cpu);
        *data.add(CS_ETE_TS_SOURCE) = (-1i64) as u64;
    }
}

unsafe extern "C" fn cs_etm_get_metadata(cpu: perf_cpu, offset: *mut u32, itr: *mut auxtrace_record, info: *mut perf_record_auxtrace_info) {
    let increment: u32;
    let nr_trc_params: u32;
    let magic: u64;
    let cs_etm_pmu = cs_etm_get_pmu(itr);
    let priv_base = (*info).priv_.as_mut_ptr();

    match cs_etm_get_version(cs_etm_pmu, cpu) {
        cs_etm_version::CS_ETE => {
            magic = __perf_cs_ete_magic;
            cs_etm_save_ete_header(priv_base.add(*offset as usize), itr, cpu);
            increment = CS_ETE_PRIV_MAX as u32;
            nr_trc_params = CS_ETE_PRIV_MAX as u32 - CS_ETM_COMMON_BLK_MAX_V1;
        }
        cs_etm_version::CS_ETMV4 => {
            magic = __perf_cs_etmv4_magic;
            cs_etm_save_etmv4_header(priv_base.add(*offset as usize), itr, cpu);
            increment = CS_ETMV4_PRIV_MAX as u32;
            nr_trc_params = CS_ETMV4_PRIV_MAX as u32 - CS_ETMV4_TRCCONFIGR as u32;
        }
        cs_etm_version::CS_ETMV3 => {
            magic = __perf_cs_etmv3_magic;
            *priv_base.add(*offset as usize + CS_ETM_ETMCR) = cs_etm_synth_etmcr(itr);
            *priv_base.add(*offset as usize + CS_ETM_ETMTRACEIDR) = cs_etm_get_legacy_trace_id(cpu);
            cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv3_ro[CS_ETM_ETMCCER], priv_base.add(*offset as usize + CS_ETM_ETMCCER));
            cs_etm_get_ro(cs_etm_pmu, cpu, metadata_etmv3_ro[CS_ETM_ETMIDR], priv_base.add(*offset as usize + CS_ETM_ETMIDR));
            increment = CS_ETM_PRIV_MAX as u32;
            nr_trc_params = CS_ETM_PRIV_MAX as u32 - CS_ETM_ETMCR as u32;
        }
        cs_etm_version::CS_NOT_PRESENT => {
            assert(true);
            return;
        }
    }

    *priv_base.add(*offset as usize + CS_ETM_MAGIC) = magic;
    *priv_base.add(*offset as usize + CS_ETM_CPU) = cpu.cpu as u64;
    *priv_base.add(*offset as usize + CS_ETM_NR_TRC_PARAMS) = nr_trc_params as u64;
    *offset += increment;
}

unsafe extern "C" fn cs_etm_info_fill(itr: *mut auxtrace_record, session: *mut perf_session, info: *mut perf_record_auxtrace_info, priv_size: size_t) -> c_int {
    let mut i: c_uint = 0;
    let mut offset: u32;
    let nr_cpu: u64;
    let type_: u64;
    let cpu_map: *mut perf_cpu_map;
    let event_cpus = (*evlist__core((*session).evlist)).user_requested_cpus;
    let online_cpus = perf_cpu_map__new_online_cpus();
    let ptr = cs_etm_ptr(itr);
    let cs_etm_pmu = (*ptr).cs_etm_pmu;
    let mut cpu = perf_cpu { cpu: 0 };

    if priv_size != cs_etm_info_priv_size(itr, (*session).evlist) {
        return -EINVAL;
    }

    if (*evlist__core((*session).evlist)).nr_mmaps == 0 {
        return -EINVAL;
    }

    if perf_cpu_map__has_any_cpu(event_cpus) {
        cpu_map = online_cpus;
    } else {
        while perf_cpu_map__for_each_cpu_next(event_cpus, &mut i, &mut cpu) {
            if !perf_cpu_map__has(online_cpus, cpu) {
                return -EINVAL;
            }
        }
        cpu_map = event_cpus;
    }

    nr_cpu = perf_cpu_map__nr(cpu_map);
    type_ = (*cs_etm_pmu).type_ as u64;

    (*info).type_ = PERF_AUXTRACE_CS_ETM;
    let priv_base = (*info).priv_.as_mut_ptr();
    *priv_base.add(CS_HEADER_VERSION) = CS_HEADER_CURRENT_VERSION;
    *priv_base.add(CS_PMU_TYPE_CPUS) = type_ << 32;
    *priv_base.add(CS_PMU_TYPE_CPUS) |= nr_cpu;
    *priv_base.add(CS_ETM_SNAPSHOT) = (*ptr).snapshot_mode as u64;

    offset = (CS_ETM_SNAPSHOT + 1) as u32;
    i = 0;
    while perf_cpu_map__for_each_cpu_next(cpu_map, &mut i, &mut cpu) {
        assert((offset as usize) < priv_size);
        cs_etm_get_metadata(cpu, &mut offset, itr, info);
    }

    perf_cpu_map__put(online_cpus);
    0
}

unsafe extern "C" fn cs_etm_snapshot_start(itr: *mut auxtrace_record) -> c_int {
    let ptr = cs_etm_ptr(itr);
    let evsel = cs_etm_get_evsel((*ptr).evlist, (*ptr).cs_etm_pmu);
    if !evsel.is_null() {
        return evsel__disable(evsel);
    }
    -EINVAL
}

unsafe extern "C" fn cs_etm_snapshot_finish(itr: *mut auxtrace_record) -> c_int {
    let ptr = cs_etm_ptr(itr);
    let mut ret = -EINVAL;
    evlist_for_each_entry((*ptr).evlist, |evsel| {
        if (*evsel).core.attr.type_ == (*(*ptr).cs_etm_pmu).type_ {
            ret = evsel__enable(evsel);
            false
        } else {
            true
        }
    });
    ret
}

unsafe extern "C" fn cs_etm_reference(_itr: *mut auxtrace_record) -> u64 {
    (((rand() as u64) << 0) & 0x00000000FFFFFFFFu64) |
        (((rand() as u64) << 32) & 0xFFFFFFFF00000000u64)
}

unsafe extern "C" fn cs_etm_recording_free(itr: *mut auxtrace_record) {
    let ptr = cs_etm_ptr(itr);
    free(ptr as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs_etm_record_init(err: *mut c_int) -> *mut auxtrace_record {
    let cs_etm_pmu: *mut perf_pmu;
    let ptr: *mut cs_etm_recording;

    cs_etm_pmu = perf_pmus__find(CORESIGHT_ETM_PMU_NAME);
    if cs_etm_pmu.is_null() {
        *err = -EINVAL;
        return ptr::null_mut();
    }

    ptr = zalloc(size_of::<cs_etm_recording>()) as *mut cs_etm_recording;
    if ptr.is_null() {
        *err = -ENOMEM;
        return ptr::null_mut();
    }

    (*ptr).cs_etm_pmu = cs_etm_pmu;
    (*ptr).itr.parse_snapshot_options = Some(cs_etm_parse_snapshot_options);
    (*ptr).itr.recording_options = Some(cs_etm_recording_options);
    (*ptr).itr.info_priv_size = Some(cs_etm_info_priv_size);
    (*ptr).itr.info_fill = Some(cs_etm_info_fill);
    (*ptr).itr.snapshot_start = Some(cs_etm_snapshot_start);
    (*ptr).itr.snapshot_finish = Some(cs_etm_snapshot_finish);
    (*ptr).itr.reference = Some(cs_etm_reference);
    (*ptr).itr.free = Some(cs_etm_recording_free);
    (*ptr).itr.read_finish = Some(auxtrace_record__read_finish);

    *err = 0;
    &mut (*ptr).itr
}

/*
 * Set a default config to enable the user changed config tracking mechanism
 * (CFG_CHG and evsel__set_config_if_unset()). If no default is set then user
 * changes aren't tracked.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs_etm_get_default_config(_pmu: *const perf_pmu, attr: *mut perf_event_attr) {
    (*attr).sample_period = 1;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
