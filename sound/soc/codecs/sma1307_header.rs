/* SPDX-License-Identifier: GPL-2.0-or-later
 * sma1307.h -- sma1307 ALSA SoC Audio driver
 *
 * Copyright 2024 Iron Device Corporation
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

use core::ffi::{c_char, c_int};

// C dependency intent: #include <sound/soc.h>

#[repr(C)]
pub enum sma1307_fault {
    SMA1307_FAULT_OT1,
    SMA1307_FAULT_OT2,
    SMA1307_FAULT_UVLO,
    SMA1307_FAULT_OVP_BST,
    SMA1307_FAULT_OCP_SPK,
    SMA1307_FAULT_OCP_BST,
    SMA1307_FAULT_CLK,
}

#[repr(C)]
pub enum sma1307_mode {
    SMA1307_MONO_MODE,
    SMA1307_LEFT_MODE,
    SMA1307_RIGHT_MODE,
}

#[repr(C)]
pub enum sma1307_sdo_mode {
    SMA1307_OUT_DATA_ONE_48K,
    SMA1307_OUT_DATA_TWO_48K,
    SMA1307_OUT_DATA_TWO_24K,
    SMA1307_OUT_CLK_PLL,
    SMA1307_OUT_CLK_OSC,
}

#[repr(C)]
pub enum sma1307_sdo_source {
    SMA1307_OUT_DISABLE,
    SMA1307_OUT_FORMAT_C,
    SMA1307_OUT_MIXER_OUT,
    SMA1307_OUT_AFTER_DSP,
    SMA1307_OUT_VRMS2_AVG,
    SMA1307_OUT_BATTERY,
    SMA1307_OUT_TEMP,
    SMA1307_OUT_AFTER_DELAY,
}

#[repr(C)]
pub struct sma1307_setting_file {
    pub status: bool,
    pub header: *mut c_char,
    pub def: *mut c_int,
    pub mode_set: [*mut c_int; 5],
    pub checksum: c_int,
    pub num_mode: c_int,
    pub header_size: usize,
    pub def_size: usize,
    pub mode_size: usize,
}

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const SMA1307_I2C_ADDR_00: u32 = 0x1e;
pub const SMA1307_I2C_ADDR_01: u32 = 0x3e;
pub const SMA1307_I2C_ADDR_10: u32 = 0x5e;
pub const SMA1307_I2C_ADDR_11: u32 = 0x7e;

pub const DEVICE_NAME_SMA1307A: &str = "sma1307a";
pub const DEVICE_NAME_SMA1307AQ: &str = "sma1307aq";

pub const SMA1307_EXTERNAL_CLOCK_19_2: u32 = 0x00;
pub const SMA1307_EXTERNAL_CLOCK_24_576: u32 = 0x01;
pub const SMA1307_PLL_CLKIN_MCLK: u32 = 0x02;
pub const SMA1307_PLL_CLKIN_BCLK: u32 = 0x03;

pub const SMA1307_OFFSET_DEFAULT_MODE: u32 = 0x00;
pub const SMA1307_OFFSET_BURNING_MODE: u32 = 0x01;

pub const SMA1307_SETTING_HEADER_SIZE: u32 = 0x08;
pub const SMA1307_SETTING_DEFAULT_SIZE: u32 = 0xC0;

pub const SMA1307_DEFAULT_SET: u32 = 0x00;
pub const SMA1307_BINARY_FILE_SET: u32 = 0x01;

/* Controls Name */
pub const SMA1307_REG_CTRL_NAME: &str = "Register Byte Control";
pub const SMA1307_VOL_CTRL_NAME: &str = "Speaker Volume";
pub const SMA1307_FORCE_MUTE_CTRL_NAME: &str = "Force Mute Switch";
pub const SMA1307_TDM_RX0_POS_NAME: &str = "TDM RX Slot0 Position";
pub const SMA1307_TDM_RX1_POS_NAME: &str = "TDM RX Slot1 Position";
pub const SMA1307_TDM_TX0_POS_NAME: &str = "TDM TX Slot0 Position";
pub const SMA1307_TDM_TX1_POS_NAME: &str = "TDM TX Slot1 Position";
pub const SMA1307_OT1_SW_PROT_CTRL_NAME: &str = "OT1 SW Protection Switch";
pub const SMA1307_RESET_CTRL_NAME: &str = "Reset Switch";
pub const SMA1307_CHECK_FAULT_STATUS_NAME: &str = "Check Fault Status";
pub const SMA1307_CHECK_FAULT_PERIOD_NAME: &str = "Check Fault Period";

