/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/G3L family pinctrl bindings.
 *
 * Copyright (C) 2026 Renesas Electronics Corp.
 *
 */

// Dependency: <dt-bindings/pinctrl/rzg2l-pinctrl.h>

/* RZG3L_Px = Offset address of PFC_P_mn  - 0x22 */
pub const RZG3L_P2: u32 = 2;
pub const RZG3L_P3: u32 = 3;
pub const RZG3L_P5: u32 = 5;
pub const RZG3L_P6: u32 = 6;
pub const RZG3L_P7: u32 = 7;
pub const RZG3L_P8: u32 = 8;
pub const RZG3L_PA: u32 = 10;
pub const RZG3L_PB: u32 = 11;
pub const RZG3L_PC: u32 = 12;
pub const RZG3L_PD: u32 = 13;
pub const RZG3L_PE: u32 = 14;
pub const RZG3L_PF: u32 = 15;
pub const RZG3L_PG: u32 = 16;
pub const RZG3L_PH: u32 = 17;
pub const RZG3L_PJ: u32 = 19;
pub const RZG3L_PK: u32 = 20;
pub const RZG3L_PL: u32 = 21;
pub const RZG3L_PM: u32 = 22;
pub const RZG3L_PS: u32 = 28;

macro_rules! RZG3L_PORT_PINMUX {
    ($b:ident, $p:expr, $f:expr) => {
        RZG2L_PORT_PINMUX!(RZG3L_PORT_VALUE!($b), $p, $f)
    };
}

macro_rules! RZG3L_GPIO {
    ($port:ident, $pin:expr) => {
        RZG2L_GPIO!(RZG3L_PORT_VALUE!($port), $pin)
    };
}

macro_rules! RZG3L_PORT_VALUE {
    (P2) => { RZG3L_P2 };
    (P3) => { RZG3L_P3 };
    (P5) => { RZG3L_P5 };
    (P6) => { RZG3L_P6 };
    (P7) => { RZG3L_P7 };
    (P8) => { RZG3L_P8 };
    (PA) => { RZG3L_PA };
    (PB) => { RZG3L_PB };
    (PC) => { RZG3L_PC };
    (PD) => { RZG3L_PD };
    (PE) => { RZG3L_PE };
    (PF) => { RZG3L_PF };
    (PG) => { RZG3L_PG };
    (PH) => { RZG3L_PH };
    (PJ) => { RZG3L_PJ };
    (PK) => { RZG3L_PK };
    (PL) => { RZG3L_PL };
    (PM) => { RZG3L_PM };
    (PS) => { RZG3L_PS };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
