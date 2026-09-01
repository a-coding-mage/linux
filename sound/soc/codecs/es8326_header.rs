/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * es8326.h -- es8326 ALSA SoC audio driver
 * Copyright Everest Semiconductor Co.,Ltd
 *
 * Authors: David Yang <yangxiaohua@everest-semi.com>
 */

/* ES8326 register space */
pub const ES8326_RESET: u32 = 0x00;
pub const ES8326_CLK_CTL: u32 = 0x01;
pub const ES8326_CLK_INV: u32 = 0x02;
pub const ES8326_CLK_RESAMPLE: u32 = 0x03;
pub const ES8326_CLK_DIV1: u32 = 0x04;
pub const ES8326_CLK_DIV2: u32 = 0x05;
pub const ES8326_CLK_DLL: u32 = 0x06;
pub const ES8326_CLK_MUX: u32 = 0x07;
pub const ES8326_CLK_ADC_SEL: u32 = 0x08;
pub const ES8326_CLK_DAC_SEL: u32 = 0x09;
pub const ES8326_CLK_ADC_OSR: u32 = 0x0a;
pub const ES8326_CLK_DAC_OSR: u32 = 0x0b;
pub const ES8326_CLK_DIV_CPC: u32 = 0x0c;
pub const ES8326_CLK_DIV_BCLK: u32 = 0x0d;
pub const ES8326_CLK_TRI: u32 = 0x0e;
pub const ES8326_CLK_DIV_LRCK: u32 = 0x0f;
pub const ES8326_CLK_VMIDS1: u32 = 0x10;
pub const ES8326_CLK_VMIDS2: u32 = 0x11;
pub const ES8326_CLK_CAL_TIME: u32 = 0x12;
pub const ES8326_FMT: u32 = 0x13;

pub const ES8326_DAC_MUTE: u32 = 0x14;
pub const ES8326_ADC_MUTE: u32 = 0x15;
pub const ES8326_ANA_PDN: u32 = 0x16;
pub const ES8326_PGA_PDN: u32 = 0x17;
pub const ES8326_VMIDSEL: u32 = 0x18;
pub const ES8326_ANA_LP: u32 = 0x19;
pub const ES8326_ANA_DMS: u32 = 0x1a;
pub const ES8326_ANA_MICBIAS: u32 = 0x1b;
pub const ES8326_ANA_VSEL: u32 = 0x1c;
pub const ES8326_SYS_BIAS: u32 = 0x1d;
pub const ES8326_BIAS_SW1: u32 = 0x1e;
pub const ES8326_BIAS_SW2: u32 = 0x1f;
pub const ES8326_BIAS_SW3: u32 = 0x20;
pub const ES8326_BIAS_SW4: u32 = 0x21;
pub const ES8326_VMIDLOW: u32 = 0x22;
pub const ES8326_PGAGAIN: u32 = 0x23;
pub const ES8326_HP_DRIVER: u32 = 0x24;
pub const ES8326_DAC2HPMIX: u32 = 0x25;
pub const ES8326_HP_VOL: u32 = 0x26;
pub const ES8326_HP_CAL: u32 = 0x27;
pub const ES8326_HP_DRIVER_REF: u32 = 0x28;
pub const ES8326_ADC_SCALE: u32 = 0x29;
pub const ES8326_ADC1_SRC: u32 = 0x2a;
pub const ES8326_ADC2_SRC: u32 = 0x2b;
pub const ES8326_ADC1_VOL: u32 = 0x2c;
pub const ES8326_ADC2_VOL: u32 = 0x2d;
pub const ES8326_ADC_RAMPRATE: u32 = 0x2e;
pub const ES8326_ADC_DRE: u32 = 0x2f;
pub const ES8326_ADC_DRE_GAIN: u32 = 0x30;
pub const ES8326_ADC_DRE_GATE: u32 = 0x31;
pub const ES8326_ALC_RECOVERY: u32 = 0x32;
pub const ES8326_ALC_LEVEL: u32 = 0x33;
pub const ES8326_ADC_HPFS1: u32 = 0x34;
pub const ES8326_ADC_HPFS2: u32 = 0x35;
pub const ES8326_ADC_EQ: u32 = 0x36;
pub const ES8326_HP_OFFSET_CAL: u32 = 0x4A;
pub const ES8326_HPL_OFFSET_INI: u32 = 0x4B;
pub const ES8326_HPR_OFFSET_INI: u32 = 0x4C;
pub const ES8326_DAC_DSM: u32 = 0x4D;
pub const ES8326_DAC_RAMPRATE: u32 = 0x4E;
pub const ES8326_DAC_VPPSCALE: u32 = 0x4F;
pub const ES8326_DACL_VOL: u32 = 0x50;
pub const ES8326_DRC_RECOVERY: u32 = 0x53;
pub const ES8326_DRC_WINSIZE: u32 = 0x54;
pub const ES8326_DAC_CROSSTALK: u32 = 0x55;
pub const ES8326_HPJACK_TIMER: u32 = 0x56;
pub const ES8326_HPDET_TYPE: u32 = 0x57;
pub const ES8326_INT_SOURCE: u32 = 0x58;
pub const ES8326_INTOUT_IO: u32 = 0x59;
pub const ES8326_SDINOUT1_IO: u32 = 0x5A;
pub const ES8326_SDINOUT23_IO: u32 = 0x5B;
pub const ES8326_JACK_PULSE: u32 = 0x5C;

