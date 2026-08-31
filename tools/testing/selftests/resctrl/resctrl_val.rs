// SPDX-License-Identifier: GPL-2.0
/*
 * Memory bandwidth monitoring and allocation library
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
// C dependency: #include "resctrl.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type pid_t = c_int;
type size_t = usize;

const UNCORE_IMC: &[u8] = b"uncore_imc\0";
const READ_FILE_NAME: &[u8] = b"cas_count_read\0";
const DYN_PMU_PATH: &[u8] = b"/sys/bus/event_source/devices\0";
const SCALE: f32 = 0.00006103515625;
const MAX_IMCS: usize = 40;
const MAX_TOKENS: usize = 5;

const CON_MBM_LOCAL_BYTES_PATH: &[u8] =
    b"%s/%s/mon_data/mon_L3_%02d/mbm_local_bytes\0";

const PATH_MAX: usize = 4096;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 2;
const PERF_EVENT_IOC_RESET: c_ulong = 0x2403;
const PERF_EVENT_IOC_ENABLE: c_ulong = 0x2400;
const PERF_EVENT_IOC_DISABLE: c_ulong = 0x2401;
const PERF_FLAG_FD_CLOEXEC: c_ulong = 8;
const SIGKILL: c_int = 9;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGHUP: c_int = 1;
const SIG_DFL: sighandler_t = 0 as sighandler_t;
const SA_SIGINFO: c_int = 4;
const EXIT_SUCCESS: c_int = 0;
const ENOMEM: c_int = 12;
const END_OF_TESTS: c_int = -1;
const MB: c_ulong = 1024 * 1024;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
union sigaction_handler {
    sa_handler: sighandler_t,
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

#[repr(C)]
struct sigaction {
    handler: sigaction_handler,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct sigset_t {
    _private: [u64; 16],
}

#[repr(C)]
struct cpu_set_t {
    _private: [u64; 16],
}

#[repr(C)]
struct perf_event_attr {
    type_: __u32,
    size: __u32,
    config: __u64,
    sample_period_or_freq: __u64,
    sample_type: __u64,
    read_format: __u64,
    flags: __u64,
}

impl perf_event_attr {
    unsafe fn set_disabled(&mut self, v: __u64) {
        if v != 0 {
            self.flags |= 1 << 0;
        } else {
            self.flags &= !(1 << 0);
        }
    }

    unsafe fn set_inherit(&mut self, v: __u64) {
        if v != 0 {
            self.flags |= 1 << 1;
        } else {
            self.flags &= !(1 << 1);
        }
    }

    unsafe fn set_exclude_guest(&mut self, v: __u64) {
        if v != 0 {
            self.flags |= 1 << 20;
        } else {
            self.flags &= !(1 << 20);
        }
    }
}

#[repr(C)]
struct resctrl_test {
    resource: *const c_char,
    cleanup: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct fill_buf_param {
    buf_size: size_t,
    memflush: c_int,
}

#[repr(C)]
struct user_params {
    cpu: c_int,
    benchmark_cmd: [*mut c_char; 0],
}

#[repr(C)]
struct resctrl_val_param {
    filename: *mut c_char,
    ctrlgrp: *const c_char,
    mongrp: *const c_char,
    init: Option<
        unsafe extern "C" fn(
            *const resctrl_test,
            *const user_params,
            *mut resctrl_val_param,
            c_int,
        ) -> c_int,
    >,
    fill_buf: *mut fill_buf_param,
    setup:
        Option<unsafe extern "C" fn(*const resctrl_test, *const user_params, *mut resctrl_val_param) -> c_int>,
    measure:
        Option<unsafe extern "C" fn(*const user_params, *mut resctrl_val_param, pid_t) -> c_int>,
}

#[repr(C)]
struct membw_read_format {
    value: __u64,        /* The value of the event */
    time_enabled: __u64, /* if PERF_FORMAT_TOTAL_TIME_ENABLED */
    time_running: __u64, /* if PERF_FORMAT_TOTAL_TIME_RUNNING */
    id: __u64,           /* if PERF_FORMAT_ID */
}

