/* SPDX-License-Identifier: GPL-2.0 */
/*
 * nau8325.h -- Nuvoton NAU8325 audio codec driver
 *
 * Copyright 2023 Nuvoton Technology Crop.
 * Author: Seven Lee <WTLI@nuvoton.com>
 *	   David Lin <CTLIN0@nuvoton.com>
 */

use core::ffi::{c_int, c_uint};

extern "C" {
    pub type device;
    pub type regmap;
}

pub const NAU8325_R00_HARDWARE_RST: c_uint = 0x00;
pub const NAU8325_R01_SOFTWARE_RST: c_uint = 0x01;
pub const NAU8325_R02_DEVICE_ID: c_uint = 0x02;
pub const NAU8325_R03_CLK_CTRL: c_uint = 0x03;
pub const NAU8325_R04_ENA_CTRL: c_uint = 0x04;
pub const NAU8325_R05_INTERRUPT_CTRL: c_uint = 0x05;
pub const NAU8325_R06_INT_CLR_STATUS: c_uint = 0x06;
pub const NAU8325_R09_IRQOUT: c_uint = 0x09;
pub const NAU8325_R0A_IO_CTRL: c_uint = 0x0a;
pub const NAU8325_R0B_PDM_CTRL: c_uint = 0x0b;
pub const NAU8325_R0C_TDM_CTRL: c_uint = 0x0c;
pub const NAU8325_R0D_I2S_PCM_CTRL1: c_uint = 0x0d;
pub const NAU8325_R0E_I2S_PCM_CTRL2: c_uint = 0x0e;
pub const NAU8325_R0F_L_TIME_SLOT: c_uint = 0x0f;
pub const NAU8325_R10_R_TIME_SLOT: c_uint = 0x10;
pub const NAU8325_R11_HPF_CTRL: c_uint = 0x11;
pub const NAU8325_R12_MUTE_CTRL: c_uint = 0x12;
pub const NAU8325_R13_DAC_VOLUME: c_uint = 0x13;
pub const NAU8325_R1D_DEBUG_READ1: c_uint = 0x1d;
pub const NAU8325_R1F_DEBUG_READ2: c_uint = 0x1f;
pub const NAU8325_R22_DEBUG_READ3: c_uint = 0x22;
pub const NAU8325_R29_DAC_CTRL1: c_uint = 0x29;
pub const NAU8325_R2A_DAC_CTRL2: c_uint = 0x2a;
pub const NAU8325_R2C_ALC_CTRL1: c_uint = 0x2c;
pub const NAU8325_R2D_ALC_CTRL2: c_uint = 0x2d;
pub const NAU8325_R2E_ALC_CTRL3: c_uint = 0x2e;
pub const NAU8325_R2F_ALC_CTRL4: c_uint = 0x2f;
pub const NAU8325_R40_CLK_DET_CTRL: c_uint = 0x40;
pub const NAU8325_R49_TEST_STATUS: c_uint = 0x49;
pub const NAU8325_R4A_ANALOG_READ: c_uint = 0x4a;
pub const NAU8325_R50_MIXER_CTRL: c_uint = 0x50;
pub const NAU8325_R55_MISC_CTRL: c_uint = 0x55;
pub const NAU8325_R60_BIAS_ADJ: c_uint = 0x60;
pub const NAU8325_R61_ANALOG_CONTROL_1: c_uint = 0x61;
pub const NAU8325_R62_ANALOG_CONTROL_2: c_uint = 0x62;
pub const NAU8325_R63_ANALOG_CONTROL_3: c_uint = 0x63;
pub const NAU8325_R64_ANALOG_CONTROL_4: c_uint = 0x64;
pub const NAU8325_R65_ANALOG_CONTROL_5: c_uint = 0x65;
pub const NAU8325_R66_ANALOG_CONTROL_6: c_uint = 0x66;
pub const NAU8325_R69_CLIP_CTRL: c_uint = 0x69;
pub const NAU8325_R73_RDAC: c_uint = 0x73;
pub const NAU8325_REG_MAX: c_uint = NAU8325_R73_RDAC;

/* 16-bit control register address, and 16-bits control register data */
pub const NAU8325_REG_ADDR_LEN: c_uint = 16;
pub const NAU8325_REG_DATA_LEN: c_uint = 16;

