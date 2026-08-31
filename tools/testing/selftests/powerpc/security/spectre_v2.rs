// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2018-2019 IBM Corporation.
 */

// Translated from C. External symbols are supplied by the selftest harness,
// utils.h, and ../pmu/event.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type bool_t = bool;
type s64 = i64;
type u64 = u64;

const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32;
const PR_TASK_PERF_EVENTS_DISABLE: c_int = 33;

const PPC_FEATURE2_ARCH_2_07: c_ulong = 0x8000_0000;
const SPRN_PVR: c_int = 0x11f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct event_result {
    pub value: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
    pub fd: c_int,
    pub result: event_result,
}

#[repr(C)]
pub struct event_attr {
    pub disabled: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;

    fn pattern_cache_loop();
    fn indirect_branch_loop();

    fn event_init_named(e: *mut event, config: u64, name: *mut c_char);
    fn event_read(e: *mut event);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_report_justified(e: *mut event, name_width: c_int, result_width: c_int);
    fn event_close(e: *mut event);

    fn read_sysfs_file(path: *const c_char, buf: *mut c_char, count: usize) -> c_int;
    fn have_hwcap2(feature: c_ulong) -> bool_t;
    fn mfspr(spr: c_int) -> c_ulong;
    fn test_harness(test: Option<unsafe extern "C" fn() -> c_int>, name: *const c_char) -> c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 4;
        }
    };
}

unsafe fn do_count_loop(events: *mut event, is_p9: bool_t, miss_percent: *mut s64) -> c_int {
    let mut pred: u64;
    let mut mpred: u64;

    unsafe {
        prctl(PR_TASK_PERF_EVENTS_ENABLE);

        if is_p9 {
            pattern_cache_loop();
        } else {
            indirect_branch_loop();
        }

        prctl(PR_TASK_PERF_EVENTS_DISABLE);

        event_read(events.add(0));
        event_read(events.add(1));

        // We could scale all the events by running/enabled but we're lazy
        // As long as the PMU is uncontended they should all run
        FAIL_IF!((*events.add(0)).result.running != (*events.add(0)).result.enabled);
        FAIL_IF!((*events.add(1)).result.running != (*events.add(1)).result.enabled);

        pred = (*events.add(0)).result.value;
        mpred = (*events.add(1)).result.value;

        if is_p9 {
            event_read(events.add(2));
            event_read(events.add(3));
            FAIL_IF!((*events.add(2)).result.running != (*events.add(2)).result.enabled);
            FAIL_IF!((*events.add(3)).result.running != (*events.add(3)).result.enabled);

            pred = pred.wrapping_add((*events.add(2)).result.value);
            mpred = mpred.wrapping_add((*events.add(3)).result.value);
        }

        *miss_percent = (100_u64.wrapping_mul(mpred).wrapping_div(pred)) as s64;
    }

    0
}