/* DAPM Name */
pub const SMA1307_AIF_IN_NAME: &str = "AIF IN Source";
pub const SMA1307_AIF_OUT0_NAME: &str = "AIF OUT0 Source";
pub const SMA1307_AIF_OUT1_NAME: &str = "AIF OUT1 Source";

/*
 * SMA1307 Register Definition
 */

/* SMA1307 Register Addresses */
pub const SMA1307_00_SYSTEM_CTRL: u32 = 0x00;
pub const SMA1307_01_INPUT_CTRL1: u32 = 0x01;
pub const SMA1307_02_BROWN_OUT_PROT1: u32 = 0x02;
pub const SMA1307_03_BROWN_OUT_PROT2: u32 = 0x03;
pub const SMA1307_04_BROWN_OUT_PROT3: u32 = 0x04;
pub const SMA1307_05_BROWN_OUT_PROT8: u32 = 0x05;
pub const SMA1307_06_BROWN_OUT_PROT9: u32 = 0x06;
pub const SMA1307_07_BROWN_OUT_PROT10: u32 = 0x07;
pub const SMA1307_08_BROWN_OUT_PROT11: u32 = 0x08;
pub const SMA1307_09_OUTPUT_CTRL: u32 = 0x09;
pub const SMA1307_0A_SPK_VOL: u32 = 0x0A;
pub const SMA1307_0B_BST_TEST: u32 = 0x0B;
pub const SMA1307_0C_BOOST_CTRL8: u32 = 0x0C;
pub const SMA1307_0D_SPK_TEST: u32 = 0x0D;
pub const SMA1307_0E_MUTE_VOL_CTRL: u32 = 0x0E;
pub const SMA1307_0F_VBAT_TEMP_SENSING: u32 = 0x0F;

pub const SMA1307_10_SYSTEM_CTRL1: u32 = 0x10;
pub const SMA1307_11_SYSTEM_CTRL2: u32 = 0x11;
pub const SMA1307_12_SYSTEM_CTRL3: u32 = 0x12;
pub const SMA1307_13_DELAY: u32 = 0x13;
pub const SMA1307_14_MODULATOR: u32 = 0x14;
pub const SMA1307_15_BASS_SPK1: u32 = 0x15;
pub const SMA1307_16_BASS_SPK2: u32 = 0x16;
pub const SMA1307_17_BASS_SPK3: u32 = 0x17;
pub const SMA1307_18_BASS_SPK4: u32 = 0x18;
pub const SMA1307_19_BASS_SPK5: u32 = 0x19;
pub const SMA1307_1A_BASS_SPK6: u32 = 0x1A;
pub const SMA1307_1B_BASS_SPK7: u32 = 0x1B;
pub const SMA1307_1C_BROWN_OUT_PROT20: u32 = 0x1C;
pub const SMA1307_1D_BROWN_OUT_PROT0: u32 = 0x1D;
pub const SMA1307_1E_TONE_GENERATOR: u32 = 0x1E;
pub const SMA1307_1F_TONE_FINE_VOLUME: u32 = 0x1F;

