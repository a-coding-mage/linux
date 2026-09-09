/* SPDX-License-Identifier: GPL-2.0-or-later */
/* TI Palmas */
/* Copyright 2011-2013 Texas Instruments Inc. */
/* Authors: Graeme Gregory <gg@slimlogic.co.uk>, Ian Lartey <ian@slimlogic.co.uk> */

use core::ffi::c_char;

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct regmap_irq_chip_data { _private: [u8; 0] }
#[repr(C)] pub struct of_regulator_match { _private: [u8; 0] }
#[repr(C)] pub struct regulator_init_data { _private: [u8; 0] }
#[repr(C)] pub struct regulator_config { _private: [u8; 0] }

pub const PALMAS_NUM_CLIENTS: usize = 3;
pub const PALMAS_CHIP_OLD_ID: i32 = 0x0000;
pub const PALMAS_CHIP_ID: i32 = 0xC035;
pub const PALMAS_CHIP_CHARGER_ID: i32 = 0xC036;
pub const TPS65917_RESERVED: i32 = -1;

#[inline] pub const fn is_palmas(a: i32) -> bool { a == PALMAS_CHIP_OLD_ID || a == PALMAS_CHIP_ID }
#[inline] pub const fn is_palmas_charger(a: i32) -> bool { a == PALMAS_CHIP_CHARGER_ID }

pub const PALMAS_PMIC_FEATURE_SMPS10_BOOST: u32 = 1 << 0;
#[macro_export]
macro_rules! PALMAS_PMIC_HAS { ($b:expr, $f:ident) => { ($b).features & $crate::PALMAS_PMIC_FEATURE_SMPS10_BOOST }; }
pub const PALMAS_EXT_REQ: u32 = PALMAS_EXT_CONTROL_ENABLE1 | PALMAS_EXT_CONTROL_ENABLE2 | PALMAS_EXT_CONTROL_NSLEEP;

pub struct palmas_pmic; pub struct palmas_gpadc; pub struct palmas_resource; pub struct palmas_usb;
pub struct palmas_pmic_driver_data; pub struct palmas_pmic_platform_data;

#[repr(C)] pub enum palmas_usb_state { PALMAS_USB_STATE_DISCONNECT, PALMAS_USB_STATE_VBUS, PALMAS_USB_STATE_ID }

#[repr(C)]
pub struct palmas {
    pub dev: *mut device,
    pub i2c_clients: [*mut i2c_client; PALMAS_NUM_CLIENTS],
    pub regmap: [*mut regmap; PALMAS_NUM_CLIENTS],
    pub id: i32,
    pub features: u32,
    pub irq: i32,
    pub irq_mask: u32,
    pub irq_lock: mutex,
    pub irq_data: *mut regmap_irq_chip_data,
    pub pmic_ddata: *mut palmas_pmic_driver_data,
    pub pmic: *mut palmas_pmic,
    pub gpadc: *mut palmas_gpadc,
    pub resource: *mut palmas_resource,
    pub usb: *mut palmas_usb,
    pub gpio_muxed: u8, pub led_muxed: u8, pub pwm_muxed: u8,
}

#[repr(C)] pub struct palmas_sleep_requestor_info { pub id: i32, pub reg_offset: i32, pub bit_pos: i32 }
#[repr(C)] pub struct palmas_regs_info { pub name: *const c_char, pub sname: *const c_char, pub vsel_addr: u8, pub ctrl_addr: u8, pub tstep_addr: u8, pub sleep_id: i32 }

#[repr(C)] pub struct palmas_pmic_driver_data {
    pub smps_start: i32, pub smps_end: i32, pub ldo_begin: i32, pub ldo_end: i32, pub max_reg: i32, pub has_regen3: bool,
    pub palmas_regs_info: *mut palmas_regs_info, pub palmas_matches: *mut of_regulator_match, pub sleep_req_info: *mut palmas_sleep_requestor_info,
    pub smps_register: Option<unsafe extern "C" fn(*mut palmas_pmic, *mut palmas_pmic_driver_data, *mut palmas_pmic_platform_data, *const c_char, regulator_config)>,
    pub ldo_register: Option<unsafe extern "C" fn(*mut palmas_pmic, *mut palmas_pmic_driver_data, *mut palmas_pmic_platform_data, *const c_char, regulator_config)>,
}

