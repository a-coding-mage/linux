// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2016 SUSE Software Solutions GmbH
 *           Thomas Renninger <trenn@suse.de>
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::ptr;

// C dependencies: <unistd.h>, <stdio.h>, <errno.h>, <stdlib.h>, <stdint.h>,
// <string.h>, <getopt.h>, "powercap.h", and "helpers/helpers.h".

const EXIT_FAILURE: c_int = 1;
const NO_ARGUMENT: c_int = 0;

// Provided by powercap.h.
const POWERCAP_MAX_TREE_DEPTH: c_int = 8;
const MAX_LINE_LEN: usize = 4096;

#[repr(C)]
pub struct powercap_zone {
    pub name: *const c_char,
    pub tree_depth: c_int,
    pub has_power_uw: c_int,
    pub has_energy_uj: c_int,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe extern "C" {
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    static mut stderr: *mut c_void;

    fn powercap_get_driver(line: *mut c_char, max_len: c_int) -> c_int;
    fn powercap_get_enabled(val: *mut c_int) -> c_int;
    fn powercap_init_zones() -> *mut powercap_zone;
    fn powercap_walk_zones(
        root_zone: *mut powercap_zone,
        callback: Option<unsafe extern "C" fn(*mut powercap_zone) -> c_int>,
    );
    fn powercap_zone_get_enabled(zone: *mut powercap_zone, mode: *mut c_int) -> c_int;

    // helpers/helpers.h provides the gettext-style _(...) macro in C.
    fn gettext(msgid: *const c_char) -> *const c_char;
}

#[inline]
unsafe fn tr(msgid: *const c_char) -> *const c_char {
    unsafe { gettext(msgid) }
}

#[unsafe(no_mangle)]
pub static mut powercap_show_all: c_int = 0;

static mut info_opts: [option; 2] = [
    option {
        name: c"all".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'a' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe extern "C" fn powercap_print_one_zone(zone: *mut powercap_zone) -> c_int {
    let mut mode: c_int = 0;
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut pr_prefix = [0 as c_char; 1024];

    i = 0;
    while i < unsafe { (*zone).tree_depth } && i < POWERCAP_MAX_TREE_DEPTH {
        unsafe {
            strcat(pr_prefix.as_mut_ptr(), c"\t".as_ptr());
        }
        i += 1;
    }

    unsafe {
        printf(c"%sZone: %s".as_ptr(), pr_prefix.as_ptr(), (*zone).name);
    }
    ret = unsafe { powercap_zone_get_enabled(zone, &mut mode) };
    if ret < 0 {
        return ret;
    }
    unsafe {
        printf(
            c" (%s)\n".as_ptr(),
            if mode != 0 {
                c"enabled".as_ptr()
            } else {
                c"disabled".as_ptr()
            },
        );
    }

    if unsafe { (*zone).has_power_uw } != 0 {
        unsafe {
            printf(
                tr(c"%sPower can be monitored in micro Watts\n".as_ptr()),
                pr_prefix.as_ptr(),
            );
        }
    }

    if unsafe { (*zone).has_energy_uj } != 0 {
        unsafe {
            printf(
                tr(c"%sPower can be monitored in micro Jules\n".as_ptr()),
                pr_prefix.as_ptr(),
            );
        }
    }

    unsafe {
        printf(c"\n".as_ptr());
    }

    ret
}

unsafe fn powercap_show() -> c_int {
    let mut root_zone: *mut powercap_zone;
    let mut line = [0 as c_char; MAX_LINE_LEN];
    let mut ret: c_int;
    let mut val: c_int = 0;

    ret = unsafe { powercap_get_driver(line.as_mut_ptr(), MAX_LINE_LEN as c_int) };
    if ret < 0 {
        unsafe {
            printf(tr(c"No powercapping driver loaded\n".as_ptr()));
        }
        return ret;
    }

    unsafe {
        printf(c"Driver: %s\n".as_ptr(), line.as_ptr());
    }
    ret = unsafe { powercap_get_enabled(&mut val) };
    if ret < 0 {
        return ret;
    }
    if val == 0 {
        unsafe {
            printf(tr(c"Powercapping is disabled\n".as_ptr()));
        }
        return -1;
    }

    unsafe {
        printf(tr(c"Powercap domain hierarchy:\n\n".as_ptr()));
    }
    root_zone = unsafe { powercap_init_zones() };

    if root_zone.is_null() {
        unsafe {
            printf(tr(c"No powercap info found\n".as_ptr()));
        }
        return 1;
    }

    unsafe {
        powercap_walk_zones(root_zone, Some(powercap_print_one_zone));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_cap_set(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_cap_info(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut cont: c_int = 1;

    while cont != 0 {
        ret = unsafe {
            getopt_long(
                argc,
                argv,
                c"a".as_ptr(),
                info_opts.as_ptr(),
                ptr::null_mut(),
            )
        };
        match ret {
            x if x == '?' as c_int => {
                cont = 0;
            }
            -1 => {
                cont = 0;
            }
            x if x == 'a' as c_int => unsafe {
                powercap_show_all = 1;
            },
            _ => {
                unsafe {
                    fprintf(stderr, tr(c"invalid or unknown argument\n".as_ptr()));
                }
                return EXIT_FAILURE;
            }
        }
    }

    unsafe {
        powercap_show();
    }
    0
}
