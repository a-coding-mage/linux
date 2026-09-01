// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type __u64 = u64;
pub type pid_t = c_int;
pub type size_t = usize;

// External declarations expected from translated headers and support files:
// perf_event_attr, resctrl_test, user_params, resctrl_val_param, FILE,
// PERF_TYPE_HARDWARE, PERF_EVENT_IOC_RESET, PERF_EVENT_IOC_ENABLE,
// PERF_EVENT_IOC_DISABLE, PERF_FLAG_FD_CLOEXEC, perf_event_open,
// ksft_perror, ksft_print_msg, resctrl_resource_exists, and write_schemata.

pub static mut llc_occup_path: [c_char; 1024] = [0; 1024];

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

pub unsafe fn perf_event_attr_initialize(pea: *mut perf_event_attr, config: __u64) {
    unsafe {
        memset(
            pea as *mut c_void,
            0,
            core::mem::size_of_val(&*pea) as size_t,
        );
        (*pea).type_ = PERF_TYPE_HARDWARE;
        (*pea).size = core::mem::size_of_val(&*pea) as _;
        (*pea).exclude_kernel = 1;
        (*pea).exclude_hv = 1;
        (*pea).exclude_idle = 1;
        (*pea).exclude_callchain_kernel = 1;
        (*pea).inherit = 1;
        (*pea).exclude_guest = 1;
        (*pea).disabled = 1;
        (*pea).config = config;
    }
}

/* Start counters to log values */
pub unsafe fn perf_event_reset_enable(pe_fd: c_int) -> c_int {
    let mut ret: c_int;

    unsafe {
        ret = ioctl(pe_fd, PERF_EVENT_IOC_RESET as c_ulong, 0);
        if ret < 0 {
            return ret;
        }

        ret = ioctl(pe_fd, PERF_EVENT_IOC_ENABLE as c_ulong, 0);
        if ret < 0 {
            return ret;
        }
    }

    0
}

pub unsafe fn perf_open(pea: *mut perf_event_attr, pid: pid_t, cpu_no: c_int) -> c_int {
    let pe_fd: c_int;

    unsafe {
        pe_fd = perf_event_open(pea, pid, cpu_no, -1, PERF_FLAG_FD_CLOEXEC);
        if pe_fd == -1 {
            ksft_perror(c"Unable to set up performance monitoring".as_ptr());
            return -1;
        }

        perf_event_reset_enable(pe_fd);
    }

    pe_fd
}

/*
 * Get LLC Occupancy as reported by RESCTRL FS
 * For CMT,
 * 1. If con_mon grp and mon grp given, then read from mon grp in
 * con_mon grp
 * 2. If only con_mon grp given, then read from con_mon grp
 * 3. If both not given, then read from root con_mon grp
 * For CAT,
 * 1. If con_mon grp given, then read from it
 * 2. If con_mon grp not given, then read from root con_mon grp
 *
 * Return: =0 on success.  <0 on failure.
 */
unsafe fn get_llc_occu_resctrl(llc_occupancy: *mut c_ulong) -> c_int {
    let fp: *mut FILE;

    unsafe {
        fp = fopen(llc_occup_path.as_ptr(), c"r".as_ptr());
        if fp.is_null() {
            ksft_perror(c"Failed to open results file".as_ptr());

            return -1;
        }
        if fscanf(fp, c"%lu".as_ptr(), llc_occupancy) <= 0 {
            ksft_perror(c"Could not get llc occupancy".as_ptr());
            fclose(fp);

            return -1;
        }
        fclose(fp);
    }

    0
}

/*
 * print_results_cache:	the cache results are stored in a file
 * @filename:		file that stores the results
 * @bm_pid:		child pid that runs benchmark
 * @llc_value:		perf miss value /
 *			llc occupancy value reported by resctrl FS
 *
 * Return:		0 on success, < 0 on error.
 */
