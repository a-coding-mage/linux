// SPDX-License-Identifier: GPL-2.0
//
// Translated from C implementation source.
// C includes removed; the referenced symbols and layouts are provided by
// surrounding perf sources.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

// cmask=0, inv=0, pc=0, edge=0, umask=4, event=0
pub const TOPDOWN_SLOTS: u64 = 0x0400;

pub const PERF_TYPE_RAW: c_uint = 4;
pub const ENOMEM: c_int = 12;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub config: c_ulonglong,
    pub config1: c_ulonglong,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
    pub leader: *mut evsel,
    pub node: list_head,
    pub cpus: *mut perf_cpu_map,
    pub pmu_cpus: *mut perf_cpu_map,
    pub is_pmu_core: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub evlist: *mut evlist,
    pub pmu: *mut perf_pmu,
    pub name: *mut c_char,
    pub precise_max: bool,
    pub sample_read: bool,
    pub weak_group: bool,
    pub bpf_counter: bool,
    pub retire_lat: bool,
}

unsafe extern "C" {
    fn perf_pmus__find_by_type(type_: c_uint) -> *mut perf_pmu;
    fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn evsel__sys_has_perf_metrics(evsel: *mut evsel) -> bool;
    fn evsel__new_idx(attr: *const perf_event_attr, idx: c_int) -> *mut evsel;
    fn perf_cpu_map__get(map: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);

    /*
     * Rust translation of evlist__for_each_entry(leader->evlist, evsel).
     * The concrete iterator helpers are supplied by the surrounding perf
     * bindings for the C list macro.
     */
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

/* Check whether there is a PMU which supports the perf metrics. */
pub unsafe fn topdown_sys_has_perf_metrics() -> bool {
    static mut HAS_PERF_METRICS: bool = false;
    static mut CACHED: bool = false;
    let pmu: *mut perf_pmu;

    if unsafe { CACHED } {
        return unsafe { HAS_PERF_METRICS };
    }

    /*
     * The perf metrics feature is a core PMU feature.
     * The PERF_TYPE_RAW type is the type of a core PMU.
     * The slots event is only available when the core PMU
     * supports the perf metrics feature.
     */
    pmu = unsafe { perf_pmus__find_by_type(PERF_TYPE_RAW) };
    if !pmu.is_null() && unsafe { perf_pmu__have_event(pmu, c"slots".as_ptr()) } {
        unsafe {
            HAS_PERF_METRICS = true;
        }
    }

    unsafe {
        CACHED = true;
        HAS_PERF_METRICS
    }
}

pub unsafe fn arch_is_topdown_slots(evsel: *const evsel) -> bool {
    unsafe {
        (*evsel).core.attr.type_ == PERF_TYPE_RAW
            && (*evsel).core.attr.config == TOPDOWN_SLOTS
            && (*evsel).core.attr.config1 == 0
    }
}

pub unsafe fn arch_is_topdown_metrics(evsel: *const evsel) -> bool {
    // cmask=0, inv=0, pc=0, edge=0, umask=0x80-0x87, event=0
    unsafe {
        (*evsel).core.attr.type_ == PERF_TYPE_RAW
            && ((*evsel).core.attr.config & 0xFFFFF8FF) == 0x8000
            && (*evsel).core.attr.config1 == 0
    }
}

/*
 * Check whether a topdown group supports sample-read.
 *
 * Only Topdown metric supports sample-read. The slots
 * event must be the leader of the topdown group.
 */
pub unsafe fn arch_topdown_sample_read(leader: *mut evsel) -> bool {
    let mut evsel: *mut evsel;

    if !unsafe { evsel__sys_has_perf_metrics(leader) } {
        return false;
    }

    if !unsafe { arch_is_topdown_slots(leader) } {
        return false;
    }

    /*
     * If slots event as leader event but no topdown metric events
     * in group, slots event should still sample as leader.
     */
    evsel = unsafe { evlist__first((*leader).evlist) };
    while !evsel.is_null() {
        if unsafe { (*evsel).core.leader != (*leader).core.leader } {
            evsel = unsafe { evlist__next((*leader).evlist, evsel) };
            continue;
        }
        if evsel != leader && unsafe { arch_is_topdown_metrics(evsel) } {
            return true;
        }
        evsel = unsafe { evlist__next((*leader).evlist, evsel) };
    }

    false
}

/*
 * Make a copy of the topdown metric event metric_event with the given index but
 * change its configuration to be a topdown slots event. Copying from
 * metric_event ensures modifiers are the same.
 */
pub unsafe fn topdown_insert_slots_event(
    list: *mut list_head,
    idx: c_int,
    metric_event: *mut evsel,
) -> c_int {
    let evsel = unsafe { evsel__new_idx(&(*metric_event).core.attr, idx) };

    if evsel.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*evsel).core.attr.config = TOPDOWN_SLOTS;
        (*evsel).core.cpus = perf_cpu_map__get((*metric_event).core.cpus);
        (*evsel).core.pmu_cpus = perf_cpu_map__get((*metric_event).core.pmu_cpus);
        (*evsel).core.is_pmu_core = true;
        (*evsel).pmu = (*metric_event).pmu;
        (*evsel).name = strdup(c"slots".as_ptr());
        (*evsel).precise_max = (*metric_event).precise_max;
        (*evsel).sample_read = (*metric_event).sample_read;
        (*evsel).weak_group = (*metric_event).weak_group;
        (*evsel).bpf_counter = (*metric_event).bpf_counter;
        (*evsel).retire_lat = (*metric_event).retire_lat;
        evsel__set_leader(evsel, evsel__leader(metric_event));
        list_add_tail(&mut (*evsel).core.node, list);
    }
    0
}
