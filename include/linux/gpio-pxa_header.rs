/* SPDX-License-Identifier: GPL-2.0 */

/// Equivalent of the C macro `GPIO_bit(x)`.
#[inline]
pub const fn GPIO_bit(x: i32) -> i32 {
    1i32 << (x & 0x1f)
}

/// Equivalent of the C macro `gpio_to_bank(gpio)`.
#[inline]
pub const fn gpio_to_bank(gpio: i32) -> i32 {
    gpio >> 5
}

/* NOTE: some PXAs have fewer on-chip GPIOs (like PXA255, with 85).
 * Those cases currently cause holes in the GPIO number space, the
 * actual number of the last GPIO is recorded by 'pxa_last_gpio'.
 */
extern "C" {
    pub static mut pxa_last_gpio: i32;

    pub fn pxa_irq_to_gpio(irq: i32) -> i32;
}

#[repr(C)]
pub struct pxa_gpio_platform_data {
    pub irq_base: i32,
    pub gpio_set_wake: Option<unsafe extern "C" fn(gpio: u32, on: u32) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
