// SPDX-License-Identifier: GPL-2.0
// Translated from perf/arch/x86/tests/hybrid.c.
// C includes referenced: arch-tests.h, debug.h, evlist.h, evsel.h, pmu.h,
// pmus.h, tests/tests.h.

use core::ffi::{c_char, c_int};

type __u64 = u64;

const PERF_HW_EVENT_MASK: __u64 = 0xffffffff;
const PERF_PMU_TYPE_SHIFT: c_int = 32;
const PERF_TYPE_HARDWARE: __u64 = 0;
const PERF_TYPE_SOFTWARE: __u64 = 1;
const PERF_TYPE_RAW: __u64 = 4;
const PERF_TYPE_HW_CACHE: __u64 = 3;
const PERF_COUNT_HW_CPU_CYCLES: __u64 = 0;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: __u64 = 4;

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = 2;

#[repr(C)]
struct perf_event_attr {
    type_: __u64,
    config: __u64,
    config1: __u64,
    config2: __u64,
    config3: __u64,
    sample_period: __u64,
    exclude_user: bool,
    exclude_kernel: bool,
}

#[repr(C)]
struct perf_evsel {
    attr: perf_event_attr,
}

#[repr(C)]
struct evsel_core {
    attr: perf_event_attr,
}

#[repr(C)]
struct evsel {
    core: evsel_core,
}

#[repr(C)]
struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_pmu {
    name: *const c_char,
}

#[repr(C)]
struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
struct evlist_test {
    name: *const c_char,
    valid: Option<unsafe extern "C" fn() -> bool>,
    check: Option<unsafe extern "C" fn(evlist: *mut evlist) -> c_int>,
}

unsafe extern "C" {
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evsel__has_leader(evsel: *mut evsel, leader: *mut evsel) -> bool;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
    fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist;
    fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel;
    fn perf_evsel__next(evsel: *mut perf_evsel) -> *mut perf_evsel;
    fn perf_pmus__find_by_type(type_: __u64) -> *mut perf_pmu;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, event: *const c_char);
    fn parse_events_error__contains(err: *mut parse_events_error, str_: *const c_char) -> bool;
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !($cond) {
            return TEST_FAIL;
        }
    };
}

unsafe fn test_config(evsel: *const evsel, expected_config: __u64) -> bool {
    ((*evsel).core.attr.config & PERF_HW_EVENT_MASK) == expected_config
}

unsafe fn test_perf_config(evsel: *const perf_evsel, expected_config: __u64) -> bool {
    ((*evsel).attr.config & PERF_HW_EVENT_MASK) == expected_config
}

unsafe fn test_hybrid_type(evsel: *const evsel, expected_config: __u64) -> bool {
    ((*evsel).core.attr.config >> PERF_PMU_TYPE_SHIFT) == expected_config
}

unsafe extern "C" fn test__hybrid_hw_event_with_pmu(evlist: *mut evlist) -> c_int {
    let evsel = evlist__first(evlist);

    TEST_ASSERT_VAL!("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_OK
}

unsafe extern "C" fn test__hybrid_hw_group_event(evlist: *mut evlist) -> c_int {
    let mut evsel: *mut evsel;
    let leader: *mut evsel;

    evsel = evlist__first(evlist);
    leader = evsel;
    TEST_ASSERT_VAL!("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));

    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_BRANCH_INSTRUCTIONS));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_OK
}

