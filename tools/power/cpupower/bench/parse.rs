// SPDX-License-Identifier: GPL-2.0-or-later
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

/* Translated from dependencies: errno.h, stdio.h, stdlib.h, string.h,
 * time.h, dirent.h, sys/utsname.h, sys/types.h, sys/stat.h, parse.h,
 * and config.h declarations are expected from the surrounding build.
 */

pub type size_t = usize;
pub type time_t = c_long;
pub type mode_t = c_uint;
pub type FILE = c_void;
pub type DIR = c_void;

const GOVERNOR_LEN: usize = 16;

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sched_prio {
    SCHED_ERR = 0,
    SCHED_HIGH = 1,
    SCHED_DEFAULT = 2,
    SCHED_LOW = 3,
}

#[repr(C)]
pub struct config {
    pub sleep: c_long,
    pub load: c_long,
    pub sleep_step: c_long,
    pub load_step: c_long,
    pub cycles: c_uint,
    pub rounds: c_uint,
    pub cpu: c_uint,
    pub prio: sched_prio,
    pub verbose: c_uint,
    pub governor: [c_char; GOVERNOR_LEN],
    pub output: *mut FILE,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn uname(buf: *mut utsname) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn dprintf(format: *const c_char, ...) -> c_int;
}

/**
 * converts priority string to priority
 *
 * @param str string that represents a scheduler priority
 *
 * @retval priority
 * @retval SCHED_ERR when the priority doesn't exit
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_to_prio(str_: *const c_char) -> sched_prio {
    if unsafe { strncasecmp(c"high".as_ptr(), str_, strlen(str_)) } == 0 {
        sched_prio::SCHED_HIGH
    } else if unsafe { strncasecmp(c"default".as_ptr(), str_, strlen(str_)) } == 0 {
        sched_prio::SCHED_DEFAULT
    } else if unsafe { strncasecmp(c"low".as_ptr(), str_, strlen(str_)) } == 0 {
        sched_prio::SCHED_LOW
    } else {
        sched_prio::SCHED_ERR
    }
}

/**
 * create and open logfile
 *
 * @param dir directory in which the logfile should be created
 *
 * @retval logfile on success
 * @retval NULL when the file can't be created
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_output(dirname: *const c_char) -> *mut FILE {
    let mut output: *mut FILE = core::ptr::null_mut();
    let mut len: c_int;
    let mut filename: *mut c_char;
    let mut filename_tmp: *mut c_char;
    let mut sysdata: utsname = unsafe { core::mem::zeroed() };
    let dir: *mut DIR;

    dir = unsafe { opendir(dirname) };
    if dir.is_null() {
        if unsafe { mkdir(dirname, 0o755) } != 0 {
            unsafe {
                perror(c"mkdir".as_ptr());
                fprintf(
                    stderr,
                    c"error: Cannot create dir %s\n".as_ptr(),
                    dirname,
                );
            }
            return core::ptr::null_mut();
        }
    }

    len = (unsafe { strlen(dirname) } + 30) as c_int;
    filename = unsafe { malloc(core::mem::size_of::<c_char>() * len as usize) as *mut c_char };
    if filename.is_null() {
        unsafe {
            perror(c"malloc".as_ptr());
        }
        unsafe {
            closedir(dir);
        }
        return output;
    }

    if unsafe { uname(&mut sysdata) } == 0 {
        len += (unsafe { strlen(sysdata.nodename.as_ptr()) + strlen(sysdata.release.as_ptr()) }) as c_int;
        filename_tmp = unsafe {
            realloc(
                filename as *mut c_void,
                core::mem::size_of_val(&*filename) * len as usize,
            ) as *mut c_char
        };

        if filename_tmp.is_null() {
            unsafe {
                free(filename as *mut c_void);
                perror(c"realloc".as_ptr());
                closedir(dir);
            }
            return output;
        }

        filename = filename_tmp;
        unsafe {
            snprintf(
                filename,
                (len - 1) as size_t,
                c"%s/benchmark_%s_%s_%li.log".as_ptr(),
                dirname,
                sysdata.nodename.as_ptr(),
                sysdata.release.as_ptr(),
                time(core::ptr::null_mut::<time_t>()),
            );
        }
    } else {
        unsafe {
            snprintf(
                filename,
                (len - 1) as size_t,
                c"%s/benchmark_%li.log".as_ptr(),
                dirname,
                time(core::ptr::null_mut::<time_t>()),
            );
        }
    }

    unsafe {
        dprintf(c"logfilename: %s\n".as_ptr(), filename);
    }

    output = unsafe { fopen(filename, c"w+".as_ptr()) };
    if output.is_null() {
        unsafe {
            perror(c"fopen".as_ptr());
            fprintf(stderr, c"error: unable to open logfile\n".as_ptr());
        }
    } else {
        unsafe {
            fprintf(stdout, c"Logfile: %s\n".as_ptr(), filename);
            fprintf(
                output,
                c"#round load sleep performance powersave percentage\n".as_ptr(),
            );
        }
    }

    unsafe {
        free(filename as *mut c_void);
        closedir(dir);
    }
    output
}

/**
 * returns the default config
 *
 * @retval default config on success
 * @retval NULL when the output file can't be created
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_default_config() -> *mut config {
    let config: *mut config =
        unsafe { malloc(core::mem::size_of::<config>()) as *mut config };
    if config.is_null() {
        unsafe {
            perror(c"malloc".as_ptr());
        }
        return core::ptr::null_mut();
    }

    unsafe {
        dprintf(c"loading defaults\n".as_ptr());

        (*config).sleep = 500000;
        (*config).load = 500000;
        (*config).sleep_step = 500000;
        (*config).load_step = 500000;
        (*config).cycles = 5;
        (*config).rounds = 50;
        (*config).cpu = 0;
        (*config).prio = sched_prio::SCHED_HIGH;
        (*config).verbose = 0;
        strncpy(
            (*config).governor.as_mut_ptr(),
            c"ondemand".as_ptr(),
            core::mem::size_of_val(&(*config).governor),
        );

        (*config).output = stdout;
    }

    /* #ifdef DEFAULT_CONFIG_FILE
     * if (prepare_config(DEFAULT_CONFIG_FILE, config))
     *     return NULL;
     * #endif
     */
    config
}

