// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
// C dependencies: <stdio.h>, <sys/time.h>, <linux/sysinfo.h>, "thermal-tools.h"

use core::ptr;

#[repr(C)]
pub struct timeval {
    pub tv_sec: libc::time_t,
    pub tv_usec: libc::suseconds_t,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::time_t,
    pub tv_nsec: libc::c_long,
}

#[repr(C)]
pub struct sysinfo {
    pub uptime: libc::c_long,
    pub loads: [libc::c_ulong; 3],
    pub totalram: libc::c_ulong,
    pub freeram: libc::c_ulong,
    pub sharedram: libc::c_ulong,
    pub bufferram: libc::c_ulong,
    pub totalswap: libc::c_ulong,
    pub freeswap: libc::c_ulong,
    pub procs: libc::c_ushort,
    pub pad: libc::c_ushort,
    pub totalhigh: libc::c_ulong,
    pub freehigh: libc::c_ulong,
    pub mem_unit: libc::c_uint,
    pub _f: [libc::c_char; 0],
}

unsafe extern "C" {
    fn sysinfo(info: *mut sysinfo) -> libc::c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut libc::c_void) -> libc::c_int;
}

static mut __offset: libc::c_ulong = 0;
static mut __tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uptimeofday_init() -> libc::c_int {
    let mut info: sysinfo = unsafe { core::mem::zeroed() };

    if unsafe { sysinfo(&mut info) } != 0 {
        return -1;
    }

    unsafe {
        gettimeofday(ptr::addr_of_mut!(__tv), ptr::null_mut());

        __offset = ((*ptr::addr_of!(__tv)).tv_sec - info.uptime) as libc::c_ulong;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getuptimeofday_ms() -> libc::c_ulong {
    unsafe {
        gettimeofday(ptr::addr_of_mut!(__tv), ptr::null_mut());

        (((*ptr::addr_of!(__tv)).tv_sec as libc::c_ulong - __offset) * 1000)
            + ((*ptr::addr_of!(__tv)).tv_usec as libc::c_ulong / 1000)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn msec_to_timespec(msec: libc::c_int) -> timespec {
    let tv: timespec = timespec {
        tv_sec: (msec / 1000) as libc::time_t,
        tv_nsec: ((msec % 1000) * 1000000) as libc::c_long,
    };

    tv
}
