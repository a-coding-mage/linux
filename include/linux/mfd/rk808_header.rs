
/*
 * Register definitions for Rockchip's RK808/RK818 PMIC
 *
 * Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
 *
 * Author: Chris Zhong <zyw@rock-chips.com>
 * Author: Zhang Qing <zhangqing@rock-chips.com>
 *
 * Copyright (C) 2016 PHYTEC Messtechnik GmbH
 *
 * Author: Wadim Egorov <w.egorov@phytec.de>
 */


// dependency: <linux/regulator/machine.h>
// dependency: <linux/regmap.h>

/*
 * rk808 Global Register Map.
 */

pub const RK808_DCDC1: u32 = 0 /* (0+RK808_START) */;
pub const RK808_LDO1: u32 = 4 /* (4+RK808_START) */;
pub const RK808_NUM_REGULATORS: u32 = 14;
#[repr(u32)]
pub enum rk808_reg {
	RK808_ID_DCDC1,
	RK808_ID_DCDC2,
	RK808_ID_DCDC3,
	RK808_ID_DCDC4,
	RK808_ID_LDO1,
	RK808_ID_LDO2,
	RK808_ID_LDO3,
	RK808_ID_LDO4,
	RK808_ID_LDO5,
	RK808_ID_LDO6,
	RK808_ID_LDO7,
	RK808_ID_LDO8,
	RK808_ID_SWITCH1,
	RK808_ID_SWITCH2,
};
pub const RK808_SECONDS_REG: u32 = 0x00;
pub const RK808_MINUTES_REG: u32 = 0x01;
pub const RK808_HOURS_REG: u32 = 0x02;
pub const RK808_DAYS_REG: u32 = 0x03;
pub const RK808_MONTHS_REG: u32 = 0x04;
pub const RK808_YEARS_REG: u32 = 0x05;
pub const RK808_WEEKS_REG: u32 = 0x06;
pub const RK808_ALARM_SECONDS_REG: u32 = 0x08;
pub const RK808_ALARM_MINUTES_REG: u32 = 0x09;
pub const RK808_ALARM_HOURS_REG: u32 = 0x0a;
pub const RK808_ALARM_DAYS_REG: u32 = 0x0b;
pub const RK808_ALARM_MONTHS_REG: u32 = 0x0c;
pub const RK808_ALARM_YEARS_REG: u32 = 0x0d;
pub const RK808_RTC_CTRL_REG: u32 = 0x10;
pub const RK808_RTC_STATUS_REG: u32 = 0x11;
pub const RK808_RTC_INT_REG: u32 = 0x12;
pub const RK808_RTC_COMP_LSB_REG: u32 = 0x13;
pub const RK808_RTC_COMP_MSB_REG: u32 = 0x14;
pub const RK808_ID_MSB: u32 = 0x17;
pub const RK808_ID_LSB: u32 = 0x18;
pub const RK808_CLK32OUT_REG: u32 = 0x20;
pub const RK808_VB_MON_REG: u32 = 0x21;
pub const RK808_THERMAL_REG: u32 = 0x22;
pub const RK808_DCDC_EN_REG: u32 = 0x23;
pub const RK808_LDO_EN_REG: u32 = 0x24;
pub const RK808_SLEEP_SET_OFF_REG1: u32 = 0x25;
pub const RK808_SLEEP_SET_OFF_REG2: u32 = 0x26;
pub const RK808_DCDC_UV_STS_REG: u32 = 0x27;
pub const RK808_DCDC_UV_ACT_REG: u32 = 0x28;
pub const RK808_LDO_UV_STS_REG: u32 = 0x29;
pub const RK808_LDO_UV_ACT_REG: u32 = 0x2a;
pub const RK808_DCDC_PG_REG: u32 = 0x2b;
pub const RK808_LDO_PG_REG: u32 = 0x2c;
pub const RK808_VOUT_MON_TDB_REG: u32 = 0x2d;
pub const RK808_BUCK1_CONFIG_REG: u32 = 0x2e;
pub const RK808_BUCK1_ON_VSEL_REG: u32 = 0x2f;
pub const RK808_BUCK1_SLP_VSEL_REG: u32 = 0x30;
pub const RK808_BUCK1_DVS_VSEL_REG: u32 = 0x31;
pub const RK808_BUCK2_CONFIG_REG: u32 = 0x32;
pub const RK808_BUCK2_ON_VSEL_REG: u32 = 0x33;
pub const RK808_BUCK2_SLP_VSEL_REG: u32 = 0x34;
pub const RK808_BUCK2_DVS_VSEL_REG: u32 = 0x35;
pub const RK808_BUCK3_CONFIG_REG: u32 = 0x36;
pub const RK808_BUCK4_CONFIG_REG: u32 = 0x37;
pub const RK808_BUCK4_ON_VSEL_REG: u32 = 0x38;
pub const RK808_BUCK4_SLP_VSEL_REG: u32 = 0x39;
pub const RK808_BOOST_CONFIG_REG: u32 = 0x3a;
pub const RK808_LDO1_ON_VSEL_REG: u32 = 0x3b;
pub const RK808_LDO1_SLP_VSEL_REG: u32 = 0x3c;
pub const RK808_LDO2_ON_VSEL_REG: u32 = 0x3d;
pub const RK808_LDO2_SLP_VSEL_REG: u32 = 0x3e;
pub const RK808_LDO3_ON_VSEL_REG: u32 = 0x3f;
pub const RK808_LDO3_SLP_VSEL_REG: u32 = 0x40;
pub const RK808_LDO4_ON_VSEL_REG: u32 = 0x41;
pub const RK808_LDO4_SLP_VSEL_REG: u32 = 0x42;
pub const RK808_LDO5_ON_VSEL_REG: u32 = 0x43;
pub const RK808_LDO5_SLP_VSEL_REG: u32 = 0x44;
pub const RK808_LDO6_ON_VSEL_REG: u32 = 0x45;
pub const RK808_LDO6_SLP_VSEL_REG: u32 = 0x46;
pub const RK808_LDO7_ON_VSEL_REG: u32 = 0x47;
pub const RK808_LDO7_SLP_VSEL_REG: u32 = 0x48;
pub const RK808_LDO8_ON_VSEL_REG: u32 = 0x49;
pub const RK808_LDO8_SLP_VSEL_REG: u32 = 0x4a;
pub const RK808_DEVCTRL_REG: u32 = 0x4b;
pub const RK808_INT_STS_REG1: u32 = 0x4c;
pub const RK808_INT_STS_MSK_REG1: u32 = 0x4d;
pub const RK808_INT_STS_REG2: u32 = 0x4e;
pub const RK808_INT_STS_MSK_REG2: u32 = 0x4f;
pub const RK808_IO_POL_REG: u32 = 0x50;
#[repr(u32)]
pub enum rk816_reg {
	RK816_ID_DCDC1,
	RK816_ID_DCDC2,
	RK816_ID_DCDC3,
	RK816_ID_DCDC4,
	RK816_ID_LDO1,
	RK816_ID_LDO2,
	RK816_ID_LDO3,
	RK816_ID_LDO4,
	RK816_ID_LDO5,
	RK816_ID_LDO6,
	RK816_ID_BOOST,
	RK816_ID_OTG_SW,
};
#[repr(u32)]
pub enum rk816_irqs {

	RK816_IRQ_PWRON_FALL,
	RK816_IRQ_PWRON_RISE,

	RK816_IRQ_VB_LOW,
	RK816_IRQ_PWRON,
	RK816_IRQ_PWRON_LP,
	RK816_IRQ_HOTDIE,
	RK816_IRQ_RTC_ALARM,
	RK816_IRQ_RTC_PERIOD,
	RK816_IRQ_USB_OV,

	RK816_IRQ_PLUG_IN,
	RK816_IRQ_PLUG_OUT,
	RK816_IRQ_CHG_OK,
	RK816_IRQ_CHG_TE,
	RK816_IRQ_CHG_TS,
	RK816_IRQ_CHG_CVTLIM,
	RK816_IRQ_DISCHG_ILIM,
};
pub const RK816_DCDC_EN_REG1: u32 = 0x23;
pub const RK816_DCDC_EN_REG2: u32 = 0x24;
pub const RK816_BOOST_EN: u32 = ((1u32) << 1);
pub const RK816_OTG_EN: u32 = ((1u32) << 2);
pub const RK816_BOOST_EN_MSK: u32 = ((1u32) << 5);
pub const RK816_OTG_EN_MSK: u32 = ((1u32) << 6);
pub const RK816_BUCK_DVS_CONFIRM: u32 = ((1u32) << 7);
pub const RK816_LDO_EN_REG1: u32 = 0x27;
pub const RK816_LDO_EN_REG2: u32 = 0x28;
pub const RK816_INT_STS_REG1: u32 = 0x49;
pub const RK816_INT_STS_MSK_REG1: u32 = 0x4a;
pub const RK816_INT_STS_PWRON_FALL: u32 = ((1u32) << 5);
pub const RK816_INT_STS_PWRON_RISE: u32 = ((1u32) << 6);
pub const RK816_INT_STS_REG2: u32 = 0x4c;
pub const RK816_INT_STS_MSK_REG2: u32 = 0x4d;
pub const RK816_INT_STS_VB_LOW: u32 = ((1u32) << 1);
pub const RK816_INT_STS_PWRON: u32 = ((1u32) << 2);
pub const RK816_INT_STS_PWRON_LP: u32 = ((1u32) << 3);
pub const RK816_INT_STS_HOTDIE: u32 = ((1u32) << 4);
pub const RK816_INT_STS_RTC_ALARM: u32 = ((1u32) << 5);
pub const RK816_INT_STS_RTC_PERIOD: u32 = ((1u32) << 6);
pub const RK816_INT_STS_USB_OV: u32 = ((1u32) << 7);
pub const RK816_INT_STS_REG3: u32 = 0x4e;
pub const RK816_INT_STS_MSK_REG3: u32 = 0x4f;
pub const RK816_INT_STS_PLUG_IN: u32 = ((1u32) << 0);
pub const RK816_INT_STS_PLUG_OUT: u32 = ((1u32) << 1);
pub const RK816_INT_STS_CHG_OK: u32 = ((1u32) << 2);
pub const RK816_INT_STS_CHG_TE: u32 = ((1u32) << 3);
pub const RK816_INT_STS_CHG_TS: u32 = ((1u32) << 4);
pub const RK816_INT_STS_CHG_CVTLIM: u32 = ((1u32) << 6);
pub const RK816_INT_STS_DISCHG_ILIM: u32 = ((1u32) << 7);
pub const fn RK816_IRQ_STS_OFFSET(x: u32) -> u32 { ((x) - RK816_INT_STS_REG1) }
pub const fn RK816_IRQ_MSK_OFFSET(x: u32) -> u32 { ((x) - RK816_INT_STS_MSK_REG1) }

