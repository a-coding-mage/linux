// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_double, c_int, c_void};

// Dependencies originally provided by:
// linux/compiler.h, perf/cpumap.h, string.h, cpumap.h, evlist.h, evsel.h,
// header.h, machine.h, util/synthetic-events.h, target.h, tool.h, tests.h,
// debug.h

const PERF_EVENT_UPDATE__UNIT: u32 = 0;
const PERF_EVENT_UPDATE__SCALE: u32 = 1;
const PERF_EVENT_UPDATE__NAME: u32 = 2;
const PERF_EVENT_UPDATE__CPUS: u32 = 3;

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: [u64; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_record_event_update_scale {
    pub scale: c_double,
}

#[repr(C)]
pub struct perf_record_event_update_cpus {
    pub cpus: c_void,
}

#[repr(C)]
pub union perf_record_event_update_payload {
    pub scale: perf_record_event_update_scale,
    pub cpus: perf_record_event_update_cpus,
}

#[repr(C)]
pub struct perf_record_event_update {
    pub id: u64,
    pub type_: u32,
    pub payload: perf_record_event_update_payload,
    pub unit: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub pmu_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
    pub unit: *const c_char,
    pub scale: c_double,
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
struct event_name {
    tool: perf_tool,
    name: *const c_char,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);

    fn cpu_map__new_data(data: *const c_void) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;

    fn evlist__new_default(target: *mut target, sample_callchains: bool) -> *mut evlist;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__core(evlist: *mut evlist) -> *mut c_void;
    fn evlist__put(evlist: *mut evlist);

    fn perf_evsel__alloc_id(evsel: *mut perf_evsel, ncpus: c_int, nthreads: c_int) -> c_int;
    fn perf_evlist__id_add(
        evlist: *mut c_void,
        evsel: *mut perf_evsel,
        cpu: c_int,
        thread: c_int,
        id: u64,
    );

    fn perf_event__synthesize_event_update_unit(
        tool: *mut perf_tool,
        evsel: *mut evsel,
        process: Option<
            unsafe extern "C" fn(
                *const perf_tool,
                *mut perf_event,
                *mut perf_sample,
                *mut machine,
            ) -> c_int,
        >,
    ) -> c_int;
    fn perf_event__synthesize_event_update_scale(
        tool: *mut perf_tool,
        evsel: *mut evsel,
        process: Option<
            unsafe extern "C" fn(
                *const perf_tool,
                *mut perf_event,
                *mut perf_sample,
                *mut machine,
            ) -> c_int,
        >,
    ) -> c_int;
    fn perf_event__synthesize_event_update_name(
        tool: *mut perf_tool,
        evsel: *mut evsel,
        process: Option<
            unsafe extern "C" fn(
                *const perf_tool,
                *mut perf_event,
                *mut perf_sample,
                *mut machine,
            ) -> c_int,
        >,
    ) -> c_int;
    fn perf_event__synthesize_event_update_cpus(
        tool: *mut perf_tool,
        evsel: *mut evsel,
        process: Option<
            unsafe extern "C" fn(
                *const perf_tool,
                *mut perf_event,
                *mut perf_sample,
                *mut machine,
            ) -> c_int,
        >,
    ) -> c_int;

    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !$cond {
            return -1;
        }
    };
}

unsafe extern "C" fn process_event_unit(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let ev = event as *mut perf_record_event_update;

    TEST_ASSERT_VAL!("wrong id", (*ev).id == 123);
    TEST_ASSERT_VAL!("wrong id", (*ev).type_ == PERF_EVENT_UPDATE__UNIT);
    TEST_ASSERT_VAL!(
        "wrong unit",
        strcmp((*ev).unit, c"KRAVA".as_ptr()) == 0
    );
    0
}

unsafe extern "C" fn process_event_scale(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let ev = event as *mut perf_record_event_update;

    TEST_ASSERT_VAL!("wrong id", (*ev).id == 123);
    TEST_ASSERT_VAL!("wrong id", (*ev).type_ == PERF_EVENT_UPDATE__SCALE);
    TEST_ASSERT_VAL!("wrong scale", (*ev).payload.scale.scale == 0.123);
    0
}

