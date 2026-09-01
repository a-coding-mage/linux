// SPDX-License-Identifier: GPL-2.0
/*
 * Memory Bandwidth Monitoring (MBM) test
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
/* Rust translation of implementation source originally including "resctrl.h". */

use core::ffi::{c_char, c_float, c_int, c_long, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const RESULT_FILE_NAME: &[u8] = b"result_mbm\0";
const MAX_DIFF_PERCENT: c_int = 15;
const NUM_OF_RUNS: usize = 5;

const END_OF_TESTS: c_int = 1;
const ARCH_INTEL: c_int = 1;
const MB: usize = 1024 * 1024;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fill_buf_param {
    pub buf_size: usize,
    pub memflush: bool,
}

#[repr(C)]
pub struct user_params {
    pub cpu: c_int,
    pub fill_buf: *mut fill_buf_param,
    pub benchmark_cmd: [c_char; 1024],
}

#[repr(C)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub filename: *const c_char,
    pub init: Option<
        unsafe extern "C" fn(
            test: *const resctrl_test,
            uparams: *const user_params,
            param: *const resctrl_val_param,
            domain_id: c_int,
        ) -> c_int,
    >,
    pub setup: Option<
        unsafe extern "C" fn(
            test: *const resctrl_test,
            uparams: *const user_params,
            p: *mut resctrl_val_param,
        ) -> c_int,
    >,
    pub measure: Option<
        unsafe extern "C" fn(
            uparams: *const user_params,
            param: *mut resctrl_val_param,
            bm_pid: pid_t,
        ) -> c_int,
    >,
    pub fill_buf: *mut fill_buf_param,
    pub num_of_runs: c_int,
}

#[repr(C)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub resource: *const c_char,
    pub vendor_specific: c_int,
    pub feature_check: Option<unsafe extern "C" fn(test: *const resctrl_test) -> bool>,
    pub run_test:
        Option<unsafe extern "C" fn(test: *const resctrl_test, uparams: *const user_params) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn()>,
}

#[allow(non_camel_case_types)]
type pid_t = c_int;

unsafe extern "C" {
    fn labs(j: c_long) -> c_long;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn remove(pathname: *const c_char) -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn initialize_read_mem_bw_imc() -> c_int;
    fn initialize_mem_bw_resctrl(param: *const resctrl_val_param, domain_id: c_int);
    fn resctrl_resource_exists(resource: *const c_char) -> bool;
    fn write_schemata(
        ctrlgrp: *const c_char,
        schemata: *const c_char,
        cpu: c_int,
        resource: *const c_char,
    ) -> c_int;
    fn measure_read_mem_bw(
        uparams: *const user_params,
        param: *mut resctrl_val_param,
        bm_pid: pid_t,
    ) -> c_int;
    fn get_fill_buf_size(cpu: c_int, cache_type: *const c_char) -> isize;
    fn resctrl_val(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *mut resctrl_val_param,
    ) -> c_int;
    fn get_vendor() -> c_int;
    fn snc_kernel_support() -> bool;
    fn resctrl_mon_feature_exists(resource: *const c_char, feature: *const c_char) -> bool;
}

unsafe extern "C" fn show_bw_info(
    bw_imc: *mut c_ulong,
    bw_resc: *mut c_ulong,
    span: usize,
) -> c_int {
    let mut sum_bw_imc: c_ulong = 0;
    let mut sum_bw_resc: c_ulong = 0;
    let mut avg_bw_imc: c_long = 0;
    let mut avg_bw_resc: c_long = 0;
    let mut runs: c_int;
    let ret: c_int;
    let avg_diff_per: c_int;
    let mut avg_diff: c_float = 0.0;

    runs = 0;
    while runs < NUM_OF_RUNS as c_int {
        sum_bw_imc = sum_bw_imc.wrapping_add(*bw_imc.add(runs as usize));
        sum_bw_resc = sum_bw_resc.wrapping_add(*bw_resc.add(runs as usize));
        runs += 1;
    }

    avg_bw_imc = (sum_bw_imc / NUM_OF_RUNS as c_ulong) as c_long;
    avg_bw_resc = (sum_bw_resc / NUM_OF_RUNS as c_ulong) as c_long;
    avg_diff = labs(avg_bw_resc - avg_bw_imc) as c_float / avg_bw_imc as c_float;
    avg_diff_per = (avg_diff * 100.0) as c_int;

    ret = (avg_diff_per > MAX_DIFF_PERCENT) as c_int;
    ksft_print_msg(
        b"%s Check MBM diff within %d%%\n\0".as_ptr() as *const c_char,
        if ret != 0 {
            b"Fail:\0".as_ptr()
        } else {
            b"Pass:\0".as_ptr()
        } as *const c_char,
        MAX_DIFF_PERCENT,
    );
    ksft_print_msg(
        b"avg_diff_per: %d%%\n\0".as_ptr() as *const c_char,
        avg_diff_per,
    );
    if span != 0 {
        ksft_print_msg(
            b"Span (MB): %zu\n\0".as_ptr() as *const c_char,
            span / MB,
        );
    }
    ksft_print_msg(
        b"avg_bw_imc: %lu\n\0".as_ptr() as *const c_char,
        avg_bw_imc as c_ulong,
    );
    ksft_print_msg(
        b"avg_bw_resc: %lu\n\0".as_ptr() as *const c_char,
        avg_bw_resc as c_ulong,
    );

    ret
}

