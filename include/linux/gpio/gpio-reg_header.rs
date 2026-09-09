/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;
use core::ffi::c_void;

// Forward declarations corresponding to the C header's opaque structures.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

extern "C" {
    pub fn gpio_reg_init(
        dev: *mut device,
        reg: *mut c_void,
        base: i32,
        num: i32,
        label: *const c_char,
        direction: u32,
        def_out: u32,
        names: *const *const c_char,
        irqdom: *mut irq_domain,
        irqs: *const i32,
    ) -> *mut gpio_chip;

    pub fn gpio_reg_resume(gc: *mut gpio_chip) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
