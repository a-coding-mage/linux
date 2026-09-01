// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const MAX_MSG_LENGTH: usize = 1024;
pub static mut config_debug: bool = false;

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type time_t = c_long;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const DT_DIR: u8 = 4;
const SCHED_OTHER: c_int = 0;
const SCHED_FIFO: c_int = 1;
const SCHED_RR: c_int = 2;
const SCHED_DEADLINE: c_int = 6;
const INVALID_VAL: c_long = !0;

#[cfg(target_arch = "x86_64")]
const __NR_sched_setattr: c_long = 314;
#[cfg(target_arch = "x86")]
const __NR_sched_setattr: c_long = 351;
#[cfg(target_arch = "arm")]
const __NR_sched_setattr: c_long = 380;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
const __NR_sched_setattr: c_long = 274;
#[cfg(target_arch = "powerpc")]
const __NR_sched_setattr: c_long = 355;
#[cfg(target_arch = "s390x")]
const __NR_sched_setattr: c_long = 345;
#[cfg(target_arch = "loongarch64")]
const __NR_sched_setattr: c_long = 274;

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_attr {
    pub size: c_uint,
    pub sched_policy: c_uint,
    pub sched_flags: u64,
    pub sched_nice: c_int,
    pub sched_priority: c_uint,
    pub sched_runtime: u64,
    pub sched_deadline: u64,
    pub sched_period: u64,
}

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut nr_cpus: c_int;

    static STACK_FORMAT_TRUNCATE: c_int;
    static STACK_FORMAT_SKIP: c_int;
    static STACK_FORMAT_FULL: c_int;
    static ERROR: c_int;
    static MAX_PATH: c_int;
    static MIN_NICE: c_long;
    static MAX_NICE: c_long;

    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: core::ffi::VaList) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: core::ffi::VaList) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn time(tloc: *mut time_t) -> time_t;
    fn difftime(time1: time_t, time0: time_t) -> f64;
    fn gmtime(timer: *const time_t) -> *mut tm;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getpid() -> pid_t;
    fn sched_get_priority_min(policy: c_int) -> c_int;
    fn sched_get_priority_max(policy: c_int) -> c_int;
    fn sched_getaffinity(pid: pid_t, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;

    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn CPU_AND(destset: *mut cpu_set_t, srcset1: *const cpu_set_t, srcset2: *const cpu_set_t);
    fn CPU_XOR(destset: *mut cpu_set_t, srcset1: *const cpu_set_t, srcset2: *const cpu_set_t);
    fn CPU_COUNT(set: *const cpu_set_t) -> c_int;
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
}

