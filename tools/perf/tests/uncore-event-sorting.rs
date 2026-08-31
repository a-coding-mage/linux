// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Translated from perf/tests/uncore-event-sorting.c.
// C dependencies: debug.h, evlist.h, parse-events.h, pmu.h, pmus.h, tests.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub nr_members: c_int,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub pmu: *mut perf_pmu,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub is_core: bool,
}

#[repr(C)]
pub struct pmu_event_info {
    pub name: *const c_char,
    pub pmu: *mut perf_pmu,
}

#[repr(C)]
struct match_state {
    event1: *mut c_char,
    event2: *mut c_char,
}

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = 2;

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;

    fn pr_debug(format: *const c_char, ...);
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn pmu_name_len_no_suffix(name: *const c_char) -> usize;
    fn perf_pmu__for_each_event(
        pmu: *mut perf_pmu,
        skip_duplicate_pmus: bool,
        state: *mut c_void,
        cb: Option<unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int>,
    );
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
    fn evlist__is_last(evlist: *mut evlist, evsel: *mut evsel) -> bool;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
}

unsafe fn zfree_char(ptr: *mut *mut c_char) {
    unsafe {
        if !(*ptr).is_null() {
            free(*ptr as *mut c_void);
            *ptr = core::ptr::null_mut();
        }
    }
}

unsafe fn clean_event_name(info: *mut pmu_event_info) -> *mut c_char {
    unsafe {
        let mut name = (*info).name;
        let pmu_name = (*(*info).pmu).name;
        let pmu_len = strlen(pmu_name);
        let res: *mut c_char;
        let len: usize;

        if strncmp(name, pmu_name, pmu_len) == 0 && *name.add(pmu_len) == b'/' as c_char {
            name = name.add(pmu_len + 1);
        }

        res = strdup(name);
        if res.is_null() {
            return core::ptr::null_mut();
        }

        len = strlen(res);
        if len > 0 && *res.add(len - 1) == b'/' as c_char {
            *res.add(len - 1) = b'\0' as c_char;
        }

        res
    }
}

unsafe extern "C" fn event_cb(state: *mut c_void, info: *mut pmu_event_info) -> c_int {
    unsafe {
        let m = state as *mut match_state;
        let clean_name: *mut c_char;

        if !(*m).event1.is_null() && !(*m).event2.is_null() {
            return 1;
        }

        clean_name = clean_event_name(info);
        if clean_name.is_null() {
            return 0;
        }

        if (*m).event1.is_null() {
            (*m).event1 = clean_name;
        } else if strcmp((*m).event1, clean_name) != 0 {
            (*m).event2 = clean_name;
            return 1;
        } else {
            free(clean_name as *mut c_void);
        }
        0
    }
}

macro_rules! check_cond {
    ($cond:expr, $text:expr, $ret:ident) => {
        if !($cond) {
            unsafe {
                pr_debug(
                    c"FAILED %s:%d %s\n".as_ptr(),
                    c"uncore-event-sorting.rs".as_ptr(),
                    line!() as c_int,
                    $text.as_ptr(),
                );
            }
            $ret = TEST_FAIL;
            break 'out_err;
        }
    };
}

macro_rules! check_equal {
    ($val:expr, $expected:expr, $text:expr, $ret:ident) => {
        if ($val) != ($expected) {
            unsafe {
                pr_debug(
                    c"FAILED %s:%d %s (%d != %d)\n".as_ptr(),
                    c"uncore-event-sorting.rs".as_ptr(),
                    line!() as c_int,
                    $text.as_ptr(),
                    $val,
                    $expected,
                );
            }
            $ret = TEST_FAIL;
            break 'out_err;
        }
    };
}

