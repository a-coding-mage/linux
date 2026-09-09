/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2020 NXP. */

// Dependency in the original header: <linux/regmap.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pca9450_chip_type {
    PCA9450_TYPE_PCA9450A = 0,
    PCA9450_TYPE_PCA9450BC,
    PCA9450_TYPE_PCA9451A,
    PCA9450_TYPE_PCA9452,
    PCA9450_TYPE_AMOUNT,
}

pub const PCA9450_BUCK1: i32 = 0;
pub const PCA9450_BUCK2: i32 = 1;
pub const PCA9450_BUCK3: i32 = 2;
pub const PCA9450_BUCK4: i32 = 3;
pub const PCA9450_BUCK5: i32 = 4;
pub const PCA9450_BUCK6: i32 = 5;
pub const PCA9450_LDO1: i32 = 6;
pub const PCA9450_LDO2: i32 = 7;
pub const PCA9450_LDO3: i32 = 8;
pub const PCA9450_LDO4: i32 = 9;
pub const PCA9450_LDO5: i32 = 10;
pub const PCA9450_REGULATOR_CNT: i32 = 11;

pub const PCA9450_DVS_LEVEL_RUN: i32 = 0;
pub const PCA9450_DVS_LEVEL_STANDBY: i32 = 1;
pub const PCA9450_DVS_LEVEL_MAX: i32 = 2;

pub const PCA9450_RESTART_HANDLER_PRIORITY: i32 = 130;

pub const PCA9450_BUCK1_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_BUCK2_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_BUCK3_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_BUCK4_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_BUCK5_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_BUCK6_VOLTAGE_NUM: i32 = 0x80;
pub const PCA9450_LDO1_VOLTAGE_NUM: i32 = 0x08;
pub const PCA9450_LDO2_VOLTAGE_NUM: i32 = 0x08;
pub const PCA9450_LDO3_VOLTAGE_NUM: i32 = 0x20;
pub const PCA9450_LDO4_VOLTAGE_NUM: i32 = 0x20;
pub const PCA9450_LDO5_VOLTAGE_NUM: i32 = 0x10;

pub const PCA9450_REG_DEV_ID: i32 = 0x00;
pub const PCA9450_REG_INT1: i32 = 0x01;
pub const PCA9450_REG_INT1_MSK: i32 = 0x02;
pub const PCA9450_REG_STATUS1: i32 = 0x03;
pub const PCA9450_REG_STATUS2: i32 = 0x04;
pub const PCA9450_REG_PWRON_STAT: i32 = 0x05;
pub const PCA9450_REG_SWRST: i32 = 0x06;
pub const PCA9450_REG_PWRCTRL: i32 = 0x07;
pub const PCA9450_REG_RESET_CTRL: i32 = 0x08;
pub const PCA9450_REG_CONFIG1: i32 = 0x09;
pub const PCA9450_REG_CONFIG2: i32 = 0x0A;
pub const PCA9450_REG_BUCK123_DVS: i32 = 0x0C;
pub const PCA9450_REG_BUCK1OUT_LIMIT: i32 = 0x0D;
pub const PCA9450_REG_BUCK2OUT_LIMIT: i32 = 0x0E;
pub const PCA9450_REG_BUCK3OUT_LIMIT: i32 = 0x0F;
pub const PCA9450_REG_BUCK1CTRL: i32 = 0x10;
pub const PCA9450_REG_BUCK1OUT_DVS0: i32 = 0x11;
pub const PCA9450_REG_BUCK1OUT_DVS1: i32 = 0x12;
pub const PCA9450_REG_BUCK2CTRL: i32 = 0x13;
pub const PCA9450_REG_BUCK2OUT_DVS0: i32 = 0x14;
pub const PCA9450_REG_BUCK2OUT_DVS1: i32 = 0x15;
pub const PCA9450_REG_BUCK3CTRL: i32 = 0x16;
pub const PCA9450_REG_BUCK3OUT_DVS0: i32 = 0x17;
pub const PCA9450_REG_BUCK3OUT_DVS1: i32 = 0x18;
pub const PCA9450_REG_BUCK4CTRL: i32 = 0x19;
pub const PCA9450_REG_BUCK4OUT: i32 = 0x1A;
pub const PCA9450_REG_BUCK5CTRL: i32 = 0x1B;
pub const PCA9450_REG_BUCK5OUT: i32 = 0x1C;
pub const PCA9450_REG_BUCK6CTRL: i32 = 0x1D;
pub const PCA9450_REG_BUCK6OUT: i32 = 0x1E;
pub const PCA9450_REG_LDO_AD_CTRL: i32 = 0x20;
pub const PCA9450_REG_LDO1CTRL: i32 = 0x21;
pub const PCA9450_REG_LDO2CTRL: i32 = 0x22;
pub const PCA9450_REG_LDO3CTRL: i32 = 0x23;
pub const PCA9450_REG_LDO4CTRL: i32 = 0x24;
pub const PCA9450_REG_LDO5CTRL_L: i32 = 0x25;
pub const PCA9450_REG_LDO5CTRL_H: i32 = 0x26;
pub const PCA9450_REG_LOADSW_CTRL: i32 = 0x2A;
pub const PCA9450_REG_VRFLT1_STS: i32 = 0x2B;
pub const PCA9450_REG_VRFLT2_STS: i32 = 0x2C;
pub const PCA9450_REG_VRFLT1_MASK: i32 = 0x2D;
pub const PCA9450_REG_VRFLT2_MASK: i32 = 0x2E;
pub const PCA9450_MAX_REGISTER: i32 = 0x2F;

