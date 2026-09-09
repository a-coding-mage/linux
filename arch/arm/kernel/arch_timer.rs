// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/arch_timer.c
 *
 *  Copyright (C) 2011 ARM Ltd.
 *  All Rights Reserved
 */

// Declarations supplied by the corresponding kernel headers.
extern "C" {
    fn arch_timer_read_counter() -> ::core::ffi::c_ulong;
    fn arch_timer_get_rate() -> u32;
    fn register_current_timer_delay(timer: *mut delay_timer);
}

#[repr(C)]
struct delay_timer {
    read_current_timer: Option<unsafe extern "C" fn() -> ::core::ffi::c_ulong>,
    freq: u32,
}

const ENXIO: i32 = 6;

unsafe fn arch_timer_read_counter_long() -> ::core::ffi::c_ulong {
    arch_timer_read_counter()
}

static mut arch_delay_timer: delay_timer = delay_timer {
    read_current_timer: None,
    freq: 0,
};

unsafe fn arch_timer_delay_timer_register() {
    /* Use the architected timer for the delay loop. */
    arch_delay_timer.read_current_timer = Some(arch_timer_read_counter_long);
    arch_delay_timer.freq = arch_timer_get_rate();
    register_current_timer_delay(&raw mut arch_delay_timer);
}

pub unsafe fn arch_timer_arch_init() -> i32 {
    let arch_timer_rate: u32 = arch_timer_get_rate();

    if arch_timer_rate == 0 {
        return -ENXIO;
    }

    arch_timer_delay_timer_register();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