pub const SMA1307_22_COMP_HYS_SEL: u32 = 0x22;
pub const SMA1307_23_COMPLIM1: u32 = 0x23;
pub const SMA1307_24_COMPLIM2: u32 = 0x24;
pub const SMA1307_25_COMPLIM3: u32 = 0x25;
pub const SMA1307_26_COMPLIM4: u32 = 0x26;
pub const SMA1307_27_BROWN_OUT_PROT4: u32 = 0x27;
pub const SMA1307_28_BROWN_OUT_PROT5: u32 = 0x28;
pub const SMA1307_29_BROWN_OUT_PROT12: u32 = 0x29;
pub const SMA1307_2A_BROWN_OUT_PROT13: u32 = 0x2A;
pub const SMA1307_2B_BROWN_OUT_PROT14: u32 = 0x2B;
pub const SMA1307_2C_BROWN_OUT_PROT15: u32 = 0x2C;
pub const SMA1307_2D_BROWN_OUT_PROT6: u32 = 0x2D;
pub const SMA1307_2E_BROWN_OUT_PROT7: u32 = 0x2E;
pub const SMA1307_2F_BROWN_OUT_PROT16: u32 = 0x2F;

pub const SMA1307_30_BROWN_OUT_PROT17: u32 = 0x30;
pub const SMA1307_31_BROWN_OUT_PROT18: u32 = 0x31;
pub const SMA1307_32_BROWN_OUT_PROT19: u32 = 0x32;
pub const SMA1307_34_OCP_SPK: u32 = 0x34;
pub const SMA1307_35_FDPEC_CTRL0: u32 = 0x35;
pub const SMA1307_36_PROTECTION: u32 = 0x36;
pub const SMA1307_37_SLOPECTRL: u32 = 0x37;
pub const SMA1307_38_POWER_METER: u32 = 0x38;
pub const SMA1307_39_PMT_NZ_VAL: u32 = 0x39;
pub const SMA1307_3B_TEST1: u32 = 0x3B;
pub const SMA1307_3C_TEST2: u32 = 0x3C;
pub const SMA1307_3D_TEST3: u32 = 0x3D;
pub const SMA1307_3E_IDLE_MODE_CTRL: u32 = 0x3E;
pub const SMA1307_3F_ATEST2: u32 = 0x3F;
pub const SMA1307_8B_PLL_POST_N: u32 = 0x8B;
pub const SMA1307_8C_PLL_N: u32 = 0x8C;
pub const SMA1307_8D_PLL_A_SETTING: u32 = 0x8D;
pub const SMA1307_8E_PLL_P_CP: u32 = 0x8E;
pub const SMA1307_8F_ANALOG_TEST: u32 = 0x8F;

pub const SMA1307_90_CRESTLIM1: u32 = 0x90;
pub const SMA1307_91_CRESTLIM2: u32 = 0x91;
pub const SMA1307_92_FDPEC_CTRL1: u32 = 0x92;
pub const SMA1307_93_INT_CTRL: u32 = 0x93;
pub const SMA1307_94_BOOST_CTRL9: u32 = 0x94;
pub const SMA1307_95_BOOST_CTRL10: u32 = 0x95;
pub const SMA1307_96_BOOST_CTRL11: u32 = 0x96;
pub const SMA1307_97_OTP_TRM0: u32 = 0x97;
pub const SMA1307_98_OTP_TRM1: u32 = 0x98;
pub const SMA1307_99_OTP_TRM2: u32 = 0x99;
pub const SMA1307_9A_OTP_TRM3: u32 = 0x9A;

pub const SMA1307_A0_PAD_CTRL0: u32 = 0xA0;
pub const SMA1307_A1_PAD_CTRL1: u32 = 0xA1;
pub const SMA1307_A2_TOP_MAN1: u32 = 0xA2;
pub const SMA1307_A3_TOP_MAN2: u32 = 0xA3;
pub const SMA1307_A4_TOP_MAN3: u32 = 0xA4;
pub const SMA1307_A5_TDM1: u32 = 0xA5;
pub const SMA1307_A6_TDM2: u32 = 0xA6;
pub const SMA1307_A7_CLK_MON: u32 = 0xA7;
pub const SMA1307_A8_BOOST_CTRL1: u32 = 0xA8;
pub const SMA1307_A9_BOOST_CTRL2: u32 = 0xA9;
pub const SMA1307_AA_BOOST_CTRL3: u32 = 0xAA;
pub const SMA1307_AB_BOOST_CTRL4: u32 = 0xAB;
pub const SMA1307_AC_BOOST_CTRL5: u32 = 0xAC;
pub const SMA1307_AD_BOOST_CTRL6: u32 = 0xAD;
pub const SMA1307_AE_BOOST_CTRL7: u32 = 0xAE;
pub const SMA1307_AF_LPF: u32 = 0xAF;

