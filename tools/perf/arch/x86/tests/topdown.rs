// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/arch/x86/tests/topdown.c.
// C include dependencies are preserved here as external declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ENOMEM: c_int = 12;
const PERF_TYPE_RAW: c_uint = 4;

extern "C" {
    static TEST_OK: c_int;
    static TEST_FAIL: c_int;
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *mut perf_pmu,
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
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
    pub pmu: *mut evsel_pmu,
    pub core: evsel_core,
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_pmu {
    pub type_: c_uint,
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

extern "C" {
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
    fn evlist__is_entry(evlist: *mut evlist, evsel: *mut evsel) -> bool;

    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, event: *const c_char);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_group_leader(evsel: *mut evsel) -> bool;

    fn arch_is_topdown_slots(evsel: *mut evsel) -> bool;
    fn arch_is_topdown_metrics(evsel: *mut evsel) -> bool;
    fn topdown_sys_has_perf_metrics() -> bool;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmu__for_each_event(
        pmu: *mut perf_pmu,
        skip_duplicate_pmus: bool,
        state: *mut c_void,
        cb: unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int,
    ) -> c_int;

    fn pr_debug(format: *const c_char, ...);
    fn test_assert_equal(text: *const c_char, val: c_int, expected: c_int) -> c_int;
}

macro_rules! evlist__for_each_entry {
    ($evlist:expr, $evsel:ident, $body:block) => {{
        $evsel = evlist__first($evlist);
        while evlist__is_entry($evlist, $evsel) {
            $body
            $evsel = evsel__next($evsel);
        }
    }};
}

macro_rules! CHECK_COND {
    ($ret:ident, $cond:expr, $text:expr) => {{
        if !($cond) {
            pr_debug(
                b"FAILED %s:%d %s\n\0".as_ptr() as *const c_char,
                concat!(file!(), "\0").as_ptr() as *const c_char,
                line!() as c_int,
                $text,
            );
            $ret = TEST_FAIL;
            break 'out_err;
        }
    }};
}

macro_rules! CHECK_EQUAL {
    ($ret:ident, $val:expr, $expected:expr, $text:expr) => {{
        if ($val) != ($expected) {
            pr_debug(
                b"FAILED %s:%d %s (%d != %d)\n\0".as_ptr() as *const c_char,
                concat!(file!(), "\0").as_ptr() as *const c_char,
                line!() as c_int,
                $text,
                $val,
                $expected,
            );
            $ret = TEST_FAIL;
            break 'out_err;
        }
    }};
}

macro_rules! TEST_ASSERT_EQUAL {
    ($text:expr, $val:expr, $expected:expr) => {{
        test_assert_equal($text, $val, $expected);
    }};
}

unsafe extern "C" fn event_cb(state: *mut c_void, info: *mut pmu_event_info) -> c_int {
    let mut buf: [c_char; 256] = [0; 256];
    let mut parse_err = core::mem::MaybeUninit::<parse_events_error>::uninit();
    let ret = state as *mut c_int;
    let mut err: c_int;
    let evlist = evlist__new();
    let mut evsel: *mut evsel;

    if evlist.is_null() {
        return -ENOMEM;
    }

    parse_events_error__init(parse_err.as_mut_ptr());
    snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        b"%s/%s/\0".as_ptr() as *const c_char,
        (*(*info).pmu).name,
        (*info).name,
    );
    err = parse_events(evlist, buf.as_ptr(), parse_err.as_mut_ptr());
    if err != 0 {
        parse_events_error__print(parse_err.as_mut_ptr(), buf.as_ptr());
        *ret = TEST_FAIL;
    }
    parse_events_error__exit(parse_err.as_mut_ptr());
    evlist__for_each_entry!(evlist, evsel, {
        let mut fail = false;
        let p_core_pmu = (*(*evsel).pmu).type_ == PERF_TYPE_RAW;
        let name = evsel__name(evsel);

        if !strcasestr(name, b"uops_retired.slots\0".as_ptr() as *const c_char).is_null()
            || !strcasestr(
                name,
                b"topdown.backend_bound_slots\0".as_ptr() as *const c_char,
            )
            .is_null()
            || !strcasestr(
                name,
                b"topdown.br_mispredict_slots\0".as_ptr() as *const c_char,
            )
            .is_null()
            || !strcasestr(
                name,
                b"topdown.memory_bound_slots\0".as_ptr() as *const c_char,
            )
            .is_null()
            || !strcasestr(name, b"topdown.bad_spec_slots\0".as_ptr() as *const c_char)
                .is_null()
            || !strcasestr(name, b"topdown.slots_p\0".as_ptr() as *const c_char).is_null()
        {
            if arch_is_topdown_slots(evsel) || arch_is_topdown_metrics(evsel) {
                fail = true;
            }
        } else if !strcasestr(name, b"slots\0".as_ptr() as *const c_char).is_null() {
            if arch_is_topdown_slots(evsel) != p_core_pmu || arch_is_topdown_metrics(evsel) {
                fail = true;
            }
        } else if !strcasestr(name, b"topdown\0".as_ptr() as *const c_char).is_null() {
            if arch_is_topdown_slots(evsel) || arch_is_topdown_metrics(evsel) != p_core_pmu {
                fail = true;
            }
        } else if arch_is_topdown_slots(evsel) || arch_is_topdown_metrics(evsel) {
            fail = true;
        }
        if fail {
            pr_debug(
                b"Broken topdown information for '%s'\n\0".as_ptr() as *const c_char,
                evsel__name(evsel),
            );
            *ret = TEST_FAIL;
        }
    });
    evlist__put(evlist);
    0
}

