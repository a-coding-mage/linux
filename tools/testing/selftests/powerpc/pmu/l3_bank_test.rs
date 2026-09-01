// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: stdio.h, stdlib.h, "event.h", "utils.h"

const MALLOC_SIZE: usize = 0x10000 * 10; /* Ought to be enough .. */

// External definitions supplied by the translated equivalents of event.h and utils.h.
// TODO: Replace these dependency declarations with the exact translated definitions
// when the surrounding selftest support code is available.
#[repr(C)]
pub struct event_result {
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct event {
    pub result: event_result,
}

extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);

    static PPC_FEATURE2_ARCH_2_07: u64;

    fn have_hwcap2(feature: u64) -> bool;
    fn event_init(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> i32;
    fn event_read(event: *mut event);
    fn event_report(event: *mut event);
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> i32, name: *const core::ffi::c_char) -> i32;
    fn skip_if(cond: bool);
    fn fail_if(cond: bool);
}

/*
 * Tests that the L3 bank handling is correct. We fixed it in commit e9aaac1.
 */
unsafe extern "C" fn l3_bank_test() -> i32 {
    let mut event: event = core::mem::zeroed();
    let p: *mut core::ffi::c_char;
    let mut i: i32;

    // The L3 bank logic is only used on Power8 or later
    skip_if(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

    p = malloc(MALLOC_SIZE) as *mut core::ffi::c_char;
    fail_if(p.is_null());

    event_init(&mut event, 0x84918F);

    fail_if(event_open(&mut event) != 0);

    i = 0;
    while i < MALLOC_SIZE as i32 {
        *p.offset(i as isize) = i as core::ffi::c_char;
        i += 0x10000;
    }

    event_read(&mut event);
    event_report(&mut event);

    fail_if(event.result.running == 0);
    fail_if(event.result.enabled == 0);

    event_close(&mut event);
    free(p as *mut core::ffi::c_void);

    0
}

pub unsafe fn main() -> i32 {
    test_harness(l3_bank_test, c"l3_bank_test".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
