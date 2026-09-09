/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * lm8323.h - Configuration for LM8323 keypad driver.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

/*
 * Largest keycode that the chip can send, plus one,
 * so keys can be mapped directly at the index of the
 * LM8323 keycode instead of subtracting one.
 */
pub const LM8323_KEYMAP_SIZE: usize = 0x7f + 1;

pub const LM8323_NUM_PWMS: usize = 3;

#[repr(C)]
pub struct lm8323_platform_data {
    pub debounce_time: i32, /* Time to watch for key bouncing, in ms. */
    pub active_time: i32, /* Idle time until sleep, in ms. */

    pub size_x: i32,
    pub size_y: i32,
    pub repeat: bool,
    pub keymap: *const u16,

    pub pwm_names: [*const core::ffi::c_char; LM8323_NUM_PWMS],

    pub name: *const core::ffi::c_char, /* Device name. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
