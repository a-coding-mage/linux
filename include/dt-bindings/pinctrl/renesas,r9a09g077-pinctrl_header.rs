/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/T2H family pinctrl bindings.
 *
 * Copyright (C) 2025 Renesas Electronics Corp.
 */

pub const RZT2H_PINS_PER_PORT: i32 = 8;

/*
 * Create the pin index from its bank and position numbers and store in
 * the upper 16 bits the alternate function identifier
 */
#[macro_export]
macro_rules! RZT2H_PORT_PINMUX {
    ($b:expr, $p:expr, $f:expr) => {
        (($b) * $crate::RZT2H_PINS_PER_PORT + ($p) | (($f) << 16))
    };
}

/* Convert a port and pin label to its global pin index */
#[macro_export]
macro_rules! RZT2H_GPIO {
    ($port:expr, $pin:expr) => {
        (($port) * $crate::RZT2H_PINS_PER_PORT + ($pin))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
