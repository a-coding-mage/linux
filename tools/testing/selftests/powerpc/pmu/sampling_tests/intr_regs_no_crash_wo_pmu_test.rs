// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

/*
 * C header dependencies translated as external Rust dependencies:
 * ../event.h, misc.h, utils.h provide event, event_init_opts, event_open,
 * event_close, test_harness, PERF_TYPE_SOFTWARE, and PERF_SAMPLE_REGS_INTR.
 */

extern "C" {
    fn event_init_opts(event: *mut event, arg1: i32, arg2: u32, name: *const ::std::os::raw::c_char);
    fn event_open(event: *mut event) -> i32;
    fn event_close(event: *mut event);
    fn test_harness(
        test: unsafe extern "C" fn() -> i32,
        name: *const ::std::os::raw::c_char,
    ) -> i32;
}

/*
 * A perf sampling test for making sure
 * sampling with -intr-regs doesn't crash
 * in any environment, say:
 *  - With generic compat PMU
 *  - without any PMU registered
 *  - With platform specific PMU.
 *  A fix for crash with intr_regs was
 *  addressed in commit: f75e7d73bdf7 in kernel.
 *
 * This testcase exercises this code path by doing
 * intr_regs using software event. Software event is
 * used since s/w event will work even in platform
 * without PMU.
 */
unsafe extern "C" fn intr_regs_no_crash_wo_pmu_test() -> i32 {
    let mut event: event = ::std::mem::zeroed();

    /*
     * Init the event for the sampling test.
     * This uses software event which works on
     * any platform.
     */
    event_init_opts(
        &mut event,
        0,
        PERF_TYPE_SOFTWARE,
        b"cycles\0".as_ptr() as *const ::std::os::raw::c_char,
    );

    event.attr.sample_period = 1000;
    event.attr.sample_type = PERF_SAMPLE_REGS_INTR;
    event.attr.disabled = 1;

    /*
     * Return code of event_open is not considered
     * since test just expects no crash from using
     * PERF_SAMPLE_REGS_INTR.
     */
    event_open(&mut event);

    event_close(&mut event);
    0
}

fn main() -> i32 {
    unsafe {
        test_harness(
            intr_regs_no_crash_wo_pmu_test,
            b"intr_regs_no_crash_wo_pmu_test\0".as_ptr() as *const ::std::os::raw::c_char,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
