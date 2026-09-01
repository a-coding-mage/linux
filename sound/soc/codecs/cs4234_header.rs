/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC Audio driver for CS4234 codec
 *
 * Copyright (C) 2020 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

pub const CS4234_H: u32 = ;
pub const CS4234_DEVID_AB: u32 = 0x01;
pub const CS4234_DEVID_CD: u32 = 0x02;
pub const CS4234_DEVID_EF: u32 = 0x03;
pub const CS4234_REVID: u32 = 0x05;
pub const CS4234_CLOCK_SP: u32 = 0x06;
pub const CS4234_BASE_RATE_MASK: u32 = 0xC0;
pub const CS4234_BASE_RATE_SHIFT: u32 = 6;
pub const CS4234_SPEED_MODE_MASK: u32 = 0x30;
pub const CS4234_SPEED_MODE_SHIFT: u32 = 4;
pub const CS4234_MCLK_RATE_MASK: u32 = 0x0E;
pub const CS4234_MCLK_RATE_SHIFT: u32 = 1;
pub const CS4234_SAMPLE_WIDTH: u32 = 0x07;
pub const CS4234_SDOUTX_SW_MASK: u32 = 0xC0;
pub const CS4234_SDOUTX_SW_SHIFT: u32 = 6;
pub const CS4234_INPUT_SW_MASK: u32 = 0x30;
pub const CS4234_INPUT_SW_SHIFT: u32 = 4;
pub const CS4234_LOW_LAT_SW_MASK: u32 = 0x0C;
pub const CS4234_LOW_LAT_SW_SHIFT: u32 = 2;
pub const CS4234_DAC5_SW_MASK: u32 = 0x03;
pub const CS4234_DAC5_SW_SHIFT: u32 = 0;
pub const CS4234_SP_CTRL: u32 = 0x08;
pub const CS4234_INVT_SCLK_MASK: u32 = 0x80;
pub const CS4234_INVT_SCLK_SHIFT: u32 = 7;
pub const CS4234_DAC5_SRC_MASK: u32 = 0x70;
pub const CS4234_DAC5_SRC_SHIFT: u32 = 4;
pub const CS4234_SP_FORMAT_MASK: u32 = 0x0C;
pub const CS4234_SP_FORMAT_SHIFT: u32 = 2;
pub const CS4234_SDO_CHAIN_MASK: u32 = 0x02;
pub const CS4234_SDO_CHAIN_SHIFT: u32 = 1;
pub const CS4234_MST_SLV_MASK: u32 = 0x01;
pub const CS4234_MST_SLV_SHIFT: u32 = 0;
pub const CS4234_SP_DATA_SEL: u32 = 0x09;
pub const CS4234_DAC14_SRC_MASK: u32 = 0x38;
pub const CS4234_DAC14_SRC_SHIFT: u32 = 3;
pub const CS4234_LL_SRC_MASK: u32 = 0x07;
pub const CS4234_LL_SRC_SHIFT: u32 = 0;
pub const CS4234_SDIN1_MASK1: u32 = 0x0A;
pub const CS4234_SDIN1_MASK2: u32 = 0x0B;
pub const CS4234_SDIN2_MASK1: u32 = 0x0C;
pub const CS4234_SDIN2_MASK2: u32 = 0x0D;
pub const CS4234_TPS_CTRL: u32 = 0x0E;
pub const CS4234_TPS_MODE_MASK: u32 = 0x80;
pub const CS4234_TPS_MODE_SHIFT: u32 = 7;
pub const CS4234_TPS_OFST_MASK: u32 = 0x70;
pub const CS4234_TPS_OFST_SHIFT: u32 = 4;
pub const CS4234_GRP_DELAY_MASK: u32 = 0x0F;
pub const CS4234_GRP_DELAY_SHIFT: u32 = 0;
pub const CS4234_ADC_CTRL1: u32 = 0x0F;
pub const CS4234_VA_SEL_MASK: u32 = 0x20;
pub const CS4234_VA_SEL_SHIFT: u32 = 5;
pub const CS4234_ENA_HPF_MASK: u32 = 0x10;
pub const CS4234_ENA_HPF_SHIFT: u32 = 4;
pub const CS4234_INV_ADC_MASK: u32 = 0x0F;
pub const CS4234_INV_ADC4_MASK: u32 = 0x08;
pub const CS4234_INV_ADC4_SHIFT: u32 = 3;
pub const CS4234_INV_ADC3_MASK: u32 = 0x04;
pub const CS4234_INV_ADC3_SHIFT: u32 = 2;
pub const CS4234_INV_ADC2_MASK: u32 = 0x02;
pub const CS4234_INV_ADC2_SHIFT: u32 = 1;
pub const CS4234_INV_ADC1_MASK: u32 = 0x01;
pub const CS4234_INV_ADC1_SHIFT: u32 = 0;
pub const CS4234_ADC_CTRL2: u32 = 0x10;
pub const CS4234_MUTE_ADC4_MASK: u32 = 0x80;
pub const CS4234_MUTE_ADC4_SHIFT: u32 = 7;
pub const CS4234_MUTE_ADC3_MASK: u32 = 0x40;
pub const CS4234_MUTE_ADC3_SHIFT: u32 = 6;
pub const CS4234_MUTE_ADC2_MASK: u32 = 0x20;
pub const CS4234_MUTE_ADC2_SHIFT: u32 = 5;
pub const CS4234_MUTE_ADC1_MASK: u32 = 0x10;
pub const CS4234_MUTE_ADC1_SHIFT: u32 = 4;
pub const CS4234_PDN_ADC4_MASK: u32 = 0x08;
pub const CS4234_PDN_ADC4_SHIFT: u32 = 3;
pub const CS4234_PDN_ADC3_MASK: u32 = 0x04;
pub const CS4234_PDN_ADC3_SHIFT: u32 = 2;
pub const CS4234_PDN_ADC2_MASK: u32 = 0x02;
pub const CS4234_PDN_ADC2_SHIFT: u32 = 1;
pub const CS4234_PDN_ADC1_MASK: u32 = 0x01;
pub const CS4234_PDN_ADC1_SHIFT: u32 = 0;
pub const CS4234_LOW_LAT_CTRL1: u32 = 0x11;
pub const CS4234_LL_NG_MASK: u32 = 0xE0;
pub const CS4234_LL_NG_SHIFT: u32 = 5;
pub const CS4234_INV_LL_MASK: u32 = 0x0F;
pub const CS4234_INV_LL4_MASK: u32 = 0x08;
pub const CS4234_INV_LL4_SHIFT: u32 = 3;
pub const CS4234_INV_LL3_MASK: u32 = 0x04;
pub const CS4234_INV_LL3_SHIFT: u32 = 2;
pub const CS4234_INV_LL2_MASK: u32 = 0x02;
pub const CS4234_INV_LL2_SHIFT: u32 = 1;
pub const CS4234_INV_LL1_MASK: u32 = 0x01;
pub const CS4234_INV_LL1_SHIFT: u32 = 0;
pub const CS4234_DAC_CTRL1: u32 = 0x12;
pub const CS4234_DAC14_NG_MASK: u32 = 0xE0;
pub const CS4234_DAC14_NG_SHIFT: u32 = 5;
pub const CS4234_DAC14_DE_MASK: u32 = 0x10;
pub const CS4234_DAC14_DE_SHIFT: u32 = 4;
pub const CS4234_DAC5_DE_MASK: u32 = 0x08;
pub const CS4234_DAC5_DE_SHIFT: u32 = 3;
pub const CS4234_DAC5_MVC_MASK: u32 = 0x04;
pub const CS4234_DAC5_MVC_SHIFT: u32 = 2;
pub const CS4234_DAC5_CFG_FLTR_MASK: u32 = 0x03;
pub const CS4234_DAC5_CFG_FLTR_SHIFT: u32 = 0;
pub const CS4234_DAC_CTRL2: u32 = 0x13;
pub const CS4234_DAC5_NG_MASK: u32 = 0xE0;
pub const CS4234_DAC5_NG_SHIFT: u32 = 5;
pub const CS4234_INV_DAC_MASK: u32 = 0x1F;
pub const CS4234_INV_DAC5_MASK: u32 = 0x10;
pub const CS4234_INV_DAC5_SHIFT: u32 = 4;
pub const CS4234_INV_DAC4_MASK: u32 = 0x08;
pub const CS4234_INV_DAC4_SHIFT: u32 = 3;
pub const CS4234_INV_DAC3_MASK: u32 = 0x04;
pub const CS4234_INV_DAC3_SHIFT: u32 = 2;
pub const CS4234_INV_DAC2_MASK: u32 = 0x02;
pub const CS4234_INV_DAC2_SHIFT: u32 = 1;
pub const CS4234_INV_DAC1_MASK: u32 = 0x01;
pub const CS4234_INV_DAC1_SHIFT: u32 = 0;
pub const CS4234_DAC_CTRL3: u32 = 0x14;
pub const CS4234_DAC5_ATT_MASK: u32 = 0x80;
pub const CS4234_DAC5_ATT_SHIFT: u32 = 7;
pub const CS4234_DAC14_ATT_MASK: u32 = 0x40;
pub const CS4234_DAC14_ATT_SHIFT: u32 = 6;
pub const CS4234_MUTE_LL_MASK: u32 = 0x20;
pub const CS4234_MUTE_LL_SHIFT: u32 = 5;
pub const CS4234_MUTE_DAC5_MASK: u32 = 0x10;
pub const CS4234_MUTE_DAC5_SHIFT: u32 = 4;
pub const CS4234_MUTE_DAC4_MASK: u32 = 0x08;
pub const CS4234_MUTE_DAC4_SHIFT: u32 = 3;
pub const CS4234_MUTE_DAC3_MASK: u32 = 0x04;
pub const CS4234_MUTE_DAC3_SHIFT: u32 = 2;
pub const CS4234_MUTE_DAC2_MASK: u32 = 0x02;
pub const CS4234_MUTE_DAC2_SHIFT: u32 = 1;
pub const CS4234_MUTE_DAC1_MASK: u32 = 0x01;
pub const CS4234_MUTE_DAC1_SHIFT: u32 = 0;
pub const CS4234_DAC_CTRL4: u32 = 0x15;
pub const CS4234_VQ_RAMP_MASK: u32 = 0x80;
pub const CS4234_VQ_RAMP_SHIFT: u32 = 7;
pub const CS4234_TPS_GAIN_MASK: u32 = 0x40;
pub const CS4234_TPS_GAIN_SHIFT: u32 = 6;
pub const CS4234_PDN_DAC5_MASK: u32 = 0x10;
pub const CS4234_PDN_DAC5_SHIFT: u32 = 4;
pub const CS4234_PDN_DAC4_MASK: u32 = 0x08;
pub const CS4234_PDN_DAC4_SHIFT: u32 = 3;
pub const CS4234_PDN_DAC3_MASK: u32 = 0x04;
pub const CS4234_PDN_DAC3_SHIFT: u32 = 2;
pub const CS4234_PDN_DAC2_MASK: u32 = 0x02;
pub const CS4234_PDN_DAC2_SHIFT: u32 = 1;
pub const CS4234_PDN_DAC1_MASK: u32 = 0x01;
pub const CS4234_PDN_DAC1_SHIFT: u32 = 0;
pub const CS4234_VOLUME_MODE: u32 = 0x16;
pub const CS4234_MUTE_DELAY_MASK: u32 = 0xC0;
pub const CS4234_MUTE_DELAY_SHIFT: u32 = 6;
pub const CS4234_MIN_DELAY_MASK: u32 = 0x38;
pub const CS4234_MIN_DELAY_SHIFT: u32 = 3;
pub const CS4234_MAX_DELAY_MASK: u32 = 0x07;
pub const CS4234_MAX_DELAY_SHIFT: u32 = 0;
pub const CS4234_MASTER_VOL: u32 = 0x17;
pub const CS4234_DAC1_VOL: u32 = 0x18;
pub const CS4234_DAC2_VOL: u32 = 0x19;
pub const CS4234_DAC3_VOL: u32 = 0x1A;
pub const CS4234_DAC4_VOL: u32 = 0x1B;
pub const CS4234_DAC5_VOL: u32 = 0x1C;
pub const CS4234_INT_CTRL: u32 = 0x1E;
pub const CS4234_INT_MODE_MASK: u32 = 0x80;
pub const CS4234_INT_MODE_SHIFT: u32 = 7;
pub const CS4234_INT_PIN_MASK: u32 = 0x60;
pub const CS4234_INT_PIN_SHIFT: u32 = 5;
pub const CS4234_INT_MASK1: u32 = 0x1F;
pub const CS4234_MSK_TST_MODE_MASK: u32 = 0x80;
pub const CS4234_MSK_TST_MODE_ERR_SHIFT: u32 = 7;
pub const CS4234_MSK_SP_ERR_MASK: u32 = 0x40;
pub const CS4234_MSK_SP_ERR_SHIFT: u32 = 6;
pub const CS4234_MSK_CLK_ERR_MASK: u32 = 0x08;
pub const CS4234_MSK_CLK_ERR_SHIFT: u32 = 5;
pub const CS4234_MSK_ADC4_OVFL_MASK: u32 = 0x08;
pub const CS4234_MSK_ADC4_OVFL_SHIFT: u32 = 3;
pub const CS4234_MSK_ADC3_OVFL_MASK: u32 = 0x04;
pub const CS4234_MSK_ADC3_OVFL_SHIFT: u32 = 2;
pub const CS4234_MSK_ADC2_OVFL_MASK: u32 = 0x02;
pub const CS4234_MSK_ADC2_OVFL_SHIFT: u32 = 1;
pub const CS4234_MSK_ADC1_OVFL_MASK: u32 = 0x01;
pub const CS4234_MSK_ADC1_OVFL_SHIFT: u32 = 0;
pub const CS4234_INT_MASK2: u32 = 0x20;
pub const CS4234_MSK_DAC5_CLIP_MASK: u32 = 0x10;
pub const CS4234_MSK_DAC5_CLIP_SHIFT: u32 = 4;
pub const CS4234_MSK_DAC4_CLIP_MASK: u32 = 0x08;
pub const CS4234_MSK_DAC4_CLIP_SHIFT: u32 = 3;
pub const CS4234_MSK_DAC3_CLIP_MASK: u32 = 0x04;
pub const CS4234_MSK_DAC3_CLIP_SHIFT: u32 = 2;
pub const CS4234_MSK_DAC2_CLIP_MASK: u32 = 0x02;
pub const CS4234_MSK_DAC2_CLIP_SHIFT: u32 = 1;
pub const CS4234_MSK_DAC1_CLIP_MASK: u32 = 0x01;
pub const CS4234_MSK_DAC1_CLIP_SHIFT: u32 = 0;
pub const CS4234_INT_NOTIFY1: u32 = 0x21;
pub const CS4234_TST_MODE_MASK: u32 = 0x80;
pub const CS4234_TST_MODE_SHIFT: u32 = 7;
pub const CS4234_SP_ERR_MASK: u32 = 0x40;
pub const CS4234_SP_ERR_SHIFT: u32 = 6;
pub const CS4234_CLK_MOD_ERR_MASK: u32 = 0x08;
pub const CS4234_CLK_MOD_ERR_SHIFT: u32 = 5;
pub const CS4234_ADC4_OVFL_MASK: u32 = 0x08;
pub const CS4234_ADC4_OVFL_SHIFT: u32 = 3;
pub const CS4234_ADC3_OVFL_MASK: u32 = 0x04;
pub const CS4234_ADC3_OVFL_SHIFT: u32 = 2;
pub const CS4234_ADC2_OVFL_MASK: u32 = 0x02;
pub const CS4234_ADC2_OVFL_SHIFT: u32 = 1;
pub const CS4234_ADC1_OVFL_MASK: u32 = 0x01;
pub const CS4234_ADC1_OVFL_SHIFT: u32 = 0;
pub const CS4234_INT_NOTIFY2: u32 = 0x22;
pub const CS4234_DAC5_CLIP_MASK: u32 = 0x10;
pub const CS4234_DAC5_CLIP_SHIFT: u32 = 4;
pub const CS4234_DAC4_CLIP_MASK: u32 = 0x08;
pub const CS4234_DAC4_CLIP_SHIFT: u32 = 3;
pub const CS4234_DAC3_CLIP_MASK: u32 = 0x04;
pub const CS4234_DAC3_CLIP_SHIFT: u32 = 2;
pub const CS4234_DAC2_CLIP_MASK: u32 = 0x02;
pub const CS4234_DAC2_CLIP_SHIFT: u32 = 1;
pub const CS4234_DAC1_CLIP_MASK: u32 = 0x01;
pub const CS4234_DAC1_CLIP_SHIFT: u32 = 0;
pub const CS4234_MAX_REGISTER: u32 = CS4234_INT_NOTIFY2;
pub const CS4234_SUPPORTED_ID: u32 = 0x423400;
pub const CS4234_BOOT_TIME_US: u32 = 3000;
pub const CS4234_HOLD_RESET_TIME_US: u32 = 1000;
pub const CS4234_VQ_CHARGE_MS: u32 = 1000;
pub const CS4234_PCM_RATES: u32 = SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |  SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_64000 | ;
pub const CS4234_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S18_3LE |  SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | ;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cs4234_supplies {
    CS4234_SUPPLY_VA = 0,
    CS4234_SUPPLY_VL,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cs4234_va_sel {
    CS4234_3V3 = 0,
    CS4234_5V,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cs4234_sp_format {
    CS4234_LEFT_J = 0,
    CS4234_I2S,
    CS4234_TDM,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cs4234_base_rate_advisory {
    CS4234_48K = 0,
    CS4234_44K1,
    CS4234_32K,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
