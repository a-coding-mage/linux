/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI LMU (Lighting Management Unit) Device Register Map
 *
 * Copyright 2017 Texas Instruments
 *
 * Author: Milo Kim <milo.kim@ti.com>
 */

// linux/bitops.h dependency: BIT(n) is represented directly as 1u32 << n.

/* LM3631 */
pub const LM3631_REG_DEVCTRL: u32 = 0x00;
pub const LM3631_LCD_EN_MASK: u32 = 1u32 << 1;
pub const LM3631_BL_EN_MASK: u32 = 1u32 << 0;
pub const LM3631_REG_BRT_LSB: u32 = 0x01;
pub const LM3631_REG_BRT_MSB: u32 = 0x02;
pub const LM3631_REG_BL_CFG: u32 = 0x06;
pub const LM3631_BL_CHANNEL_MASK: u32 = 1u32 << 3;
pub const LM3631_BL_DUAL_CHANNEL: u32 = 0;
pub const LM3631_BL_SINGLE_CHANNEL: u32 = 1u32 << 3;
pub const LM3631_MAP_MASK: u32 = 1u32 << 5;
pub const LM3631_EXPONENTIAL_MAP: u32 = 0;
pub const LM3631_REG_BRT_MODE: u32 = 0x08;
pub const LM3631_MODE_MASK: u32 = (1u32 << 1) | (1u32 << 2) | (1u32 << 3);
pub const LM3631_DEFAULT_MODE: u32 = (1u32 << 1) | (1u32 << 3);
pub const LM3631_REG_SLOPE: u32 = 0x09;
pub const LM3631_SLOPE_MASK: u32 = 0xF0;
pub const LM3631_SLOPE_SHIFT: u32 = 4;
pub const LM3631_REG_LDO_CTRL1: u32 = 0x0A;
pub const LM3631_EN_OREF_MASK: u32 = 1u32 << 0;
pub const LM3631_EN_VNEG_MASK: u32 = 1u32 << 1;
pub const LM3631_EN_VPOS_MASK: u32 = 1u32 << 2;
pub const LM3631_REG_LDO_CTRL2: u32 = 0x0B;
pub const LM3631_EN_CONT_MASK: u32 = 1u32 << 0;
pub const LM3631_REG_VOUT_CONT: u32 = 0x0C;
pub const LM3631_VOUT_CONT_MASK: u32 = (1u32 << 6) | (1u32 << 7);
pub const LM3631_REG_VOUT_BOOST: u32 = 0x0C;
pub const LM3631_REG_VOUT_POS: u32 = 0x0D;
pub const LM3631_REG_VOUT_NEG: u32 = 0x0E;
pub const LM3631_REG_VOUT_OREF: u32 = 0x0F;
pub const LM3631_VOUT_MASK: u32 = 0x3F;
pub const LM3631_REG_ENTIME_VCONT: u32 = 0x0B;
pub const LM3631_ENTIME_CONT_MASK: u32 = 0x70;
pub const LM3631_REG_ENTIME_VOREF: u32 = 0x0F;
pub const LM3631_REG_ENTIME_VPOS: u32 = 0x10;
pub const LM3631_REG_ENTIME_VNEG: u32 = 0x11;
pub const LM3631_ENTIME_MASK: u32 = 0xF0;
pub const LM3631_ENTIME_SHIFT: u32 = 4;
pub const LM3631_MAX_REG: u32 = 0x16;

