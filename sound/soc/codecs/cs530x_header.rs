/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CS530x CODEC driver internal data
 *
 * Copyright (C) 2023-2025 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

/* Dependencies from the original C header:
 * <linux/device.h>
 * <linux/gpio/consumer.h>
 * <linux/regmap.h>
 * <linux/regulator/consumer.h>
 */

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    _private: [u8; 0],
}

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

/* Devices */
pub const CS530X_2CH_CODEC_DEV_ID: u32 = 0x4282;
pub const CS530X_2CH_DAC_DEV_ID: u32 = 0x4302;
pub const CS530X_4CH_DAC_DEV_ID: u32 = 0x4304;
pub const CS530X_8CH_DAC_DEV_ID: u32 = 0x4308;
pub const CS530X_2CH_ADC_DEV_ID: u32 = 0x5302;
pub const CS530X_4CH_ADC_DEV_ID: u32 = 0x5304;
pub const CS530X_8CH_ADC_DEV_ID: u32 = 0x5308;

/* Registers */

pub const CS530X_DEVID: u32 = 0x0000000;
pub const CS530X_REVID: u32 = 0x0000004;
pub const CS530X_SW_RESET: u32 = 0x0000022;

pub const CS530X_CLK_CFG_0: u32 = 0x0000040;
pub const CS530X_CLK_CFG_1: u32 = 0x0000042;
pub const CS530X_CHIP_ENABLE: u32 = 0x0000044;
pub const CS530X_ASP_CFG: u32 = 0x0000048;
pub const CS530X_SIGNAL_PATH_CFG: u32 = 0x0000050;
pub const CS530X_IN_ENABLES: u32 = 0x0000080;
pub const CS530X_IN_RAMP_SUM: u32 = 0x0000082;
pub const CS530X_IN_FILTER: u32 = 0x0000086;
pub const CS530X_IN_HIZ: u32 = 0x0000088;
pub const CS530X_IN_INV: u32 = 0x000008A;
pub const CS530X_IN_VOL_CTRL1_0: u32 = 0x0000090;
pub const CS530X_IN_VOL_CTRL1_1: u32 = 0x0000092;
pub const CS530X_IN_VOL_CTRL2_0: u32 = 0x0000094;
pub const CS530X_IN_VOL_CTRL2_1: u32 = 0x0000096;
pub const CS530X_IN_VOL_CTRL3_0: u32 = 0x0000098;
pub const CS530X_IN_VOL_CTRL3_1: u32 = 0x000009A;
pub const CS530X_IN_VOL_CTRL4_0: u32 = 0x000009C;
pub const CS530X_IN_VOL_CTRL4_1: u32 = 0x000009E;
pub const CS530X_IN_VOL_CTRL5: u32 = 0x00000A0;

pub const CS530X_OUT_ENABLES: u32 = 0x00000C0;
pub const CS530X_OUT_RAMP_SUM: u32 = 0x00000C2;
pub const CS530X_OUT_DEEMPH: u32 = 0x00000C4;
pub const CS530X_OUT_FILTER: u32 = 0x00000C6;
pub const CS530X_OUT_INV: u32 = 0x00000CA;
pub const CS530X_OUT_VOL_CTRL1_0: u32 = 0x00000D0;
pub const CS530X_OUT_VOL_CTRL1_1: u32 = 0x00000D2;
pub const CS530X_OUT_VOL_CTRL2_0: u32 = 0x00000D4;
pub const CS530X_OUT_VOL_CTRL2_1: u32 = 0x00000D6;
pub const CS530X_OUT_VOL_CTRL3_0: u32 = 0x00000D8;
pub const CS530X_OUT_VOL_CTRL3_1: u32 = 0x00000DA;
pub const CS530X_OUT_VOL_CTRL4_0: u32 = 0x00000DC;
pub const CS530X_OUT_VOL_CTRL4_1: u32 = 0x00000DE;
pub const CS530X_OUT_VOL_CTRL5: u32 = 0x00000E0;

pub const CS530X_PAD_FN: u32 = 0x0003D24;
pub const CS530X_PAD_LVL: u32 = 0x0003D28;

pub const CS530X_MAX_REGISTER: u32 = CS530X_PAD_LVL;

/* Register Fields */

/* REVID */
pub const CS530X_MTLREVID: u32 = genmask(3, 0);
pub const CS530X_AREVID: u32 = genmask(7, 4);

/* SW_RESET */
pub const CS530X_SW_RST_SHIFT: u32 = 8;
pub const CS530X_SW_RST_VAL: u32 = 0x5A << CS530X_SW_RST_SHIFT;