unsafe fn print_results_cache(filename: *const c_char, bm_pid: pid_t, llc_value: __u64) -> c_int {
    let fp: *mut FILE;

    unsafe {
        if strcmp(filename, c"stdio".as_ptr()) == 0 || strcmp(filename, c"stderr".as_ptr()) == 0 {
            printf(
                c"Pid: %d \t LLC_value: %llu\n".as_ptr(),
                bm_pid as c_int,
                llc_value,
            );
        } else {
            fp = fopen(filename, c"a".as_ptr());
            if fp.is_null() {
                ksft_perror(c"Cannot open results file".as_ptr());

                return -1;
            }
            fprintf(
                fp,
                c"Pid: %d \t llc_value: %llu\n".as_ptr(),
                bm_pid as c_int,
                llc_value,
            );
            fclose(fp);
        }
    }

    0
}

/*
 * perf_event_measure - Measure perf events
 * @filename:	Filename for writing the results
 * @bm_pid:	PID that runs the benchmark
 *
 * Measures perf events (e.g., cache misses) and writes the results into
 * @filename. @bm_pid is written to the results file along with the measured
 * value.
 *
 * Return: =0 on success. <0 on failure.
 */
pub unsafe fn perf_event_measure(pe_fd: c_int, filename: *const c_char, bm_pid: pid_t) -> c_int {
    let mut value: __u64 = 0;
    let mut ret: c_int;

    unsafe {
        /* Stop counters after one span to get miss rate */
        ret = ioctl(pe_fd, PERF_EVENT_IOC_DISABLE as c_ulong, 0);
        if ret < 0 {
            return ret;
        }

        ret = read(
            pe_fd,
            &mut value as *mut __u64 as *mut c_void,
            core::mem::size_of_val(&value) as size_t,
        ) as c_int;
        if ret == -1 {
            ksft_perror(c"Could not get perf value".as_ptr());
            return -1;
        }

        print_results_cache(filename, bm_pid, value)
    }
}

/*
 * measure_llc_resctrl - Measure resctrl LLC value from resctrl
 * @filename:	Filename for writing the results
 * @bm_pid:	PID that runs the benchmark
 *
 * Measures LLC occupancy from resctrl and writes the results into @filename.
 * @bm_pid is written to the results file along with the measured value.
 *
 * Return: =0 on success. <0 on failure.
 */
pub unsafe fn measure_llc_resctrl(filename: *const c_char, bm_pid: pid_t) -> c_int {
    let mut llc_occu_resc: c_ulong = 0;
    let ret: c_int;

    unsafe {
        ret = get_llc_occu_resctrl(&mut llc_occu_resc);
        if ret < 0 {
            return ret;
        }

        print_results_cache(filename, bm_pid, llc_occu_resc as __u64)
    }
}

/*
 * Reduce L2 allocation to minimum when testing L3 cache allocation.
 */
pub unsafe fn minimize_l2_occupancy(
    test: *const resctrl_test,
    uparams: *const user_params,
    param: *const resctrl_val_param,
) -> c_int {
    unsafe {
        if strcmp((*test).resource, c"L3".as_ptr()) == 0 && resctrl_resource_exists(c"L2".as_ptr()) {
            return write_schemata((*param).ctrlgrp, c"0x1".as_ptr(), (*uparams).cpu, c"L2".as_ptr());
        }
    }

    0
}

/*
 * show_cache_info - Show generic cache test information
 * @no_of_bits:		Number of bits
 * @avg_llc_val:	Average of LLC cache result data
 * @cache_span:		Cache span
 * @lines:		@cache_span in lines or bytes
 */
pub unsafe fn show_cache_info(no_of_bits: c_int, avg_llc_val: __u64, cache_span: size_t, lines: bool) {
    unsafe {
        ksft_print_msg(c"Number of bits: %d\n".as_ptr(), no_of_bits);
        ksft_print_msg(c"Average LLC val: %llu\n".as_ptr(), avg_llc_val);
        ksft_print_msg(
            c"Cache span (%s): %zu\n".as_ptr(),
            if lines {
                c"lines".as_ptr()
            } else {
                c"bytes".as_ptr()
            },
            cache_span,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
