// SPDX-License-Identifier: GPL-2.0-only
/*
 * rodata_test.c: functional test for mark_rodata_ro function
 *
 * (C) Copyright 2008 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */
// #define pr_fmt(fmt) "rodata_test: " fmt
// External Linux kernel headers provide the referenced helpers and symbols.

const TEST_VALUE: i32 = 0xC3;
static rodata_test_data: i32 = TEST_VALUE;

extern "C" {
    static __start_rodata: u8;
    static __end_rodata: u8;

    fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void,
                               src: *const core::ffi::c_void,
                               size: usize) -> i32;
}

pub unsafe fn rodata_test() {
    let zero: i32 = 0;

    /* test 1: read the value */
    /* If this test fails, some previous testrun has clobbered the state */
    if unlikely(READ_ONCE!(rodata_test_data) != TEST_VALUE) {
        pr_err!("test 1 fails (start data)\n");
        return;
    }

    /* test 2: write to the variable; this should fault */
    if copy_to_kernel_nofault(
        (&rodata_test_data as *const i32).cast_mut().cast(),
        (&zero as *const i32).cast(),
        core::mem::size_of::<i32>(),
    ) == 0 {
        pr_err!("test data was not read only\n");
        return;
    }

    /* test 3: check the value hasn't changed */
    if unlikely(READ_ONCE!(rodata_test_data) != TEST_VALUE) {
        pr_err!("test data was changed\n");
        return;
    }

    /* test 4: check if the rodata section is PAGE_SIZE aligned */
    if !PAGE_ALIGNED!((&__start_rodata as *const u8).cast()) {
        pr_err!("start of .rodata is not page size aligned\n");
        return;
    }
    if !PAGE_ALIGNED!((&__end_rodata as *const u8).cast()) {
        pr_err!("end of .rodata is not page size aligned\n");
        return;
    }

    pr_info!("all tests were successful\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