pub const RK816_OTG_BUCK_LDO_CONFIG_REG: u32 = 0x2a;
pub const RK816_CHRG_CONFIG_REG: u32 = 0x2b;
pub const RK816_BOOST_ON_VESL_REG: u32 = 0x54;
pub const RK816_BOOST_SLP_VSEL_REG: u32 = 0x55;
pub const RK816_CHRG_BOOST_CONFIG_REG: u32 = 0x9a;
pub const RK816_SUP_STS_REG: u32 = 0xa0;
pub const RK816_USB_CTRL_REG: u32 = 0xa1;
pub const fn RK816_CHRG_CTRL(x: u32) -> u32 { (0xa3 + (x)) }
pub const RK816_BAT_CTRL_REG: u32 = 0xa6;
pub const RK816_BAT_HTS_TS_REG: u32 = 0xa8;
pub const RK816_BAT_LTS_TS_REG: u32 = 0xa9;
pub const RK816_TS_CTRL_REG: u32 = 0xac;
pub const RK816_ADC_CTRL_REG: u32 = 0xad;
pub const RK816_GGCON_REG: u32 = 0xb0;
pub const RK816_GGSTS_REG: u32 = 0xb1;
pub const RK816_ZERO_CUR_ADC_REGH: u32 = 0xb2;
pub const RK816_ZERO_CUR_ADC_REGL: u32 = 0xb3;
pub const fn RK816_GASCNT_CAL_REG(x: u32) -> u32 { (0xb7 - (x)) }
pub const fn RK816_GASCNT_REG(x: u32) -> u32 { (0xbb - (x)) }
pub const RK816_BAT_CUR_AVG_REGH: u32 = 0xbc;
pub const RK816_BAT_CUR_AVG_REGL: u32 = 0xbd;
pub const RK816_TS_ADC_REGH: u32 = 0xbe;
pub const RK816_TS_ADC_REGL: u32 = 0xbf;
pub const RK816_USB_ADC_REGH: u32 = 0xc0;
pub const RK816_USB_ADC_REGL: u32 = 0xc1;
pub const RK816_BAT_OCV_REGH: u32 = 0xc2;
pub const RK816_BAT_OCV_REGL: u32 = 0xc3;
pub const RK816_BAT_VOL_REGH: u32 = 0xc4;
pub const RK816_BAT_VOL_REGL: u32 = 0xc5;
pub const RK816_RELAX_ENTRY_THRES_REGH: u32 = 0xc6;
pub const RK816_RELAX_ENTRY_THRES_REGL: u32 = 0xc7;
pub const RK816_RELAX_EXIT_THRES_REGH: u32 = 0xc8;
pub const RK816_RELAX_EXIT_THRES_REGL: u32 = 0xc9;
pub const RK816_RELAX_VOL1_REGH: u32 = 0xca;
pub const RK816_RELAX_VOL1_REGL: u32 = 0xcb;
pub const RK816_RELAX_VOL2_REGH: u32 = 0xcc;
pub const RK816_RELAX_VOL2_REGL: u32 = 0xcd;
pub const RK816_RELAX_CUR1_REGH: u32 = 0xce;
pub const RK816_RELAX_CUR1_REGL: u32 = 0xcf;
pub const RK816_RELAX_CUR2_REGH: u32 = 0xd0;
pub const RK816_RELAX_CUR2_REGL: u32 = 0xd1;
pub const RK816_CAL_OFFSET_REGH: u32 = 0xd2;
pub const RK816_CAL_OFFSET_REGL: u32 = 0xd3;
pub const RK816_NON_ACT_TIMER_CNT_REG: u32 = 0xd4;
pub const RK816_VCALIB0_REGH: u32 = 0xd5;
pub const RK816_VCALIB0_REGL: u32 = 0xd6;
pub const RK816_VCALIB1_REGH: u32 = 0xd7;
pub const RK816_VCALIB1_REGL: u32 = 0xd8;
pub const fn RK816_FCC_GASCNT_REG(x: u32) -> u32 { (0xdc - (x)) }
pub const RK816_IOFFSET_REGH: u32 = 0xdd;
pub const RK816_IOFFSET_REGL: u32 = 0xde;
pub const RK816_SLEEP_CON_SAMP_CUR_REG: u32 = 0xdf;
pub const fn RK816_DATA_REG(x: u32) -> u32 { (0xe0 + (x)) }

