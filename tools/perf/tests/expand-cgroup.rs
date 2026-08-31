// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/expand-cgroup.c.
// Original C dependencies:
// tests.h, debug.h, evlist.h, cgroup.h, rblist.h, metricgroup.h,
// parse-events.h, pmu-events/pmu-events.h, pfm.h, target.h,
// subcmd/parse-options.h, stdio.h, stdlib.h, string.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct evsel_core {
    pub nr_members: c_int,
}

#[repr(C)]
pub struct cgroup {
    pub name: *const c_char,
}

#[repr(C)]
pub struct evsel {
    pub name: *const c_char,
    pub cgrp: *mut cgroup,
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct pmu_metrics_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub event_group: bool,
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);

    fn evlist__empty(evlist: *mut evlist) -> bool;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__expand_cgroup(evlist: *mut evlist, str_: *const c_char, open: bool) -> c_int;
    fn evlist__new_default(target: *mut target, sample_callchains: bool) -> *mut evlist;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);

    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;

    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, event: *const c_char);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;

    fn parse_libpfm_events_option(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;

    fn find_core_metrics_table(arch: *const c_char, cpuid: *const c_char) -> *const pmu_metrics_table;
    fn metricgroup__parse_groups_test(
        evlist: *mut evlist,
        table: *const pmu_metrics_table,
        str_: *const c_char,
        cputype_filter: bool,
    ) -> c_int;
}

unsafe fn test_expand_events(evlist: *mut evlist) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = TEST_FAIL;
    let nr_events: c_int;
    let was_group_event: bool;
    let nr_members: c_int; /* for the first evsel only */
    let cgrp_str = b"A,B,C\0";
    let cgrp_name: [*const c_char; 3] = [
        b"A\0".as_ptr() as *const c_char,
        b"B\0".as_ptr() as *const c_char,
        b"C\0".as_ptr() as *const c_char,
    ];
    let nr_cgrps: c_int = cgrp_name.len() as c_int;
    let ev_name: *mut *mut c_char;
    let mut evsel: *mut evsel;

    TEST_ASSERT_VAL!(b"evlist is empty\0".as_ptr() as *const c_char, !evlist__empty(evlist));

    nr_events = evlist__nr_entries(evlist);
    ev_name = calloc(nr_events as usize, mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if ev_name.is_null() {
        pr_debug(b"memory allocation failure\n\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }
    'out: loop {
        i = 0;
        evlist__for_each_entry!(evlist, evsel, {
            *ev_name.add(i as usize) = strdup((*evsel).name);
            if (*ev_name.add(i as usize)).is_null() {
                pr_debug(b"memory allocation failure\n\0".as_ptr() as *const c_char);
                break 'out;
            }
            i += 1;
        });
        /* remember grouping info */
        was_group_event = evsel__is_group_event(evlist__first(evlist));
        nr_members = (*evlist__first(evlist)).core.nr_members;

        ret = evlist__expand_cgroup(evlist, cgrp_str.as_ptr() as *const c_char, false);
        if ret < 0 {
            pr_debug(b"failed to expand events for cgroups\n\0".as_ptr() as *const c_char);
            break;
        }

        ret = TEST_FAIL;
        if evlist__nr_entries(evlist) != nr_events * nr_cgrps {
            pr_debug(b"event count doesn't match\n\0".as_ptr() as *const c_char);
            break;
        }

        i = 0;
        evlist__for_each_entry!(evlist, evsel, {
            if !evsel__name_is(evsel, *ev_name.add((i % nr_events) as usize)) {
                pr_debug(b"event name doesn't match:\n\0".as_ptr() as *const c_char);
                pr_debug(
                    b"  evsel[%d]: %s\n  expected: %s\n\0".as_ptr() as *const c_char,
                    i,
                    (*evsel).name,
                    *ev_name.add((i % nr_events) as usize),
                );
                break 'out;
            }
            if strcmp((*(*evsel).cgrp).name, cgrp_name[(i / nr_events) as usize]) != 0 {
                pr_debug(b"cgroup name doesn't match:\n\0".as_ptr() as *const c_char);
                pr_debug(
                    b"  evsel[%d]: %s\n  expected: %s\n\0".as_ptr() as *const c_char,
                    i,
                    (*(*evsel).cgrp).name,
                    cgrp_name[(i / nr_events) as usize],
                );
                break 'out;
            }

            if (i % nr_events) == 0 {
                if evsel__is_group_event(evsel) != was_group_event {
                    pr_debug(
                        b"event group doesn't match: got %s, expect %s\n\0".as_ptr() as *const c_char,
                        if evsel__is_group_event(evsel) {
                            b"true\0".as_ptr() as *const c_char
                        } else {
                            b"false\0".as_ptr() as *const c_char
                        },
                        if was_group_event {
                            b"true\0".as_ptr() as *const c_char
                        } else {
                            b"false\0".as_ptr() as *const c_char
                        },
                    );
                    break 'out;
                }
                if (*evsel).core.nr_members != nr_members {
                    pr_debug(
                        b"event group member doesn't match: %d vs %d\n\0".as_ptr() as *const c_char,
                        (*evsel).core.nr_members,
                        nr_members,
                    );
                    break 'out;
                }
            }
            i += 1;
        });
        ret = TEST_OK;
        break;
    }

    i = 0;
    while i < nr_events {
        free(*ev_name.add(i as usize) as *mut c_void);
        i += 1;
    }
    free(ev_name as *mut c_void);
    ret
}

unsafe fn expand_default_events() -> c_int {
    let ret: c_int;
    let mut target: target = mem::zeroed();
    let evlist: *mut evlist = evlist__new_default(&mut target, false /*sample_callchains=*/);

    TEST_ASSERT_VAL!(b"failed to get evlist\0".as_ptr() as *const c_char, !evlist.is_null());

    ret = test_expand_events(evlist);
    evlist__put(evlist);
    ret
}

unsafe fn expand_group_events() -> c_int {
    let mut ret: c_int;
    let evlist: *mut evlist;
    let mut err: parse_events_error = mem::zeroed();
    let event_str = b"{cycles,instructions}\0";

    symbol_conf.event_group = true;

    evlist = evlist__new();
    TEST_ASSERT_VAL!(b"failed to get evlist\0".as_ptr() as *const c_char, !evlist.is_null());

    'out: loop {
        parse_events_error__init(&mut err);
        ret = parse_events(evlist, event_str.as_ptr() as *const c_char, &mut err);
        if ret < 0 {
            pr_debug(
                b"failed to parse event '%s', err %d\n\0".as_ptr() as *const c_char,
                event_str.as_ptr() as *const c_char,
                ret,
            );
            parse_events_error__print(&mut err, event_str.as_ptr() as *const c_char);
            break 'out;
        }

        ret = test_expand_events(evlist);
        break;
    }
    parse_events_error__exit(&mut err);
    evlist__put(evlist);
    ret
}