unsafe extern "C" fn test__x86_topdown(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret = TEST_OK;
    let mut pmu: *mut perf_pmu = ptr::null_mut();

    if !topdown_sys_has_perf_metrics() {
        return TEST_OK;
    }

    loop {
        pmu = perf_pmus__scan_core(pmu);
        if pmu.is_null() {
            break;
        }
        if perf_pmu__for_each_event(pmu, false, &mut ret as *mut _ as *mut c_void, event_cb) != 0 {
            break;
        }
    }
    ret
}

unsafe fn test_sort(
    str_: *const c_char,
    expected_slots_group_size: c_int,
    expected_instructions_group_size: c_int,
) -> c_int {
    let mut evlist: *mut evlist = ptr::null_mut();
    let mut err = core::mem::MaybeUninit::<parse_events_error>::uninit();
    let mut evsel: *mut evsel;
    let mut ret = TEST_FAIL;
    let mut slots_seen = false;

    parse_events_error__init(err.as_mut_ptr());

    'out_err: loop {
        evlist = evlist__new();
        if evlist.is_null() {
            break 'out_err;
        }

        if parse_events(evlist, str_, err.as_mut_ptr()) != 0 {
            pr_debug(b"parse_events failed for %s\n\0".as_ptr() as *const c_char, str_);
            break 'out_err;
        }

        evlist__for_each_entry!(evlist, evsel, {
            if !evsel__is_group_leader(evsel) {
                continue;
            }

            if !strstr(evsel__name(evsel), b"slots\0".as_ptr() as *const c_char).is_null() {
                /*
                 * Slots as a leader means the PMU is for a perf metric
                 * group as the slots event isn't present when not.
                 */
                slots_seen = true;
                CHECK_EQUAL!(
                    ret,
                    (*evsel).core.nr_members,
                    expected_slots_group_size,
                    b"slots group size\0".as_ptr() as *const c_char
                );
                if expected_slots_group_size == 3 {
                    let next = evsel__next(evsel);
                    let next2 = evsel__next(next);

                    CHECK_COND!(
                        ret,
                        !strstr(evsel__name(next), b"instructions\0".as_ptr() as *const c_char)
                            .is_null(),
                        b"slots second event is instructions\0".as_ptr() as *const c_char
                    );
                    CHECK_COND!(
                        ret,
                        !strstr(
                            evsel__name(next2),
                            b"topdown-retiring\0".as_ptr() as *const c_char,
                        )
                        .is_null(),
                        b"slots third event is topdown-retiring\0".as_ptr() as *const c_char
                    );
                } else if expected_slots_group_size == 2 {
                    let next = evsel__next(evsel);

                    CHECK_COND!(
                        ret,
                        !strstr(
                            evsel__name(next),
                            b"topdown-retiring\0".as_ptr() as *const c_char,
                        )
                        .is_null(),
                        b"slots second event is topdown-retiring\0".as_ptr() as *const c_char
                    );
                }
            } else if !strstr(evsel__name(evsel), b"instructions\0".as_ptr() as *const c_char)
                .is_null()
            {
                CHECK_EQUAL!(
                    ret,
                    (*evsel).core.nr_members,
                    expected_instructions_group_size,
                    b"instructions group size\0".as_ptr() as *const c_char
                );
                if expected_instructions_group_size == 2 {
                    /*
                     * On Intel hybrid CPUs (e.g., Alder Lake/
                     * Raptor Lake), E-cores (cpu_atom) do not
                     * support/enforce the slots event. When
                     * parsing event groups containing slots
                     * across all PMUs, slots is automatically
                     * filtered out from cpu_atom, leaving
                     * {cpu_atom/instructions/,
                     *  cpu_atom/topdown-retiring/}. On cpu_atom,
                     * instructions correctly leads this group of
                     * 2 without slots reordering.
                     */
                    let next = evsel__next(evsel);

                    CHECK_COND!(
                        ret,
                        !strstr(
                            evsel__name(next),
                            b"topdown-retiring\0".as_ptr() as *const c_char,
                        )
                        .is_null(),
                        b"instructions second event is topdown-retiring\0".as_ptr() as *const c_char
                    );
                }
            } else if !strstr(
                evsel__name(evsel),
                b"topdown-retiring\0".as_ptr() as *const c_char,
            )
            .is_null()
            {
                /*
                 * A perf metric event where the PMU doesn't require
                 * slots as a leader.
                 */
                CHECK_EQUAL!(
                    ret,
                    (*evsel).core.nr_members,
                    1,
                    b"topdown-retiring group size\0".as_ptr() as *const c_char
                );
            } else if !strstr(evsel__name(evsel), b"cycles\0".as_ptr() as *const c_char).is_null()
            {
                CHECK_EQUAL!(
                    ret,
                    (*evsel).core.nr_members,
                    1,
                    b"cycles group size\0".as_ptr() as *const c_char
                );
            }
        });
        CHECK_COND!(
            ret,
            slots_seen,
            b"slots seen\0".as_ptr() as *const c_char
        );
        ret = TEST_OK;
        break 'out_err;
    }
    evlist__put(evlist);
    parse_events_error__exit(err.as_mut_ptr());
    ret
}

