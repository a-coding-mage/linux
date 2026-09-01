// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Madhavan Srinivasan, IBM Corp.
 */

// C dependencies translated from:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;

/* All successful D-side store dispatches for this thread */
const EventCode: u64 = 0x010000046080;

const MALLOC_SIZE: usize = 0x10000 * 10; /* Ought to be enough .. */

extern "C" {
    static platform_extended_mask: u64;

    static PPC_FEATURE2_ARCH_3_1: c_ulong;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn check_pvr_for_sampling_tests() -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;

    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcr2_l2l3(value: u64, width: c_int) -> u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
struct perf_event_attr {
    config: u64,
    sample_regs_intr: u64,
}

#[repr(C)]
struct event {
    attr: perf_event_attr,
    fd: c_int,
    mmap_buffer: *mut c_void,
}

/* Direct Rust equivalent of the local macro use: EV_CODE_EXTRACT(config, l2l3). */
const fn EV_CODE_EXTRACT_l2l3(config: u64) -> u64 {
    config & 0xf
}

/*
 * A perf sampling test for mmcr2
 * fields : l2l3
 */
unsafe extern "C" fn mmcr2_l2l3() -> c_int {
    let mut event = MaybeUninit::<event>::uninit();
    let mut intr_regs: *mut u64;
    let mut p: *mut c_char;
    let mut i: c_int;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests());
    SKIP_IF!(have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0);

    /* Init the event for the sampling test */
    event_init_sampling(event.as_mut_ptr(), EventCode);
    let mut event = event.assume_init();
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    p = malloc(MALLOC_SIZE) as *mut c_char;
    FAIL_IF!(p.is_null());

    i = 0;
    while i < MALLOC_SIZE as c_int {
        *p.add(i as usize) = i as c_char;
        i += 0x10000;
    }

    FAIL_IF!(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF!(collect_samples(event.mmap_buffer) == 0);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /*
     * Verify that l2l3 field of MMCR2 match with
     * corresponding event code field
     */
    FAIL_IF!(
        EV_CODE_EXTRACT_l2l3(event.attr.config)
            != get_mmcr2_l2l3(get_reg_value(intr_regs, b"MMCR2\0".as_ptr() as *const c_char), 4)
    );

    event_close(&mut event);
    free(p as *mut c_void);

    0
}

pub unsafe fn main() -> c_int {
    test_harness(mmcr2_l2l3, b"mmcr2_l2l3\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
