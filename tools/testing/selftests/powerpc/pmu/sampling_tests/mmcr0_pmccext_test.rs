// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: ../event.h, misc.h, utils.h

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

type u64 = c_ulonglong;

#[repr(C)]
pub struct perf_event_attr {
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
    static PPC_FEATURE2_ARCH_3_1: u64;

    fn thirty_two_instruction_loop(loops: c_int);
    fn check_pvr_for_sampling_tests() -> c_int;
    fn have_hwcap2(feature: u64) -> c_int;
    fn event_init_sampling(event: *mut event, config: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcr0_pmccext(value: u64, width: c_int) -> c_int;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: Option<unsafe extern "C" fn() -> c_int>, name: *const c_char) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 1;
        }
    };
}

/*
 * A perf sampling test for mmcr0
 * field: pmccext
 */
unsafe extern "C" fn mmcr0_pmccext() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests());
    SKIP_IF!((have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0) as c_int);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, 0x4001e);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF!(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF!((collect_samples(event.mmap_buffer) == 0) as c_int);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null() as c_int);

    /* Verify that pmccext field is set in MMCR0 */
    FAIL_IF!(
        (get_mmcr0_pmccext(get_reg_value(intr_regs, b"MMCR0\0".as_ptr() as *const c_char), 4) == 0)
            as c_int
    );

    event_close(&mut event);
    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            Some(mmcr0_pmccext),
            b"mmcr0_pmccext\0".as_ptr() as *const c_char,
        ));
    }
}
