// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * TAS571x amplifier audio driver
 *
 * Copyright (C) 2015 Google, Inc.
 */

/* device registers */
pub const TAS571X_CLK_CTRL_REG: u32 = 0x00;
pub const TAS571X_DEV_ID_REG: u32 = 0x01;
pub const TAS571X_ERR_STATUS_REG: u32 = 0x02;
pub const TAS571X_SYS_CTRL_1_REG: u32 = 0x03;
pub const TAS571X_SDI_REG: u32 = 0x04;
pub const TAS571X_SDI_FMT_MASK: u32 = 0x0f;

pub const TAS571X_SYS_CTRL_2_REG: u32 = 0x05;
pub const TAS571X_SYS_CTRL_2_SDN_MASK: u32 = 0x40;

pub const TAS571X_SOFT_MUTE_REG: u32 = 0x06;
pub const TAS571X_SOFT_MUTE_CH1_SHIFT: u32 = 0;
pub const TAS571X_SOFT_MUTE_CH2_SHIFT: u32 = 1;
pub const TAS571X_SOFT_MUTE_CH3_SHIFT: u32 = 2;

pub const TAS571X_MVOL_REG: u32 = 0x07;
pub const TAS571X_CH1_VOL_REG: u32 = 0x08;
pub const TAS571X_CH2_VOL_REG: u32 = 0x09;
pub const TAS571X_CH3_VOL_REG: u32 = 0x0a;
pub const TAS571X_VOL_CFG_REG: u32 = 0x0e;
pub const TAS571X_MODULATION_LIMIT_REG: u32 = 0x10;
pub const TAS571X_IC_DELAY_CH1_REG: u32 = 0x11;
pub const TAS571X_IC_DELAY_CH2_REG: u32 = 0x12;
pub const TAS571X_IC_DELAY_CH3_REG: u32 = 0x13;
pub const TAS571X_IC_DELAY_CH4_REG: u32 = 0x14;

pub const TAS571X_PWM_CH_SDN_GROUP_REG: u32 = 0x19; /* N/A on TAS5717, TAS5719 */
pub const TAS571X_PWM_CH1_SDN_MASK: u32 = 1 << 0;
pub const TAS571X_PWM_CH2_SDN_SHIFT: u32 = 1 << 1;
pub const TAS571X_PWM_CH3_SDN_SHIFT: u32 = 1 << 2;
pub const TAS571X_PWM_CH4_SDN_SHIFT: u32 = 1 << 3;

pub const TAS571X_START_STOP_PERIOD_REG: u32 = 0x1a;
pub const TAS571X_OSC_TRIM_REG: u32 = 0x1b;
pub const TAS571X_BKND_ERR_REG: u32 = 0x1c;
pub const TAS571X_INPUT_MUX_REG: u32 = 0x20;
pub const TAS571X_CH4_SRC_SELECT_REG: u32 = 0x21;
pub const TAS571X_PWM_MUX_REG: u32 = 0x25;

/* 20-byte biquad registers */
pub const TAS5707_CH1_BQ0_REG: u32 = 0x29;
pub const TAS5707_CH1_BQ1_REG: u32 = 0x2a;
pub const TAS5707_CH1_BQ2_REG: u32 = 0x2b;
pub const TAS5707_CH1_BQ3_REG: u32 = 0x2c;
pub const TAS5707_CH1_BQ4_REG: u32 = 0x2d;
pub const TAS5707_CH1_BQ5_REG: u32 = 0x2e;
pub const TAS5707_CH1_BQ6_REG: u32 = 0x2f;

pub const TAS5707_CH2_BQ0_REG: u32 = 0x30;
pub const TAS5707_CH2_BQ1_REG: u32 = 0x31;
pub const TAS5707_CH2_BQ2_REG: u32 = 0x32;
pub const TAS5707_CH2_BQ3_REG: u32 = 0x33;
pub const TAS5707_CH2_BQ4_REG: u32 = 0x34;
pub const TAS5707_CH2_BQ5_REG: u32 = 0x35;
pub const TAS5707_CH2_BQ6_REG: u32 = 0x36;

pub const TAS5717_CH1_BQ0_REG: u32 = 0x26;
pub const TAS5717_CH1_BQ1_REG: u32 = 0x27;
pub const TAS5717_CH1_BQ2_REG: u32 = 0x28;
pub const TAS5717_CH1_BQ3_REG: u32 = 0x29;
pub const TAS5717_CH1_BQ4_REG: u32 = 0x2a;
pub const TAS5717_CH1_BQ5_REG: u32 = 0x2b;
pub const TAS5717_CH1_BQ6_REG: u32 = 0x2c;
pub const TAS5717_CH1_BQ7_REG: u32 = 0x2d;
pub const TAS5717_CH1_BQ8_REG: u32 = 0x2e;
pub const TAS5717_CH1_BQ9_REG: u32 = 0x2f;

