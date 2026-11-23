// SPDX-License-Identifier: LGPL-2.0+
/*
 * Copyright (C) 1993, 1994, 1995, 1996, 1997 Free Software Foundation, Inc.
 * This file is part of the GNU C Library.
 * Contributed by Paul Eggert (eggert@twinsun.com).
 *
 * The GNU C Library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 *
 * The GNU C Library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with the GNU C Library; see the file COPYING.LIB.  If not,
 * write to the Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Converts the calendar time to broken-down time representation
 *
 * 2009-7-14:
 *   Moved from glibc-2.6 to kernel by Zhaolei<zhaolei@cn.fujitsu.com>
 * 2021-06-02:
 *   Reimplemented by Cassio Neri <cassio.neri@gmail.com>
 */

// Dependencies: linux/time.h, linux/module.h, linux/kernel.h.

use kernel::bindings::{time64_t, tm};
use kernel::ffi::{c_int, c_long};

const SECS_PER_HOUR: c_long = 60 * 60;
const SECS_PER_DAY: c_long = SECS_PER_HOUR * 24;

/**
 * time64_to_tm - converts the calendar time to local broken-down time
 *
 * @totalsecs:	the number of seconds elapsed since 00:00:00 on January 1, 1970,
 *		Coordinated Universal Time (UTC).
 * @offset:	offset seconds adding to totalsecs.
 * @result:	pointer to struct tm variable to receive broken-down time
 */
/// # Safety
/// `result` points to writable `struct tm` storage.
#[no_mangle]
pub unsafe extern "C" fn time64_to_tm(totalsecs: time64_t, offset: c_int, result: *mut tm) {
    // div_s64_rem(totalsecs, SECS_PER_DAY, &remainder)
    let mut days: c_long = (totalsecs / SECS_PER_DAY as i64) as c_long;
    let remainder = (totalsecs % SECS_PER_DAY as i64) as c_int;
    let mut rem: c_long = remainder as c_long;
    rem = rem.wrapping_add(offset as c_long);
    while rem < 0 {
        rem += SECS_PER_DAY;
        days = days.wrapping_sub(1);
    }
    while rem >= SECS_PER_DAY {
        rem -= SECS_PER_DAY;
        days = days.wrapping_add(1);
    }

    // SAFETY: result is writable tm storage.
    unsafe {
        (*result).tm_hour = (rem / SECS_PER_HOUR) as c_int;
        rem %= SECS_PER_HOUR;
        (*result).tm_min = (rem / 60) as c_int;
        (*result).tm_sec = (rem % 60) as c_int;

        /* January 1, 1970 was a Thursday. */
        (*result).tm_wday = (4_isize.wrapping_add(days) % 7) as c_int;
        if (*result).tm_wday < 0 {
            (*result).tm_wday += 7;
        }
    }

    /*
     * The following algorithm is, basically, Proposition 6.3 of Neri
     * and Schneider [1]. In a few words: it works on the computational
     * (fictitious) calendar where the year starts in March, month = 2
     * (*), and finishes in February, month = 13. This calendar is
     * mathematically convenient because the day of the year does not
     * depend on whether the year is leap or not. For instance:
     *
     * March 1st		0-th day of the year;
     * ...
     * April 1st		31-st day of the year;
     * ...
     * January 1st		306-th day of the year; (Important!)
     * ...
     * February 28th	364-th day of the year;
     * February 29th	365-th day of the year (if it exists).
     *
     * After having worked out the date in the computational calendar
     * (using just arithmetics) it's easy to convert it to the
     * corresponding date in the Gregorian calendar.
     *
     * [1] "Euclidean Affine Functions and Applications to Calendar
     * Algorithms". https://arxiv.org/abs/2102.06959
     *
     * (*) The numbering of months follows tm more closely and thus,
     * is slightly different from [1].
     */

    let udays: u64 = (days as u64).wrapping_add(2305843009213814918);

    let mut u64tmp: u64 = udays.wrapping_mul(4).wrapping_add(3);
    // div64_u64_rem(u64tmp, 146097, &u64tmp)
    let century: u64 = u64tmp / 146097;
    u64tmp %= 146097;
    let day_of_century: u32 = (u64tmp / 4) as u32;

    let mut u32tmp: u32 = day_of_century.wrapping_mul(4).wrapping_add(3);
    u64tmp = 2939745u64.wrapping_mul(u64::from(u32tmp));
    let year_of_century: u32 = (u64tmp >> 32) as u32;
    let mut day_of_year: u32 = (u64tmp as u32) / 2939745 / 4;

    let mut year: u64 = century.wrapping_mul(100).wrapping_add(u64::from(year_of_century));
    let is_leap_year: bool = if year_of_century != 0 {
        year_of_century % 4 == 0
    } else {
        century % 4 == 0
    };

    u32tmp = day_of_year.wrapping_mul(2141).wrapping_add(132377);
    let mut month: u32 = u32tmp >> 16;
    let mut day: u32 = u32::from(u32tmp as u16) / 2141;

    /*
     * Recall that January 1st is the 306-th day of the year in the
     * computational (not Gregorian) calendar.
     */
    let is_jan_or_feb: bool = day_of_year >= 306;

    /* Convert to the Gregorian calendar and adjust to Unix time. */
    year = year.wrapping_add(u64::from(is_jan_or_feb)).wrapping_sub(6313183731940000);
    month = if is_jan_or_feb { month.wrapping_sub(12) } else { month };
    day = day.wrapping_add(1);
    let adjust: c_int = if is_jan_or_feb { -306 } else { 31 + 28 + c_int::from(is_leap_year) };
    day_of_year = day_of_year.wrapping_add(adjust as u32);

    // SAFETY: result is writable tm storage.
    unsafe {
        /* Convert to tm's format. */
        (*result).tm_year = year.wrapping_sub(1900) as c_long;
        (*result).tm_mon = month as c_int;
        (*result).tm_mday = day as c_int;
        (*result).tm_yday = day_of_year as c_int;
    }
}
// EXPORT_SYMBOL(time64_to_tm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
