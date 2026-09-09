/* SPDX-License-Identifier: GPL-2.0+ OR MIT */
/*
 * This header provides constants for Apple pinctrl bindings.
 */

/// Equivalent of the C `APPLE_PINMUX(pin, func)` macro.
#[inline]
pub const fn APPLE_PINMUX(pin: u32, func: u32) -> u32 {
    pin | (func << 16)
}

/// Equivalent of the C `APPLE_PIN(pinmux)` macro.
#[inline]
pub const fn APPLE_PIN(pinmux: u32) -> u32 {
    pinmux & 0xffff
}

/// Equivalent of the C `APPLE_FUNC(pinmux)` macro.
#[inline]
pub const fn APPLE_FUNC(pinmux: u32) -> u32 {
    pinmux >> 16
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
