/* SPDX-License-Identifier: GPL-2.0 */
/*
 * goldfish-timer clocksource
 * Registers definition for the goldfish-timer device
 */

/*
 * TIMER_TIME_LOW\t get low bits of current time and update TIMER_TIME_HIGH
 * TIMER_TIME_HIGH\t get high bits of time at last TIMER_TIME_LOW read
 * TIMER_ALARM_LOW\t set low bits of alarm and activate it
 * TIMER_ALARM_HIGH\t set high bits of next alarm
 * TIMER_IRQ_ENABLED\t enable alarm interrupt
 * TIMER_CLEAR_ALARM\t disarm an existing alarm
 * TIMER_ALARM_STATUS\t alarm status (running or not)
 * TIMER_CLEAR_INTERRUPT clear interrupt
 */
pub const TIMER_TIME_LOW: u32 = 0x00;
pub const TIMER_TIME_HIGH: u32 = 0x04;
pub const TIMER_ALARM_LOW: u32 = 0x08;
pub const TIMER_ALARM_HIGH: u32 = 0x0c;
pub const TIMER_IRQ_ENABLED: u32 = 0x10;
pub const TIMER_CLEAR_ALARM: u32 = 0x14;
pub const TIMER_ALARM_STATUS: u32 = 0x18;
pub const TIMER_CLEAR_INTERRUPT: u32 = 0x1c;

extern "C" {
    pub fn goldfish_timer_init(irq: core::ffi::c_int, base: *mut core::ffi::c_void) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
