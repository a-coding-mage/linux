// SPDX-License-Identifier: GPL-2.0
/*
 * Xilinx Video IP Core
 *
 * Copyright (C) 2013-2015 Ideas on Board
 * Copyright (C) 2013-2015 Xilinx, Inc.
 *
 * Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
 *           Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

/*
 * Video format codes as defined in "AXI4-Stream Video IP and System Design
 * Guide".
 */
pub const XVIP_VF_YUV_422: u32 = 0;
pub const XVIP_VF_YUV_444: u32 = 1;
pub const XVIP_VF_RBG: u32 = 2;
pub const XVIP_VF_YUV_420: u32 = 3;
pub const XVIP_VF_YUVA_422: u32 = 4;
pub const XVIP_VF_YUVA_444: u32 = 5;
pub const XVIP_VF_RGBA: u32 = 6;
pub const XVIP_VF_YUVA_420: u32 = 7;
pub const XVIP_VF_YUVD_422: u32 = 8;
pub const XVIP_VF_YUVD_444: u32 = 9;
pub const XVIP_VF_RGBD: u32 = 10;
pub const XVIP_VF_YUVD_420: u32 = 11;
pub const XVIP_VF_MONO_SENSOR: u32 = 12;
pub const XVIP_VF_CUSTOM2: u32 = 13;
pub const XVIP_VF_CUSTOM3: u32 = 14;
pub const XVIP_VF_CUSTOM4: u32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
