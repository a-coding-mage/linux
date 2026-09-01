/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * sma1303.h -- sma1303 ALSA SoC Audio driver
 *
 * Copyright 2023 Iron Device Corporation
 *
 * Author: Kiseok Jo <kiseok.jo@irondevice.com>
 *
 */


pub const SMA1303_I2C_ADDR_00: u32 = 0x1eu32;
pub const SMA1303_I2C_ADDR_01: u32 = 0x3eu32;
pub const SMA1303_I2C_ADDR_10: u32 = 0x5eu32;
pub const SMA1303_I2C_ADDR_11: u32 = 0x7eu32;

pub const SMA1303_EXTERNAL_CLOCK_19_2: u32 = 0x00u32;
pub const SMA1303_EXTERNAL_CLOCK_24_576: u32 = 0x01u32;
pub const SMA1303_PLL_CLKIN_MCLK: u32 = 0x02u32;
pub const SMA1303_PLL_CLKIN_BCLK: u32 = 0x03u32;

pub const SMA1303_MONO: u32 = 0x00u32;
pub const SMA1303_STEREO: u32 = 0x01u32;

pub const SMA1303_I2C_RETRY_COUNT: u32 = 3u32;

/*
 * SMA1303 Register Definition
 */

/* SMA1303 Register Addresses */
pub const SMA1303_00_SYSTEM_CTRL: u32 = 0x00u32;
pub const SMA1303_01_INPUT1_CTRL1: u32 = 0x01u32;
pub const SMA1303_02_INPUT1_CTRL2: u32 = 0x02u32;
pub const SMA1303_03_INPUT1_CTRL3: u32 = 0x03u32;
pub const SMA1303_04_INPUT1_CTRL4: u32 = 0x04u32;
/* 0x05 ~ 0x08 : Reserved */
pub const SMA1303_09_OUTPUT_CTRL: u32 = 0x09u32;
pub const SMA1303_0A_SPK_VOL: u32 = 0x0au32;
pub const SMA1303_0B_BST_TEST: u32 = 0x0bu32;
pub const SMA1303_0C_BST_TEST1: u32 = 0x0cu32;
pub const SMA1303_0D_SPK_TEST: u32 = 0x0du32;
pub const SMA1303_0E_MUTE_VOL_CTRL: u32 = 0x0eu32;
/* 0x0F : Reserved */
pub const SMA1303_10_SYSTEM_CTRL1: u32 = 0x10u32;
pub const SMA1303_11_SYSTEM_CTRL2: u32 = 0x11u32;
pub const SMA1303_12_SYSTEM_CTRL3: u32 = 0x12u32;
/* 0x13 : Reserved */
pub const SMA1303_14_MODULATOR: u32 = 0x14u32;
pub const SMA1303_15_BASS_SPK1: u32 = 0x15u32;
pub const SMA1303_16_BASS_SPK2: u32 = 0x16u32;
pub const SMA1303_17_BASS_SPK3: u32 = 0x17u32;
pub const SMA1303_18_BASS_SPK4: u32 = 0x18u32;
pub const SMA1303_19_BASS_SPK5: u32 = 0x19u32;
pub const SMA1303_1A_BASS_SPK6: u32 = 0x1au32;
pub const SMA1303_1B_BASS_SPK7: u32 = 0x1bu32;
/* 0x1C ~ 0x22 : Reserved */
pub const SMA1303_23_COMP_LIM1: u32 = 0x23u32;
pub const SMA1303_24_COMP_LIM2: u32 = 0x24u32;
pub const SMA1303_25_COMP_LIM3: u32 = 0x25u32;
pub const SMA1303_26_COMP_LIM4: u32 = 0x26u32;
/* 0x27 ~ 0x32 : Reserved */
pub const SMA1303_33_SDM_CTRL: u32 = 0x33u32;
pub const SMA1303_34_OTP_DATA1: u32 = 0x34u32;
/* 0x35 : Reserved */
pub const SMA1303_36_PROTECTION: u32 = 0x36u32;
pub const SMA1303_37_SLOPE_CTRL: u32 = 0x37u32;
pub const SMA1303_38_OTP_TRM0: u32 = 0x38u32;
/* 0x39 ~ 0x3A : Reserved */
pub const SMA1303_3B_TEST1: u32 = 0x3bu32;
pub const SMA1303_3C_TEST2: u32 = 0x3cu32;
pub const SMA1303_3D_TEST3: u32 = 0x3du32;
pub const SMA1303_3E_ATEST1: u32 = 0x3eu32;
pub const SMA1303_3F_ATEST2: u32 = 0x3fu32;
/* 0x40 ~ 0x8A : Reserved */
pub const SMA1303_8B_PLL_POST_N: u32 = 0x8bu32;
pub const SMA1303_8C_PLL_N: u32 = 0x8cu32;
pub const SMA1303_8D_PLL_A_SETTING: u32 = 0x8du32;
pub const SMA1303_8E_PLL_CTRL: u32 = 0x8eu32;
pub const SMA1303_8F_PLL_P_CP: u32 = 0x8fu32;
pub const SMA1303_90_POSTSCALER: u32 = 0x90u32;
pub const SMA1303_91_CLASS_G_CTRL: u32 = 0x91u32;
pub const SMA1303_92_FDPEC_CTRL: u32 = 0x92u32;
/* 0x93 : Reserved */
pub const SMA1303_94_BOOST_CTRL1: u32 = 0x94u32;
pub const SMA1303_95_BOOST_CTRL2: u32 = 0x95u32;
pub const SMA1303_96_BOOST_CTRL3: u32 = 0x96u32;
pub const SMA1303_97_BOOST_CTRL4: u32 = 0x97u32;
/* 0x98 ~ 0x9F : Reserved */
pub const SMA1303_A0_PAD_CTRL0: u32 = 0xa0u32;
pub const SMA1303_A1_PAD_CTRL1: u32 = 0xa1u32;
pub const SMA1303_A2_TOP_MAN1: u32 = 0xa2u32;
pub const SMA1303_A3_TOP_MAN2: u32 = 0xa3u32;
pub const SMA1303_A4_TOP_MAN3: u32 = 0xa4u32;
pub const SMA1303_A5_TDM1: u32 = 0xa5u32;
pub const SMA1303_A6_TDM2: u32 = 0xa6u32;
pub const SMA1303_A7_CLK_MON: u32 = 0xa7u32;
/* 0xA8 ~ 0xF9 : Reserved */
pub const SMA1303_FA_STATUS1: u32 = 0xfau32;
pub const SMA1303_FB_STATUS2: u32 = 0xfbu32;
/* 0xFC ~ 0xFE : Reserved */
pub const SMA1303_FF_DEVICE_INDEX: u32 = 0xffu32;

