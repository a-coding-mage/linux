/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NTFS time conversion functions.
 *
 * Copyright (c) 2001-2005 Anton Altaparmakov
 */

// Dependencies corresponding to <linux/time.h> and <asm/div64.h> are
// supplied by the surrounding translation unit.  The latter provides
// div_s64_rem().

pub const NTFS_TIME_OFFSET: i64 = (369i64 * 365 + 89) * 24 * 3600;

/*
 * utc2ntfs - convert Linux UTC time to NTFS time
 * @ts: Linux UTC time to convert to NTFS time
 *
 * Convert the Linux UTC time @ts to its corresponding NTFS time and return
 * that in little endian format.
 *
 * Linux stores time in a struct timespec64 consisting of a time64_t tv_sec
 * and a long tv_nsec where tv_sec is the number of 1-second intervals since
 * 1st January 1970, 00:00:00 UTC and tv_nsec is the number of 1-nano-second
 * intervals since the value of tv_sec.
 *
 * NTFS uses Microsoft's standard time format which is stored in a s64 and is
 * measured as the number of 100-nano-second intervals since 1st January 1601,
 * 00:00:00 UTC.
 */
#[inline]
pub fn utc2ntfs(ts: timespec64) -> __le64 {
    /*
     * Convert the seconds to 100ns intervals, add the nano-seconds
     * converted to 100ns intervals, and then add the NTFS time offset.
     */
    cpu_to_le64(((ts.tv_sec + NTFS_TIME_OFFSET) as u64) * 10000000
        + (ts.tv_nsec / 100) as u64)
}

/*
 * get_current_ntfs_time - get the current time in little endian NTFS format
 *
 * Get the current time from the Linux kernel, convert it to its corresponding
 * NTFS time and return that in little endian format.
 */
#[inline]
pub fn get_current_ntfs_time() -> __le64 {
    let mut ts: timespec64 = unsafe { core::mem::zeroed() };

    unsafe {
        ktime_get_coarse_real_ts64(&mut ts);
    }
    utc2ntfs(ts)
}

/*
 * ntfs2utc - convert NTFS time to Linux time
 * @time: NTFS time (little endian) to convert to Linux UTC
 *
 * Convert the little endian NTFS time @time to its corresponding Linux UTC
 * time and return that in cpu format.
 *
 * Linux stores time in a struct timespec64 consisting of a time64_t tv_sec
 * and a long tv_nsec where tv_sec is the number of 1-second intervals since
 * 1st January 1970, 00:00:00 UTC and tv_nsec is the number of 1-nano-second
 * intervals since the value of tv_sec.
 *
 * NTFS uses Microsoft's standard time format which is stored in a s64 and is
 * measured as the number of 100 nano-second intervals since 1st January 1601,
 * 00:00:00 UTC.
 */
#[inline]
pub fn ntfs2utc(time: __le64) -> timespec64 {
    let mut ts: timespec64 = unsafe { core::mem::zeroed() };
    let mut t32: i32 = 0;

    /* Subtract the NTFS time offset. */
    let t: i64 = le64_to_cpu(time) as i64 - NTFS_TIME_OFFSET * 10000000;
    /*
     * Convert the time to 1-second intervals and the remainder to
     * 1-nano-second intervals.
     */
    ts.tv_sec = div_s64_rem(t, 10000000, &mut t32);
    ts.tv_nsec = t32 * 100;
    ts
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
