// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/clocksource/dummy_timer.c
 *
 *  Copyright (C) 2013 ARM Ltd.
 *  All Rights Reserved
 */

// The following types, constants, globals, and functions are supplied by the
// surrounding kernel bindings.

use crate::bindings::{
    clock_event_device, clockevents_register_device, cpuhp_setup_state,
    cpumask_of, per_cpu_ptr, CPUHP_AP_DUMMY_TIMER_STARTING,
    CLOCK_EVT_FEAT_DUMMY, CLOCK_EVT_FEAT_ONESHOT, CLOCK_EVT_FEAT_PERIODIC,
    dummy_timer_evt,
};

// DEFINE_PER_CPU(struct clock_event_device, dummy_timer_evt);

unsafe fn dummy_timer_starting_cpu(cpu: u32) -> i32 {
    let evt: *mut clock_event_device =
        per_cpu_ptr(&raw mut dummy_timer_evt, cpu);

    (*evt).name = b"dummy_timer\0".as_ptr() as *const _;
    (*evt).features = CLOCK_EVT_FEAT_PERIODIC
        | CLOCK_EVT_FEAT_ONESHOT
        | CLOCK_EVT_FEAT_DUMMY;
    (*evt).rating = 100;
    (*evt).cpumask = cpumask_of(cpu);

    clockevents_register_device(evt);
    0
}

unsafe fn dummy_timer_register() -> i32 {
    cpuhp_setup_state(
        CPUHP_AP_DUMMY_TIMER_STARTING,
        b"clockevents/dummy_timer:starting\0".as_ptr() as *const _,
        Some(dummy_timer_starting_cpu),
        None,
    )
}

// early_initcall(dummy_timer_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
