/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C header guard: _ASM_HEXAGON_INTRINSICS_H

// These macros preserve the corresponding Hexagon compiler built-in mappings.
macro_rules! HEXAGON_P_vrmpyhacc_PP {
    () => { __builtin_HEXAGON_M2_vrmac_s0 };
}

macro_rules! HEXAGON_P_vrmpyh_PP {
    () => { __builtin_HEXAGON_M2_vrmpy_s0 };
}

macro_rules! HEXAGON_R_cl0_R {
    () => { __builtin_HEXAGON_S2_cl0 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
