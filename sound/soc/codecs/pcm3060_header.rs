/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PCM3060 codec driver
 *
 * Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>
 */

/* Dependencies from the original C header:
 * #include <linux/device.h>
 * #include <linux/regmap.h>
 */

unsafe extern "C" {
    pub static pcm3060_regmap: regmap_config;

    pub fn pcm3060_probe(dev: *mut device) -> i32;
    pub fn pcm3060_remove(dev: *mut device) -> i32;
}

pub const PCM3060_DAI_ID_DAC: usize = 0;
pub const PCM3060_DAI_ID_ADC: usize = 1;
pub const PCM3060_DAI_IDS_NUM: usize = 2;

/* ADC and DAC can be clocked from separate or same sources CLK1 and CLK2 */
pub const PCM3060_CLK_DEF: u32 = 0; /* default: CLK1->ADC, CLK2->DAC */
pub const PCM3060_CLK1: u32 = 1;
pub const PCM3060_CLK2: u32 = 2;

#[repr(C)]
pub struct pcm3060_priv_dai {
    pub is_provider: bool,
    pub sclk_freq: u32,
}

#[repr(C)]
pub struct pcm3060_priv {
    pub regmap: *mut regmap,
    pub dai: [pcm3060_priv_dai; PCM3060_DAI_IDS_NUM],
    /* C bitfield: u8 out_se: 1; */
    pub out_se: u8,
}

/* registers */

pub const PCM3060_REG64: u32 = 0x40;
pub const PCM3060_REG_MRST: u32 = 0x80;
pub const PCM3060_REG_SRST: u32 = 0x40;
pub const PCM3060_REG_ADPSV: u32 = 0x20;
pub const PCM3060_REG_SHIFT_ADPSV: u32 = 0x05;
pub const PCM3060_REG_DAPSV: u32 = 0x10;
pub const PCM3060_REG_SHIFT_DAPSV: u32 = 0x04;
pub const PCM3060_REG_SE: u32 = 0x01;

pub const PCM3060_REG65: u32 = 0x41;
pub const PCM3060_REG66: u32 = 0x42;
pub const PCM3060_REG_AT2_MIN: u32 = 0x36;
pub const PCM3060_REG_AT2_MAX: u32 = 0xFF;

pub const PCM3060_REG67: u32 = 0x43;
pub const PCM3060_REG72: u32 = 0x48;
pub const PCM3060_REG_CSEL: u32 = 0x80;
pub const PCM3060_REG_MASK_MS: u32 = 0x70;
pub const PCM3060_REG_MS_S: u32 = 0x00;
pub const PCM3060_REG_MS_M768: u32 = 0x01 << 4;
pub const PCM3060_REG_MS_M512: u32 = 0x02 << 4;
pub const PCM3060_REG_MS_M384: u32 = 0x03 << 4;
pub const PCM3060_REG_MS_M256: u32 = 0x04 << 4;
pub const PCM3060_REG_MS_M192: u32 = 0x05 << 4;
pub const PCM3060_REG_MS_M128: u32 = 0x06 << 4;
pub const PCM3060_REG_MASK_FMT: u32 = 0x03;
pub const PCM3060_REG_FMT_I2S: u32 = 0x00;
pub const PCM3060_REG_FMT_LJ: u32 = 0x01;
pub const PCM3060_REG_FMT_RJ: u32 = 0x02;

pub const PCM3060_REG68: u32 = 0x44;
pub const PCM3060_REG_OVER: u32 = 0x40;
pub const PCM3060_REG_DREV2: u32 = 0x04;
pub const PCM3060_REG_SHIFT_MUT21: u32 = 0x00;
pub const PCM3060_REG_SHIFT_MUT22: u32 = 0x01;

pub const PCM3060_REG69: u32 = 0x45;
pub const PCM3060_REG_FLT: u32 = 0x80;
pub const PCM3060_REG_MASK_DMF: u32 = 0x60;
pub const PCM3060_REG_DMC: u32 = 0x10;
pub const PCM3060_REG_ZREV: u32 = 0x02;
pub const PCM3060_REG_AZRO: u32 = 0x01;

pub const PCM3060_REG70: u32 = 0x46;
pub const PCM3060_REG71: u32 = 0x47;
pub const PCM3060_REG_AT1_MIN: u32 = 0x0E;
pub const PCM3060_REG_AT1_MAX: u32 = 0xFF;

pub const PCM3060_REG73: u32 = 0x49;
pub const PCM3060_REG_ZCDD: u32 = 0x10;
pub const PCM3060_REG_BYP: u32 = 0x08;
pub const PCM3060_REG_DREV1: u32 = 0x04;
pub const PCM3060_REG_SHIFT_MUT11: u32 = 0x00;
pub const PCM3060_REG_SHIFT_MUT12: u32 = 0x01;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
