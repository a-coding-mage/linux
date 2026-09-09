/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/G2L family pinctrl bindings.
 *
 * Copyright (C) 2021 Renesas Electronics Corp.
 *
 */

// Original C header guard: __DT_BINDINGS_RZG2L_PINCTRL_H

pub const RZG2L_PINS_PER_PORT: u32 = 8;

/*
 * Create the pin index from its bank and position numbers and store in
 * the upper 16 bits the alternate function identifier
 */
#[macro_export]
macro_rules! RZG2L_PORT_PINMUX {
    ($b:expr, $p:expr, $f:expr) => {
        (($b) * $crate::RZG2L_PINS_PER_PORT + ($p) | (($f) << 16))
    };
}

/* Convert a port and pin label to its global pin index */
#[macro_export]
macro_rules! RZG2L_GPIO {
    ($port:expr, $pin:expr) => {
        (($port) * $crate::RZG2L_PINS_PER_PORT + ($pin))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