unsafe fn test__uncore_event_sorting(_test: *mut test_suite, _subtest: c_int) -> c_int {
    unsafe {
        let mut evlist: *mut evlist = core::ptr::null_mut();
        let mut err: parse_events_error = core::mem::zeroed();
        let mut evsel: *mut evsel;
        let mut pmu: *mut perf_pmu = core::ptr::null_mut();
        let mut pmu_prefix: *mut c_char = core::ptr::null_mut();
        let mut m = match_state {
            event1: core::ptr::null_mut(),
            event2: core::ptr::null_mut(),
        };
        let mut buf = [0 as c_char; 1024];
        let mut ret: c_int;

        parse_events_error__init(&mut err);

        'out_err: loop {
            loop {
                pmu = perf_pmus__scan(pmu);
                if pmu.is_null() {
                    break;
                }

                let len: usize;
                let mut sibling: *mut perf_pmu;

                if (*pmu).is_core {
                    continue;
                }

                len = pmu_name_len_no_suffix((*pmu).name);
                if len == strlen((*pmu).name) {
                    continue;
                }

                sibling = pmu;
                loop {
                    sibling = perf_pmus__scan(sibling);
                    if sibling.is_null() {
                        break;
                    }
                    if (*sibling).is_core {
                        continue;
                    }
                    if pmu_name_len_no_suffix((*sibling).name) == len
                        && strncmp((*pmu).name, (*sibling).name, len) == 0
                    {
                        break;
                    }
                }

                if sibling.is_null() {
                    continue;
                }

                m.event1 = core::ptr::null_mut();
                m.event2 = core::ptr::null_mut();
                perf_pmu__for_each_event(
                    pmu,
                    false,
                    &mut m as *mut match_state as *mut c_void,
                    Some(event_cb),
                );

                if !m.event1.is_null() && !m.event2.is_null() {
                    pmu_prefix = strndup((*pmu).name, len);
                    break;
                }
                zfree_char(&mut m.event1);
            }

            if pmu_prefix.is_null() {
                pr_debug(c"No suitable uncore PMU found\n".as_ptr());
                ret = TEST_SKIP;
                break 'out_err;
            }

            evlist = evlist__new();
            if evlist.is_null() {
                ret = TEST_FAIL;
                break 'out_err;
            }

            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                c"{%s/%s/,%s/%s/}".as_ptr(),
                pmu_prefix,
                m.event1,
                pmu_prefix,
                m.event2,
            );
            pr_debug(c"Parsing: %s\n".as_ptr(), buf.as_ptr());

            ret = parse_events(evlist, buf.as_ptr(), &mut err);
            if ret != 0 {
                pr_debug(c"parse_events failed\n".as_ptr());
                ret = TEST_FAIL;
                break 'out_err;
            }

            check_cond!(
                evlist__nr_entries(evlist) >= 4,
                c"Number of events is >= 4",
                ret
            );
            check_equal!(
                evlist__nr_entries(evlist) % 2,
                0,
                c"Number of events is a multiple of 2",
                ret
            );

            evsel = evlist__first(evlist);
            while !evsel.is_null() && !evlist__is_last(evlist, evsel) {
                let next: *mut evsel;

                if !evsel__is_group_leader(evsel) {
                    evsel = evsel__next(evsel);
                    continue;
                }

                next = evsel__next(evsel);
                check_equal!((*evsel).core.nr_members, 2, c"Group size is 2", ret);
                check_cond!((*evsel).pmu == (*next).pmu, c"PMU match", ret);
                check_cond!(
                    !strstr(evsel__name(evsel), m.event1).is_null(),
                    c"First event name",
                    ret
                );
                check_cond!(
                    !strstr(evsel__name(next), m.event2).is_null(),
                    c"Second event name",
                    ret
                );

                evsel = evsel__next(evsel);
            }
            ret = TEST_OK;
            break 'out_err;
        }

        evlist__put(evlist);
        parse_events_error__exit(&mut err);
        zfree_char(&mut pmu_prefix);
        zfree_char(&mut m.event1);
        zfree_char(&mut m.event2);
        ret
    }
}

// DEFINE_SUITE("Uncore event sorting", uncore_event_sorting);
// The original C macro publishes the test suite using repository test
// infrastructure supplied outside this isolated file.
