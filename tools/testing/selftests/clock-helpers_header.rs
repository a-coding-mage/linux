/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <sys/types.h>, <time.h>.

pub const MSEC_PER_SEC: i64 = 1000;
pub const USEC_PER_MSEC: i64 = 1000;
pub const NSEC_PER_USEC: i64 = 1000;
pub const NSEC_PER_MSEC: i64 = 1000000;
pub const USEC_PER_SEC: i64 = 1000000;
pub const NSEC_PER_SEC: i64 = 1000000000;
pub const PSEC_PER_SEC: i64 = 1000000000000;
pub const FSEC_PER_SEC: i64 = 1000000000000000;

// Defined by this header only when not already provided by the build.
pub const CLOCK_AUX: libc::clockid_t = 16;

// Defined by this header only when not already provided by the build.
pub const MAX_AUX_CLOCKS: libc::clockid_t = 8;

// Defined by this header only when not already provided by the build.
pub const CLOCK_AUX_LAST: libc::clockid_t = CLOCK_AUX + MAX_AUX_CLOCKS - 1;

#[allow(dead_code)]
pub fn clock_name(clockid: libc::clockid_t) -> *const std::os::raw::c_char {
    match clockid {
        libc::CLOCK_REALTIME => b"CLOCK_REALTIME\0".as_ptr() as *const std::os::raw::c_char,
        libc::CLOCK_MONOTONIC => b"CLOCK_MONOTONIC\0".as_ptr() as *const std::os::raw::c_char,
        libc::CLOCK_PROCESS_CPUTIME_ID => {
            b"CLOCK_PROCESS_CPUTIME_ID\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_THREAD_CPUTIME_ID => {
            b"CLOCK_THREAD_CPUTIME_ID\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_MONOTONIC_RAW => {
            b"CLOCK_MONOTONIC_RAW\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_REALTIME_COARSE => {
            b"CLOCK_REALTIME_COARSE\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_MONOTONIC_COARSE => {
            b"CLOCK_MONOTONIC_COARSE\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_BOOTTIME => b"CLOCK_BOOTTIME\0".as_ptr() as *const std::os::raw::c_char,
        libc::CLOCK_REALTIME_ALARM => {
            b"CLOCK_REALTIME_ALARM\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_BOOTTIME_ALARM => {
            b"CLOCK_BOOTTIME_ALARM\0".as_ptr() as *const std::os::raw::c_char
        }
        libc::CLOCK_TAI => b"CLOCK_TAI\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 0 => b"CLOCK_AUX0\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 1 => b"CLOCK_AUX1\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 2 => b"CLOCK_AUX2\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 3 => b"CLOCK_AUX3\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 4 => b"CLOCK_AUX4\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 5 => b"CLOCK_AUX5\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 6 => b"CLOCK_AUX6\0".as_ptr() as *const std::os::raw::c_char,
        x if x == CLOCK_AUX + 7 => b"CLOCK_AUX7\0".as_ptr() as *const std::os::raw::c_char,
        _ => b"UNKNOWN_CLOCKID\0".as_ptr() as *const std::os::raw::c_char,
    }
}
