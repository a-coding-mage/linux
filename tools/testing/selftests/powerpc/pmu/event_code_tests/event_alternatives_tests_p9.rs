// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: <stdio.h>, "../event.h", "../sampling_tests/misc.h"

const PM_RUN_CYC_ALT: u64 = 0x200f4;
const PM_INST_DISP: u64 = 0x200f2;
const PM_BR_2PATH: u64 = 0x20036;
const PM_LD_MISS_L1: u64 = 0x3e054;
const PM_RUN_INST_CMPL_ALT: u64 = 0x400fa;

const EventCode_1: u64 = 0x200fa;
const EventCode_2: u64 = 0x200fc;
const EventCode_3: u64 = 0x300fc;
const EventCode_4: u64 = 0x400fc;

extern "C" {
    type event;

    static SPRN_PVR: u64;
    static POWER9: u64;

    fn platform_check_for_tests() -> i32;
    fn mfspr(sprn: u64) -> u64;
    fn PVR_VER(pvr: u64) -> u64;
    fn check_for_generic_compat_pmu() -> i32;

    fn event_init(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> i32;
    fn event_open_with_group(event: *mut event, group_fd: i32) -> i32;
    fn event_close(event: *mut event);

    fn event_fd(event: *const event) -> i32;

    fn test_harness(
        test: unsafe extern "C" fn() -> i32,
        name: *const ::std::os::raw::c_char,
    ) -> i32;
}

unsafe fn SKIP_IF(cond: bool) {
    if cond {
        // External test harness macro in the C source.
        return;
    }
}

unsafe fn FAIL_IF(cond: bool) {
    if cond {
        // External test harness macro in the C source.
        return;
    }
}

/*
 * Check for event alternatives.
 */

unsafe extern "C" fn event_alternatives_tests_p9() -> i32 {
    let mut event: event = ::std::mem::zeroed();
    let mut leader: event = ::std::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF(platform_check_for_tests() != 0);

    /*
     * PVR check is used here since PMU specific data like
     * alternative events is handled by respective PMU driver
     * code and using PVR will work correctly for all cases
     * including generic compat mode.
     */
    SKIP_IF(PVR_VER(mfspr(SPRN_PVR)) != POWER9);

    /* Skip for generic compat PMU */
    SKIP_IF(check_for_generic_compat_pmu() != 0);

    /* Init the event for PM_RUN_CYC_ALT */
    event_init(&mut leader, PM_RUN_CYC_ALT);
    FAIL_IF(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_1);

    /*
     * Expected to pass since PM_RUN_CYC_ALT in PMC2 has alternative event
     * 0x600f4. So it can go in with EventCode_1 which is using PMC2
     */
    FAIL_IF(event_open_with_group(&mut event, event_fd(&leader)) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    event_init(&mut leader, PM_INST_DISP);
    FAIL_IF(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_2);
    /*
     * Expected to pass since PM_INST_DISP in PMC2 has alternative event
     * 0x300f2 in PMC3. So it can go in with EventCode_2 which is using PMC2
     */
    FAIL_IF(event_open_with_group(&mut event, event_fd(&leader)) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    event_init(&mut leader, PM_BR_2PATH);
    FAIL_IF(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_2);
    /*
     * Expected to pass since PM_BR_2PATH in PMC2 has alternative event
     * 0x40036 in PMC4. So it can go in with EventCode_2 which is using PMC2
     */
    FAIL_IF(event_open_with_group(&mut event, event_fd(&leader)) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    event_init(&mut leader, PM_LD_MISS_L1);
    FAIL_IF(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_3);
    /*
     * Expected to pass since PM_LD_MISS_L1 in PMC3 has alternative event
     * 0x400f0 in PMC4. So it can go in with EventCode_3 which is using PMC3
     */
    FAIL_IF(event_open_with_group(&mut event, event_fd(&leader)) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    event_init(&mut leader, PM_RUN_INST_CMPL_ALT);
    FAIL_IF(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_4);
    /*
     * Expected to pass since PM_RUN_INST_CMPL_ALT in PMC4 has alternative event
     * 0x500fa in PMC5. So it can go in with EventCode_4 which is using PMC4
     */
    FAIL_IF(event_open_with_group(&mut event, event_fd(&leader)) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    return 0;
}

fn main() -> i32 {
    unsafe {
        return test_harness(
            event_alternatives_tests_p9,
            b"event_alternatives_tests_p9\0".as_ptr() as *const ::std::os::raw::c_char,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