/* CLK_CFG_0 */
pub const CS530X_PLL_REFCLK_SRC_MASK: u32 = bit(0);
pub const CS530X_PLL_REFCLK_FREQ_MASK: u32 = genmask(5, 4);
pub const CS530X_SYSCLK_SRC_MASK: u32 = bit(12);
pub const CS530X_SYSCLK_SRC_SHIFT: u32 = 12;
pub const CS530X_REFCLK_2P822_3P072: u32 = 0;
pub const CS530X_REFCLK_5P6448_6P144: u32 = 0x10;
pub const CS530X_REFCLK_11P2896_12P288: u32 = 0x20;
pub const CS530X_REFCLK_24P5792_24P576: u32 = 0x30;

/* CLK_CFG_1 */
pub const CS530X_SAMPLE_RATE_MASK: u32 = genmask(2, 0);
pub const CS530X_FS_32K: u32 = 0;
pub const CS530X_FS_44P1K_48K: u32 = 1;
pub const CS530X_FS_88P2K_96K: u32 = 2;
pub const CS530X_FS_176P4K_192K: u32 = 3;
pub const CS530X_FS_356P8K_384K: u32 = 4;
pub const CS530X_FS_705P6K_768K: u32 = 5;

/* CHIP_ENABLE */
pub const CS530X_GLOBAL_EN: u32 = bit(0);

/* ASP_CFG */
pub const CS530X_ASP_BCLK_FREQ_MASK: u32 = genmask(1, 0);
pub const CS530X_ASP_PRIMARY: u32 = bit(5);
pub const CS530X_ASP_BCLK_INV: u32 = bit(6);
pub const CS530X_BCLK_2P822_3P072: u32 = 0;
pub const CS530X_BCLK_5P6448_6P144: u32 = 1;
pub const CS530X_BCLK_11P2896_12P288: u32 = 2;
pub const CS530X_BCLK_24P5792_24P576: u32 = 3;

/* SIGNAL_PATH_CFG */
pub const CS530X_ASP_FMT_MASK: u32 = genmask(2, 0);
pub const CS530X_ASP_TDM_SLOT_MASK: u32 = genmask(5, 3);
pub const CS530X_ASP_TDM_SLOT_SHIFT: u32 = 3;
pub const CS530X_ASP_CH_REVERSE: u32 = bit(9);
pub const CS530X_TDM_EN_MASK: u32 = bit(2);
pub const CS530X_ASP_FMT_I2S: u32 = 0;
pub const CS530X_ASP_FMT_LJ: u32 = 1;
pub const CS530X_ASP_FMT_DSP_A: u32 = 6;

/* TDM Slots */
pub const CS530X_0_1_TDM_SLOT_MASK: u32 = genmask(1, 0);
pub const CS530X_0_3_TDM_SLOT_MASK: u32 = genmask(3, 0);
pub const CS530X_0_7_TDM_SLOT_MASK: u32 = genmask(7, 0);
pub const CS530X_0_7_TDM_SLOT_VAL: u32 = 0;

pub const CS530X_2_3_TDM_SLOT_MASK: u32 = genmask(3, 2);
pub const CS530X_2_3_TDM_SLOT_VAL: u32 = 1;

pub const CS530X_4_5_TDM_SLOT_MASK: u32 = genmask(5, 4);
pub const CS530X_4_7_TDM_SLOT_MASK: u32 = genmask(7, 4);
pub const CS530X_4_7_TDM_SLOT_VAL: u32 = 2;

pub const CS530X_6_7_TDM_SLOT_MASK: u32 = genmask(7, 6);
pub const CS530X_6_7_TDM_SLOT_VAL: u32 = 3;

pub const CS530X_8_9_TDM_SLOT_MASK: u32 = genmask(9, 8);
pub const CS530X_8_11_TDM_SLOT_MASK: u32 = genmask(11, 8);
pub const CS530X_8_15_TDM_SLOT_MASK: u32 = genmask(15, 8);
pub const CS530X_8_15_TDM_SLOT_VAL: u32 = 4;

pub const CS530X_10_11_TDM_SLOT_MASK: u32 = genmask(11, 10);
pub const CS530X_10_11_TDM_SLOT_VAL: u32 = 5;

pub const CS530X_12_13_TDM_SLOT_MASK: u32 = genmask(13, 12);
pub const CS530X_12_15_TDM_SLOT_MASK: u32 = genmask(15, 12);
pub const CS530X_12_15_TDM_SLOT_VAL: u32 = 6;

pub const CS530X_14_15_TDM_SLOT_MASK: u32 = genmask(15, 14);
pub const CS530X_14_15_TDM_SLOT_VAL: u32 = 7;

