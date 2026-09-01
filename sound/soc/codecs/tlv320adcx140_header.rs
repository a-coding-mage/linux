// SPDX-License-Identifier: GPL-2.0
// TLV320ADCX140 Sound driver
// Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com/

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h >= 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !(BIT(l) - 1)
    }
}

pub const ADCX140_RATES: u32 = SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;

pub const ADCX140_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

pub const ADCX140_PAGE_SELECT: u32 = 0x00;
pub const ADCX140_SW_RESET: u32 = 0x01;
pub const ADCX140_SLEEP_CFG: u32 = 0x02;
pub const ADCX140_SHDN_CFG: u32 = 0x05;
pub const ADCX140_ASI_CFG0: u32 = 0x07;
pub const ADCX140_ASI_CFG1: u32 = 0x08;
pub const ADCX140_ASI_CFG2: u32 = 0x09;
pub const ADCX140_ASI_CH1: u32 = 0x0b;
pub const ADCX140_ASI_CH2: u32 = 0x0c;
pub const ADCX140_ASI_CH3: u32 = 0x0d;
pub const ADCX140_ASI_CH4: u32 = 0x0e;
pub const ADCX140_ASI_CH5: u32 = 0x0f;
pub const ADCX140_ASI_CH6: u32 = 0x10;
pub const ADCX140_ASI_CH7: u32 = 0x11;
pub const ADCX140_ASI_CH8: u32 = 0x12;
pub const ADCX140_MST_CFG0: u32 = 0x13;
pub const ADCX140_MST_CFG1: u32 = 0x14;
pub const ADCX140_ASI_STS: u32 = 0x15;
pub const ADCX140_CLK_SRC: u32 = 0x16;
pub const ADCX140_PDMCLK_CFG: u32 = 0x1f;
pub const ADCX140_PDM_CFG: u32 = 0x20;
pub const ADCX140_GPIO_CFG0: u32 = 0x21;
pub const ADCX140_GPO_CFG0: u32 = 0x22;
pub const ADCX140_GPO_CFG1: u32 = 0x23;
pub const ADCX140_GPO_CFG2: u32 = 0x24;
pub const ADCX140_GPO_CFG3: u32 = 0x25;
pub const ADCX140_GPO_VAL: u32 = 0x29;
pub const ADCX140_GPIO_MON: u32 = 0x2a;
pub const ADCX140_GPI_CFG0: u32 = 0x2b;
pub const ADCX140_GPI_CFG1: u32 = 0x2c;
pub const ADCX140_GPI_MON: u32 = 0x2f;
pub const ADCX140_INT_CFG: u32 = 0x32;
pub const ADCX140_INT_MASK0: u32 = 0x33;
pub const ADCX140_INT_LTCH0: u32 = 0x36;
pub const ADCX140_BIAS_CFG: u32 = 0x3b;
pub const ADCX140_CH1_CFG0: u32 = 0x3c;
pub const ADCX140_CH1_CFG1: u32 = 0x3d;
pub const ADCX140_CH1_CFG2: u32 = 0x3e;
pub const ADCX140_CH1_CFG3: u32 = 0x3f;
pub const ADCX140_CH1_CFG4: u32 = 0x40;
pub const ADCX140_CH2_CFG0: u32 = 0x41;
pub const ADCX140_CH2_CFG1: u32 = 0x42;
pub const ADCX140_CH2_CFG2: u32 = 0x43;
pub const ADCX140_CH2_CFG3: u32 = 0x44;
pub const ADCX140_CH2_CFG4: u32 = 0x45;
pub const ADCX140_CH3_CFG0: u32 = 0x46;
pub const ADCX140_CH3_CFG1: u32 = 0x47;
pub const ADCX140_CH3_CFG2: u32 = 0x48;
pub const ADCX140_CH3_CFG3: u32 = 0x49;
pub const ADCX140_CH3_CFG4: u32 = 0x4a;
pub const ADCX140_CH4_CFG0: u32 = 0x4b;
pub const ADCX140_CH4_CFG1: u32 = 0x4c;
pub const ADCX140_CH4_CFG2: u32 = 0x4d;
pub const ADCX140_CH4_CFG3: u32 = 0x4e;
pub const ADCX140_CH4_CFG4: u32 = 0x4f;
pub const ADCX140_CH5_CFG2: u32 = 0x52;
pub const ADCX140_CH5_CFG3: u32 = 0x53;
pub const ADCX140_CH5_CFG4: u32 = 0x54;
pub const ADCX140_CH6_CFG2: u32 = 0x57;
pub const ADCX140_CH6_CFG3: u32 = 0x58;
pub const ADCX140_CH6_CFG4: u32 = 0x59;
pub const ADCX140_CH7_CFG2: u32 = 0x5c;
pub const ADCX140_CH7_CFG3: u32 = 0x5d;
pub const ADCX140_CH7_CFG4: u32 = 0x5e;
pub const ADCX140_CH8_CFG2: u32 = 0x61;
pub const ADCX140_CH8_CFG3: u32 = 0x62;
pub const ADCX140_CH8_CFG4: u32 = 0x63;
pub const ADCX140_DSP_CFG0: u32 = 0x6b;
pub const ADCX140_DSP_CFG1: u32 = 0x6c;
pub const ADCX140_DRE_CFG0: u32 = 0x6d;
pub const ADCX140_AGC_CFG0: u32 = 0x70;
pub const ADCX140_IN_CH_EN: u32 = 0x73;
pub const ADCX140_ASI_OUT_CH_EN: u32 = 0x74;
pub const ADCX140_PWR_CFG: u32 = 0x75;
pub const ADCX140_DEV_STS0: u32 = 0x76;
pub const ADCX140_DEV_STS1: u32 = 0x77;
pub const ADCX140_PHASE_CALIB: u32 = 0x7b;

