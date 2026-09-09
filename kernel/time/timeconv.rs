// SPDX-License-Identifier: LGPL-2.0+
/*
 * Copyright (C) 1993, 1994, 1995, 1996, 1997 Free Software Foundation, Inc.
 * This file is part of the GNU C Library.
 * Contributed by Paul Eggert (eggert@twinsun.com).
 *
 * The GNU C Library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License
 * as published by the Free Software Foundation; either version 2.
 * of the License, or (at your option) any later version.
 *
 * The GNU C Library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU General Public
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

const SECS_PER_HOUR: i64 = 60 * 60;
const SECS_PER_DAY: i64 = SECS_PER_HOUR * 24;

/**
 * time64_to_tm - converts the calendar time to local broken-down time
 *
 * @totalsecs: the number of seconds elapsed since 00:00:00 on January 1, 1970,
 *             Coordinated Universal Time (UTC).
 * @offset:    offset seconds adding to totalsecs.
 * @result:    pointer to struct tm variable to receive broken-down time
 */
pub unsafe fn time64_to_tm(totalsecs: time64_t, offset: i32, result: *mut tm) {
    let mut u32tmp: u32;
    let mut day_of_century: u32;
    let mut year_of_century: u32;
    let mut day_of_year: u32;
    let mut month: u32;
    let mut day: u32;
    let mut u64tmp: u64;
    let mut udays: u64;
    let mut century: u64;
    let mut year: u64;
    let mut is_jan_or_feb: bool;
    let is_leap_year: bool;
    let mut days: i64;
    let mut rem: i64;
    let mut remainder: i32 = 0;

    days = div_s64_rem(totalsecs, SECS_PER_DAY, &mut remainder);
    rem = remainder as i64;
    rem += offset as i64;
    while rem < 0 {
        rem += SECS_PER_DAY;
        days -= 1;
    }
    while rem >= SECS_PER_DAY {
        rem -= SECS_PER_DAY;
        days += 1;
    }

    (*result).tm_hour = (rem / SECS_PER_HOUR) as _;
    rem %= SECS_PER_HOUR;
    (*result).tm_min = (rem / 60) as _;
    (*result).tm_sec = (rem % 60) as _;

    /* January 1, 1970 was a Thursday. */
    (*result).tm_wday = ((4 + days) % 7) as _;
    if (*result).tm_wday < 0 {
        (*result).tm_wday += 7;
    }

    /* The following algorithm is Proposition 6.3 of Neri and Schneider. */
    udays = (days as u64).wrapping_add(2305843009213814918u64);

    u64tmp = 4 * udays + 3;
    century = div64_u64_rem(u64tmp, 146097, &mut u64tmp);
    day_of_century = (u64tmp / 4) as u32;

    u32tmp = 4 * day_of_century + 3;
    u64tmp = 2939745u64 * u32tmp as u64;
    year_of_century = (u64tmp >> 32) as u32;
    day_of_year = (u64tmp as u32) / 2939745 / 4;

    year = 100 * century + year_of_century as u64;
    is_leap_year = if year_of_century != 0 {
        year_of_century % 4 == 0
    } else {
        century % 4 == 0
    };

    u32tmp = 2141 * day_of_year + 132377;
    month = u32tmp >> 16;
    day = (u32tmp as u16 as u32) / 2141;

    /* Recall that January 1st is the 306-th day of the computational calendar. */
    is_jan_or_feb = day_of_year >= 306;

    /* Convert to the Gregorian calendar and adjust to Unix time. */
    year = year + is_jan_or_feb as u64 - 6313183731940000u64;
    month = if is_jan_or_feb { month - 12 } else { month };
    day += 1;
    day_of_year += if is_jan_or_feb {
        (-306i32) as u32
    } else {
        31 + 28 + is_leap_year as u32
    };

    /* Convert to tm's format. */
    (*result).tm_year = (year - 1900) as _;
    (*result).tm_mon = month as _;
    (*result).tm_mday = day as _;
    (*result).tm_yday = day_of_year as _;
}

// EXPORT_SYMBOL(time64_to_tm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
