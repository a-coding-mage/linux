// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 *  (C) 2010       Thomas Renninger <trenn@suse.de>
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_ulong, c_ulonglong, c_void};

const LINE_LEN: usize = 10;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const NO_ARGUMENT: c_int = 0;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn cpuidle_state_count(cpu: u32) -> u32;
    fn cpuidle_state_name(cpu: u32, idlestate: u32) -> *mut c_char;
    fn cpuidle_state_desc(cpu: u32, idlestate: u32) -> *mut c_char;
    fn cpuidle_state_latency(cpu: u32, idlestate: u32) -> c_ulong;
    fn cpuidle_state_residency(cpu: u32, idlestate: u32) -> c_ulong;
    fn cpuidle_state_usage(cpu: u32, idlestate: u32) -> c_ulong;
    fn cpuidle_state_time(cpu: u32, idlestate: u32) -> c_ulonglong;
    fn cpuidle_is_state_disabled(cpu: u32, idlestate: u32) -> c_int;
    fn cpuidle_get_driver() -> *mut c_char;
    fn cpuidle_get_governor() -> *mut c_char;

    fn sysfs_is_cpu_online(cpu: u32) -> c_int;

    fn bitmask_isallclear(mask: *mut c_void) -> c_int;
    fn bitmask_setbit(mask: *mut c_void, bit: u32) -> c_int;
    fn bitmask_first(mask: *mut c_void) -> u32;
    fn bitmask_last(mask: *mut c_void) -> u32;
    fn bitmask_isbitset(mask: *mut c_void, bit: u32) -> c_int;

    static mut cpus_chosen: *mut c_void;
    static mut base_cpu: u32;
}

unsafe fn _(s: &'static CStr) -> *const c_char {
    s.as_ptr()
}

unsafe fn cpuidle_cpu_output(cpu: u32, verbose: c_int) {
    let mut idlestate: u32;
    let mut tmp: *mut c_char;

    let idlestates = cpuidle_state_count(cpu);
    if idlestates == 0 {
        printf(_(c"CPU %u: No idle states\n"), cpu);
        return;
    }

    printf(_(c"Number of idle states: %d\n"), idlestates);
    printf(_(c"Available idle states:"));
    idlestate = 0;
    while idlestate < idlestates {
        tmp = cpuidle_state_name(cpu, idlestate);
        if tmp.is_null() {
            idlestate += 1;
            continue;
        }
        printf(c" %s".as_ptr(), tmp);
        free(tmp as *mut c_void);
        idlestate += 1;
    }
    printf(c"\n".as_ptr());

    if verbose == 0 {
        return;
    }

    idlestate = 0;
    while idlestate < idlestates {
        let mut disabled = cpuidle_is_state_disabled(cpu, idlestate);
        /* Disabled interface not supported on older kernels */
        if disabled < 0 {
            disabled = 0;
        }
        tmp = cpuidle_state_name(cpu, idlestate);
        if tmp.is_null() {
            idlestate += 1;
            continue;
        }
        printf(
            c"%s%s:\n".as_ptr(),
            tmp,
            if disabled != 0 {
                c" (DISABLED) ".as_ptr()
            } else {
                c"".as_ptr()
            },
        );
        free(tmp as *mut c_void);

        tmp = cpuidle_state_desc(cpu, idlestate);
        if tmp.is_null() {
            idlestate += 1;
            continue;
        }
        printf(_(c"Flags/Description: %s\n"), tmp);
        free(tmp as *mut c_void);

        printf(
            _(c"Latency: %lu\n"),
            cpuidle_state_latency(cpu, idlestate),
        );
        printf(
            _(c"Residency: %lu\n"),
            cpuidle_state_residency(cpu, idlestate),
        );
        printf(_(c"Usage: %lu\n"), cpuidle_state_usage(cpu, idlestate));
        printf(_(c"Duration: %llu\n"), cpuidle_state_time(cpu, idlestate));
        idlestate += 1;
    }
}

