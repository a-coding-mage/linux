/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Defines macros and constants for Renesas RZ/A1 pin controller pin
 * muxing functions.
 */

// Header guard: __DT_BINDINGS_PINCTRL_RENESAS_RZA1_H

pub const RZA1_PINS_PER_PORT: u32 = 16;

/*
 * Create the pin index from its bank and position numbers and store in
 * the upper 16 bits the alternate function identifier
 */
#[inline]
pub const fn RZA1_PINMUX(b: u32, p: u32, f: u32) -> u32 {
    (b * RZA1_PINS_PER_PORT + p) | (f << 16)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