pub const ADCX140_RESET: u32 = BIT(0);

pub const ADCX140_WAKE_DEV: u32 = BIT(0);
pub const ADCX140_AREG_INTERNAL: u32 = BIT(7);

pub const ADCX140_BCLKINV_BIT: u32 = BIT(2);
pub const ADCX140_FSYNCINV_BIT: u32 = BIT(3);
pub const ADCX140_INV_MSK: u32 = ADCX140_BCLKINV_BIT | ADCX140_FSYNCINV_BIT;
pub const ADCX140_BCLK_FSYNC_MASTER: u32 = BIT(7);
pub const ADCX140_I2S_MODE_BIT: u32 = BIT(6);
pub const ADCX140_LEFT_JUST_BIT: u32 = BIT(7);
pub const ADCX140_ASI_FORMAT_MSK: u32 = ADCX140_I2S_MODE_BIT | ADCX140_LEFT_JUST_BIT;

pub const ADCX140_16_BIT_WORD: u32 = 0x0;
pub const ADCX140_20_BIT_WORD: u32 = BIT(4);
pub const ADCX140_24_BIT_WORD: u32 = BIT(5);
pub const ADCX140_32_BIT_WORD: u32 = BIT(4) | BIT(5);
pub const ADCX140_WORD_LEN_MSK: u32 = 0x30;

pub const ADCX140_MAX_CHANNELS: u32 = 8;

pub const ADCX140_MIC_BIAS_VAL_VREF: u32 = 0;
pub const ADCX140_MIC_BIAS_VAL_VREF_1096: u32 = 1;
pub const ADCX140_MIC_BIAS_VAL_AVDD: u32 = 6;
pub const ADCX140_MIC_BIAS_VAL_MSK: u32 = GENMASK(6, 4);
pub const ADCX140_MIC_BIAS_SHIFT: u32 = 4;

pub const ADCX140_MIC_BIAS_VREF_275V: u32 = 0;
pub const ADCX140_MIC_BIAS_VREF_25V: u32 = 1;
pub const ADCX140_MIC_BIAS_VREF_1375V: u32 = 2;
pub const ADCX140_MIC_BIAS_VREF_MSK: u32 = GENMASK(1, 0);

pub const ADCX140_PWR_CTRL_MSK: u32 = GENMASK(7, 5);
pub const ADCX140_PWR_CFG_BIAS_PDZ: u32 = BIT(7);
pub const ADCX140_PWR_CFG_ADC_PDZ: u32 = BIT(6);
pub const ADCX140_PWR_CFG_PLL_PDZ: u32 = BIT(5);

pub const ADCX140_TX_OFFSET_MASK: u32 = GENMASK(4, 0);

pub const ADCX140_NUM_PDM_EDGES: u32 = 4;
pub const ADCX140_PDM_EDGE_SHIFT: u32 = 7;

pub const ADCX140_NUM_GPI_PINS: u32 = 4;
pub const ADCX140_GPI_SHIFT: u32 = 4;
pub const ADCX140_GPI1_INDEX: u32 = 0;
pub const ADCX140_GPI2_INDEX: u32 = 1;
pub const ADCX140_GPI3_INDEX: u32 = 2;
pub const ADCX140_GPI4_INDEX: u32 = 3;

pub const ADCX140_NUM_GPOS: u32 = 4;
pub const ADCX140_NUM_GPO_CFGS: u32 = 2;
pub const ADCX140_GPO_SHIFT: u32 = 4;
pub const ADCX140_GPO_CFG_MAX: u32 = 4;
pub const ADCX140_GPO_DRV_MAX: u32 = 5;

pub const ADCX140_TX_FILL: u32 = BIT(0);

pub const ADCX140_NUM_GPIO_CFGS: u32 = 2;
pub const ADCX140_GPIO_SHIFT: u32 = 4;
pub const ADCX140_GPIO_CFG_MAX: u32 = 15;
pub const ADCX140_GPIO_DRV_MAX: u32 = 5;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
