// SPDX-License-Identifier: GPL-2.0
/*
 * Cache Allocation Technology (CAT) test
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
/* Translated from C. External declarations are supplied by resctrl.h/libc. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type __u64 = u64;
type __s64 = i64;
type size_t = usize;
type pid_t = c_int;

const RESULT_FILE_NAME: &[u8] = b"result_cat\0";
const NUM_OF_RUNS: c_int = 5;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub group: *const c_char,
    pub resource: *const c_char,
    pub feature_check: Option<unsafe extern "C" fn(*const resctrl_test) -> bool>,
    pub run_test: Option<unsafe extern "C" fn(*const resctrl_test, *const user_params) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct user_params {
    pub cpu: c_int,
    pub bits: c_int,
}

#[repr(C)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub mongrp: *const c_char,
    pub filename: *const c_char,
    pub num_of_runs: c_int,
    pub mask: c_ulong,
}

unsafe extern "C" {
    static ARCH_INTEL: c_uint;
    static ARCH_AMD: c_uint;
    static ARCH_HYGON: c_uint;
    static PERF_COUNT_HW_CACHE_MISSES: c_uint;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn show_cache_info(no_of_bits: c_int, avg_llc_val: __u64, cache_span: c_ulong, platform: bool);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn remove(pathname: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn getpid() -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);

    fn cache_portion_size(cache_total_size: c_ulong, mask: c_ulong, full_cache_mask: c_ulong) -> c_ulong;
    fn count_bits(mask: c_ulong) -> c_int;
    fn get_vendor() -> c_uint;
    fn taskset_benchmark(bm_pid: pid_t, cpu_no: c_int, old_affinity: *mut cpu_set_t) -> c_int;
    fn write_bm_pid_to_resctrl(bm_pid: pid_t, ctrlgrp: *const c_char, mongrp: *const c_char) -> c_int;
    fn minimize_l2_occupancy(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *mut resctrl_val_param,
    ) -> c_int;
    fn perf_event_attr_initialize(pea: *mut perf_event_attr, config: c_uint);
    fn perf_open(pea: *mut perf_event_attr, pid: pid_t, cpu: c_int) -> c_int;
    fn alloc_buffer(span: size_t, memflush: c_int) -> *mut u8;
    fn write_schemata(ctrlgrp: *const c_char, schemata: *const c_char, cpu_no: c_int, resource: *const c_char)
        -> c_int;
    fn mem_flush(buf: *mut u8, span: size_t);
    fn fill_cache_read(buf: *mut u8, span: size_t, once: bool);
    fn perf_event_reset_enable(pe_fd: c_int) -> c_int;
    fn perf_event_measure(pe_fd: c_int, filename: *const c_char, bm_pid: pid_t) -> c_int;
    fn taskset_restore(bm_pid: pid_t, old_affinity: *mut cpu_set_t);
    fn get_full_cbm(resource: *const c_char, mask: *mut c_ulong) -> c_int;
    fn get_mask_no_shareable(resource: *const c_char, mask: *mut c_ulong) -> c_int;
    fn get_cache_size(cpu_no: c_int, resource: *const c_char, cache_size: *mut c_ulong) -> c_int;
    fn count_contiguous_bits(mask: c_ulong, start: *mut c_uint) -> c_int;
    fn create_bit_mask(start: c_uint, len: c_int) -> c_ulong;
    fn resource_info_unsigned_get(resource: *const c_char, filename: *const c_char, val: *mut c_uint) -> c_int;
    fn resctrl_resource_exists(resource: *const c_char) -> bool;
    fn resource_info_file_exists(resource: *const c_char, file: *const c_char) -> bool;
    fn test_resource_feature_check(test: *const resctrl_test) -> bool;
}

unsafe fn show_results_info(
    sum_llc_val: __u64,
    no_of_bits: c_int,
    cache_span: c_ulong,
    num_of_runs: c_ulong,
    platform: bool,
    prev_avg_llc_val: *mut __s64,
) -> c_int {
    let mut ret: c_int = 0;
    let avg_llc_val: __u64 = sum_llc_val / num_of_runs as __u64;

    if *prev_avg_llc_val != 0 {
        ret = (platform && avg_llc_val < *prev_avg_llc_val as __u64) as c_int;

        ksft_print_msg(
            b"%s Check cache miss rate increased\n\0".as_ptr() as *const c_char,
            if ret != 0 {
                b"Fail:\0".as_ptr()
            } else {
                b"Pass:\0".as_ptr()
            } as *const c_char,
        );
    }
    *prev_avg_llc_val = avg_llc_val as __s64;

    show_cache_info(no_of_bits, avg_llc_val, cache_span, true);

    ret
}

/* Remove the highest bits from CBM */
fn next_mask(current_mask: c_ulong) -> c_ulong {
    current_mask & (current_mask >> 2)
}