/* LM3632 */
pub const LM3632_REG_CONFIG1: u32 = 0x02;
pub const LM3632_OVP_MASK: u32 = (1u32 << 5) | (1u32 << 6) | (1u32 << 7);
pub const LM3632_OVP_25V: u32 = 1u32 << 6;
pub const LM3632_REG_CONFIG2: u32 = 0x03;
pub const LM3632_SWFREQ_MASK: u32 = 1u32 << 7;
pub const LM3632_SWFREQ_1MHZ: u32 = 1u32 << 7;
pub const LM3632_REG_BRT_LSB: u32 = 0x04;
pub const LM3632_REG_BRT_MSB: u32 = 0x05;
pub const LM3632_REG_IO_CTRL: u32 = 0x09;
pub const LM3632_PWM_MASK: u32 = 1u32 << 6;
pub const LM3632_I2C_MODE: u32 = 0;
pub const LM3632_PWM_MODE: u32 = 1u32 << 6;
pub const LM3632_REG_ENABLE: u32 = 0x0A;
pub const LM3632_BL_EN_MASK: u32 = 1u32 << 0;
pub const LM3632_BL_CHANNEL_MASK: u32 = (1u32 << 3) | (1u32 << 4);
pub const LM3632_BL_SINGLE_CHANNEL: u32 = 1u32 << 4;
pub const LM3632_BL_DUAL_CHANNEL: u32 = 1u32 << 3;
pub const LM3632_REG_BIAS_CONFIG: u32 = 0x0C;
pub const LM3632_EXT_EN_MASK: u32 = 1u32 << 0;
pub const LM3632_EN_VNEG_MASK: u32 = 1u32 << 1;
pub const LM3632_EN_VPOS_MASK: u32 = 1u32 << 2;
pub const LM3632_REG_VOUT_BOOST: u32 = 0x0D;
pub const LM3632_REG_VOUT_POS: u32 = 0x0E;
pub const LM3632_REG_VOUT_NEG: u32 = 0x0F;
pub const LM3632_VOUT_MASK: u32 = 0x3F;
pub const LM3632_MAX_REG: u32 = 0x10;

/* LM3633 */
pub const LM3633_REG_HVLED_OUTPUT_CFG: u32 = 0x10;
pub const LM3633_HVLED1_CFG_MASK: u32 = 1u32 << 0;
pub const LM3633_HVLED2_CFG_MASK: u32 = 1u32 << 1;
pub const LM3633_HVLED3_CFG_MASK: u32 = 1u32 << 2;
pub const LM3633_HVLED1_CFG_SHIFT: u32 = 0;
pub const LM3633_HVLED2_CFG_SHIFT: u32 = 1;
pub const LM3633_HVLED3_CFG_SHIFT: u32 = 2;
pub const LM3633_REG_BANK_SEL: u32 = 0x11;
pub const LM3633_REG_BL0_RAMP: u32 = 0x12;
pub const LM3633_REG_BL1_RAMP: u32 = 0x13;
pub const LM3633_BL_RAMPUP_MASK: u32 = 0xF0;
pub const LM3633_BL_RAMPUP_SHIFT: u32 = 4;
pub const LM3633_BL_RAMPDN_MASK: u32 = 0x0F;
pub const LM3633_BL_RAMPDN_SHIFT: u32 = 0;
pub const LM3633_REG_BL_RAMP_CONF: u32 = 0x1B;
pub const LM3633_BL_RAMP_MASK: u32 = 0x0F;
pub const LM3633_BL_RAMP_EACH: u32 = 0x05;
pub const LM3633_REG_PTN0_RAMP: u32 = 0x1C;
pub const LM3633_REG_PTN1_RAMP: u32 = 0x1D;
pub const LM3633_PTN_RAMPUP_MASK: u32 = 0x70;
pub const LM3633_PTN_RAMPUP_SHIFT: u32 = 4;
pub const LM3633_PTN_RAMPDN_MASK: u32 = 0x07;
pub const LM3633_PTN_RAMPDN_SHIFT: u32 = 0;
pub const LM3633_REG_LED_MAPPING_MODE: u32 = 0x1F;
pub const LM3633_LED_EXPONENTIAL: u32 = 1u32 << 1;
pub const LM3633_REG_IMAX_HVLED_A: u32 = 0x20;
pub const LM3633_REG_IMAX_HVLED_B: u32 = 0x21;
pub const LM3633_REG_IMAX_LVLED_BASE: u32 = 0x22;
pub const LM3633_REG_BL_FEEDBACK_ENABLE: u32 = 0x28;
pub const LM3633_REG_ENABLE: u32 = 0x2B;
pub const LM3633_LED_BANK_OFFSET: u32 = 2;
pub const LM3633_REG_PATTERN: u32 = 0x2C;
pub const LM3633_REG_BOOST_CFG: u32 = 0x2D;
pub const LM3633_OVP_MASK: u32 = (1u32 << 1) | (1u32 << 2);
pub const LM3633_OVP_40V: u32 = 0x6;
pub const LM3633_REG_PWM_CFG: u32 = 0x2F;
pub const LM3633_PWM_A_MASK: u32 = 1u32 << 0;
pub const LM3633_PWM_B_MASK: u32 = 1u32 << 1;
pub const LM3633_REG_BRT_HVLED_A_LSB: u32 = 0x40;
pub const LM3633_REG_BRT_HVLED_A_MSB: u32 = 0x41;
pub const LM3633_REG_BRT_HVLED_B_LSB: u32 = 0x42;
pub const LM3633_REG_BRT_HVLED_B_MSB: u32 = 0x43;
pub const LM3633_REG_BRT_LVLED_BASE: u32 = 0x44;
pub const LM3633_REG_PTN_DELAY: u32 = 0x50;
pub const LM3633_REG_PTN_LOWTIME: u32 = 0x51;
pub const LM3633_REG_PTN_HIGHTIME: u32 = 0x52;
pub const LM3633_REG_PTN_LOWBRT: u32 = 0x53;
pub const LM3633_REG_PTN_HIGHBRT: u32 = LM3633_REG_BRT_LVLED_BASE;
pub const LM3633_REG_BL_OPEN_FAULT_STATUS: u32 = 0xB0;
pub const LM3633_REG_BL_SHORT_FAULT_STATUS: u32 = 0xB2;
pub const LM3633_REG_MONITOR_ENABLE: u32 = 0xB4;
pub const LM3633_MAX_REG: u32 = 0xB4;

