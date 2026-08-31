// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

use core::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_void};

// C dependencies:
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

type u64 = u64;

/* All successful D-side store dispatches for this thread that were L2 Miss */
const EventCode: u64 = 0x46880;

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
    pub sample_regs_intr: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

extern "C" {
    static platform_extended_mask: u64;

    fn thirty_two_instruction_loop_with_ll_sc(loops: u64, ll_sc_target: *mut u64);

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn get_mmcr1_comb(value: u64, width: c_int) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * EV_CODE_EXTRACT(event.attr.config, comb) is supplied by the C dependencies.
 * Keep the operation as an external Rust-form dependency for this translation.
 */
extern "C" {
    fn EV_CODE_EXTRACT_comb(value: u64) -> u64;
}

/*
 * A perf sampling test for mmcr1
 * fields : comb.
 */
unsafe extern "C" fn mmcr1_comb() -> c_int {
    let mut event_uninit = MaybeUninit::<event>::uninit();
    let mut intr_regs: *mut u64;
    let mut dummy: u64 = 0;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests());

    /* Init the event for the sampling test */
    event_init_sampling(event_uninit.as_mut_ptr(), EventCode);
    let mut event = event_uninit.assume_init();
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop_with_ll_sc(10000000, &mut dummy);

    FAIL_IF!(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF!((collect_samples(event.mmap_buffer) == 0) as c_int);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null() as c_int);

    /*
     * Verify that comb field match with
     * corresponding event code fields
     */
    FAIL_IF!(
        (EV_CODE_EXTRACT_comb(event.attr.config)
            != get_mmcr1_comb(get_reg_value(intr_regs, c"MMCR1".as_ptr()), 4)) as c_int
    );

    event_close(&mut event);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    test_harness(mmcr1_comb, c"mmcr1_comb".as_ptr())
}