/* IN_RAMP_SUM and OUT_RAMP_SUM */
pub const CS530X_RAMP_RATE_INC_SHIFT: u32 = 0;
pub const CS530X_RAMP_RATE_DEC_SHIFT: u32 = 4;
pub const CS530X_INOUT_SUM_MODE_SHIFT: u32 = 13;

/* IN_FILTER and OUT_FILTER */
pub const CS530X_INOUT_FILTER_SHIFT: u32 = 8;
pub const CS530X_INOUT_HPF_EN_SHIFT: u32 = 12;

/* IN_HIZ */
pub const CS530X_IN12_HIZ: u32 = bit(0);
pub const CS530X_IN34_HIZ: u32 = bit(1);
pub const CS530X_IN56_HIZ: u32 = bit(2);
pub const CS530X_IN78_HIZ: u32 = bit(3);

/* IN_INV and OUT_INV */
pub const CS530X_INOUT1_INV_SHIFT: u32 = 0;
pub const CS530X_INOUT2_INV_SHIFT: u32 = 1;
pub const CS530X_INOUT3_INV_SHIFT: u32 = 2;
pub const CS530X_INOUT4_INV_SHIFT: u32 = 3;
pub const CS530X_INOUT5_INV_SHIFT: u32 = 4;
pub const CS530X_INOUT6_INV_SHIFT: u32 = 5;
pub const CS530X_INOUT7_INV_SHIFT: u32 = 6;
pub const CS530X_INOUT8_INV_SHIFT: u32 = 7;

/* IN_VOL_CTLy_z and OUT_VOL_CTLy_z */
pub const CS530X_INOUT_MUTE: u32 = bit(15);

/* IN_VOL_CTL5 */
pub const CS530X_IN_VU: u32 = bit(0);

/* PAD_FN */
pub const CS530X_DOUT2_FN: u32 = bit(0);
pub const CS530X_DOUT3_FN: u32 = bit(1);
pub const CS530X_DOUT4_FN: u32 = bit(2);
pub const CS530X_SPI_CS_FN: u32 = bit(3);
pub const CS530X_CONFIG2_FN: u32 = bit(6);
pub const CS530X_CONFIG3_FN: u32 = bit(7);
pub const CS530X_CONFIG4_FN: u32 = bit(8);
pub const CS530X_CONFIG5_FN: u32 = bit(9);

/* PAD_LVL */
pub const CS530X_CONFIG2_LVL: u32 = bit(6);
pub const CS530X_CONFIG3_LVL: u32 = bit(7);
pub const CS530X_CONFIG4_LVL: u32 = bit(8);
pub const CS530X_CONFIG5_LVL: u32 = bit(9);
/* IN_VOL_CTL5 and OUT_VOL_CTL5 */
pub const CS530X_INOUT_VU: u32 = bit(0);

/* System Clock Source */
pub const CS530X_SYSCLK_SRC_MCLK: u32 = 0;
pub const CS530X_SYSCLK_SRC_PLL: u32 = 1;

/* PLL Reference Clock Source */
pub const CS530X_PLL_SRC_BCLK: u32 = 0;
pub const CS530X_PLL_SRC_MCLK: u32 = 1;

pub const CS530X_NUM_SUPPLIES: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs530x_type {
    CS4282 = CS530X_2CH_CODEC_DEV_ID as isize,
    CS4302 = CS530X_2CH_DAC_DEV_ID as isize,
    CS4304 = CS530X_4CH_DAC_DEV_ID as isize,
    CS4308 = CS530X_8CH_DAC_DEV_ID as isize,
    CS5302 = CS530X_2CH_ADC_DEV_ID as isize,
    CS5304 = CS530X_4CH_ADC_DEV_ID as isize,
    CS5308 = CS530X_8CH_ADC_DEV_ID as isize,
}

/* codec private data */
#[repr(C)]
pub struct cs530x_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub dev_dai: *mut snd_soc_dai_driver,

    pub devtype: cs530x_type,
    pub num_adcs: ::std::os::raw::c_int,
    pub num_dacs: ::std::os::raw::c_int,

    pub supplies: [regulator_bulk_data; CS530X_NUM_SUPPLIES],

    pub tdm_width: ::std::os::raw::c_int,
    pub tdm_slots: ::std::os::raw::c_int,
    pub adc_pairs_count: ::std::os::raw::c_int,
    pub dac_pairs_count: ::std::os::raw::c_int,

    pub reset_gpio: *mut gpio_desc,
}

unsafe extern "C" {
    pub static cs530x_regmap_i2c: regmap_config;
    pub static cs530x_regmap_spi: regmap_config;
    pub fn cs530x_probe(cs530x: *mut cs530x_priv) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