/* PCA9450 BUCK ENMODE bits */
pub const BUCK_ENMODE_OFF: i32 = 0x00;
pub const BUCK_ENMODE_ONREQ: i32 = 0x01;
pub const BUCK_ENMODE_ONREQ_STBYREQ: i32 = 0x02;
pub const BUCK_ENMODE_ON: i32 = 0x03;

/* PCA9450_REG_BUCK[1-3]_CTRL bits */
pub const BUCK1_RAMP_MASK: i32 = 0xC0;
pub const BUCK1_RAMP_25MV: i32 = 0x0;
pub const BUCK1_RAMP_12P5MV: i32 = 0x1;
pub const BUCK1_RAMP_6P25MV: i32 = 0x2;
pub const BUCK1_RAMP_3P125MV: i32 = 0x3;
pub const BUCK1_DVS_CTRL: i32 = 0x10;
pub const BUCK1_AD: i32 = 0x08;
pub const BUCK1_FPWM: i32 = 0x04;
pub const BUCK1_ENMODE_MASK: i32 = 0x03;
pub const BUCK2_RAMP_MASK: i32 = 0xC0;
pub const BUCK2_RAMP_25MV: i32 = 0x0;
pub const BUCK2_RAMP_12P5MV: i32 = 0x1;
pub const BUCK2_RAMP_6P25MV: i32 = 0x2;
pub const BUCK2_RAMP_3P125MV: i32 = 0x3;
pub const BUCK2_DVS_CTRL: i32 = 0x10;
pub const BUCK2_AD: i32 = 0x08;
pub const BUCK2_FPWM: i32 = 0x04;
pub const BUCK2_ENMODE_MASK: i32 = 0x03;
pub const BUCK3_RAMP_MASK: i32 = 0xC0;
pub const BUCK3_RAMP_25MV: i32 = 0x0;
pub const BUCK3_RAMP_12P5MV: i32 = 0x1;
pub const BUCK3_RAMP_6P25MV: i32 = 0x2;
pub const BUCK3_RAMP_3P125MV: i32 = 0x3;
pub const BUCK3_DVS_CTRL: i32 = 0x10;
pub const BUCK3_AD: i32 = 0x08;
pub const BUCK3_FPWM: i32 = 0x04;
pub const BUCK3_ENMODE_MASK: i32 = 0x03;

pub const BUCK4_AD: i32 = 0x08;
pub const BUCK4_FPWM: i32 = 0x04;
pub const BUCK4_ENMODE_MASK: i32 = 0x03;
pub const BUCK5_AD: i32 = 0x08;
pub const BUCK5_FPWM: i32 = 0x04;
pub const BUCK5_ENMODE_MASK: i32 = 0x03;
pub const BUCK6_AD: i32 = 0x08;
pub const BUCK6_FPWM: i32 = 0x04;
pub const BUCK6_ENMODE_MASK: i32 = 0x03;
pub const BUCK123_PRESET_EN: i32 = 0x80;

pub const BUCK1OUT_DVS0_MASK: i32 = 0x7F;
pub const BUCK1OUT_DVS0_DEFAULT: i32 = 0x14;
pub const BUCK1OUT_DVS1_MASK: i32 = 0x7F;
pub const BUCK1OUT_DVS1_DEFAULT: i32 = 0x14;
pub const BUCK2OUT_DVS0_MASK: i32 = 0x7F;
pub const BUCK2OUT_DVS0_DEFAULT: i32 = 0x14;
pub const BUCK2OUT_DVS1_MASK: i32 = 0x7F;
pub const BUCK2OUT_DVS1_DEFAULT: i32 = 0x14;
pub const BUCK3OUT_DVS0_MASK: i32 = 0x7F;
pub const BUCK3OUT_DVS0_DEFAULT: i32 = 0x14;
pub const BUCK3OUT_DVS1_MASK: i32 = 0x7F;
pub const BUCK3OUT_DVS1_DEFAULT: i32 = 0x14;
pub const BUCK4OUT_MASK: i32 = 0x7F;
pub const BUCK4OUT_DEFAULT: i32 = 0x6C;
pub const BUCK5OUT_MASK: i32 = 0x7F;
pub const BUCK5OUT_DEFAULT: i32 = 0x30;
pub const BUCK6OUT_MASK: i32 = 0x7F;
pub const BUCK6OUT_DEFAULT: i32 = 0x14;

