/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: BCM63XX_TIMER_H_

use core::ffi::c_void;

pub unsafe extern "C" {
    pub fn bcm63xx_timer_register(
        id: i32,
        callback: Option<unsafe extern "C" fn(data: *mut c_void)>,
        data: *mut c_void,
    ) -> i32;

    pub fn bcm63xx_timer_unregister(id: i32);

    pub fn bcm63xx_timer_set(id: i32, monotonic: i32, countdown_us: u32) -> i32;

    pub fn bcm63xx_timer_enable(id: i32) -> i32;

    pub fn bcm63xx_timer_disable(id: i32) -> i32;

    pub fn bcm63xx_timer_countdown(countdown_us: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