pub const SMA1307_B0_RMS_TC1: u32 = 0xB0;
pub const SMA1307_B1_RMS_TC2: u32 = 0xB1;
pub const SMA1307_B2_AVG_TC1: u32 = 0xB2;
pub const SMA1307_B3_AVG_TC2: u32 = 0xB3;
pub const SMA1307_B4_PRVALUE1: u32 = 0xB4;
pub const SMA1307_B5_PRVALUE2: u32 = 0xB5;
pub const SMA1307_B8_SPK_NG_CTRL1: u32 = 0xB8;
pub const SMA1307_B9_SPK_NG_CTRL2: u32 = 0xB9;
pub const SMA1307_BA_DGC1: u32 = 0xBA;
pub const SMA1307_BB_DGC2: u32 = 0xBB;
pub const SMA1307_BC_DGC3: u32 = 0xBC;
pub const SMA1307_BD_MCBS_CTRL1: u32 = 0xBD;
pub const SMA1307_BE_MCBS_CTRL2: u32 = 0xBE;

/* Status Register Read Only */
pub const SMA1307_F5_READY_FOR_V_SAR: u32 = 0xF5;
pub const SMA1307_F7_READY_FOR_T_SAR: u32 = 0xF7;
pub const SMA1307_F8_STATUS_T1: u32 = 0xF8;
pub const SMA1307_F9_STATUS_T2: u32 = 0xF9;
pub const SMA1307_FA_STATUS1: u32 = 0xFA;
pub const SMA1307_FB_STATUS2: u32 = 0xFB;
pub const SMA1307_FC_STATUS3: u32 = 0xFC;
pub const SMA1307_FD_STATUS4: u32 = 0xFD;
pub const SMA1307_FE_STATUS5: u32 = 0xFE;
pub const SMA1307_FF_DEVICE_INDEX: u32 = 0xFF;

/* SMA1307 Registers Bit Fields */
/* Power On/Off */
pub const SMA1307_POWER_MASK: u32 = BIT(0);
pub const SMA1307_POWER_OFF: u32 = 0;
pub const SMA1307_POWER_ON: u32 = BIT(0);

/* Reset */
pub const SMA1307_RESET_MASK: u32 = BIT(1);
pub const SMA1307_RESET_ON: u32 = BIT(1);

/* Left Polarity */
pub const SMA1307_LEFTPOL_MASK: u32 = BIT(3);
pub const SMA1307_LOW_FIRST_CH: u32 = 0;
pub const SMA1307_HIGH_FIRST_CH: u32 = BIT(3);

/* SCK Falling/Rising */
pub const SMA1307_SCK_RISING_MASK: u32 = BIT(2);
pub const SMA1307_SCK_FALLING_EDGE: u32 = 0;
pub const SMA1307_SCK_RISING_EDGE: u32 = BIT(2);

/* SPK Mute */
pub const SMA1307_SPK_MUTE_MASK: u32 = BIT(0);
pub const SMA1307_SPK_UNMUTE: u32 = 0;
pub const SMA1307_SPK_MUTE: u32 = BIT(0);

/* SPK Mode */
pub const SMA1307_SPK_MODE_MASK: u32 = BIT(2) | BIT(3) | BIT(4);
pub const SMA1307_SPK_OFF: u32 = 0;
pub const SMA1307_SPK_MONO: u32 = BIT(2);
pub const SMA1307_SPK_STEREO: u32 = BIT(4);

/* Mono Mix */
pub const SMA1307_MONOMIX_MASK: u32 = BIT(0);
pub const SMA1307_MONOMIX_OFF: u32 = 0;
pub const SMA1307_MONOMIX_ON: u32 = BIT(0);

/* LR Data Swap */
pub const SMA1307_LR_DATA_SW_MASK: u32 = BIT(4);
pub const SMA1307_LR_DATA_SW_NORMAL: u32 = 0;
pub const SMA1307_LR_DATA_SW_SWAP: u32 = BIT(4);