pub const LDO1_EN_MASK: i32 = 0xC0;
pub const LDO1OUT_MASK: i32 = 0x07;
pub const LDO2_EN_MASK: i32 = 0xC0;
pub const LDO2OUT_MASK: i32 = 0x07;
pub const LDO3_EN_MASK: i32 = 0xC0;
pub const LDO3OUT_MASK: i32 = 0x1F;
pub const LDO4_EN_MASK: i32 = 0xC0;
pub const LDO4OUT_MASK: i32 = 0x1F;
pub const LDO5L_EN_MASK: i32 = 0xC0;
pub const LDO5LOUT_MASK: i32 = 0x0F;
pub const LDO5H_EN_MASK: i32 = 0xC0;
pub const LDO5HOUT_MASK: i32 = 0x0F;

pub const IRQ_PWRON: i32 = 0x80;
pub const IRQ_WDOGB: i32 = 0x40;
pub const IRQ_RSVD: i32 = 0x20;
pub const IRQ_VR_FLT1: i32 = 0x10;
pub const IRQ_VR_FLT2: i32 = 0x08;
pub const IRQ_LOWVSYS: i32 = 0x04;
pub const IRQ_THERM_105: i32 = 0x02;
pub const IRQ_THERM_125: i32 = 0x01;

pub const T_ON_DEB_MASK: i32 = 0xC0;
pub const T_ON_DEB_120US: i32 = 0 << 6;
pub const T_ON_DEB_20MS: i32 = 1 << 6;
pub const T_ON_DEB_100MS: i32 = 2 << 6;
pub const T_ON_DEB_750MS: i32 = 3 << 6;
pub const T_OFF_DEB_MASK: i32 = 0x20;
pub const T_OFF_DEB_120US: i32 = 0 << 5;
pub const T_OFF_DEB_2MS: i32 = 1 << 5;
pub const T_ON_STEP_MASK: i32 = 0x18;
pub const T_ON_STEP_1MS: i32 = 0 << 3;
pub const T_ON_STEP_2MS: i32 = 1 << 3;
pub const T_ON_STEP_4MS: i32 = 2 << 3;
pub const T_ON_STEP_8MS: i32 = 3 << 3;
pub const T_OFF_STEP_MASK: i32 = 0x06;
pub const T_OFF_STEP_2MS: i32 = 0 << 1;
pub const T_OFF_STEP_4MS: i32 = 1 << 1;
pub const T_OFF_STEP_8MS: i32 = 2 << 1;
pub const T_OFF_STEP_16MS: i32 = 3 << 1;
pub const T_RESTART_MASK: i32 = 0x01;
pub const T_RESTART_250MS: i32 = 0;
pub const T_RESTART_500MS: i32 = 1;

pub const WDOG_B_CFG_MASK: i32 = 0xC0;
pub const WDOG_B_CFG_NONE: i32 = 0x00;
pub const WDOG_B_CFG_WARM: i32 = 0x40;
pub const WDOG_B_CFG_COLD_LDO12: i32 = 0x80;
pub const WDOG_B_CFG_COLD: i32 = 0xC0;
pub const T_PMIC_RST_DEB_MASK: i32 = 0x07;
pub const T_PMIC_RST_DEB_10MS: i32 = 0x00;
pub const T_PMIC_RST_DEB_50MS: i32 = 0x01;
pub const T_PMIC_RST_DEB_100MS: i32 = 0x02;
pub const T_PMIC_RST_DEB_500MS: i32 = 0x03;
pub const T_PMIC_RST_DEB_1S: i32 = 0x04;
pub const T_PMIC_RST_DEB_2S: i32 = 0x05;
pub const T_PMIC_RST_DEB_4S: i32 = 0x06;
pub const T_PMIC_RST_DEB_8S: i32 = 0x07;

pub const I2C_LT_MASK: i32 = 0x03;
pub const I2C_LT_FORCE_DISABLE: i32 = 0x00;
pub const I2C_LT_ON_STANDBY_RUN: i32 = 0x01;
pub const I2C_LT_ON_RUN: i32 = 0x02;
pub const I2C_LT_FORCE_ENABLE: i32 = 0x03;

pub const SW_RST_COMMAND: i32 = 0x14;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
