/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Xilinx Controls Header
 *
 * Copyright (C) 2013-2015 Ideas on Board
 * Copyright (C) 2013-2015 Xilinx, Inc.
 *
 * Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
 *           Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 *
 * This software is licensed under the terms of the GNU General Public
 * License version 2, as published by the Free Software Foundation, and
 * may be copied, distributed, and modified under those terms.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// Dependency: V4L2_CID_USER_BASE is supplied by linux/v4l2-controls.h.

pub const V4L2_CID_XILINX_OFFSET: u32 = 0xc000;
pub const V4L2_CID_XILINX_BASE: u32 = V4L2_CID_USER_BASE + V4L2_CID_XILINX_OFFSET;

/*
 * Private Controls for Xilinx Video IPs
 */

/*
 * Xilinx TPG Video IP
 */

pub const V4L2_CID_XILINX_TPG: u32 = V4L2_CID_USER_BASE + 0xc000;

/* Draw cross hairs */
pub const V4L2_CID_XILINX_TPG_CROSS_HAIRS: u32 = V4L2_CID_XILINX_TPG + 1;
/* Enable a moving box */
pub const V4L2_CID_XILINX_TPG_MOVING_BOX: u32 = V4L2_CID_XILINX_TPG + 2;
/* Mask out a color component */
pub const V4L2_CID_XILINX_TPG_COLOR_MASK: u32 = V4L2_CID_XILINX_TPG + 3;
/* Enable a stuck pixel feature */
pub const V4L2_CID_XILINX_TPG_STUCK_PIXEL: u32 = V4L2_CID_XILINX_TPG + 4;
/* Enable a noisy output */
pub const V4L2_CID_XILINX_TPG_NOISE: u32 = V4L2_CID_XILINX_TPG + 5;
/* Enable the motion feature */
pub const V4L2_CID_XILINX_TPG_MOTION: u32 = V4L2_CID_XILINX_TPG + 6;
/* Configure the motion speed of moving patterns */
pub const V4L2_CID_XILINX_TPG_MOTION_SPEED: u32 = V4L2_CID_XILINX_TPG + 7;
/* The row of horizontal cross hair location */
pub const V4L2_CID_XILINX_TPG_CROSS_HAIR_ROW: u32 = V4L2_CID_XILINX_TPG + 8;
/* The colum of vertical cross hair location */
pub const V4L2_CID_XILINX_TPG_CROSS_HAIR_COLUMN: u32 = V4L2_CID_XILINX_TPG + 9;
/* Set starting point of sine wave for horizontal component */
pub const V4L2_CID_XILINX_TPG_ZPLATE_HOR_START: u32 = V4L2_CID_XILINX_TPG + 10;
/* Set speed of the horizontal component */
pub const V4L2_CID_XILINX_TPG_ZPLATE_HOR_SPEED: u32 = V4L2_CID_XILINX_TPG + 11;
/* Set starting point of sine wave for vertical component */
pub const V4L2_CID_XILINX_TPG_ZPLATE_VER_START: u32 = V4L2_CID_XILINX_TPG + 12;
/* Set speed of the vertical component */
pub const V4L2_CID_XILINX_TPG_ZPLATE_VER_SPEED: u32 = V4L2_CID_XILINX_TPG + 13;
/* Moving box size */
pub const V4L2_CID_XILINX_TPG_BOX_SIZE: u32 = V4L2_CID_XILINX_TPG + 14;
/* Moving box color */
pub const V4L2_CID_XILINX_TPG_BOX_COLOR: u32 = V4L2_CID_XILINX_TPG + 15;
/* Upper limit count of generated stuck pixels */
pub const V4L2_CID_XILINX_TPG_STUCK_PIXEL_THRESH: u32 = V4L2_CID_XILINX_TPG + 16;
/* Noise level */
pub const V4L2_CID_XILINX_TPG_NOISE_GAIN: u32 = V4L2_CID_XILINX_TPG + 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
