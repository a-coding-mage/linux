/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for hisilicon pinctrl bindings.
 *
 * Copyright (c) 2015 HiSilicon Limited.
 * Copyright (c) 2015 Linaro Limited.
 */

/* iomg bit definition */
pub const MUX_M0: u32 = 0;
pub const MUX_M1: u32 = 1;
pub const MUX_M2: u32 = 2;
pub const MUX_M3: u32 = 3;
pub const MUX_M4: u32 = 4;
pub const MUX_M5: u32 = 5;
pub const MUX_M6: u32 = 6;
pub const MUX_M7: u32 = 7;

/* iocg bit definition */
pub const PULL_MASK: u32 = 3;
pub const PULL_DIS: u32 = 0;
pub const PULL_UP: u32 = 1 << 0;
pub const PULL_DOWN: u32 = 1 << 1;

/* drive strength definition */
pub const DRIVE_MASK: u32 = 7 << 4;
pub const DRIVE1_02MA: u32 = 0 << 4;
pub const DRIVE1_04MA: u32 = 1 << 4;
pub const DRIVE1_08MA: u32 = 2 << 4;
pub const DRIVE1_10MA: u32 = 3 << 4;
pub const DRIVE2_02MA: u32 = 0 << 4;
pub const DRIVE2_04MA: u32 = 1 << 4;
pub const DRIVE2_08MA: u32 = 2 << 4;
pub const DRIVE2_10MA: u32 = 3 << 4;
pub const DRIVE3_04MA: u32 = 0 << 4;
pub const DRIVE3_08MA: u32 = 1 << 4;
pub const DRIVE3_12MA: u32 = 2 << 4;
pub const DRIVE3_16MA: u32 = 3 << 4;
pub const DRIVE3_20MA: u32 = 4 << 4;
pub const DRIVE3_24MA: u32 = 5 << 4;
pub const DRIVE3_32MA: u32 = 6 << 4;
pub const DRIVE3_40MA: u32 = 7 << 4;
pub const DRIVE4_02MA: u32 = 0 << 4;
pub const DRIVE4_04MA: u32 = 2 << 4;
pub const DRIVE4_08MA: u32 = 4 << 4;
pub const DRIVE4_10MA: u32 = 6 << 4;

/* drive strength definition for hi3660 */
pub const DRIVE6_MASK: u32 = 15 << 4;
pub const DRIVE6_04MA: u32 = 0 << 4;
pub const DRIVE6_12MA: u32 = 4 << 4;
pub const DRIVE6_19MA: u32 = 8 << 4;
pub const DRIVE6_27MA: u32 = 10 << 4;
pub const DRIVE6_32MA: u32 = 15 << 4;
pub const DRIVE7_02MA: u32 = 0 << 4;
pub const DRIVE7_04MA: u32 = 1 << 4;
pub const DRIVE7_06MA: u32 = 2 << 4;
pub const DRIVE7_08MA: u32 = 3 << 4;
pub const DRIVE7_10MA: u32 = 4 << 4;
pub const DRIVE7_12MA: u32 = 5 << 4;
pub const DRIVE7_14MA: u32 = 6 << 4;
pub const DRIVE7_16MA: u32 = 7 << 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
