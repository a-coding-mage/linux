// SPDX-License-Identifier: LGPL-2.0+
/* Copyright (C) 1993, 1994, 1995, 1996, 1997 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Paul Eggert (eggert@twinsun.com). */

/*
 * dgb 10/02/98: ripped this from glibc source to help convert timestamps
 *               to unix time
 *     10/04/98: added new table-based lookup after seeing how ugly
 *               the gnu code is
 * blf 09/27/99: ripped out all the old code and inserted new table from
 *              John Brockmeyer (without leap second corrections)
 *              rewrote udf_stamp_to_time and fixed timezone accounting in
 *              udf_time_to_stamp.
 */

/*
 * We don't take into account leap seconds. This may be correct or incorrect.
 * For more NIST information (especially dealing with leap seconds), see:
 * http://www.boulder.nist.gov/timefreq/pubs/bulletin/leapsecond.htm
 */

// Types, globals, byte-order helpers, and time helpers are supplied by the
// translated declarations corresponding to udfdecl.h and the Linux headers.
use crate::{cpu_to_le16, le16_to_cpu, mktime64, sys_tz, time64_to_tm, timestamp,
            timespec64, tm, time64_t};

pub unsafe fn udf_disk_stamp_to_time(dest: *mut timespec64, src: timestamp) {
    let type_and_timezone: u16 = le16_to_cpu(src.typeAndTimezone);
    let year: u16 = le16_to_cpu(src.year);
    let type_: u8 = (type_and_timezone >> 12) as u8;
    let mut offset: i16;

    if type_ == 1 {
        offset = (type_and_timezone << 4) as i16;
        // sign extent offset
        offset >>= 4;
        if offset == -2047 {
            // unspecified offset
            offset = 0;
        }
    } else {
        offset = 0;
    }

    (*dest).tv_sec = mktime64(
        year as i64,
        src.month,
        src.day,
        src.hour,
        src.minute,
        src.second,
    );
    (*dest).tv_sec -= (offset as i64) * 60;

    /*
     * Sanitize nanosecond field since reportedly some filesystems are
     * recorded with bogus sub-second values.
     */
    if src.centiseconds < 100
        && src.hundredsOfMicroseconds < 100
        && src.microseconds < 100
    {
        (*dest).tv_nsec = 1000
            * (src.centiseconds * 10000
                + src.hundredsOfMicroseconds * 100
                + src.microseconds);
    } else {
        (*dest).tv_nsec = 0;
    }
}

pub unsafe fn udf_time_to_disk_stamp(dest: *mut timestamp, ts: timespec64) {
    let seconds: time64_t;
    let offset: i16;
    let mut tm_value: tm = core::mem::zeroed();

    offset = -sys_tz.tz_minuteswest;

    (*dest).typeAndTimezone = cpu_to_le16(0x1000 | ((offset as u16) & 0x0fff));

    seconds = ts.tv_sec + (offset as i64) * 60;
    time64_to_tm(seconds, 0, &mut tm_value);
    (*dest).year = cpu_to_le16((tm_value.tm_year + 1900) as u16);
    (*dest).month = tm_value.tm_mon + 1;
    (*dest).day = tm_value.tm_mday;
    (*dest).hour = tm_value.tm_hour;
    (*dest).minute = tm_value.tm_min;
    (*dest).second = tm_value.tm_sec;
    (*dest).centiseconds = ts.tv_nsec / 10000000;
    (*dest).hundredsOfMicroseconds = (ts.tv_nsec / 1000
        - (*dest).centiseconds * 10000)
        / 100;
    (*dest).microseconds = ts.tv_nsec / 1000
        - (*dest).centiseconds * 10000
        - (*dest).hundredsOfMicroseconds * 100;
}

/* EOF */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