pub const TAS5717_CH2_BQ0_REG: u32 = 0x30;
pub const TAS5717_CH2_BQ1_REG: u32 = 0x31;
pub const TAS5717_CH2_BQ2_REG: u32 = 0x32;
pub const TAS5717_CH2_BQ3_REG: u32 = 0x33;
pub const TAS5717_CH2_BQ4_REG: u32 = 0x34;
pub const TAS5717_CH2_BQ5_REG: u32 = 0x35;
pub const TAS5717_CH2_BQ6_REG: u32 = 0x36;
pub const TAS5717_CH2_BQ7_REG: u32 = 0x37;
pub const TAS5717_CH2_BQ8_REG: u32 = 0x38;
pub const TAS5717_CH2_BQ9_REG: u32 = 0x39;

pub const TAS5717_CH1_BQ10_REG: u32 = 0x58;
pub const TAS5717_CH1_BQ11_REG: u32 = 0x59;

pub const TAS5717_CH4_BQ0_REG: u32 = 0x5a;
pub const TAS5717_CH4_BQ1_REG: u32 = 0x5b;

pub const TAS5717_CH2_BQ10_REG: u32 = 0x5c;
pub const TAS5717_CH2_BQ11_REG: u32 = 0x5d;

pub const TAS5717_CH3_BQ0_REG: u32 = 0x5e;
pub const TAS5717_CH3_BQ1_REG: u32 = 0x5f;

pub const TAS5717_CH1_RIGHT_CH_MIX_REG: u32 = 0x72;
pub const TAS5717_CH1_LEFT_CH_MIX_REG: u32 = 0x73;
pub const TAS5717_CH2_LEFT_CH_MIX_REG: u32 = 0x76;
pub const TAS5717_CH2_RIGHT_CH_MIX_REG: u32 = 0x77;

pub const TAS5733_CH1_BQ0_REG: u32 = 0x26;
pub const TAS5733_CH1_BQ1_REG: u32 = 0x27;
pub const TAS5733_CH1_BQ2_REG: u32 = 0x28;
pub const TAS5733_CH1_BQ3_REG: u32 = 0x29;
pub const TAS5733_CH1_BQ4_REG: u32 = 0x2a;
pub const TAS5733_CH1_BQ5_REG: u32 = 0x2b;
pub const TAS5733_CH1_BQ6_REG: u32 = 0x2c;
pub const TAS5733_CH1_BQ7_REG: u32 = 0x2d;
pub const TAS5733_CH1_BQ8_REG: u32 = 0x2e;
pub const TAS5733_CH1_BQ9_REG: u32 = 0x2f;

pub const TAS5733_CH2_BQ0_REG: u32 = 0x30;
pub const TAS5733_CH2_BQ1_REG: u32 = 0x31;
pub const TAS5733_CH2_BQ2_REG: u32 = 0x32;
pub const TAS5733_CH2_BQ3_REG: u32 = 0x33;
pub const TAS5733_CH2_BQ4_REG: u32 = 0x34;
pub const TAS5733_CH2_BQ5_REG: u32 = 0x35;
pub const TAS5733_CH2_BQ6_REG: u32 = 0x36;
pub const TAS5733_CH2_BQ7_REG: u32 = 0x37;
pub const TAS5733_CH2_BQ8_REG: u32 = 0x38;
pub const TAS5733_CH2_BQ9_REG: u32 = 0x39;

pub const TAS5733_CH1_BQ10_REG: u32 = 0x58;
pub const TAS5733_CH1_CBQ0_REG: u32 = 0x59;
pub const TAS5733_CH1_CBQ1_REG: u32 = 0x5a;
pub const TAS5733_CH1_CBQ2_REG: u32 = 0x5b;
pub const TAS5733_CH1_CBQ3_REG: u32 = 0x5c;

pub const TAS5733_CH2_BQ10_REG: u32 = 0x5d;
pub const TAS5733_CH2_CBQ0_REG: u32 = 0x5e;
pub const TAS5733_CH2_CBQ1_REG: u32 = 0x5f;
pub const TAS5733_CH2_CBQ2_REG: u32 = 0x60;
pub const TAS5733_CH2_CBQ3_REG: u32 = 0x61;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