#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
unsafe extern "C" {
    fn cpuidle_state_count(cpu: c_uint) -> c_uint;
    fn cpuidle_is_state_disabled(cpu: c_uint, state: c_uint) -> c_int;
    fn cpuidle_state_disable(cpu: c_uint, state: c_uint, disabled: c_int) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe fn isdigit(ch: c_char) -> bool {
    ch >= b'0' as c_char && ch <= b'9' as c_char
}

/*
 * err_msg - print an error message to the stderr
 */
pub unsafe extern "C" fn err_msg(fmt: *const c_char, mut args: ...) {
    let mut message = [0 as c_char; MAX_MSG_LENGTH];

    vsnprintf(message.as_mut_ptr(), message.len(), fmt, args.as_va_list());

    fprintf(stderr, c"%s".as_ptr(), message.as_ptr());
}

/*
 * debug_msg - print a debug message to stderr if debug is set
 */
pub unsafe extern "C" fn debug_msg(fmt: *const c_char, mut args: ...) {
    let mut message = [0 as c_char; MAX_MSG_LENGTH];

    if !config_debug {
        return;
    }

    vsnprintf(message.as_mut_ptr(), message.len(), fmt, args.as_va_list());

    fprintf(stderr, c"%s".as_ptr(), message.as_ptr());
}

/*
 * fatal - print an error message and EOL to stderr and exit with ERROR
 */
pub unsafe extern "C" fn fatal(fmt: *const c_char, mut args: ...) -> ! {
    vfprintf(stderr, fmt, args.as_va_list());
    fprintf(stderr, c"\n".as_ptr());

    exit(ERROR);
}

/*
 * get_llong_from_str - get a long long int from a string
 */
pub unsafe extern "C" fn get_llong_from_str(start: *mut c_char) -> i64 {
    let mut end: *mut c_char = ptr::null_mut();

    errno = 0;
    let value = strtoll(start, &mut end, 10);
    if errno != 0 || start == end {
        return -1;
    }

    value
}

/*
 * get_duration - fill output with a human readable duration since start_time
 */
pub unsafe extern "C" fn get_duration(start_time: time_t, output: *mut c_char, output_size: c_int) {
    let now = time(ptr::null_mut());
    let duration = difftime(now, start_time) as time_t;
    let tm_info = gmtime(&duration);

    snprintf(
        output,
        output_size as size_t,
        c"%3d %02d:%02d:%02d".as_ptr(),
        (*tm_info).tm_yday,
        (*tm_info).tm_hour,
        (*tm_info).tm_min,
        (*tm_info).tm_sec,
    );
}

/*
 * parse_cpu_set - parse a cpu_list filling cpu_set_t argument
 *
 * Receives a cpu list, like 1-3,5 (cpus 1, 2, 3, 5), and then set
 * filling cpu_set_t argument.
 *
 * Returns 0 on success, 1 otherwise.
 */
pub unsafe extern "C" fn parse_cpu_set(cpu_list: *mut c_char, set: *mut cpu_set_t) -> c_int {
    let mut p: *const c_char;
    let mut end_cpu: c_int;
    let mut cpu: c_int;
    let mut i: c_int;

    CPU_ZERO(set);

    p = cpu_list;
    while *p != 0 {
        cpu = atoi(p);
        if cpu < 0 || (cpu == 0 && *p != b'0' as c_char) || cpu >= nr_cpus {
            debug_msg(c"Error parsing the cpu set %s\n".as_ptr(), cpu_list);
            return 1;
        }

        while isdigit(*p) {
            p = p.add(1);
        }
        if *p == b'-' as c_char {
            p = p.add(1);
            end_cpu = atoi(p);
            if end_cpu < cpu || (end_cpu == 0 && *p != b'0' as c_char) || end_cpu >= nr_cpus {
                debug_msg(c"Error parsing the cpu set %s\n".as_ptr(), cpu_list);
                return 1;
            }
            while isdigit(*p) {
                p = p.add(1);
            }
        } else {
            end_cpu = cpu;
        }

        if cpu == end_cpu {
            debug_msg(c"cpu_set: adding cpu %d\n".as_ptr(), cpu);
            CPU_SET(cpu, set);
        } else {
            i = cpu;
            while i <= end_cpu {
                debug_msg(c"cpu_set: adding cpu %d\n".as_ptr(), i);
                CPU_SET(i, set);
                i += 1;
            }
        }

        if *p == b',' as c_char {
            p = p.add(1);
        }
    }

    0
}

/*
 * parse_stack_format - parse the stack format
 *
 * Return: the stack format on success, -1 otherwise.
 */
pub unsafe extern "C" fn parse_stack_format(arg: *mut c_char) -> c_int {
    if strcmp(arg, c"truncate".as_ptr()) == 0 {
        return STACK_FORMAT_TRUNCATE;
    }
    if strcmp(arg, c"skip".as_ptr()) == 0 {
        return STACK_FORMAT_SKIP;
    }
    if strcmp(arg, c"full".as_ptr()) == 0 {
        return STACK_FORMAT_FULL;
    }

    debug_msg(c"Error parsing the stack format %s\n".as_ptr(), arg);
    -1
}

/*
 * parse_duration - parse duration with s/m/h/d suffix converting it to seconds
 */
pub unsafe extern "C" fn parse_seconds_duration(val: *mut c_char) -> c_long {
    let mut end: *mut c_char = ptr::null_mut();
    let mut t = strtol(val, &mut end, 10);

    if !end.is_null() {
        match *end {
            x if x == b's' as c_char || x == b'S' as c_char => {}
            x if x == b'm' as c_char || x == b'M' as c_char => t *= 60,
            x if x == b'h' as c_char || x == b'H' as c_char => t *= 60 * 60,
            x if x == b'd' as c_char || x == b'D' as c_char => t *= 24 * 60 * 60,
            _ => {}
        }
    }

    t
}

/*
 * match_time_unit - check if str starts with unit followed by end-of-string or ':'
 *
 * This allows the time unit parser to work both in standalone duration strings
 * like "100ms" and in colon-delimited SCHED_DEADLINE specifications like
 * "d:10ms:100ms", while still rejecting malformed input like "100msx".
 */
unsafe fn match_time_unit(str_: *const c_char, unit: *const c_char) -> bool {
    let len = strlen(unit);

    strncmp(str_, unit, len) == 0 && (*str_.add(len) == 0 || *str_.add(len) == b':' as c_char)
}

/*
 * parse_ns_duration - parse duration with ns/us/ms/s converting it to nanoseconds
 */
pub unsafe extern "C" fn parse_ns_duration(val: *mut c_char) -> c_long {
    let mut end: *mut c_char = ptr::null_mut();
    let mut t = strtol(val, &mut end, 10);

    if !end.is_null() {
        if match_time_unit(end, c"ns".as_ptr()) {
            return t;
        } else if match_time_unit(end, c"us".as_ptr()) {
            t *= 1000;
            return t;
        } else if match_time_unit(end, c"ms".as_ptr()) {
            t *= 1000 * 1000;
            return t;
        } else if match_time_unit(end, c"s".as_ptr()) {
            t *= 1000 * 1000 * 1000;
            return t;
        }
        return -1;
    }

    t
}

/*
 * This is a set of helper functions to use SCHED_DEADLINE.
 */
#[inline]
unsafe fn syscall_sched_setattr(pid: pid_t, attr: *const sched_attr, flags: c_uint) -> c_int {
    syscall(__NR_sched_setattr, pid, attr, flags) as c_int
}

pub unsafe extern "C" fn __set_sched_attr(pid: c_int, attr: *mut sched_attr) -> c_int {
    let flags: c_int = 0;

    let retval = syscall_sched_setattr(pid, attr, flags as c_uint);
    if retval < 0 {
        err_msg(
            c"Failed to set sched attributes to the pid %d: %s\n".as_ptr(),
            pid,
            strerror(errno),
        );
        return 1;
    }

    0
}

/*
 * procfs_is_workload_pid - check if a procfs entry contains a comm_prefix* comm
 *
 * Check if the procfs entry is a directory of a process, and then check if the
 * process has a comm with the prefix set in char *comm_prefix. As the
 * current users of this function only check for kernel threads, there is no
 * need to check for the threads for the process.
 *
 * Return: True if the proc_entry contains a comm file with comm_prefix*.
 * Otherwise returns false.
 */
unsafe fn procfs_is_workload_pid(comm_prefix: *const c_char, proc_entry: *mut dirent) -> c_int {
    let max_path = MAX_PATH as usize;
    let mut buffer = vec![0 as c_char; max_path];
    let mut t_name: *mut c_char;

    if (*proc_entry).d_type != DT_DIR {
        return 0;
    }

    if (*proc_entry).d_name[0] == b'.' as c_char {
        return 0;
    }

    /* check if the string is a pid */
    t_name = (*proc_entry).d_name.as_mut_ptr();
    while *t_name != 0 {
        if !isdigit(*t_name) {
            break;
        }
        t_name = t_name.add(1);
    }

    if *t_name != 0 {
        return 0;
    }

    snprintf(
        buffer.as_mut_ptr(),
        max_path,
        c"/proc/%s/comm".as_ptr(),
        (*proc_entry).d_name.as_ptr(),
    );
    let comm_fd = open(buffer.as_ptr(), O_RDONLY);
    if comm_fd < 0 {
        return 0;
    }

    memset(buffer.as_mut_ptr() as *mut c_void, 0, max_path);
    let retval = read(comm_fd, buffer.as_mut_ptr() as *mut c_void, max_path);

    close(comm_fd);

    if retval <= 0 {
        return 0;
    }

    buffer[max_path - 1] = 0;
    if !str_has_prefix(buffer.as_ptr(), comm_prefix) {
        return 0;
    }

    /* comm already have \n */
    debug_msg(
        c"Found workload pid:%s comm:%s".as_ptr(),
        (*proc_entry).d_name.as_ptr(),
        buffer.as_ptr(),
    );

    1
}

/*
 * set_comm_sched_attr - set sched params to threads starting with char *comm_prefix
 *
 * This function uses procfs to list the currently running threads and then set the
 * sched_attr *attr to the threads that start with char *comm_prefix. It is
 * mainly used to set the priority to the kernel threads created by the
 * tracers.
 */
pub unsafe extern "C" fn set_comm_sched_attr(comm_prefix: *const c_char, attr: *mut sched_attr) -> c_int {
    let mut retval: c_int;
    let mut pid: c_int = 0;

    if strlen(comm_prefix) >= MAX_PATH as usize {
        err_msg(
            c"Command prefix is too long: %d < strlen(%s)\n".as_ptr(),
            MAX_PATH,
            comm_prefix,
        );
        return 1;
    }

    let procfs = opendir(c"/proc".as_ptr());
    if procfs.is_null() {
        err_msg(c"Could not open procfs\n".as_ptr());
        return 1;
    }

    loop {
        let proc_entry = readdir(procfs);
        if proc_entry.is_null() {
            break;
        }

        retval = procfs_is_workload_pid(comm_prefix, proc_entry);
        if retval == 0 {
            continue;
        }

        if strtoi((*proc_entry).d_name.as_ptr(), &mut pid) != 0 {
            err_msg(c"'%s' is not a valid pid".as_ptr(), (*proc_entry).d_name.as_ptr());
            retval = 1;
            closedir(procfs);
            return retval;
        }
        /* procfs_is_workload_pid confirmed it is a pid */
        retval = __set_sched_attr(pid, attr);
        if retval != 0 {
            err_msg(c"Error setting sched attributes for pid:%s\n".as_ptr(), (*proc_entry).d_name.as_ptr());
            closedir(procfs);
            return retval;
        }

        debug_msg(c"Set sched attributes for pid:%s\n".as_ptr(), (*proc_entry).d_name.as_ptr());
    }

    retval = 0;
    closedir(procfs);
    retval
}

unsafe fn get_long_ns_after_colon(mut start: *mut c_char) -> c_long {
    let mut val = INVALID_VAL;

    /* find the ":" */
    start = strstr(start, c":".as_ptr());
    if start.is_null() {
        return -1;
    }

    /* skip ":" */
    start = start.add(1);
    val = parse_ns_duration(start);

    val
}

unsafe fn get_long_after_colon(mut start: *mut c_char) -> c_long {
    let mut val = INVALID_VAL;

    /* find the ":" */
    start = strstr(start, c":".as_ptr());
    if start.is_null() {
        return -1;
    }

    /* skip ":" */
    start = start.add(1);
    val = get_llong_from_str(start) as c_long;

    val
}

/*
 * parse priority in the format:
 * SCHED_OTHER:
 *		o:<prio>
 *		O:<prio>
 * SCHED_RR:
 *		r:<prio>
 *		R:<prio>
 * SCHED_FIFO:
 *		f:<prio>
 *		F:<prio>
 * SCHED_DEADLINE:
 *		d:runtime:period
 *		D:runtime:period
 */
pub unsafe extern "C" fn parse_prio(arg: *mut c_char, sched_param: *mut sched_attr) -> c_int {
    let prio: c_long;
    let runtime: c_long;
    let period: c_long;

    memset(
        sched_param as *mut c_void,
        0,
        core::mem::size_of_val(&*sched_param),
    );
    (*sched_param).size = core::mem::size_of_val(&*sched_param) as c_uint;

    match *arg {
        x if x == b'd' as c_char || x == b'D' as c_char => {
            /* d:runtime:period */
            if strlen(arg) < 4 {
                return -1;
            }

            runtime = get_long_ns_after_colon(arg);
            if runtime == INVALID_VAL {
                return -1;
            }

            period = get_long_ns_after_colon(arg.add(2));
            if period == INVALID_VAL {
                return -1;
            }

            if runtime > period {
                return -1;
            }

            (*sched_param).sched_policy = SCHED_DEADLINE as c_uint;
            (*sched_param).sched_runtime = runtime as u64;
            (*sched_param).sched_deadline = period as u64;
            (*sched_param).sched_period = period as u64;
        }
        x if x == b'f' as c_char || x == b'F' as c_char => {
            /* f:prio */
            prio = get_long_after_colon(arg);
            if prio == INVALID_VAL {
                return -1;
            }

            if prio < sched_get_priority_min(SCHED_FIFO) as c_long {
                return -1;
            }
            if prio > sched_get_priority_max(SCHED_FIFO) as c_long {
                return -1;
            }

            (*sched_param).sched_policy = SCHED_FIFO as c_uint;
            (*sched_param).sched_priority = prio as c_uint;
        }
        x if x == b'r' as c_char || x == b'R' as c_char => {
            /* r:prio */
            prio = get_long_after_colon(arg);
            if prio == INVALID_VAL {
                return -1;
            }

            if prio < sched_get_priority_min(SCHED_RR) as c_long {
                return -1;
            }
            if prio > sched_get_priority_max(SCHED_RR) as c_long {
                return -1;
            }

            (*sched_param).sched_policy = SCHED_RR as c_uint;
            (*sched_param).sched_priority = prio as c_uint;
        }
        x if x == b'o' as c_char || x == b'O' as c_char => {
            /* o:prio */
            prio = get_long_after_colon(arg);
            if prio == INVALID_VAL {
                return -1;
            }

            if prio < MIN_NICE {
                return -1;
            }
            if prio > MAX_NICE {
                return -1;
            }

            (*sched_param).sched_policy = SCHED_OTHER as c_uint;
            (*sched_param).sched_nice = prio as c_int;
        }
        _ => return -1,
    }
    0
}

/*
 * set_cpu_dma_latency - set the /dev/cpu_dma_latecy
 *
 * This is used to reduce the exit from idle latency. The value
 * will be reset once the file descriptor of /dev/cpu_dma_latecy
 * is closed.
 *
 * Return: the /dev/cpu_dma_latecy file descriptor
 */
pub unsafe extern "C" fn set_cpu_dma_latency(latency: i32) -> c_int {
    let fd = open(c"/dev/cpu_dma_latency".as_ptr(), O_RDWR);
    if fd < 0 {
        err_msg(c"Error opening /dev/cpu_dma_latency\n".as_ptr());
        return -1;
    }

    let retval = write(fd, &latency as *const i32 as *const c_void, 4);
    if retval < 1 {
        err_msg(c"Error setting /dev/cpu_dma_latency\n".as_ptr());
        close(fd);
        return -1;
    }

    debug_msg(c"Set /dev/cpu_dma_latency to %d\n".as_ptr(), latency);

    fd
}

#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
static mut saved_cpu_idle_disable_state: *mut *mut c_uint = ptr::null_mut();
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
static mut saved_cpu_idle_disable_state_alloc_ctr: size_t = 0;

/*
 * save_cpu_idle_state_disable - save disable for all idle states of a cpu
 *
 * Saves the current disable of all idle states of a cpu, to be subsequently
 * restored via restore_cpu_idle_disable_state.
 *
 * Return: idle state count on success, negative on error
 */
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
pub unsafe extern "C" fn save_cpu_idle_disable_state(cpu: c_uint) -> c_int {
    let nr_states = cpuidle_state_count(cpu);

    if nr_states == 0 {
        return 0;
    }

    if saved_cpu_idle_disable_state.is_null() {
        saved_cpu_idle_disable_state = calloc(nr_cpus as size_t, core::mem::size_of::<*mut c_uint>()) as *mut *mut c_uint;
        if saved_cpu_idle_disable_state.is_null() {
            return -1;
        }
    }

    *saved_cpu_idle_disable_state.add(cpu as usize) =
        calloc(nr_states as size_t, core::mem::size_of::<c_uint>()) as *mut c_uint;
    if (*saved_cpu_idle_disable_state.add(cpu as usize)).is_null() {
        return -1;
    }
    saved_cpu_idle_disable_state_alloc_ctr += 1;

    let mut state: c_uint = 0;
    while state < nr_states {
        let disabled = cpuidle_is_state_disabled(cpu, state);
        if disabled < 0 {
            return disabled;
        }
        *(*saved_cpu_idle_disable_state.add(cpu as usize)).add(state as usize) = disabled as c_uint;
        state += 1;
    }

    nr_states as c_int
}

/*
 * restore_cpu_idle_disable_state - restore disable for all idle states of a cpu
 *
 * Restores the current disable state of all idle states of a cpu that was
 * previously saved by save_cpu_idle_disable_state.
 *
 * Return: idle state count on success, negative on error
 */
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
pub unsafe extern "C" fn restore_cpu_idle_disable_state(cpu: c_uint) -> c_int {
    let nr_states = cpuidle_state_count(cpu);

    if nr_states == 0 {
        return 0;
    }

    if saved_cpu_idle_disable_state.is_null() {
        return -1;
    }

    let mut state: c_uint = 0;
    while state < nr_states {
        if (*saved_cpu_idle_disable_state.add(cpu as usize)).is_null() {
            return -1;
        }
        let disabled = *(*saved_cpu_idle_disable_state.add(cpu as usize)).add(state as usize) as c_int;
        let result = cpuidle_state_disable(cpu, state, disabled);
        if result < 0 {
            return result;
        }
        state += 1;
    }

    free(*saved_cpu_idle_disable_state.add(cpu as usize) as *mut c_void);
    *saved_cpu_idle_disable_state.add(cpu as usize) = ptr::null_mut();
    saved_cpu_idle_disable_state_alloc_ctr -= 1;
    if saved_cpu_idle_disable_state_alloc_ctr == 0 {
        free(saved_cpu_idle_disable_state as *mut c_void);
        saved_cpu_idle_disable_state = ptr::null_mut();
    }

    nr_states as c_int
}

/*
 * free_cpu_idle_disable_states - free saved idle state disable for all cpus
 *
 * Frees the memory used for storing cpu idle state disable for all cpus
 * and states.
 *
 * Normally, the memory is freed automatically in
 * restore_cpu_idle_disable_state; this is mostly for cleaning up after an
 * error.
 */
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
pub unsafe extern "C" fn free_cpu_idle_disable_states() {
    if saved_cpu_idle_disable_state.is_null() {
        return;
    }

    let mut cpu: c_int = 0;
    while cpu < nr_cpus {
        free(*saved_cpu_idle_disable_state.add(cpu as usize) as *mut c_void);
        *saved_cpu_idle_disable_state.add(cpu as usize) = ptr::null_mut();
        cpu += 1;
    }

    free(saved_cpu_idle_disable_state as *mut c_void);
    saved_cpu_idle_disable_state = ptr::null_mut();
}

/*
 * set_deepest_cpu_idle_state - limit idle state of cpu
 *
 * Disables all idle states deeper than the one given in
 * deepest_state (assuming states with higher number are deeper).
 *
 * This is used to reduce the exit from idle latency. Unlike
 * set_cpu_dma_latency, it can disable idle states per cpu.
 *
 * Return: idle state count on success, negative on error
 */
#[cfg(HAVE_LIBCPUPOWER_SUPPORT)]
pub unsafe extern "C" fn set_deepest_cpu_idle_state(cpu: c_uint, deepest_state: c_uint) -> c_int {
    let nr_states = cpuidle_state_count(cpu);

    let mut state = deepest_state + 1;
    while state < nr_states {
        let result = cpuidle_state_disable(cpu, state, 1);
        if result < 0 {
            return result;
        }
        state += 1;
    }

    nr_states as c_int
}

/*
 * find_mount - find a the mount point of a given fs
 *
 * Returns 0 if mount is not found, otherwise return 1 and fill mp
 * with the mount point.
 */
unsafe fn find_mount(fs: *const c_char, mp: *mut c_char, sizeof_mp: c_int) -> c_int {
    let max_path = MAX_PATH as usize;
    let mut mount_point = vec![0 as c_char; max_path + 1];
    let mut type_ = [0 as c_char; 100];
    let mut found = 0;

    let fp = fopen(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return 0;
    }

    while fscanf(
        fp,
        c"%*s %4096s %99s %*s %*d %*d\n".as_ptr(),
        mount_point.as_mut_ptr(),
        type_.as_mut_ptr(),
    ) == 2
    {
        if strcmp(type_.as_ptr(), fs) == 0 {
            found = 1;
            break;
        }
    }
    fclose(fp);

    if found == 0 {
        return 0;
    }

    memset(mp as *mut c_void, 0, sizeof_mp as size_t);
    strncpy(mp, mount_point.as_ptr(), (sizeof_mp - 1) as size_t);

    debug_msg(c"Fs %s found at %s\n".as_ptr(), fs, mp);
    1
}

/*
 * get_self_cgroup - get the current thread cgroup path
 *
 * Parse /proc/$$/cgroup file to get the thread's cgroup. As an example of line to parse:
 *
 * 0::/user.slice/user-0.slice/session-3.scope'\n'
 *
 * This function is interested in the content after the second : and before the '\n'.
 *
 * Returns 1 if a string was found, 0 otherwise.
 */
unsafe fn get_self_cgroup(self_cg: *mut c_char, sizeof_self_cg: c_int) -> c_int {
    let max_path = MAX_PATH as usize;
    let mut path = vec![0 as c_char; max_path];
    let mut start: *mut c_char;

    snprintf(path.as_mut_ptr(), max_path, c"/proc/%d/cgroup".as_ptr(), getpid());

    let fd = open(path.as_ptr(), O_RDONLY);
    if fd < 0 {
        return 0;
    }

    memset(path.as_mut_ptr() as *mut c_void, 0, max_path);
    let retval = read(fd, path.as_mut_ptr() as *mut c_void, max_path);

    close(fd);

    if retval <= 0 {
        return 0;
    }

    path[max_path - 1] = 0;
    start = path.as_mut_ptr();

    start = strstr(start, c":".as_ptr());
    if start.is_null() {
        return 0;
    }

    /* skip ":" */
    start = start.add(1);

    start = strstr(start, c":".as_ptr());
    if start.is_null() {
        return 0;
    }

    /* skip ":" */
    start = start.add(1);

    if strlen(start) >= sizeof_self_cg as usize {
        return 0;
    }

    snprintf(self_cg, sizeof_self_cg as size_t, c"%s".as_ptr(), start);

    /* Swap '\n' with '\0' */
    start = strstr(self_cg, c"\n".as_ptr());

    /* there must be '\n' */
    if start.is_null() {
        return 0;
    }

    /* ok, it found a string after the second : and before the \n */
    *start = 0;

    1
}

/*
 * open_cgroup_procs - Open the cgroup.procs file for the given cgroup
 *
 * If cgroup argument is not NULL, the cgroup.procs file for that cgroup
 * will be opened. Otherwise, the cgroup of the calling, i.e., rtla, thread
 * will be used.
 *
 * Supports cgroup v2.
 *
 * Returns the file descriptor on success, -1 otherwise.
 */
unsafe fn open_cgroup_procs(cgroup: *const c_char) -> c_int {
    let max_path = MAX_PATH as usize;
    let mut cgroup_path = vec![0 as c_char; max_path - strlen(c"/cgroup.procs".as_ptr())];
    let mut cgroup_procs = vec![0 as c_char; max_path];

    let mut retval = find_mount(c"cgroup2".as_ptr(), cgroup_path.as_mut_ptr(), cgroup_path.len() as c_int);
    if retval == 0 {
        err_msg(c"Did not find cgroupv2 mount point\n".as_ptr());
        return -1;
    }

    let cg_path_len = strlen(cgroup_path.as_ptr());

    if cgroup.is_null() {
        retval = get_self_cgroup(
            cgroup_path.as_mut_ptr().add(cg_path_len),
            (cgroup_path.len() - cg_path_len) as c_int,
        );
        if retval == 0 {
            err_msg(c"Did not find self cgroup\n".as_ptr());
            return -1;
        }
    } else {
        snprintf(
            cgroup_path.as_mut_ptr().add(cg_path_len),
            cgroup_path.len() - cg_path_len,
            c"%s/".as_ptr(),
            cgroup,
        );
    }

    snprintf(
        cgroup_procs.as_mut_ptr(),
        max_path,
        c"%s/cgroup.procs".as_ptr(),
        cgroup_path.as_ptr(),
    );

    debug_msg(c"Using cgroup path at: %s\n".as_ptr(), cgroup_procs.as_ptr());

    let cg_fd = open(cgroup_procs.as_ptr(), O_RDWR);
    if cg_fd < 0 {
        return -1;
    }

    cg_fd
}

/*
 * set_pid_cgroup - Set cgroup to pid_t pid
 *
 * If cgroup argument is not NULL, the threads will move to the given cgroup.
 * Otherwise, the cgroup of the calling, i.e., rtla, thread will be used.
 *
 * Supports cgroup v2.
 *
 * Returns 1 on success, 0 otherwise.
 */
pub unsafe extern "C" fn set_pid_cgroup(pid: pid_t, cgroup: *const c_char) -> c_int {
    let mut pid_str = [0 as c_char; 24];

    let cg_fd = open_cgroup_procs(cgroup);
    if cg_fd < 0 {
        return 0;
    }

    snprintf(pid_str.as_mut_ptr(), pid_str.len(), c"%d\n".as_ptr(), pid);

    let retval = write(cg_fd, pid_str.as_ptr() as *const c_void, strlen(pid_str.as_ptr()));
    if retval < 0 {
        err_msg(
            c"Error setting cgroup attributes for pid:%s - %s\n".as_ptr(),
            pid_str.as_ptr(),
            strerror(errno),
        );
    } else {
        debug_msg(c"Set cgroup attributes for pid:%s\n".as_ptr(), pid_str.as_ptr());
    }

    close(cg_fd);

    (retval >= 0) as c_int
}

/**
 * set_comm_cgroup - Set cgroup to threads starting with char *comm_prefix
 *
 * If cgroup argument is not NULL, the threads will move to the given cgroup.
 * Otherwise, the cgroup of the calling, i.e., rtla, thread will be used.
 *
 * Supports cgroup v2.
 *
 * Returns 1 on success, 0 otherwise.
 */
pub unsafe extern "C" fn set_comm_cgroup(comm_prefix: *const c_char, cgroup: *const c_char) -> c_int {
    let mut retval: c_int;

    if strlen(comm_prefix) >= MAX_PATH as usize {
        err_msg(
            c"Command prefix is too long: %d < strlen(%s)\n".as_ptr(),
            MAX_PATH,
            comm_prefix,
        );
        return 0;
    }

    let cg_fd = open_cgroup_procs(cgroup);
    if cg_fd < 0 {
        return 0;
    }

    let procfs = opendir(c"/proc".as_ptr());
    if procfs.is_null() {
        err_msg(c"Could not open procfs\n".as_ptr());
        close(cg_fd);
        return 0;
    }

    loop {
        let proc_entry = readdir(procfs);
        if proc_entry.is_null() {
            break;
        }

        retval = procfs_is_workload_pid(comm_prefix, proc_entry);
        if retval == 0 {
            continue;
        }

        retval = write(
            cg_fd,
            (*proc_entry).d_name.as_ptr() as *const c_void,
            strlen((*proc_entry).d_name.as_ptr()),
        ) as c_int;
        if retval < 0 {
            err_msg(
                c"Error setting cgroup attributes for pid:%s - %s\n".as_ptr(),
                (*proc_entry).d_name.as_ptr(),
                strerror(errno),
            );
            closedir(procfs);
            close(cg_fd);
            return 0;
        }

        debug_msg(c"Set cgroup attributes for pid:%s\n".as_ptr(), (*proc_entry).d_name.as_ptr());
    }

    closedir(procfs);
    close(cg_fd);
    1
}

/**
 * auto_house_keeping - Automatically move rtla out of measurement threads
 *
 * Try to move rtla away from the tracer, if possible.
 *
 * Returns 1 on success, 0 otherwise.
 */
pub unsafe extern "C" fn auto_house_keeping(monitored_cpus: *mut cpu_set_t) -> c_int {
    let mut rtla_cpus: cpu_set_t = core::mem::zeroed();
    let mut house_keeping_cpus: cpu_set_t = core::mem::zeroed();

    /* first get the CPUs in which rtla can actually run. */
    let mut retval = sched_getaffinity(
        getpid(),
        core::mem::size_of_val(&rtla_cpus),
        &mut rtla_cpus,
    );
    if retval == -1 {
        debug_msg(c"Could not get rtla affinity, rtla might run with the threads!\n".as_ptr());
        return 0;
    }

    /* then check if the existing setup is already good. */
    CPU_AND(&mut house_keeping_cpus, &rtla_cpus, monitored_cpus);
    if CPU_COUNT(&house_keeping_cpus) == 0 {
        debug_msg(c"rtla and the monitored CPUs do not share CPUs.".as_ptr());
        debug_msg(c"Skipping auto house-keeping\n".as_ptr());
        return 1;
    }

    /* remove the intersection */
    CPU_XOR(&mut house_keeping_cpus, &rtla_cpus, monitored_cpus);

    /* get only those that rtla can run */
    CPU_AND(&mut house_keeping_cpus, &house_keeping_cpus, &rtla_cpus);

    /* is there any cpu left? */
    if CPU_COUNT(&house_keeping_cpus) == 0 {
        debug_msg(c"Could not find any CPU for auto house-keeping\n".as_ptr());
        return 0;
    }

    retval = sched_setaffinity(
        getpid(),
        core::mem::size_of_val(&house_keeping_cpus),
        &house_keeping_cpus,
    );
    if retval == -1 {
        debug_msg(c"Could not set affinity for auto house-keeping\n".as_ptr());
        return 0;
    }

    debug_msg(c"rtla automatically moved to an auto house-keeping cpu set\n".as_ptr());

    1
}

/*
 * strtoi - convert string to integer with error checking
 *
 * Returns 0 on success, -1 if conversion fails or result is out of int range.
 */
pub unsafe extern "C" fn strtoi(s: *const c_char, res: *mut c_int) -> c_int {
    let mut end_ptr: *mut c_char = ptr::null_mut();

    if *s == 0 {
        return -1;
    }

    errno = 0;
    let lres = strtol(s, &mut end_ptr, 0);
    if errno != 0 || *end_ptr != 0 || lres > c_int::MAX as c_long || lres < c_int::MIN as c_long {
        return -1;
    }

    *res = lres as c_int;
    0
}

#[inline]
unsafe fn fatal_alloc() {
    fatal(c"Error allocating memory\n".as_ptr());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
