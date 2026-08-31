// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// <subcmd/parse-options.h>, <stdio.h>, <time.h>, <strings.h>,
// <linux/time64.h>, "debug.h", "clockid.h", "record.h"

use core::ffi::{c_char, c_int, c_void};

type clockid_t = c_int;

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct record_opts {
    pub use_clockid: bool,
    pub clockid: clockid_t,
    pub clockid_res_ns: u64,
}

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct clockid_map {
    name: *const c_char,
    clockid: c_int,
}

unsafe impl Sync for clockid_map {}

const NSEC_PER_SEC: u64 = 1_000_000_000;

/*
 * Add the missing ones, we need to build on many distros...
 *
 * In C these fallback definitions are guarded by #ifndef. The Rust translation
 * preserves the fallback values used by this file.
 */
const CLOCK_REALTIME: clockid_t = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_MONOTONIC_RAW: clockid_t = 4;
const CLOCK_BOOTTIME: clockid_t = 7;
const CLOCK_TAI: clockid_t = 11;

macro_rules! CLOCKID_MAP {
    ($n:expr, $c:expr) => {
        clockid_map {
            name: $n.as_ptr() as *const c_char,
            clockid: $c,
        }
    };
}

macro_rules! CLOCKID_END {
    () => {
        clockid_map {
            name: core::ptr::null(),
            clockid: 0,
        }
    };
}

static clockids: [clockid_map; 10] = [
    /* available for all events, NMI safe */
    CLOCKID_MAP!(b"monotonic\0", CLOCK_MONOTONIC),
    CLOCKID_MAP!(b"monotonic_raw\0", CLOCK_MONOTONIC_RAW),

    /* available for some events */
    CLOCKID_MAP!(b"realtime\0", CLOCK_REALTIME),
    CLOCKID_MAP!(b"boottime\0", CLOCK_BOOTTIME),
    CLOCKID_MAP!(b"tai\0", CLOCK_TAI),

    /* available for the lazy */
    CLOCKID_MAP!(b"mono\0", CLOCK_MONOTONIC),
    CLOCKID_MAP!(b"raw\0", CLOCK_MONOTONIC_RAW),
    CLOCKID_MAP!(b"real\0", CLOCK_REALTIME),
    CLOCKID_MAP!(b"boot\0", CLOCK_BOOTTIME),

    CLOCKID_END!(),
];

extern "C" {
    fn clock_getres(clk_id: clockid_t, res: *mut timespec) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn pr_warning(format: *const c_char, ...);
    fn ui__warning(format: *const c_char, ...);
}

unsafe fn get_clockid_res(clk_id: clockid_t, res_ns: *mut u64) -> c_int {
    let mut res = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    *res_ns = 0;
    if clock_getres(clk_id, &mut res) == 0 {
        *res_ns = (res.tv_nsec as u64).wrapping_add((res.tv_sec as u64).wrapping_mul(NSEC_PER_SEC));
    } else {
        pr_warning(
            b"WARNING: Failed to determine specified clock resolution.\n\0".as_ptr()
                as *const c_char,
        );
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_clockid(
    opt: *const option,
    mut str_: *const c_char,
    unset: c_int,
) -> c_int {
    let opts = (*opt).value as *mut record_opts;
    let mut cm: *const clockid_map;
    let ostr = str_;

    if unset != 0 {
        (*opts).use_clockid = false;
        return 0;
    }

    /* no arg passed */
    if str_.is_null() {
        return 0;
    }

    /* no setting it twice */
    if (*opts).use_clockid {
        return -1;
    }

    (*opts).use_clockid = true;

    /* if its a number, we're done */
    if sscanf(
        str_,
        b"%d\0".as_ptr() as *const c_char,
        &mut (*opts).clockid as *mut clockid_t,
    ) == 1
    {
        return get_clockid_res((*opts).clockid, &mut (*opts).clockid_res_ns);
    }

    /* allow a "CLOCK_" prefix to the name */
    if strncasecmp(str_, b"CLOCK_\0".as_ptr() as *const c_char, 6) == 0 {
        str_ = str_.add(6);
    }

    cm = clockids.as_ptr();
    while !(*cm).name.is_null() {
        if strcasecmp(str_, (*cm).name) == 0 {
            (*opts).clockid = (*cm).clockid;
            return get_clockid_res((*opts).clockid, &mut (*opts).clockid_res_ns);
        }
        cm = cm.add(1);
    }

    (*opts).use_clockid = false;
    ui__warning(
        b"unknown clockid %s, check man page\n\0".as_ptr() as *const c_char,
        ostr,
    );
    -1
}

#[no_mangle]
pub unsafe extern "C" fn clockid_name(clk_id: clockid_t) -> *const c_char {
    let mut cm: *const clockid_map;

    cm = clockids.as_ptr();
    while !(*cm).name.is_null() {
        if (*cm).clockid == clk_id {
            return (*cm).name;
        }
        cm = cm.add(1);
    }
    b"(not found)\0".as_ptr() as *const c_char
}
