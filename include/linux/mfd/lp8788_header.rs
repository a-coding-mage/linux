/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI LP8788 MFD Device
 *
 * Copyright 2012 Texas Instruments
 *
 * Author: Milo(Woogyom) Kim <milo.kim@ti.com>
 */

// Dependencies supplied by other translation units.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_map {
    _private: [u8; 0],
}

pub const LP8788_DEV_BUCK: &str = "lp8788-buck";
pub const LP8788_DEV_DLDO: &str = "lp8788-dldo";
pub const LP8788_DEV_ALDO: &str = "lp8788-aldo";
pub const LP8788_DEV_CHARGER: &str = "lp8788-charger";
pub const LP8788_DEV_RTC: &str = "lp8788-rtc";
pub const LP8788_DEV_BACKLIGHT: &str = "lp8788-backlight";
pub const LP8788_DEV_VIBRATOR: &str = "lp8788-vibrator";
pub const LP8788_DEV_KEYLED: &str = "lp8788-keyled";
pub const LP8788_DEV_ADC: &str = "lp8788-adc";

pub const LP8788_NUM_BUCKS: usize = 4;
pub const LP8788_NUM_DLDOS: usize = 12;
pub const LP8788_NUM_ALDOS: usize = 10;
pub const LP8788_NUM_BUCK2_DVS: usize = 2;

