// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5575.h  --  ALC5575 ALSA SoC audio driver
 *
 * Copyright(c) 2025 Realtek Semiconductor Corp.
 *
 */

// C header guard removed: __RT5575_H__

pub const RT5575_DEVICE_ID: u32 = 0x10ec5575;
pub const RT5575_DSP_MAPPING: u32 = 0x18000000;

pub const RT5575_BOOT: u32 = 0x8004;
pub const RT5575_ID: u32 = 0x8008;
pub const RT5575_ID_1: u32 = 0x800c;
pub const RT5575_MIXL_VOL: u32 = 0x8a14;
pub const RT5575_MIXR_VOL: u32 = 0x8a18;
pub const RT5575_PROMPT_VOL: u32 = 0x8a84;
pub const RT5575_SPK01_VOL: u32 = 0x8a88;
pub const RT5575_SPK23_VOL: u32 = 0x8a8c;
pub const RT5575_MIC1_VOL: u32 = 0x8a98;
pub const RT5575_MIC2_VOL: u32 = 0x8a9c;
pub const RT5575_WNC_CTRL: u32 = 0x80ec;
pub const RT5575_MODE_CTRL: u32 = 0x80f0;
pub const RT5575_I2S_RATE_CTRL: u32 = 0x80f4;
pub const RT5575_SLEEP_CTRL: u32 = 0x80f8;
pub const RT5575_ALG_BYPASS_CTRL: u32 = 0x80fc;
pub const RT5575_PINMUX_CTRL_2: u32 = 0x81a4;
pub const RT5575_GPIO_CTRL_1: u32 = 0x8208;
pub const RT5575_DSP_BUS_CTRL: u32 = 0x880c;
pub const RT5575_SW_INT: u32 = 0x0018;
pub const RT5575_DSP_BOOT_ERR: u32 = 0x8e14;
pub const RT5575_DSP_READY: u32 = 0x8e24;
pub const RT5575_DSP_CMD_ADDR: u32 = 0x8e28;
pub const RT5575_EFUSE_DATA_2: u32 = 0xc638;
pub const RT5575_EFUSE_DATA_3: u32 = 0xc63c;
pub const RT5575_EFUSE_PID: u32 = 0xc660;

pub const RT5575_BOOT_MASK: u32 = 0x3;
pub const RT5575_BOOT_SPI: u32 = 0x0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5575_aif {
    RT5575_AIF1 = 0,
    RT5575_AIF2 = 1,
    RT5575_AIF3 = 2,
    RT5575_AIF4 = 3,
    RT5575_AIFS = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rt5575_priv {
    pub i2c: *mut i2c_client,
    pub component: *mut snd_soc_component,
    pub dsp_regmap: *mut regmap,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
