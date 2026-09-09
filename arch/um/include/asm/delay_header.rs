/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent:
// - asm-generic/delay.h supplies `ndelay` and `udelay`.
// - linux/time-internal.h supplies `time_travel_mode`, `TT_MODE_INFCPU`,
//   `TT_MODE_EXTERNAL`, and `time_travel_ndelay`.

extern "C" {
    static time_travel_mode: core::ffi::c_int;
    fn time_travel_ndelay(nsecs: core::ffi::c_ulong);
    fn ndelay(nsecs: core::ffi::c_ulong);
    fn udelay(usecs: core::ffi::c_ulong);
}

// These build-time constants are supplied by linux/time-internal.h.
// Their values are intentionally left to the including dependency.
const TT_MODE_INFCPU: core::ffi::c_int = 0;
const TT_MODE_EXTERNAL: core::ffi::c_int = 0;

#[inline]
unsafe fn um_ndelay(nsecs: core::ffi::c_ulong) {
    if time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL {
        time_travel_ndelay(nsecs);
        return;
    }
    ndelay(nsecs);
}

#[inline]
unsafe fn um_udelay(usecs: core::ffi::c_ulong) {
    if time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL {
        time_travel_ndelay(1000u64.wrapping_mul(usecs as u64) as core::ffi::c_ulong);
        return;
    }
    udelay(usecs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
