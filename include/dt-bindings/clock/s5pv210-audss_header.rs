/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Tomasz Figa <tomasz.figa@gmail.com>
 *
 * This header provides constants for Samsung audio subsystem
 * clock controller.
 *
 * The constants defined in this header are being used in dts
 * and s5pv210 audss driver.
 */

pub const CLK_MOUT_AUDSS: u32 = 0;
pub const CLK_MOUT_I2S_A: u32 = 1;

pub const CLK_DOUT_AUD_BUS: u32 = 2;
pub const CLK_DOUT_I2S_A: u32 = 3;

pub const CLK_I2S: u32 = 4;
pub const CLK_HCLK_I2S: u32 = 5;
pub const CLK_HCLK_UART: u32 = 6;
pub const CLK_HCLK_HWA: u32 = 7;
pub const CLK_HCLK_DMA: u32 = 8;
pub const CLK_HCLK_BUF: u32 = 9;
pub const CLK_HCLK_RP: u32 = 10;

pub const AUDSS_MAX_CLKS: u32 = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
