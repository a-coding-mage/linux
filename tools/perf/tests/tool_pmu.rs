// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Translated from perf/tests/tool_pmu.c.
// C dependencies: debug.h, evlist.h, parse-events.h, tests.h, tool_pmu.h.

use core::ffi::{c_char, c_int, c_longlong};

pub type bool_ = bool;

pub const TEST_OK: c_int = 0;
pub const TEST_FAIL: c_int = -1;

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

pub type tool_pmu_event = c_int;

unsafe extern "C" {
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__for_each_entry_next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;

    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, event: *const c_char);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;

    fn perf_pmu__is_tool(pmu: *mut perf_pmu) -> bool;
    fn tool_pmu__event_to_str(ev: tool_pmu_event) -> *const c_char;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn snprintf(str_: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

unsafe fn do_test(ev: tool_pmu_event, with_pmu: bool) -> c_int {
    let evlist = evlist__new();
    let mut evsel: *mut evsel;
    let mut err = core::mem::MaybeUninit::<parse_events_error>::uninit();
    let mut ret: c_int;
    let mut str_: [c_char; 128] = [0; 128];
    let mut found = false;

    if evlist.is_null() {
        pr_err(c"evlist allocation failed\n".as_ptr());
        return TEST_FAIL;
    }

    if with_pmu {
        snprintf(
            str_.as_mut_ptr(),
            str_.len(),
            c"tool/%s/".as_ptr(),
            tool_pmu__event_to_str(ev),
        );
    } else {
        snprintf(
            str_.as_mut_ptr(),
            str_.len(),
            c"%s".as_ptr(),
            tool_pmu__event_to_str(ev),
        );
    }

    parse_events_error__init(err.as_mut_ptr());
    ret = parse_events(evlist, str_.as_ptr(), err.as_mut_ptr());
    if ret != 0 {
        if tool_pmu__event_to_str(ev).is_null() {
            ret = TEST_OK;
            parse_events_error__exit(err.as_mut_ptr());
            evlist__put(evlist);
            return ret;
        }

        pr_debug(
            c"FAILED %s:%d failed to parse event '%s', err %d\n".as_ptr(),
            c"tool_pmu.c".as_ptr(),
            line!() as c_int,
            str_.as_ptr(),
            ret,
        );
        parse_events_error__print(err.as_mut_ptr(), str_.as_ptr());
        ret = TEST_FAIL;
        parse_events_error__exit(err.as_mut_ptr());
        evlist__put(evlist);
        return ret;
    }

    ret = TEST_OK;
    if if with_pmu {
        evlist__nr_entries(evlist) != 1
    } else {
        evlist__nr_entries(evlist) < 1
    } {
        pr_debug(
            c"FAILED %s:%d Unexpected number of events for '%s' of %d\n".as_ptr(),
            c"tool_pmu.c".as_ptr(),
            line!() as c_int,
            str_.as_ptr(),
            evlist__nr_entries(evlist),
        );
        ret = TEST_FAIL;
        parse_events_error__exit(err.as_mut_ptr());
        evlist__put(evlist);
        return ret;
    }

    evsel = evlist__for_each_entry_next(evlist, core::ptr::null_mut());
    while !evsel.is_null() {
        if perf_pmu__is_tool((*evsel).pmu) {
            if (*evsel).core.attr.config != ev as u64 {
                pr_debug(
                    c"FAILED %s:%d Unexpected config for '%s', %lld != %d\n".as_ptr(),
                    c"tool_pmu.c".as_ptr(),
                    line!() as c_int,
                    str_.as_ptr(),
                    (*evsel).core.attr.config as c_longlong,
                    ev,
                );
                ret = TEST_FAIL;
                parse_events_error__exit(err.as_mut_ptr());
                evlist__put(evlist);
                return ret;
            }
            found = true;
        }

        evsel = evlist__for_each_entry_next(evlist, evsel);
    }

    if !found && !tool_pmu__event_to_str(ev).is_null() {
        pr_debug(
            c"FAILED %s:%d Didn't find tool event '%s' in parsed evsels\n".as_ptr(),
            c"tool_pmu.c".as_ptr(),
            line!() as c_int,
            str_.as_ptr(),
        );
        ret = TEST_FAIL;
    }

    parse_events_error__exit(err.as_mut_ptr());
    evlist__put(evlist);
    ret
}

unsafe extern "C" fn test__tool_pmu_without_pmu(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut i: c_int = 0;

    // C source uses tool_pmu__for_each_event(i), supplied by tool_pmu.h.
    tool_pmu__for_each_event!(i, {
        let ret = do_test(i, false);

        if ret != TEST_OK {
            return ret;
        }
    });
    TEST_OK
}

unsafe extern "C" fn test__tool_pmu_with_pmu(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut i: c_int = 0;

    // C source uses tool_pmu__for_each_event(i), supplied by tool_pmu.h.
    tool_pmu__for_each_event!(i, {
        let ret = do_test(i, true);

        if ret != TEST_OK {
            return ret;
        }
    });
    TEST_OK
}

static mut tests__tool_pmu: [test_case; 3] = [
    test_case {
        name: c"Parsing without PMU name".as_ptr(),
        run_case: Some(test__tool_pmu_without_pmu),
    },
    test_case {
        name: c"Parsing with PMU name".as_ptr(),
        run_case: Some(test__tool_pmu_with_pmu),
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
    },
];

#[unsafe(no_mangle)]
pub static mut suite__tool_pmu: test_suite = test_suite {
    desc: c"Tool PMU".as_ptr(),
    test_cases: unsafe { tests__tool_pmu.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
