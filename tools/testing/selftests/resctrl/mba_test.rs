// SPDX-License-Identifier: GPL-2.0
/*
 * Memory Bandwidth Allocation (MBA) test
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
/* Bindings expected from resctrl.h and the C runtime. */

use core::ffi::{c_char, c_float, c_int, c_long, c_ulong};
use core::ptr;

const RESULT_FILE_NAME: &[u8] = b"result_mba\0";
const NUM_OF_RUNS: c_int = 5;
const MAX_DIFF_PERCENT: c_int = 15;
const ALLOCATION_MAX: c_uint = 100;
const ALLOCATION_MIN: c_uint = 10;
const ALLOCATION_STEP: c_uint = 10;

type c_uint = u32;
type pid_t = c_int;
type ssize_t = isize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fill_buf_param {
    pub buf_size: ssize_t,
    pub memflush: bool,
}

#[repr(C)]
pub struct user_params {
    pub cpu: c_int,
    pub fill_buf: *mut fill_buf_param,
    pub benchmark_cmd: *mut c_char,
}

#[repr(C)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub filename: *const c_char,
    pub init: Option<
        unsafe extern "C" fn(
            *const resctrl_test,
            *const user_params,
            *const resctrl_val_param,
            c_int,
        ) -> c_int,
    >,
    pub setup:
        Option<unsafe extern "C" fn(*const resctrl_test, *const user_params, *mut resctrl_val_param) -> c_int>,
    pub measure:
        Option<unsafe extern "C" fn(*const user_params, *mut resctrl_val_param, pid_t) -> c_int>,
    pub fill_buf: *mut fill_buf_param,
}

#[repr(C)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub resource: *const c_char,
    pub vendor_specific: c_int,
    pub feature_check: Option<unsafe extern "C" fn(*const resctrl_test) -> bool>,
    pub run_test: Option<unsafe extern "C" fn(*const resctrl_test, *const user_params) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static THROTTLE_THRESHOLD: c_int;
    static ARCH_INTEL: c_int;
    static END_OF_TESTS: c_int;

    fn initialize_read_mem_bw_imc() -> c_int;
    fn initialize_mem_bw_resctrl(param: *const resctrl_val_param, domain_id: c_int);
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
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn labs(n: c_long) -> c_long;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn fclose(stream: *mut FILE) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn get_fill_buf_size(cpu: c_int, cache_type: *const c_char) -> ssize_t;
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