/* CLK_CTRL (0x03) */
pub const NAU8325_CLK_DAC_SRC_SFT: c_uint = 12;
pub const NAU8325_CLK_DAC_SRC_MASK: c_uint = 0x3 << NAU8325_CLK_DAC_SRC_SFT;
pub const NAU8325_CLK_MUL_SRC_SFT: c_uint = 6;
pub const NAU8325_CLK_MUL_SRC_MASK: c_uint = 0x3 << NAU8325_CLK_MUL_SRC_SFT;
pub const NAU8325_MCLK_SEL_SFT: c_uint = 3;
pub const NAU8325_MCLK_SEL_MASK: c_uint = 0x7 << NAU8325_MCLK_SEL_SFT;
pub const NAU8325_MCLK_SRC_MASK: c_uint = 0x7;

/* ENA_CTRL (0x04) */
pub const NAU8325_DAC_LEFT_CH_EN_SFT: c_uint = 3;
pub const NAU8325_DAC_LEFT_CH_EN: c_uint = 0x1 << NAU8325_DAC_LEFT_CH_EN_SFT;
pub const NAU8325_DAC_RIGHT_CH_EN_SFT: c_uint = 2;
pub const NAU8325_DAC_RIGHT_CH_EN: c_uint = 0x1 << NAU8325_DAC_RIGHT_CH_EN_SFT;

/* INTERRUPT_CTRL (0x05) */
pub const NAU8325_ARP_DWN_INT_SFT: c_uint = 12;
pub const NAU8325_ARP_DWN_INT_MASK: c_uint = 0x1 << NAU8325_ARP_DWN_INT_SFT;
pub const NAU8325_CLIP_INT_SFT: c_uint = 11;
pub const NAU8325_CLIP_INT_MASK: c_uint = 0x1 << NAU8325_CLIP_INT_SFT;
pub const NAU8325_LVD_INT_SFT: c_uint = 10;
pub const NAU8325_LVD_INT_MASK: c_uint = 0x1 << NAU8325_LVD_INT_SFT;
pub const NAU8325_PWR_INT_DIS_SFT: c_uint = 8;
pub const NAU8325_PWR_INT_DIS: c_uint = 0x1 << NAU8325_PWR_INT_DIS_SFT;
pub const NAU8325_OCP_OTP_SHTDWN_INT_SFT: c_uint = 4;
pub const NAU8325_OCP_OTP_SHTDWN_INT_MASK: c_uint = 0x1 << NAU8325_OCP_OTP_SHTDWN_INT_SFT;
pub const NAU8325_CLIP_INT_DIS_SFT: c_uint = 3;
pub const NAU8325_CLIP_INT_DIS: c_uint = 0x1 << NAU8325_CLIP_INT_DIS_SFT;
pub const NAU8325_LVD_INT_DIS_SFT: c_uint = 2;
pub const NAU8325_LVD_INT_DIS: c_uint = 0x1 << NAU8325_LVD_INT_DIS_SFT;
pub const NAU8325_PWR_INT_MASK: c_uint = 0x1;

/* INT_CLR_STATUS (0x06) */
pub const NAU8325_INT_STATUS_MASK: c_uint = 0x7f;

/* IRQOUT (0x9) */
pub const NAU8325_IRQOUT_SEL_SEF: c_uint = 12;
pub const NAU8325_IRQOUT_SEL_MASK: c_uint = 0xf << NAU8325_IRQOUT_SEL_SEF;
pub const NAU8325_DEM_DITH_SFT: c_uint = 7;
pub const NAU8325_DEM_DITH_EN: c_uint = 0x1 << NAU8325_DEM_DITH_SFT;
pub const NAU8325_GAINZI3_SFT: c_uint = 5;
pub const NAU8325_GAINZI3_MASK: c_uint = 0x1 << NAU8325_GAINZI3_SFT;
pub const NAU8325_GAINZI2_MASK: c_uint = 0x1f;

