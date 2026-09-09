/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2014 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

// Dependency: <dt-bindings/clock/rk3188-cru-common.h>

/* soft-reset indices */
pub const SRST_SRST1: i32 = 0;
pub const SRST_SRST2: i32 = 1;

pub const SRST_L2MEM: i32 = 18;
pub const SRST_I2S0: i32 = 23;
pub const SRST_I2S1: i32 = 24;
pub const SRST_I2S2: i32 = 25;
pub const SRST_TIMER2: i32 = 29;

pub const SRST_GPIO4: i32 = 36;
pub const SRST_GPIO6: i32 = 38;

pub const SRST_TSADC: i32 = 92;

pub const SRST_HDMI: i32 = 96;
pub const SRST_HDMI_APB: i32 = 97;
pub const SRST_CIF1: i32 = 111;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