unsafe fn check_results(
    param: *mut resctrl_val_param,
    _cache_type: *const c_char,
    cache_total_size: c_ulong,
    full_cache_mask: c_ulong,
    mut current_mask: c_ulong,
) -> c_int {
    let mut token_array: [*mut c_char; 8] = [ptr::null_mut(); 8];
    let mut temp: [c_char; 512] = [0; 512];
    let mut sum_llc_perf_miss: __u64 = 0;
    let mut prev_avg_llc_val: __s64 = 0;
    let mut runs: c_int = 0;
    let mut fail: c_int = 0;

    ksft_print_msg(b"Checking for pass/fail\n\0".as_ptr() as *const c_char);
    let fp = fopen((*param).filename, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        ksft_perror(b"Cannot open file\0".as_ptr() as *const c_char);

        return -1;
    }

    while !fgets(temp.as_mut_ptr(), temp.len() as c_int, fp).is_null() {
        let mut token = strtok(temp.as_mut_ptr(), b":\t\0".as_ptr() as *const c_char);
        let mut fields: c_int = 0;

        while !token.is_null() {
            token_array[fields as usize] = token;
            fields += 1;
            token = strtok(ptr::null_mut(), b":\t\0".as_ptr() as *const c_char);
        }

        sum_llc_perf_miss += strtoull(token_array[3], ptr::null_mut(), 0);
        runs += 1;

        if runs < NUM_OF_RUNS {
            continue;
        }

        if current_mask == 0 {
            ksft_print_msg(b"Unexpected empty cache mask\n\0".as_ptr() as *const c_char);
            break;
        }

        let alloc_size = cache_portion_size(cache_total_size, current_mask, full_cache_mask);

        let bits = count_bits(current_mask);

        let ret = show_results_info(
            sum_llc_perf_miss,
            bits,
            alloc_size / 64,
            runs as c_ulong,
            get_vendor() == ARCH_INTEL,
            &mut prev_avg_llc_val,
        );
        if ret != 0 {
            fail = 1;
        }

        runs = 0;
        sum_llc_perf_miss = 0;
        current_mask = next_mask(current_mask);
    }

    fclose(fp);

    fail
}

unsafe extern "C" fn cat_test_cleanup() {
    remove(RESULT_FILE_NAME.as_ptr() as *const c_char);
}

/*
 * cat_test - Execute CAT benchmark and measure cache misses
 * @test:		Test information structure
 * @uparams:		User supplied parameters
 * @param:		Parameters passed to cat_test()
 * @span:		Buffer size for the benchmark
 * @current_mask	Start mask for the first iteration
 *
 * Run CAT selftest by varying the allocated cache portion and comparing the
 * impact on cache misses (the result analysis is done in check_results()
 * and show_results_info(), not in this function).
 *
 * One bit is removed from the CAT allocation bit mask (in current_mask) for
 * each subsequent test which keeps reducing the size of the allocated cache
 * portion. A single test flushes the buffer, reads it to warm up the cache,
 * and reads the buffer again. The cache misses are measured during the last
 * read pass.
 *
 * Return:		0 when the test was run, < 0 on error.
 */
