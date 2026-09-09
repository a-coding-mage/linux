/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Samsung Electronics Co., Ltd.
 */

// Translation of <linux/spinlock.h> dependency; the required type is supplied
// by the surrounding kernel bindings.

pub const SAMSUNG_PWM_NUM: u32 = 5;

/*
 * Following declaration must be in an ifdef due to this symbol being static
 * in pwm-samsung driver if the clocksource driver is not compiled in and the
 * spinlock is not shared between both drivers.
 *
 * CONFIG_CLKSRC_SAMSUNG_PWM is a build-time C condition. Enable the
 * declaration only when the corresponding Rust configuration is selected.
 */
#[cfg(feature = "CONFIG_CLKSRC_SAMSUNG_PWM")]
extern "C" {
    pub static mut samsung_pwm_lock: raw_spinlock_t;
}

#[repr(C)]
pub struct samsung_pwm_variant {
    pub bits: u8,
    pub div_base: u8,
    pub tclk_mask: u8,
    pub output_mask: u8,
    pub has_tint_cstat: bool,
}

extern "C" {
    pub fn samsung_pwm_clocksource_init(
        base: *mut core::ffi::c_void,
        irqs: *mut u32,
        variant: *const samsung_pwm_variant,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