/* IO_CTRL (0x0a) */
pub const NAU8325_IRQ_PL_SFT: c_uint = 15;
pub const NAU8325_IRQ_PL_ACT_HIGH: c_uint = 0x1 << NAU8325_IRQ_PL_SFT;
pub const NAU8325_IRQ_PS_SFT: c_uint = 14;
pub const NAU8325_IRQ_PS_UP: c_uint = 0x1 << NAU8325_IRQ_PS_SFT;
pub const NAU8325_IRQ_PE_SFT: c_uint = 13;
pub const NAU8325_IRQ_PE_EN: c_uint = 0x1 << NAU8325_IRQ_PE_SFT;
pub const NAU8325_IRQ_DS_SFT: c_uint = 12;
pub const NAU8325_IRQ_DS_HIGH: c_uint = 0x1 << NAU8325_IRQ_DS_SFT;
pub const NAU8325_IRQ_OUTPUT_SFT: c_uint = 11;
pub const NAU8325_IRQ_OUTPUT_EN: c_uint = 0x1 << NAU8325_IRQ_OUTPUT_SFT;
pub const NAU8325_IRQ_PIN_DEBUG_SFT: c_uint = 10;
pub const NAU8325_IRQ_PIN_DEBUG_EN: c_uint = 0x1 << NAU8325_IRQ_PIN_DEBUG_SFT;

/* PDM_CTRL (0x0b) */
pub const NAU8325_PDM_LCH_EDGE_SFT: c_uint = 1;
pub const NAU8325_PDM_LCH_EDGE__MASK: c_uint = 0x1 << NAU8325_PDM_LCH_EDGE_SFT;
pub const NAU8325_PDM_MODE_EN: c_uint = 0x1;

/* TDM_CTRL (0x0c) */
pub const NAU8325_TDM_SFT: c_uint = 15;
pub const NAU8325_TDM_EN: c_uint = 0x1 << NAU8325_TDM_SFT;
pub const NAU8325_PCM_OFFSET_CTRL_SFT: c_uint = 14;
pub const NAU8325_PCM_OFFSET_CTRL_EN: c_uint = 0x1 << NAU8325_PCM_OFFSET_CTRL_SFT;
pub const NAU8325_DAC_LEFT_SFT: c_uint = 6;
pub const NAU8325_NAU8325_DAC_LEFT_MASK: c_uint = 0x7 << NAU8325_DAC_LEFT_SFT;
pub const NAU8325_DAC_RIGHT_SFT: c_uint = 3;
pub const NAU8325_DAC_RIGHT_MASK: c_uint = 0x7 << NAU8325_DAC_RIGHT_SFT;

/* I2S_PCM_CTRL1 (0x0d) */
pub const NAU8325_DACCM_CTL_SFT: c_uint = 14;
pub const NAU8325_DACCM_CTL_MASK: c_uint = 0x3 << NAU8325_DACCM_CTL_SFT;
pub const NAU8325_CMB8_0_SFT: c_uint = 10;
pub const NAU8325_CMB8_0_MASK: c_uint = 0x1 << NAU8325_CMB8_0_SFT;
pub const NAU8325_UA_OFFSET_SFT: c_uint = 9;
pub const NAU8325_UA_OFFSET_MASK: c_uint = 0x1 << NAU8325_UA_OFFSET_SFT;
pub const NAU8325_I2S_BP_SFT: c_uint = 7;
pub const NAU8325_I2S_BP_MASK: c_uint = 0x1 << NAU8325_I2S_BP_SFT;
pub const NAU8325_I2S_BP_INV: c_uint = 0x1 << NAU8325_I2S_BP_SFT;
pub const NAU8325_I2S_PCMB_SFT: c_uint = 6;
pub const NAU8325_I2S_PCMB_EN: c_uint = 0x1 << NAU8325_I2S_PCMB_SFT;
pub const NAU8325_I2S_DACPSHS0_SFT: c_uint = 5;
pub const NAU8325_I2S_DACPSHS0_MASK: c_uint = 0x1 << NAU8325_I2S_DACPSHS0_SFT;
pub const NAU8325_I2S_DL_SFT: c_uint = 2;
pub const NAU8325_I2S_DL_MASK: c_uint = 0x3 << NAU8325_I2S_DL_SFT;
pub const NAU8325_I2S_DL_32: c_uint = 0x3 << NAU8325_I2S_DL_SFT;
pub const NAU8325_I2S_DL_24: c_uint = 0x2 << NAU8325_I2S_DL_SFT;
pub const NAU8325_I2S_DL_20: c_uint = 0x1 << NAU8325_I2S_DL_SFT;
pub const NAU8325_I2S_DL_16: c_uint = 0x0 << NAU8325_I2S_DL_SFT;
pub const NAU8325_I2S_DF_MASK: c_uint = 0x3;
pub const NAU8325_I2S_DF_RIGTH: c_uint = 0x0;
pub const NAU8325_I2S_DF_LEFT: c_uint = 0x1;
pub const NAU8325_I2S_DF_I2S: c_uint = 0x2;
pub const NAU8325_I2S_DF_PCM_AB: c_uint = 0x3;

