/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tas5720.h - ALSA SoC Texas Instruments TAS5720 Mono Audio Amplifier
 *
 * Copyright (C)2015-2016 Texas Instruments Incorporated -  https://www.ti.com
 *
 * Author: Andreas Dannenberg <dannenberg@ti.com>
 */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* Register Address Map - first 3 regs are common for all variants */
pub const TAS5720_DEVICE_ID_REG: u32 = 0x00;
pub const TAS5720_POWER_CTRL_REG: u32 = 0x01;
pub const TAS5720_DIGITAL_CTRL1_REG: u32 = 0x02;
pub const TAS5720_DIGITAL_CTRL2_REG: u32 = 0x03;
pub const TAS5720_VOLUME_CTRL_REG: u32 = 0x04;
pub const TAS5720_ANALOG_CTRL_REG: u32 = 0x06;
pub const TAS5720_FAULT_REG: u32 = 0x08;
pub const TAS5720_DIGITAL_CLIP2_REG: u32 = 0x10;
pub const TAS5720_DIGITAL_CLIP1_REG: u32 = 0x11;
pub const TAS5720_MAX_REG: u32 = TAS5720_DIGITAL_CLIP1_REG;

/* Additional TAS5722-specific Registers */
pub const TAS5722_DIGITAL_CTRL2_REG: u32 = 0x13;
pub const TAS5722_ANALOG_CTRL2_REG: u32 = 0x14;
pub const TAS5722_MAX_REG: u32 = TAS5722_ANALOG_CTRL2_REG;

/* Register Address Map - volume controls for the TAS5720-Q1 variant */
pub const TAS5720_Q1_VOLUME_CTRL_CFG_REG: u32 = 0x03;
pub const TAS5720_Q1_VOLUME_CTRL_LEFT_REG: u32 = 0x04;
pub const TAS5720_Q1_VOLUME_CTRL_RIGHT_REG: u32 = 0x05;

/* TAS5720_DEVICE_ID_REG */
pub const TAS5720A_Q1_DEVICE_ID: u32 = 0x00;
pub const TAS5720_DEVICE_ID: u32 = 0x01;
pub const TAS5722_DEVICE_ID: u32 = 0x12;

/* TAS5720_POWER_CTRL_REG */
pub const TAS5720_DIG_CLIP_MASK: u32 = GENMASK(7, 2);
pub const TAS5720_SLEEP: u32 = BIT(1);
pub const TAS5720_SDZ: u32 = BIT(0);

/* TAS5720_DIGITAL_CTRL1_REG */
pub const TAS5720_HPF_BYPASS: u32 = BIT(7);
pub const TAS5720_TDM_CFG_SRC: u32 = BIT(6);
pub const TAS5720_SSZ_DS: u32 = BIT(3);
pub const TAS5720_SAIF_RIGHTJ_24BIT: u32 = 0x0;
pub const TAS5720_SAIF_RIGHTJ_20BIT: u32 = 0x1;
pub const TAS5720_SAIF_RIGHTJ_18BIT: u32 = 0x2;
pub const TAS5720_SAIF_RIGHTJ_16BIT: u32 = 0x3;
pub const TAS5720_SAIF_I2S: u32 = 0x4;
pub const TAS5720_SAIF_LEFTJ: u32 = 0x5;
pub const TAS5720_SAIF_FORMAT_MASK: u32 = GENMASK(2, 0);

/* TAS5720_DIGITAL_CTRL2_REG */
pub const TAS5722_VOL_RAMP_RATE: u32 = BIT(6);
pub const TAS5720_MUTE: u32 = BIT(4);
pub const TAS5720_TDM_SLOT_SEL_MASK: u32 = GENMASK(2, 0);

/* TAS5720_Q1_VOLUME_CTRL_CFG_REG */
pub const TAS5720_Q1_FADE: u32 = BIT(7);
pub const TAS5720_Q1_MUTE: u32 = GENMASK(1, 0);

