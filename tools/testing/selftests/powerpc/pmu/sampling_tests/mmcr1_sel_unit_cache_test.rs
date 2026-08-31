// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int, c_void};

type u64 = u64;

const MALLOC_SIZE: usize = 0x10000 * 10; /* Ought to be enough .. */

/* The data cache was reloaded from local core's L3 due to a demand load */
const EventCode: u64 = 0x21c040;

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
    pub sample_period: u64,
    pub sample_regs_intr: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

#[repr(C)]
pub enum ev_code_field {
    pmcxsel,
    unit,
    cache,
}

unsafe extern "C" {
    static platform_extended_mask: u64;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event);
    fn event_disable(event: *mut event);
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn get_mmcr1_pmcxsel(value: u64, idx: c_int) -> u64;
    fn get_mmcr1_unit(value: u64, idx: c_int) -> u64;
    fn get_mmcr1_cache(value: u64, idx: c_int) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn EV_CODE_EXTRACT(config: u64, field: ev_code_field) -> u64;
    fn FAIL_IF(condition: c_int);
    fn SKIP_IF(condition: c_int);
}

/*
 * A perf sampling test for mmcr1
 * fields : pmcxsel, unit, cache.
 */
unsafe extern "C" fn mmcr1_sel_unit_cache() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;
    let p: *mut c_char;
    let mut i: c_int;

    /* Check for platform support for the test */
    SKIP_IF(check_pvr_for_sampling_tests());

    p = malloc(MALLOC_SIZE) as *mut c_char;
    FAIL_IF((p.is_null()) as c_int);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_period = 1;
    FAIL_IF(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    event_enable(&mut event);

    /* workload to make the event overflow */
    i = 0;
    while i < MALLOC_SIZE as c_int {
        *p.offset(i as isize) = i as c_char;
        i += 0x10000;
    }

    event_disable(&mut event);

    /* Check for sample count */
    FAIL_IF((collect_samples(event.mmap_buffer) == 0) as c_int);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF((intr_regs.is_null()) as c_int);

    /*
     * Verify that  pmcxsel, unit and cache field of MMCR1
     * match with corresponding event code fields
     */
    FAIL_IF(
        (EV_CODE_EXTRACT(event.attr.config, ev_code_field::pmcxsel)
            != get_mmcr1_pmcxsel(get_reg_value(intr_regs, c"MMCR1".as_ptr()), 1)) as c_int,
    );
    FAIL_IF(
        (EV_CODE_EXTRACT(event.attr.config, ev_code_field::unit)
            != get_mmcr1_unit(get_reg_value(intr_regs, c"MMCR1".as_ptr()), 1)) as c_int,
    );
    FAIL_IF(
        (EV_CODE_EXTRACT(event.attr.config, ev_code_field::cache)
            != get_mmcr1_cache(get_reg_value(intr_regs, c"MMCR1".as_ptr()), 1)) as c_int,
    );

    free(p as *mut c_void);
    event_close(&mut event);
    return 0;
}

fn main() {
    unsafe {
        FAIL_IF(test_harness(
            mmcr1_sel_unit_cache,
            c"mmcr1_sel_unit_cache".as_ptr(),
        ));
    }
}