/* I2S_PCM_CTRL2 (0x0e) */
pub const NAU8325_PCM_TS_SFT: c_uint = 10;
pub const NAU8325_PCM_TS_EN: c_uint = 0x1 << NAU8325_PCM_TS_SFT;
pub const NAU8325_PCM8BIT0_SFT: c_uint = 8;
pub const NAU8325_PCM8BIT0_MASK: c_uint = 0x1 << NAU8325_PCM8BIT0_SFT;

/* L_TIME_SLOT (0x0f)*/
pub const NAU8325_SHORT_FS_DET_SFT: c_uint = 13;
pub const NAU8325_SHORT_FS_DET_DIS: c_uint = 0x1 << NAU8325_SHORT_FS_DET_SFT;
pub const NAU8325_TSLOT_L0_MASK: c_uint = 0x3ff;

/* R_TIME_SLOT (0x10)*/
pub const NAU8325_TSLOT_R0_MASK: c_uint = 0x3ff;

/* HPF_CTRL (0x11)*/
pub const NAU8325_DAC_HPF_SFT: c_uint = 15;
pub const NAU8325_DAC_HPF_EN: c_uint = 0x1 << NAU8325_DAC_HPF_SFT;
pub const NAU8325_DAC_HPF_APP_SFT: c_uint = 14;
pub const NAU8325_DAC_HPF_APP_MASK: c_uint = 0x1 << NAU8325_DAC_HPF_APP_SFT;
pub const NAU8325_DAC_HPF_FCUT_SFT: c_uint = 11;
pub const NAU8325_DAC_HPF_FCUT_MASK: c_uint = 0x7 << NAU8325_DAC_HPF_FCUT_SFT;

/* MUTE_CTRL (0x12)*/
pub const NAU8325_SOFT_MUTE_SFT: c_uint = 15;
pub const NAU8325_SOFT_MUTE: c_uint = 0x1 << NAU8325_SOFT_MUTE_SFT;
pub const NAU8325_DAC_ZC_SFT: c_uint = 8;
pub const NAU8325_DAC_ZC_EN: c_uint = 0x1 << NAU8325_DAC_ZC_SFT;
pub const NAU8325_UNMUTE_CTL_SFT: c_uint = 6;
pub const NAU8325_UNMUTE_CTL_MASK: c_uint = 0x3 << NAU8325_UNMUTE_CTL_SFT;
pub const NAU8325_ANA_MUTE_SFT: c_uint = 4;
pub const NAU8325_ANA_MUTE_MASK: c_uint = 0x3 << NAU8325_ANA_MUTE_SFT;
pub const NAU8325_AUTO_MUTE_SFT: c_uint = 3;
pub const NAU8325_AUTO_MUTE_DIS: c_uint = 0x1 << NAU8325_AUTO_MUTE_SFT;

/* DAC_VOLUME (0x13) */
pub const NAU8325_DAC_VOLUME_L_SFT: c_uint = 8;
pub const NAU8325_DAC_VOLUME_L_EN: c_uint = 0xff << NAU8325_DAC_VOLUME_L_SFT;
pub const NAU8325_DAC_VOLUME_R_SFT: c_uint = 0;
pub const NAU8325_DAC_VOLUME_R_EN: c_uint = 0xff << NAU8325_DAC_VOLUME_R_SFT;
pub const NAU8325_DAC_VOL_MAX: c_uint = 0xff;

/* DEBUG_READ1 (0x1d)*/
pub const NAU8325_OSR100_MASK: c_uint = 0x1 << 6;
pub const NAU8325_MIPS500_MASK: c_uint = 0x1 << 5;
pub const NAU8325_SHUTDWNDRVR_R_MASK: c_uint = 0x1 << 4;
pub const NAU8325_SHUTDWNDRVR_L_MASK: c_uint = 0x1 << 3;
pub const NAU8325_MUTEB_MASK: c_uint = 0x1 << 2;
pub const NAU8325_PDOSCB_MASK: c_uint = 0x1 << 1;
pub const NAU8325_POWERDOWN1B_D_MASK: c_uint = 0x1;