pub const RK818_DCDC1: u32 = 0;
pub const RK818_LDO1: u32 = 4;
pub const RK818_NUM_REGULATORS: u32 = 17;
#[repr(u32)]
pub enum rk818_reg {
	RK818_ID_DCDC1,
	RK818_ID_DCDC2,
	RK818_ID_DCDC3,
	RK818_ID_DCDC4,
	RK818_ID_BOOST,
	RK818_ID_LDO1,
	RK818_ID_LDO2,
	RK818_ID_LDO3,
	RK818_ID_LDO4,
	RK818_ID_LDO5,
	RK818_ID_LDO6,
	RK818_ID_LDO7,
	RK818_ID_LDO8,
	RK818_ID_LDO9,
	RK818_ID_SWITCH,
	RK818_ID_HDMI_SWITCH,
	RK818_ID_OTG_SWITCH,
};
pub const RK818_DCDC_EN_REG: u32 = 0x23;
pub const RK818_LDO_EN_REG: u32 = 0x24;
pub const RK818_SLEEP_SET_OFF_REG1: u32 = 0x25;
pub const RK818_SLEEP_SET_OFF_REG2: u32 = 0x26;
pub const RK818_DCDC_UV_STS_REG: u32 = 0x27;
pub const RK818_DCDC_UV_ACT_REG: u32 = 0x28;
pub const RK818_LDO_UV_STS_REG: u32 = 0x29;
pub const RK818_LDO_UV_ACT_REG: u32 = 0x2a;
pub const RK818_DCDC_PG_REG: u32 = 0x2b;
pub const RK818_LDO_PG_REG: u32 = 0x2c;
pub const RK818_VOUT_MON_TDB_REG: u32 = 0x2d;
pub const RK818_BUCK1_CONFIG_REG: u32 = 0x2e;
pub const RK818_BUCK1_ON_VSEL_REG: u32 = 0x2f;
pub const RK818_BUCK1_SLP_VSEL_REG: u32 = 0x30;
pub const RK818_BUCK2_CONFIG_REG: u32 = 0x32;
pub const RK818_BUCK2_ON_VSEL_REG: u32 = 0x33;
pub const RK818_BUCK2_SLP_VSEL_REG: u32 = 0x34;
pub const RK818_BUCK3_CONFIG_REG: u32 = 0x36;
pub const RK818_BUCK4_CONFIG_REG: u32 = 0x37;
pub const RK818_BUCK4_ON_VSEL_REG: u32 = 0x38;
pub const RK818_BUCK4_SLP_VSEL_REG: u32 = 0x39;
pub const RK818_BOOST_CONFIG_REG: u32 = 0x3a;
pub const RK818_LDO1_ON_VSEL_REG: u32 = 0x3b;
pub const RK818_LDO1_SLP_VSEL_REG: u32 = 0x3c;
pub const RK818_LDO2_ON_VSEL_REG: u32 = 0x3d;
pub const RK818_LDO2_SLP_VSEL_REG: u32 = 0x3e;
pub const RK818_LDO3_ON_VSEL_REG: u32 = 0x3f;
pub const RK818_LDO3_SLP_VSEL_REG: u32 = 0x40;
pub const RK818_LDO4_ON_VSEL_REG: u32 = 0x41;
pub const RK818_LDO4_SLP_VSEL_REG: u32 = 0x42;
pub const RK818_LDO5_ON_VSEL_REG: u32 = 0x43;
pub const RK818_LDO5_SLP_VSEL_REG: u32 = 0x44;
pub const RK818_LDO6_ON_VSEL_REG: u32 = 0x45;
pub const RK818_LDO6_SLP_VSEL_REG: u32 = 0x46;
pub const RK818_LDO7_ON_VSEL_REG: u32 = 0x47;
pub const RK818_LDO7_SLP_VSEL_REG: u32 = 0x48;
pub const RK818_LDO8_ON_VSEL_REG: u32 = 0x49;
pub const RK818_LDO8_SLP_VSEL_REG: u32 = 0x4a;
pub const RK818_BOOST_LDO9_ON_VSEL_REG: u32 = 0x54;
pub const RK818_BOOST_LDO9_SLP_VSEL_REG: u32 = 0x55;
pub const RK818_DEVCTRL_REG: u32 = 0x4b;
pub const RK818_INT_STS_REG1: u32 = 0x4c;
pub const RK818_INT_STS_MSK_REG1: u32 = 0x4d;
pub const RK818_INT_STS_REG2: u32 = 0x4e;
pub const RK818_INT_STS_MSK_REG2: u32 = 0x4f;
pub const RK818_IO_POL_REG: u32 = 0x50;
pub const RK818_H5V_EN_REG: u32 = 0x52;
pub const RK818_SLEEP_SET_OFF_REG3: u32 = 0x53;
pub const RK818_BOOST_LDO9_ON_VSEL_REG: u32 = 0x54;
pub const RK818_BOOST_LDO9_SLP_VSEL_REG: u32 = 0x55;
pub const RK818_BOOST_CTRL_REG: u32 = 0x56;
pub const RK818_DCDC_ILMAX: u32 = 0x90;
pub const RK818_USB_CTRL_REG: u32 = 0xa1;
pub const RK818_H5V_EN: u32 = ((1u32) << 0);
pub const RK818_REF_RDY_CTRL: u32 = ((1u32) << 1);
pub const RK818_USB_ILIM_SEL_MASK: u32 = 0xf;
pub const RK818_USB_ILMIN_2000MA: u32 = 0x7;
pub const RK818_USB_CHG_SD_VSEL_MASK: u32 = 0x70;
#[repr(u32)]
pub enum rk801_reg {
	RK801_ID_DCDC1,
	RK801_ID_DCDC2,
	RK801_ID_DCDC4,
	RK801_ID_DCDC3,
	RK801_ID_LDO1,
	RK801_ID_LDO2,
	RK801_ID_SWITCH,
	RK801_ID_MAX,
};
pub const RK801_SLP_REG_OFFSET: u32 = 5;
pub const RK801_NUM_REGULATORS: u32 = 7;
pub const RK801_HW_SYNC_US: u32 = 32;
pub const RK801_ID_MSB: u32 = 0x00;
pub const RK801_ID_LSB: u32 = 0x01;
pub const RK801_OTP_VER_REG: u32 = 0x02;
pub const RK801_POWER_EN0_REG: u32 = 0x03;
pub const RK801_POWER_EN1_REG: u32 = 0x04;
pub const RK801_POWER_SLP_EN_REG: u32 = 0x05;
pub const RK801_POWER_FPWM_EN_REG: u32 = 0x06;
pub const RK801_SLP_LP_CONFIG_REG: u32 = 0x07;
pub const RK801_BUCK_CONFIG_REG: u32 = 0x08;
pub const RK801_BUCK1_ON_VSEL_REG: u32 = 0x09;
pub const RK801_BUCK2_ON_VSEL_REG: u32 = 0x0a;
pub const RK801_BUCK4_ON_VSEL_REG: u32 = 0x0b;
pub const RK801_LDO1_ON_VSEL_REG: u32 = 0x0c;
pub const RK801_LDO2_ON_VSEL_REG: u32 = 0x0d;
pub const RK801_BUCK1_SLP_VSEL_REG: u32 = 0x0e;
pub const RK801_BUCK2_SLP_VSEL_REG: u32 = 0x0f;
pub const RK801_BUCK4_SLP_VSEL_REG: u32 = 0x10;
pub const RK801_LDO1_SLP_VSEL_REG: u32 = 0x11;
pub const RK801_LDO2_SLP_VSEL_REG: u32 = 0x12;
pub const RK801_LDO_SW_IMAX_REG: u32 = 0x13;
pub const RK801_SYS_STS_REG: u32 = 0x14;
pub const RK801_SYS_CFG0_REG: u32 = 0x15;
pub const RK801_SYS_CFG1_REG: u32 = 0x16;
pub const RK801_SYS_CFG2_REG: u32 = 0x17;
pub const RK801_SYS_CFG3_REG: u32 = 0x18;
pub const RK801_SYS_CFG4_REG: u32 = 0x19;
pub const RK801_SLEEP_CFG_REG: u32 = 0x1a;
pub const RK801_ON_SOURCE_REG: u32 = 0x1b;
pub const RK801_OFF_SOURCE_REG: u32 = 0x1c;
pub const RK801_PWRON_KEY_REG: u32 = 0x1d;
pub const RK801_INT_STS0_REG: u32 = 0x1e;
pub const RK801_INT_MASK0_REG: u32 = 0x1f;
pub const RK801_INT_CONFIG_REG: u32 = 0x20;
pub const RK801_CON_BACK1_REG: u32 = 0x21;
pub const RK801_CON_BACK2_REG: u32 = 0x22;
pub const RK801_DATA_CON0_REG: u32 = 0x23;
pub const RK801_DATA_CON1_REG: u32 = 0x24;
pub const RK801_DATA_CON2_REG: u32 = 0x25;
pub const RK801_DATA_CON3_REG: u32 = 0x26;
pub const RK801_POWER_EXIT_SLP_SEQ0_REG: u32 = 0x27;
pub const RK801_POWER_EXIT_SLP_SEQ1_REG: u32 = 0x28;
pub const RK801_POWER_EXIT_SLP_SEQ2_REG: u32 = 0x29;
pub const RK801_POWER_EXIT_SLP_SEQ3_REG: u32 = 0x2a;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ0_REG: u32 = 0x2b;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ1_REG: u32 = 0x2c;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ2_REG: u32 = 0x2d;
pub const RK801_POWER_ENTER_SLP_OR_SHTD_SEQ3_REG: u32 = 0x2e;
pub const RK801_BUCK_DEBUG1_REG: u32 = 0x2f;
pub const RK801_BUCK_DEBUG2_REG: u32 = 0x30;
pub const RK801_BUCK_DEBUG3_REG: u32 = 0x31;
pub const RK801_BUCK_DEBUG4_REG: u32 = 0x32;
pub const RK801_BUCK_DEBUG5_REG: u32 = 0x33;
pub const RK801_BUCK_DEBUG7_REG: u32 = 0x34;
pub const RK801_OTP_EN_CON_REG: u32 = 0x35;
pub const RK801_TEST_CON_REG: u32 = 0x36;
pub const RK801_EFUSE_CONTROL_REG: u32 = 0x37;
pub const RK801_SYS_CFG3_OTP_REG: u32 = 0x38;
pub const RK801_IRQ_PWRON_FALL: u32 = 0;
pub const RK801_IRQ_PWRON_RISE: u32 = 1;
pub const RK801_IRQ_PWRON: u32 = 2;
pub const RK801_IRQ_PWRON_LP: u32 = 3;
pub const RK801_IRQ_HOTDIE: u32 = 4;
pub const RK801_IRQ_VDC_RISE: u32 = 5;
pub const RK801_IRQ_VDC_FALL: u32 = 6;
pub const RK801_IRQ_PWRON_FALL_MSK: u32 = ((1u32) << 0);
pub const RK801_IRQ_PWRON_RISE_MSK: u32 = ((1u32) << 1);
pub const RK801_IRQ_PWRON_MSK: u32 = ((1u32) << 2);
pub const RK801_IRQ_PWRON_LP_MSK: u32 = ((1u32) << 3);
pub const RK801_IRQ_HOTDIE_MSK: u32 = ((1u32) << 4);
pub const RK801_IRQ_VDC_RISE_MSK: u32 = ((1u32) << 5);
pub const RK801_IRQ_VDC_FALL_MSK: u32 = ((1u32) << 6);
pub const RK801_BUCK_SLP_LP_EN: u32 = ((1u32) << 3);
pub const RK801_PLDO_SLP_LP_EN: u32 = ((1u32) << 1);
pub const RK801_SLP_LP_MASK: u32 = (RK801_PLDO_SLP_LP_EN | RK801_BUCK_SLP_LP_EN);
pub const RK801_SLEEP_FUN_MSK: u32 = 0x3;
pub const RK801_NONE_FUN: u32 = 0x0;
pub const RK801_SLEEP_FUN: u32 = 0x1;
pub const RK801_SHUTDOWN_FUN: u32 = 0x2;
pub const RK801_RESET_FUN: u32 = 0x3;
pub const RK801_SLEEP_POL_MSK: u32 = ((1u32) << 1);
pub const RK801_SLEEP_ACT_H: u32 = ((1u32) << 1);
pub const RK801_SLEEP_ACT_L: u32 = 0;
pub const RK801_RST_MSK: u32 = ((0x3 << 4);
pub const RK801_RST_RESTART_PMU: u32 = ((0x0 << 4);
pub const RK801_RST_RESTART_REG: u32 = ((0x1 << 4);
pub const RK801_RST_RESTART_REG_RESETB: u32 = ((0x2 << 4);
pub const RK801_INT_POL_MSK: u32 = ((1u32) << 1);
pub const RK801_INT_ACT_H: u32 = ((1u32) << 1);
pub const RK801_INT_ACT_L: u32 = 0;
pub const RK801_FPWM_MODE: u32 = 1;
pub const RK801_AUTO_PWM_MODE: u32 = 0;
pub const RK801_PLDO_HRDEC_EN: u32 = ((1u32) << 6);
#[repr(u32)]
pub enum rk805_reg {
	RK805_ID_DCDC1,
	RK805_ID_DCDC2,
	RK805_ID_DCDC3,
	RK805_ID_DCDC4,
	RK805_ID_LDO1,
	RK805_ID_LDO2,
	RK805_ID_LDO3,
};
pub const RK805_VB_MON_REG: u32 = 0x21;
pub const RK805_THERMAL_REG: u32 = 0x22;
pub const RK805_DCDC_EN_REG: u32 = 0x23;
pub const RK805_SLP_DCDC_EN_REG: u32 = 0x25;
pub const RK805_SLP_LDO_EN_REG: u32 = 0x26;
pub const RK805_LDO_EN_REG: u32 = 0x27;
pub const RK805_BUCK_LDO_SLP_LP_EN_REG: u32 = 0x2A;
pub const RK805_BUCK1_CONFIG_REG: u32 = 0x2E;
pub const RK805_BUCK1_ON_VSEL_REG: u32 = 0x2F;
pub const RK805_BUCK1_SLP_VSEL_REG: u32 = 0x30;
pub const RK805_BUCK2_CONFIG_REG: u32 = 0x32;
pub const RK805_BUCK2_ON_VSEL_REG: u32 = 0x33;
pub const RK805_BUCK2_SLP_VSEL_REG: u32 = 0x34;
pub const RK805_BUCK3_CONFIG_REG: u32 = 0x36;
pub const RK805_BUCK4_CONFIG_REG: u32 = 0x37;
pub const RK805_BUCK4_ON_VSEL_REG: u32 = 0x38;
pub const RK805_BUCK4_SLP_VSEL_REG: u32 = 0x39;
pub const RK805_LDO1_ON_VSEL_REG: u32 = 0x3B;
pub const RK805_LDO1_SLP_VSEL_REG: u32 = 0x3C;
pub const RK805_LDO2_ON_VSEL_REG: u32 = 0x3D;
pub const RK805_LDO2_SLP_VSEL_REG: u32 = 0x3E;
pub const RK805_LDO3_ON_VSEL_REG: u32 = 0x3F;
pub const RK805_LDO3_SLP_VSEL_REG: u32 = 0x40;
pub const RK805_PWRON_LP_INT_TIME_REG: u32 = 0x47;
pub const RK805_PWRON_DB_REG: u32 = 0x48;
pub const RK805_DEV_CTRL_REG: u32 = 0x4B;
pub const RK805_INT_STS_REG: u32 = 0x4C;
pub const RK805_INT_STS_MSK_REG: u32 = 0x4D;
pub const RK805_GPIO_IO_POL_REG: u32 = 0x50;
pub const RK805_OUT_REG: u32 = 0x52;
pub const RK805_ON_SOURCE_REG: u32 = 0xAE;
pub const RK805_OFF_SOURCE_REG: u32 = 0xAF;
pub const RK805_NUM_REGULATORS: u32 = 7;
pub const RK805_PWRON_FALL_RISE_INT_EN: u32 = 0x0;
pub const RK805_PWRON_FALL_RISE_INT_MSK: u32 = 0x81;
pub const RK805_IRQ_PWRON_RISE: u32 = 0;
pub const RK805_IRQ_VB_LOW: u32 = 1;
pub const RK805_IRQ_PWRON: u32 = 2;
pub const RK805_IRQ_PWRON_LP: u32 = 3;
pub const RK805_IRQ_HOTDIE: u32 = 4;
pub const RK805_IRQ_RTC_ALARM: u32 = 5;
pub const RK805_IRQ_RTC_PERIOD: u32 = 6;
pub const RK805_IRQ_PWRON_FALL: u32 = 7;
pub const RK805_IRQ_PWRON_RISE_MSK: u32 = ((1u32) << 0);
pub const RK805_IRQ_VB_LOW_MSK: u32 = ((1u32) << 1);
pub const RK805_IRQ_PWRON_MSK: u32 = ((1u32) << 2);
pub const RK805_IRQ_PWRON_LP_MSK: u32 = ((1u32) << 3);
pub const RK805_IRQ_HOTDIE_MSK: u32 = ((1u32) << 4);
pub const RK805_IRQ_RTC_ALARM_MSK: u32 = ((1u32) << 5);
pub const RK805_IRQ_RTC_PERIOD_MSK: u32 = ((1u32) << 6);
pub const RK805_IRQ_PWRON_FALL_MSK: u32 = ((1u32) << 7);
pub const RK805_PWR_RISE_INT_STATUS: u32 = ((1u32) << 0);
pub const RK805_VB_LOW_INT_STATUS: u32 = ((1u32) << 1);
pub const RK805_PWRON_INT_STATUS: u32 = ((1u32) << 2);
pub const RK805_PWRON_LP_INT_STATUS: u32 = ((1u32) << 3);
pub const RK805_HOTDIE_INT_STATUS: u32 = ((1u32) << 4);
pub const RK805_ALARM_INT_STATUS: u32 = ((1u32) << 5);
pub const RK805_PERIOD_INT_STATUS: u32 = ((1u32) << 6);
pub const RK805_PWR_FALL_INT_STATUS: u32 = ((1u32) << 7);
pub const RK805_BUCK1_2_ILMAX_MASK: u32 = ((3 << 6);
pub const RK805_BUCK3_4_ILMAX_MASK: u32 = ((3 << 3);
pub const RK805_RTC_PERIOD_INT_MASK: u32 = ((1 << 6);
pub const RK805_RTC_ALARM_INT_MASK: u32 = ((1 << 5);
pub const RK805_INT_ALARM_EN: u32 = ((1 << 3);
pub const RK805_INT_TIMER_EN: u32 = ((1 << 2);
pub const RK806_POWER_EN0: u32 = 0x0;
pub const RK806_POWER_EN1: u32 = 0x1;
pub const RK806_POWER_EN2: u32 = 0x2;
pub const RK806_POWER_EN3: u32 = 0x3;
pub const RK806_POWER_EN4: u32 = 0x4;
pub const RK806_POWER_EN5: u32 = 0x5;
pub const RK806_POWER_SLP_EN0: u32 = 0x6;
pub const RK806_POWER_SLP_EN1: u32 = 0x7;
pub const RK806_POWER_SLP_EN2: u32 = 0x8;
pub const RK806_POWER_DISCHRG_EN0: u32 = 0x9;
pub const RK806_POWER_DISCHRG_EN1: u32 = 0xA;
pub const RK806_POWER_DISCHRG_EN2: u32 = 0xB;
pub const RK806_BUCK_FB_CONFIG: u32 = 0xC;
pub const RK806_SLP_LP_CONFIG: u32 = 0xD;
pub const RK806_POWER_FPWM_EN0: u32 = 0xE;
pub const RK806_POWER_FPWM_EN1: u32 = 0xF;
pub const RK806_BUCK1_CONFIG: u32 = 0x10;
pub const RK806_BUCK2_CONFIG: u32 = 0x11;
pub const RK806_BUCK3_CONFIG: u32 = 0x12;
pub const RK806_BUCK4_CONFIG: u32 = 0x13;
pub const RK806_BUCK5_CONFIG: u32 = 0x14;
pub const RK806_BUCK6_CONFIG: u32 = 0x15;
pub const RK806_BUCK7_CONFIG: u32 = 0x16;
pub const RK806_BUCK8_CONFIG: u32 = 0x17;
pub const RK806_BUCK9_CONFIG: u32 = 0x18;
pub const RK806_BUCK10_CONFIG: u32 = 0x19;
pub const RK806_BUCK1_ON_VSEL: u32 = 0x1A;
pub const RK806_BUCK2_ON_VSEL: u32 = 0x1B;
pub const RK806_BUCK3_ON_VSEL: u32 = 0x1C;
pub const RK806_BUCK4_ON_VSEL: u32 = 0x1D;
pub const RK806_BUCK5_ON_VSEL: u32 = 0x1E;
pub const RK806_BUCK6_ON_VSEL: u32 = 0x1F;
pub const RK806_BUCK7_ON_VSEL: u32 = 0x20;
pub const RK806_BUCK8_ON_VSEL: u32 = 0x21;
pub const RK806_BUCK9_ON_VSEL: u32 = 0x22;
pub const RK806_BUCK10_ON_VSEL: u32 = 0x23;
pub const RK806_BUCK1_SLP_VSEL: u32 = 0x24;
pub const RK806_BUCK2_SLP_VSEL: u32 = 0x25;
pub const RK806_BUCK3_SLP_VSEL: u32 = 0x26;
pub const RK806_BUCK4_SLP_VSEL: u32 = 0x27;
pub const RK806_BUCK5_SLP_VSEL: u32 = 0x28;
pub const RK806_BUCK6_SLP_VSEL: u32 = 0x29;
pub const RK806_BUCK7_SLP_VSEL: u32 = 0x2A;
pub const RK806_BUCK8_SLP_VSEL: u32 = 0x2B;
pub const RK806_BUCK9_SLP_VSEL: u32 = 0x2D;
pub const RK806_BUCK10_SLP_VSEL: u32 = 0x2E;
pub const RK806_BUCK_DEBUG1: u32 = 0x30;
pub const RK806_BUCK_DEBUG2: u32 = 0x31;
pub const RK806_BUCK_DEBUG3: u32 = 0x32;
pub const RK806_BUCK_DEBUG4: u32 = 0x33;
pub const RK806_BUCK_DEBUG5: u32 = 0x34;
pub const RK806_BUCK_DEBUG6: u32 = 0x35;
pub const RK806_BUCK_DEBUG7: u32 = 0x36;
pub const RK806_BUCK_DEBUG8: u32 = 0x37;
pub const RK806_BUCK_DEBUG9: u32 = 0x38;
pub const RK806_BUCK_DEBUG10: u32 = 0x39;
pub const RK806_BUCK_DEBUG11: u32 = 0x3A;
pub const RK806_BUCK_DEBUG12: u32 = 0x3B;
pub const RK806_BUCK_DEBUG13: u32 = 0x3C;
pub const RK806_BUCK_DEBUG14: u32 = 0x3D;
pub const RK806_BUCK_DEBUG15: u32 = 0x3E;
pub const RK806_BUCK_DEBUG16: u32 = 0x3F;
pub const RK806_BUCK_DEBUG17: u32 = 0x40;
pub const RK806_BUCK_DEBUG18: u32 = 0x41;
pub const RK806_NLDO_IMAX: u32 = 0x42;
pub const RK806_NLDO1_ON_VSEL: u32 = 0x43;
pub const RK806_NLDO2_ON_VSEL: u32 = 0x44;
pub const RK806_NLDO3_ON_VSEL: u32 = 0x45;
pub const RK806_NLDO4_ON_VSEL: u32 = 0x46;
pub const RK806_NLDO5_ON_VSEL: u32 = 0x47;
pub const RK806_NLDO1_SLP_VSEL: u32 = 0x48;
pub const RK806_NLDO2_SLP_VSEL: u32 = 0x49;
pub const RK806_NLDO3_SLP_VSEL: u32 = 0x4A;
pub const RK806_NLDO4_SLP_VSEL: u32 = 0x4B;
pub const RK806_NLDO5_SLP_VSEL: u32 = 0x4C;
pub const RK806_PLDO_IMAX: u32 = 0x4D;
pub const RK806_PLDO1_ON_VSEL: u32 = 0x4E;
pub const RK806_PLDO2_ON_VSEL: u32 = 0x4F;
pub const RK806_PLDO3_ON_VSEL: u32 = 0x50;
pub const RK806_PLDO4_ON_VSEL: u32 = 0x51;
pub const RK806_PLDO5_ON_VSEL: u32 = 0x52;
pub const RK806_PLDO6_ON_VSEL: u32 = 0x53;
pub const RK806_PLDO1_SLP_VSEL: u32 = 0x54;
pub const RK806_PLDO2_SLP_VSEL: u32 = 0x55;
pub const RK806_PLDO3_SLP_VSEL: u32 = 0x56;
pub const RK806_PLDO4_SLP_VSEL: u32 = 0x57;
pub const RK806_PLDO5_SLP_VSEL: u32 = 0x58;
pub const RK806_PLDO6_SLP_VSEL: u32 = 0x59;
pub const RK806_CHIP_NAME: u32 = 0x5A;
pub const RK806_CHIP_VER: u32 = 0x5B;
pub const RK806_OTP_VER: u32 = 0x5C;
pub const RK806_SYS_STS: u32 = 0x5D;
pub const RK806_SYS_CFG0: u32 = 0x5E;
pub const RK806_SYS_CFG1: u32 = 0x5F;
pub const RK806_SYS_OPTION: u32 = 0x61;
pub const RK806_SLEEP_CONFIG0: u32 = 0x62;
pub const RK806_SLEEP_CONFIG1: u32 = 0x63;
pub const RK806_SLEEP_CTR_SEL0: u32 = 0x64;
pub const RK806_SLEEP_CTR_SEL1: u32 = 0x65;
pub const RK806_SLEEP_CTR_SEL2: u32 = 0x66;
pub const RK806_SLEEP_CTR_SEL3: u32 = 0x67;
pub const RK806_SLEEP_CTR_SEL4: u32 = 0x68;
pub const RK806_SLEEP_CTR_SEL5: u32 = 0x69;
pub const RK806_DVS_CTRL_SEL0: u32 = 0x6A;
pub const RK806_DVS_CTRL_SEL1: u32 = 0x6B;
pub const RK806_DVS_CTRL_SEL2: u32 = 0x6C;
pub const RK806_DVS_CTRL_SEL3: u32 = 0x6D;
pub const RK806_DVS_CTRL_SEL4: u32 = 0x6E;
pub const RK806_DVS_CTRL_SEL5: u32 = 0x6F;
pub const RK806_DVS_START_CTRL: u32 = 0x70;
pub const RK806_SLEEP_GPIO: u32 = 0x71;
pub const RK806_SYS_CFG3: u32 = 0x72;
pub const RK806_ON_SOURCE: u32 = 0x74;
pub const RK806_OFF_SOURCE: u32 = 0x75;
pub const RK806_PWRON_KEY: u32 = 0x76;
pub const RK806_INT_STS0: u32 = 0x77;
pub const RK806_INT_MSK0: u32 = 0x78;
pub const RK806_INT_STS1: u32 = 0x79;
pub const RK806_INT_MSK1: u32 = 0x7A;
pub const RK806_GPIO_INT_CONFIG: u32 = 0x7B;
pub const RK806_DATA_REG0: u32 = 0x7C;
pub const RK806_DATA_REG1: u32 = 0x7D;
pub const RK806_DATA_REG2: u32 = 0x7E;
pub const RK806_DATA_REG3: u32 = 0x7F;
pub const RK806_DATA_REG4: u32 = 0x80;
pub const RK806_DATA_REG5: u32 = 0x81;
pub const RK806_DATA_REG6: u32 = 0x82;
pub const RK806_DATA_REG7: u32 = 0x83;
pub const RK806_DATA_REG8: u32 = 0x84;
pub const RK806_DATA_REG9: u32 = 0x85;
pub const RK806_DATA_REG10: u32 = 0x86;
pub const RK806_DATA_REG11: u32 = 0x87;
pub const RK806_DATA_REG12: u32 = 0x88;
pub const RK806_DATA_REG13: u32 = 0x89;
pub const RK806_DATA_REG14: u32 = 0x8A;
pub const RK806_DATA_REG15: u32 = 0x8B;
pub const RK806_TM_REG: u32 = 0x8C;
pub const RK806_OTP_EN_REG: u32 = 0x8D;
pub const RK806_FUNC_OTP_EN_REG: u32 = 0x8E;
pub const RK806_TEST_REG1: u32 = 0x8F;
pub const RK806_TEST_REG2: u32 = 0x90;
pub const RK806_TEST_REG3: u32 = 0x91;
pub const RK806_TEST_REG4: u32 = 0x92;
pub const RK806_TEST_REG5: u32 = 0x93;
pub const RK806_BUCK_VSEL_OTP_REG0: u32 = 0x94;
pub const RK806_BUCK_VSEL_OTP_REG1: u32 = 0x95;
pub const RK806_BUCK_VSEL_OTP_REG2: u32 = 0x96;
pub const RK806_BUCK_VSEL_OTP_REG3: u32 = 0x97;
pub const RK806_BUCK_VSEL_OTP_REG4: u32 = 0x98;
pub const RK806_BUCK_VSEL_OTP_REG5: u32 = 0x99;
pub const RK806_BUCK_VSEL_OTP_REG6: u32 = 0x9A;
pub const RK806_BUCK_VSEL_OTP_REG7: u32 = 0x9B;
pub const RK806_BUCK_VSEL_OTP_REG8: u32 = 0x9C;
pub const RK806_BUCK_VSEL_OTP_REG9: u32 = 0x9D;
pub const RK806_NLDO1_VSEL_OTP_REG0: u32 = 0x9E;
pub const RK806_NLDO1_VSEL_OTP_REG1: u32 = 0x9F;
pub const RK806_NLDO1_VSEL_OTP_REG2: u32 = 0xA0;
pub const RK806_NLDO1_VSEL_OTP_REG3: u32 = 0xA1;
pub const RK806_NLDO1_VSEL_OTP_REG4: u32 = 0xA2;
pub const RK806_PLDO_VSEL_OTP_REG0: u32 = 0xA3;
pub const RK806_PLDO_VSEL_OTP_REG1: u32 = 0xA4;
pub const RK806_PLDO_VSEL_OTP_REG2: u32 = 0xA5;
pub const RK806_PLDO_VSEL_OTP_REG3: u32 = 0xA6;
pub const RK806_PLDO_VSEL_OTP_REG4: u32 = 0xA7;
pub const RK806_PLDO_VSEL_OTP_REG5: u32 = 0xA8;
pub const RK806_BUCK_EN_OTP_REG1: u32 = 0xA9;
pub const RK806_NLDO_EN_OTP_REG1: u32 = 0xAA;
pub const RK806_PLDO_EN_OTP_REG1: u32 = 0xAB;
pub const RK806_BUCK_FB_RES_OTP_REG1: u32 = 0xAC;
pub const RK806_OTP_RESEV_REG0: u32 = 0xAD;
pub const RK806_OTP_RESEV_REG1: u32 = 0xAE;
pub const RK806_OTP_RESEV_REG2: u32 = 0xAF;
pub const RK806_OTP_RESEV_REG3: u32 = 0xB0;
pub const RK806_OTP_RESEV_REG4: u32 = 0xB1;
pub const RK806_BUCK_SEQ_REG0: u32 = 0xB2;
pub const RK806_BUCK_SEQ_REG1: u32 = 0xB3;
pub const RK806_BUCK_SEQ_REG2: u32 = 0xB4;
pub const RK806_BUCK_SEQ_REG3: u32 = 0xB5;
pub const RK806_BUCK_SEQ_REG4: u32 = 0xB6;
pub const RK806_BUCK_SEQ_REG5: u32 = 0xB7;
pub const RK806_BUCK_SEQ_REG6: u32 = 0xB8;
pub const RK806_BUCK_SEQ_REG7: u32 = 0xB9;
pub const RK806_BUCK_SEQ_REG8: u32 = 0xBA;
pub const RK806_BUCK_SEQ_REG9: u32 = 0xBB;
pub const RK806_BUCK_SEQ_REG10: u32 = 0xBC;
pub const RK806_BUCK_SEQ_REG11: u32 = 0xBD;
pub const RK806_BUCK_SEQ_REG12: u32 = 0xBE;
pub const RK806_BUCK_SEQ_REG13: u32 = 0xBF;
pub const RK806_BUCK_SEQ_REG14: u32 = 0xC0;
pub const RK806_BUCK_SEQ_REG15: u32 = 0xC1;
pub const RK806_BUCK_SEQ_REG16: u32 = 0xC2;
pub const RK806_BUCK_SEQ_REG17: u32 = 0xC3;
pub const RK806_HK_TRIM_REG1: u32 = 0xC4;
pub const RK806_HK_TRIM_REG2: u32 = 0xC5;
pub const RK806_BUCK_REF_TRIM_REG1: u32 = 0xC6;
pub const RK806_BUCK_REF_TRIM_REG2: u32 = 0xC7;
pub const RK806_BUCK_REF_TRIM_REG3: u32 = 0xC8;
pub const RK806_BUCK_REF_TRIM_REG4: u32 = 0xC9;
pub const RK806_BUCK_REF_TRIM_REG5: u32 = 0xCA;
pub const RK806_BUCK_OSC_TRIM_REG1: u32 = 0xCB;
pub const RK806_BUCK_OSC_TRIM_REG2: u32 = 0xCC;
pub const RK806_BUCK_OSC_TRIM_REG3: u32 = 0xCD;
pub const RK806_BUCK_OSC_TRIM_REG4: u32 = 0xCE;
pub const RK806_BUCK_OSC_TRIM_REG5: u32 = 0xCF;
pub const RK806_BUCK_TRIM_ZCDIOS_REG1: u32 = 0xD0;
pub const RK806_BUCK_TRIM_ZCDIOS_REG2: u32 = 0xD1;
pub const RK806_NLDO_TRIM_REG1: u32 = 0xD2;
pub const RK806_NLDO_TRIM_REG2: u32 = 0xD3;
pub const RK806_NLDO_TRIM_REG3: u32 = 0xD4;
pub const RK806_PLDO_TRIM_REG1: u32 = 0xD5;
pub const RK806_PLDO_TRIM_REG2: u32 = 0xD6;
pub const RK806_PLDO_TRIM_REG3: u32 = 0xD7;
pub const RK806_TRIM_ICOMP_REG1: u32 = 0xD8;
pub const RK806_TRIM_ICOMP_REG2: u32 = 0xD9;
pub const RK806_EFUSE_CONTROL_REGH: u32 = 0xDA;
pub const RK806_FUSE_PROG_REG: u32 = 0xDB;
pub const RK806_MAIN_FSM_STS_REG: u32 = 0xDD;
pub const RK806_FSM_REG: u32 = 0xDE;
pub const RK806_TOP_RESEV_OFFR: u32 = 0xEC;
pub const RK806_TOP_RESEV_POR: u32 = 0xED;
pub const RK806_BUCK_VRSN_REG1: u32 = 0xEE;
pub const RK806_BUCK_VRSN_REG2: u32 = 0xEF;
pub const RK806_NLDO_RLOAD_SEL_REG1: u32 = 0xF0;
pub const RK806_PLDO_RLOAD_SEL_REG1: u32 = 0xF1;
pub const RK806_PLDO_RLOAD_SEL_REG2: u32 = 0xF2;
pub const RK806_BUCK_CMIN_MX_REG1: u32 = 0xF3;
pub const RK806_BUCK_CMIN_MX_REG2: u32 = 0xF4;
pub const RK806_BUCK_FREQ_SET_REG1: u32 = 0xF5;
pub const RK806_BUCK_FREQ_SET_REG2: u32 = 0xF6;
pub const RK806_BUCK_RS_MEABS_REG1: u32 = 0xF7;
pub const RK806_BUCK_RS_MEABS_REG2: u32 = 0xF8;
pub const RK806_BUCK_RS_ZDLEB_REG1: u32 = 0xF9;
pub const RK806_BUCK_RS_ZDLEB_REG2: u32 = 0xFA;
pub const RK806_BUCK_RSERVE_REG1: u32 = 0xFB;
pub const RK806_BUCK_RSERVE_REG2: u32 = 0xFC;
pub const RK806_BUCK_RSERVE_REG3: u32 = 0xFD;
pub const RK806_BUCK_RSERVE_REG4: u32 = 0xFE;
pub const RK806_BUCK_RSERVE_REG5: u32 = 0xFF;
pub const RK806_INT_STS_PWRON_FALL: u32 = ((1u32) << 0);
pub const RK806_INT_STS_PWRON_RISE: u32 = ((1u32) << 1);
pub const RK806_INT_STS_PWRON: u32 = ((1u32) << 2);
pub const RK806_INT_STS_PWRON_LP: u32 = ((1u32) << 3);
pub const RK806_INT_STS_HOTDIE: u32 = ((1u32) << 4);
pub const RK806_INT_STS_VDC_RISE: u32 = ((1u32) << 5);
pub const RK806_INT_STS_VDC_FALL: u32 = ((1u32) << 6);
pub const RK806_INT_STS_VB_LO: u32 = ((1u32) << 7);
pub const RK806_INT_STS_REV0: u32 = ((1u32) << 0);
pub const RK806_INT_STS_REV1: u32 = ((1u32) << 1);
pub const RK806_INT_STS_REV2: u32 = ((1u32) << 2);
pub const RK806_INT_STS_CRC_ERROR: u32 = ((1u32) << 3);
pub const RK806_INT_STS_SLP3_GPIO: u32 = ((1u32) << 4);
pub const RK806_INT_STS_SLP2_GPIO: u32 = ((1u32) << 5);
pub const RK806_INT_STS_SLP1_GPIO: u32 = ((1u32) << 6);
pub const RK806_INT_STS_WDT: u32 = ((1u32) << 7);
pub const RK806_CMD_READ: u32 = 0;
pub const RK806_CMD_WRITE: u32 = ((1u32) << 7);
pub const RK806_CMD_CRC_EN: u32 = ((1u32) << 6);
pub const RK806_CMD_CRC_DIS: u32 = 0;
pub const RK806_CMD_LEN_MSK: u32 = 0x0f;
pub const RK806_REG_H: u32 = 0x00;
pub const VERSION_AB: u32 = 0x01;
#[repr(u32)]
pub enum rk806_reg_id {
	RK806_ID_DCDC1 = 0,
	RK806_ID_DCDC2,
	RK806_ID_DCDC3,
	RK806_ID_DCDC4,
	RK806_ID_DCDC5,
	RK806_ID_DCDC6,
	RK806_ID_DCDC7,
	RK806_ID_DCDC8,
	RK806_ID_DCDC9,
	RK806_ID_DCDC10,

	RK806_ID_NLDO1,
	RK806_ID_NLDO2,
	RK806_ID_NLDO3,
	RK806_ID_NLDO4,
	RK806_ID_NLDO5,

	RK806_ID_PLDO1,
	RK806_ID_PLDO2,
	RK806_ID_PLDO3,
	RK806_ID_PLDO4,
	RK806_ID_PLDO5,
	RK806_ID_PLDO6,
	RK806_ID_END,
};
#[repr(u32)]
pub enum rk806_irqs {

	RK806_IRQ_PWRON_FALL,
	RK806_IRQ_PWRON_RISE,
	RK806_IRQ_PWRON,
	RK806_IRQ_PWRON_LP,
	RK806_IRQ_HOTDIE,
	RK806_IRQ_VDC_RISE,
	RK806_IRQ_VDC_FALL,
	RK806_IRQ_VB_LO,

	RK806_IRQ_REV0,
	RK806_IRQ_REV1,
	RK806_IRQ_REV2,
	RK806_IRQ_CRC_ERROR,
	RK806_IRQ_SLP3_GPIO,
	RK806_IRQ_SLP2_GPIO,
	RK806_IRQ_SLP1_GPIO,
	RK806_IRQ_WDT,
};
#[repr(u32)]
pub enum rk806_lv_sel {
	VB_LO_SEL_2800,
	VB_LO_SEL_2900,
	VB_LO_SEL_3000,
	VB_LO_SEL_3100,
	VB_LO_SEL_3200,
	VB_LO_SEL_3300,
	VB_LO_SEL_3400,
	VB_LO_SEL_3500,
};
#[repr(u32)]
pub enum rk806_uv_sel {
	VB_UV_SEL_2700,
	VB_UV_SEL_2800,
	VB_UV_SEL_2900,
	VB_UV_SEL_3000,
	VB_UV_SEL_3100,
	VB_UV_SEL_3200,
	VB_UV_SEL_3300,
	VB_UV_SEL_3400,
};
#[repr(u32)]
pub enum rk806_pwrctrl_fun {
	PWRCTRL_NULL_FUN,
	PWRCTRL_SLP_FUN,
	PWRCTRL_POWOFF_FUN,
	PWRCTRL_RST_FUN,
	PWRCTRL_DVS_FUN,
	PWRCTRL_GPIO_FUN,
};
#[repr(u32)]
pub enum rk806_pin_level {
	POL_LOW,
	POL_HIGH,
};
#[repr(u32)]
pub enum rk806_vsel_ctr_sel {
	CTR_BY_NO_EFFECT,
	CTR_BY_PWRCTRL1,
	CTR_BY_PWRCTRL2,
	CTR_BY_PWRCTRL3,
};
#[repr(u32)]
pub enum rk806_dvs_ctr_sel {
	CTR_SEL_NO_EFFECT,
	CTR_SEL_DVS_START1,
	CTR_SEL_DVS_START2,
	CTR_SEL_DVS_START3,
};
#[repr(u32)]
pub enum rk806_pin_dr_sel {
	RK806_PIN_INPUT,
	RK806_PIN_OUTPUT,
};
pub const RK806_INT_POL_MSK: u32 = ((1u32) << 1);
pub const RK806_INT_POL_H: u32 = ((1u32) << 1);
pub const RK806_INT_POL_L: u32 = 0;
pub const RK806_RST_FUN_MSK: u32 = (((1u32 << (7 + 1)) - 1) & !((1u32 << 6) - 1));
pub const RK806_SLAVE_RESTART_FUN_MSK: u32 = ((1u32) << 1);
pub const RK806_SLAVE_RESTART_FUN_EN: u32 = ((1u32) << 1);
pub const RK806_SLAVE_RESTART_FUN_OFF: u32 = 0;
pub const RK806_SYS_ENB2_2M_MSK: u32 = ((1u32) << 1);
pub const RK806_SYS_ENB2_2M_EN: u32 = ((1u32) << 1);
pub const RK806_SYS_ENB2_2M_OFF: u32 = 0;
#[repr(u32)]
pub enum rk806_int_fun {
	RK806_INT_ONLY,
	RK806_INT_ADN_WKUP,
};
#[repr(u32)]
pub enum rk806_dvs_mode {
	RK806_DVS_NOT_SUPPORT,
	RK806_DVS_START1,
	RK806_DVS_START2,
	RK806_DVS_START3,
	RK806_DVS_PWRCTRL1,
	RK806_DVS_PWRCTRL2,
	RK806_DVS_PWRCTRL3,
	RK806_DVS_START_PWRCTR1,
	RK806_DVS_START_PWRCTR2,
	RK806_DVS_START_PWRCTR3,
	RK806_DVS_END,
};
pub const RK808_IRQ_VOUT_LO: u32 = 0;
pub const RK808_IRQ_VB_LO: u32 = 1;
pub const RK808_IRQ_PWRON: u32 = 2;
pub const RK808_IRQ_PWRON_LP: u32 = 3;
pub const RK808_IRQ_HOTDIE: u32 = 4;
pub const RK808_IRQ_RTC_ALARM: u32 = 5;
pub const RK808_IRQ_RTC_PERIOD: u32 = 6;
pub const RK808_IRQ_PLUG_IN_INT: u32 = 7;
pub const RK808_IRQ_PLUG_OUT_INT: u32 = 8;
pub const RK808_NUM_IRQ: u32 = 9;
pub const RK808_IRQ_VOUT_LO_MSK: u32 = ((1u32) << 0);
pub const RK808_IRQ_VB_LO_MSK: u32 = ((1u32) << 1);
pub const RK808_IRQ_PWRON_MSK: u32 = ((1u32) << 2);
pub const RK808_IRQ_PWRON_LP_MSK: u32 = ((1u32) << 3);
pub const RK808_IRQ_HOTDIE_MSK: u32 = ((1u32) << 4);
pub const RK808_IRQ_RTC_ALARM_MSK: u32 = ((1u32) << 5);
pub const RK808_IRQ_RTC_PERIOD_MSK: u32 = ((1u32) << 6);
pub const RK808_IRQ_PLUG_IN_INT_MSK: u32 = ((1u32) << 0);
pub const RK808_IRQ_PLUG_OUT_INT_MSK: u32 = ((1u32) << 1);
pub const RK818_IRQ_VOUT_LO: u32 = 0;
pub const RK818_IRQ_VB_LO: u32 = 1;
pub const RK818_IRQ_PWRON: u32 = 2;
pub const RK818_IRQ_PWRON_LP: u32 = 3;
pub const RK818_IRQ_HOTDIE: u32 = 4;
pub const RK818_IRQ_RTC_ALARM: u32 = 5;
pub const RK818_IRQ_RTC_PERIOD: u32 = 6;
pub const RK818_IRQ_USB_OV: u32 = 7;
pub const RK818_IRQ_PLUG_IN: u32 = 8;
pub const RK818_IRQ_PLUG_OUT: u32 = 9;
pub const RK818_IRQ_CHG_OK: u32 = 10;
pub const RK818_IRQ_CHG_TE: u32 = 11;
pub const RK818_IRQ_CHG_TS1: u32 = 12;
pub const RK818_IRQ_TS2: u32 = 13;
pub const RK818_IRQ_CHG_CVTLIM: u32 = 14;
pub const RK818_IRQ_DISCHG_ILIM: u32 = 15;
pub const RK818_IRQ_VOUT_LO_MSK: u32 = ((1u32) << 0);
pub const RK818_IRQ_VB_LO_MSK: u32 = ((1u32) << 1);
pub const RK818_IRQ_PWRON_MSK: u32 = ((1u32) << 2);
pub const RK818_IRQ_PWRON_LP_MSK: u32 = ((1u32) << 3);
pub const RK818_IRQ_HOTDIE_MSK: u32 = ((1u32) << 4);
pub const RK818_IRQ_RTC_ALARM_MSK: u32 = ((1u32) << 5);
pub const RK818_IRQ_RTC_PERIOD_MSK: u32 = ((1u32) << 6);
pub const RK818_IRQ_USB_OV_MSK: u32 = ((1u32) << 7);
pub const RK818_IRQ_PLUG_IN_MSK: u32 = ((1u32) << 0);
pub const RK818_IRQ_PLUG_OUT_MSK: u32 = ((1u32) << 1);
pub const RK818_IRQ_CHG_OK_MSK: u32 = ((1u32) << 2);
pub const RK818_IRQ_CHG_TE_MSK: u32 = ((1u32) << 3);
pub const RK818_IRQ_CHG_TS1_MSK: u32 = ((1u32) << 4);
pub const RK818_IRQ_TS2_MSK: u32 = ((1u32) << 5);
pub const RK818_IRQ_CHG_CVTLIM_MSK: u32 = ((1u32) << 6);
pub const RK818_IRQ_DISCHG_ILIM_MSK: u32 = ((1u32) << 7);
pub const RK818_NUM_IRQ: u32 = 16;
pub const RK808_VBAT_LOW_2V8: u32 = 0x00;
pub const RK808_VBAT_LOW_2V9: u32 = 0x01;
pub const RK808_VBAT_LOW_3V0: u32 = 0x02;
pub const RK808_VBAT_LOW_3V1: u32 = 0x03;
pub const RK808_VBAT_LOW_3V2: u32 = 0x04;
pub const RK808_VBAT_LOW_3V3: u32 = 0x05;
pub const RK808_VBAT_LOW_3V4: u32 = 0x06;
pub const RK808_VBAT_LOW_3V5: u32 = 0x07;
pub const VBAT_LOW_VOL_MASK: u32 = ((0x07 << 0);
pub const EN_VABT_LOW_SHUT_DOWN: u32 = ((0x00 << 4);
pub const EN_VBAT_LOW_IRQ: u32 = ((0x1 << 4);
pub const VBAT_LOW_ACT_MASK: u32 = ((0x1 << 4);
pub const BUCK_ILMIN_MASK: u32 = ((7 << 0);
pub const BOOST_ILMIN_MASK: u32 = ((7 << 0);
pub const BUCK1_RATE_MASK: u32 = ((3 << 3);
pub const BUCK2_RATE_MASK: u32 = ((3 << 3);
pub const MASK_ALL: u32 = 0xff;
pub const BUCK_UV_ACT_MASK: u32 = 0x0f;
pub const BUCK_UV_ACT_DISABLE: u32 = 0;
pub const SWITCH2_EN: u32 = ((1u32) << 6);
pub const SWITCH1_EN: u32 = ((1u32) << 5);
pub const DEV_OFF_RST: u32 = ((1u32) << 3);
pub const DEV_RST: u32 = ((1u32) << 2);
pub const DEV_OFF: u32 = ((1u32) << 0);
pub const RTC_STOP: u32 = ((1u32) << 0);
pub const VB_LO_ACT: u32 = ((1u32) << 4);
pub const VB_LO_SEL_3500MV: u32 = ((7 << 0);
pub const VOUT_LO_INT: u32 = ((1u32) << 0);
pub const CLK32KOUT2_EN: u32 = ((1u32) << 0);
pub const TEMP105C: u32 = 0x08;
pub const TEMP115C: u32 = 0x0c;
pub const TEMP_HOTDIE_MSK: u32 = 0x0c;
pub const SLP_SD_MSK: u32 = ((0x3 << 2);
pub const SHUTDOWN_FUN: u32 = ((0x2 << 2);
pub const SLEEP_FUN: u32 = ((0x1 << 2);
pub const RK8XX_ID_MSK: u32 = 0xfff0;
pub const PWM_MODE_MSK: u32 = ((1u32) << 7);
pub const FPWM_MODE: u32 = ((1u32) << 7);
pub const AUTO_PWM_MODE: u32 = 0;
#[repr(u32)]
pub enum rk817_reg_id {
	RK817_ID_DCDC1 = 0,
	RK817_ID_DCDC2,
	RK817_ID_DCDC3,
	RK817_ID_DCDC4,
	RK817_ID_LDO1,
	RK817_ID_LDO2,
	RK817_ID_LDO3,
	RK817_ID_LDO4,
	RK817_ID_LDO5,
	RK817_ID_LDO6,
	RK817_ID_LDO7,
	RK817_ID_LDO8,
	RK817_ID_LDO9,
	RK817_ID_BOOST,
	RK817_ID_BOOST_OTG_SW,
	RK817_NUM_REGULATORS
};
#[repr(u32)]
pub enum rk809_reg_id {
	RK809_ID_DCDC5 = RK817_ID_BOOST,
	RK809_ID_SW1,
	RK809_ID_SW2,
	RK809_NUM_REGULATORS
};
pub const RK817_SECONDS_REG: u32 = 0x00;
pub const RK817_MINUTES_REG: u32 = 0x01;
pub const RK817_HOURS_REG: u32 = 0x02;
pub const RK817_DAYS_REG: u32 = 0x03;
pub const RK817_MONTHS_REG: u32 = 0x04;
pub const RK817_YEARS_REG: u32 = 0x05;
pub const RK817_WEEKS_REG: u32 = 0x06;
pub const RK817_ALARM_SECONDS_REG: u32 = 0x07;
pub const RK817_ALARM_MINUTES_REG: u32 = 0x08;
pub const RK817_ALARM_HOURS_REG: u32 = 0x09;
pub const RK817_ALARM_DAYS_REG: u32 = 0x0a;
pub const RK817_ALARM_MONTHS_REG: u32 = 0x0b;
pub const RK817_ALARM_YEARS_REG: u32 = 0x0c;
pub const RK817_RTC_CTRL_REG: u32 = 0xd;
pub const RK817_RTC_STATUS_REG: u32 = 0xe;
pub const RK817_RTC_INT_REG: u32 = 0xf;
pub const RK817_RTC_COMP_LSB_REG: u32 = 0x10;
pub const RK817_RTC_COMP_MSB_REG: u32 = 0x11;
pub const RK817_CODEC_DTOP_VUCTL: u32 = 0x12;
pub const RK817_CODEC_DTOP_VUCTIME: u32 = 0x13;
pub const RK817_CODEC_DTOP_LPT_SRST: u32 = 0x14;
pub const RK817_CODEC_DTOP_DIGEN_CLKE: u32 = 0x15;
pub const RK817_CODEC_AREF_RTCFG0: u32 = 0x16;
pub const RK817_CODEC_AREF_RTCFG1: u32 = 0x17;
pub const RK817_CODEC_AADC_CFG0: u32 = 0x18;
pub const RK817_CODEC_AADC_CFG1: u32 = 0x19;
pub const RK817_CODEC_DADC_VOLL: u32 = 0x1a;
pub const RK817_CODEC_DADC_VOLR: u32 = 0x1b;
pub const RK817_CODEC_DADC_SR_ACL0: u32 = 0x1e;
pub const RK817_CODEC_DADC_ALC1: u32 = 0x1f;
pub const RK817_CODEC_DADC_ALC2: u32 = 0x20;
pub const RK817_CODEC_DADC_NG: u32 = 0x21;
pub const RK817_CODEC_DADC_HPF: u32 = 0x22;
pub const RK817_CODEC_DADC_RVOLL: u32 = 0x23;
pub const RK817_CODEC_DADC_RVOLR: u32 = 0x24;
pub const RK817_CODEC_AMIC_CFG0: u32 = 0x27;
pub const RK817_CODEC_AMIC_CFG1: u32 = 0x28;
pub const RK817_CODEC_DMIC_PGA_GAIN: u32 = 0x29;
pub const RK817_CODEC_DMIC_LMT1: u32 = 0x2a;
pub const RK817_CODEC_DMIC_LMT2: u32 = 0x2b;
pub const RK817_CODEC_DMIC_NG1: u32 = 0x2c;
pub const RK817_CODEC_DMIC_NG2: u32 = 0x2d;
pub const RK817_CODEC_ADAC_CFG0: u32 = 0x2e;
pub const RK817_CODEC_ADAC_CFG1: u32 = 0x2f;
pub const RK817_CODEC_DDAC_POPD_DACST: u32 = 0x30;
pub const RK817_CODEC_DDAC_VOLL: u32 = 0x31;
pub const RK817_CODEC_DDAC_VOLR: u32 = 0x32;
pub const RK817_CODEC_DDAC_SR_LMT0: u32 = 0x35;
pub const RK817_CODEC_DDAC_LMT1: u32 = 0x36;
pub const RK817_CODEC_DDAC_LMT2: u32 = 0x37;
pub const RK817_CODEC_DDAC_MUTE_MIXCTL: u32 = 0x38;
pub const RK817_CODEC_DDAC_RVOLL: u32 = 0x39;
pub const RK817_CODEC_DDAC_RVOLR: u32 = 0x3a;
pub const RK817_CODEC_AHP_ANTI0: u32 = 0x3b;
pub const RK817_CODEC_AHP_ANTI1: u32 = 0x3c;
pub const RK817_CODEC_AHP_CFG0: u32 = 0x3d;
pub const RK817_CODEC_AHP_CFG1: u32 = 0x3e;
pub const RK817_CODEC_AHP_CP: u32 = 0x3f;
pub const RK817_CODEC_ACLASSD_CFG1: u32 = 0x40;
pub const RK817_CODEC_ACLASSD_CFG2: u32 = 0x41;
pub const RK817_CODEC_APLL_CFG0: u32 = 0x42;
pub const RK817_CODEC_APLL_CFG1: u32 = 0x43;
pub const RK817_CODEC_APLL_CFG2: u32 = 0x44;
pub const RK817_CODEC_APLL_CFG3: u32 = 0x45;
pub const RK817_CODEC_APLL_CFG4: u32 = 0x46;
pub const RK817_CODEC_APLL_CFG5: u32 = 0x47;
pub const RK817_CODEC_DI2S_CKM: u32 = 0x48;
pub const RK817_CODEC_DI2S_RSD: u32 = 0x49;
pub const RK817_CODEC_DI2S_RXCR1: u32 = 0x4a;
pub const RK817_CODEC_DI2S_RXCR2: u32 = 0x4b;
pub const RK817_CODEC_DI2S_RXCMD_TSD: u32 = 0x4c;
pub const RK817_CODEC_DI2S_TXCR1: u32 = 0x4d;
pub const RK817_CODEC_DI2S_TXCR2: u32 = 0x4e;
pub const RK817_CODEC_DI2S_TXCR3_TXCMD: u32 = 0x4f;
pub const RK817_I2S_MODE_MASK: u32 = ((0x1 << 0);
pub const RK817_I2S_MODE_MST: u32 = ((0x1 << 0);
pub const RK817_I2S_MODE_SLV: u32 = ((0x0 << 0);
pub const DACMT_MASK: u32 = ((0x1 << 0);
pub const DACMT_ENABLE: u32 = ((0x1 << 0);
pub const DACMT_DISABLE: u32 = ((0x0 << 0);
pub const VDW_RX_24BITS: u32 = (0x17);
pub const VDW_RX_16BITS: u32 = (0x0f);
pub const VDW_TX_24BITS: u32 = (0x17);
pub const VDW_TX_16BITS: u32 = (0x0f);
pub const MIC_DIFF_MASK: u32 = ((0x1 << 7);
pub const MIC_DIFF_DIS: u32 = ((0x0 << 7);
pub const MIC_DIFF_EN: u32 = ((0x1 << 7);
pub const RK817_GAS_GAUGE_ADC_CONFIG0: u32 = 0x50;
pub const RK817_GG_EN: u32 = ((0x1 << 7);
pub const RK817_SYS_VOL_ADC_EN: u32 = ((0x1 << 6);
pub const RK817_TS_ADC_EN: u32 = ((0x1 << 5);
pub const RK817_USB_VOL_ADC_EN: u32 = ((0x1 << 4);
pub const RK817_BAT_VOL_ADC_EN: u32 = ((0x1 << 3);
pub const RK817_BAT_CUR_ADC_EN: u32 = ((0x1 << 2);
pub const RK817_GAS_GAUGE_ADC_CONFIG1: u32 = 0x55;
pub const RK817_VOL_CUR_CALIB_UPD: u32 = ((1u32) << 7);
pub const RK817_GAS_GAUGE_GG_CON: u32 = 0x56;
pub const RK817_GAS_GAUGE_GG_STS: u32 = 0x57;
pub const RK817_BAT_CON: u32 = ((0x1 << 4);
pub const RK817_RELAX_VOL_UPD: u32 = ((0x3 << 2);
pub const RK817_RELAX_STS: u32 = ((0x1 << 1);
pub const RK817_GAS_GAUGE_RELAX_THRE_H: u32 = 0x58;
pub const RK817_GAS_GAUGE_RELAX_THRE_L: u32 = 0x59;
pub const RK817_GAS_GAUGE_OCV_THRE_VOL: u32 = 0x62;
pub const RK817_GAS_GAUGE_OCV_VOL_H: u32 = 0x63;
pub const RK817_GAS_GAUGE_OCV_VOL_L: u32 = 0x64;
pub const RK817_GAS_GAUGE_PWRON_VOL_H: u32 = 0x6b;
pub const RK817_GAS_GAUGE_PWRON_VOL_L: u32 = 0x6c;
pub const RK817_GAS_GAUGE_PWRON_CUR_H: u32 = 0x6d;
pub const RK817_GAS_GAUGE_PWRON_CUR_L: u32 = 0x6e;
pub const RK817_GAS_GAUGE_OFF_CNT: u32 = 0x6f;
pub const RK817_GAS_GAUGE_Q_INIT_H3: u32 = 0x70;
pub const RK817_GAS_GAUGE_Q_INIT_H2: u32 = 0x71;
pub const RK817_GAS_GAUGE_Q_INIT_L1: u32 = 0x72;
pub const RK817_GAS_GAUGE_Q_INIT_L0: u32 = 0x73;
pub const RK817_GAS_GAUGE_Q_PRES_H3: u32 = 0x74;
pub const RK817_GAS_GAUGE_Q_PRES_H2: u32 = 0x75;
pub const RK817_GAS_GAUGE_Q_PRES_L1: u32 = 0x76;
pub const RK817_GAS_GAUGE_Q_PRES_L0: u32 = 0x77;
pub const RK817_GAS_GAUGE_BAT_VOL_H: u32 = 0x78;
pub const RK817_GAS_GAUGE_BAT_VOL_L: u32 = 0x79;
pub const RK817_GAS_GAUGE_BAT_CUR_H: u32 = 0x7a;
pub const RK817_GAS_GAUGE_BAT_CUR_L: u32 = 0x7b;
pub const RK817_GAS_GAUGE_USB_VOL_H: u32 = 0x7e;
pub const RK817_GAS_GAUGE_USB_VOL_L: u32 = 0x7f;
pub const RK817_GAS_GAUGE_SYS_VOL_H: u32 = 0x80;
pub const RK817_GAS_GAUGE_SYS_VOL_L: u32 = 0x81;
pub const RK817_GAS_GAUGE_Q_MAX_H3: u32 = 0x82;
pub const RK817_GAS_GAUGE_Q_MAX_H2: u32 = 0x83;
pub const RK817_GAS_GAUGE_Q_MAX_L1: u32 = 0x84;
pub const RK817_GAS_GAUGE_Q_MAX_L0: u32 = 0x85;
pub const RK817_GAS_GAUGE_SLEEP_CON_SAMP_CUR_H: u32 = 0x8f;
pub const RK817_GAS_GAUGE_SLEEP_CON_SAMP_CUR_L: u32 = 0x90;
pub const RK817_GAS_GAUGE_CAL_OFFSET_H: u32 = 0x91;
pub const RK817_GAS_GAUGE_CAL_OFFSET_L: u32 = 0x92;
pub const RK817_GAS_GAUGE_VCALIB0_H: u32 = 0x93;
pub const RK817_GAS_GAUGE_VCALIB0_L: u32 = 0x94;
pub const RK817_GAS_GAUGE_VCALIB1_H: u32 = 0x95;
pub const RK817_GAS_GAUGE_VCALIB1_L: u32 = 0x96;
pub const RK817_GAS_GAUGE_IOFFSET_H: u32 = 0x97;
pub const RK817_GAS_GAUGE_IOFFSET_L: u32 = 0x98;
pub const RK817_GAS_GAUGE_BAT_R1: u32 = 0x9a;
pub const RK817_GAS_GAUGE_BAT_R2: u32 = 0x9b;
pub const RK817_GAS_GAUGE_BAT_R3: u32 = 0x9c;
pub const RK817_GAS_GAUGE_DATA0: u32 = 0x9d;
pub const RK817_GAS_GAUGE_DATA1: u32 = 0x9e;
pub const RK817_GAS_GAUGE_DATA2: u32 = 0x9f;
pub const RK817_GAS_GAUGE_DATA3: u32 = 0xa0;
pub const RK817_GAS_GAUGE_DATA4: u32 = 0xa1;
pub const RK817_GAS_GAUGE_DATA5: u32 = 0xa2;
pub const RK817_GAS_GAUGE_CUR_ADC_K0: u32 = 0xb0;
pub const fn RK817_POWER_EN_REG(i: u32) -> u32 { (0xb1 + (i)) }
pub const fn RK817_POWER_SLP_EN_REG(i: u32) -> u32 { (0xb5 + (i)) }

pub const RK817_POWER_CONFIG: u32 = (0xb9);
pub const fn RK817_BUCK_CONFIG_REG(i: u32) -> u32 { (0xba + (i) * 3) }

pub const RK817_BUCK1_ON_VSEL_REG: u32 = 0xBB;
pub const RK817_BUCK1_SLP_VSEL_REG: u32 = 0xBC;
pub const RK817_BUCK2_CONFIG_REG: u32 = 0xBD;
pub const RK817_BUCK2_ON_VSEL_REG: u32 = 0xBE;
pub const RK817_BUCK2_SLP_VSEL_REG: u32 = 0xBF;
pub const RK817_BUCK3_CONFIG_REG: u32 = 0xC0;
pub const RK817_BUCK3_ON_VSEL_REG: u32 = 0xC1;
pub const RK817_BUCK3_SLP_VSEL_REG: u32 = 0xC2;
pub const RK817_BUCK4_CONFIG_REG: u32 = 0xC3;
pub const RK817_BUCK4_ON_VSEL_REG: u32 = 0xC4;
pub const RK817_BUCK4_SLP_VSEL_REG: u32 = 0xC5;
pub const fn RK817_LDO_ON_VSEL_REG(idx: u32) -> u32 { (0xcc + (idx) * 2) }
pub const RK817_BOOST_OTG_CFG: u32 = (0xde);
pub const RK817_PMIC_CHRG_OUT: u32 = 0xe4;
pub const RK817_CHRG_VOL_SEL: u32 = ((0x07 << 4);
pub const RK817_CHRG_CUR_SEL: u32 = ((0x07 << 0);
pub const RK817_PMIC_CHRG_IN: u32 = 0xe5;
pub const RK817_USB_VLIM_EN: u32 = ((0x01 << 7);
pub const RK817_USB_VLIM_SEL: u32 = ((0x07 << 4);
pub const RK817_USB_ILIM_EN: u32 = ((0x01 << 3);
pub const RK817_USB_ILIM_SEL: u32 = ((0x07 << 0);
pub const RK817_PMIC_CHRG_TERM: u32 = 0xe6;
pub const RK817_CHRG_TERM_ANA_DIG: u32 = ((0x01 << 2);
pub const RK817_CHRG_TERM_ANA_SEL: u32 = ((0x03 << 0);
pub const RK817_CHRG_EN: u32 = ((0x01 << 6);
pub const RK817_PMIC_CHRG_STS: u32 = 0xeb;
pub const RK817_BAT_EXS: u32 = ((1u32) << 7);
pub const RK817_CHG_STS: u32 = ((0x07 << 4);
pub const RK817_ID_MSB: u32 = 0xed;
pub const RK817_ID_LSB: u32 = 0xee;
pub const RK817_SYS_STS: u32 = 0xf0;
pub const RK817_PLUG_IN_STS: u32 = ((0x1 << 6);
pub const fn RK817_SYS_CFG(i: u32) -> u32 { (0xf1 + (i)) }

pub const RK817_ON_SOURCE_REG: u32 = 0xf5;
pub const RK817_OFF_SOURCE_REG: u32 = 0xf6;
pub const RK817_INT_STS_REG0: u32 = 0xf8;
pub const RK817_INT_STS_MSK_REG0: u32 = 0xf9;
pub const RK817_INT_STS_REG1: u32 = 0xfa;
pub const RK817_INT_STS_MSK_REG1: u32 = 0xfb;
pub const RK817_INT_STS_REG2: u32 = 0xfc;
pub const RK817_INT_STS_MSK_REG2: u32 = 0xfd;
pub const RK817_GPIO_INT_CFG: u32 = 0xfe;
pub const RK817_IRQ_PWRON_FALL: u32 = 0;
pub const RK817_IRQ_PWRON_RISE: u32 = 1;
pub const RK817_IRQ_PWRON: u32 = 2;
pub const RK817_IRQ_PWMON_LP: u32 = 3;
pub const RK817_IRQ_HOTDIE: u32 = 4;
pub const RK817_IRQ_RTC_ALARM: u32 = 5;
pub const RK817_IRQ_RTC_PERIOD: u32 = 6;
pub const RK817_IRQ_VB_LO: u32 = 7;
pub const RK817_IRQ_PLUG_IN: u32 = 8;
pub const RK817_IRQ_PLUG_OUT: u32 = 9;
pub const RK817_IRQ_CHRG_TERM: u32 = 10;
pub const RK817_IRQ_CHRG_TIME: u32 = 11;
pub const RK817_IRQ_CHRG_TS: u32 = 12;
pub const RK817_IRQ_USB_OV: u32 = 13;
pub const RK817_IRQ_CHRG_IN_CLMP: u32 = 14;
pub const RK817_IRQ_BAT_DIS_ILIM: u32 = 15;
pub const RK817_IRQ_GATE_GPIO: u32 = 16;
pub const RK817_IRQ_TS_GPIO: u32 = 17;
pub const RK817_IRQ_CODEC_PD: u32 = 18;
pub const RK817_IRQ_CODEC_PO: u32 = 19;
pub const RK817_IRQ_CLASSD_MUTE_DONE: u32 = 20;
pub const RK817_IRQ_CLASSD_OCP: u32 = 21;
pub const RK817_IRQ_BAT_OVP: u32 = 22;
pub const RK817_IRQ_CHRG_BAT_HI: u32 = 23;
pub const RK817_IRQ_END: u32 = (RK817_IRQ_CHRG_BAT_HI + 1);
/*
 * rtc_ctrl 0xd
 * same as 808, except bit4
 */
pub const RK817_RTC_CTRL_RSV4: u32 = ((1u32) << 4);
pub const RK817_BUCK3_FB_RES_MSK: u32 = ((1u32) << 6);
pub const RK817_BUCK3_FB_RES_INTER: u32 = ((1u32) << 6);
pub const RK817_BUCK3_FB_RES_EXT: u32 = 0;
pub const RK817_RAMP_RATE_OFFSET: u32 = 6;
pub const RK817_RAMP_RATE_MASK: u32 = ((0x3 << RK817_RAMP_RATE_OFFSET);
pub const RK817_RAMP_RATE_3MV_PER_US: u32 = ((0x0 << RK817_RAMP_RATE_OFFSET);
pub const RK817_RAMP_RATE_6_3MV_PER_US: u32 = ((0x1 << RK817_RAMP_RATE_OFFSET);
pub const RK817_RAMP_RATE_12_5MV_PER_US: u32 = ((0x2 << RK817_RAMP_RATE_OFFSET);
pub const RK817_RAMP_RATE_25MV_PER_US: u32 = ((0x3 << RK817_RAMP_RATE_OFFSET);
pub const RK817_HOTDIE_TEMP_MSK: u32 = ((0x3 << 4);
pub const RK817_HOTDIE_85: u32 = ((0x0 << 4);
pub const RK817_HOTDIE_95: u32 = ((0x1 << 4);
pub const RK817_HOTDIE_105: u32 = ((0x2 << 4);
pub const RK817_HOTDIE_115: u32 = ((0x3 << 4);
pub const RK817_TSD_TEMP_MSK: u32 = ((1u32) << 6);
pub const RK817_TSD_140: u32 = 0;
pub const RK817_TSD_160: u32 = ((1u32) << 6);
pub const RK817_CLK32KOUT2_EN: u32 = ((1u32) << 7);
pub const RK817_SLPPIN_FUNC_MSK: u32 = ((0x3 << 3);
pub const SLPPIN_NULL_FUN: u32 = ((0x0 << 3);
pub const SLPPIN_SLP_FUN: u32 = ((0x1 << 3);
pub const SLPPIN_DN_FUN: u32 = ((0x2 << 3);
pub const SLPPIN_RST_FUN: u32 = ((0x3 << 3);
pub const RK817_RST_FUNC_MSK: u32 = ((0x3 << 6);
pub const RK817_RST_FUNC_SFT: u32 = (6);
pub const RK817_RST_FUNC_CNT: u32 = (3);
pub const RK817_RST_FUNC_DEV: u32 = (0) /* reset the dev */;
pub const RK817_RST_FUNC_REG: u32 = ((0x1 << 6) /* reset the reg only */;
pub const RK817_SLPPOL_MSK: u32 = ((1u32) << 5);
pub const RK817_SLPPOL_H: u32 = ((1u32) << 5);
pub const RK817_SLPPOL_L: u32 = (0);
pub const RK817_INT_POL_MSK: u32 = ((1u32) << 1);
pub const RK817_INT_POL_H: u32 = ((1u32) << 1);
pub const RK817_INT_POL_L: u32 = 0;
pub const fn RK809_BUCK5_CONFIG(i: u32) -> u32 { (RK817_BOOST_OTG_CFG + (i) * 1) }

enum {
	BUCK_ILMIN_50MA,
	BUCK_ILMIN_100MA,
	BUCK_ILMIN_150MA,
	BUCK_ILMIN_200MA,
	BUCK_ILMIN_250MA,
	BUCK_ILMIN_300MA,
	BUCK_ILMIN_350MA,
	BUCK_ILMIN_400MA,
};
enum {
	BOOST_ILMIN_75MA,
	BOOST_ILMIN_100MA,
	BOOST_ILMIN_125MA,
	BOOST_ILMIN_150MA,
	BOOST_ILMIN_175MA,
	BOOST_ILMIN_200MA,
	BOOST_ILMIN_225MA,
	BOOST_ILMIN_250MA,
};
enum {
	RK805_BUCK1_2_ILMAX_2500MA,
	RK805_BUCK1_2_ILMAX_3000MA,
	RK805_BUCK1_2_ILMAX_3500MA,
	RK805_BUCK1_2_ILMAX_4000MA,
};
enum {
	RK805_BUCK3_ILMAX_1500MA,
	RK805_BUCK3_ILMAX_2000MA,
	RK805_BUCK3_ILMAX_2500MA,
	RK805_BUCK3_ILMAX_3000MA,
};
enum {
	RK805_BUCK4_ILMAX_2000MA,
	RK805_BUCK4_ILMAX_2500MA,
	RK805_BUCK4_ILMAX_3000MA,
	RK805_BUCK4_ILMAX_3500MA,
};
enum {
	RK801_ID = 0x8010,
	RK805_ID = 0x8050,
	RK806_ID = 0x8060,
	RK808_ID = 0x0000,
	RK809_ID = 0x8090,
	RK816_ID = 0x8160,
	RK817_ID = 0x8170,
	RK818_ID = 0x8180,
};
struct rk808 {
	struct device			*dev;
	struct regmap_irq_chip_data	*irq_data;
	struct regmap			*regmap;
	long				variant;
	const struct regmap_config	*regmap_cfg;
	const struct regmap_irq_chip	*regmap_irq_chip;
};
void rk8xx_shutdown(struct device *dev);
int rk8xx_probe(struct device *dev, int variant, unsigned int irq, struct regmap *regmap);
int rk8xx_suspend(struct device *dev);
int rk8xx_resume(struct device *dev);
#endif /* __LINUX_REGULATOR_RK808_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
