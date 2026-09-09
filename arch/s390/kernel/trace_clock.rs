// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux trace clock and timex interfaces.

/*
 * trace_clock_s390_tod(): trace clock based on the s390 TOD clock
 *
 * Unlike the other clocks, this is not in nanoseconds.
 */

unsafe extern "C" {
    fn get_tod_clock() -> u64;
}

// `notrace` is a kernel build attribute; preserve the C ABI and exported name.
#[no_mangle]
pub unsafe extern "C" fn trace_clock_s390_tod() -> u64 {
    unsafe { get_tod_clock() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