/* DEBUG_READ2 (0x1f)*/
pub const NAU8325_R_CHANNEL_Vol_SFT: c_uint = 8;
pub const NAU8325_R_CHANNEL_Vol_MASK: c_uint = 0xff << NAU8325_R_CHANNEL_Vol_SFT;
pub const NAU8325_L_CHANNEL_Vol_MASK: c_uint = 0xff;

/* DEBUG_READ3(0x22)*/
pub const NAU8325_PGAL_GAIN_MASK: c_uint = 0x3f << 7;
pub const NAU8325_CLIP_MASK: c_uint = 0x1 << 6;
pub const NAU8325_SCAN_MODE_MASK: c_uint = 0x1 << 5;
pub const NAU8325_SDB_MASK: c_uint = 0x1 << 4;
pub const NAU8325_TALARM_MASK: c_uint = 0x1 << 3;
pub const NAU8325_SHORTR_MASK: c_uint = 0x1 << 2;
pub const NAU8325_SHORTL_MASK: c_uint = 0x1 << 1;
pub const NAU8325_TMDET_MASK: c_uint = 0x1;

/* DAC_CTRL1 (0x29) */
pub const NAU8325_DAC_OVERSAMPLE_SFT: c_uint = 0;
pub const NAU8325_DAC_OVERSAMPLE_MASK: c_uint = 0x7;
pub const NAU8325_DAC_OVERSAMPLE_256: c_uint = 1;
pub const NAU8325_DAC_OVERSAMPLE_128: c_uint = 2;
pub const NAU8325_DAC_OVERSAMPLE_64: c_uint = 0;
pub const NAU8325_DAC_OVERSAMPLE_32: c_uint = 4;

/* ALC_CTRL1 (0x2c) */
pub const NAU8325_ALC_MAXGAIN_SFT: c_uint = 5;
pub const NAU8325_ALC_MAXGAIN_MAX: c_uint = 0x7;
pub const NAU8325_ALC_MAXGAIN_MASK: c_uint = 0x7 << NAU8325_ALC_MAXGAIN_SFT;
pub const NAU8325_ALC_MINGAIN_MAX: c_uint = 4;
pub const NAU8325_ALC_MINGAIN_SFT: c_uint = 1;
pub const NAU8325_ALC_MINGAIN_MASK: c_uint = 0x7 << NAU8325_ALC_MINGAIN_SFT;

/* ALC_CTRL2 (0x2d) */
pub const NAU8325_ALC_DCY_SFT: c_uint = 12;
pub const NAU8325_ALC_DCY_MAX: c_uint = 0xb;
pub const NAU8325_ALC_DCY_MASK: c_uint = 0xf << NAU8325_ALC_DCY_SFT;
pub const NAU8325_ALC_ATK_SFT: c_uint = 8;
pub const NAU8325_ALC_ATK_MAX: c_uint = 0xb;
pub const NAU8325_ALC_ATK_MASK: c_uint = 0xf << NAU8325_ALC_ATK_SFT;
pub const NAU8325_ALC_HLD_SFT: c_uint = 4;
pub const NAU8325_ALC_HLD_MAX: c_uint = 0xa;
pub const NAU8325_ALC_HLD_MASK: c_uint = 0xf << NAU8325_ALC_HLD_SFT;
pub const NAU8325_ALC_LVL_SFT: c_uint = 0;
pub const NAU8325_ALC_LVL_MAX: c_uint = 0xf;
pub const NAU8325_ALC_LVL_MASK: c_uint = 0xf;

/* ALC_CTRL3 (0x2e) */
pub const NAU8325_ALC_EN_SFT: c_uint = 15;
pub const NAU8325_ALC_EN: c_uint = 0x1 << NAU8325_ALC_EN_SFT;

/* TEMP_COMP_CTRL (0x30) */
pub const NAU8325_TEMP_COMP_ACT2_MASK: c_uint = 0xff;

