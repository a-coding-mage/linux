// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2011 Thomas Renninger <trenn@suse.de>, Novell Inc.
 */

use libc::{
    c_char, c_int, c_uint, fprintf, printf, setlocale, sscanf, stderr, textdomain, uname, utsname,
    EXIT_FAILURE, LC_ALL,
};

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
// #include "helpers/bitmask.h"

const REQUIRED_ARGUMENT: c_int = 1;
const EINVAL: c_int = 22;

static PACKAGE: &[u8] = b"cpupower\0";

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct cpupower_cpu_info {
    pub vendor: c_int,
}

#[repr(C)]
pub struct bitmask {
    _private: [u8; 0],
}

extern "C" {
    static mut optarg: *mut c_char;

    static mut cpupower_cpu_info: cpupower_cpu_info;
    static mut cpus_chosen: *mut bitmask;

    static X86_VENDOR_AMD: c_int;
    static X86_VENDOR_INTEL: c_int;

    fn gettext(msgid: *const c_char) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longind: *mut c_int,
    ) -> c_int;

    fn cpupower_set_amd_pstate_mode(mode: *mut c_char) -> c_int;
    fn cpupower_set_intel_turbo_boost(turbo_boost: c_int) -> c_int;
    fn cpupower_set_generic_turbo_boost(turbo_boost: c_int) -> c_int;
    fn cpupower_intel_set_perf_bias(cpu: c_uint, perf_bias: c_int) -> c_int;
    fn cpupower_set_epp(cpu: c_uint, epp: *mut c_char) -> c_int;

    fn bitmask_isallclear(mask: *const bitmask) -> c_int;
    fn bitmask_setall(mask: *mut bitmask);
    fn bitmask_first(mask: *const bitmask) -> c_uint;
    fn bitmask_last(mask: *const bitmask) -> c_uint;
    fn bitmask_isbitset(mask: *const bitmask, bit: c_uint) -> c_int;

    fn sysfs_is_cpu_online(cpu: c_uint) -> c_int;
}

static mut SET_OPTS: [option; 6] = [
    option {
        name: b"perf-bias\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: 0 as *mut c_int,
        val: b'b' as c_int,
    },
    option {
        name: b"epp\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: 0 as *mut c_int,
        val: b'e' as c_int,
    },
    option {
        name: b"amd-pstate-mode\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: 0 as *mut c_int,
        val: b'm' as c_int,
    },
    option {
        name: b"turbo-boost\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: 0 as *mut c_int,
        val: b't' as c_int,
    },
    option {
        name: b"boost\0".as_ptr() as *const c_char,
        has_arg: REQUIRED_ARGUMENT,
        flag: 0 as *mut c_int,
        val: b't' as c_int,
    },
    option {
        name: 0 as *const c_char,
        has_arg: 0,
        flag: 0 as *mut c_int,
        val: 0,
    },
];

#[inline]
unsafe fn tr(msgid: *const c_char) -> *mut c_char {
    gettext(msgid)
}

unsafe fn print_wrong_arg_exit() {
    printf(tr(b"invalid or unknown argument\n\0".as_ptr() as *const c_char));
    exit(EXIT_FAILURE);
}

#[derive(Copy, Clone)]
struct Params {
    perf_bias: c_int,
    epp: c_int,
    mode: c_int,
    turbo_boost: c_int,
    params: c_int,
}

impl Params {
    fn zeroed() -> Self {
        Self {
            perf_bias: 0,
            epp: 0,
            mode: 0,
            turbo_boost: 0,
            params: 0,
        }
    }

