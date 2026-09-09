/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/G3E family pinctrl bindings.
 *
 * Copyright (C) 2024 Renesas Electronics Corp.
 *
 */

// Dependency translated from: <dt-bindings/pinctrl/rzg2l-pinctrl.h>

/* RZG3E_Px = Offset address of PFC_P_mn  - 0x20 */
pub const RZG3E_P0: i32 = 0;
pub const RZG3E_P1: i32 = 1;
pub const RZG3E_P2: i32 = 2;
pub const RZG3E_P3: i32 = 3;
pub const RZG3E_P4: i32 = 4;
pub const RZG3E_P5: i32 = 5;
pub const RZG3E_P6: i32 = 6;
pub const RZG3E_P7: i32 = 7;
pub const RZG3E_P8: i32 = 8;
pub const RZG3E_PA: i32 = 10;
pub const RZG3E_PB: i32 = 11;
pub const RZG3E_PC: i32 = 12;
pub const RZG3E_PD: i32 = 13;
pub const RZG3E_PE: i32 = 14;
pub const RZG3E_PF: i32 = 15;
pub const RZG3E_PG: i32 = 16;
pub const RZG3E_PH: i32 = 17;
pub const RZG3E_PJ: i32 = 19;
pub const RZG3E_PK: i32 = 20;
pub const RZG3E_PL: i32 = 21;
pub const RZG3E_PM: i32 = 22;
pub const RZG3E_PS: i32 = 28;

macro_rules! RZG3E_PORT_PINMUX {
    ($b:tt, $p:expr, $f:expr) => {
        RZG2L_PORT_PINMUX!(RZG3E_P$b, $p, $f)
    };
}

macro_rules! RZG3E_GPIO {
    ($port:tt, $pin:expr) => {
        RZG2L_GPIO!(RZG3E_P$port, $pin)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