#[repr(C)] pub struct palmas_gpadc_platform_data { pub ch3_current: i32, pub ch0_current: i32, pub extended_delay: bool, pub bat_removal: i32, pub start_polarity: i32, pub auto_conversion_period_ms: i32 }
#[repr(C)] pub struct palmas_reg_init { pub warm_reset: i32, pub roof_floor: i32, pub mode_sleep: i32, pub vsel: u8 }

#[repr(C)] pub enum palmas_regulators {
    PALMAS_REG_SMPS12, PALMAS_REG_SMPS123, PALMAS_REG_SMPS3, PALMAS_REG_SMPS45, PALMAS_REG_SMPS457, PALMAS_REG_SMPS6, PALMAS_REG_SMPS7, PALMAS_REG_SMPS8, PALMAS_REG_SMPS9, PALMAS_REG_SMPS10_OUT2, PALMAS_REG_SMPS10_OUT1,
    PALMAS_REG_LDO1, PALMAS_REG_LDO2, PALMAS_REG_LDO3, PALMAS_REG_LDO4, PALMAS_REG_LDO5, PALMAS_REG_LDO6, PALMAS_REG_LDO7, PALMAS_REG_LDO8, PALMAS_REG_LDO9, PALMAS_REG_LDOLN, PALMAS_REG_LDOUSB,
    PALMAS_REG_REGEN1, PALMAS_REG_REGEN2, PALMAS_REG_REGEN3, PALMAS_REG_SYSEN1, PALMAS_REG_SYSEN2, PALMAS_NUM_REGS,
}
#[repr(C)] pub enum tps65917_regulators { TPS65917_REG_SMPS1, TPS65917_REG_SMPS2, TPS65917_REG_SMPS3, TPS65917_REG_SMPS4, TPS65917_REG_SMPS5, TPS65917_REG_SMPS12, TPS65917_REG_LDO1, TPS65917_REG_LDO2, TPS65917_REG_LDO3, TPS65917_REG_LDO4, TPS65917_REG_LDO5, TPS65917_REG_REGEN1, TPS65917_REG_REGEN2, TPS65917_REG_REGEN3, TPS65917_NUM_REGS }

pub const PALMAS_EXT_CONTROL_ENABLE1: u32 = 0x1; pub const PALMAS_EXT_CONTROL_ENABLE2: u32 = 0x2; pub const PALMAS_EXT_CONTROL_NSLEEP: u32 = 0x4;

#[repr(C)] pub struct palmas_pmic_platform_data { pub reg_data: [*mut regulator_init_data; palmas_regulators::PALMAS_NUM_REGS as usize], pub reg_init: [*mut palmas_reg_init; palmas_regulators::PALMAS_NUM_REGS as usize], pub ldo6_vibrator: i32, pub enable_ldo8_tracking: bool }
#[repr(C)] pub struct palmas_usb_platform_data { pub wakeup: i32 }
#[repr(C)] pub struct palmas_resource_platform_data { pub regen1_mode_sleep: i32, pub regen2_mode_sleep: i32, pub sysen1_mode_sleep: i32, pub sysen2_mode_sleep: i32, pub nsleep_res: u8, pub nsleep_smps: u8, pub nsleep_ldo1: u8, pub nsleep_ldo2: u8, pub enable1_res: u8, pub enable1_smps: u8, pub enable1_ldo1: u8, pub enable1_ldo2: u8, pub enable2_res: u8, pub enable2_smps: u8, pub enable2_ldo1: u8, pub enable2_ldo2: u8 }
#[repr(C)] pub struct palmas_clk_platform_data { pub clk32kg_mode_sleep: i32, pub clk32kgaudio_mode_sleep: i32 }
#[repr(C)] pub struct palmas_platform_data { pub irq_flags: i32, pub gpio_base: i32, pub power_ctrl: u8, pub mux_from_pdata: i32, pub pad1: u8, pub pad2: u8, pub pm_off: bool, pub pmic_pdata: *mut palmas_pmic_platform_data, pub gpadc_pdata: *mut palmas_gpadc_platform_data, pub usb_pdata: *mut palmas_usb_platform_data, pub resource_pdata: *mut palmas_resource_platform_data, pub clk_pdata: *mut palmas_clk_platform_data }
#[repr(C)] pub struct palmas_gpadc_calibration { pub gain: i32, pub gain_error: i32, pub offset_error: i32 }
#[macro_export] macro_rules! PALMAS_DATASHEET_NAME { ($name:ident) => { concat!("palmas-gpadc-chan-", stringify!($name)) }; }
#[repr(C)] pub struct palmas_gpadc_result { pub raw_code: i32, pub corrected_code: i32, pub result: i32 }
pub const PALMAS_MAX_CHANNELS: usize = 16;