unsafe extern "C" fn test__x86_topdown_sorting(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int;

    if !topdown_sys_has_perf_metrics() {
        return TEST_OK;
    }

    ret = test_sort(
        b"{instructions,topdown-retiring,slots}\0".as_ptr() as *const c_char,
        3,
        2,
    );
    TEST_ASSERT_EQUAL!(
        b"all events in a group\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"instructions,topdown-retiring,slots\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"all events not in a group\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions,slots},topdown-retiring\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"slots event in a group but topdown metrics events outside the group\0".as_ptr()
            as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions,slots},{topdown-retiring}\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"slots event and topdown metrics events in two groups\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions,slots},cycles,topdown-retiring\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"slots event and metrics event are not in a group and not adjacent\0".as_ptr()
            as *const c_char,
        ret,
        TEST_OK
    );

    TEST_OK
}

unsafe extern "C" fn test__x86_topdown_slots_injection(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int;

    if !topdown_sys_has_perf_metrics() {
        return TEST_OK;
    }

    ret = test_sort(
        b"{instructions,topdown-retiring}\0".as_ptr() as *const c_char,
        3,
        2,
    );
    TEST_ASSERT_EQUAL!(
        b"all events in a group\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"instructions,topdown-retiring\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"all events not in a group\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions},topdown-retiring\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"event in a group but topdown metrics events outside the group\0".as_ptr()
            as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions},{topdown-retiring}\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"event and topdown metrics events in two groups\0".as_ptr() as *const c_char,
        ret,
        TEST_OK
    );
    ret = test_sort(
        b"{instructions},cycles,topdown-retiring\0".as_ptr() as *const c_char,
        2,
        1,
    );
    TEST_ASSERT_EQUAL!(
        b"event and metrics event are not in a group and not adjacent\0".as_ptr()
            as *const c_char,
        ret,
        TEST_OK
    );

    TEST_OK
}

#[no_mangle]
pub static mut x86_topdown_tests: [test_case; 4] = [
    test_case {
        name: b"topdown events\0".as_ptr() as *const c_char,
        run_case: Some(test__x86_topdown),
    },
    test_case {
        name: b"topdown sorting\0".as_ptr() as *const c_char,
        run_case: Some(test__x86_topdown_sorting),
    },
    test_case {
        name: b"topdown slots injection\0".as_ptr() as *const c_char,
        run_case: Some(test__x86_topdown_slots_injection),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
    },
];

#[no_mangle]
pub static mut suite__x86_topdown: test_suite = test_suite {
    desc: b"x86 topdown\0".as_ptr() as *const c_char,
    test_cases: unsafe { x86_topdown_tests.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
