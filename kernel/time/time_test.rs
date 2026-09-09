// SPDX-License-Identifier: LGPL-2.1+

// Dependencies supplied by the kernel KUnit and time interfaces.

/*
 * Traditional implementation of leap year evaluation, but note that long
 * is a signed type and the tests do cover negative year values. So this
 * can't use the is_leap_year() helper from rtc.h.
 */
fn is_leap(year: libc::c_long) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/*
 * Gets the last day of a month.
 */
fn last_day_of_month(year: libc::c_long, month: libc::c_int) -> libc::c_int {
    if month == 2 {
        return 28 + is_leap(year) as libc::c_int;
    }
    if month == 4 || month == 6 || month == 9 || month == 11 {
        return 30;
    }
    31
}

/*
 * Advances a date by one day.
 */
unsafe fn advance_date(
    year: *mut libc::c_long,
    month: *mut libc::c_int,
    mday: *mut libc::c_int,
    yday: *mut libc::c_int,
) {
    if *mday != last_day_of_month(*year, *month) {
        *mday += 1;
        *yday += 1;
        return;
    }

    *mday = 1;
    if *month != 12 {
        *month += 1;
        *yday += 1;
        return;
    }

    *month = 1;
    *yday = 0;
    *year += 1;
}

/*
 * Checks every day in a 160000 years interval centered at 1970-01-01
 * against the expected result.
 */
unsafe fn time64_to_tm_test_date_range(test: *mut kunit) {
    /*
     * 80000 years = (80000 / 400) * 400 years
     *              = (80000 / 400) * 146097 days
     *              = (80000 / 400) * 146097 * 86400 seconds
     */
    let total_secs: time64_t = (80000 as time64_t) / 400 * 146097 * 86400;
    let mut year: libc::c_long = 1970 - 80000;
    let mut month: libc::c_int = 1;
    let mut mdday: libc::c_int = 1;
    let mut yday: libc::c_int = 0;

    let mut result: tm = core::mem::zeroed();
    let mut secs: time64_t;
    let mut days: s64;

    secs = -total_secs;
    while secs <= total_secs {
        time64_to_tm(secs, 0, &mut result);

        days = div_s64(secs, 86400);

        KUNIT_ASSERT_EQ_MSG!(test, year - 1900, result.tm_year,
            "{:05}/{:02}/{:02} ({:2}) : {}", year, month, mdday, yday, days);
        KUNIT_ASSERT_EQ_MSG!(test, month - 1, result.tm_mon,
            "{:05}/{:02}/{:02} ({:2}) : {}", year, month, mdday, yday, days);
        KUNIT_ASSERT_EQ_MSG!(test, mdday, result.tm_mday,
            "{:05}/{:02}/{:02} ({:2}) : {}", year, month, mdday, yday, days);
        KUNIT_ASSERT_EQ_MSG!(test, yday, result.tm_yday,
            "{:05}/{:02}/{:02} ({:2}) : {}", year, month, mdday, yday, days);

        advance_date(&mut year, &mut month, &mut mdday, &mut yday);
        secs += 86400;
    }
}

static mut time_test_cases: [kunit_case; 2] = [
    KUNIT_CASE_SLOW!(time64_to_tm_test_date_range),
    kunit_case {},
];

static mut time_test_suite: kunit_suite = kunit_suite {
    name: "time_test_cases",
    test_cases: time_test_cases.as_mut_ptr(),
};

kunit_test_suite!(time_test_suite);
module_description!("time unit test suite");
module_license!("GPL");

// External types and functions supplied by the translated kernel dependencies.
type time64_t = i64;
type s64 = i64;

#[repr(C)]
struct tm {
    tm_year: libc::c_int,
    tm_mon: libc::c_int,
    tm_mday: libc::c_int,
    tm_yday: libc::c_int,
}

#[repr(C)]
struct kunit;
#[repr(C)]
struct kunit_case;
#[repr(C)]
struct kunit_suite {
    name: &'static str,
    test_cases: *mut kunit_case,
}

extern "C" {
    fn time64_to_tm(secs: time64_t, offset: libc::c_int, result: *mut tm);
    fn div_s64(dividend: s64, divisor: s64) -> s64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
