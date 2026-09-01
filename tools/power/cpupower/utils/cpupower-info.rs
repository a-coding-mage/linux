// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2011 Thomas Renninger <trenn@suse.de>, Novell Inc.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

// C dependencies:
// #include <unistd.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <errno.h>
// #include <string.h>
// #include <getopt.h>
// #include <sys/utsname.h>
// #include "helpers/helpers.h"
// #include "helpers/sysfs.h"

const OPTIONAL_ARGUMENT: c_int = 2;
const EXIT_FAILURE: c_int = 1;
const LC_ALL: c_int = 6;

const CPUPOWER_CAP_PERF_BIAS: c_uint = 0x0000_0008;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe impl Sync for option {}

#[repr(C)]
struct utsname {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

#[repr(C)]
struct cpupower_cpu_info_t {
    caps: c_uint,
}

#[derive(Copy, Clone)]
struct params_t {
    perf_bias: c_int,
    params: c_int,
}

static SET_OPTS: [option; 2] = [
    option {
        name: b"perf-bias\0".as_ptr() as *const c_char,
        has_arg: OPTIONAL_ARGUMENT,
        flag: ptr::null_mut(),
        val: b'b' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn textdomain(domainname: *const c_char) -> *mut c_char;
    fn gettext(msgid: *const c_char) -> *mut c_char;

    static mut stderr: *mut c_void;

    static mut cpus_chosen: *mut c_void;
    static mut base_cpu: c_uint;
    static mut run_as_root: c_int;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    fn bitmask_isallclear(mask: *mut c_void) -> c_int;
    fn bitmask_setbit(mask: *mut c_void, bit: c_uint) -> c_int;
    fn bitmask_first(mask: *mut c_void) -> c_uint;
    fn bitmask_last(mask: *mut c_void) -> c_uint;
    fn bitmask_isbitset(mask: *mut c_void, bit: c_uint) -> c_int;
    fn sysfs_is_cpu_online(cpu: c_uint) -> c_int;
    fn cpupower_intel_get_perf_bias(cpu: c_uint) -> c_int;
}

unsafe fn _(msgid: *const c_char) -> *mut c_char {
    gettext(msgid)
}

unsafe fn print_wrong_arg_exit() {
    printf(_(b"invalid or unknown argument\n\0".as_ptr() as *const c_char));
    exit(EXIT_FAILURE);
}

#[no_mangle]
pub unsafe extern "C" fn cmd_info(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut cpu: c_uint;
    let mut uts: utsname = std::mem::zeroed();

    let mut params = params_t {
        perf_bias: 0,
        params: 0,
    };
    let mut ret: c_int = 0;

    ret = uname(&mut uts);
    if ret == 0
        && (strcmp(
            uts.machine.as_ptr(),
            b"ppc64le\0".as_ptr() as *const c_char,
        ) == 0
            || strcmp(
                uts.machine.as_ptr(),
                b"ppc64\0".as_ptr() as *const c_char,
            ) == 0)
    {
        fprintf(
            stderr,
            _(b"Subcommand not supported on POWER.\n\0".as_ptr() as *const c_char),
        );
        return ret;
    }

    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);
    textdomain(PACKAGE.as_ptr() as *const c_char);

    /* parameter parsing */
    loop {
        ret = getopt_long(
            argc,
            argv,
            b"b\0".as_ptr() as *const c_char,
            SET_OPTS.as_ptr(),
            ptr::null_mut(),
        );
        if ret == -1 {
            break;
        }

        match ret {
            x if x == b'b' as c_int => {
                if params.perf_bias != 0 {
                    print_wrong_arg_exit();
                }
                params.perf_bias = 1;
                params.params |= 1;
            }
            _ => {
                print_wrong_arg_exit();
            }
        }
    }

    if params.params == 0 {
        params.params = 0x7;
        params.perf_bias = params.params & 1;
    }

    /* Default is: show output of base_cpu only */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setbit(cpus_chosen, base_cpu);
    }

    /* Add more per cpu options here */
    if params.perf_bias == 0 {
        return ret;
    }

    if params.perf_bias != 0 {
        if run_as_root == 0 {
            params.perf_bias = 0;
            params.params &= !1;
            printf(
                _(b"Intel's performance bias setting needs root privileges\n\0".as_ptr()
                    as *const c_char),
            );
        } else if (cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS) == 0 {
            printf(
                _(b"System does not support Intel's performance bias setting\n\0".as_ptr()
                    as *const c_char),
            );
            params.perf_bias = 0;
            params.params &= !1;
        }
    }

    /* loop over CPUs */
    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        printf(
            _(b"analyzing CPU %d:\n\0".as_ptr() as *const c_char),
            cpu,
        );

        if sysfs_is_cpu_online(cpu) != 1 {
            printf(_(b" *is offline\n\0".as_ptr() as *const c_char));
            cpu = cpu.wrapping_add(1);
            continue;
        }

        if params.perf_bias != 0 {
            ret = cpupower_intel_get_perf_bias(cpu);
            if ret < 0 {
                fprintf(
                    stderr,
                    _(b"Could not read perf-bias value[%d]\n\0".as_ptr() as *const c_char),
                    ret,
                );
                exit(EXIT_FAILURE);
            } else {
                printf(_(b"perf-bias: %d\n\0".as_ptr() as *const c_char), ret);
            }
        }

        cpu = cpu.wrapping_add(1);
    }
    0
}

extern "C" {
    static PACKAGE: [c_char; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
