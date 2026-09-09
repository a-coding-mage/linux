/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration corresponding to the C `struct gpio_chip`.
#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

/* arch/sh/boards/mach-x3proto/gpio.c */
unsafe extern "C" {
    pub fn x3proto_gpio_setup() -> ::core::ffi::c_int;

    pub static mut x3proto_gpio_chip: gpio_chip;
}

pub const NR_BASEBOARD_GPIOS: ::core::ffi::c_int = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
