/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2013 Tomasz Figa <tomasz.figa@gmail.com>
 *
 * Samsung PWM controller platform data helpers.
 */

/* Dependency supplied by <clocksource/samsung_pwm.h>. */
#[repr(C)]
pub struct samsung_pwm_variant {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SAMSUNG_DEV_PWM")]
extern "C" {
    pub fn samsung_pwm_set_platdata(pd: *mut samsung_pwm_variant);
}

#[cfg(not(feature = "CONFIG_SAMSUNG_DEV_PWM"))]
#[inline]
pub unsafe fn samsung_pwm_set_platdata(_pd: *mut samsung_pwm_variant) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
