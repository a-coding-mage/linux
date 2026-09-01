// SPDX-License-Identifier: GPL-2.0
/*
 * Cache Monitoring Technology (CMT) test
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
/* Translated from resctrl.h-dependent C implementation. */

use core::ffi::{c_char, c_float, c_int, c_ulong, c_void};

const RESULT_FILE_NAME: *const c_char = b"result_cmt\0".as_ptr() as *const c_char;
const NUM_OF_RUNS: c_int = 5;
const MAX_DIFF: c_ulong = 2000000;
const MAX_DIFF_PERCENT: c_ulong = 15;

const CON_MON_LCC_OCCUP_PATH: *const c_char =
    b"%s/%s/mon_data/mon_L3_%02d/llc_occupancy\0".as_ptr() as *const c_char;

const END_OF_TESTS: c_int = 1;
const ARCH_INTEL: c_int = 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub resource: *const c_char,
    pub feature_check: Option<unsafe extern "C" fn(*const resctrl_test) -> bool>,
    pub run_test: Option<unsafe extern "C" fn(*const resctrl_test, *const user_params) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct user_params {
    pub cpu: c_int,
    pub bits: c_int,
    pub fill_buf: *mut fill_buf_param,
    pub benchmark_cmd: [c_char; 0],
}

#[repr(C)]
pub struct fill_buf_param {
    pub buf_size: usize,
    pub memflush: bool,
}

#[repr(C)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub filename: *const c_char,
    pub mask: c_ulong,
    pub num_of_runs: c_int,
    pub init: Option<
        unsafe extern "C" fn(
            *const resctrl_test,
            *const user_params,
            *const resctrl_val_param,
            c_int,
        ) -> c_int,
    >,
    pub setup: Option<
        unsafe extern "C" fn(
            *const resctrl_test,
            *const user_params,
            *mut resctrl_val_param,
        ) -> c_int,
    >,
    pub measure:
        Option<unsafe extern "C" fn(*const user_params, *mut resctrl_val_param, pid_t) -> c_int>,
    pub fill_buf: *mut fill_buf_param,
}

#[allow(non_camel_case_types)]
pub type pid_t = c_int;

unsafe extern "C" {
    static mut llc_occup_path: [c_char; 0];
    static RESCTRL_PATH: *const c_char;

    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn remove(pathname: *const c_char) -> c_int;

    fn get_full_cbm(resource: *const c_char, mask: *mut c_ulong) -> c_int;
    fn write_schemata(
        ctrlgrp: *const c_char,
        schemata: *const c_char,
        cpu: c_int,
        resource: *const c_char,
    ) -> c_int;
    fn minimize_l2_occupancy(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *const resctrl_val_param,
    ) -> c_int;
    fn measure_llc_resctrl(filename: *const c_char, bm_pid: pid_t) -> c_int;
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn show_cache_info(
        no_of_bits: c_int,
        avg_llc_val: c_ulong,
        cache_span: c_ulong,
        lines: bool,
    );
    fn get_cache_size(cpu: c_int, resource: *const c_char, cache_size: *mut c_ulong) -> c_int;
    fn count_bits(mask: c_ulong) -> c_int;
    fn cache_portion_size(cache_total_size: c_ulong, mask: c_ulong, long_mask: c_ulong) -> usize;
    fn resctrl_val(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *mut resctrl_val_param,
    ) -> c_int;
    fn get_vendor() -> c_int;
    fn snc_kernel_support() -> bool;
    fn test_resource_feature_check(test: *const resctrl_test) -> bool;
    fn resctrl_mon_feature_exists(resource: *const c_char, feature: *const c_char) -> bool;
}

#[allow(non_camel_case_types)]
type c_uint = u32;

/*
 * Initialize capacity bitmasks (CBMs) of:
 * - control group being tested per test parameters,
 * - default resource group as inverse of control group being tested to prevent
 *   other tasks from interfering with test,
 * - L2 resource of control group being tested to minimize allocations into
 *   L2 if possible to better predict L3 occupancy.
 */