/**
 * parses config file and returns the config to the caller
 *
 * @param path config file name
 *
 * @retval 1 on error
 * @retval 0 on success
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_config(path: *const c_char, config: *mut config) -> c_int {
    let mut len: size_t = 0;
    let mut opt: [c_char; 16] = [0; 16];
    let mut val: [c_char; 32] = [0; 32];
    let mut line: *mut c_char = core::ptr::null_mut();
    let configfile: *mut FILE;

    if config.is_null() {
        unsafe {
            fprintf(stderr, c"error: config is NULL\n".as_ptr());
        }
        return 1;
    }

    configfile = unsafe { fopen(path, c"r".as_ptr()) };
    if configfile.is_null() {
        unsafe {
            fprintf(
                stderr,
                c"error: unable to read configfile: %s, %s\n".as_ptr(),
                path,
                strerror(errno),
            );
            free(config as *mut c_void);
        }
        return 1;
    }

    while unsafe { getline(&mut line, &mut len, configfile) } != -1 {
        if unsafe { *line.add(0) } == b'#' as c_char
            || unsafe { *line.add(0) } == b' ' as c_char
            || unsafe { *line.add(0) } == b'\n' as c_char
        {
            continue;
        }

        if unsafe { sscanf(line, c"%14s = %30s".as_ptr(), opt.as_mut_ptr(), val.as_mut_ptr()) } < 2
        {
            continue;
        }

        unsafe {
            dprintf(c"parsing: %s -> %s\n".as_ptr(), opt.as_ptr(), val.as_ptr());
        }

        if unsafe { strcmp(c"sleep".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%li".as_ptr(), &mut (*config).sleep);
            }
        } else if unsafe { strcmp(c"load".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%li".as_ptr(), &mut (*config).load);
            }
        } else if unsafe { strcmp(c"load_step".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%li".as_ptr(), &mut (*config).load_step);
            }
        } else if unsafe { strcmp(c"sleep_step".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%li".as_ptr(), &mut (*config).sleep_step);
            }
        } else if unsafe { strcmp(c"cycles".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%u".as_ptr(), &mut (*config).cycles);
            }
        } else if unsafe { strcmp(c"rounds".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%u".as_ptr(), &mut (*config).rounds);
            }
        } else if unsafe { strcmp(c"verbose".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%u".as_ptr(), &mut (*config).verbose);
            }
        } else if unsafe { strcmp(c"output".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                (*config).output = prepare_output(val.as_ptr());
            }
        } else if unsafe { strcmp(c"cpu".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                sscanf(val.as_ptr(), c"%u".as_ptr(), &mut (*config).cpu);
            }
        } else if unsafe { strcmp(c"governor".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                strncpy(
                    (*config).governor.as_mut_ptr(),
                    val.as_ptr(),
                    core::mem::size_of_val(&(*config).governor),
                );
                (*config).governor[core::mem::size_of_val(&(*config).governor) - 1] = b'\0' as c_char;
            }
        } else if unsafe { strcmp(c"priority".as_ptr(), opt.as_ptr()) } == 0 {
            unsafe {
                if string_to_prio(val.as_ptr()) != sched_prio::SCHED_ERR {
                    (*config).prio = string_to_prio(val.as_ptr());
                }
            }
        }
    }

    unsafe {
        free(line as *mut c_void);
    }

    0
}