pub const LP8788_CHG_IRQ: &str = "CHG_IRQ";
pub const LP8788_PRSW_IRQ: &str = "PRSW_IRQ";
pub const LP8788_BATT_IRQ: &str = "BATT_IRQ";
pub const LP8788_ALM_IRQ: &str = "ALARM_IRQ";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_int_id {
    LP8788_INT_TSDL,
    LP8788_INT_TSDH,
    LP8788_INT_UVLO,
    LP8788_INT_FLAGMON,
    LP8788_INT_PWRON_TIME,
    LP8788_INT_PWRON,
    LP8788_INT_COMP1,
    LP8788_INT_COMP2,
    LP8788_INT_CHG_INPUT_STATE,
    LP8788_INT_CHG_STATE,
    LP8788_INT_EOC,
    LP8788_INT_CHG_RESTART,
    LP8788_INT_RESTART_TIMEOUT,
    LP8788_INT_FULLCHG_TIMEOUT,
    LP8788_INT_PRECHG_TIMEOUT,
    LP8788_INT_RTC_ALARM1 = 17,
    LP8788_INT_RTC_ALARM2,
    LP8788_INT_ENTER_SYS_SUPPORT,
    LP8788_INT_EXIT_SYS_SUPPORT,
    LP8788_INT_BATT_LOW,
    LP8788_INT_NO_BATT,
    LP8788_INT_MAX = 24,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_dvs_sel { DVS_SEL_V0, DVS_SEL_V1, DVS_SEL_V2, DVS_SEL_V3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_ext_ldo_en_id { EN_ALDO1, EN_ALDO234, EN_ALDO5, EN_ALDO7, EN_DLDO7, EN_DLDO911, EN_LDOS_MAX }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_charger_event { NO_CHARGER, CHARGER_DETECTED }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_bl_dim_mode { LP8788_DIM_EXPONENTIAL, LP8788_DIM_LINEAR }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_bl_full_scale_current {
    LP8788_FULLSCALE_5000uA, LP8788_FULLSCALE_8500uA, LP8788_FULLSCALE_1200uA, LP8788_FULLSCALE_1550uA,
    LP8788_FULLSCALE_1900uA, LP8788_FULLSCALE_2250uA, LP8788_FULLSCALE_2600uA, LP8788_FULLSCALE_2950uA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_bl_ramp_step {
    LP8788_RAMP_8us, LP8788_RAMP_1024us, LP8788_RAMP_2048us, LP8788_RAMP_4096us,
    LP8788_RAMP_8192us, LP8788_RAMP_16384us, LP8788_RAMP_32768us, LP8788_RAMP_65538us,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_isink_scale { LP8788_ISINK_SCALE_100mA, LP8788_ISINK_SCALE_120mA }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_isink_number { LP8788_ISINK_1, LP8788_ISINK_2, LP8788_ISINK_3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_alarm_sel { LP8788_ALARM_1, LP8788_ALARM_2, LP8788_ALARM_MAX }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp8788_adc_id {
    LPADC_VBATT_5P5, LPADC_VIN_CHG, LPADC_IBATT, LPADC_IC_TEMP, LPADC_VBATT_6P0, LPADC_VBATT_5P0,
    LPADC_ADC1, LPADC_ADC2, LPADC_VDD, LPADC_VCOIN, LPADC_VDD_LDO, LPADC_ADC3, LPADC_ADC4, LPADC_MAX,
}

#[repr(C)]
pub struct lp8788_buck1_dvs { pub vsel: lp8788_dvs_sel }
#[repr(C)]
pub struct lp8788_buck2_dvs { pub vsel: lp8788_dvs_sel }
#[repr(C)]
pub struct lp8788_chg_param { pub addr: u8, pub val: u8 }

#[repr(C)]
pub struct lp8788_charger_platform_data {
    pub adc_vbatt: *const core::ffi::c_char,
    pub adc_batt_temp: *const core::ffi::c_char,
    pub max_vbatt_mv: core::ffi::c_uint,
    pub chg_params: *mut lp8788_chg_param,
    pub num_chg_params: core::ffi::c_int,
    pub charger_event: Option<unsafe extern "C" fn(*mut lp8788, lp8788_charger_event)>,
}

#[repr(C)]
pub struct lp8788_led_platform_data {
    pub name: *mut core::ffi::c_char,
    pub scale: lp8788_isink_scale,
    pub num: lp8788_isink_number,
    pub iout_code: core::ffi::c_int,
}

#[repr(C)]
pub struct lp8788_vib_platform_data {
    pub name: *mut core::ffi::c_char,
    pub scale: lp8788_isink_scale,
    pub num: lp8788_isink_number,
    pub iout_code: core::ffi::c_int,
    pub pwm_code: core::ffi::c_int,
}

#[repr(C)]
pub struct lp8788_platform_data {
    pub init_func: Option<unsafe extern "C" fn(*mut lp8788) -> core::ffi::c_int>,
    pub buck_data: [*mut regulator_init_data; LP8788_NUM_BUCKS],
    pub dldo_data: [*mut regulator_init_data; LP8788_NUM_DLDOS],
    pub aldo_data: [*mut regulator_init_data; LP8788_NUM_ALDOS],
    pub buck1_dvs: *mut lp8788_buck1_dvs,
    pub buck2_dvs: *mut lp8788_buck2_dvs,
    pub chg_pdata: *mut lp8788_charger_platform_data,
    pub alarm_sel: lp8788_alarm_sel,
    pub led_pdata: *mut lp8788_led_platform_data,
    pub vib_pdata: *mut lp8788_vib_platform_data,
    pub adc_pdata: *mut iio_map,
}

#[repr(C)]
pub struct lp8788 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irqdm: *mut irq_domain,
    pub irq: core::ffi::c_int,
    pub pdata: *mut lp8788_platform_data,
}

extern "C" {
    pub fn lp8788_irq_init(lp: *mut lp8788, chip_irq: core::ffi::c_int) -> core::ffi::c_int;
    pub fn lp8788_irq_exit(lp: *mut lp8788);
    pub fn lp8788_read_byte(lp: *mut lp8788, reg: u8, data: *mut u8) -> core::ffi::c_int;
    pub fn lp8788_read_multi_bytes(lp: *mut lp8788, reg: u8, data: *mut u8, count: usize) -> core::ffi::c_int;
    pub fn lp8788_write_byte(lp: *mut lp8788, reg: u8, data: u8) -> core::ffi::c_int;
    pub fn lp8788_update_bits(lp: *mut lp8788, reg: u8, mask: u8, data: u8) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