pub const ES8326_DACR_VOL: u32 = 0xF4;
pub const ES8326_SPKL_VOL: u32 = 0xF5;
pub const ES8326_SPKR_VOL: u32 = 0xF6;
pub const ES8326_HP_MISC: u32 = 0xF7;
pub const ES8326_CTIA_OMTP_STA: u32 = 0xF8;
pub const ES8326_PULLUP_CTL: u32 = 0xF9;
pub const ES8326_CSM_I2C_STA: u32 = 0xFA;
pub const ES8326_HPDET_STA: u32 = 0xFB;
pub const ES8326_CSM_MUTE_STA: u32 = 0xFC;
pub const ES8326_CHIP_ID1: u32 = 0xFD;
pub const ES8326_CHIP_ID2: u32 = 0xFE;
pub const ES8326_CHIP_VERSION: u32 = 0xFF;

/* ES8326_RESET */
pub const ES8326_CSM_ON: u32 = 1 << 7;
pub const ES8326_MASTER_MODE_EN: u32 = 1 << 6;
pub const ES8326_PWRUP_SEQ_EN: u32 = 1 << 5;
pub const ES8326_CODEC_RESET: u32 = 0x0f << 0;
pub const ES8326_CSM_OFF: u32 = 0 << 7;
pub const ES8326_MUTE_MASK: u32 = 3 << 0;
pub const ES8326_MUTE: u32 = 3 << 0;

/* ES8326_CLK_CTL */
pub const ES8326_CLK_ON: u32 = 0x7f << 0;
pub const ES8326_CLK_OFF: u32 = 0 << 0;

/* ES8326_CLK_INV */
pub const ES8326_BCLK_AS_MCLK: u32 = 1 << 3;

/* ES8326_FMT */
pub const ES8326_S24_LE: u32 = 0 << 2;
pub const ES8326_S20_3_LE: u32 = 1 << 2;
pub const ES8326_S18_LE: u32 = 2 << 2;
pub const ES8326_S16_LE: u32 = 3 << 2;
pub const ES8326_S32_LE: u32 = 4 << 2;
pub const ES8326_DATA_LEN_MASK: u32 = 7 << 2;

pub const ES8326_DAIFMT_MASK: u32 = (1 << 5) | (3 << 0);
pub const ES8326_DAIFMT_I2S: u32 = 0;
pub const ES8326_DAIFMT_LEFT_J: u32 = 1 << 0;
pub const ES8326_DAIFMT_DSP_A: u32 = 3 << 0;
pub const ES8326_DAIFMT_DSP_B: u32 = (1 << 5) | (3 << 0);

/* ES8326_PGAGAIN */
pub const ES8326_MIC_SEL_MASK: u32 = 3 << 4;
pub const ES8326_MIC1_SEL: u32 = 1 << 4;
pub const ES8326_MIC2_SEL: u32 = 1 << 5;

/* ES8326_HP_CAL */
pub const ES8326_HP_OFF: u32 = 0;
pub const ES8326_HP_FORCE_CAL: u32 = (1 << 7) | (1 << 3);
pub const ES8326_HP_ON: u32 = (7 << 4) | (7 << 0);

