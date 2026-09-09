/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/plat-omap/include/mach/l4_3xxx.h - L4 firewall definitions
 *
 * Copyright (C) 2009 Nokia Corporation
 * Paul Walmsley
 */

/* L4 CORE */
pub const OMAP3_L4_CORE_FW_I2C1_REGION: i32 = 21;
pub const OMAP3_L4_CORE_FW_I2C1_TA_REGION: i32 = 22;
pub const OMAP3_L4_CORE_FW_I2C2_REGION: i32 = 23;
pub const OMAP3_L4_CORE_FW_I2C2_TA_REGION: i32 = 24;
pub const OMAP3_L4_CORE_FW_I2C3_REGION: i32 = 73;
pub const OMAP3_L4_CORE_FW_I2C3_TA_REGION: i32 = 74;

/* Display Sub system (DSS) */
pub const OMAP3_L4_CORE_FW_DSS_PROT_GROUP: i32 = 2;

pub const OMAP3_L4_CORE_FW_DSS_DSI_REGION: i32 = 104;
pub const OMAP3ES1_L4_CORE_FW_DSS_CORE_REGION: i32 = 3;
pub const OMAP3_L4_CORE_FW_DSS_CORE_REGION: i32 = 4;
pub const OMAP3_L4_CORE_FW_DSS_DISPC_REGION: i32 = 4;
pub const OMAP3_L4_CORE_FW_DSS_RFBI_REGION: i32 = 5;
pub const OMAP3_L4_CORE_FW_DSS_VENC_REGION: i32 = 6;
pub const OMAP3_L4_CORE_FW_DSS_TA_REGION: i32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
