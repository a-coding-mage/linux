/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AM33XX PRM_XXX register bits
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependency intent from the original header: declarations from "prm.h" may
// be required by users of this header.

pub const AM33XX_GFX_MEM_ONSTATE_MASK: u32 = 0x3u32 << 17;
pub const AM33XX_GFX_MEM_RETSTATE_MASK: u32 = 1u32 << 6;
pub const AM33XX_GFX_MEM_STATEST_MASK: u32 = 0x3u32 << 4;
pub const AM33XX_GLOBAL_WARM_SW_RST_MASK: u32 = 1u32 << 1;
pub const AM33XX_RST_GLOBAL_WARM_SW_MASK: u32 = 1u32 << 0;
pub const AM33XX_RST_GLOBAL_COLD_SW_MASK: u32 = 1u32 << 1;
pub const AM33XX_PRUSS_MEM_ONSTATE_MASK: u32 = 0x3u32 << 5;
pub const AM33XX_PRUSS_MEM_RETSTATE_MASK: u32 = 1u32 << 7;
pub const AM33XX_PRUSS_MEM_STATEST_MASK: u32 = 0x3u32 << 23;
pub const AM33XX_LASTPOWERSTATEENTERED_SHIFT: u32 = 24;
pub const AM33XX_LASTPOWERSTATEENTERED_MASK: u32 = 0x3u32 << 24;
pub const AM33XX_LOGICRETSTATE_MASK: u32 = 1u32 << 2;
pub const AM33XX_LOGICRETSTATE_3_3_MASK: u32 = 1u32 << 3;
pub const AM33XX_LOGICSTATEST_SHIFT: u32 = 2;
pub const AM33XX_LOGICSTATEST_MASK: u32 = 1u32 << 2;
pub const AM33XX_LOWPOWERSTATECHANGE_SHIFT: u32 = 4;
pub const AM33XX_LOWPOWERSTATECHANGE_MASK: u32 = 1u32 << 4;
pub const AM33XX_MPU_L1_ONSTATE_MASK: u32 = 0x3u32 << 18;
pub const AM33XX_MPU_L1_RETSTATE_MASK: u32 = 1u32 << 22;
pub const AM33XX_MPU_L1_STATEST_MASK: u32 = 0x3u32 << 6;
pub const AM33XX_MPU_L2_ONSTATE_MASK: u32 = 0x3u32 << 20;
pub const AM33XX_MPU_L2_RETSTATE_MASK: u32 = 1u32 << 23;
pub const AM33XX_MPU_L2_STATEST_MASK: u32 = 0x3u32 << 8;
pub const AM33XX_MPU_RAM_ONSTATE_MASK: u32 = 0x3u32 << 16;
pub const AM33XX_MPU_RAM_RETSTATE_MASK: u32 = 1u32 << 24;
pub const AM33XX_MPU_RAM_STATEST_MASK: u32 = 0x3u32 << 4;
pub const AM33XX_PER_MEM_ONSTATE_MASK: u32 = 0x3u32 << 25;
pub const AM33XX_PER_MEM_RETSTATE_MASK: u32 = 1u32 << 29;
pub const AM33XX_PER_MEM_STATEST_MASK: u32 = 0x3u32 << 17;
pub const AM33XX_RAM_MEM_ONSTATE_MASK: u32 = 0x3u32 << 30;
pub const AM33XX_RAM_MEM_RETSTATE_MASK: u32 = 1u32 << 27;
pub const AM33XX_RAM_MEM_STATEST_MASK: u32 = 0x3u32 << 21;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