/* ES8326_ADC1_SRC */
pub const ES8326_ADC1_SHIFT: u32 = 0;
pub const ES8326_ADC2_SHIFT: u32 = 4;
pub const ES8326_ADC_SRC_ANA: u32 = 0;
pub const ES8326_ADC_SRC_ANA_INV_SW0: u32 = 1;
pub const ES8326_ADC_SRC_ANA_INV_SW1: u32 = 2;
pub const ES8326_ADC_SRC_DMIC_MCLK: u32 = 3;
pub const ES8326_ADC_SRC_DMIC_SDIN2: u32 = 4;
pub const ES8326_ADC_SRC_DMIC_SDIN2_INV: u32 = 5;
pub const ES8326_ADC_SRC_DMIC_SDIN3: u32 = 6;
pub const ES8326_ADC_SRC_DMIC_SDIN3_INV: u32 = 7;

pub const ES8326_ADC_AMIC: u32 =
    (ES8326_ADC_SRC_ANA_INV_SW1 << ES8326_ADC2_SHIFT)
        | (ES8326_ADC_SRC_ANA_INV_SW1 << ES8326_ADC1_SHIFT);
pub const ES8326_ADC_DMIC: u32 =
    (ES8326_ADC_SRC_DMIC_SDIN2 << ES8326_ADC2_SHIFT)
        | (ES8326_ADC_SRC_DMIC_SDIN2 << ES8326_ADC1_SHIFT);

/* ES8326_ADC2_SRC */
pub const ES8326_ADC3_SHIFT: u32 = 0;
pub const ES8326_ADC4_SHIFT: u32 = 3;

/* ES8326_HPDET_TYPE */
pub const ES8326_HP_DET_SRC_PIN27: u32 = 1 << 5;
pub const ES8326_HP_DET_SRC_PIN9: u32 = 1 << 4;
pub const ES8326_HP_DET_JACK_POL: u32 = 1 << 3;
pub const ES8326_HP_DET_BUTTON_POL: u32 = 1 << 2;
pub const ES8326_HP_TYPE_OMTP: u32 = 3 << 0;
pub const ES8326_HP_TYPE_CTIA: u32 = 2 << 0;
pub const ES8326_HP_TYPE_AUTO: u32 = 1 << 0;
pub const ES8326_HP_TYPE_AUTO_INV: u32 = 0 << 0;

/* ES8326_INT_SOURCE */
pub const ES8326_INT_SRC_DAC_MOZ: u32 = 1 << 0;
pub const ES8326_INT_SRC_ADC_MOZ: u32 = 1 << 1;
pub const ES8326_INT_SRC_BUTTON: u32 = 1 << 2;
pub const ES8326_INT_SRC_PIN9: u32 = 1 << 3;
pub const ES8326_INT_SRC_PIN27: u32 = 1 << 4;

/* ES8326_SDINOUT1_IO */
pub const ES8326_IO_INPUT: u32 = 0 << 0;
pub const ES8326_IO_SDIN_SLOT0: u32 = 1 << 0;
pub const ES8326_IO_SDIN_SLOT1: u32 = 2 << 0;
pub const ES8326_IO_SDIN_SLOT2: u32 = 3 << 0;
pub const ES8326_IO_SDIN_SLOT7: u32 = 8 << 0;
pub const ES8326_IO_DMIC_CLK: u32 = 9 << 0;
pub const ES8326_IO_DMIC_CLK_INV: u32 = 0x0a << 0;
pub const ES8326_IO_SDOUT2: u32 = 0x0b << 0;
pub const ES8326_IO_LOW: u32 = 0x0e << 0;
pub const ES8326_IO_HIGH: u32 = 0x0f << 0;
pub const ES8326_ADC2DAC: u32 = 1 << 3;
pub const ES8326_SDINOUT1_SHIFT: u32 = 4;

/* ES8326_SDINOUT23_IO */
pub const ES8326_SDINOUT2_SHIFT: u32 = 4;
pub const ES8326_SDINOUT3_SHIFT: u32 = 0;

/* ES8326_HPDET_STA */
pub const ES8326_HPINSERT_FLAG: u32 = 1 << 1;
pub const ES8326_HPBUTTON_FLAG: u32 = 1 << 0;

/* ES8326_CHIP_VERSION 0xFF */
pub const ES8326_VERSION: u32 = 1 << 0;
pub const ES8326_VERSION_B: u32 = 3 << 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