/* LPF_CTRL (0x33) */
pub const NAU8325_LPF_IN1_EN_SFT: c_uint = 15;
pub const NAU8325_LPF_IN1_EN: c_uint = 0x1 << NAU8325_LPF_IN1_EN_SFT;
pub const NAU8325_LPF_IN1_TC_SFT: c_uint = 11;
pub const NAU8325_LPF_IN1_TC_MASK: c_uint = 0xf << NAU8325_LPF_IN1_TC_SFT;
pub const NAU8325_LPF_IN2_EN_SFT: c_uint = 10;
pub const NAU8325_LPF_IN2_EN: c_uint = 0x1 << NAU8325_LPF_IN2_EN_SFT;
pub const NAU8325_LPF_IN2_TC_SFT: c_uint = 6;
pub const NAU8325_LPF_IN2_TC_MASK: c_uint = 0xf << NAU8325_LPF_IN2_TC_SFT;

/* CLK_DET_CTRL (0x40) */
pub const NAU8325_APWRUP_SFT: c_uint = 15;
pub const NAU8325_APWRUP_EN: c_uint = 0x1 << NAU8325_APWRUP_SFT;
pub const NAU8325_CLKPWRUP_SFT: c_uint = 14;
pub const NAU8325_CLKPWRUP_DIS: c_uint = 0x1 << NAU8325_CLKPWRUP_SFT;
pub const NAU8325_PWRUP_DFT_SFT: c_uint = 13;
pub const NAU8325_PWRUP_DFT: c_uint = 0x1 << NAU8325_PWRUP_DFT_SFT;
pub const NAU8325_REG_SRATE_SFT: c_uint = 10;
pub const NAU8325_REG_SRATE_MASK: c_uint = 0x7 << NAU8325_REG_SRATE_SFT;
pub const NAU8325_REG_ALT_SRATE_SFT: c_uint = 9;
pub const NAU8325_REG_ALT_SRATE_EN: c_uint = 0x1 << NAU8325_REG_ALT_SRATE_SFT;
pub const NAU8325_REG_DIV_MAX: c_uint = 0x1;

/* BIAS_ADJ (0x60) */
pub const NAU8325_BIAS_VMID_SEL_SFT: c_uint = 4;
pub const NAU8325_BIAS_VMID_SEL_MASK: c_uint = 0x3 << NAU8325_BIAS_VMID_SEL_SFT;

/* ANALOG_CONTROL_1 (0x61) */
pub const NAU8325_VMDFSTENB_SFT: c_uint = 14;
pub const NAU8325_VMDFSTENB_MASK: c_uint = 0x3 << NAU8325_VMDFSTENB_SFT;
pub const NAU8325_CLASSDEN_SFT: c_uint = 12;
pub const NAU8325_CLASSDEN_MASK: c_uint = 0x3 << NAU8325_CLASSDEN_SFT;
pub const NAU8325_DACCLKEN_R_SFT: c_uint = 10;
pub const NAU8325_DACCLKEN_R_MASK: c_uint = 0x3 << NAU8325_DACCLKEN_R_SFT;
pub const NAU8325_DACEN_R_SFT: c_uint = 8;
pub const NAU8325_DACEN_R_MASK: c_uint = 0x3 << NAU8325_DACEN_R_SFT;
pub const NAU8325_DACCLKEN_SFT: c_uint = 6;
pub const NAU8325_DACCLKEN_MASK: c_uint = 0x3 << NAU8325_DACCLKEN_SFT;
pub const NAU8325_DACEN_SFT: c_uint = 4;
pub const NAU8325_DACEN_MASK: c_uint = 0x3 << NAU8325_DACEN_SFT;
pub const NAU8325_BIASEN_SFT: c_uint = 2;
pub const NAU8325_BIASEN_MASK: c_uint = 0x3 << NAU8325_BIASEN_SFT;
pub const NAU8325_VMIDEN_MASK: c_uint = 0x3;

/* ANALOG_CONTROL_2 (0x62) */
pub const NAU8325_PWMMOD_SFT: c_uint = 14;
pub const NAU8325_PWMMOD_MASK: c_uint = 0x1 << NAU8325_PWMMOD_SFT;
pub const NAU8325_DACTEST_SFT: c_uint = 6;
pub const NAU8325_DACTEST_MASK: c_uint = 0x3 << NAU8325_DACTEST_SFT;
pub const NAU8325_DACREFCAP_SFT: c_uint = 4;
pub const NAU8325_DACREFCAP_MASK: c_uint = 0x3 << NAU8325_DACREFCAP_SFT;