unsafe extern "C" fn check_results(span: usize) -> c_int {
    let mut bw_imc: [c_ulong; NUM_OF_RUNS] = [0; NUM_OF_RUNS];
    let mut bw_resc: [c_ulong; NUM_OF_RUNS] = [0; NUM_OF_RUNS];
    let mut temp: [c_char; 1024] = [0; 1024];
    let mut token_array: [*mut c_char; 8] = [ptr::null_mut(); 8];
    let output: [c_char; 11] = *b"result_mbm\0".as_ptr().cast::<[c_char; 11]>();
    let mut runs: c_int;
    let ret: c_int;
    let fp: *mut FILE;

    ksft_print_msg(b"Checking for pass/fail\n\0".as_ptr() as *const c_char);

    fp = fopen(output.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(output.as_ptr());

        return -1;
    }

    runs = 0;
    while !fgets(temp.as_mut_ptr(), temp.len() as c_int, fp).is_null() {
        let mut token: *mut c_char =
            strtok(temp.as_mut_ptr(), b":\t\0".as_ptr() as *const c_char);
        let mut i: c_int = 0;

        while !token.is_null() {
            token_array[i as usize] = token;
            i += 1;
            token = strtok(ptr::null_mut(), b":\t\0".as_ptr() as *const c_char);
        }

        bw_resc[runs as usize] = strtoul(token_array[5], ptr::null_mut(), 0);
        bw_imc[runs as usize] = strtoul(token_array[3], ptr::null_mut(), 0);
        runs += 1;
    }

    ret = show_bw_info(bw_imc.as_mut_ptr(), bw_resc.as_mut_ptr(), span);

    fclose(fp);

    ret
}

unsafe extern "C" fn mbm_init(
    _test: *const resctrl_test,
    _uparams: *const user_params,
    param: *const resctrl_val_param,
    domain_id: c_int,
) -> c_int {
    let ret: c_int;

    ret = initialize_read_mem_bw_imc();
    if ret != 0 {
        return ret;
    }

    initialize_mem_bw_resctrl(param, domain_id);

    0
}

unsafe extern "C" fn mbm_setup(
    test: *const resctrl_test,
    uparams: *const user_params,
    p: *mut resctrl_val_param,
) -> c_int {
    let mut ret: c_int = 0;

    /* Run NUM_OF_RUNS times */
    if (*p).num_of_runs >= NUM_OF_RUNS as c_int {
        return END_OF_TESTS;
    }

    /* Set up shemata with 100% allocation on the first run. */
    if (*p).num_of_runs == 0 && resctrl_resource_exists(b"MB\0".as_ptr() as *const c_char) {
        ret = write_schemata(
            (*p).ctrlgrp,
            b"100\0".as_ptr() as *const c_char,
            (*uparams).cpu,
            (*test).resource,
        );
    }

    (*p).num_of_runs += 1;

    ret
}

unsafe extern "C" fn mbm_measure(
    uparams: *const user_params,
    param: *mut resctrl_val_param,
    bm_pid: pid_t,
) -> c_int {
    measure_read_mem_bw(uparams, param, bm_pid)
}

unsafe extern "C" fn mbm_test_cleanup() {
    remove(RESULT_FILE_NAME.as_ptr() as *const c_char);
}

unsafe extern "C" fn mbm_run_test(
    test: *const resctrl_test,
    uparams: *const user_params,
) -> c_int {
    let mut param = resctrl_val_param {
        ctrlgrp: b"c1\0".as_ptr() as *const c_char,
        filename: RESULT_FILE_NAME.as_ptr() as *const c_char,
        init: Some(mbm_init),
        setup: Some(mbm_setup),
        measure: Some(mbm_measure),
        fill_buf: ptr::null_mut(),
        num_of_runs: 0,
    };
    let mut fill_buf: fill_buf_param = MaybeUninit::zeroed().assume_init();
    let mut ret: c_int;

    remove(RESULT_FILE_NAME.as_ptr() as *const c_char);

    if !(*uparams).fill_buf.is_null() {
        fill_buf.buf_size = (*(*uparams).fill_buf).buf_size;
        fill_buf.memflush = (*(*uparams).fill_buf).memflush;
        param.fill_buf = &mut fill_buf;
    } else if (*uparams).benchmark_cmd[0] == 0 {
        let buf_size: isize;

        buf_size = get_fill_buf_size((*uparams).cpu, b"L3\0".as_ptr() as *const c_char);
        if buf_size < 0 {
            return buf_size as c_int;
        }
        fill_buf.buf_size = buf_size as usize;
        fill_buf.memflush = true;
        param.fill_buf = &mut fill_buf;
    }

    ret = resctrl_val(test, uparams, &mut param);
    if ret != 0 {
        return ret;
    }

    ret = check_results(if !param.fill_buf.is_null() {
        (*param.fill_buf).buf_size
    } else {
        0
    });
    if ret != 0 && get_vendor() == ARCH_INTEL && !snc_kernel_support() {
        ksft_print_msg(b"Kernel doesn't support Sub-NUMA Clustering but it is enabled on the system.\n\0".as_ptr() as *const c_char);
    }

    ret
}

unsafe extern "C" fn mbm_feature_check(_test: *const resctrl_test) -> bool {
    resctrl_mon_feature_exists(
        b"L3_MON\0".as_ptr() as *const c_char,
        b"mbm_total_bytes\0".as_ptr() as *const c_char,
    ) && resctrl_mon_feature_exists(
        b"L3_MON\0".as_ptr() as *const c_char,
        b"mbm_local_bytes\0".as_ptr() as *const c_char,
    )
}

#[no_mangle]
pub static mut mbm_test: resctrl_test = resctrl_test {
    name: b"MBM\0".as_ptr() as *const c_char,
    resource: b"MB\0".as_ptr() as *const c_char,
    vendor_specific: ARCH_INTEL,
    feature_check: Some(mbm_feature_check),
    run_test: Some(mbm_run_test),
    cleanup: Some(mbm_test_cleanup),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