unsafe fn cat_test(
    test: *const resctrl_test,
    uparams: *const user_params,
    param: *mut resctrl_val_param,
    span: size_t,
    mut current_mask: c_ulong,
) -> c_int {
    let mut pea = MaybeUninit::<perf_event_attr>::uninit();
    let mut old_affinity = MaybeUninit::<cpu_set_t>::uninit();
    let mut schemata: [c_char; 64] = [0; 64];
    let mut ret: c_int;

    if strcmp((*param).filename, b"\0".as_ptr() as *const c_char) == 0 {
        sprintf((*param).filename as *mut c_char, b"stdio\0".as_ptr() as *const c_char);
    }

    let bm_pid = getpid();

    /* Taskset benchmark to specified cpu */
    ret = taskset_benchmark(bm_pid, (*uparams).cpu, old_affinity.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    /* Write benchmark to specified con_mon grp, mon_grp in resctrl FS*/
    ret = write_bm_pid_to_resctrl(bm_pid, (*param).ctrlgrp, (*param).mongrp);
    if ret != 0 {
        taskset_restore(bm_pid, old_affinity.as_mut_ptr());
        return ret;
    }

    ret = minimize_l2_occupancy(test, uparams, param);
    if ret != 0 {
        taskset_restore(bm_pid, old_affinity.as_mut_ptr());
        return ret;
    }

    perf_event_attr_initialize(pea.as_mut_ptr(), PERF_COUNT_HW_CACHE_MISSES);
    let pe_fd = perf_open(pea.as_mut_ptr(), bm_pid, (*uparams).cpu);
    if pe_fd < 0 {
        ret = -1;
        taskset_restore(bm_pid, old_affinity.as_mut_ptr());
        return ret;
    }

    let buf = alloc_buffer(span, 1);
    if buf.is_null() {
        ret = -1;
        close(pe_fd);
        taskset_restore(bm_pid, old_affinity.as_mut_ptr());
        return ret;
    }

    while current_mask != 0 {
        snprintf(
            schemata.as_mut_ptr(),
            schemata.len(),
            b"%lx\0".as_ptr() as *const c_char,
            (*param).mask & !current_mask,
        );
        ret = write_schemata(
            b"\0".as_ptr() as *const c_char,
            schemata.as_ptr(),
            (*uparams).cpu,
            (*test).resource,
        );
        if ret != 0 {
            break;
        }
        snprintf(
            schemata.as_mut_ptr(),
            schemata.len(),
            b"%lx\0".as_ptr() as *const c_char,
            current_mask,
        );
        ret = write_schemata((*param).ctrlgrp, schemata.as_ptr(), (*uparams).cpu, (*test).resource);
        if ret != 0 {
            break;
        }

        let mut i: c_int = 0;
        while i < NUM_OF_RUNS {
            mem_flush(buf, span);
            fill_cache_read(buf, span, true);

            ret = perf_event_reset_enable(pe_fd);
            if ret != 0 {
                break;
            }

            fill_cache_read(buf, span, true);

            ret = perf_event_measure(pe_fd, (*param).filename, bm_pid);
            if ret != 0 {
                break;
            }
            i += 1;
        }
        if ret != 0 {
            break;
        }
        current_mask = next_mask(current_mask);
    }

    free(buf as *mut c_void);
    close(pe_fd);
    taskset_restore(bm_pid, old_affinity.as_mut_ptr());

    ret
}

unsafe extern "C" fn cat_run_test(test: *const resctrl_test, uparams: *const user_params) -> c_int {
    let mut long_mask: c_ulong = 0;
    let mut start_mask: c_ulong;
    let mut full_cache_mask: c_ulong = 0;
    let mut cache_total_size: c_ulong = 0;
    let mut n = (*uparams).bits;
    let mut start: c_uint = 0;
    let mut ret: c_int;

    ret = get_full_cbm((*test).resource, &mut full_cache_mask);
    if ret != 0 {
        return ret;
    }
    /* Get the largest contiguous exclusive portion of the cache */
    ret = get_mask_no_shareable((*test).resource, &mut long_mask);
    if ret != 0 {
        return ret;
    }

    /* Get L3/L2 cache size */
    ret = get_cache_size((*uparams).cpu, (*test).resource, &mut cache_total_size);
    if ret != 0 {
        return ret;
    }
    ksft_print_msg(
        b"Cache size :%lu\n\0".as_ptr() as *const c_char,
        cache_total_size,
    );

    let count_of_bits = count_contiguous_bits(long_mask, &mut start);

    if n == 0 {
        n = count_of_bits / 2;
    }

    if n > count_of_bits - 1 {
        ksft_print_msg(b"Invalid input value for no_of_bits n!\n\0".as_ptr() as *const c_char);
        ksft_print_msg(
            b"Please enter value in range 1 to %d\n\0".as_ptr() as *const c_char,
            count_of_bits - 1,
        );
        return -1;
    }
    start_mask = create_bit_mask(start, n);

    let mut param = resctrl_val_param {
        ctrlgrp: b"c1\0".as_ptr() as *const c_char,
        mongrp: ptr::null(),
        filename: RESULT_FILE_NAME.as_ptr() as *const c_char,
        num_of_runs: 0,
        mask: 0,
    };
    param.mask = long_mask;
    let span = cache_portion_size(cache_total_size, start_mask, full_cache_mask);

    remove(param.filename);

    ret = cat_test(test, uparams, &mut param, span, start_mask);
    if ret != 0 {
        return ret;
    }

    ret = check_results(
        &mut param,
        (*test).resource,
        cache_total_size,
        full_cache_mask,
        start_mask,
    );
    ret
}

unsafe fn arch_supports_noncont_cat(test: *const resctrl_test) -> bool {
    let vendor_id = get_vendor();

    /* AMD and Hygon always support non-contiguous CBM. */
    if vendor_id == ARCH_AMD || vendor_id == ARCH_HYGON {
        return true;
    }

    /* Intel support for non-contiguous CBM needs to be discovered. */
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::__cpuid_count;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::__cpuid_count;

        let r = if strcmp((*test).resource, b"L3\0".as_ptr() as *const c_char) == 0 {
            __cpuid_count(0x10, 1)
        } else if strcmp((*test).resource, b"L2\0".as_ptr() as *const c_char) == 0 {
            __cpuid_count(0x10, 2)
        } else {
            return false;
        };

        return ((r.ecx >> 3) & 1) != 0;
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        return false;
    }
}