/* PLL On/Off */
pub const SMA1307_PLL_MASK: u32 = BIT(6);
pub const SMA1307_PLL_ON: u32 = 0;
pub const SMA1307_PLL_OFF: u32 = BIT(6);

/* Input Format */
pub const SMA1307_I2S_MODE_MASK: u32 = BIT(4) | BIT(5) | BIT(6);
pub const SMA1307_STANDARD_I2S: u32 = 0;
pub const SMA1307_LJ: u32 = BIT(4);
pub const SMA1307_RJ_16BIT: u32 = BIT(6);
pub const SMA1307_RJ_18BIT: u32 = BIT(4) | BIT(6);
pub const SMA1307_RJ_20BIT: u32 = BIT(5) | BIT(6);
pub const SMA1307_RJ_24BIT: u32 = BIT(4) | BIT(5) | BIT(6);

/* Controller / Device Setting */
pub const SMA1307_CONTROLLER_DEVICE_MASK: u32 = BIT(7);
pub const SMA1307_DEVICE_MODE: u32 = 0;
pub const SMA1307_CONTROLLER_MODE: u32 = BIT(7);

/* Port Config */
pub const SMA1307_PORT_CONFIG_MASK: u32 = BIT(6) | BIT(7);
pub const SMA1307_INPUT_PORT_ONLY: u32 = 0;
pub const SMA1307_OUTPUT_PORT_ENABLE: u32 = BIT(7);

/* SDO Output */
pub const SMA1307_SDO_OUTPUT_MASK: u32 = BIT(3);
pub const SMA1307_LOGIC_OUTPUT: u32 = 0;
pub const SMA1307_HIGH_Z_OUTPUT: u32 = BIT(3);

pub const SMA1307_DATA_CLK_SEL_MASK: u32 = BIT(6) | BIT(7);
pub const SMA1307_SDO_DATA: u32 = 0;
pub const SMA1307_SDO_CLK_PLL: u32 = BIT(6);
pub const SMA1307_SDO_CLK_OSC: u32 = BIT(6) | BIT(7);

/* SDO Output2 */
pub const SMA1307_SDO_OUTPUT2_MASK: u32 = BIT(0);
pub const SMA1307_ONE_SDO_PER_CH: u32 = 0;
pub const SMA1307_TWO_SDO_PER_CH: u32 = BIT(0);

/* SDO Output3 */
pub const SMA1307_SDO_OUTPUT3_MASK: u32 = BIT(2);
pub const SMA1307_SDO_OUTPUT3_DIS: u32 = 0;
pub const SMA1307_TWO_SDO_PER_CH_24K: u32 = BIT(2);

/* SDO OUT1 Select*/
pub const SMA1307_SDO_OUT1_SEL_MASK: u32 = BIT(3) | BIT(4) | BIT(5);
pub const SMA1307_SDO1_DISABLE: u32 = 0;
pub const SMA1307_SDO1_FORMAT_C: u32 = BIT(3);
pub const SMA1307_SDO1_MONO_MIX: u32 = BIT(4);
pub const SMA1307_SDO1_AFTER_DSP: u32 = BIT(3) | BIT(4);
pub const SMA1307_SDO1_VRMS2_AVG: u32 = BIT(5);
pub const SMA1307_SDO1_VBAT_MON: u32 = BIT(3) | BIT(5);
pub const SMA1307_SDO1_TEMP_MON: u32 = BIT(4) | BIT(5);
pub const SMA1307_SDO1_AFTER_DELAY: u32 = BIT(3) | BIT(4) | BIT(5);

/* SDO OUT0 Select*/
pub const SMA1307_SDO_OUT0_SEL_MASK: u32 = BIT(0) | BIT(1) | BIT(2);
pub const SMA1307_SDO0_DISABLE: u32 = 0;
pub const SMA1307_SDO0_FORMAT_C: u32 = BIT(0);
pub const SMA1307_SDO0_MONO_MIX: u32 = BIT(1);
pub const SMA1307_SDO0_AFTER_DSP: u32 = BIT(0) | BIT(1);
pub const SMA1307_SDO0_VRMS2_AVG: u32 = BIT(2);
pub const SMA1307_SDO0_VBAT_MON: u32 = BIT(0) | BIT(2);
pub const SMA1307_SDO0_TEMP_MON: u32 = BIT(1) | BIT(2);
pub const SMA1307_SDO0_AFTER_DELAY: u32 = BIT(0) | BIT(1) | BIT(2);

