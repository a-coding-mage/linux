// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

extern "C" {
    fn platform_check_for_tests() -> ::std::os::raw::c_int;
    fn check_for_generic_compat_pmu() -> ::std::os::raw::c_int;
    fn event_init(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> ::std::os::raw::c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

/*
 * External type from ../event.h. Its layout is intentionally supplied by the
 * translated dependency, matching the original C file's reliance on that header.
 */
type event = crate::event;

/*
 * Testcase for reserved bits in Monitor Mode
 * Control Register A (MMCRA) thresh_ctl bits.
 * For MMCRA[48:51]/[52:55]) Threshold Start/Stop,
 * 0b11110000/0b00001111 is reserved.
 */
unsafe extern "C" fn reserved_bits_mmcra_thresh_ctl() -> ::std::os::raw::c_int {
    let mut event: event = ::std::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /* Skip for Generic compat PMU */
    SKIP_IF!(check_for_generic_compat_pmu());

    /*
     * MMCRA[48:51]/[52:55]) Threshold Start/Stop
     * events Selection. 0b11110000/0b00001111 is reserved.
     * Expected to fail when using these reserved values.
     */
    event_init(&mut event, 0xf0340401e0);
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x0f340401e0);
    FAIL_IF!(event_open(&mut event) == 0);

    return 0;
}

fn main() -> ::std::os::raw::c_int {
    unsafe {
        return test_harness(
            reserved_bits_mmcra_thresh_ctl,
            b"reserved_bits_mmcra_thresh_ctl\0".as_ptr() as *const ::std::os::raw::c_char,
        );
    }
}
