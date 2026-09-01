/* SPDX-License-Identifier: GPL-2.0-only */
/*
* ES8389.h  --  ES8389 ALSA SoC Audio Codec
*
* Authors:
*
* Based on ES8374.h by Michael Zhang
*/

/*
*     ES8389_REGISTER NAME_REG_REGISTER ADDRESS
*/
pub const ES8389_RESET: u32 = 0x00; /*reset digital,csm,clock manager etc.*/

/*
* Clock Scheme Register definition
*/
pub const ES8389_MASTER_MODE: u32 = 0x01;
pub const ES8389_MASTER_CLK: u32 = 0x02;
pub const ES8389_CLK_OFF1: u32 = 0x03;
pub const ES8389_CLK_DIV1: u32 = 0x04;
pub const ES8389_CLK_MUL: u32 = 0x05;
pub const ES8389_CLK_MUX1: u32 = 0x06;
pub const ES8389_CLK_MUX2: u32 = 0x07;
pub const ES8389_CLK_CTL1: u32 = 0x08;
pub const ES8389_CLK_CTL2: u32 = 0x09;
pub const ES8389_CLK_CTL3: u32 = 0x0A;
pub const ES8389_SCLK_DIV: u32 = 0x0B;
pub const ES8389_LRCK_DIV1: u32 = 0x0C;
pub const ES8389_LRCK_DIV2: u32 = 0x0D;
pub const ES8389_CLK_OFF2: u32 = 0x0E;
pub const ES8389_OSC_CLK: u32 = 0x0F;
pub const ES8389_CSM_JUMP: u32 = 0x10;
pub const ES8389_CLK_DIV2: u32 = 0x11;
pub const ES8389_SYSTEM12: u32 = 0x12;
pub const ES8389_SYSTEM13: u32 = 0x13;
pub const ES8389_SYSTEM14: u32 = 0x14;
pub const ES8389_SYSTEM15: u32 = 0x15;
pub const ES8389_SYSTEM16: u32 = 0x16;
pub const ES8389_SYSTEM17: u32 = 0x17;
pub const ES8389_SYSTEM18: u32 = 0x18;
pub const ES8389_SYSTEM19: u32 = 0x19;
pub const ES8389_SYSTEM1A: u32 = 0x1A;
pub const ES8389_SYSTEM1B: u32 = 0x1B;
pub const ES8389_SYSTEM1C: u32 = 0x1C;
pub const ES8389_ADC_FORMAT_MUTE: u32 = 0x20;
pub const ES8389_ADC_OSR: u32 = 0x21;
pub const ES8389_ADC_DSP: u32 = 0x22;
pub const ES8389_ADC_MODE: u32 = 0x23;
pub const ES8389_ADC_HPF1: u32 = 0x24;
pub const ES8389_ADC_HPF2: u32 = 0x25;
pub const ES8389_OSR_VOL: u32 = 0x26;
pub const ES8389_ADCL_VOL: u32 = 0x27;
pub const ES8389_ADCR_VOL: u32 = 0x28;
pub const ES8389_ALC_CTL: u32 = 0x29;
pub const ES8389_PTDM_SLOT: u32 = 0x2A;
pub const ES8389_ALC_ON: u32 = 0x2B;
pub const ES8389_ALC_TARGET: u32 = 0x2C;
pub const ES8389_ALC_GAIN: u32 = 0x2D;
pub const ES8389_SYSTEM2E: u32 = 0x2E;
pub const ES8389_ADC_MUTE: u32 = 0x2F;
pub const ES8389_SYSTEM30: u32 = 0x30;
pub const ES8389_ADC_RESET: u32 = 0x31;
pub const ES8389_DAC_FORMAT_MUTE: u32 = 0x40;
pub const ES8389_DAC_DSM_OSR: u32 = 0x41;
pub const ES8389_DAC_DSP_OSR: u32 = 0x42;
pub const ES8389_DAC_MISC: u32 = 0x43;
pub const ES8389_DAC_MIX: u32 = 0x44;
pub const ES8389_DAC_INV: u32 = 0x45;
pub const ES8389_DACL_VOL: u32 = 0x46;
pub const ES8389_DACR_VOL: u32 = 0x47;
pub const ES8389_MIX_VOL: u32 = 0x48;
pub const ES8389_DAC_RAMP: u32 = 0x49;
pub const ES8389_SYSTEM4C: u32 = 0x4C;
pub const ES8389_DAC_RESET: u32 = 0x4D;
pub const ES8389_VMID: u32 = 0x60;
pub const ES8389_ANA_CTL1: u32 = 0x61;
pub const ES8389_ANA_VSEL: u32 = 0x62;
pub const ES8389_ANA_CTL2: u32 = 0x63;
pub const ES8389_ADC_EN: u32 = 0x64;
pub const ES8389_HPSW: u32 = 0x69;
pub const ES8389_LOW_POWER1: u32 = 0x6B;
pub const ES8389_LOW_POWER2: u32 = 0x6C;
pub const ES8389_DMIC_EN: u32 = 0x6D;
pub const ES8389_PGA_SW: u32 = 0x6E;
pub const ES8389_MOD_SW1: u32 = 0x6F;
pub const ES8389_MOD_SW2: u32 = 0x70;
pub const ES8389_MOD_SW3: u32 = 0x71;
pub const ES8389_MIC1_GAIN: u32 = 0x72;
pub const ES8389_MIC2_GAIN: u32 = 0x73;