/* SMA1303 Registers Bit Fields */

/* SYSTEM_CTRL : 0x00 */
pub const SMA1303_RESETBYI2C_MASK: u32 = ((1u32) << 1u32);
pub const SMA1303_RESETBYI2C_NORMAL: u32 = ((0u32) << 1u32);
pub const SMA1303_RESETBYI2C_RESET: u32 = ((1u32) << 1u32);

pub const SMA1303_POWER_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_POWER_OFF: u32 = ((0u32) << 0u32);
pub const SMA1303_POWER_ON: u32 = ((1u32) << 0u32);

/* INTPUT CTRL1 : 0x01 */
pub const SMA1303_CONTROLLER_DEVICE_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_DEVICE_MODE: u32 = ((0u32) << 7u32);
pub const SMA1303_CONTROLLER_MODE: u32 = ((1u32) << 7u32);

pub const SMA1303_I2S_MODE_MASK: u32 = ((7u32) << 4u32);
pub const SMA1303_STANDARD_I2S: u32 = ((0u32) << 4u32);
pub const SMA1303_LJ: u32 = ((1u32) << 4u32);
pub const SMA1303_RJ_16BIT: u32 = ((4u32) << 4u32);
pub const SMA1303_RJ_18BIT: u32 = ((5u32) << 4u32);
pub const SMA1303_RJ_20BIT: u32 = ((6u32) << 4u32);
pub const SMA1303_RJ_24BIT: u32 = ((7u32) << 4u32);

pub const SMA1303_LEFTPOL_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_LOW_FIRST_CH: u32 = ((0u32) << 3u32);
pub const SMA1303_HIGH_FIRST_CH: u32 = ((1u32) << 3u32);

pub const SMA1303_SCK_RISING_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_SCK_FALLING_EDGE: u32 = ((0u32) << 2u32);
pub const SMA1303_SCK_RISING_EDGE: u32 = ((1u32) << 2u32);

/* INTPUT CTRL2 : 0x02 */
pub const SMA1303_IMODE_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_I2S: u32 = ((0u32) << 6u32);
pub const SMA1303_PCM_SHORT: u32 = ((1u32) << 6u32);
pub const SMA1303_PCM_LONG: u32 = ((2u32) << 6u32);

pub const RSMA1303_IGHT_FIRST_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_LEFT_NORMAL: u32 = ((0u32) << 5u32);
pub const SMA1303_RIGHT_INVERTED: u32 = ((1u32) << 5u32);

pub const SMA1303_PCM_ALAW_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_PCM_U_DECODING: u32 = ((0u32) << 4u32);
pub const SMA1303_PCM_A_DECODING: u32 = ((1u32) << 4u32);

pub const SMA1303_PCM_COMP_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_PCM_LINEAR: u32 = ((0u32) << 3u32);
pub const SMA1303_PCM_COMPANDING: u32 = ((1u32) << 3u32);

pub const SMA1303_INPUTSEL_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_PCM_8KHZ: u32 = ((0u32) << 2u32);
pub const SMA1303_PCM_16KHZ: u32 = ((1u32) << 2u32);

