// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated as external Rust declarations:
// unistd.h, stdio.h, errno.h, stdlib.h, limits.h, string.h, ctype.h, getopt.h
// cpufreq.h, cpuidle.h, helpers/helpers.h

use std::os::raw::{c_char, c_int, c_ulonglong, c_void};
use std::ptr;

const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut cpus_chosen: *mut c_void;

    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> isize;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn get_cpustate();
    fn bitmask_isallclear(mask: *const c_void) -> c_int;
    fn bitmask_setall(mask: *mut c_void) -> c_int;
    fn bitmask_first(mask: *const c_void) -> u32;
    fn bitmask_last(mask: *const c_void) -> u32;
    fn bitmask_isbitset(mask: *const c_void, i: u32) -> c_int;
    fn cpupower_is_cpu_online(cpu: u32) -> c_int;
    fn cpuidle_state_count(cpu: u32) -> c_int;
    fn cpuidle_state_disable(cpu: u32, idlestate: u32, disable: c_int) -> c_int;
    fn cpuidle_is_state_disabled(cpu: u32, idlestate: u32) -> c_int;
    fn cpuidle_state_latency(cpu: u32, idlestate: u32) -> c_ulonglong;
    fn print_offline_cpus();
}

static mut INFO_OPTS: [option; 5] = [
    option {
        name: c"disable".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'd' as c_int,
    },
    option {
        name: c"enable".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'e' as c_int,
    },
    option {
        name: c"disable-by-latency".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'D' as c_int,
    },
    option {
        name: c"enable-all".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: ptr::null_mut(),
        val: 'E' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_idle_set(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut cont: c_int = 1;
    let mut param: c_int = 0;
    let mut disabled: c_int;
    let mut latency: c_ulonglong = 0;
    let mut state_latency: c_ulonglong;
    let mut cpu: u32 = 0;
    let mut idlestate: u32 = 0;
    let mut idlestates: c_int = 0;
    let mut endptr: *mut c_char = ptr::null_mut();

    loop {
        ret = getopt_long(
            argc,
            argv,
            c"d:e:ED:".as_ptr(),
            INFO_OPTS.as_ptr(),
            ptr::null_mut(),
        );
        if ret == -1 {
            break;
        }
        match ret {
            x if x == '?' as c_int => {
                param = '?' as c_int;
                cont = 0;
            }
            x if x == 'd' as c_int || x == 'e' as c_int => {
                if param != 0 {
                    param = -1;
                    cont = 0;
                } else {
                    param = ret;
                    strtol(optarg, &mut endptr, 10);
                    if *endptr != 0 {
                        printf(c"Bad value: %s, Integer expected\n".as_ptr(), optarg);
                        exit(EXIT_FAILURE);
                    } else {
                        idlestate = atoi(optarg) as u32;
                    }
                }
            }
            x if x == 'D' as c_int => {
                if param != 0 {
                    param = -1;
                    cont = 0;
                } else {
                    param = ret;
                    latency = strtoull(optarg, &mut endptr, 10);
                    if *endptr != 0 {
                        printf(c"Bad latency value: %s\n".as_ptr(), optarg);
                        exit(EXIT_FAILURE);
                    }
                }
            }
            x if x == 'E' as c_int => {
                if param != 0 {
                    param = -1;
                    cont = 0;
                } else {
                    param = ret;
                }
            }
            -1 => {
                cont = 0;
            }
            _ => {}
        }
        if cont == 0 {
            break;
        }
    }

    match param {
        -1 => {
            printf(c"You can't specify more than one output-specific argument\n".as_ptr());
            exit(EXIT_FAILURE);
        }
        x if x == '?' as c_int => {
            printf(c"invalid or unknown argument\n".as_ptr());
            exit(EXIT_FAILURE);
        }
        _ => {}
    }

    get_cpustate();

    /* Default is: set all CPUs */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setall(cpus_chosen);
    }

    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        if cpupower_is_cpu_online(cpu) != 1 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        idlestates = cpuidle_state_count(cpu);
        if idlestates <= 0 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        match param {
            x if x == 'd' as c_int => {
                ret = cpuidle_state_disable(cpu, idlestate, 1);
                if ret == 0 {
                    printf(c"Idlestate %u disabled on CPU %u\n".as_ptr(), idlestate, cpu);
                } else if ret == -1 {
                    printf(c"Idlestate %u not available on CPU %u\n".as_ptr(), idlestate, cpu);
                } else if ret == -2 {
                    printf(c"Idlestate disabling not supported by kernel\n".as_ptr());
                } else {
                    printf(c"Idlestate %u not disabled on CPU %u\n".as_ptr(), idlestate, cpu);
                }
            }
            x if x == 'e' as c_int => {
                ret = cpuidle_state_disable(cpu, idlestate, 0);
                if ret == 0 {
                    printf(c"Idlestate %u enabled on CPU %u\n".as_ptr(), idlestate, cpu);
                } else if ret == -1 {
                    printf(c"Idlestate %u not available on CPU %u\n".as_ptr(), idlestate, cpu);
                } else if ret == -2 {
                    printf(c"Idlestate enabling not supported by kernel\n".as_ptr());
                } else {
                    printf(c"Idlestate %u not enabled on CPU %u\n".as_ptr(), idlestate, cpu);
                }
            }
            x if x == 'D' as c_int => {
                idlestate = 0;
                while idlestate < idlestates as u32 {
                    disabled = cpuidle_is_state_disabled(cpu, idlestate);
                    state_latency = cpuidle_state_latency(cpu, idlestate);
                    if disabled == 1 {
                        if latency > state_latency {
                            ret = cpuidle_state_disable(cpu, idlestate, 0);
                            if ret == 0 {
                                printf(c"Idlestate %u enabled on CPU %u\n".as_ptr(), idlestate, cpu);
                            }
                        }
                        idlestate = idlestate.wrapping_add(1);
                        continue;
                    }
                    if latency <= state_latency {
                        ret = cpuidle_state_disable(cpu, idlestate, 1);
                        if ret == 0 {
                            printf(c"Idlestate %u disabled on CPU %u\n".as_ptr(), idlestate, cpu);
                        }
                    }
                    idlestate = idlestate.wrapping_add(1);
                }
            }
            x if x == 'E' as c_int => {
                idlestate = 0;
                while idlestate < idlestates as u32 {
                    disabled = cpuidle_is_state_disabled(cpu, idlestate);
                    if disabled == 1 {
                        ret = cpuidle_state_disable(cpu, idlestate, 0);
                        if ret == 0 {
                            printf(c"Idlestate %u enabled on CPU %u\n".as_ptr(), idlestate, cpu);
                        }
                    }
                    idlestate = idlestate.wrapping_add(1);
                }
            }
            _ => {
                /* Not reachable with proper args checking */
                printf(c"Invalid or unknown argument\n".as_ptr());
                exit(EXIT_FAILURE);
            }
        }

        cpu = cpu.wrapping_add(1);
    }

    print_offline_cpus();
    EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
