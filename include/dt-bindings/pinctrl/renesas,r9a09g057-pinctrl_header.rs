/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/V2H family pinctrl bindings.
 *
 * Copyright (C) 2024 Renesas Electronics Corp.
 *
 */

// Dependency: <dt-bindings/pinctrl/rzg2l-pinctrl.h>

/* RZV2H_Px = Offset address of PFC_P_mn  - 0x20 */
pub const RZV2H_P0: i32 = 0;
pub const RZV2H_P1: i32 = 1;
pub const RZV2H_P2: i32 = 2;
pub const RZV2H_P3: i32 = 3;
pub const RZV2H_P4: i32 = 4;
pub const RZV2H_P5: i32 = 5;
pub const RZV2H_P6: i32 = 6;
pub const RZV2H_P7: i32 = 7;
pub const RZV2H_P8: i32 = 8;
pub const RZV2H_P9: i32 = 9;
pub const RZV2H_PA: i32 = 10;
pub const RZV2H_PB: i32 = 11;

/* The included header supplies RZG2L_PORT_PINMUX and RZG2L_GPIO. */
#[macro_export]
macro_rules! RZV2H_PORT_PINMUX {
    (0, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P0, $p, $f) };
    (1, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P1, $p, $f) };
    (2, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P2, $p, $f) };
    (3, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P3, $p, $f) };
    (4, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P4, $p, $f) };
    (5, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P5, $p, $f) };
    (6, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P6, $p, $f) };
    (7, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P7, $p, $f) };
    (8, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P8, $p, $f) };
    (9, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_P9, $p, $f) };
    (A, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_PA, $p, $f) };
    (B, $p:expr, $f:expr) => { RZG2L_PORT_PINMUX!(RZV2H_PB, $p, $f) };
}

#[macro_export]
macro_rules! RZV2H_GPIO {
    ($port:tt, $pin:expr) => {
        RZV2H_GPIO!(@port $port, $pin)
    };
    (@port 0, $pin:expr) => { RZG2L_GPIO!(RZV2H_P0, $pin) };
    (@port 1, $pin:expr) => { RZG2L_GPIO!(RZV2H_P1, $pin) };
    (@port 2, $pin:expr) => { RZG2L_GPIO!(RZV2H_P2, $pin) };
    (@port 3, $pin:expr) => { RZG2L_GPIO!(RZV2H_P3, $pin) };
    (@port 4, $pin:expr) => { RZG2L_GPIO!(RZV2H_P4, $pin) };
    (@port 5, $pin:expr) => { RZG2L_GPIO!(RZV2H_P5, $pin) };
    (@port 6, $pin:expr) => { RZG2L_GPIO!(RZV2H_P6, $pin) };
    (@port 7, $pin:expr) => { RZG2L_GPIO!(RZV2H_P7, $pin) };
    (@port 8, $pin:expr) => { RZG2L_GPIO!(RZV2H_P8, $pin) };
    (@port 9, $pin:expr) => { RZG2L_GPIO!(RZV2H_P9, $pin) };
    (@port A, $pin:expr) => { RZG2L_GPIO!(RZV2H_PA, $pin) };
    (@port B, $pin:expr) => { RZG2L_GPIO!(RZV2H_PB, $pin) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