    fn any(&self) -> c_int {
        self.perf_bias | self.epp | self.mode | self.turbo_boost | self.params
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmd_set(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut cpu: c_uint;
    let mut uts: utsname = std::mem::zeroed();

    /*
     * C used a union containing 1-bit int fields and an int named params.
     * The source only zeroes params and tests individual flags, so preserve
     * the file-local behavior with explicit integer flags.
     */
    let mut params = Params::zeroed();
    let mut perf_bias: c_int = 0;
    let mut turbo_boost: c_int = 1;
    let mut ret: c_int = 0;
    let mut epp: [c_char; 30] = [0; 30];
    let mut mode: [c_char; 20] = [0; 20];

    ret = uname(&mut uts as *mut utsname);
    if ret == 0
        && (strcmp(
            uts.machine.as_ptr() as *const c_char,
            b"ppc64le\0".as_ptr() as *const c_char,
        ) == 0
            || strcmp(
                uts.machine.as_ptr() as *const c_char,
                b"ppc64\0".as_ptr() as *const c_char,
            ) == 0)
    {
        fprintf(
            stderr,
            tr(b"Subcommand not supported on POWER.\n\0".as_ptr() as *const c_char),
        );
        return ret;
    }

    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);
    textdomain(PACKAGE.as_ptr() as *const c_char);

    params = Params::zeroed();
    /* parameter parsing */
    loop {
        ret = getopt_long(
            argc,
            argv,
            b"b:e:m:t:\0".as_ptr() as *const c_char,
            SET_OPTS.as_mut_ptr() as *const option,
            0 as *mut c_int,
        );
        if ret == -1 {
            break;
        }

        match ret {
            x if x == b'b' as c_int => {
                if params.perf_bias != 0 {
                    print_wrong_arg_exit();
                }
                perf_bias = atoi(optarg);
                if perf_bias < 0 || perf_bias > 15 {
                    printf(
                        tr(b"--perf-bias param out of range [0-%d]\n\0".as_ptr()
                            as *const c_char),
                        15,
                    );
                    print_wrong_arg_exit();
                }
                params.perf_bias = 1;
            }
            x if x == b'e' as c_int => {
                if params.epp != 0 {
                    print_wrong_arg_exit();
                }
                if sscanf(
                    optarg,
                    b"%29s\0".as_ptr() as *const c_char,
                    epp.as_mut_ptr(),
                ) != 1
                {
                    print_wrong_arg_exit();
                    return -EINVAL;
                }
                params.epp = 1;
            }
            x if x == b'm' as c_int => {
                if cpupower_cpu_info.vendor != X86_VENDOR_AMD {
                    print_wrong_arg_exit();
                }
                if params.mode != 0 {
                    print_wrong_arg_exit();
                }
                if sscanf(
                    optarg,
                    b"%19s\0".as_ptr() as *const c_char,
                    mode.as_mut_ptr(),
                ) != 1
                {
                    print_wrong_arg_exit();
                    return -EINVAL;
                }
                params.mode = 1;
            }
            x if x == b't' as c_int => {
                if params.turbo_boost != 0 {
                    print_wrong_arg_exit();
                }
                turbo_boost = atoi(optarg);
                if turbo_boost < 0 || turbo_boost > 1 {
                    printf(b"--turbo-boost param out of range [0-1]\n\0".as_ptr()
                        as *const c_char);
                    print_wrong_arg_exit();
                }
                params.turbo_boost = 1;
            }
            _ => {
                print_wrong_arg_exit();
            }
        }
    }

    if params.any() == 0 {
        print_wrong_arg_exit();
    }

    if params.mode != 0 {
        ret = cpupower_set_amd_pstate_mode(mode.as_mut_ptr());
        if ret != 0 {
            fprintf(stderr, b"Error setting mode\n\0".as_ptr() as *const c_char);
        }
    }

    if params.turbo_boost != 0 {
        if cpupower_cpu_info.vendor == X86_VENDOR_INTEL {
            ret = cpupower_set_intel_turbo_boost(turbo_boost);
        } else {
            ret = cpupower_set_generic_turbo_boost(turbo_boost);
        }

        if ret != 0 {
            fprintf(
                stderr,
                b"Error setting turbo-boost\n\0".as_ptr() as *const c_char,
            );
        }
    }

    /* Default is: set all CPUs */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setall(cpus_chosen);
    }

    /* loop over CPUs */
    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        if sysfs_is_cpu_online(cpu) != 1 {
            fprintf(
                stderr,
                tr(b"Cannot set values on CPU %d:\0".as_ptr() as *const c_char),
                cpu,
            );
            fprintf(stderr, tr(b" *is offline\n\0".as_ptr() as *const c_char));
            cpu = cpu.wrapping_add(1);
            continue;
        }

        if params.perf_bias != 0 {
            ret = cpupower_intel_set_perf_bias(cpu, perf_bias);
            if ret != 0 {
                fprintf(
                    stderr,
                    tr(b"Error setting perf-bias value on CPU %d\n\0".as_ptr()
                        as *const c_char),
                    cpu,
                );
                break;
            }
        }

        if params.epp != 0 {
            ret = cpupower_set_epp(cpu, epp.as_mut_ptr());
            if ret != 0 {
                fprintf(
                    stderr,
                    b"Error setting epp value on CPU %d\n\0".as_ptr() as *const c_char,
                    cpu,
                );
                break;
            }
        }

        cpu = cpu.wrapping_add(1);
    }
    ret
}
