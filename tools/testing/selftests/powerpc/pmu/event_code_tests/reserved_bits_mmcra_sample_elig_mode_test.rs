// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

/*
 * Testcase for reserved bits in Monitor Mode Control
 * Register A (MMCRA) Random Sampling Mode (SM) value.
 * As per Instruction Set Architecture (ISA), the values
 * 0x5, 0x9, 0xD, 0x19, 0x1D, 0x1A, 0x1E are reserved
 * for sampling mode field. Test that having these reserved
 * bit values should cause event_open to fail.
 * Input event code uses these sampling bits along with
 * 401e0 (PM_MRK_INST_CMPL).
 */

extern "C" {
    fn PVR_VER(value: u64) -> i32;
    fn mfspr(spr: u64) -> u64;
    fn platform_check_for_tests() -> i32;
    fn check_for_generic_compat_pmu() -> i32;
    fn event_init(event: *mut event, config: u64);
    fn event_open(event: *mut event) -> i32;
    fn test_harness(test: extern "C" fn() -> i32, name: *const i8) -> i32;
    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

extern "C" {
    static SPRN_PVR: u64;
    static POWER9: i32;
    static POWER10: i32;
    static POWER11: i32;
}

// Type supplied by "../event.h".
#[repr(C)]
struct event {
    _data: [u8; 0],
}

extern "C" fn reserved_bits_mmcra_sample_elig_mode() -> i32 {
    let mut event: event = event { _data: [] };
    let pvr: i32 = unsafe { PVR_VER(mfspr(SPRN_PVR)) };

    /* Check for platform support for the test */
    unsafe {
        SKIP_IF(platform_check_for_tests() != 0);
    }

    /* Skip for Generic compat PMU */
    unsafe {
        SKIP_IF(check_for_generic_compat_pmu() != 0);
    }

    /*
     * MMCRA Random Sampling Mode (SM) values: 0x5
     * 0x9, 0xD, 0x19, 0x1D, 0x1A, 0x1E is reserved.
     * Expected to fail when using these reserved values.
     */
    unsafe {
        event_init(&mut event, 0x50401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0x90401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0xD0401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0x190401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0x1D0401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0x1A0401e0);
        FAIL_IF(event_open(&mut event) == 0);

        event_init(&mut event, 0x1E0401e0);
        FAIL_IF(event_open(&mut event) == 0);
    }

    /*
     * MMCRA Random Sampling Mode (SM) value 0x10
     * is reserved in power10/power11 and 0xC is reserved in
     * power9.
     */
    unsafe {
        if (pvr == POWER10) || (pvr == POWER11) {
            event_init(&mut event, 0x100401e0);
            FAIL_IF(event_open(&mut event) == 0);
        } else if PVR_VER(mfspr(SPRN_PVR)) == POWER9 {
            event_init(&mut event, 0xC0401e0);
            FAIL_IF(event_open(&mut event) == 0);
        }
    }

    return 0;
}

fn main() -> i32 {
    unsafe {
        return test_harness(
            reserved_bits_mmcra_sample_elig_mode,
            b"reserved_bits_mmcra_sample_elig_mode\0".as_ptr() as *const i8,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