pub const SMA1303_PCM_STEREO_MASK: u32 = ((1u32) << 1u32);
pub const SMA1303_PCM_MONO: u32 = ((0u32) << 1u32);
pub const SMA1303_PCM_STEREO: u32 = ((1u32) << 1u32);

pub const SMA1303_PCM_DL_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_PCM_8BIT: u32 = ((0u32) << 0u32);
pub const SMA1303_PCM_16BIT: u32 = ((1u32) << 0u32);

/* INTPUT CTRL3 : 0x03 */
pub const SMA1303_PCM_N_SLOT_MASK: u32 = ((15u32) << 0u32);
pub const SMA1303_PCM_N_SLOT1: u32 = ((0u32) << 0u32);
pub const SMA1303_PCM_N_SLOT2: u32 = ((1u32) << 0u32);
pub const SMA1303_PCM_N_SLOT3: u32 = ((2u32) << 0u32);
pub const SMA1303_PCM_N_SLOT4: u32 = ((3u32) << 0u32);
pub const SMA1303_PCM_N_SLOT5: u32 = ((4u32) << 0u32);
pub const SMA1303_PCM_N_SLOT6: u32 = ((5u32) << 0u32);
pub const SMA1303_PCM_N_SLOT7: u32 = ((6u32) << 0u32);
pub const SMA1303_PCM_N_SLOT8: u32 = ((7u32) << 0u32);
pub const SMA1303_PCM_N_SLOT9: u32 = ((8u32) << 0u32);
pub const SMA1303_PCM_N_SLOT10: u32 = ((9u32) << 0u32);
pub const SMA1303_PCM_N_SLOT11: u32 = ((10u32) << 0u32);
pub const SMA1303_PCM_N_SLOT12: u32 = ((11u32) << 0u32);
pub const SMA1303_PCM_N_SLOT13: u32 = ((12u32) << 0u32);
pub const SMA1303_PCM_N_SLOT14: u32 = ((13u32) << 0u32);
pub const SMA1303_PCM_N_SLOT15: u32 = ((14u32) << 0u32);
pub const SMA1303_PCM_N_SLOT16: u32 = ((15u32) << 0u32);

/* INTPUT CTRL4 : 0x04 */
pub const SMA1303_PCM1_SLOT_MASK: u32 = ((15u32) << 4u32);
pub const SMA1303_PCM1_SLOT1: u32 = ((0u32) << 4u32);
pub const SMA1303_PCM1_SLOT2: u32 = ((1u32) << 4u32);
pub const SMA1303_PCM1_SLOT3: u32 = ((2u32) << 4u32);
pub const SMA1303_PCM1_SLOT4: u32 = ((3u32) << 4u32);
pub const SMA1303_PCM1_SLOT5: u32 = ((4u32) << 4u32);
pub const SMA1303_PCM1_SLOT6: u32 = ((5u32) << 4u32);
pub const SMA1303_PCM1_SLOT7: u32 = ((6u32) << 4u32);
pub const SMA1303_PCM1_SLOT8: u32 = ((7u32) << 4u32);
pub const SMA1303_PCM1_SLOT9: u32 = ((8u32) << 4u32);
pub const SMA1303_PCM1_SLOT10: u32 = ((9u32) << 4u32);
pub const SMA1303_PCM1_SLOT11: u32 = ((10u32) << 4u32);
pub const SMA1303_PCM1_SLOT12: u32 = ((11u32) << 4u32);
pub const SMA1303_PCM1_SLOT13: u32 = ((12u32) << 4u32);
pub const SMA1303_PCM1_SLOT14: u32 = ((13u32) << 4u32);
pub const SMA1303_PCM1_SLOT15: u32 = ((14u32) << 4u32);
pub const SMA1303_PCM1_SLOT16: u32 = ((15u32) << 4u32);

pub const SMA1303_PCM2_SLOT_MASK: u32 = ((15u32) << 0u32);
pub const SMA1303_PCM2_SLOT1: u32 = ((0u32) << 0u32);
pub const SMA1303_PCM2_SLOT2: u32 = ((1u32) << 0u32);
pub const SMA1303_PCM2_SLOT3: u32 = ((2u32) << 0u32);
pub const SMA1303_PCM2_SLOT4: u32 = ((3u32) << 0u32);
pub const SMA1303_PCM2_SLOT5: u32 = ((4u32) << 0u32);
pub const SMA1303_PCM2_SLOT6: u32 = ((5u32) << 0u32);
pub const SMA1303_PCM2_SLOT7: u32 = ((6u32) << 0u32);
pub const SMA1303_PCM2_SLOT8: u32 = ((7u32) << 0u32);
pub const SMA1303_PCM2_SLOT9: u32 = ((8u32) << 0u32);
pub const SMA1303_PCM2_SLOT10: u32 = ((9u32) << 0u32);
pub const SMA1303_PCM2_SLOT11: u32 = ((10u32) << 0u32);
pub const SMA1303_PCM2_SLOT12: u32 = ((11u32) << 0u32);
pub const SMA1303_PCM2_SLOT13: u32 = ((12u32) << 0u32);
pub const SMA1303_PCM2_SLOT14: u32 = ((13u32) << 0u32);
pub const SMA1303_PCM2_SLOT15: u32 = ((14u32) << 0u32);
pub const SMA1303_PCM2_SLOT16: u32 = ((15u32) << 0u32);