unsafe extern "C" fn cmt_init(
    test: *const resctrl_test,
    uparams: *const user_params,
    param: *const resctrl_val_param,
    domain_id: c_int,
) -> c_int {
    let mut full_mask: c_ulong = 0;
    let mut schemata: [c_char; 64] = [0; 64];
    let mut ret: c_int;

    sprintf(
        llc_occup_path.as_mut_ptr(),
        CON_MON_LCC_OCCUP_PATH,
        RESCTRL_PATH,
        (*param).ctrlgrp,
        domain_id,
    );

    ret = get_full_cbm((*test).resource, &mut full_mask);
    if ret != 0 {
        return ret;
    }

    snprintf(
        schemata.as_mut_ptr(),
        schemata.len(),
        b"%lx\0".as_ptr() as *const c_char,
        !(*param).mask & full_mask,
    );
    ret = write_schemata(
        b"\0".as_ptr() as *const c_char,
        schemata.as_ptr(),
        (*uparams).cpu,
        (*test).resource,
    );
    if ret != 0 {
        return ret;
    }

    snprintf(
        schemata.as_mut_ptr(),
        schemata.len(),
        b"%lx\0".as_ptr() as *const c_char,
        (*param).mask,
    );
    ret = write_schemata(
        (*param).ctrlgrp,
        schemata.as_ptr(),
        (*uparams).cpu,
        (*test).resource,
    );
    if ret != 0 {
        return ret;
    }

    minimize_l2_occupancy(test, uparams, param)
}

unsafe extern "C" fn cmt_setup(
    _test: *const resctrl_test,
    _uparams: *const user_params,
    p: *mut resctrl_val_param,
) -> c_int {
    /* Run NUM_OF_RUNS times */
    if (*p).num_of_runs >= NUM_OF_RUNS {
        return END_OF_TESTS;
    }

    (*p).num_of_runs += 1;

    0
}

unsafe extern "C" fn cmt_measure(
    _uparams: *const user_params,
    param: *mut resctrl_val_param,
    bm_pid: pid_t,
) -> c_int {
    sleep(1);
    measure_llc_resctrl((*param).filename, bm_pid)
}

unsafe extern "C" fn show_results_info(
    sum_llc_val: c_ulong,
    no_of_bits: c_int,
    cache_span: c_ulong,
    max_diff: c_ulong,
    max_diff_percent: c_ulong,
    num_of_runs: c_ulong,
    platform: bool,
) -> c_int {
    let mut avg_llc_val: c_ulong = 0;
    let diff_percent: c_float;
    let mut avg_diff: i64 = 0;
    let ret: c_int;

    avg_llc_val = sum_llc_val / num_of_runs;
    avg_diff = (cache_span.wrapping_sub(avg_llc_val)) as i64;
    diff_percent = ((cache_span as c_float) - (avg_llc_val as c_float)) / (cache_span as c_float)
        * 100.0;

    ret = (platform
        && (diff_percent as c_int).abs() as c_ulong > max_diff_percent
        && avg_diff.abs() as c_ulong > max_diff) as c_int;

    ksft_print_msg(
        b"%s Check cache miss rate within %lu%%\n\0".as_ptr() as *const c_char,
        if ret != 0 {
            b"Fail:\0".as_ptr() as *const c_char
        } else {
            b"Pass:\0".as_ptr() as *const c_char
        },
        max_diff_percent,
    );

    ksft_print_msg(
        b"Percent diff=%d\n\0".as_ptr() as *const c_char,
        (diff_percent as c_int).abs(),
    );

    show_cache_info(no_of_bits, avg_llc_val, cache_span, false);

    ret
}

unsafe extern "C" fn check_results(
    param: *mut resctrl_val_param,
    span: usize,
    no_of_bits: c_int,
) -> c_int {
    let mut token_array: [*mut c_char; 8] = [core::ptr::null_mut(); 8];
    let mut temp: [c_char; 512] = [0; 512];
    let mut sum_llc_occu_resc: c_ulong = 0;
    let mut runs: c_int = 0;
    let fp: *mut FILE;

    ksft_print_msg(b"Checking for pass/fail\n\0".as_ptr() as *const c_char);
    fp = fopen((*param).filename, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(b"Error in opening file\0".as_ptr() as *const c_char);

        return -1;
    }

    while !fgets(temp.as_mut_ptr(), temp.len() as c_int, fp).is_null() {
        let mut token: *mut c_char =
            strtok(temp.as_mut_ptr(), b":\t\0".as_ptr() as *const c_char);
        let mut fields: c_int = 0;

        while !token.is_null() {
            token_array[fields as usize] = token;
            fields += 1;
            token = strtok(
                core::ptr::null_mut(),
                b":\t\0".as_ptr() as *const c_char,
            );
        }

        /* Field 3 is llc occ resc value */
        sum_llc_occu_resc = sum_llc_occu_resc.wrapping_add(strtoul(
            token_array[3],
            core::ptr::null_mut(),
            0,
        ));
        runs += 1;
    }
    fclose(fp);

    show_results_info(
        sum_llc_occu_resc,
        no_of_bits,
        span as c_ulong,
        MAX_DIFF,
        MAX_DIFF_PERCENT,
        runs as c_ulong,
        true,
    )
}

