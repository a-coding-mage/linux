// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, "../event.h", "misc.h", "utils.h".

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

const PPC_FEATURE2_ARCH_3_1: c_ulonglong = 0x0004_0000;

#[repr(C)]
pub struct event_attr {
    pub sample_regs_intr: c_ulonglong,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

extern "C" {
    static platform_extended_mask: c_ulonglong;

    fn thirty_two_instruction_loop(loops: c_int);

    fn check_pvr_for_sampling_tests() -> c_int;
    fn have_hwcap2(feature: c_ulonglong) -> c_int;

    fn event_init_sampling(event: *mut event, config: c_ulonglong);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcr0_cc56run(value: u64, bit: c_int) -> c_int;
    fn get_reg_value(intr_regs: *mut u64, reg: *const c_char) -> u64;
    fn event_close(event: *mut event);

    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe extern "C" fn mmcr0_cc56run() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;

    /* Check for platform support for the test */
    if check_pvr_for_sampling_tests() != 0 {
        return 4;
    }
    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0 {
        return 4;
    }

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, 0x500fa);
    event.attr.sample_regs_intr = platform_extended_mask;
    if event_open(&mut event) != 0 {
        return 1;
    }
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    if event_enable(&mut event) != 0 {
        return 1;
    }

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    if event_disable(&mut event) != 0 {
        return 1;
    }

    /* Check for sample count */
    if collect_samples(event.mmap_buffer) == 0 {
        return 1;
    }

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    if intr_regs.is_null() {
        return 1;
    }

    /* Verify that cc56run bit is set in MMCR0 */
    if get_mmcr0_cc56run(get_reg_value(intr_regs, b"MMCR0\0".as_ptr() as *const c_char), 5) == 0 {
        return 1;
    }

    event_close(&mut event);
    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            mmcr0_cc56run,
            b"mmcr0_cc56run\0".as_ptr() as *const c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