/* OUTPUT CTRL : 0x09 */
pub const SMA1303_PORT_CONFIG_MASK: u32 = ((3u32) << 5u32);
pub const SMA1303_INPUT_PORT_ONLY: u32 = ((0u32) << 5u32);
pub const SMA1303_OUTPUT_PORT_ENABLE: u32 = ((2u32) << 5u32);

pub const SMA1303_PORT_OUT_SEL_MASK: u32 = ((7u32) << 0u32);
pub const SMA1303_OUT_SEL_DISABLE: u32 = ((0u32) << 0u32);
pub const SMA1303_FORMAT_CONVERTER: u32 = ((1u32) << 0u32);
pub const SMA1303_MIXER_OUTPUT: u32 = ((2u32) << 0u32);
pub const SMA1303_SPEAKER_PATH: u32 = ((3u32) << 0u32);
pub const SMA1303_POSTSCALER_OUTPUT: u32 = ((4u32) << 0u32);

/* BST_TEST : 0x0B */
pub const SMA1303_BST_OFF_SLOPE_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_BST_OFF_SLOPE_6_7ns: u32 = ((0u32) << 6u32);
pub const SMA1303_BST_OFF_SLOPE_4_8ns: u32 = ((1u32) << 6u32);
pub const SMA1303_BST_OFF_SLOPE_2_6ns: u32 = ((2u32) << 6u32);
pub const SMA1303_BST_OFF_SLOPE_1_2ns: u32 = ((3u32) << 6u32);

pub const SMA1303_OCP_TEST_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_OCP_NORMAL_MODE: u32 = ((0u32) << 5u32);
pub const SMA1303_OCP_TEST_MODE: u32 = ((1u32) << 5u32);

pub const SMA1303_BST_FAST_LEBN_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_BST_SHORT_LEB: u32 = ((0u32) << 4u32);
pub const SMA1303_BST_LONG_LEB: u32 = ((1u32) << 4u32);

pub const SMA1303_HIGH_PGAIN_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_NORMAL_P_GAIN: u32 = ((0u32) << 3u32);
pub const SMA1303_HIGH_P_GAIN: u32 = ((1u32) << 3u32);

pub const SMA1303_VCOMP_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_VCOMP_NORMAL_MODE: u32 = ((0u32) << 2u32);
pub const SMA1303_VCOMP_V_MON_MODE: u32 = ((1u32) << 2u32);

pub const SMA1303_PMOS_ON_MASK: u32 = ((1u32) << 1u32);
pub const SMA1303_PMOS_NORMAL_MODE: u32 = ((0u32) << 1u32);
pub const SMA1303_PMOS_TEST_MODE: u32 = ((1u32) << 1u32);

pub const SMA1303_NMOS_ON_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_NMOS_NORMAL_MODE: u32 = ((0u32) << 0u32);
pub const SMA1303_NMOS_TEST_MODE: u32 = ((1u32) << 0u32);

/* BST_TEST1 : 0x0C */
pub const SMA1303_SET_OCP_H_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_HIGH_OCP_4_5_LVL: u32 = ((0u32) << 6u32);
pub const SMA1303_HIGH_OCP_3_2_LVL: u32 = ((1u32) << 6u32);
pub const SMA1303_HIGH_OCP_2_1_LVL: u32 = ((2u32) << 6u32);
pub const SMA1303_HIGH_OCP_0_9_LVL: u32 = ((3u32) << 6u32);

pub const SMA1303_OCL_TEST_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_OCL_NORMAL_MODE: u32 = ((0u32) << 5u32);
pub const SMA1303_OCL_TEST_MODE: u32 = ((1u32) << 5u32);

pub const SMA1303_LOOP_CHECK_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_BST_LOOP_NORMAL_MODE: u32 = ((0u32) << 4u32);
pub const SMA1303_BST_LOOP_CHECK_MODE: u32 = ((1u32) << 4u32);

pub const SMA1303_EN_SH_PRT_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_EN_SH_PRT_DISABLE: u32 = ((0u32) << 3u32);
pub const SMA1303_EN_SH_PRT_ENABLE: u32 = ((1u32) << 3u32);

/* SPK_TEST : 0x0D */
pub const SMA1303_VREF_MON_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_VREF_NORMAL_MODE: u32 = ((0u32) << 3u32);
pub const SMA1303_VREF_V_MON_MODE: u32 = ((1u32) << 3u32);

