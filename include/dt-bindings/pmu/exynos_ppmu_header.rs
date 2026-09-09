/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Samsung Exynos PPMU event types for counting in regs
 *
 * Copyright (c) 2019, Samsung Electronics
 * Author: Lukasz Luba <l.luba@partner.samsung.com>
 */

pub const PPMU_RO_BUSY_CYCLE_CNT: u32 = 0x0;
pub const PPMU_WO_BUSY_CYCLE_CNT: u32 = 0x1;
pub const PPMU_RW_BUSY_CYCLE_CNT: u32 = 0x2;
pub const PPMU_RO_REQUEST_CNT: u32 = 0x3;
pub const PPMU_WO_REQUEST_CNT: u32 = 0x4;
pub const PPMU_RO_DATA_CNT: u32 = 0x5;
pub const PPMU_WO_DATA_CNT: u32 = 0x6;
pub const PPMU_RO_LATENCY: u32 = 0x12;
pub const PPMU_WO_LATENCY: u32 = 0x16;
pub const PPMU_V2_RO_DATA_CNT: u32 = 0x4;
pub const PPMU_V2_WO_DATA_CNT: u32 = 0x5;
pub const PPMU_V2_EVT3_RW_DATA_CNT: u32 = 0x22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
