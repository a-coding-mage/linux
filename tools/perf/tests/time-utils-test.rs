// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/time-utils-test.c. C includes are represented by
// external declarations for symbols supplied elsewhere in the repository.

use core::ffi::{c_char, c_int, c_void};

type u64 = u64;

const NSEC_PER_SEC: u64 = 1_000_000_000;
const TEST_MAX: usize = 64;

#[repr(C)]
pub struct perf_time_interval {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
struct test_data {
    str: *const c_char,
    first: u64,
    last: u64,
    ptime: [perf_time_interval; TEST_MAX],
    num: c_int,
    skip: [u64; TEST_MAX],
    noskip: [u64; TEST_MAX],
}

const TEST_FAIL: c_int = -1;

unsafe extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
    fn parse_nsec_time(str: *const c_char, ptime: *mut u64) -> c_int;
    fn perf_time__parse_str(ptime: *mut perf_time_interval, ostr: *const c_char) -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__set_first_sample_time(evlist: *mut evlist, time: u64);
    fn evlist__set_last_sample_time(evlist: *mut evlist, time: u64);
    fn perf_time__parse_for_ranges(
        str: *const c_char,
        session: *mut perf_session,
        ptime: *mut *mut perf_time_interval,
        range_size: *mut c_int,
        range_num: *mut c_int,
    ) -> c_int;
    fn perf_time__ranges_skip_sample(
        ptime: *mut perf_time_interval,
        num: c_int,
        timestamp: u64,
    ) -> bool;
    fn evlist__put(evlist: *mut evlist);
    fn free(ptr: *mut c_void);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn pti(start: u64, end: u64) -> perf_time_interval {
    perf_time_interval { start, end }
}

fn test__parse_nsec_time(str_: *const c_char, expected: u64) -> bool {
    let mut ptime: u64 = 0;
    let err: c_int;

    unsafe {
        pr_debug(c_str!("\nparse_nsec_time(\"%s\")\n"), str_);

        err = parse_nsec_time(str_, &mut ptime);
        if err != 0 {
            pr_debug(c_str!("error %d\n"), err);
            return false;
        }

        if ptime != expected {
            pr_debug(
                c_str!("Failed. ptime %llu expected %llu\n"),
                ptime,
                expected,
            );
            return false;
        }

        pr_debug(c_str!("%llu\n"), ptime);
    }

    true
}

fn test__perf_time__parse_str(ostr: *const c_char, start: u64, end: u64) -> bool {
    let mut ptime = perf_time_interval { start: 0, end: 0 };
    let err: c_int;

    unsafe {
        pr_debug(c_str!("\nperf_time__parse_str(\"%s\")\n"), ostr);

        err = perf_time__parse_str(&mut ptime, ostr);
        if err != 0 {
            pr_debug(c_str!("Error %d\n"), err);
            return false;
        }

        if ptime.start != start || ptime.end != end {
            pr_debug(c_str!("Failed. Expected %llu to %llu\n"), start, end);
            return false;
        }
    }

    true
}

fn test__perf_time__parse_for_ranges(d: *mut test_data) -> bool {
    let evlist = unsafe { evlist__new() };
    let mut session = perf_session { evlist };
    let mut ptime: *mut perf_time_interval = core::ptr::null_mut();
    let mut range_size: c_int = 0;
    let mut range_num: c_int = 0;
    let mut pass = false;
    let mut i: c_int;
    let err: c_int;

    unsafe {
        if evlist.is_null() {
            pr_debug(c_str!("Missing evlist\n"));
            return false;
        }
        evlist__set_first_sample_time(evlist, (*d).first);
        evlist__set_last_sample_time(evlist, (*d).last);
        pr_debug(c_str!("\nperf_time__parse_for_ranges(\"%s\")\n"), (*d).str);

        if !strchr((*d).str, '%' as c_int).is_null() {
            pr_debug(
                c_str!("first_sample_time %llu last_sample_time %llu\n"),
                (*d).first,
                (*d).last,
            );
        }

        err = perf_time__parse_for_ranges(
            (*d).str,
            &mut session,
            &mut ptime,
            &mut range_size,
            &mut range_num,
        );
        if err != 0 {
            pr_debug(c_str!("error %d\n"), err);
            evlist__put(evlist);
            free(ptime as *mut c_void);
            return pass;
        }

        if range_size < (*d).num || range_num != (*d).num {
            pr_debug(
                c_str!("bad size: range_size %d range_num %d expected num %d\n"),
                range_size,
                range_num,
                (*d).num,
            );
            evlist__put(evlist);
            free(ptime as *mut c_void);
            return pass;
        }

        i = 0;
        while i < (*d).num {
            if (*ptime.add(i as usize)).start != (*d).ptime[i as usize].start
                || (*ptime.add(i as usize)).end != (*d).ptime[i as usize].end
            {
                pr_debug(
                    c_str!("bad range %d expected %llu to %llu\n"),
                    i,
                    (*d).ptime[i as usize].start,
                    (*d).ptime[i as usize].end,
                );
                evlist__put(evlist);
                free(ptime as *mut c_void);
                return pass;
            }
            i += 1;
        }

        if perf_time__ranges_skip_sample(ptime, (*d).num, 0) {
            pr_debug(c_str!("failed to keep 0\n"));
            evlist__put(evlist);
            free(ptime as *mut c_void);
            return pass;
        }

        i = 0;
        while i < TEST_MAX as c_int {
            if (*d).skip[i as usize] != 0
                && !perf_time__ranges_skip_sample(ptime, (*d).num, (*d).skip[i as usize])
            {
                pr_debug(c_str!("failed to skip %llu\n"), (*d).skip[i as usize]);
                evlist__put(evlist);
                free(ptime as *mut c_void);
                return pass;
            }
            if (*d).noskip[i as usize] != 0
                && perf_time__ranges_skip_sample(ptime, (*d).num, (*d).noskip[i as usize])
            {
                pr_debug(c_str!("failed to keep %llu\n"), (*d).noskip[i as usize]);
                evlist__put(evlist);
                free(ptime as *mut c_void);
                return pass;
            }
            i += 1;
        }

        pass = true;
        evlist__put(evlist);
        free(ptime as *mut c_void);
    }

    pass
}

fn empty_ptime() -> [perf_time_interval; TEST_MAX] {
    [pti(0, 0); TEST_MAX]
}

fn empty_u64() -> [u64; TEST_MAX] {
    [0; TEST_MAX]
}

fn test__time_utils(_t: *mut test_suite, _subtest: c_int) -> c_int {
    let mut pass = true;

    pass &= test__parse_nsec_time(c_str!("0"), 0);
    pass &= test__parse_nsec_time(c_str!("1"), 1000000000u64);
    pass &= test__parse_nsec_time(c_str!("0.000000001"), 1);
    pass &= test__parse_nsec_time(c_str!("1.000000001"), 1000000001u64);
    pass &= test__parse_nsec_time(c_str!("123456.123456"), 123456123456000u64);
    pass &= test__parse_nsec_time(c_str!("1234567.123456789"), 1234567123456789u64);
    pass &= test__parse_nsec_time(c_str!("18446744073.709551615"), 0xFFFFFFFFFFFFFFFFu64);

    pass &= test__perf_time__parse_str(
        c_str!("1234567.123456789,1234567.123456789"),
        1234567123456789u64,
        1234567123456789u64,
    );
    pass &= test__perf_time__parse_str(
        c_str!("1234567.123456789,1234567.123456790"),
        1234567123456789u64,
        1234567123456790u64,
    );
    pass &= test__perf_time__parse_str(
        c_str!("1234567.123456789,"),
        1234567123456789u64,
        0,
    );
    pass &= test__perf_time__parse_str(c_str!(",1234567.123456789"), 0, 1234567123456789u64);
    pass &= test__perf_time__parse_str(c_str!("0,1234567.123456789"), 0, 1234567123456789u64);

    {
        let b = 1234567123456789u64;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b, b + 1);
        let mut skip = empty_u64();
        skip[0] = b - 1;
        skip[1] = b + 2;
        let mut noskip = empty_u64();
        noskip[0] = b;
        noskip[1] = b + 1;
        let mut d = test_data {
            str: c_str!("1234567.123456789,1234567.123456790"),
            first: 0,
            last: 0,
            ptime,
            num: 1,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    {
        let b = 1234567123456789u64;
        let c = 7654321987654321u64;
        let e = 8000000000000000u64;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b, b + 1);
        ptime[1] = pti(c, c + 123);
        ptime[2] = pti(e, e + 5);
        let mut skip = empty_u64();
        skip[0] = b - 1;
        skip[1] = b + 2;
        skip[2] = c - 1;
        skip[3] = c + 124;
        skip[4] = e - 1;
        skip[5] = e + 6;
        let mut noskip = empty_u64();
        noskip[0] = b;
        noskip[1] = b + 1;
        noskip[2] = c;
        noskip[3] = c + 123;
        noskip[4] = e;
        noskip[5] = e + 5;
        let mut d = test_data {
            str: c_str!(
                "1234567.123456789,1234567.123456790 \
                 7654321.987654321,7654321.987654444 \
                 8000000,8000000.000000005"
            ),
            first: 0,
            last: 0,
            ptime,
            num: 3,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    {
        let b = 7654321u64 * NSEC_PER_SEC;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b, b + 9);
        let mut skip = empty_u64();
        skip[0] = b - 1;
        skip[1] = b + 10;
        let mut noskip = empty_u64();
        noskip[0] = b;
        noskip[1] = b + 9;
        let mut d = test_data {
            str: c_str!("10%/1"),
            first: b,
            last: b + 100,
            ptime,
            num: 1,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    {
        let b = 7654321u64 * NSEC_PER_SEC;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b + 10, b + 19);
        let mut skip = empty_u64();
        skip[0] = b + 9;
        skip[1] = b + 20;
        let mut noskip = empty_u64();
        noskip[0] = b + 10;
        noskip[1] = b + 19;
        let mut d = test_data {
            str: c_str!("10%/2"),
            first: b,
            last: b + 100,
            ptime,
            num: 1,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    {
        let b = 11223344u64 * NSEC_PER_SEC;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b, b + 9);
        ptime[1] = pti(b + 10, b + 19);
        let mut skip = empty_u64();
        skip[0] = b - 1;
        skip[1] = b + 20;
        let mut noskip = empty_u64();
        noskip[0] = b;
        noskip[1] = b + 8;
        noskip[2] = b + 9;
        noskip[3] = b + 10;
        noskip[4] = b + 11;
        noskip[5] = b + 12;
        noskip[6] = b + 19;
        let mut d = test_data {
            str: c_str!("10%/1,10%/2"),
            first: b,
            last: b + 100,
            ptime,
            num: 2,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    {
        let b = 11223344u64 * NSEC_PER_SEC;
        let mut ptime = empty_ptime();
        ptime[0] = pti(b, b + 9);
        ptime[1] = pti(b + 20, b + 29);
        ptime[2] = pti(b + 90, b + 100);
        let mut skip = empty_u64();
        skip[0] = b - 1;
        skip[1] = b + 10;
        skip[2] = b + 19;
        skip[3] = b + 30;
        skip[4] = b + 89;
        skip[5] = b + 101;
        let mut noskip = empty_u64();
        noskip[0] = b;
        noskip[1] = b + 9;
        noskip[2] = b + 20;
        noskip[3] = b + 29;
        noskip[4] = b + 90;
        noskip[5] = b + 100;
        let mut d = test_data {
            str: c_str!("10%/1,10%/3,10%/10"),
            first: b,
            last: b + 100,
            ptime,
            num: 3,
            skip,
            noskip,
        };

        pass &= test__perf_time__parse_for_ranges(&mut d);
    }

    unsafe {
        pr_debug(c_str!("\n"));
    }

    if pass {
        0
    } else {
        TEST_FAIL
    }
}

// DEFINE_SUITE("time utils", time_utils);
// The suite registration macro is provided by the surrounding test framework.
define_suite!("time utils", time_utils);