unsafe extern "C" fn noncont_cat_run_test(
    test: *const resctrl_test,
    uparams: *const user_params,
) -> c_int {
    let mut full_cache_mask: c_ulong = 0;
    let mut sparse_masks: c_uint = 0;
    let mut schemata: [c_char; 64] = [0; 64];

    /* Check to compare sparse_masks content to CPUID output. */
    let mut ret = resource_info_unsigned_get(
        (*test).resource,
        b"sparse_masks\0".as_ptr() as *const c_char,
        &mut sparse_masks,
    );
    if ret != 0 {
        return ret;
    }

    if arch_supports_noncont_cat(test) != (sparse_masks != 0) {
        ksft_print_msg(
            b"Hardware and kernel differ on non-contiguous CBM support!\n\0".as_ptr() as *const c_char,
        );
        return 1;
    }

    /* Write checks initialization. */
    ret = get_full_cbm((*test).resource, &mut full_cache_mask);
    if ret < 0 {
        return ret;
    }
    let bit_center = count_bits(full_cache_mask) / 2;

    /*
     * The bit_center needs to be at least 3 to properly calculate the CBM
     * hole in the noncont_mask. If it's smaller return an error since the
     * cache mask is too short and that shouldn't happen.
     */
    if bit_center < 3 {
        return -EINVAL;
    }
    let cont_mask = full_cache_mask >> bit_center;

    /* Contiguous mask write check. */
    snprintf(
        schemata.as_mut_ptr(),
        schemata.len(),
        b"%lx\0".as_ptr() as *const c_char,
        cont_mask,
    );
    ret = write_schemata(
        b"\0".as_ptr() as *const c_char,
        schemata.as_ptr(),
        (*uparams).cpu,
        (*test).resource,
    );
    if ret != 0 {
        ksft_print_msg(b"Write of contiguous CBM failed\n\0".as_ptr() as *const c_char);
        return 1;
    }

    /*
     * Non-contiguous mask write check. CBM has a 0xf hole approximately in the middle.
     * Output is compared with support information to catch any edge case errors.
     */
    let noncont_mask = !(0xf_u64 << (bit_center - 2)) as c_ulong & full_cache_mask;
    snprintf(
        schemata.as_mut_ptr(),
        schemata.len(),
        b"%lx\0".as_ptr() as *const c_char,
        noncont_mask,
    );
    ret = write_schemata(
        b"\0".as_ptr() as *const c_char,
        schemata.as_ptr(),
        (*uparams).cpu,
        (*test).resource,
    );
    if ret != 0 && sparse_masks != 0 {
        ksft_print_msg(
            b"Non-contiguous CBMs supported but write of non-contiguous CBM failed\n\0".as_ptr()
                as *const c_char,
        );
    } else if ret != 0 && sparse_masks == 0 {
        ksft_print_msg(
            b"Non-contiguous CBMs not supported and write of non-contiguous CBM failed as expected\n\0"
                .as_ptr() as *const c_char,
        );
    } else if ret == 0 && sparse_masks == 0 {
        ksft_print_msg(
            b"Non-contiguous CBMs not supported but write of non-contiguous CBM succeeded\n\0"
                .as_ptr() as *const c_char,
        );
    }

    ((ret == 0) == (sparse_masks == 0)) as c_int
}