unsafe fn expand_libpfm_events() -> c_int {
    let mut ret: c_int;
    let evlist: *mut evlist;
    let event_str = b"CYCLES\0";
    let mut opt = option {
        value: ptr::null_mut(),
    };

    symbol_conf.event_group = true;

    evlist = evlist__new();
    opt.value = &evlist as *const *mut evlist as *mut c_void;
    TEST_ASSERT_VAL!(b"failed to get evlist\0".as_ptr() as *const c_char, !evlist.is_null());

    'out: loop {
        ret = parse_libpfm_events_option(&opt, event_str.as_ptr() as *const c_char, 0);
        if ret < 0 {
            pr_debug(
                b"failed to parse libpfm event '%s', err %d\n\0".as_ptr() as *const c_char,
                event_str.as_ptr() as *const c_char,
                ret,
            );
            break 'out;
        }
        if evlist__empty(evlist) {
            pr_debug(b"libpfm was not enabled\n\0".as_ptr() as *const c_char);
            break 'out;
        }

        ret = test_expand_events(evlist);
        break;
    }
    evlist__put(evlist);
    ret
}

unsafe fn expand_metric_events() -> c_int {
    let mut ret: c_int;
    let evlist: *mut evlist;
    let metric_str = b"CPI\0";
    let pme_test: *const pmu_metrics_table;

    evlist = evlist__new();
    TEST_ASSERT_VAL!(b"failed to get evlist\0".as_ptr() as *const c_char, !evlist.is_null());

    'out: loop {
        pme_test = find_core_metrics_table(
            b"testarch\0".as_ptr() as *const c_char,
            b"testcpu\0".as_ptr() as *const c_char,
        );
        ret = metricgroup__parse_groups_test(
            evlist,
            pme_test,
            metric_str.as_ptr() as *const c_char,
            false, /*cputype_filter=*/
        );
        if ret < 0 {
            pr_debug(
                b"failed to parse '%s' metric\n\0".as_ptr() as *const c_char,
                metric_str.as_ptr() as *const c_char,
            );
            break 'out;
        }

        ret = test_expand_events(evlist);
        break;
    }
    evlist__put(evlist);
    ret
}

unsafe fn test__expand_cgroup_events(
    test: *mut test_suite, /* __maybe_unused */
    subtest: c_int,       /* __maybe_unused */
) -> c_int {
    let mut ret: c_int;

    let _ = test;
    let _ = subtest;

    ret = expand_default_events();
    TEST_ASSERT_EQUAL!(b"failed to expand default events\0".as_ptr() as *const c_char, ret, 0);

    ret = expand_group_events();
    TEST_ASSERT_EQUAL!(b"failed to expand event group\0".as_ptr() as *const c_char, ret, 0);

    ret = expand_libpfm_events();
    TEST_ASSERT_EQUAL!(b"failed to expand event group\0".as_ptr() as *const c_char, ret, 0);

    ret = expand_metric_events();
    TEST_ASSERT_EQUAL!(b"failed to expand metric events\0".as_ptr() as *const c_char, ret, 0);

    ret
}

DEFINE_SUITE!(b"Event expansion for cgroups\0".as_ptr() as *const c_char, expand_cgroup_events);
