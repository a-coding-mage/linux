/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::ffi::c_void;

#[repr(C, align(8))]
pub struct xt_led_info {
    pub id: [i8; 27], /* Unique ID for this trigger in the LED class */
    pub always_blink: u8, /* Blink even if the LED is already on */
    pub delay: u32, /* Delay until LED is switched off after trigger */

    /* Kernel data used in the module */
    pub internal_data: *mut c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
