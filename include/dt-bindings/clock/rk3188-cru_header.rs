/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2014 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

// Dependency provided by the original include:
// #include <dt-bindings/clock/rk3188-cru-common.h>

/* soft-reset indices */
pub const SRST_PTM_CORE2: i32 = 0;
pub const SRST_PTM_CORE3: i32 = 1;
pub const SRST_CORE2: i32 = 5;
pub const SRST_CORE3: i32 = 6;
pub const SRST_CORE2_DBG: i32 = 10;
pub const SRST_CORE3_DBG: i32 = 11;

pub const SRST_TIMER2: i32 = 16;
pub const SRST_TIMER4: i32 = 23;
pub const SRST_I2S0: i32 = 24;
pub const SRST_TIMER5: i32 = 25;
pub const SRST_TIMER3: i32 = 29;
pub const SRST_TIMER6: i32 = 31;

pub const SRST_PTM3: i32 = 36;
pub const SRST_PTM3_ATB: i32 = 37;

pub const SRST_GPS: i32 = 67;
pub const SRST_HSICPHY: i32 = 75;
pub const SRST_TIMER: i32 = 78;

pub const SRST_PTM2: i32 = 92;
pub const SRST_CORE2_WDT: i32 = 94;
pub const SRST_CORE3_WDT: i32 = 95;

pub const SRST_PTM2_ATB: i32 = 111;

pub const SRST_HSIC: i32 = 117;
pub const SRST_CTI2: i32 = 118;
pub const SRST_CTI2_APB: i32 = 119;
pub const SRST_GPU_BRIDGE: i32 = 121;
pub const SRST_CTI3: i32 = 123;
pub const SRST_CTI3_APB: i32 = 124;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
