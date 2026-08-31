// SPDX-License-Identifier: GPL-2.0-or-later
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint};

/* Dependencies from config.h, system.h, benchmark.h, getopt.h, stdio.h,
 * stdlib.h, and unistd.h are declared here and supplied by other files.
 */

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct config {
    pub output: *mut FILE,
    pub sleep: c_long,
    pub load: c_long,
    pub sleep_step: c_long,
    pub load_step: c_long,
    pub cpu: c_uint,
    pub cycles: c_uint,
    pub rounds: c_uint,
    pub governor: [c_char; 15],
    pub prio: c_int,
    pub verbose: c_int,
}

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stdout: *mut FILE;

    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;

    fn prepare_default_config() -> *mut config;
    fn prepare_output(output: *const c_char) -> *mut FILE;
    fn string_to_prio(prio: *const c_char) -> c_int;
    fn prepare_config(configfile: *const c_char, config: *mut config) -> c_int;
    fn prepare_user(config: *mut config);
    fn prepare_system(config: *mut config);
    fn start_benchmark(config: *mut config);

    fn dprintf(format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static SCHED_ERR: c_int;
}

static mut long_options: [option; 14] = [
    option {
        name: b"output\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'o' as c_int,
    },
    option {
        name: b"sleep\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b's' as c_int,
    },
    option {
        name: b"load\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'l' as c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'v' as c_int,
    },
    option {
        name: b"cpu\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'c' as c_int,
    },
    option {
        name: b"governor\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'g' as c_int,
    },
    option {
        name: b"prio\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'p' as c_int,
    },
    option {
        name: b"file\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'f' as c_int,
    },
    option {
        name: b"cycles\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'n' as c_int,
    },
    option {
        name: b"rounds\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'r' as c_int,
    },
    option {
        name: b"load-step\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'x' as c_int,
    },
    option {
        name: b"sleep-step\0".as_ptr() as *const c_char,
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'y' as c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: core::ptr::null(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: 0,
    },
];

/*******************************************************************
 usage
*******************************************************************/

#[no_mangle]
pub unsafe extern "C" fn usage() {
    unsafe {
        printf(c"usage: ./bench\n".as_ptr());
        printf(c"Options:\n".as_ptr());
        printf(c" -l, --load=<long int>\t\tinitial load time in us\n".as_ptr());
        printf(c" -s, --sleep=<long int>\t\tinitial sleep time in us\n".as_ptr());
        printf(c" -x, --load-step=<long int>\ttime to be added to load time, in us\n".as_ptr());
        printf(c" -y, --sleep-step=<long int>\ttime to be added to sleep time, in us\n".as_ptr());
        printf(c" -c, --cpu=<cpu #>\t\t\tCPU Nr. to use, starting at 0\n".as_ptr());
        printf(c" -p, --prio=<priority>\t\t\tscheduler priority, HIGH, LOW or DEFAULT\n".as_ptr());
        printf(c" -g, --governor=<governor>\t\tcpufreq governor to test\n".as_ptr());
        printf(c" -n, --cycles=<int>\t\t\tload/sleep cycles\n".as_ptr());
        printf(c" -r, --rounds<int>\t\t\tload/sleep rounds\n".as_ptr());
        printf(c" -f, --file=<configfile>\t\tconfig file to use\n".as_ptr());
        printf(c" -o, --output=<dir>\t\t\toutput path. Filename will be OUTPUTPATH/benchmark_TIMESTAMP.log\n".as_ptr());
        printf(c" -v, --verbose\t\t\t\tverbose output on/off\n".as_ptr());
        printf(c" -h, --help\t\t\t\tPrint this help screen\n".as_ptr());
        exit(1);
    }
}

/*******************************************************************
 main
*******************************************************************/

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut c: c_int;
        let mut option_index: c_int = 0;
        let mut config: *mut config = core::ptr::null_mut();

        config = prepare_default_config();

        if config.is_null() {
            return EXIT_FAILURE;
        }

        loop {
            c = getopt_long(
                argc,
                argv,
                c"hg:o:s:l:vc:p:f:n:r:x:y:".as_ptr(),
                core::ptr::addr_of!(long_options) as *const option,
                &mut option_index,
            );
            if c == -1 {
                break;
            }

            match c {
                x if x == b'o' as c_int => {
                    if !(*config).output.is_null() {
                        fclose((*config).output);
                    }

                    (*config).output = prepare_output(optarg);

                    if (*config).output.is_null() {
                        return EXIT_FAILURE;
                    }

                    dprintf(c"user output path -> %s\n".as_ptr(), optarg);
                }
                x if x == b's' as c_int => {
                    sscanf(optarg, c"%li".as_ptr(), &mut (*config).sleep);
                    dprintf(c"user sleep time -> %s\n".as_ptr(), optarg);
                }
                x if x == b'l' as c_int => {
                    sscanf(optarg, c"%li".as_ptr(), &mut (*config).load);
                    dprintf(c"user load time -> %s\n".as_ptr(), optarg);
                }
                x if x == b'c' as c_int => {
                    sscanf(optarg, c"%u".as_ptr(), &mut (*config).cpu);
                    dprintf(c"user cpu -> %s\n".as_ptr(), optarg);
                }
                x if x == b'g' as c_int => {
                    strncpy((*config).governor.as_mut_ptr(), optarg, 14);
                    dprintf(c"user governor -> %s\n".as_ptr(), optarg);
                }
                x if x == b'p' as c_int => {
                    if string_to_prio(optarg) != SCHED_ERR {
                        (*config).prio = string_to_prio(optarg);
                        dprintf(c"user prio -> %s\n".as_ptr(), optarg);
                    } else {
                        if !config.is_null() {
                            if !(*config).output.is_null() {
                                fclose((*config).output);
                            }
                            free(config as *mut c_void);
                        }
                        usage();
                    }
                }
                x if x == b'n' as c_int => {
                    sscanf(optarg, c"%u".as_ptr(), &mut (*config).cycles);
                    dprintf(c"user cycles -> %s\n".as_ptr(), optarg);
                }
                x if x == b'r' as c_int => {
                    sscanf(optarg, c"%u".as_ptr(), &mut (*config).rounds);
                    dprintf(c"user rounds -> %s\n".as_ptr(), optarg);
                }
                x if x == b'x' as c_int => {
                    sscanf(optarg, c"%li".as_ptr(), &mut (*config).load_step);
                    dprintf(c"user load_step -> %s\n".as_ptr(), optarg);
                }
                x if x == b'y' as c_int => {
                    sscanf(optarg, c"%li".as_ptr(), &mut (*config).sleep_step);
                    dprintf(c"user sleep_step -> %s\n".as_ptr(), optarg);
                }
                x if x == b'f' as c_int => {
                    if prepare_config(optarg, config) != 0 {
                        return EXIT_FAILURE;
                    }
                }
                x if x == b'v' as c_int => {
                    (*config).verbose = 1;
                    dprintf(c"verbose output enabled\n".as_ptr());
                }
                x if x == b'h' as c_int || x == b'?' as c_int => {
                    if !config.is_null() {
                        if !(*config).output.is_null() {
                            fclose((*config).output);
                        }
                        free(config as *mut c_void);
                    }
                    usage();
                }
                _ => {
                    if !config.is_null() {
                        if !(*config).output.is_null() {
                            fclose((*config).output);
                        }
                        free(config as *mut c_void);
                    }
                    usage();
                }
            }
        }

        if (*config).verbose != 0 {
            printf(c"starting benchmark with parameters:\n".as_ptr());
            printf(
                c"config:\n\tsleep=%li\n\tload=%li\n\tsleep_step=%li\n\tload_step=%li\n\tcpu=%u\n\tcycles=%u\n\trounds=%u\n\tgovernor=%s\n\n".as_ptr(),
                (*config).sleep,
                (*config).load,
                (*config).sleep_step,
                (*config).load_step,
                (*config).cpu,
                (*config).cycles,
                (*config).rounds,
                (*config).governor.as_ptr(),
            );
        }

        prepare_user(config);
        prepare_system(config);
        start_benchmark(config);

        if (*config).output != stdout {
            fclose((*config).output);
        }

        free(config as *mut c_void);

        EXIT_SUCCESS
    }
}