pub const ES8389_CHIP_MISC: u32 = 0xF0;
pub const ES8389_CSM_STATE1: u32 = 0xF1;
pub const ES8389_PULL_DOWN: u32 = 0xF2;
pub const ES8389_ISO_CTL: u32 = 0xF3;
pub const ES8389_CSM_STATE2: u32 = 0xF4;

pub const ES8389_CHIP_ID0: u32 = 0xFD;
pub const ES8389_CHIP_ID1: u32 = 0xFE;

pub const ES8389_MAX_REGISTER: u32 = 0xFF;

pub const ES8389_MIC_SEL_MASK: u32 = 7 << 4;
pub const ES8389_MIC_DEFAULT: u32 = 1 << 4;

pub const ES8389_HPF_DEFAULT: u32 = 16;
pub const ES8389_HPF_OFFSET: u32 = 4;

pub const ES8389_MASTER_MODE_EN: u32 = 1 << 0;

pub const ES8389_TDM_OFF: u32 = 0 << 0;
pub const ES8389_STDM_ON: u32 = 1 << 7;
pub const ES8389_PTDM_ON: u32 = 1 << 6;

pub const ES8389_TDM_MODE: u32 = ES8389_TDM_OFF;
pub const ES8389_TDM_SLOT: u32 = 0x70 << 0;
pub const ES8389_TDM_SHIFT: u32 = 4;

pub const ES8389_MCLK_MASK: u32 = 3 << 6;
pub const ES8389_MCLK_FROM_SCLK: u32 = 1 << 6;
pub const ES8389_MCLK_SOURCE: u32 = ES8389_MCLK_PIN;
pub const ES8389_MCLK_PIN: u32 = 0;
pub const ES8389_SCLK_PIN: u32 = 1;

/* ES8389_FMT */
pub const ES8389_S24_LE: u32 = 0 << 5;
pub const ES8389_S20_3_LE: u32 = 1 << 5;
pub const ES8389_S18_LE: u32 = 2 << 5;
pub const ES8389_S16_LE: u32 = 3 << 5;
pub const ES8389_S32_LE: u32 = 4 << 5;
pub const ES8389_DATA_LEN_MASK: u32 = 7 << 5;

pub const ES8389_DAIFMT_MASK: u32 = 7 << 2;
pub const ES8389_DAIFMT_I2S: u32 = 0;
pub const ES8389_DAIFMT_LEFT_J: u32 = 1 << 2;
pub const ES8389_DAIFMT_DSP_A: u32 = 1 << 3;
pub const ES8389_DAIFMT_DSP_B: u32 = 3 << 3;

pub const ES8389_STATE_ON: u32 = 13 << 0;
pub const ES8389_STATE_STANDBY: u32 = 7 << 0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ES8389_supplies {
    ES8389_SUPPLY_VD = 0,
    ES8389_SUPPLY_VA = 1,
}

pub const ES8389_3V3: u32 = 1;
pub const ES8389_1V8: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