pub const SMA1303_SPK_OCP_DLYN_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_SPK_OCP_LONG_DELAY: u32 = ((0u32) << 2u32);
pub const SMA1303_SPK_OCP_NORMAL: u32 = ((1u32) << 2u32);

pub const SMA1303_SPK_OFF_SLOPE_MASK: u32 = ((3u32) << 0u32);
pub const SMA1303_SPK_OFF_SLOPE_SLOW: u32 = ((0u32) << 0u32);
pub const SMA1303_SPK_OFF_SLOPE_FAST: u32 = ((3u32) << 0u32);

/* MUTE_VOL_CTRL : 0x0E */
pub const SMA1303_VOL_SLOPE_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_VOL_SLOPE_OFF: u32 = ((0u32) << 6u32);
pub const SMA1303_VOL_SLOPE_SLOW: u32 = ((1u32) << 6u32);
pub const SMA1303_VOL_SLOPE_MID: u32 = ((2u32) << 6u32);
pub const SMA1303_VOL_SLOPE_FAST: u32 = ((3u32) << 6u32);

pub const SMA1303_MUTE_SLOPE_MASK: u32 = ((3u32) << 4u32);
pub const SMA1303_MUTE_SLOPE_OFF: u32 = ((0u32) << 4u32);
pub const SMA1303_MUTE_SLOPE_SLOW: u32 = ((1u32) << 4u32);
pub const SMA1303_MUTE_SLOPE_MID: u32 = ((2u32) << 4u32);
pub const SMA1303_MUTE_SLOPE_FAST: u32 = ((3u32) << 4u32);

pub const SMA1303_SPK_MUTE_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_SPK_UNMUTE: u32 = ((0u32) << 0u32);
pub const SMA1303_SPK_MUTE: u32 = ((1u32) << 0u32);

/* SYSTEM_CTRL1 :0x10 */
pub const SMA1303_SPK_MODE_MASK: u32 = ((7u32) << 2u32);
pub const SMA1303_SPK_OFF: u32 = ((0u32) << 2u32);
pub const SMA1303_SPK_MONO: u32 = ((1u32) << 2u32);
pub const SMA1303_SPK_STEREO: u32 = ((4u32) << 2u32);

/* SYSTEM_CTRL2 : 0x11 */
pub const SMA1303_SPK_BS_MASK: u32 = ((1u32) << 6u32);
pub const SMA1303_SPK_BS_BYP: u32 = ((0u32) << 6u32);
pub const SMA1303_SPK_BS_EN: u32 = ((1u32) << 6u32);
pub const SMA1303_SPK_LIM_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_SPK_LIM_BYP: u32 = ((0u32) << 5u32);
pub const SMA1303_SPK_LIM_EN: u32 = ((1u32) << 5u32);

pub const SMA1303_LR_DATA_SW_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_LR_DATA_SW_NORMAL: u32 = ((0u32) << 4u32);
pub const SMA1303_LR_DATA_SW_SWAP: u32 = ((1u32) << 4u32);

pub const SMA1303_MONOMIX_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_MONOMIX_OFF: u32 = ((0u32) << 0u32);
pub const SMA1303_MONOMIX_ON: u32 = ((1u32) << 0u32);

/* SYSTEM_CTRL3 : 0x12 */
pub const SMA1303_INPUT_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_INPUT_0_DB: u32 = ((0u32) << 6u32);
pub const SMA1303_INPUT_M6_DB: u32 = ((1u32) << 6u32);
pub const SMA1303_INPUT_M12_DB: u32 = ((2u32) << 6u32);
pub const SMA1303_INPUT_INFI_DB: u32 = ((3u32) << 6u32);
pub const SMA1303_INPUT_R_MASK: u32 = ((3u32) << 4u32);
pub const SMA1303_INPUT_R_0_DB: u32 = ((0u32) << 4u32);
pub const SMA1303_INPUT_R_M6_DB: u32 = ((1u32) << 4u32);
pub const SMA1303_INPUT_R_M12_DB: u32 = ((2u32) << 4u32);
pub const SMA1303_INPUT_R_INFI_DB: u32 = ((3u32) << 4u32);

/* Modulator : 0x14 */
pub const SMA1303_SPK_HYSFB_MASK: u32 = ((3u32) << 6u32);
pub const SMA1303_HYSFB_625K: u32 = ((0u32) << 6u32);
pub const SMA1303_HYSFB_414K: u32 = ((1u32) << 6u32);
pub const SMA1303_HYSFB_297K: u32 = ((2u32) << 6u32);
pub const SMA1303_HYSFB_226K: u32 = ((3u32) << 6u32);
pub const SMA1303_SPK_BDELAY_MASK: u32 = ((63u32) << 0u32);

/* SDM CONTROL : 0x33 */
pub const SMA1303_SDM_Q_SEL_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_QUART_SEL_1_DIV_4: u32 = ((0u32) << 2u32);
pub const SMA1303_QUART_SEL_1_DIV_8: u32 = ((1u32) << 2u32);