unsafe fn setup_event(e: *mut event, config: u64, name: *mut c_char) {
    unsafe {
        event_init_named(e, config, name);

        (*e).attr.disabled = 1;
        (*e).attr.exclude_kernel = 1;
        (*e).attr.exclude_hv = 1;
        (*e).attr.exclude_idle = 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum spectre_v2_state {
    VULNERABLE = 0,
    UNKNOWN = 1, // Works with FAIL_IF()
    NOT_AFFECTED,
    BRANCH_SERIALISATION,
    COUNT_CACHE_DISABLED,
    COUNT_CACHE_FLUSH_SW,
    COUNT_CACHE_FLUSH_HW,
    BTB_FLUSH,
}

unsafe fn get_sysfs_state() -> spectre_v2_state {
    let mut state = spectre_v2_state::UNKNOWN;
    let mut buf = [0 as c_char; 256];
    let len: c_int;

    unsafe {
        memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
        FAIL_IF!(read_sysfs_file(
            c"devices/system/cpu/vulnerabilities/spectre_v2".as_ptr(),
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf)
        ) != 0);

        // Make sure it's NULL terminated
        buf[core::mem::size_of_val(&buf) - 1] = b'\0' as c_char;

        // Trim the trailing newline
        len = strlen(buf.as_ptr()) as c_int;
        FAIL_IF!(len < 1);
        buf[(len - 1) as usize] = b'\0' as c_char;

        printf(c"sysfs reports: '%s'\n".as_ptr(), buf.as_ptr());

        // Order matters
        if !strstr(buf.as_ptr(), c"Vulnerable".as_ptr()).is_null() {
            state = spectre_v2_state::VULNERABLE;
        } else if !strstr(buf.as_ptr(), c"Not affected".as_ptr()).is_null() {
            state = spectre_v2_state::NOT_AFFECTED;
        } else if !strstr(
            buf.as_ptr(),
            c"Indirect branch serialisation (kernel only)".as_ptr(),
        )
        .is_null()
        {
            state = spectre_v2_state::BRANCH_SERIALISATION;
        } else if !strstr(buf.as_ptr(), c"Indirect branch cache disabled".as_ptr()).is_null() {
            state = spectre_v2_state::COUNT_CACHE_DISABLED;
        } else if !strstr(
            buf.as_ptr(),
            c"Software count cache flush (hardware accelerated)".as_ptr(),
        )
        .is_null()
        {
            state = spectre_v2_state::COUNT_CACHE_FLUSH_HW;
        } else if !strstr(buf.as_ptr(), c"Software count cache flush".as_ptr()).is_null() {
            state = spectre_v2_state::COUNT_CACHE_FLUSH_SW;
        } else if !strstr(buf.as_ptr(), c"Branch predictor state flush".as_ptr()).is_null() {
            state = spectre_v2_state::BTB_FLUSH;
        }
    }

    state
}

const PM_BR_PRED_CCACHE: u64 = 0x040a4; // P8 + P9
const PM_BR_MPRED_CCACHE: u64 = 0x040ac; // P8 + P9
const PM_BR_PRED_PCACHE: u64 = 0x048a0; // P9 only
const PM_BR_MPRED_PCACHE: u64 = 0x048b0; // P9 only

#[unsafe(no_mangle)]
pub unsafe extern "C" fn spectre_v2_test() -> c_int {
    let state: spectre_v2_state;
    let mut events: [event; 4] = unsafe { core::mem::zeroed() };
    let mut miss_percent: s64 = 0;
    let is_p9: bool_t;

    unsafe {
        // The PMU events we use only work on Power8 or later
        SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

        state = get_sysfs_state();
        if state == spectre_v2_state::UNKNOWN {
            printf(c"Error: couldn't determine spectre_v2 mitigation state?\n".as_ptr());
            return -1;
        }

        memset(
            events.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&events),
        );

        setup_event(
            &mut events[0],
            PM_BR_PRED_CCACHE,
            c"PM_BR_PRED_CCACHE".as_ptr() as *mut c_char,
        );
        setup_event(
            &mut events[1],
            PM_BR_MPRED_CCACHE,
            c"PM_BR_MPRED_CCACHE".as_ptr() as *mut c_char,
        );
        FAIL_IF!(event_open(&mut events[0]) != 0);
        FAIL_IF!(event_open_with_group(&mut events[1], events[0].fd) == -1);

        is_p9 = ((mfspr(SPRN_PVR) >> 16) & 0xFFFF) == 0x4e;

        if is_p9 {
            // Count pattern cache too
            setup_event(
                &mut events[2],
                PM_BR_PRED_PCACHE,
                c"PM_BR_PRED_PCACHE".as_ptr() as *mut c_char,
            );
            setup_event(
                &mut events[3],
                PM_BR_MPRED_PCACHE,
                c"PM_BR_MPRED_PCACHE".as_ptr() as *mut c_char,
            );

            FAIL_IF!(event_open_with_group(&mut events[2], events[0].fd) == -1);
            FAIL_IF!(event_open_with_group(&mut events[3], events[0].fd) == -1);
        }

        FAIL_IF!(do_count_loop(events.as_mut_ptr(), is_p9, &mut miss_percent) != 0);

        event_report_justified(&mut events[0], 18, 10);
        event_report_justified(&mut events[1], 18, 10);
        event_close(&mut events[0]);
        event_close(&mut events[1]);

        if is_p9 {
            event_report_justified(&mut events[2], 18, 10);
            event_report_justified(&mut events[3], 18, 10);
            event_close(&mut events[2]);
            event_close(&mut events[3]);
        }

        printf(c"Miss percent %lld %%\n".as_ptr(), miss_percent as c_long);

        match state {
            spectre_v2_state::VULNERABLE
            | spectre_v2_state::NOT_AFFECTED
            | spectre_v2_state::COUNT_CACHE_FLUSH_SW
            | spectre_v2_state::COUNT_CACHE_FLUSH_HW => {
                // These should all not affect userspace branch prediction
                if miss_percent > 15 {
                    if miss_percent > 95 {
                        /*
                         * Such a mismatch may be caused by a system being unaware
                         * the count cache is disabled. This may be to enable
                         * guest migration between hosts with different settings.
                         * Return skip code to avoid detecting this as an error.
                         * We are not vulnerable and reporting otherwise, so
                         * missing such a mismatch is safe.
                         */
                        printf(c"Branch misses > 95%% unexpected in this configuration.\n".as_ptr());
                        printf(c"Count cache likely disabled without Linux knowing.\n".as_ptr());
                        if state == spectre_v2_state::COUNT_CACHE_FLUSH_SW {
                            printf(c"WARNING: Kernel performing unnecessary flushes.\n".as_ptr());
                        }
                        return 4;
                    }
                    printf(c"Branch misses > 15%% unexpected in this configuration!\n".as_ptr());
                    printf(c"Possible mismatch between reported & actual mitigation\n".as_ptr());

                    return 1;
                }
            }
            spectre_v2_state::BRANCH_SERIALISATION => {
                // This seems to affect userspace branch prediction a bit?
                if miss_percent > 25 {
                    printf(c"Branch misses > 25%% unexpected in this configuration!\n".as_ptr());
                    printf(c"Possible mismatch between reported & actual mitigation\n".as_ptr());
                    return 1;
                }
            }
            spectre_v2_state::COUNT_CACHE_DISABLED => {
                if miss_percent < 95 {
                    printf(c"Branch misses < 95%% unexpected in this configuration!\n".as_ptr());
                    printf(c"Possible mismatch between reported & actual mitigation\n".as_ptr());
                    return 1;
                }
            }
            spectre_v2_state::UNKNOWN | spectre_v2_state::BTB_FLUSH => {
                printf(c"Not sure!\n".as_ptr());
                return 1;
            }
        }

        printf(c"OK - Measured branch prediction rates match reported spectre v2 mitigation.\n".as_ptr());
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            Some(spectre_v2_test),
            c"spectre_v2".as_ptr(),
        ));
    }
}