/* TAS5720_ANALOG_CTRL_REG */
pub const TAS5720_PWM_RATE_6_3_FSYNC: u32 = 0x0 << 4;
pub const TAS5720_PWM_RATE_8_4_FSYNC: u32 = 0x1 << 4;
pub const TAS5720_PWM_RATE_10_5_FSYNC: u32 = 0x2 << 4;
pub const TAS5720_PWM_RATE_12_6_FSYNC: u32 = 0x3 << 4;
pub const TAS5720_PWM_RATE_14_7_FSYNC: u32 = 0x4 << 4;
pub const TAS5720_PWM_RATE_16_8_FSYNC: u32 = 0x5 << 4;
pub const TAS5720_PWM_RATE_20_10_FSYNC: u32 = 0x6 << 4;
pub const TAS5720_PWM_RATE_24_12_FSYNC: u32 = 0x7 << 4;
pub const TAS5720_PWM_RATE_MASK: u32 = GENMASK(6, 4);
pub const TAS5720_ANALOG_GAIN_19_2DBV: u32 = 0x0 << 2;
pub const TAS5720_ANALOG_GAIN_20_7DBV: u32 = 0x1 << 2;
pub const TAS5720_ANALOG_GAIN_23_5DBV: u32 = 0x2 << 2;
pub const TAS5720_ANALOG_GAIN_26_3DBV: u32 = 0x3 << 2;
pub const TAS5720_ANALOG_GAIN_MASK: u32 = GENMASK(3, 2);
pub const TAS5720_ANALOG_GAIN_SHIFT: u32 = 0x2;

/* TAS5720_Q1_ANALOG_CTRL_REG */
pub const TAS5720_Q1_RESERVED7_BIT: u32 = BIT(7);
pub const TAS5720_Q1_CHAN_SEL: u32 = BIT(1);

/* TAS5720_FAULT_REG */
pub const TAS5720_OC_THRESH_100PCT: u32 = 0x0 << 4;
pub const TAS5720_OC_THRESH_75PCT: u32 = 0x1 << 4;
pub const TAS5720_OC_THRESH_50PCT: u32 = 0x2 << 4;
pub const TAS5720_OC_THRESH_25PCT: u32 = 0x3 << 4;
pub const TAS5720_OC_THRESH_MASK: u32 = GENMASK(5, 4);
pub const TAS5720_CLKE: u32 = BIT(3);
pub const TAS5720_OCE: u32 = BIT(2);
pub const TAS5720_DCE: u32 = BIT(1);
pub const TAS5720_OTE: u32 = BIT(0);
pub const TAS5720_FAULT_MASK: u32 = GENMASK(3, 0);

/* TAS5720_DIGITAL_CLIP1_REG */
pub const TAS5720_CLIP1_MASK: u32 = GENMASK(7, 2);
pub const TAS5720_CLIP1_SHIFT: u32 = 0x2;

/* TAS5722_DIGITAL_CTRL2_REG */
pub const TAS5722_HPF_3_7HZ: u32 = 0x0 << 5;
pub const TAS5722_HPF_7_4HZ: u32 = 0x1 << 5;
pub const TAS5722_HPF_14_9HZ: u32 = 0x2 << 5;
pub const TAS5722_HPF_29_7HZ: u32 = 0x3 << 5;
pub const TAS5722_HPF_59_4HZ: u32 = 0x4 << 5;
pub const TAS5722_HPF_118_4HZ: u32 = 0x5 << 5;
pub const TAS5722_HPF_235_0HZ: u32 = 0x6 << 5;
pub const TAS5722_HPF_463_2HZ: u32 = 0x7 << 5;
pub const TAS5722_HPF_MASK: u32 = GENMASK(7, 5);
pub const TAS5722_AUTO_SLEEP_OFF: u32 = 0x0 << 3;
pub const TAS5722_AUTO_SLEEP_1024LR: u32 = 0x1 << 3;
pub const TAS5722_AUTO_SLEEP_65536LR: u32 = 0x2 << 3;
pub const TAS5722_AUTO_SLEEP_262144LR: u32 = 0x3 << 3;
pub const TAS5722_AUTO_SLEEP_MASK: u32 = GENMASK(4, 3);
pub const TAS5722_TDM_SLOT_16B: u32 = BIT(2);
pub const TAS5722_MCLK_PIN_CFG: u32 = BIT(1);
pub const TAS5722_VOL_CONTROL_LSB: u32 = BIT(0);

/* TAS5722_ANALOG_CTRL2_REG */
pub const TAS5722_FAULTZ_PU: u32 = BIT(3);
pub const TAS5722_VREG_LVL: u32 = BIT(2);
pub const TAS5722_PWR_TUNE: u32 = BIT(0);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