/* ANALOG_CONTROL_3 (0x63) */
pub const NAU8325_POWER_DOWN_L_SFT: c_uint = 12;
pub const NAU8325_POWER_DOWN_L_MASK: c_uint = 0x3 << NAU8325_POWER_DOWN_L_SFT;
pub const NAU8325_POWER_DOWN_R_SFT: c_uint = 11;
pub const NAU8325_POWER_DOWN_R_MASK: c_uint = 0x3 << NAU8325_DACREFCAP_SFT;
pub const NAU8325_CLASSD_FINE_SFT: c_uint = 5;
pub const NAU8325_CLASSD_FINE_MASK: c_uint = 0x3 << NAU8325_CLASSD_FINE_SFT;
pub const NAU8325_CLASSD_COARSE_GAIN_MASK: c_uint = 0xf;

/* ANALOG_CONTROL_4 (0x64) */
pub const NAU8325_CLASSD_OCPN_SFT: c_uint = 12;
pub const NAU8325_CLASSD_OCPN_MASK: c_uint = 0xf << NAU8325_CLASSD_OCPN_SFT;
pub const NAU8325_CLASSD_OCPP_SFT: c_uint = 8;
pub const NAU8325_CLASSD_OCPP_MASK: c_uint = 0xf << NAU8325_CLASSD_OCPP_SFT;
pub const NAU8325_CLASSD_SLEWN_MASK: c_uint = 0xff;

/* ANALOG_CONTROL_5 (0x65) */
pub const NAU8325_MCLK_RANGE_SFT: c_uint = 2;
pub const NAU8325_MCLK_RANGE_EN: c_uint = 0x1 << NAU8325_MCLK_RANGE_SFT;
pub const NAU8325_MCLK8XEN_SFT: c_uint = 1;
pub const NAU8325_MCLK8XEN_EN: c_uint = 0x1 << NAU8325_MCLK8XEN_SFT;
pub const NAU8325_MCLK4XEN_EN: c_uint = 0x1;

/* ANALOG_CONTROL_6 (0x66) */
pub const NAU8325_VBATLOW_SFT: c_uint = 4;
pub const NAU8325_VBATLOW_MASK: c_uint = 0x1 << NAU8325_VBATLOW_SFT;
pub const NAU8325_VDDSPK_LIM_SFT: c_uint = 3;
pub const NAU8325_VDDSPK_LIM_EN: c_uint = 0x1 << NAU8325_VDDSPK_LIM_SFT;
pub const NAU8325_VDDSPK_LIM_MASK: c_uint = 0x7;

/* CLIP_CTRL (0x69)*/
pub const NAU8325_ANTI_CLIP_SFT: c_uint = 4;
pub const NAU8325_ANTI_CLIP_EN: c_uint = 0x1 << NAU8325_ANTI_CLIP_SFT;

/* RDAC (0x73) */
pub const NAU8325_CLK_DAC_DELAY_SFT: c_uint = 4;
pub const NAU8325_CLK_DAC_DELAY_EN: c_uint = 0x7 << NAU8325_CLK_DAC_DELAY_SFT;
pub const NAU8325_DACVREFSEL_SFT: c_uint = 2;
pub const NAU8325_DACVREFSEL_MASK: c_uint = 0x3 << NAU8325_DACVREFSEL_SFT;

pub const NAU8325_CODEC_DAI: &[u8; 12] = b"nau8325-hifi";

#[repr(C)]
pub struct nau8325 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mclk: c_int,
    pub fs: c_int,
    pub vref_impedance_ohms: c_int,
    pub dac_vref_microvolt: c_int,
    pub clock_detection: c_int,
    pub clock_det_data: c_int,
    pub alc_enable: c_int,
}

#[repr(C)]
pub struct nau8325_src_attr {
    pub param: c_int,
    pub val: c_uint,
}

pub const NAU8325_MCLK_FS_RATIO_256: c_int = 0;
pub const NAU8325_MCLK_FS_RATIO_400: c_int = 1;
pub const NAU8325_MCLK_FS_RATIO_500: c_int = 2;
pub const NAU8325_MCLK_FS_RATIO_NUM: usize = 3;

#[repr(C)]
pub struct nau8325_srate_attr {
    pub fs: c_int,
    pub range: c_int,
    pub max: bool,
    pub mclk_src: [c_uint; NAU8325_MCLK_FS_RATIO_NUM],
}

#[repr(C)]
pub struct nau8325_osr_attr {
    pub osr: c_uint,
    pub clk_src: c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
