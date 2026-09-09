/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependency: "qcom,sm8450-videocc.h"

/* SM8650 introduces below new clocks and resets compared to SM8450 */

/* VIDEO_CC clocks */
pub const VIDEO_CC_MVS0_SHIFT_CLK: i32 = 12;
pub const VIDEO_CC_MVS0C_SHIFT_CLK: i32 = 13;
pub const VIDEO_CC_MVS1_SHIFT_CLK: i32 = 14;
pub const VIDEO_CC_MVS1C_SHIFT_CLK: i32 = 15;
pub const VIDEO_CC_XO_CLK_SRC: i32 = 16;

/* VIDEO_CC resets */
pub const VIDEO_CC_XO_CLK_ARES: i32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