unsafe extern "C" fn test__hybrid_sw_hw_group_event(evlist: *mut evlist) -> c_int {
    let mut evsel: *mut evsel;
    let leader: *mut evsel;

    evsel = evlist__first(evlist);
    leader = evsel;
    TEST_ASSERT_VAL!("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_SOFTWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));

    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_OK
}

unsafe extern "C" fn test__hybrid_hw_sw_group_event(evlist: *mut evlist) -> c_int {
    let mut evsel: *mut evsel;
    let leader: *mut evsel;

    evsel = evlist__first(evlist);
    leader = evsel;
    TEST_ASSERT_VAL!("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));

    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_SOFTWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_OK
}

unsafe extern "C" fn test__hybrid_group_modifier1(evlist: *mut evlist) -> c_int {
    let mut evsel: *mut evsel;
    let leader: *mut evsel;

    evsel = evlist__first(evlist);
    leader = evsel;
    TEST_ASSERT_VAL!("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_ASSERT_VAL!("wrong exclude_user", (*evsel).core.attr.exclude_user);
    TEST_ASSERT_VAL!("wrong exclude_kernel", !(*evsel).core.attr.exclude_kernel);

    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_BRANCH_INSTRUCTIONS));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_ASSERT_VAL!("wrong exclude_user", !(*evsel).core.attr.exclude_user);
    TEST_ASSERT_VAL!("wrong exclude_kernel", (*evsel).core.attr.exclude_kernel);
    TEST_OK
}

unsafe extern "C" fn test__hybrid_raw1(evlist: *mut evlist) -> c_int {
    let mut evsel = perf_evlist__first(evlist__core(evlist));

    while !evsel.is_null() {
        let pmu = perf_pmus__find_by_type((*evsel).attr.type_);

        TEST_ASSERT_VAL!("missing pmu", !pmu.is_null());
        TEST_ASSERT_VAL!(
            "unexpected pmu",
            strncmp((*pmu).name, c"cpu_".as_ptr(), 4) == 0
        );
        TEST_ASSERT_VAL!("wrong config", test_perf_config(evsel, 0x1a));
        evsel = perf_evsel__next(evsel);
    }
    TEST_OK
}

unsafe extern "C" fn test__hybrid_raw2(evlist: *mut evlist) -> c_int {
    let evsel = evlist__first(evlist);

    TEST_ASSERT_VAL!("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_RAW == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, 0x1a));
    TEST_OK
}

unsafe extern "C" fn test__hybrid_cache_event(evlist: *mut evlist) -> c_int {
    let evsel = evlist__first(evlist);

    TEST_ASSERT_VAL!("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HW_CACHE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong config", 0x2 == ((*evsel).core.attr.config & 0xffffffff));
    TEST_OK
}

unsafe extern "C" fn test__checkevent_pmu(evlist: *mut evlist) -> c_int {
    let evsel = evlist__first(evlist);

    TEST_ASSERT_VAL!("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_RAW == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong config", 10 == (*evsel).core.attr.config);
    TEST_ASSERT_VAL!("wrong config1", 1 == (*evsel).core.attr.config1);
    TEST_ASSERT_VAL!("wrong config2", 3 == (*evsel).core.attr.config2);
    TEST_ASSERT_VAL!("wrong config3", 0 == (*evsel).core.attr.config3);
    /*
     * The period value gets configured within evlist__config,
     * while this test executes only parse events method.
     */
    TEST_ASSERT_VAL!("wrong period", 0 == (*evsel).core.attr.sample_period);

    TEST_OK
}

unsafe extern "C" fn test__hybrid_hw_group_event_2(evlist: *mut evlist) -> c_int {
    let mut evsel: *mut evsel;
    let leader: *mut evsel;

    evsel = evlist__first(evlist);
    leader = evsel;
    TEST_ASSERT_VAL!("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_HARDWARE == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL!("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));

    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL!("wrong type", PERF_TYPE_RAW == (*evsel).core.attr.type_);
    TEST_ASSERT_VAL!("wrong config", (*evsel).core.attr.config == 0x3c);
    TEST_ASSERT_VAL!("wrong leader", evsel__has_leader(evsel, leader));
    TEST_OK
}

static TEST__HYBRID_EVENTS: [evlist_test; 10] = [
    evlist_test {
        name: c"cpu_core/cycles/".as_ptr(),
        valid: None,
        check: Some(test__hybrid_hw_event_with_pmu),
        /* 0 */
    },
    evlist_test {
        name: c"{cpu_core/cycles/,cpu_core/branches/}".as_ptr(),
        valid: None,
        check: Some(test__hybrid_hw_group_event),
        /* 1 */
    },
    evlist_test {
        name: c"{cpu-clock,cpu_core/cycles/}".as_ptr(),
        valid: None,
        check: Some(test__hybrid_sw_hw_group_event),
        /* 2 */
    },
    evlist_test {
        name: c"{cpu_core/cycles/,cpu-clock}".as_ptr(),
        valid: None,
        check: Some(test__hybrid_hw_sw_group_event),
        /* 3 */
    },
    evlist_test {
        name: c"{cpu_core/cycles/k,cpu_core/branches/u}".as_ptr(),
        valid: None,
        check: Some(test__hybrid_group_modifier1),
        /* 4 */
    },
    evlist_test {
        name: c"r1a".as_ptr(),
        valid: None,
        check: Some(test__hybrid_raw1),
        /* 5 */
    },
    evlist_test {
        name: c"cpu_core/r1a/".as_ptr(),
        valid: None,
        check: Some(test__hybrid_raw2),
        /* 6 */
    },
    evlist_test {
        name: c"cpu_core/config=10,config1,config2=3,period=1000/u".as_ptr(),
        valid: None,
        check: Some(test__checkevent_pmu),
        /* 7 */
    },
    evlist_test {
        name: c"cpu_core/LLC-loads/".as_ptr(),
        valid: None,
        check: Some(test__hybrid_cache_event),
        /* 8 */
    },
    evlist_test {
        name: c"{cpu_core/cycles/,cpu_core/cpu-cycles/}".as_ptr(),
        valid: None,
        check: Some(test__hybrid_hw_group_event_2),
        /* 9 */
    },
];

unsafe fn test_event(e: *const evlist_test) -> c_int {
    let mut err = core::mem::MaybeUninit::<parse_events_error>::uninit();
    let evlist: *mut evlist;
    let mut ret: c_int;

    if let Some(valid) = (*e).valid {
        if !valid() {
            pr_debug(c"... SKIP\n".as_ptr());
            return TEST_OK;
        }
    }

    evlist = evlist__new();
    if evlist.is_null() {
        pr_err(c"Failed allocation".as_ptr());
        return TEST_FAIL;
    }
    parse_events_error__init(err.as_mut_ptr());
    ret = parse_events(evlist, (*e).name, err.as_mut_ptr());
    if ret != 0 {
        pr_debug(c"failed to parse event '%s', err %d\n".as_ptr(), (*e).name, ret);
        parse_events_error__print(err.as_mut_ptr(), (*e).name);
        ret = TEST_FAIL;
        if parse_events_error__contains(err.as_mut_ptr(), c"can't access trace events".as_ptr()) {
            ret = TEST_SKIP;
        }
    } else {
        ret = ((*e).check.unwrap())(evlist);
    }
    parse_events_error__exit(err.as_mut_ptr());
    evlist__put(evlist);

    ret
}

fn combine_test_results(existing: c_int, latest: c_int) -> c_int {
    if existing == TEST_FAIL {
        return TEST_FAIL;
    }
    if existing == TEST_SKIP {
        return if latest == TEST_OK { TEST_SKIP } else { latest };
    }
    latest
}

unsafe fn test_events(events: *const evlist_test, cnt: c_int) -> c_int {
    let mut ret = TEST_OK;

    for i in 0..cnt {
        let e = events.offset(i as isize);
        let test_ret: c_int;

        pr_debug(c"running test %d '%s'\n".as_ptr(), i, (*e).name);
        test_ret = test_event(e);
        if test_ret != TEST_OK {
            pr_debug(c"Event test failure: test %d '%s'".as_ptr(), i, (*e).name);
            ret = combine_test_results(ret, test_ret);
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn test__hybrid(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    if perf_pmus__num_core_pmus() == 1 {
        return TEST_SKIP;
    }

    test_events(
        TEST__HYBRID_EVENTS.as_ptr(),
        TEST__HYBRID_EVENTS.len() as c_int,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