unsafe fn cpuidle_general_output() {
    let mut tmp: *mut c_char;

    tmp = cpuidle_get_driver();
    if tmp.is_null() {
        printf(_(c"Could not determine cpuidle driver\n"));
        return;
    }

    printf(_(c"CPUidle driver: %s\n"), tmp);
    free(tmp as *mut c_void);

    tmp = cpuidle_get_governor();
    if tmp.is_null() {
        printf(_(c"Could not determine cpuidle governor\n"));
        return;
    }

    printf(_(c"CPUidle governor: %s\n"), tmp);
    free(tmp as *mut c_void);
}

unsafe fn proc_cpuidle_cpu_output(cpu: u32) {
    let max_allowed_cstate: c_long = 2000000000;
    let mut cstate: u32;

    let cstates = cpuidle_state_count(cpu);
    if cstates == 0 {
        printf(_(c"CPU %u: No C-states info\n"), cpu);
        return;
    }

    printf(_(c"active state:            C0\n"));
    printf(_(c"max_cstate:              C%u\n"), cstates - 1);
    printf(
        _(c"maximum allowed latency: %lu usec\n"),
        max_allowed_cstate as c_ulong,
    );
    printf(_(c"states:\t\n"));
    cstate = 0;
    while cstate < cstates {
        printf(
            _(c"    C%d:                  type[C%d] "),
            cstate,
            cstate,
        );
        printf(_(c"promotion[--] demotion[--] "));
        printf(
            _(c"latency[%03lu] "),
            cpuidle_state_latency(cpu, cstate),
        );
        printf(
            _(c"residency[%05lu] "),
            cpuidle_state_residency(cpu, cstate),
        );
        printf(_(c"usage[%08lu] "), cpuidle_state_usage(cpu, cstate));
        printf(
            _(c"duration[%020Lu] \n"),
            cpuidle_state_time(cpu, cstate),
        );
        cstate += 1;
    }
}

use std::os::raw::c_long;

static mut info_opts: [option; 3] = [
    option {
        name: c"silent".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: 's' as c_int,
    },
    option {
        name: c"proc".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: 'o' as c_int,
    },
    option {
        name: std::ptr::null(),
        has_arg: 0,
        flag: std::ptr::null_mut(),
        val: 0,
    },
];

#[inline]
unsafe fn cpuidle_exit(_fail: c_int) -> ! {
    exit(EXIT_FAILURE);
}

#[no_mangle]
pub unsafe extern "C" fn cmd_idle_info(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut cont: c_int = 1;
    let mut output_param: c_int = 0;
    let mut verbose: c_int = 1;
    let mut cpu: u32 = 0;

    loop {
        ret = getopt_long(
            argc,
            argv,
            c"os".as_ptr(),
            info_opts.as_ptr(),
            std::ptr::null_mut(),
        );
        if ret == -1 {
            break;
        }
        match ret {
            x if x == '?' as c_int => {
                output_param = '?' as c_int;
                cont = 0;
            }
            x if x == 's' as c_int => {
                verbose = 0;
            }
            -1 => {
                cont = 0;
            }
            x if x == 'o' as c_int => {
                if output_param != 0 {
                    output_param = -1;
                    cont = 0;
                    break;
                }
                output_param = ret;
            }
            _ => {}
        }
        if cont == 0 {
            break;
        }
    }

    match output_param {
        -1 => {
            printf(_(c"You can't specify more than one output-specific argument\n"));
            cpuidle_exit(EXIT_FAILURE);
        }
        x if x == '?' as c_int => {
            printf(_(c"invalid or unknown argument\n"));
            cpuidle_exit(EXIT_FAILURE);
        }
        _ => {}
    }

    /* Default is: show output of base_cpu only */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setbit(cpus_chosen, base_cpu);
    }

    if output_param == 0 {
        cpuidle_general_output();
    }

    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        printf(_(c"analyzing CPU %d:\n"), cpu);

        if sysfs_is_cpu_online(cpu) != 1 {
            printf(_(c" *is offline\n"));
            printf(c"\n".as_ptr());
            cpu = cpu.wrapping_add(1);
            continue;
        }

        match output_param {
            x if x == 'o' as c_int => {
                proc_cpuidle_cpu_output(cpu);
            }
            0 => {
                printf(c"\n".as_ptr());
                cpuidle_cpu_output(cpu, verbose);
            }
            _ => {}
        }
        printf(c"\n".as_ptr());
        cpu = cpu.wrapping_add(1);
    }
    EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