#[repr(C)] pub enum tps65917_irqs { TPS65917_RESERVED1, TPS65917_PWRON_IRQ, TPS65917_LONG_PRESS_KEY_IRQ, TPS65917_RESERVED2, TPS65917_PWRDOWN_IRQ, TPS65917_HOTDIE_IRQ, TPS65917_VSYS_MON_IRQ, TPS65917_RESERVED3, TPS65917_RESERVED4, TPS65917_OTP_ERROR_IRQ, TPS65917_WDT_IRQ, TPS65917_RESERVED5, TPS65917_RESET_IN_IRQ, TPS65917_FSD_IRQ, TPS65917_SHORT_IRQ, TPS65917_RESERVED6, TPS65917_GPADC_AUTO_0_IRQ, TPS65917_GPADC_AUTO_1_IRQ, TPS65917_GPADC_EOC_SW_IRQ, TPS65917_RESREVED6, TPS65917_RESERVED7, TPS65917_RESERVED8, TPS65917_RESERVED9, TPS65917_VBUS_IRQ, TPS65917_GPIO_0_IRQ, TPS65917_GPIO_1_IRQ, TPS65917_GPIO_2_IRQ, TPS65917_GPIO_3_IRQ, TPS65917_GPIO_4_IRQ, TPS65917_GPIO_5_IRQ, TPS65917_GPIO_6_IRQ, TPS65917_RESERVED10, TPS65917_NUM_IRQ }

#[repr(C)] pub enum palmas_irqs { PALMAS_CHARG_DET_N_VBUS_OVV_IRQ, PALMAS_PWRON_IRQ, PALMAS_LONG_PRESS_KEY_IRQ, PALMAS_RPWRON_IRQ, PALMAS_PWRDOWN_IRQ, PALMAS_HOTDIE_IRQ, PALMAS_VSYS_MON_IRQ, PALMAS_VBAT_MON_IRQ, PALMAS_RTC_ALARM_IRQ, PALMAS_RTC_TIMER_IRQ, PALMAS_WDT_IRQ, PALMAS_BATREMOVAL_IRQ, PALMAS_RESET_IN_IRQ, PALMAS_FBI_BB_IRQ, PALMAS_SHORT_IRQ, PALMAS_VAC_ACOK_IRQ, PALMAS_GPADC_AUTO_0_IRQ, PALMAS_GPADC_AUTO_1_IRQ, PALMAS_GPADC_EOC_SW_IRQ, PALMAS_GPADC_EOC_RT_IRQ, PALMAS_ID_OTG_IRQ, PALMAS_ID_IRQ, PALMAS_VBUS_OTG_IRQ, PALMAS_VBUS_IRQ, PALMAS_GPIO_0_IRQ, PALMAS_GPIO_1_IRQ, PALMAS_GPIO_2_IRQ, PALMAS_GPIO_3_IRQ, PALMAS_GPIO_4_IRQ, PALMAS_GPIO_5_IRQ, PALMAS_GPIO_6_IRQ, PALMAS_GPIO_7_IRQ, PALMAS_NUM_IRQ }

