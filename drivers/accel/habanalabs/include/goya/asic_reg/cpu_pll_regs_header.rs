/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

/*
 *****************************************
 *   CPU_PLL (Prototype: PLL)
 *****************************************
 */

pub const mmCPU_PLL_NR: u32 = 0x4A2100;

pub const mmCPU_PLL_NF: u32 = 0x4A2104;

pub const mmCPU_PLL_OD: u32 = 0x4A2108;

pub const mmCPU_PLL_NB: u32 = 0x4A210C;

pub const mmCPU_PLL_CFG: u32 = 0x4A2110;

pub const mmCPU_PLL_LOSE_MASK: u32 = 0x4A2120;

pub const mmCPU_PLL_LOCK_INTR: u32 = 0x4A2128;

pub const mmCPU_PLL_LOCK_BYPASS: u32 = 0x4A212C;

pub const mmCPU_PLL_DATA_CHNG: u32 = 0x4A2130;

pub const mmCPU_PLL_RST: u32 = 0x4A2134;

pub const mmCPU_PLL_SLIP_WD_CNTR: u32 = 0x4A2150;

pub const mmCPU_PLL_DIV_FACTOR_0: u32 = 0x4A2200;

pub const mmCPU_PLL_DIV_FACTOR_1: u32 = 0x4A2204;

pub const mmCPU_PLL_DIV_FACTOR_2: u32 = 0x4A2208;

pub const mmCPU_PLL_DIV_FACTOR_3: u32 = 0x4A220C;

pub const mmCPU_PLL_DIV_FACTOR_CMD_0: u32 = 0x4A2220;

pub const mmCPU_PLL_DIV_FACTOR_CMD_1: u32 = 0x4A2224;

pub const mmCPU_PLL_DIV_FACTOR_CMD_2: u32 = 0x4A2228;

pub const mmCPU_PLL_DIV_FACTOR_CMD_3: u32 = 0x4A222C;

pub const mmCPU_PLL_DIV_SEL_0: u32 = 0x4A2280;

pub const mmCPU_PLL_DIV_SEL_1: u32 = 0x4A2284;

pub const mmCPU_PLL_DIV_SEL_2: u32 = 0x4A2288;

pub const mmCPU_PLL_DIV_SEL_3: u32 = 0x4A228C;

pub const mmCPU_PLL_DIV_EN_0: u32 = 0x4A22A0;

pub const mmCPU_PLL_DIV_EN_1: u32 = 0x4A22A4;

pub const mmCPU_PLL_DIV_EN_2: u32 = 0x4A22A8;

pub const mmCPU_PLL_DIV_EN_3: u32 = 0x4A22AC;

pub const mmCPU_PLL_DIV_FACTOR_BUSY_0: u32 = 0x4A22C0;

pub const mmCPU_PLL_DIV_FACTOR_BUSY_1: u32 = 0x4A22C4;

pub const mmCPU_PLL_DIV_FACTOR_BUSY_2: u32 = 0x4A22C8;

pub const mmCPU_PLL_DIV_FACTOR_BUSY_3: u32 = 0x4A22CC;

pub const mmCPU_PLL_CLK_GATER: u32 = 0x4A2300;

pub const mmCPU_PLL_CLK_RLX_0: u32 = 0x4A2310;

pub const mmCPU_PLL_CLK_RLX_1: u32 = 0x4A2314;

pub const mmCPU_PLL_CLK_RLX_2: u32 = 0x4A2318;

pub const mmCPU_PLL_CLK_RLX_3: u32 = 0x4A231C;

pub const mmCPU_PLL_REF_CNTR_PERIOD: u32 = 0x4A2400;

pub const mmCPU_PLL_REF_LOW_THRESHOLD: u32 = 0x4A2410;

pub const mmCPU_PLL_REF_HIGH_THRESHOLD: u32 = 0x4A2420;

pub const mmCPU_PLL_PLL_NOT_STABLE: u32 = 0x4A2430;

pub const mmCPU_PLL_FREQ_CALC_EN: u32 = 0x4A2440;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