/* INTERRUPT Operation */
pub const SMA1307_SEL_INT_MASK: u32 = BIT(2);
pub const SMA1307_INT_CLEAR_AUTO: u32 = 0;
pub const SMA1307_INT_CLEAR_MANUAL: u32 = BIT(2);

/* INTERRUPT CLEAR */
pub const SMA1307_CLR_INT_MASK: u32 = BIT(1);
pub const SMA1307_INT_READY: u32 = 0;
pub const SMA1307_INT_CLEAR: u32 = BIT(1);

/* INTERRUPT Disable */
pub const SMA1307_DIS_INT_MASK: u32 = BIT(0);
pub const SMA1307_NORMAL_INT: u32 = 0;
pub const SMA1307_HIGH_Z_INT: u32 = BIT(0);

/* Interface Control */
pub const SMA1307_INTERFACE_MASK: u32 = BIT(5) | BIT(6) | BIT(7);
pub const SMA1307_LJ_FORMAT: u32 = BIT(5);
pub const SMA1307_I2S_FORMAT: u32 = BIT(5) | BIT(6);
pub const SMA1307_TDM_FORMAT: u32 = BIT(7);

pub const SMA1307_SCK_RATE_MASK: u32 = BIT(3) | BIT(4);
pub const SMA1307_SCK_64FS: u32 = 0;
pub const SMA1307_SCK_32FS: u32 = BIT(4);

pub const SMA1307_DATA_WIDTH_MASK: u32 = BIT(1) | BIT(2);
pub const SMA1307_DATA_24BIT: u32 = 0;
pub const SMA1307_DATA_16BIT: u32 = BIT(1) | BIT(2);

pub const SMA1307_TDM_TX_MODE_MASK: u32 = BIT(6);
pub const SMA1307_TDM_TX_MONO: u32 = 0;
pub const SMA1307_TDM_TX_STEREO: u32 = BIT(6);

pub const SMA1307_TDM_SLOT0_RX_POS_MASK: u32 = BIT(3) | BIT(4) | BIT(5);
pub const SMA1307_TDM_SLOT0_RX_POS_0: u32 = 0;
pub const SMA1307_TDM_SLOT0_RX_POS_1: u32 = BIT(3);
pub const SMA1307_TDM_SLOT0_RX_POS_2: u32 = BIT(4);
pub const SMA1307_TDM_SLOT0_RX_POS_3: u32 = BIT(3) | BIT(4);
pub const SMA1307_TDM_SLOT0_RX_POS_4: u32 = BIT(5);
pub const SMA1307_TDM_SLOT0_RX_POS_5: u32 = BIT(3) | BIT(5);
pub const SMA1307_TDM_SLOT0_RX_POS_6: u32 = BIT(4) | BIT(5);
pub const SMA1307_TDM_SLOT0_RX_POS_7: u32 = BIT(3) | BIT(4) | BIT(5);

pub const SMA1307_TDM_SLOT1_RX_POS_MASK: u32 = BIT(0) | BIT(1) | BIT(2);
pub const SMA1307_TDM_SLOT1_RX_POS_0: u32 = 0;
pub const SMA1307_TDM_SLOT1_RX_POS_1: u32 = BIT(0);
pub const SMA1307_TDM_SLOT1_RX_POS_2: u32 = BIT(1);
pub const SMA1307_TDM_SLOT1_RX_POS_3: u32 = BIT(0) | BIT(1);
pub const SMA1307_TDM_SLOT1_RX_POS_4: u32 = BIT(2);
pub const SMA1307_TDM_SLOT1_RX_POS_5: u32 = BIT(0) | BIT(2);
pub const SMA1307_TDM_SLOT1_RX_POS_6: u32 = BIT(1) | BIT(2);
pub const SMA1307_TDM_SLOT1_RX_POS_7: u32 = BIT(0) | BIT(1) | BIT(2);

