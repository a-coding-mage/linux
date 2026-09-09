/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/V2M pinctrl bindings.
 *
 * Copyright (C) 2022 Renesas Electronics Corp.
 *
 */

pub const RZV2M_PINS_PER_PORT: i32 = 16;

/*
 * Create the pin index from its bank and position numbers and store in
 * the upper 16 bits the alternate function identifier
 */
#[inline]
pub const fn RZV2M_PORT_PINMUX(b: i32, p: i32, f: i32) -> i32 {
    b * RZV2M_PINS_PER_PORT + p | (f << 16)
}

/* Convert a port and pin label to its global pin index */
#[inline]
pub const fn RZV2M_GPIO(port: i32, pin: i32) -> i32 {
    port * RZV2M_PINS_PER_PORT + pin
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
