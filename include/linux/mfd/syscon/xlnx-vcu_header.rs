/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Pengutronix, Michael Tretter <kernel@pengutronix.de>
 */

// Translated from the C header `xlnx-vcu.h`.

pub const VCU_ECODER_ENABLE: u32 = 0x00;
pub const VCU_DECODER_ENABLE: u32 = 0x04;
pub const VCU_MEMORY_DEPTH: u32 = 0x08;
pub const VCU_ENC_COLOR_DEPTH: u32 = 0x0c;
pub const VCU_ENC_VERTICAL_RANGE: u32 = 0x10;
pub const VCU_ENC_FRAME_SIZE_X: u32 = 0x14;
pub const VCU_ENC_FRAME_SIZE_Y: u32 = 0x18;
pub const VCU_ENC_COLOR_FORMAT: u32 = 0x1c;
pub const VCU_ENC_FPS: u32 = 0x20;
pub const VCU_MCU_CLK: u32 = 0x24;
pub const VCU_CORE_CLK: u32 = 0x28;
pub const VCU_PLL_BYPASS: u32 = 0x2c;
pub const VCU_ENC_CLK: u32 = 0x30;
pub const VCU_PLL_CLK: u32 = 0x34;
pub const VCU_ENC_VIDEO_STANDARD: u32 = 0x38;
pub const VCU_STATUS: u32 = 0x3c;
pub const VCU_AXI_ENC_CLK: u32 = 0x40;
pub const VCU_AXI_DEC_CLK: u32 = 0x44;
pub const VCU_AXI_MCU_CLK: u32 = 0x48;
pub const VCU_DEC_VIDEO_STANDARD: u32 = 0x4c;
pub const VCU_DEC_FRAME_SIZE_X: u32 = 0x50;
pub const VCU_DEC_FRAME_SIZE_Y: u32 = 0x54;
pub const VCU_DEC_FPS: u32 = 0x58;
pub const VCU_BUFFER_B_FRAME: u32 = 0x5c;
pub const VCU_WPP_EN: u32 = 0x60;
pub const VCU_PLL_CLK_DEC: u32 = 0x64;
pub const VCU_NUM_CORE: u32 = 0x6c;
pub const VCU_GASKET_INIT: u32 = 0x74;
pub const VCU_GASKET_VALUE: u32 = 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