unsafe extern "C" fn mba_init(
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

/*
 * Change schemata percentage from 100 to 10%. Write schemata to specified
 * con_mon grp, mon_grp in resctrl FS.
 * For each allocation, run 5 times in order to get average values.
 */
unsafe extern "C" fn mba_setup(
    test: *const resctrl_test,
    uparams: *const user_params,
    p: *mut resctrl_val_param,
) -> c_int {
    static mut ALLOCATION: c_uint = ALLOCATION_MIN;
    static mut RUNS_PER_ALLOCATION: c_int = 0;
    let mut allocation_str: [c_char; 64] = [0; 64];
    let ret: c_int;

    if RUNS_PER_ALLOCATION >= NUM_OF_RUNS {
        RUNS_PER_ALLOCATION = 0;
    }

    /* Only set up schemata once every NUM_OF_RUNS of allocations */
    let old_runs_per_allocation = RUNS_PER_ALLOCATION;
    RUNS_PER_ALLOCATION += 1;
    if old_runs_per_allocation != 0 {
        return 0;
    }

    if ALLOCATION > ALLOCATION_MAX {
        return END_OF_TESTS;
    }

    sprintf(allocation_str.as_mut_ptr(), b"%d\0".as_ptr() as *const c_char, ALLOCATION);

    ret = write_schemata(
        (*p).ctrlgrp,
        allocation_str.as_ptr(),
        (*uparams).cpu,
        (*test).resource,
    );
    if ret < 0 {
        return ret;
    }

    ALLOCATION += ALLOCATION_STEP;

    0
}

unsafe extern "C" fn mba_measure(
    uparams: *const user_params,
    param: *mut resctrl_val_param,
    bm_pid: pid_t,
) -> c_int {
    measure_read_mem_bw(uparams, param, bm_pid)
}

unsafe extern "C" fn show_mba_info(bw_imc: *mut c_ulong, bw_resc: *mut c_ulong) -> bool {
    let mut allocation: c_uint;
    let mut ret: bool = false;
    let mut runs: c_int;

    ksft_print_msg(b"Results are displayed in (MB)\n\0".as_ptr() as *const c_char);
    /* Memory bandwidth from 100% down to 10% */
    allocation = 0;
    while allocation < ALLOCATION_MAX / ALLOCATION_STEP {
        let mut sum_bw_imc: c_ulong = 0;
        let mut sum_bw_resc: c_ulong = 0;
        let avg_bw_imc: c_long;
        let avg_bw_resc: c_long;
        let avg_diff_per: c_int;
        let avg_diff: c_float;

        runs = (NUM_OF_RUNS as c_uint * allocation) as c_int;
        while runs < (NUM_OF_RUNS as c_uint * allocation + NUM_OF_RUNS as c_uint) as c_int {
            sum_bw_imc += *bw_imc.offset(runs as isize);
            sum_bw_resc += *bw_resc.offset(runs as isize);
            runs += 1;
        }

        avg_bw_imc = (sum_bw_imc / NUM_OF_RUNS as c_ulong) as c_long;
        avg_bw_resc = (sum_bw_resc / NUM_OF_RUNS as c_ulong) as c_long;
        if avg_bw_imc < THROTTLE_THRESHOLD as c_long || avg_bw_resc < THROTTLE_THRESHOLD as c_long {
            ksft_print_msg(
                b"Bandwidth below threshold (%d MiB). Dropping results from MBA schemata %u.\n\0"
                    .as_ptr() as *const c_char,
                THROTTLE_THRESHOLD,
                ALLOCATION_MIN + ALLOCATION_STEP * allocation,
            );
            allocation += 1;
            continue;
        }

        avg_diff = labs(avg_bw_resc - avg_bw_imc) as c_float / avg_bw_imc as c_float;
        avg_diff_per = (avg_diff * 100.0) as c_int;

        ksft_print_msg(
            b"%s Check MBA diff within %d%% for schemata %u\n\0".as_ptr() as *const c_char,
            if avg_diff_per > MAX_DIFF_PERCENT {
                b"Fail:\0".as_ptr()
            } else {
                b"Pass:\0".as_ptr()
            } as *const c_char,
            MAX_DIFF_PERCENT,
            ALLOCATION_MIN + ALLOCATION_STEP * allocation,
        );

        ksft_print_msg(
            b"avg_diff_per: %d%%\n\0".as_ptr() as *const c_char,
            avg_diff_per,
        );
        ksft_print_msg(
            b"avg_bw_imc: %lu\n\0".as_ptr() as *const c_char,
            avg_bw_imc as c_ulong,
        );
        ksft_print_msg(
            b"avg_bw_resc: %lu\n\0".as_ptr() as *const c_char,
            avg_bw_resc as c_ulong,
        );
        if avg_diff_per > MAX_DIFF_PERCENT {
            ret = true;
        }

        allocation += 1;
    }

    ksft_print_msg(
        b"%s Check schemata change using MBA\n\0".as_ptr() as *const c_char,
        if ret {
            b"Fail:\0".as_ptr()
        } else {
            b"Pass:\0".as_ptr()
        } as *const c_char,
    );
    if ret {
        ksft_print_msg(b"At least one test failed\n\0".as_ptr() as *const c_char);
    }

    ret
}

unsafe extern "C" fn check_results() -> c_int {
    let mut bw_resc: [c_ulong; (NUM_OF_RUNS as c_uint * ALLOCATION_MAX / ALLOCATION_STEP) as usize] =
        [0; (NUM_OF_RUNS as c_uint * ALLOCATION_MAX / ALLOCATION_STEP) as usize];
    let mut bw_imc: [c_ulong; (NUM_OF_RUNS as c_uint * ALLOCATION_MAX / ALLOCATION_STEP) as usize] =
        [0; (NUM_OF_RUNS as c_uint * ALLOCATION_MAX / ALLOCATION_STEP) as usize];
    let mut token_array: [*mut c_char; 8] = [ptr::null_mut(); 8];
    let mut output: [c_char; RESULT_FILE_NAME.len()] = [0; RESULT_FILE_NAME.len()];
    let mut temp: [c_char; 512] = [0; 512];
    let mut runs: c_int;
    let fp: *mut FILE;

    ptr::copy_nonoverlapping(
        RESULT_FILE_NAME.as_ptr() as *const c_char,
        output.as_mut_ptr(),
        RESULT_FILE_NAME.len(),
    );

    fp = fopen(output.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(output.as_ptr());

        return -1;
    }

    runs = 0;
    while !fgets(temp.as_mut_ptr(), temp.len() as c_int, fp).is_null() {
        let mut token: *mut c_char = strtok(temp.as_mut_ptr(), b":\t\0".as_ptr() as *const c_char);
        let mut fields: c_int = 0;

        while !token.is_null() {
            token_array[fields as usize] = token;
            fields += 1;
            token = strtok(ptr::null_mut(), b":\t\0".as_ptr() as *const c_char);
        }

        /* Field 3 is perf imc value */
        bw_imc[runs as usize] = strtoul(token_array[3], ptr::null_mut(), 0);
        /* Field 5 is resctrl value */
        bw_resc[runs as usize] = strtoul(token_array[5], ptr::null_mut(), 0);
        runs += 1;
    }

    fclose(fp);

    show_mba_info(bw_imc.as_mut_ptr(), bw_resc.as_mut_ptr()) as c_int
}

unsafe extern "C" fn mba_test_cleanup() {
    remove(RESULT_FILE_NAME.as_ptr() as *const c_char);
}

unsafe extern "C" fn mba_run_test(test: *const resctrl_test, uparams: *const user_params) -> c_int {
    let mut param: resctrl_val_param = resctrl_val_param {
        ctrlgrp: b"c1\0".as_ptr() as *const c_char,
        filename: RESULT_FILE_NAME.as_ptr() as *const c_char,
        init: Some(mba_init),
        setup: Some(mba_setup),
        measure: Some(mba_measure),
        fill_buf: ptr::null_mut(),
    };
    let mut fill_buf: fill_buf_param = fill_buf_param {
        buf_size: 0,
        memflush: false,
    };
    let mut ret: c_int;

    remove(RESULT_FILE_NAME.as_ptr() as *const c_char);

    if !(*uparams).fill_buf.is_null() {
        fill_buf.buf_size = (*(*uparams).fill_buf).buf_size;
        fill_buf.memflush = (*(*uparams).fill_buf).memflush;
        param.fill_buf = &mut fill_buf;
    } else if *(*uparams).benchmark_cmd.offset(0) == 0 {
        let buf_size: ssize_t;

        buf_size = get_fill_buf_size((*uparams).cpu, b"L3\0".as_ptr() as *const c_char);
        if buf_size < 0 {
            return buf_size as c_int;
        }
        fill_buf.buf_size = buf_size;
        fill_buf.memflush = true;
        param.fill_buf = &mut fill_buf;
    }

    ret = resctrl_val(test, uparams, &mut param);
    if ret != 0 {
        return ret;
    }

    ret = check_results();
    if ret != 0 && get_vendor() == ARCH_INTEL && !snc_kernel_support() {
        ksft_print_msg(
            b"Kernel doesn't support Sub-NUMA Clustering but it is enabled on the system.\n\0"
                .as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn mba_feature_check(test: *const resctrl_test) -> bool {
    test_resource_feature_check(test)
        && resctrl_mon_feature_exists(
            b"L3_MON\0".as_ptr() as *const c_char,
            b"mbm_local_bytes\0".as_ptr() as *const c_char,
        )
}

#[no_mangle]
pub static mut mba_test: resctrl_test = resctrl_test {
    name: b"MBA\0".as_ptr() as *const c_char,
    resource: b"MB\0".as_ptr() as *const c_char,
    vendor_specific: 0, /* ARCH_INTEL */
    feature_check: Some(mba_feature_check),
    run_test: Some(mba_run_test),
    cleanup: Some(mba_test_cleanup),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
