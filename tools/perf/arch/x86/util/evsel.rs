// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/arch/x86/util/evsel.c.
// C include dependencies are represented by external declarations below.

use core::ffi::{c_char, c_int, c_ulonglong};
use core::ptr;

type size_t = usize;
type u64 = u64;
type __u64 = u64;

const IBS_FETCH_L3MISSONLY: u64 = 1u64 << 59;
const IBS_OP_L3MISSONLY: u64 = 1u64 << 16;

extern "C" {
    static evsel__hw_names: [*const c_char; PERF_COUNT_HW_MAX as usize];
    static stat_config: perf_stat_config;

    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn topdown_sys_has_perf_metrics() -> bool;
    fn evsel__find_pmu(evsel: *const evsel) -> *mut perf_pmu;
    fn arch_is_topdown_metrics(evsel: *const evsel) -> bool;
    fn arch_is_topdown_slots(evsel: *const evsel) -> bool;
    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn perf_pmu__has_format(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn perf_pmu__format_type(pmu: *mut perf_pmu, name: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn evsel__prev(evsel: *mut evsel) -> *mut evsel;
    fn evsel__group_idx(evsel: *mut evsel) -> c_int;
    fn x86__is_amd_cpu() -> bool;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn evlist__uniquify_evsel_names(evlist: *mut evlist, stat_config: *const perf_stat_config);
    fn evlist__format_evsels(evlist: *mut evlist, sb: *mut strbuf, max: size_t);
    fn strbuf_release(sb: *mut strbuf);
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
}

extern "C" {
    static PERF_COUNT_HW_MAX: c_int;
    static PERF_HW_EVENT_MASK: u64;
    static PERF_PMU_TYPE_SHIFT: c_int;
    static PERF_TYPE_RAW: u32;
    static PERF_PMU_FORMAT_VALUE_CONFIG2: c_int;
    static WEIGHT_STRUCT: c_int;
    static EINVAL: c_int;
}

#[repr(C)]
pub struct perf_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    pub alloc: size_t,
    pub len: size_t,
    pub buf: *mut c_char,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub type_: u32,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
    pub config2: u64,
    pub precise_ip: u64,
    pub exclude_user: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
    pub exclude_host: u64,
    pub exclude_guest: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub pmu: *mut perf_pmu,
    pub name: *const c_char,
    pub evlist: *mut evlist,
}

const STRBUF_INIT: strbuf = strbuf {
    alloc: 0,
    len: 0,
    buf: ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__set_sample_weight(evsel: *mut evsel) {
    evsel__set_sample_bit(evsel, WEIGHT_STRUCT);
}

/* Check whether the evsel's PMU supports the perf metrics */
#[no_mangle]
pub unsafe extern "C" fn evsel__sys_has_perf_metrics(evsel: *const evsel) -> bool {
    let mut pmu: *mut perf_pmu;

    if !topdown_sys_has_perf_metrics() {
        return false;
    }

    /*
     * The PERF_TYPE_RAW type is the core PMU type, e.g., "cpu" PMU on a
     * non-hybrid machine, "cpu_core" PMU on a hybrid machine.  The
     * topdown_sys_has_perf_metrics checks the slots event is only available
     * for the core PMU, which supports the perf metrics feature. Checking
     * both the PERF_TYPE_RAW type and the slots event should be good enough
     * to detect the perf metrics feature.
     */
    pmu = evsel__find_pmu(evsel);
    !pmu.is_null() && (*pmu).type_ == PERF_TYPE_RAW
}

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__must_be_in_group(evsel: *const evsel) -> bool {
    if !evsel__sys_has_perf_metrics(evsel) {
        return false;
    }

    arch_is_topdown_metrics(evsel) || arch_is_topdown_slots(evsel)
}

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__hw_name(
    evsel: *mut evsel,
    bf: *mut c_char,
    size: size_t,
) -> c_int {
    let event: u64 = (*evsel).core.attr.config & PERF_HW_EVENT_MASK;
    let pmu: u64 = (*evsel).core.attr.config >> PERF_PMU_TYPE_SHIFT;
    let event_name: *const c_char;

    if event < PERF_COUNT_HW_MAX as u64 && !evsel__hw_names[event as usize].is_null() {
        event_name = evsel__hw_names[event as usize];
    } else {
        event_name = b"unknown-hardware\0".as_ptr() as *const c_char;
    }

    /* The PMU type is not required for the non-hybrid platform. */
    if pmu == 0 {
        return scnprintf(bf, size, b"%s\0".as_ptr() as *const c_char, event_name);
    }

    scnprintf(
        bf,
        size,
        b"%s/%s/\0".as_ptr() as *const c_char,
        if !(*evsel).pmu.is_null() {
            (*(*evsel).pmu).name
        } else {
            b"cpu\0".as_ptr() as *const c_char
        },
        event_name,
    )
}

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__apply_ratio_to_prev(
    evsel: *mut evsel,
    attr: *mut perf_event_attr,
) {
    let mut prev_attr: *mut perf_event_attr = ptr::null_mut();
    let mut evsel_prev: *mut evsel = ptr::null_mut();
    let name: *const c_char = b"acr_mask\0".as_ptr() as *const c_char;
    let mut evsel_idx: c_int = 0;
    let ev_mask: __u64;
    let pr_ev_mask: __u64;

    if !perf_pmu__has_format((*evsel).pmu, name) {
        pr_err(
            b"'%s' does not have acr_mask format support\n\0".as_ptr() as *const c_char,
            (*(*evsel).pmu).name,
        );
        return;
    }
    if perf_pmu__format_type((*evsel).pmu, name) != PERF_PMU_FORMAT_VALUE_CONFIG2 {
        pr_err(
            b"'%s' does not have config2 format support\n\0".as_ptr() as *const c_char,
            (*(*evsel).pmu).name,
        );
        return;
    }

    evsel_prev = evsel__prev(evsel);
    if evsel_prev.is_null() {
        pr_err(b"Previous event does not exist.\n\0".as_ptr() as *const c_char);
        return;
    }

    prev_attr = &mut (*evsel_prev).core.attr;

    if (*prev_attr).config2 != 0 {
        pr_err(
            b"'%s' has set config2 (acr_mask?) already, configuration not supported\n\0".as_ptr()
                as *const c_char,
            (*evsel_prev).name,
        );
        return;
    }

    /*
     * acr_mask (config2) is calculated using the event's index in
     * the event group. The first event will use the index of the
     * second event as its mask (e.g., 0x2), indicating that the
     * second event counter will be reset and a sample taken for
     * the first event if its counter overflows. The second event
     * will use the mask consisting of the first and second bits
     * (e.g., 0x3), meaning both counters will be reset if the
     * second event counter overflows.
     */

    evsel_idx = evsel__group_idx(evsel);
    ev_mask = 1u64 << evsel_idx;
    pr_ev_mask = 1u64 << (evsel_idx - 1);

    (*prev_attr).config2 = ev_mask;
    (*attr).config2 = ev_mask | pr_ev_mask;
}

unsafe extern "C" fn ibs_l3miss_warn() {
    pr_warning(
        b"WARNING: Hw internally resets sampling period when L3 Miss Filtering is enabled\n\
and tagged operation does not cause L3 Miss. This causes sampling period skew.\n\0"
            .as_ptr() as *const c_char,
    );
}

static mut WARNED_ONCE: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn arch__post_evsel_config(
    evsel: *mut evsel,
    attr: *mut perf_event_attr,
) {
    let evsel_pmu: *mut perf_pmu;
    let ibs_fetch_pmu: *mut perf_pmu;
    let ibs_op_pmu: *mut perf_pmu;

    if WARNED_ONCE != 0 || !x86__is_amd_cpu() {
        return;
    }

    evsel_pmu = evsel__find_pmu(evsel);
    if evsel_pmu.is_null() {
        return;
    }

    ibs_fetch_pmu = perf_pmus__find(b"ibs_fetch\0".as_ptr() as *const c_char);
    ibs_op_pmu = perf_pmus__find(b"ibs_op\0".as_ptr() as *const c_char);

    if !ibs_fetch_pmu.is_null() && (*ibs_fetch_pmu).type_ == (*evsel_pmu).type_ {
        if ((*attr).config & IBS_FETCH_L3MISSONLY) != 0 {
            ibs_l3miss_warn();
            WARNED_ONCE = 1;
        }
    } else if !ibs_op_pmu.is_null() && (*ibs_op_pmu).type_ == (*evsel_pmu).type_ {
        if ((*attr).config & IBS_OP_L3MISSONLY) != 0 {
            ibs_l3miss_warn();
            WARNED_ONCE = 1;
        }
    }
}

unsafe extern "C" fn amd_evsel__open_strerror(
    evsel: *mut evsel,
    msg: *mut c_char,
    size: size_t,
) -> c_int {
    let pmu: *mut perf_pmu;

    if (*evsel).core.attr.precise_ip == 0 {
        return 0;
    }

    pmu = evsel__find_pmu(evsel);
    if pmu.is_null() || strncmp((*pmu).name, b"ibs\0".as_ptr() as *const c_char, 3) != 0 {
        return 0;
    }

    /* More verbose IBS errors. */
    if (*evsel).core.attr.exclude_kernel != 0
        || (*evsel).core.attr.exclude_user != 0
        || (*evsel).core.attr.exclude_hv != 0
        || (*evsel).core.attr.exclude_idle != 0
        || (*evsel).core.attr.exclude_host != 0
        || (*evsel).core.attr.exclude_guest != 0
    {
        return scnprintf(
            msg,
            size,
            b"AMD IBS doesn't support privilege filtering. Try again without the privilege modifiers (like 'k') at the end.\0"
                .as_ptr() as *const c_char,
        );
    }
    0
}

unsafe extern "C" fn intel_evsel__open_strerror(
    evsel: *mut evsel,
    err: c_int,
    msg: *mut c_char,
    size: size_t,
) -> c_int {
    let mut sb: strbuf = STRBUF_INIT;
    let ret: c_int;

    if err != EINVAL {
        return 0;
    }

    if !topdown_sys_has_perf_metrics() {
        return 0;
    }

    if arch_is_topdown_slots(evsel) {
        if !evsel__is_group_leader(evsel) {
            evlist__uniquify_evsel_names((*evsel).evlist, &stat_config);
            evlist__format_evsels((*evsel).evlist, &mut sb, 2048);
            ret = scnprintf(
                msg,
                size,
                b"Topdown slots event can only be group leader in '%s'.\0".as_ptr()
                    as *const c_char,
                sb.buf,
            );
            strbuf_release(&mut sb);
            return ret;
        }
    } else if arch_is_topdown_metrics(evsel) {
        /*
         * C source uses evlist__for_each_entry(evsel->evlist, pos). The list
         * cursor API is supplied by external perf headers, so the exact
         * iteration primitive must be provided by the translated dependency.
         */
        let mut pos: *mut evsel = evlist__first_entry((*evsel).evlist);
        while !pos.is_null() {
            if pos == evsel || !arch_is_topdown_metrics(pos) {
                pos = evlist__next_entry((*evsel).evlist, pos);
                continue;
            }

            if (*pos).core.attr.config != (*evsel).core.attr.config {
                pos = evlist__next_entry((*evsel).evlist, pos);
                continue;
            }

            evlist__uniquify_evsel_names((*evsel).evlist, &stat_config);
            evlist__format_evsels((*evsel).evlist, &mut sb, 2048);
            ret = scnprintf(
                msg,
                size,
                b"Perf metric event '%s' is duplicated in the same group (only one event is allowed) in '%s'.\0"
                    .as_ptr() as *const c_char,
                evsel__name(evsel),
                sb.buf,
            );
            strbuf_release(&mut sb);
            return ret;
        }
    }
    0
}

extern "C" {
    fn evlist__first_entry(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next_entry(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
}

#[no_mangle]
pub unsafe extern "C" fn arch_evsel__open_strerror(
    evsel: *mut evsel,
    err: c_int,
    msg: *mut c_char,
    size: size_t,
) -> c_int {
    if x86__is_amd_cpu() {
        amd_evsel__open_strerror(evsel, msg, size)
    } else {
        intel_evsel__open_strerror(evsel, err, msg, size)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
