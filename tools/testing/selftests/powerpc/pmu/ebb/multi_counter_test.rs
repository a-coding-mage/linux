// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: stdio.h, stdlib.h, sys/ioctl.h, and "ebb.h".

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
}

#[repr(C)]
pub struct ebb_stats {
    pub ebb_count: c_int,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static sample_period: u64;

    static PERF_EVENT_IOC_ENABLE: c_ulong;
    static PERF_IOC_FLAG_GROUP: c_ulong;
    static SPRN_PMC1: c_int;
    static SPRN_PMC2: c_int;
    static SPRN_PMC3: c_int;
    static SPRN_PMC4: c_int;
    static SPRN_PMC5: c_int;
    static SPRN_PMC6: c_int;

    static standard_ebb_callee: unsafe extern "C" fn();

    fn ebb_is_supported() -> c_int;
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_ebb_init(event: *mut event);
    fn event_open_with_group(event: *mut event, group_fd: c_int) -> c_int;
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn event_read(event: *mut event) -> c_int;
    fn ebb_global_enable();
    fn mtspr(sprn: c_int, value: u64);
    fn pmc_sample_period(period: u64) -> u64;
    fn core_busy_loop() -> c_int;
    fn ebb_check_mmcr0() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

/*
 * Test counting multiple events using EBBs.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_counter() -> c_int {
    let mut events: [event; 6] = unsafe { std::mem::zeroed() };
    let mut i: c_int;
    let mut group_fd: c_int;

    unsafe {
        SKIP_IF(ebb_is_supported() == 0);

        event_init_named(
            &mut events[0],
            0x1001C,
            c"PM_CMPLU_STALL_THRD".as_ptr(),
        );
        event_init_named(
            &mut events[1],
            0x2D016,
            c"PM_CMPLU_STALL_FXU".as_ptr(),
        );
        event_init_named(
            &mut events[2],
            0x30006,
            c"PM_CMPLU_STALL_OTHER_CMPL".as_ptr(),
        );
        event_init_named(&mut events[3], 0x4000A, c"PM_CMPLU_STALL".as_ptr());
        event_init_named(&mut events[4], 0x600f4, c"PM_RUN_CYC".as_ptr());
        event_init_named(&mut events[5], 0x500fa, c"PM_RUN_INST_CMPL".as_ptr());

        event_leader_ebb_init(&mut events[0]);
        i = 1;
        while i < 6 {
            event_ebb_init(&mut events[i as usize]);
            i += 1;
        }

        group_fd = -1;
        i = 0;
        while i < 6 {
            events[i as usize].attr.exclude_kernel = 1;
            events[i as usize].attr.exclude_hv = 1;
            events[i as usize].attr.exclude_idle = 1;

            FAIL_IF(event_open_with_group(&mut events[i as usize], group_fd) != 0);
            if group_fd == -1 {
                group_fd = events[0].fd;
            }
            i += 1;
        }

        ebb_enable_pmc_counting(1);
        ebb_enable_pmc_counting(2);
        ebb_enable_pmc_counting(3);
        ebb_enable_pmc_counting(4);
        ebb_enable_pmc_counting(5);
        ebb_enable_pmc_counting(6);
        setup_ebb_handler(standard_ebb_callee);

        FAIL_IF(ioctl(events[0].fd, PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP) != 0);
        FAIL_IF(event_read(&mut events[0]) != 0);

        ebb_global_enable();

        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC2, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC3, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC4, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC5, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC6, pmc_sample_period(sample_period));

        while ebb_state.stats.ebb_count < 50 {
            FAIL_IF(core_busy_loop() != 0);
            FAIL_IF(ebb_check_mmcr0() != 0);
        }

        ebb_global_disable();
        ebb_freeze_pmcs();

        dump_ebb_state();

        i = 0;
        while i < 6 {
            event_close(&mut events[i as usize]);
            i += 1;
        }

        FAIL_IF(ebb_state.stats.ebb_count == 0);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(multi_counter, CStr::from_bytes_with_nul_unchecked(b"multi_counter\0").as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