#[repr(C)]
struct imc_counter_config {
    type_: __u32,
    event: __u64,
    umask: __u64,
    pe: perf_event_attr,
    fd: c_int,
}

unsafe impl Sync for imc_counter_config {}

static mut mbm_total_path: [c_char; 1024] = [0; 1024];
static mut imcs: c_int = 0;
static mut imc_counters_config: [imc_counter_config; MAX_IMCS] = [const {
    imc_counter_config {
        type_: 0,
        event: 0,
        umask: 0,
        pe: perf_event_attr {
            type_: 0,
            size: 0,
            config: 0,
            sample_period_or_freq: 0,
            sample_type: 0,
            read_format: 0,
            flags: 0,
        },
        fd: 0,
    }
}; MAX_IMCS];
static mut current_test: *const resctrl_test = ptr::null();
static mut bm_pid: pid_t = 0;

unsafe extern "C" {
    static RESCTRL_PATH: *const c_char;

    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn rewind(stream: *mut FILE);
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn fabs(x: f64) -> f64;
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn execvp(file: *const c_char, argv: *mut *mut c_char) -> c_int;
    fn free(ptr: *mut c_void);

    fn perf_event_open(
        hw_event: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn umount_resctrlfs() -> c_int;
    fn get_domain_id(resource: *const c_char, cpu_no: c_int, domain_id: *mut c_int) -> c_int;
    fn taskset_benchmark(ppid: pid_t, cpu_no: c_int, old_affinity: *mut cpu_set_t) -> c_int;
    fn write_bm_pid_to_resctrl(pid: pid_t, ctrlgrp: *const c_char, mongrp: *const c_char) -> c_int;
    fn alloc_buffer(buf_size: size_t, memflush: c_int) -> *mut u8;
    fn fill_cache_read(buf: *mut u8, buf_size: size_t, once: bool);
    fn taskset_restore(ppid: pid_t, old_affinity: *mut cpu_set_t) -> c_int;
}

unsafe fn read_mem_bw_initialize_perf_event_attr(i: c_int) {
    memset(
        &mut imc_counters_config[i as usize].pe as *mut perf_event_attr as *mut c_void,
        0,
        size_of::<perf_event_attr>(),
    );
    imc_counters_config[i as usize].pe.type_ = imc_counters_config[i as usize].type_;
    imc_counters_config[i as usize].pe.size = size_of::<perf_event_attr>() as __u32;
    imc_counters_config[i as usize].pe.set_disabled(1);
    imc_counters_config[i as usize].pe.set_inherit(1);
    imc_counters_config[i as usize].pe.set_exclude_guest(0);
    imc_counters_config[i as usize].pe.config =
        (imc_counters_config[i as usize].umask << 8) | imc_counters_config[i as usize].event;
    imc_counters_config[i as usize].pe.sample_type = PERF_SAMPLE_IDENTIFIER;
    imc_counters_config[i as usize].pe.read_format =
        PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
}

unsafe fn read_mem_bw_ioctl_perf_event_ioc_reset_enable(i: c_int) {
    ioctl(imc_counters_config[i as usize].fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(imc_counters_config[i as usize].fd, PERF_EVENT_IOC_ENABLE, 0);
}

unsafe fn read_mem_bw_ioctl_perf_event_ioc_disable(i: c_int) {
    ioctl(imc_counters_config[i as usize].fd, PERF_EVENT_IOC_DISABLE, 0);
}

/*
 * get_read_event_and_umask:	Parse config into event and umask
 * @cas_count_cfg:	Config
 * @count:		iMC number
 */
unsafe fn get_read_event_and_umask(cas_count_cfg: *mut c_char, count: c_uint) {
    let mut token: [*mut c_char; MAX_TOKENS] = [ptr::null_mut(); MAX_TOKENS];
    let mut i: c_int = 0;

    token[0] = strtok(cas_count_cfg, b"=,\0".as_ptr() as *const c_char);

    i = 1;
    while i < MAX_TOKENS as c_int {
        token[i as usize] = strtok(ptr::null_mut(), b"=,\0".as_ptr() as *const c_char);
        i += 1;
    }

    i = 0;
    while i < (MAX_TOKENS - 1) as c_int {
        if token[i as usize].is_null() {
            break;
        }
        if strcmp(token[i as usize], b"event\0".as_ptr() as *const c_char) == 0 {
            imc_counters_config[count as usize].event =
                strtol(token[(i + 1) as usize], ptr::null_mut(), 16) as __u64;
        }
        if strcmp(token[i as usize], b"umask\0".as_ptr() as *const c_char) == 0 {
            imc_counters_config[count as usize].umask =
                strtol(token[(i + 1) as usize], ptr::null_mut(), 16) as __u64;
        }
        i += 1;
    }
}

unsafe fn open_perf_read_event(i: c_int, cpu_no: c_int) -> c_int {
    imc_counters_config[i as usize].fd = perf_event_open(
        &mut imc_counters_config[i as usize].pe,
        -1,
        cpu_no,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    );

    if imc_counters_config[i as usize].fd == -1 {
        fprintf(
            ptr::null_mut(),
            b"Error opening leader %llx\n\0".as_ptr() as *const c_char,
            imc_counters_config[i as usize].pe.config,
        );

        return -1;
    }

    0
}

unsafe fn parse_imc_read_bw_events(
    imc_dir: *mut c_char,
    type_: c_uint,
    count: *mut c_uint,
) -> c_int {
    let mut imc_events_dir: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut imc_counter_cfg: [c_char; PATH_MAX] = [0; PATH_MAX];
    let orig_count: c_uint = *count;
    let mut cas_count_cfg: [c_char; 1024] = [0; 1024];
    let mut ep: *mut dirent;
    let mut path_len: c_int;
    let mut ret: c_int = -1;
    let mut num_cfg: c_int;
    let mut fp: *mut FILE;
    let dp: *mut DIR;

    path_len = snprintf(
        imc_events_dir.as_mut_ptr(),
        imc_events_dir.len(),
        b"%sevents\0".as_ptr() as *const c_char,
        imc_dir,
    );
    if path_len as usize >= imc_events_dir.len() {
        ksft_print_msg(
            b"Unable to create path to %sevents\n\0".as_ptr() as *const c_char,
            imc_dir,
        );
        return -1;
    }

    dp = opendir(imc_events_dir.as_mut_ptr());
    if dp.is_null() {
        ksft_perror(b"Unable to open PMU events directory\0".as_ptr() as *const c_char);
        return -1;
    }

    loop {
        ep = readdir(dp);
        if ep.is_null() {
            break;
        }
        /*
         * Parse all event files with READ_FILE_NAME prefix that
         * contain the event number and umask. Skip files containing
         * "." that contain unused properties of event.
         */
        if strstr((*ep).d_name.as_ptr(), READ_FILE_NAME.as_ptr() as *const c_char).is_null()
            || !strchr((*ep).d_name.as_ptr(), '.' as c_int).is_null()
        {
            continue;
        }

        path_len = snprintf(
            imc_counter_cfg.as_mut_ptr(),
            imc_counter_cfg.len(),
            b"%s/%s\0".as_ptr() as *const c_char,
            imc_events_dir.as_mut_ptr(),
            (*ep).d_name.as_ptr(),
        );
        if path_len as usize >= imc_counter_cfg.len() {
            ksft_print_msg(
                b"Unable to create path to %s/%s\n\0".as_ptr() as *const c_char,
                imc_events_dir.as_mut_ptr(),
                (*ep).d_name.as_ptr(),
            );
            closedir(dp);
            return ret;
        }
        fp = fopen(imc_counter_cfg.as_mut_ptr(), b"r\0".as_ptr() as *const c_char);
        if fp.is_null() {
            ksft_perror(b"Failed to open iMC config file\0".as_ptr() as *const c_char);
            closedir(dp);
            return ret;
        }
        num_cfg = fscanf(
            fp,
            b"%1023s\0".as_ptr() as *const c_char,
            cas_count_cfg.as_mut_ptr(),
        );
        fclose(fp);
        if num_cfg <= 0 {
            ksft_perror(b"Could not get iMC cas count read\0".as_ptr() as *const c_char);
            closedir(dp);
            return ret;
        }
        if *count >= MAX_IMCS as c_uint {
            ksft_print_msg(b"Maximum iMC count exceeded\n\0".as_ptr() as *const c_char);
            closedir(dp);
            return ret;
        }

        imc_counters_config[*count as usize].type_ = type_;
        get_read_event_and_umask(cas_count_cfg.as_mut_ptr(), *count);
        /* Do not fail after incrementing *count. */
        *count += 1;
    }
    if *count == orig_count {
        ksft_print_msg(
            b"Unable to find events in %s\n\0".as_ptr() as *const c_char,
            imc_events_dir.as_mut_ptr(),
        );
        closedir(dp);
        return ret;
    }
    ret = 0;
    closedir(dp);
    ret
}

/* Get type and config of an iMC counter's read event. */
unsafe fn read_from_imc_dir(imc_dir: *mut c_char, count: *mut c_uint) -> c_int {
    let mut imc_counter_type: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut type_: c_uint = 0;
    let path_len: c_int;
    let fp: *mut FILE;
    let mut ret: c_int;

    /* Get type of iMC counter */
    path_len = snprintf(
        imc_counter_type.as_mut_ptr(),
        imc_counter_type.len(),
        b"%s%s\0".as_ptr() as *const c_char,
        imc_dir,
        b"type\0".as_ptr() as *const c_char,
    );
    if path_len as usize >= imc_counter_type.len() {
        ksft_print_msg(
            b"Unable to create path to %s%s\n\0".as_ptr() as *const c_char,
            imc_dir,
            b"type\0".as_ptr() as *const c_char,
        );
        return -1;
    }
    fp = fopen(imc_counter_type.as_mut_ptr(), b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(b"Failed to open iMC counter type file\0".as_ptr() as *const c_char);

        return -1;
    }
    ret = fscanf(fp, b"%u\0".as_ptr() as *const c_char, &mut type_);
    fclose(fp);
    if ret <= 0 {
        ksft_perror(b"Could not get iMC type\0".as_ptr() as *const c_char);
        return -1;
    }
    ret = parse_imc_read_bw_events(imc_dir, type_, count);
    if ret != 0 {
        ksft_print_msg(b"Unable to parse bandwidth event and umask\n\0".as_ptr() as *const c_char);
        return ret;
    }

    0
}

/*
 * A system can have 'n' number of iMC (Integrated Memory Controller)
 * counters, get that 'n'. Discover the properties of the available
 * counters in support of needed performance measurement via perf.
 * For each iMC counter get it's type and config. Also obtain each
 * counter's event and umask for the memory read events that will be
 * measured.
 *
 * Enumerate all these details into an array of structures.
 *
 * Return: >= 0 on success. < 0 on failure.
 */
unsafe fn num_of_imcs() -> c_int {
    let mut imc_dir: [c_char; 512] = [0; 512];
    let mut temp: *mut c_char;
    let mut count: c_uint = 0;
    let mut ep: *mut dirent;
    let mut ret: c_int;
    let dp: *mut DIR;

    dp = opendir(DYN_PMU_PATH.as_ptr() as *const c_char);
    if !dp.is_null() {
        loop {
            ep = readdir(dp);
            if ep.is_null() {
                break;
            }
            temp = strstr((*ep).d_name.as_ptr(), UNCORE_IMC.as_ptr() as *const c_char);
            if temp.is_null() {
                continue;
            }

            /*
             * imc counters are named as "uncore_imc_<n>", hence
             * increment the pointer to point to <n>. Note that
             * sizeof(UNCORE_IMC) would count for null character as
             * well and hence the last underscore character in
             * uncore_imc'_' need not be counted.
             */
            temp = temp.add(UNCORE_IMC.len());

            /*
             * Some directories under "DYN_PMU_PATH" could have
             * names like "uncore_imc_free_running", hence, check if
             * first character is a numerical digit or not.
             */
            if *temp >= b'0' as c_char && *temp <= b'9' as c_char {
                sprintf(
                    imc_dir.as_mut_ptr(),
                    b"%s/%s/\0".as_ptr() as *const c_char,
                    DYN_PMU_PATH.as_ptr() as *const c_char,
                    (*ep).d_name.as_ptr(),
                );
                ret = read_from_imc_dir(imc_dir.as_mut_ptr(), &mut count);
                if ret != 0 {
                    closedir(dp);

                    return ret;
                }
            }
        }
        closedir(dp);
        if count == 0 {
            ksft_print_msg(b"Unable to find iMC counters\n\0".as_ptr() as *const c_char);

            return -1;
        }
    } else {
        ksft_perror(b"Unable to open PMU directory\0".as_ptr() as *const c_char);

        return -1;
    }

    count as c_int
}

#[no_mangle]
pub unsafe extern "C" fn initialize_read_mem_bw_imc() -> c_int {
    let mut imc: c_int;

    imcs = num_of_imcs();
    if imcs <= 0 {
        return imcs;
    }

    /* Initialize perf_event_attr structures for all iMC's */
    imc = 0;
    while imc < imcs {
        read_mem_bw_initialize_perf_event_attr(imc);
        imc += 1;
    }

    0
}

unsafe fn perf_close_imc_read_mem_bw() {
    let mut mc: c_int = 0;

    while mc < imcs {
        if imc_counters_config[mc as usize].fd != -1 {
            close(imc_counters_config[mc as usize].fd);
        }
        mc += 1;
    }
}

/*
 * perf_open_imc_read_mem_bw - Open perf fds for IMCs
 * @cpu_no: CPU number that the benchmark PID is bound to
 *
 * Return: = 0 on success. < 0 on failure.
 */
unsafe fn perf_open_imc_read_mem_bw(cpu_no: c_int) -> c_int {
    let mut imc: c_int;
    let mut ret: c_int;

    imc = 0;
    while imc < imcs {
        imc_counters_config[imc as usize].fd = -1;
        imc += 1;
    }

    imc = 0;
    while imc < imcs {
        ret = open_perf_read_event(imc, cpu_no);
        if ret != 0 {
            perf_close_imc_read_mem_bw();
            return -1;
        }
        imc += 1;
    }

    0
}

/*
 * do_imc_read_mem_bw_test - Perform memory bandwidth test
 *
 * Runs memory bandwidth test over one second period. Also, handles starting
 * and stopping of the IMC perf counters around the test.
 */
unsafe fn do_imc_read_mem_bw_test() {
    let mut imc: c_int;

    imc = 0;
    while imc < imcs {
        read_mem_bw_ioctl_perf_event_ioc_reset_enable(imc);
        imc += 1;
    }

    sleep(1);

    /* Stop counters after a second to get results. */
    imc = 0;
    while imc < imcs {
        read_mem_bw_ioctl_perf_event_ioc_disable(imc);
        imc += 1;
    }
}

/*
 * get_read_mem_bw_imc - Memory read bandwidth as reported by iMC counters
 *
 * Memory read bandwidth utilized by a process on a socket can be calculated
 * using iMC counters' read events. Perf events are used to read these
 * counters.
 *
 * Return: = 0 on success. < 0 on failure.
 */
unsafe fn get_read_mem_bw_imc(bw_imc: *mut f32) -> c_int {
    let mut reads: f32 = 0.0;
    let mut of_mul_read: f32 = 1.0;
    let mut imc: c_int = 0;

    /*
     * Log read event values from all iMC counters into
     * struct imc_counter_config.
     * Take overflow into consideration before calculating total bandwidth.
     */
    while imc < imcs {
        let mut measurement: membw_read_format = zeroed();
        let r: *mut imc_counter_config = &mut imc_counters_config[imc as usize];

        if read(
            (*r).fd,
            &mut measurement as *mut membw_read_format as *mut c_void,
            size_of::<membw_read_format>(),
        ) == -1
        {
            ksft_perror(b"Couldn't get read bandwidth through iMC\0".as_ptr() as *const c_char);
            return -1;
        }

        let r_time_enabled: __u64 = measurement.time_enabled;
        let r_time_running: __u64 = measurement.time_running;

        if r_time_enabled != r_time_running {
            of_mul_read = r_time_enabled as f32 / r_time_running as f32;
        }

        reads += measurement.value as f32 * of_mul_read * SCALE;
        imc += 1;
    }

    *bw_imc = reads;
    0
}

/*
 * initialize_mem_bw_resctrl:	Appropriately populate "mbm_total_path"
 * @param:	Parameters passed to resctrl_val()
 * @domain_id:	Domain ID (cache ID; for MB, L3 cache ID)
 */
#[no_mangle]
pub unsafe extern "C" fn initialize_mem_bw_resctrl(
    param: *const resctrl_val_param,
    domain_id: c_int,
) {
    sprintf(
        mbm_total_path.as_mut_ptr(),
        CON_MBM_LOCAL_BYTES_PATH.as_ptr() as *const c_char,
        RESCTRL_PATH,
        (*param).ctrlgrp,
        domain_id,
    );
}

/*
 * Open file to read MBM local bytes from resctrl FS
 */
unsafe fn open_mem_bw_resctrl(mbm_bw_file: *const c_char) -> *mut FILE {
    let fp: *mut FILE;

    fp = fopen(mbm_bw_file, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(b"Failed to open total memory bandwidth file\0".as_ptr() as *const c_char);
    }

    fp
}

/*
 * Get MBM Local bytes as reported by resctrl FS
 */
unsafe fn get_mem_bw_resctrl(fp: *mut FILE, mbm_total: *mut c_ulong) -> c_int {
    if fscanf(fp, b"%lu\n\0".as_ptr() as *const c_char, mbm_total) <= 0 {
        ksft_perror(b"Could not get MBM local bytes\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ctrlc_handler(signum: c_int, info: *mut siginfo_t, ptr_: *mut c_void) {
    let _ = signum;
    let _ = info;
    let _ = ptr_;

    /* Only kill child after bm_pid is set after fork() */
    if bm_pid != 0 {
        kill(bm_pid, SIGKILL);
    }
    umount_resctrlfs();
    if !current_test.is_null() && (*current_test).cleanup.is_some() {
        ((*current_test).cleanup.unwrap())();
    }
    ksft_print_msg(b"Ending\n\n\0".as_ptr() as *const c_char);

    exit(EXIT_SUCCESS);
}

/*
 * Register CTRL-C handler for parent, as it has to kill
 * child process before exiting.
 */
#[no_mangle]
pub unsafe extern "C" fn signal_handler_register(test: *const resctrl_test) -> c_int {
    let mut sigact: sigaction = zeroed();
    let mut ret: c_int = 0;

    bm_pid = 0;

    current_test = test;
    sigact.handler.sa_sigaction = Some(ctrlc_handler);
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = SA_SIGINFO;
    if sigaction(SIGINT, &sigact, ptr::null_mut()) != 0
        || sigaction(SIGTERM, &sigact, ptr::null_mut()) != 0
        || sigaction(SIGHUP, &sigact, ptr::null_mut()) != 0
    {
        ksft_perror(b"sigaction\0".as_ptr() as *const c_char);
        ret = -1;
    }
    ret
}

/*
 * Reset signal handler to SIG_DFL.
 * Non-Value return because the caller should keep
 * the error code of other path even if sigaction fails.
 */
#[no_mangle]
pub unsafe extern "C" fn signal_handler_unregister() {
    let mut sigact: sigaction = zeroed();

    current_test = ptr::null();
    sigact.handler.sa_handler = SIG_DFL;
    sigemptyset(&mut sigact.sa_mask);
    if sigaction(SIGINT, &sigact, ptr::null_mut()) != 0
        || sigaction(SIGTERM, &sigact, ptr::null_mut()) != 0
        || sigaction(SIGHUP, &sigact, ptr::null_mut()) != 0
    {
        ksft_perror(b"sigaction\0".as_ptr() as *const c_char);
    }
}

/*
 * print_results_bw:	the memory bandwidth results are stored in a file
 * @filename:		file that stores the results
 * @bm_pid:		child pid that runs benchmark
 * @bw_imc:		perf imc counter value
 * @bw_resc:		memory bandwidth value
 *
 * Return:		0 on success, < 0 on error.
 */
unsafe fn print_results_bw(
    filename: *mut c_char,
    bm_pid_arg: pid_t,
    bw_imc: f32,
    bw_resc: c_ulong,
) -> c_int {
    let diff: c_ulong = fabs((bw_imc as f64) - (bw_resc as f64)) as c_ulong;
    let fp: *mut FILE;

    if strcmp(filename, b"stdio\0".as_ptr() as *const c_char) == 0
        || strcmp(filename, b"stderr\0".as_ptr() as *const c_char) == 0
    {
        printf(
            b"Pid: %d \t Mem_BW_iMC: %f \t \0".as_ptr() as *const c_char,
            bm_pid_arg as c_int,
            bw_imc as f64,
        );
        printf(
            b"Mem_BW_resc: %lu \t Difference: %lu\n\0".as_ptr() as *const c_char,
            bw_resc,
            diff,
        );
    } else {
        fp = fopen(filename, b"a\0".as_ptr() as *const c_char);
        if fp.is_null() {
            ksft_perror(b"Cannot open results file\0".as_ptr() as *const c_char);

            return -1;
        }
        if fprintf(
            fp,
            b"Pid: %d \t Mem_BW_iMC: %f \t Mem_BW_resc: %lu \t Difference: %lu\n\0".as_ptr()
                as *const c_char,
            bm_pid_arg as c_int,
            bw_imc as f64,
            bw_resc,
            diff,
        ) <= 0
        {
            ksft_print_msg(b"Could not log results\n\0".as_ptr() as *const c_char);
            fclose(fp);

            return -1;
        }
        fclose(fp);
    }

    0
}

/*
 * measure_read_mem_bw - Measures read memory bandwidth numbers while benchmark runs
 * @uparams:		User supplied parameters
 * @param:		Parameters passed to resctrl_val()
 * @bm_pid:		PID that runs the benchmark
 *
 * Measure memory bandwidth from resctrl and from another source which is
 * perf imc value or could be something else if perf imc event is not
 * available. Compare the two values to validate resctrl value. It takes
 * 1 sec to measure the data.
 * resctrl does not distinguish between read and write operations so
 * its data includes all memory operations.
 */
#[no_mangle]
pub unsafe extern "C" fn measure_read_mem_bw(
    uparams: *const user_params,
    param: *mut resctrl_val_param,
    bm_pid_arg: pid_t,
) -> c_int {
    let mut bw_resc: c_ulong;
    let mut bw_resc_start: c_ulong = 0;
    let mut bw_resc_end: c_ulong = 0;
    let mem_bw_fp: *mut FILE;
    let mut bw_imc: f32 = 0.0;
    let mut ret: c_int;

    mem_bw_fp = open_mem_bw_resctrl(mbm_total_path.as_mut_ptr());
    if mem_bw_fp.is_null() {
        return -1;
    }

    ret = perf_open_imc_read_mem_bw((*uparams).cpu);
    if ret < 0 {
        fclose(mem_bw_fp);
        return ret;
    }

    ret = get_mem_bw_resctrl(mem_bw_fp, &mut bw_resc_start);
    if ret < 0 {
        perf_close_imc_read_mem_bw();
        fclose(mem_bw_fp);
        return ret;
    }

    rewind(mem_bw_fp);

    do_imc_read_mem_bw_test();

    ret = get_mem_bw_resctrl(mem_bw_fp, &mut bw_resc_end);
    if ret < 0 {
        perf_close_imc_read_mem_bw();
        fclose(mem_bw_fp);
        return ret;
    }

    ret = get_read_mem_bw_imc(&mut bw_imc);
    if ret < 0 {
        perf_close_imc_read_mem_bw();
        fclose(mem_bw_fp);
        return ret;
    }

    perf_close_imc_read_mem_bw();
    fclose(mem_bw_fp);

    bw_resc = (bw_resc_end - bw_resc_start) / MB;

    print_results_bw((*param).filename, bm_pid_arg, bw_imc, bw_resc)
}

/*
 * resctrl_val:	execute benchmark and measure memory bandwidth on
 *			the benchmark
 * @test:		test information structure
 * @uparams:		user supplied parameters
 * @param:		parameters passed to resctrl_val()
 *
 * Return:		0 when the test was run, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn resctrl_val(
    test: *const resctrl_test,
    uparams: *const user_params,
    param: *mut resctrl_val_param,
) -> c_int {
    let mut buf: *mut u8 = ptr::null_mut();
    let mut old_affinity: cpu_set_t = zeroed();
    let mut domain_id: c_int = 0;
    let mut ret: c_int = 0;
    let ppid: pid_t;

    if strcmp((*param).filename, b"\0".as_ptr() as *const c_char) == 0 {
        sprintf((*param).filename, b"stdio\0".as_ptr() as *const c_char);
    }

    ret = get_domain_id((*test).resource, (*uparams).cpu, &mut domain_id);
    if ret < 0 {
        ksft_print_msg(b"Could not get domain ID\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ppid = getpid();

    /* Taskset test to specified CPU. */
    ret = taskset_benchmark(ppid, (*uparams).cpu, &mut old_affinity);
    if ret != 0 {
        return ret;
    }

    /* Write test to specified control & monitoring group in resctrl FS. */
    ret = write_bm_pid_to_resctrl(ppid, (*param).ctrlgrp, (*param).mongrp);
    if ret != 0 {
        taskset_restore(ppid, &mut old_affinity);
        return ret;
    }

    if (*param).init.is_some() {
        ret = ((*param).init.unwrap())(test, uparams, param, domain_id);
        if ret != 0 {
            taskset_restore(ppid, &mut old_affinity);
            return ret;
        }
    }

    /*
     * If not running user provided benchmark, run the default
     * "fill_buf". First phase of "fill_buf" is to prepare the
     * buffer that the benchmark will operate on. No measurements
     * are needed during this phase and prepared memory will be
     * passed to next part of benchmark via copy-on-write thus
     * no impact on the benchmark that relies on reading from
     * memory only.
     */
    if !(*param).fill_buf.is_null() {
        buf = alloc_buffer((*(*param).fill_buf).buf_size, (*(*param).fill_buf).memflush);
        if buf.is_null() {
            ret = -ENOMEM;
            taskset_restore(ppid, &mut old_affinity);
            return ret;
        }
    }

    fflush(ptr::null_mut());
    bm_pid = fork();
    if bm_pid == -1 {
        ret = -1;
        ksft_perror(b"Unable to fork\0".as_ptr() as *const c_char);
        free(buf as *mut c_void);
        taskset_restore(ppid, &mut old_affinity);
        return ret;
    }

    /*
     * What needs to be measured runs in separate process until
     * terminated.
     */
    if bm_pid == 0 {
        if !(*param).fill_buf.is_null() {
            fill_cache_read(buf, (*(*param).fill_buf).buf_size, false);
        } else if !(*uparams).benchmark_cmd[0].is_null() {
            execvp(
                (*uparams).benchmark_cmd[0],
                (*uparams).benchmark_cmd.as_ptr() as *mut *mut c_char,
            );
        }
        exit(EXIT_SUCCESS);
    }

    ksft_print_msg(
        b"Benchmark PID: %d\n\0".as_ptr() as *const c_char,
        bm_pid as c_int,
    );

    /* Give benchmark enough time to fully run. */
    sleep(1);

    /* Test runs until the callback setup() tells the test to stop. */
    loop {
        ret = ((*param).setup.unwrap())(test, uparams, param);
        if ret == END_OF_TESTS {
            ret = 0;
            break;
        }
        if ret < 0 {
            break;
        }

        ret = ((*param).measure.unwrap())(uparams, param, bm_pid);
        if ret != 0 {
            break;
        }
    }

    kill(bm_pid, SIGKILL);
    free(buf as *mut c_void);
    taskset_restore(ppid, &mut old_affinity);
    ret
}
