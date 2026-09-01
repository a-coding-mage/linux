/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * max98371.h -- MAX98371 ALSA SoC Audio driver
 *
 * Copyright 2011-2012 Maxim Integrated Products
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub const MAX98371_IRQ_CLEAR1: u32 = 0x01;
pub const MAX98371_IRQ_CLEAR2: u32 = 0x02;
pub const MAX98371_IRQ_CLEAR3: u32 = 0x03;
pub const MAX98371_DAI_CLK: u32 = 0x10;
pub const MAX98371_DAI_BSEL_MASK: u32 = 0xF;
pub const MAX98371_DAI_BSEL_32: u32 = 2;
pub const MAX98371_DAI_BSEL_48: u32 = 3;
pub const MAX98371_DAI_BSEL_64: u32 = 4;
pub const MAX98371_SPK_SR: u32 = 0x11;
pub const MAX98371_SPK_SR_MASK: u32 = 0xF;
pub const MAX98371_SPK_SR_32: u32 = 6;
pub const MAX98371_SPK_SR_44: u32 = 7;
pub const MAX98371_SPK_SR_48: u32 = 8;
pub const MAX98371_SPK_SR_88: u32 = 10;
pub const MAX98371_SPK_SR_96: u32 = 11;
pub const MAX98371_DAI_CHANNEL: u32 = 0x15;
pub const MAX98371_CHANNEL_MASK: u32 = 0x3;
pub const MAX98371_MONOMIX_SRC: u32 = 0x18;
pub const MAX98371_MONOMIX_CFG: u32 = 0x19;
pub const MAX98371_HPF: u32 = 0x1C;
pub const MAX98371_MONOMIX_SRC_MASK: u32 = 0xFF;
pub const MONOMIX_RX_0_1: u32 = (0x1) << (4);
pub const M98371_DAI_CHANNEL_I2S: u32 = 0x3;
pub const MAX98371_DIGITAL_GAIN: u32 = 0x2D;
pub const MAX98371_DIGITAL_GAIN_WIDTH: u32 = 0x7;
pub const MAX98371_GAIN: u32 = 0x2E;
pub const MAX98371_GAIN_SHIFT: u32 = 0x4;
pub const MAX98371_GAIN_WIDTH: u32 = 0x4;
pub const MAX98371_DHT_MAX_WIDTH: u32 = 4;
pub const MAX98371_FMT: u32 = 0x14;
pub const MAX98371_CHANSZ_WIDTH: u32 = 6;
pub const MAX98371_FMT_MASK: u32 = (0x3) << (MAX98371_CHANSZ_WIDTH);
pub const MAX98371_FMT_MODE_MASK: u32 = (0x7) << (3);
pub const MAX98371_DAI_LEFT: u32 = (0x1) << (3);
pub const MAX98371_DAI_RIGHT: u32 = (0x2) << (3);
pub const MAX98371_DAI_CHANSZ_16: u32 = (1) << (MAX98371_CHANSZ_WIDTH);
pub const MAX98371_DAI_CHANSZ_24: u32 = (2) << (MAX98371_CHANSZ_WIDTH);
pub const MAX98371_DAI_CHANSZ_32: u32 = (3) << (MAX98371_CHANSZ_WIDTH);
pub const MAX98371_DHT: u32 = 0x32;
pub const MAX98371_DHT_STEP: u32 = 0x3;
pub const MAX98371_DHT_GAIN: u32 = 0x31;
pub const MAX98371_DHT_GAIN_WIDTH: u32 = 0x4;
pub const MAX98371_DHT_ROT_WIDTH: u32 = 0x4;
pub const MAX98371_SPK_ENABLE: u32 = 0x4A;
pub const MAX98371_GLOBAL_ENABLE: u32 = 0x50;
pub const MAX98371_SOFT_RESET: u32 = 0x51;
pub const MAX98371_VERSION: u32 = 0xFF;

// External dependency from included kernel headers: struct regmap.
pub enum regmap {}

#[repr(C)]
pub struct max98371_priv {
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
