// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C includes translated as external dependencies:
// <stdio.h>, <sys/prctl.h>, <limits.h>, "../event.h", "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int};

const PM_DTLB_MISS_16G: c_int = 0x1c058;
const PM_DERAT_MISS_2M: c_int = 0x1c05a;
const PM_DTLB_MISS_2M: c_int = 0x1c05c;
const PM_MRK_DTLB_MISS_1G: c_int = 0x1d15c;
const PM_DTLB_MISS_4K: c_int = 0x2c056;
const PM_DERAT_MISS_1G: c_int = 0x2c05a;
const PM_MRK_DERAT_MISS_2M: c_int = 0x2d152;
const PM_MRK_DTLB_MISS_4K: c_int = 0x2d156;
const PM_MRK_DTLB_MISS_16G: c_int = 0x2d15e;
const PM_DTLB_MISS_64K: c_int = 0x3c056;
const PM_MRK_DERAT_MISS_1G: c_int = 0x3d152;
const PM_MRK_DTLB_MISS_64K: c_int = 0x3d156;
const PM_DISP_HELD_SYNC_HOLD: c_int = 0x4003c;
const PM_DTLB_MISS_16M: c_int = 0x4c056;
const PM_DTLB_MISS_1G: c_int = 0x4c05a;
const PM_MRK_DTLB_MISS_16M: c_int = 0x4c15e;
const PM_MRK_ST_DONE_L2: c_int = 0x10134;
const PM_RADIX_PWC_L1_HIT: c_int = 0x1f056;
const PM_FLOP_CMPL: c_int = 0x100f4;
const PM_MRK_NTF_FIN: c_int = 0x20112;
const PM_RADIX_PWC_L2_HIT: c_int = 0x2d024;
const PM_IFETCH_THROTTLE: c_int = 0x3405e;
const PM_MRK_L2_TM_ST_ABORT_SISTER: c_int = 0x3e15c;
const PM_RADIX_PWC_L3_HIT: c_int = 0x3f056;
const PM_RUN_CYC_SMT2_MODE: c_int = 0x3006c;
const PM_TM_TX_PASS_RUN_INST: c_int = 0x4e014;

const PVR_POWER9_CUMULUS: c_int = 0x00002000;

#[repr(C)]
pub struct event {
    _private: [u8; 0],
}

extern "C" {
    static mut pvr: c_int;
    static POWER9: c_int;
    static SPRN_PVR: c_int;

    fn mfspr(sprn: c_int) -> c_int;
    fn PVR_MIN(pvr: c_int) -> c_int;
    fn PVR_VER(pvr: c_int) -> c_int;
    fn platform_check_for_tests() -> c_int;
    fn check_for_generic_compat_pmu() -> c_int;
    fn event_init(event: *mut event, config: c_int);
    fn event_open(event: *mut event) -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
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

static mut blacklist_events_dd21: [c_int; 11] = [
    PM_MRK_ST_DONE_L2,
    PM_RADIX_PWC_L1_HIT,
    PM_FLOP_CMPL,
    PM_MRK_NTF_FIN,
    PM_RADIX_PWC_L2_HIT,
    PM_IFETCH_THROTTLE,
    PM_MRK_L2_TM_ST_ABORT_SISTER,
    PM_RADIX_PWC_L3_HIT,
    PM_RUN_CYC_SMT2_MODE,
    PM_TM_TX_PASS_RUN_INST,
    PM_DISP_HELD_SYNC_HOLD,
];

static mut blacklist_events_dd22: [c_int; 16] = [
    PM_DTLB_MISS_16G,
    PM_DERAT_MISS_2M,
    PM_DTLB_MISS_2M,
    PM_MRK_DTLB_MISS_1G,
    PM_DTLB_MISS_4K,
    PM_DERAT_MISS_1G,
    PM_MRK_DERAT_MISS_2M,
    PM_MRK_DTLB_MISS_4K,
    PM_MRK_DTLB_MISS_16G,
    PM_DTLB_MISS_64K,
    PM_MRK_DERAT_MISS_1G,
    PM_MRK_DTLB_MISS_64K,
    PM_DISP_HELD_SYNC_HOLD,
    PM_DTLB_MISS_16M,
    PM_DTLB_MISS_1G,
    PM_MRK_DTLB_MISS_16M,
];

static mut pvr_min: c_int = 0;

/*
 * check for power9 support for 2.1 and
 * 2.2 model where blacklist is applicable.
 */
#[no_mangle]
pub unsafe extern "C" fn check_for_power9_version() -> c_int {
    pvr_min = PVR_MIN(mfspr(SPRN_PVR));

    SKIP_IF!((PVR_VER(pvr) != POWER9) as c_int);
    SKIP_IF!((!(pvr & PVR_POWER9_CUMULUS)) as c_int);

    SKIP_IF!((!(3 - pvr_min)) as c_int);

    0
}

/*
 * Testcase to ensure that using blacklisted bits in
 * event code should cause event_open to fail in power9
 */

unsafe extern "C" fn blacklisted_events() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut i: c_int = 0;

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * check for power9 support for 2.1 and
     * 2.2 model where blacklist is applicable.
     */
    SKIP_IF!(check_for_power9_version());

    /* Skip for Generic compat mode */
    SKIP_IF!(check_for_generic_compat_pmu());

    if pvr_min == 1 {
        while i < blacklist_events_dd21.len() as c_int {
            event_init(&mut event, blacklist_events_dd21[i as usize]);
            FAIL_IF!((!event_open(&mut event)) as c_int);
            i += 1;
        }
    } else if pvr_min == 2 {
        while i < blacklist_events_dd22.len() as c_int {
            event_init(&mut event, blacklist_events_dd22[i as usize]);
            FAIL_IF!((!event_open(&mut event)) as c_int);
            i += 1;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    test_harness(blacklisted_events, c"blacklisted_events".as_ptr())
}