/* OTP_DATA1 : 0x34 */
pub const SMA1303_OTP_LVL_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_OTP_LVL_NORMAL: u32 = ((0u32) << 5u32);
pub const SMA1303_OTP_LVL_LOW: u32 = ((1u32) << 5u32);

/* PROTECTION : 0x36 */
pub const SMA1303_EDGE_DIS_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_EDGE_DIS_ENABLE: u32 = ((0u32) << 7u32);
pub const SMA1303_EDGE_DIS_DISABLE: u32 = ((1u32) << 7u32);

pub const SMA1303_SPK_OCP_DIS_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_SPK_OCP_ENABLE: u32 = ((0u32) << 3u32);
pub const SMA1303_SPK_OCP_DISABLE: u32 = ((1u32) << 3u32);

pub const SMA1303_OCP_MODE_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_AUTO_RECOVER: u32 = ((0u32) << 2u32);
pub const SMA1303_SHUT_DOWN_PERMANENT: u32 = ((1u32) << 2u32);

pub const SMA1303_OTP_MODE_MASK: u32 = ((3u32) << 0u32);
pub const SMA1303_OTP_MODE_DISABLE: u32 = ((0u32) << 0u32);
pub const SMA1303_IG_THR1_SHUT_THR2: u32 = ((1u32) << 0u32);
pub const SMA1303_REC_THR1_SHUT_THR2: u32 = ((2u32) << 0u32);
pub const SMA1303_SHUT_THR1_SHUT_THR2: u32 = ((3u32) << 0u32);

/* TEST2 : 0x3C */
pub const SMA1303_SPK_HSDM_BP_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_SPK_HSDM_ENABLE: u32 = ((0u32) << 4u32);
pub const SMA1303_SPK_HSDM_BYPASS: u32 = ((1u32) << 4u32);

pub const SMA1303_SDM_SYNC_DIS_MASK: u32 = ((1u32) << 5u32);
pub const SMA1303_SDM_SYNC_NORMAL: u32 = ((0u32) << 5u32);
pub const SMA1303_SDM_SYNC_DISABLE: u32 = ((1u32) << 5u32);

/* ATEST2 : 0x3F */
pub const SMA1303_SPK_OUT_FREQ_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_SPK_OUT_FREQ_360K: u32 = ((0u32) << 2u32);
pub const SMA1303_SPK_OUT_FREQ_410K: u32 = ((1u32) << 2u32);

pub const SMA1303_LOW_POWER_MODE_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_LOW_POWER_MODE_DISABLE: u32 = ((0u32) << 3u32);
pub const SMA1303_LOW_POWER_MODE_ENABLE: u32 = ((1u32) << 3u32);

pub const SMA1303_THERMAL_ADJUST_MASK: u32 = ((3u32) << 5u32);
pub const SMA1303_THERMAL_150_110: u32 = ((0u32) << 5u32);
pub const SMA1303_THERMAL_160_120: u32 = ((1u32) << 5u32);
pub const SMA1303_THERMAL_140_100: u32 = ((2u32) << 5u32);

pub const SMA1303_FAST_OFF_DRIVE_SPK_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_FAST_OFF_DRIVE_SPK_DISABLE: u32 = ((0u32) << 0u32);
pub const SMA1303_FAST_OFF_DRIVE_SPK_ENABLE: u32 = ((1u32) << 0u32);

/* PLL_CTRL : 0x8E */
pub const SMA1303_TRM_LVL_MASK: u32 = ((1u32) << 4u32);
pub const SMA1303_TRM_LVL_NORMAL: u32 = ((0u32) << 4u32);
pub const SMA1303_TRM_LVL_LOW: u32 = ((1u32) << 4u32);

pub const SMA1303_LOW_OCL_MODE_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_LOW_OCL_MODE: u32 = ((0u32) << 3u32);
pub const SMA1303_NORMAL_OCL_MODE: u32 = ((1u32) << 3u32);

pub const SMA1303_PLL_PD2_MASK: u32 = ((7u32) << 0u32);
pub const SMA1303_PLL_PD2: u32 = ((7u32) << 0u32);
pub const SMA1303_PLL_OPERATION2: u32 = ((0u32) << 0u32);

/* POSTSCALER : 0x90 */
pub const SMA1303_BYP_POST_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_EN_POST_SCALER: u32 = ((0u32) << 0u32);
pub const SMA1303_BYP_POST_SCALER: u32 = ((1u32) << 0u32);

