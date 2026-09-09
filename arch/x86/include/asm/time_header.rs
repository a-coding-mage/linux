/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the translated Linux clocksource and MC146818 RTC headers.

/// Opaque declaration corresponding to `struct clock_event_device`.
#[repr(C)]
pub struct clock_event_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn hpet_time_init();
    pub fn pit_timer_init() -> bool;

    pub static mut global_clock_event: *mut clock_event_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