/* LM3695 */
pub const LM3695_REG_GP: u32 = 0x10;
pub const LM3695_BL_CHANNEL_MASK: u32 = 1u32 << 3;
pub const LM3695_BL_DUAL_CHANNEL: u32 = 0;
pub const LM3695_BL_SINGLE_CHANNEL: u32 = 1u32 << 3;
pub const LM3695_BRT_RW_MASK: u32 = 1u32 << 2;
pub const LM3695_BL_EN_MASK: u32 = 1u32 << 0;
pub const LM3695_REG_BRT_LSB: u32 = 0x13;
pub const LM3695_REG_BRT_MSB: u32 = 0x14;
pub const LM3695_MAX_REG: u32 = 0x14;

/* LM36274 */
pub const LM36274_REG_REV: u32 = 0x01;
pub const LM36274_REG_BL_CFG_1: u32 = 0x02;
pub const LM36274_REG_BL_CFG_2: u32 = 0x03;
pub const LM36274_REG_BRT_LSB: u32 = 0x04;
pub const LM36274_REG_BRT_MSB: u32 = 0x05;
pub const LM36274_REG_BL_EN: u32 = 0x08;
pub const LM36274_REG_BIAS_CONFIG_1: u32 = 0x09;
pub const LM36274_EXT_EN_MASK: u32 = 1u32 << 0;
pub const LM36274_EN_VNEG_MASK: u32 = 1u32 << 1;
pub const LM36274_EN_VPOS_MASK: u32 = 1u32 << 2;
pub const LM36274_REG_BIAS_CONFIG_2: u32 = 0x0a;
pub const LM36274_REG_BIAS_CONFIG_3: u32 = 0x0b;
pub const LM36274_REG_VOUT_BOOST: u32 = 0x0c;
pub const LM36274_REG_VOUT_POS: u32 = 0x0d;
pub const LM36274_REG_VOUT_NEG: u32 = 0x0e;
pub const LM36274_VOUT_MASK: u32 = 0x3F;
pub const LM36274_MAX_REG: u32 = 0x13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
