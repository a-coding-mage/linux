// SPDX-License-Identifier: GPL-2.0
// Translated from C source that included:
// "evlist.h", "evsel.h", "parse-events.h", "tests.h", "debug.h",
// and <linux/kernel.h>.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

extern "C" {
    static evsel__hw_names: *const *const c_char;
    static evsel__sw_names: *const *const c_char;

    static PERF_COUNT_HW_CACHE_MAX: c_int;
    static PERF_COUNT_HW_CACHE_OP_MAX: c_int;
    static PERF_COUNT_HW_CACHE_RESULT_MAX: c_int;
    static PERF_COUNT_HW_MAX: c_int;
    static PERF_COUNT_SW_DUMMY: c_int;

    static TEST_OK: c_int;
    static TEST_FAIL: c_int;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn parse_event(evlist: *mut evlist, name: *const c_char) -> c_int;
    fn evsel__is_cache_op_valid(cache_type: c_int, cache_op: c_int) -> bool;
    fn __evsel__hw_cache_type_op_res_name(
        cache_type: c_int,
        cache_op: c_int,
        cache_result: c_int,
        name: *mut c_char,
        size: usize,
    );
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn perf_evsel__roundtrip_cache_name_test() -> c_int {
    let mut ret = TEST_OK;

    let mut type_ = 0;
    while type_ < PERF_COUNT_HW_CACHE_MAX {
        let mut op = 0;
        while op < PERF_COUNT_HW_CACHE_OP_MAX {
            /* skip invalid cache type */
            if !evsel__is_cache_op_valid(type_, op) {
                op += 1;
                continue;
            }

            let mut res = 0;
            while res < PERF_COUNT_HW_CACHE_RESULT_MAX {
                let mut name = [0 as c_char; 128];
                let evlist = evlist__new();
                let mut evsel: *mut evsel;
                let err: c_int;

                if evlist.is_null() {
                    pr_debug(c"Failed to alloc evlist".as_ptr());
                    return TEST_FAIL;
                }
                __evsel__hw_cache_type_op_res_name(
                    type_,
                    op,
                    res,
                    name.as_mut_ptr(),
                    core::mem::size_of_val(&name),
                );

                err = parse_event(evlist, name.as_ptr());
                if err != 0 {
                    pr_debug(
                        c"Failure to parse cache event '%s' possibly as PMUs don't support it"
                            .as_ptr(),
                        name.as_ptr(),
                    );
                    evlist__put(evlist);
                    res += 1;
                    continue;
                }
                /*
                 * C source:
                 * evlist__for_each_entry(evlist, evsel) {
                 *     if (!evsel__name_is(evsel, name)) {
                 *         pr_debug("%s != %s\n", evsel__name(evsel), name);
                 *         ret = TEST_FAIL;
                 *     }
                 * }
                 *
                 * The iterator macro is supplied by the perf evlist headers.
                 */
                evsel = core::ptr::null_mut();
                while evlist__for_each_entry_next(evlist, &mut evsel) {
                    if !evsel__name_is(evsel, name.as_ptr()) {
                        pr_debug(c"%s != %s\n".as_ptr(), evsel__name(evsel), name.as_ptr());
                        ret = TEST_FAIL;
                    }
                }
                evlist__put(evlist);
                res += 1;
            }
            op += 1;
        }
        type_ += 1;
    }
    ret
}

unsafe fn perf_evsel__name_array_test(names: *const *const c_char, nr_names: c_int) -> c_int {
    let mut ret = TEST_OK;

    let mut i = 0;
    while i < nr_names {
        let evlist = evlist__new();
        let mut evsel: *mut evsel;
        let err: c_int;

        if evlist.is_null() {
            pr_debug(c"Failed to alloc evlist".as_ptr());
            return TEST_FAIL;
        }
        err = parse_event(evlist, *names.offset(i as isize));
        if err != 0 {
            pr_debug(
                c"failed to parse event '%s', err %d\n".as_ptr(),
                *names.offset(i as isize),
                err,
            );
            evlist__put(evlist);
            ret = TEST_FAIL;
            i += 1;
            continue;
        }
        /*
         * C source:
         * evlist__for_each_entry(evlist, evsel) {
         *     if (!evsel__name_is(evsel, names[i])) {
         *         pr_debug("%s != %s\n", evsel__name(evsel), names[i]);
         *         ret = TEST_FAIL;
         *     }
         * }
         *
         * The iterator macro is supplied by the perf evlist headers.
         */
        evsel = core::ptr::null_mut();
        while evlist__for_each_entry_next(evlist, &mut evsel) {
            if !evsel__name_is(evsel, *names.offset(i as isize)) {
                pr_debug(
                    c"%s != %s\n".as_ptr(),
                    evsel__name(evsel),
                    *names.offset(i as isize),
                );
                ret = TEST_FAIL;
            }
        }
        evlist__put(evlist);
        i += 1;
    }
    ret
}

unsafe fn test__perf_evsel__roundtrip_name_test(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let mut err: c_int = 0;
    let mut ret = TEST_OK;

    let _ = test;
    let _ = subtest;

    err = perf_evsel__name_array_test(evsel__hw_names, PERF_COUNT_HW_MAX);
    if err != 0 {
        ret = err;
    }

    err = perf_evsel__name_array_test(evsel__sw_names, PERF_COUNT_SW_DUMMY + 1);
    if err != 0 {
        ret = err;
    }

    err = perf_evsel__roundtrip_cache_name_test();
    if err != 0 {
        ret = err;
    }

    ret
}

extern "C" {
    /*
     * Rust placeholder for the evlist__for_each_entry(evlist, evsel) macro's
     * next-entry behavior, supplied by the translated evlist dependency.
     */
    fn evlist__for_each_entry_next(evlist: *mut evlist, evsel: *mut *mut evsel) -> bool;

    /*
     * C source:
     * DEFINE_SUITE("Roundtrip evsel->name", perf_evsel__roundtrip_name_test);
     */
    fn DEFINE_SUITE(name: *const c_char, test: *const c_void);
}

#[used]
static DEFINE_SUITE_ROUNDTRIP_EVSEL_NAME: unsafe extern "C" fn() = {
    unsafe extern "C" fn register_suite() {
        DEFINE_SUITE(
            c"Roundtrip evsel->name".as_ptr(),
            test__perf_evsel__roundtrip_name_test as *const c_void,
        );
    }
    register_suite
};
