/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ALSA SoC Audio Layer - Samsung I2S Controller driver
 *
 * Copyright (c) 2010 Samsung Electronics Co. Ltd.
 *	Jaswinder Singh <jassisinghbrar@gmail.com>
 */

pub const SAMSUNG_I2S_DAI: &str = "samsung-i2s";
pub const SAMSUNG_I2S_DAI_SEC: &str = "samsung-i2s-sec";

pub const SAMSUNG_I2S_DIV_BCLK: u32 = 1;

pub const SAMSUNG_I2S_RCLKSRC_0: u32 = 0;
pub const SAMSUNG_I2S_RCLKSRC_1: u32 = 1;
pub const SAMSUNG_I2S_CDCLK: u32 = 2;
/* Operation clock for IIS logic */
pub const SAMSUNG_I2S_OPCLK: u32 = 3;
pub const SAMSUNG_I2S_OPCLK_CDCLK_OUT: u32 = 0; /* CODEC clock out */
pub const SAMSUNG_I2S_OPCLK_CDCLK_IN: u32 = 1; /* CODEC clock in */
pub const SAMSUNG_I2S_OPCLK_BCLK_OUT: u32 = 2; /* Bit clock out */
pub const SAMSUNG_I2S_OPCLK_PCLK: u32 = 3; /* Audio bus clock */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