unsafe extern "C" fn cmt_test_cleanup() {
    remove(RESULT_FILE_NAME);
}

unsafe extern "C" fn cmt_run_test(
    test: *const resctrl_test,
    uparams: *const user_params,
) -> c_int {
    let mut fill_buf: fill_buf_param = fill_buf_param {
        buf_size: 0,
        memflush: false,
    };
    let mut cache_total_size: c_ulong = 0;
    let n: c_int = if (*uparams).bits != 0 {
        (*uparams).bits
    } else {
        5
    };
    let mut long_mask: c_ulong = 0;
    let count_of_bits: c_int;
    let span: usize;
    let mut ret: c_int;

    ret = get_full_cbm(b"L3\0".as_ptr() as *const c_char, &mut long_mask);
    if ret != 0 {
        return ret;
    }

    ret = get_cache_size(
        (*uparams).cpu,
        b"L3\0".as_ptr() as *const c_char,
        &mut cache_total_size,
    );
    if ret != 0 {
        return ret;
    }
    ksft_print_msg(
        b"Cache size :%lu\n\0".as_ptr() as *const c_char,
        cache_total_size,
    );

    count_of_bits = count_bits(long_mask);

    if n < 1 || n > count_of_bits {
        ksft_print_msg(
            b"Invalid input value for numbr_of_bits n!\n\0".as_ptr() as *const c_char,
        );
        ksft_print_msg(
            b"Please enter value in range 1 to %d\n\0".as_ptr() as *const c_char,
            count_of_bits,
        );
        return -1;
    }

    let mut param: resctrl_val_param = resctrl_val_param {
        ctrlgrp: b"c1\0".as_ptr() as *const c_char,
        filename: RESULT_FILE_NAME,
        mask: !(long_mask << n) & long_mask,
        num_of_runs: 0,
        init: Some(cmt_init),
        setup: Some(cmt_setup),
        measure: Some(cmt_measure),
        fill_buf: core::ptr::null_mut(),
    };

    span = cache_portion_size(cache_total_size, param.mask, long_mask);

    if !(*uparams).fill_buf.is_null() {
        fill_buf.buf_size = span * 2;
        fill_buf.memflush = (*(*uparams).fill_buf).memflush;
        param.fill_buf = &mut fill_buf;
    } else if *(*uparams).benchmark_cmd.as_ptr() == 0 {
        fill_buf.buf_size = span * 2;
        fill_buf.memflush = true;
        param.fill_buf = &mut fill_buf;
    }

    remove(RESULT_FILE_NAME);

    ret = resctrl_val(test, uparams, &mut param);
    if ret != 0 {
        return ret;
    }

    ret = check_results(&mut param, span, n);
    if ret != 0 && get_vendor() == ARCH_INTEL && !snc_kernel_support() {
        ksft_print_msg(
            b"Kernel doesn't support Sub-NUMA Clustering but it is enabled on the system.\n\0"
                .as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn cmt_feature_check(test: *const resctrl_test) -> bool {
    test_resource_feature_check(test)
        && resctrl_mon_feature_exists(
            b"L3_MON\0".as_ptr() as *const c_char,
            b"llc_occupancy\0".as_ptr() as *const c_char,
        )
}

#[unsafe(no_mangle)]
pub static mut cmt_test: resctrl_test = resctrl_test {
    name: b"CMT\0".as_ptr() as *const c_char,
    resource: b"L3\0".as_ptr() as *const c_char,
    feature_check: Some(cmt_feature_check),
    run_test: Some(cmt_run_test),
    cleanup: Some(cmt_test_cleanup),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