unsafe extern "C" fn process_event_name(
    tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let tmp = tool as *mut event_name;
    let ev = event as *mut perf_record_event_update;

    TEST_ASSERT_VAL!("wrong id", (*ev).id == 123);
    TEST_ASSERT_VAL!("wrong id", (*ev).type_ == PERF_EVENT_UPDATE__NAME);
    TEST_ASSERT_VAL!(
        "wrong name",
        strcmp((*ev).name, (*tmp).name) == 0
    );
    0
}

unsafe extern "C" fn process_event_cpus(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let ev = event as *mut perf_record_event_update;
    let map: *mut perf_cpu_map;

    map = cpu_map__new_data(&(*ev).payload.cpus.cpus as *const c_void);

    TEST_ASSERT_VAL!("wrong id", (*ev).id == 123);
    TEST_ASSERT_VAL!("wrong type", (*ev).type_ == PERF_EVENT_UPDATE__CPUS);
    TEST_ASSERT_VAL!("wrong cpus", perf_cpu_map__nr(map) == 3);
    TEST_ASSERT_VAL!("wrong cpus", perf_cpu_map__cpu(map, 0).cpu == 1);
    TEST_ASSERT_VAL!("wrong cpus", perf_cpu_map__cpu(map, 1).cpu == 2);
    TEST_ASSERT_VAL!("wrong cpus", perf_cpu_map__cpu(map, 2).cpu == 3);
    perf_cpu_map__put(map);
    0
}

unsafe extern "C" fn test__event_update(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut evsel: *mut evsel;
    let mut tmp = event_name {
        tool: core::mem::zeroed(),
        name: core::ptr::null(),
    };
    let mut target: target = core::mem::zeroed();
    let evlist = evlist__new_default(&mut target, false);

    TEST_ASSERT_VAL!("failed to get evlist", !evlist.is_null());

    evsel = evlist__first(evlist);

    TEST_ASSERT_VAL!(
        "failed to allocate ids",
        perf_evsel__alloc_id(&mut (*evsel).core, 1, 1) == 0
    );

    perf_evlist__id_add(evlist__core(evlist), &mut (*evsel).core, 0, 0, 123);

    free((*evsel).unit as *mut c_void);
    (*evsel).unit = strdup(c"KRAVA".as_ptr());

    TEST_ASSERT_VAL!(
        "failed to synthesize attr update unit",
        perf_event__synthesize_event_update_unit(
            core::ptr::null_mut(),
            evsel,
            Some(process_event_unit),
        ) == 0
    );

    (*evsel).scale = 0.123;

    TEST_ASSERT_VAL!(
        "failed to synthesize attr update scale",
        perf_event__synthesize_event_update_scale(
            core::ptr::null_mut(),
            evsel,
            Some(process_event_scale),
        ) == 0
    );

    perf_tool__init(&mut tmp.tool, false);
    tmp.name = evsel__name(evsel);

    TEST_ASSERT_VAL!(
        "failed to synthesize attr update name",
        perf_event__synthesize_event_update_name(
            &mut tmp.tool,
            evsel,
            Some(process_event_name),
        ) == 0
    );

    perf_cpu_map__put((*(&mut (*evsel).core as *mut perf_evsel as *mut evsel_core)).pmu_cpus);
    (*(&mut (*evsel).core as *mut perf_evsel as *mut evsel_core)).pmu_cpus =
        perf_cpu_map__new(c"1,2,3".as_ptr());

    TEST_ASSERT_VAL!(
        "failed to synthesize attr update cpus",
        perf_event__synthesize_event_update_cpus(
            &mut tmp.tool,
            evsel,
            Some(process_event_cpus),
        ) == 0
    );

    evlist__put(evlist);
    0
}

// DEFINE_SUITE("Synthesize attr update", event_update);
