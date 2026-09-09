/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (C) 2022 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

pub const MEDIA_BUS_TYPE_CSI2_CPHY: i32 = 1;
pub const MEDIA_BUS_TYPE_CSI1: i32 = 2;
pub const MEDIA_BUS_TYPE_CCP2: i32 = 3;
pub const MEDIA_BUS_TYPE_CSI2_DPHY: i32 = 4;
pub const MEDIA_BUS_TYPE_PARALLEL: i32 = 5;
pub const MEDIA_BUS_TYPE_BT656: i32 = 6;

pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_ABC: i32 = 0;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_ACB: i32 = 1;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_BAC: i32 = 2;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_BCA: i32 = 3;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_CAB: i32 = 4;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_CBA: i32 = 5;

pub const MEDIA_PCLK_SAMPLE_FALLING_EDGE: i32 = 0;
pub const MEDIA_PCLK_SAMPLE_RISING_EDGE: i32 = 1;
pub const MEDIA_PCLK_SAMPLE_DUAL_EDGE: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
