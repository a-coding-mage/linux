// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <sys/prctl.h>
// #include <limits.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulong};
use core::mem::MaybeUninit;

#[repr(C)]
pub struct event {
    // Opaque dependency from "../event.h"; the real layout is supplied externally.
    _private: [u8; 0],
}

extern "C" {
    static PPC_FEATURE2_ARCH_3_1: c_ulong;

    fn platform_check_for_tests() -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn event_init(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

// The data cache was reloaded from local core's L3 due to a demand load
const EventCode_1: u64 = 0x1340000001c040;
// PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L2
const EventCode_2: u64 = 0x14242;
// Event code with IFM, EBB, BHRB bits set in event code
const EventCode_3: u64 = 0xf00000000000001e;

unsafe fn skip_if(condition: c_int) -> Option<c_int> {
    // Rust translation of SKIP_IF(...) from "../sampling_tests/misc.h".
    // The exact skip return code is provided by that test dependency in C.
    if condition != 0 {
        Some(4)
    } else {
        None
    }
}

unsafe fn fail_if(condition: bool) -> Option<c_int> {
    // Rust translation of FAIL_IF(...) from "../sampling_tests/misc.h".
    if condition {
        Some(1)
    } else {
        None
    }
}

/*
 * Some of the bits in the event code is
 * reserved for specific platforms.
 * Event code bits 52-59 are reserved in power9,
 * whereas in ISA v3.1, these are used for programming
 * Monitor Mode Control Register 3 (MMCR3).
 * Bit 9 in event code is reserved in power9,
 * whereas it is used for programming "radix_scope_qual"
 * bit 18 in Monitor Mode Control Register 1 (MMCR1).
 *
 * Testcase to ensure that using reserved bits in
 * event code should cause event_open to fail.
 */

unsafe extern "C" fn invalid_event_code() -> c_int {
    let mut event = MaybeUninit::<event>::uninit();

    // Check for platform support for the test
    if let Some(ret) = skip_if(platform_check_for_tests()) {
        return ret;
    }

    /*
     * Events using MMCR3 bits and radix scope qual bits
     * should fail in power9 and should succeed in power10 ( ISA v3.1 )
     * Init the events and check for pass/fail in event open.
     */
    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) != 0 {
        event_init(event.as_mut_ptr(), EventCode_1);
        if let Some(ret) = fail_if(event_open(event.as_mut_ptr()) != 0) {
            return ret;
        }
        event_close(event.as_mut_ptr());

        event_init(event.as_mut_ptr(), EventCode_2);
        if let Some(ret) = fail_if(event_open(event.as_mut_ptr()) != 0) {
            return ret;
        }
        event_close(event.as_mut_ptr());
    } else {
        event_init(event.as_mut_ptr(), EventCode_1);
        if let Some(ret) = fail_if(!(event_open(event.as_mut_ptr()) != 0)) {
            return ret;
        }

        event_init(event.as_mut_ptr(), EventCode_2);
        if let Some(ret) = fail_if(!(event_open(event.as_mut_ptr()) != 0)) {
            return ret;
        }
    }

    0
}

pub unsafe extern "C" fn main() -> c_int {
    test_harness(
        Some(invalid_event_code),
        b"invalid_event_code\0".as_ptr() as *const c_char,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