unsafe extern "C" fn noncont_cat_feature_check(test: *const resctrl_test) -> bool {
    if !resctrl_resource_exists((*test).resource) {
        return false;
    }

    resource_info_file_exists((*test).resource, b"sparse_masks\0".as_ptr() as *const c_char)
}

#[unsafe(no_mangle)]
pub static mut l3_cat_test: resctrl_test = resctrl_test {
    name: b"L3_CAT\0".as_ptr() as *const c_char,
    group: b"CAT\0".as_ptr() as *const c_char,
    resource: b"L3\0".as_ptr() as *const c_char,
    feature_check: Some(test_resource_feature_check),
    run_test: Some(cat_run_test),
    cleanup: Some(cat_test_cleanup),
};

#[unsafe(no_mangle)]
pub static mut l3_noncont_cat_test: resctrl_test = resctrl_test {
    name: b"L3_NONCONT_CAT\0".as_ptr() as *const c_char,
    group: b"CAT\0".as_ptr() as *const c_char,
    resource: b"L3\0".as_ptr() as *const c_char,
    feature_check: Some(noncont_cat_feature_check),
    run_test: Some(noncont_cat_run_test),
    cleanup: None,
};

#[unsafe(no_mangle)]
pub static mut l2_noncont_cat_test: resctrl_test = resctrl_test {
    name: b"L2_NONCONT_CAT\0".as_ptr() as *const c_char,
    group: b"CAT\0".as_ptr() as *const c_char,
    resource: b"L2\0".as_ptr() as *const c_char,
    feature_check: Some(noncont_cat_feature_check),
    run_test: Some(noncont_cat_run_test),
    cleanup: None,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