/* FDPEC CONTROL : 0x92 */
pub const SMA1303_FLT_VDD_GAIN_MASK: u32 = ((15u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P40: u32 = ((0u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P45: u32 = ((1u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P50: u32 = ((2u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P55: u32 = ((3u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P60: u32 = ((4u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P65: u32 = ((5u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P70: u32 = ((6u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P75: u32 = ((7u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P80: u32 = ((8u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P85: u32 = ((9u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P90: u32 = ((10u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_2P95: u32 = ((11u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_3P00: u32 = ((12u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_3P05: u32 = ((13u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_3P10: u32 = ((14u32) << 4u32);
pub const SMA1303_FLT_VDD_GAIN_3P15: u32 = ((15u32) << 4u32);

pub const SMA1303_DIS_FCHG_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_EN_FAST_CHARGE: u32 = ((0u32) << 2u32);
pub const SMA1303_DIS_FAST_CHARGE: u32 = ((1u32) << 2u32);

/* BOOST_CONTROL4 : 0x97 */
pub const SMA1303_TRM_VBST_MASK: u32 = ((7u32) << 2u32);
pub const SMA1303_TRM_VBST_5P5: u32 = ((0u32) << 2u32);
pub const SMA1303_TRM_VBST_5P6: u32 = ((1u32) << 2u32);
pub const SMA1303_TRM_VBST_5P7: u32 = ((2u32) << 2u32);
pub const SMA1303_TRM_VBST_5P8: u32 = ((3u32) << 2u32);
pub const SMA1303_TRM_VBST_5P9: u32 = ((4u32) << 2u32);
pub const SMA1303_TRM_VBST_6P0: u32 = ((5u32) << 2u32);
pub const SMA1303_TRM_VBST_6P1: u32 = ((6u32) << 2u32);
pub const SMA1303_TRM_VBST_6P2: u32 = ((7u32) << 2u32);

/* TOP_MAN1 : 0xA2 */
pub const SMA1303_PLL_LOCK_SKIP_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_PLL_LOCK_ENABLE: u32 = ((0u32) << 7u32);
pub const SMA1303_PLL_LOCK_DISABLE: u32 = ((1u32) << 7u32);

pub const SMA1303_PLL_PD_MASK: u32 = ((1u32) << 6u32);
pub const SMA1303_PLL_OPERATION: u32 = ((0u32) << 6u32);
pub const SMA1303_PLL_PD: u32 = ((1u32) << 6u32);

pub const SMA1303_PLL_DIV_MASK: u32 = ((3u32) << 4u32);
pub const SMA1303_PLL_OUT: u32 = ((0u32) << 4u32);
pub const SMA1303_PLL_OUT_2: u32 = ((1u32) << 4u32);
pub const SMA1303_PLL_OUT_4: u32 = ((2u32) << 4u32);
pub const SMA1303_PLL_OUT_8: u32 = ((3u32) << 4u32);

pub const SMA1303_PLL_REF_CLK_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_PLL_REF_CLK1: u32 = ((0u32) << 3u32);
pub const SMA1303_PLL_SCK: u32 = ((1u32) << 3u32);

pub const SMA1303_DAC_DN_CONV_MASK: u32 = ((1u32) << 2u32);
pub const SMA1303_DAC_DN_CONV_DISABLE: u32 = ((0u32) << 2u32);
pub const SMA1303_DAC_DN_CONV_ENABLE: u32 = ((1u32) << 2u32);

pub const SMA1303_SDO_IO_MASK: u32 = ((1u32) << 1u32);
pub const SMA1303_HIGH_Z_LRCK_H: u32 = ((0u32) << 1u32);
pub const SMA1303_HIGH_Z_LRCK_L: u32 = ((1u32) << 1u32);

pub const SMA1303_SDO_OUTPUT2_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_SDO_NORMAL: u32 = ((0u32) << 0u32);
pub const SMA1303_SDO_OUTPUT_ONLY: u32 = ((1u32) << 0u32);

/* TOP_MAN2 : 0xA3 */
pub const SMA1303_MON_OSC_PLL_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_PLL_SDO: u32 = ((0u32) << 7u32);
pub const SMA1303_OSC_SDO: u32 = ((1u32) << 7u32);

pub const SMA1303_TEST_CLKO_EN_MASK: u32 = ((1u32) << 6u32);
pub const SMA1303_NORMAL_SDO: u32 = ((0u32) << 6u32);
pub const SMA1303_CLK_OUT_SDO: u32 = ((1u32) << 6u32);

pub const SMA1303_SDO_OUTPUT_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_NORMAL_OUT: u32 = ((0u32) << 3u32);
pub const SMA1303_HIGH_Z_OUT: u32 = ((1u32) << 3u32);

pub const SMA1303_CLOCK_MON_MASK: u32 = ((1u32) << 1u32);
pub const SMA1303_CLOCK_MON: u32 = ((0u32) << 1u32);
pub const SMA1303_CLOCK_NOT_MON: u32 = ((1u32) << 1u32);

pub const SMA1303_OSC_PD_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_NORMAL_OPERATION_OSC: u32 = ((0u32) << 0u32);
pub const SMA1303_POWER_DOWN_OSC: u32 = ((1u32) << 0u32);

/* TOP_MAN3 0xA4 */
pub const SMA1303_O_FORMAT_MASK: u32 = ((7u32) << 5u32);
pub const SMA1303_O_FMT_LJ: u32 = ((1u32) << 5u32);
pub const SMA1303_O_FMT_I2S: u32 = ((2u32) << 5u32);
pub const SMA1303_O_FMT_TDM: u32 = ((4u32) << 5u32);

pub const SMA1303_SCK_RATE_MASK: u32 = ((1u32) << 3u32);
pub const SMA1303_SCK_64FS: u32 = ((0u32) << 3u32);
pub const SMA1303_SCK_32FS: u32 = ((2u32) << 3u32);

pub const SMA1303_LRCK_POL_MASK: u32 = ((1u32) << 0u32);
pub const SMA1303_L_VALID: u32 = ((0u32) << 0u32);
pub const SMA1303_R_VALID: u32 = ((1u32) << 0u32);

/* TDM1 FORMAT : 0xA5 */
pub const SMA1303_TDM_CLK_POL_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_TDM_CLK_POL_RISE: u32 = ((0u32) << 7u32);
pub const SMA1303_TDM_CLK_POL_FALL: u32 = ((1u32) << 7u32);

pub const SMA1303_TDM_TX_MODE_MASK: u32 = ((1u32) << 6u32);
pub const SMA1303_TDM_TX_MONO: u32 = ((0u32) << 6u32);
pub const SMA1303_TDM_TX_STEREO: u32 = ((1u32) << 6u32);

pub const SMA1303_TDM_SLOT1_RX_POS_MASK: u32 = ((7u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_0: u32 = ((0u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_1: u32 = ((1u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_2: u32 = ((2u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_3: u32 = ((3u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_4: u32 = ((4u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_5: u32 = ((5u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_6: u32 = ((6u32) << 3u32);
pub const SMA1303_TDM_SLOT1_RX_POS_7: u32 = ((7u32) << 3u32);

pub const SMA1303_TDM_SLOT2_RX_POS_MASK: u32 = ((7u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_0: u32 = ((0u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_1: u32 = ((1u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_2: u32 = ((2u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_3: u32 = ((3u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_4: u32 = ((4u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_5: u32 = ((5u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_6: u32 = ((6u32) << 0u32);
pub const SMA1303_TDM_SLOT2_RX_POS_7: u32 = ((7u32) << 0u32);

/* TDM2 FORMAT : 0xA6 */
pub const SMA1303_TDM_DL_MASK: u32 = ((1u32) << 7u32);
pub const SMA1303_TDM_DL_16: u32 = ((0u32) << 7u32);
pub const SMA1303_TDM_DL_32: u32 = ((1u32) << 7u32);

pub const SMA1303_TDM_N_SLOT_MASK: u32 = ((1u32) << 6u32);
pub const SMA1303_TDM_N_SLOT_4: u32 = ((0u32) << 6u32);
pub const SMA1303_TDM_N_SLOT_8: u32 = ((1u32) << 6u32);

pub const SMA1303_TDM_SLOT1_TX_POS_MASK: u32 = ((7u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_0: u32 = ((0u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_1: u32 = ((1u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_2: u32 = ((2u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_3: u32 = ((3u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_4: u32 = ((4u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_5: u32 = ((5u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_6: u32 = ((6u32) << 3u32);
pub const SMA1303_TDM_SLOT1_TX_POS_7: u32 = ((7u32) << 3u32);

pub const SMA1303_TDM_SLOT2_TX_POS_MASK: u32 = ((7u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_0: u32 = ((0u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_1: u32 = ((1u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_2: u32 = ((2u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_3: u32 = ((3u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_4: u32 = ((4u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_5: u32 = ((5u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_6: u32 = ((6u32) << 0u32);
pub const SMA1303_TDM_SLOT2_TX_POS_7: u32 = ((7u32) << 0u32);

/* STATUS1 : 0xFA */
pub const SMA1303_OT1_OK_STATUS: u32 = ((1u32) << 7u32);
pub const SMA1303_OT2_OK_STATUS: u32 = ((1u32) << 6u32);

/* STATUS2 : 0xFB */
pub const SMA1303_OCP_SPK_STATUS: u32 = ((1u32) << 5u32);
pub const SMA1303_OCP_BST_STATUS: u32 = ((1u32) << 4u32);
pub const SMA1303_OTP_STAT_OK_0: u32 = ((5u32) << 1u32);
pub const SMA1303_OTP_STAT_OK_1: u32 = ((2u32) << 2u32);

pub const SMA1303_CLK_MON_STATUS: u32 = ((1u32) << 0u32);

/* DEVICE_INFO : 0xFF */
pub const SMA1303_DEVICE_ID: u32 = ((2u32) << 3u32);
pub const SMA1303_UVLO_BST_STATUS: u32 = ((1u32) << 2u32);
pub const SMA1303_REV_NUM_STATUS: u32 = ((3u32) << 0u32);
pub const SMA1303_REV_NUM_TV0: u32 = ((0u32) << 0u32);
pub const SMA1303_REV_NUM_TV1: u32 = ((1u32) << 0u32);


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
