/* SPDX-License-Identifier: GPL-2.0 */
/*
 * timer.h: Definitions for the timer chips on the Sparc.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// C header dependencies:
// linux/clocksource.h, linux/irqreturn.h, asm-generic/percpu.h,
// and asm/cpu_type.h (for SUN4M_NCPUS).

pub const SBUS_CLOCK_RATE: u32 = 2_000_000; // 2MHz
pub const TIMER_VALUE_SHIFT: u32 = 9;
pub const TIMER_VALUE_MASK: u32 = 0x3f_ffff;
pub const TIMER_LIMIT_BIT: u32 = 1u32 << 31; // Bit 31 in Counter-Timer register

/* The counter timer register has the value offset by 9 bits.
 * From sun4m manual:
 * When a counter reaches the value in the corresponding limit register,
 * the Limit bit is set and the counter is set to 500 nS (i.e. 0x00000200).
 *
 * To compensate for this add one to the value.
 */
#[inline]
pub const fn timer_value(value: u32) -> u32 {
    value.wrapping_add(1).wrapping_shl(TIMER_VALUE_SHIFT)
}

extern "C" {
    pub static mut master_l10_counter: *mut u32;

    pub fn timer_interrupt(dummy: i32, dev_id: *mut core::ffi::c_void) -> crate::irqreturn_t;
}

// #ifdef CONFIG_SMP
// DECLARE_PER_CPU(struct clock_event_device, sparc32_clockevent);
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut sparc32_clockevent: [crate::clock_event_device; crate::SUN4M_NCPUS];

    pub fn register_percpu_ce(cpu: i32);
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
