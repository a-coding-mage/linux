/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */

// Original C header guard: __PHY_LAN966X_SERDES_H__

macro_rules! CU {
    ($x:expr) => {
        ($x)
    };
}

const CU_MAX: i32 = CU!(2);

macro_rules! SERDES6G {
    ($x:expr) => {
        (CU_MAX + 1 + ($x))
    };
}

const SERDES6G_MAX: i32 = SERDES6G!(3);

macro_rules! RGMII {
    ($x:expr) => {
        (SERDES6G_MAX + 1 + ($x))
    };
}

const RGMII_MAX: i32 = RGMII!(2);
const SERDES_MAX: i32 = RGMII_MAX + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