#[repr(C)] pub enum palmas_external_requestor_id { PALMAS_EXTERNAL_REQSTR_ID_REGEN1, PALMAS_EXTERNAL_REQSTR_ID_REGEN2, PALMAS_EXTERNAL_REQSTR_ID_SYSEN1, PALMAS_EXTERNAL_REQSTR_ID_SYSEN2, PALMAS_EXTERNAL_REQSTR_ID_CLK32KG, PALMAS_EXTERNAL_REQSTR_ID_CLK32KGAUDIO, PALMAS_EXTERNAL_REQSTR_ID_REGEN3, PALMAS_EXTERNAL_REQSTR_ID_SMPS12, PALMAS_EXTERNAL_REQSTR_ID_SMPS3, PALMAS_EXTERNAL_REQSTR_ID_SMPS45, PALMAS_EXTERNAL_REQSTR_ID_SMPS6, PALMAS_EXTERNAL_REQSTR_ID_SMPS7, PALMAS_EXTERNAL_REQSTR_ID_SMPS8, PALMAS_EXTERNAL_REQSTR_ID_SMPS9, PALMAS_EXTERNAL_REQSTR_ID_SMPS10, PALMAS_EXTERNAL_REQSTR_ID_LDO1, PALMAS_EXTERNAL_REQSTR_ID_LDO2, PALMAS_EXTERNAL_REQSTR_ID_LDO3, PALMAS_EXTERNAL_REQSTR_ID_LDO4, PALMAS_EXTERNAL_REQSTR_ID_LDO5, PALMAS_EXTERNAL_REQSTR_ID_LDO6, PALMAS_EXTERNAL_REQSTR_ID_LDO7, PALMAS_EXTERNAL_REQSTR_ID_LDO8, PALMAS_EXTERNAL_REQSTR_ID_LDO9, PALMAS_EXTERNAL_REQSTR_ID_LDOLN, PALMAS_EXTERNAL_REQSTR_ID_LDOUSB, PALMAS_EXTERNAL_REQSTR_ID_MAX }
#[repr(C)] pub enum tps65917_external_requestor_id { TPS65917_EXTERNAL_REQSTR_ID_REGEN1, TPS65917_EXTERNAL_REQSTR_ID_REGEN2, TPS65917_EXTERNAL_REQSTR_ID_REGEN3, TPS65917_EXTERNAL_REQSTR_ID_SMPS1, TPS65917_EXTERNAL_REQSTR_ID_SMPS2, TPS65917_EXTERNAL_REQSTR_ID_SMPS3, TPS65917_EXTERNAL_REQSTR_ID_SMPS4, TPS65917_EXTERNAL_REQSTR_ID_SMPS5, TPS65917_EXTERNAL_REQSTR_ID_SMPS12, TPS65917_EXTERNAL_REQSTR_ID_LDO1, TPS65917_EXTERNAL_REQSTR_ID_LDO2, TPS65917_EXTERNAL_REQSTR_ID_LDO3, TPS65917_EXTERNAL_REQSTR_ID_LDO4, TPS65917_EXTERNAL_REQSTR_ID_LDO5, TPS65917_EXTERNAL_REQSTR_ID_MAX }

#[repr(C)] pub enum palmas_adc_channels { PALMAS_ADC_CH_IN0, PALMAS_ADC_CH_IN1, PALMAS_ADC_CH_IN2, PALMAS_ADC_CH_IN3, PALMAS_ADC_CH_IN4, PALMAS_ADC_CH_IN5, PALMAS_ADC_CH_IN6, PALMAS_ADC_CH_IN7, PALMAS_ADC_CH_IN8, PALMAS_ADC_CH_IN9, PALMAS_ADC_CH_IN10, PALMAS_ADC_CH_IN11, PALMAS_ADC_CH_IN12, PALMAS_ADC_CH_IN13, PALMAS_ADC_CH_IN14, PALMAS_ADC_CH_IN15, PALMAS_ADC_CH_MAX }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