/* TDM2 FORMAT : 0xA6 */
pub const SMA1307_TDM_DL_MASK: u32 = BIT(7);
pub const SMA1307_TDM_DL_16: u32 = 0;
pub const SMA1307_TDM_DL_32: u32 = BIT(7);

pub const SMA1307_TDM_N_SLOT_MASK: u32 = BIT(6);
pub const SMA1307_TDM_N_SLOT_4: u32 = 0;
pub const SMA1307_TDM_N_SLOT_8: u32 = BIT(6);

pub const SMA1307_TDM_SLOT0_TX_POS_MASK: u32 = BIT(3) | BIT(4) | BIT(5);
pub const SMA1307_TDM_SLOT0_TX_POS_0: u32 = 0;
pub const SMA1307_TDM_SLOT0_TX_POS_1: u32 = BIT(3);
pub const SMA1307_TDM_SLOT0_TX_POS_2: u32 = BIT(4);
pub const SMA1307_TDM_SLOT0_TX_POS_3: u32 = BIT(3) | BIT(4);
pub const SMA1307_TDM_SLOT0_TX_POS_4: u32 = BIT(5);
pub const SMA1307_TDM_SLOT0_TX_POS_5: u32 = BIT(3) | BIT(5);
pub const SMA1307_TDM_SLOT0_TX_POS_6: u32 = BIT(4) | BIT(5);
pub const SMA1307_TDM_SLOT0_TX_POS_7: u32 = BIT(3) | BIT(4) | BIT(5);

pub const SMA1307_TDM_SLOT1_TX_POS_MASK: u32 = BIT(0) | BIT(1) | BIT(2);
pub const SMA1307_TDM_SLOT1_TX_POS_0: u32 = 0;
pub const SMA1307_TDM_SLOT1_TX_POS_1: u32 = BIT(0);
pub const SMA1307_TDM_SLOT1_TX_POS_2: u32 = BIT(1);
pub const SMA1307_TDM_SLOT1_TX_POS_3: u32 = BIT(0) | BIT(1);
pub const SMA1307_TDM_SLOT1_TX_POS_4: u32 = BIT(2);
pub const SMA1307_TDM_SLOT1_TX_POS_5: u32 = BIT(0) | BIT(2);
pub const SMA1307_TDM_SLOT1_TX_POS_6: u32 = BIT(1) | BIT(2);
pub const SMA1307_TDM_SLOT1_TX_POS_7: u32 = BIT(0) | BIT(1) | BIT(2);

/* OTP STATUS */
pub const SMA1307_OTP_STAT_MASK: u32 = BIT(6);
pub const SMA1307_OTP_STAT_0: u32 = 0;
pub const SMA1307_OTP_STAT_1: u32 = BIT(6);

/* STATUS */
pub const SMA1307_OT1_OK_STATUS: u32 = BIT(7);
pub const SMA1307_OT2_OK_STATUS: u32 = BIT(6);
pub const SMA1307_UVLO_STATUS: u32 = BIT(5);
pub const SMA1307_OVP_BST_STATUS: u32 = BIT(4);
pub const SMA1307_POWER_FLAG: u32 = BIT(3);

pub const SMA1307_SCAN_CHK: u32 = BIT(7);
pub const SMA1307_OCP_SPK_STATUS: u32 = BIT(5);
pub const SMA1307_OCP_BST_STATUS: u32 = BIT(4);
pub const SMA1307_BOP_STATE: u32 = BIT(1) | BIT(2) | BIT(3);
pub const SMA1307_CLK_MON_STATUS: u32 = BIT(0);

pub const SMA1307_DEVICE_ID: u32 = BIT(3) | BIT(4);
pub const SMA1307_REV_NUM_STATUS: u32 = BIT(0) | BIT(1);
pub const SMA1307_REV_NUM_REV0: u32 = 0;
pub const SMA1307_REV_NUM_REV1: u32 = BIT(0);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
