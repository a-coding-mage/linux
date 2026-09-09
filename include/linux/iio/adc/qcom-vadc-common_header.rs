/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Code shared between the different Qualcomm PMIC voltage ADCs
 */

// Translated from qcom-vadc-common.h. Linux dependencies are supplied by the
// surrounding translation unit.

pub const VADC_CONV_TIME_MIN_US: u32 = 2000;
pub const VADC_CONV_TIME_MAX_US: u32 = 2100;

/* Min ADC code represents 0V */
pub const VADC_MIN_ADC_CODE: u32 = 0x6000;
/* Max ADC code represents full-scale range of 1.8V */
pub const VADC_MAX_ADC_CODE: u32 = 0xa800;

pub const VADC_ABSOLUTE_RANGE_UV: u32 = 625000;
pub const VADC_RATIOMETRIC_RANGE: u32 = 1800;

pub const VADC_DEF_PRESCALING: u32 = 0; /* 1:1 */
pub const VADC_DEF_DECIMATION: u32 = 0; /* 512 */
pub const VADC_DEF_HW_SETTLE_TIME: u32 = 0; /* 0 us */
pub const VADC_DEF_AVG_SAMPLES: u32 = 0; /* 1 sample */
pub const VADC_DEF_CALIB_TYPE: vadc_calibration = vadc_calibration::VADC_CALIB_ABSOLUTE;

pub const VADC_DECIMATION_MIN: u32 = 512;
pub const VADC_DECIMATION_MAX: u32 = 4096;
pub const ADC5_DEF_VBAT_PRESCALING: u32 = 1; /* 1:3 */
pub const ADC5_DECIMATION_SHORT: u32 = 250;
pub const ADC5_DECIMATION_MEDIUM: u32 = 420;
pub const ADC5_DECIMATION_LONG: u32 = 840;
/* Default decimation - 1024 for rev2, 840 for pmic5 */
pub const ADC5_DECIMATION_DEFAULT: u32 = 2;
pub const ADC5_DECIMATION_SAMPLES_MAX: u32 = 3;

pub const VADC_HW_SETTLE_DELAY_MAX: u32 = 10000;
pub const VADC_HW_SETTLE_SAMPLES_MAX: u32 = 16;
pub const VADC_AVG_SAMPLES_MAX: u32 = 512;
pub const ADC5_AVG_SAMPLES_MAX: u32 = 16;

pub const PMIC5_CHG_TEMP_SCALE_FACTOR: i32 = 377500;
pub const PMIC5_SMB_TEMP_CONSTANT: i32 = 419400;
pub const PMIC5_SMB_TEMP_SCALE_FACTOR: i32 = 356;

pub const PMI_CHG_SCALE_1: i32 = -138890;
pub const PMI_CHG_SCALE_2: i64 = 391750000000;

pub const VADC5_MAX_CODE: u16 = 0x7fff;
pub const ADC5_FULL_SCALE_CODE: u16 = 0x70e4;
pub const ADC5_USR_DATA_CHECK: u16 = 0x8000;

pub const R_PU_100K: u32 = 100000;
pub const RATIO_MAX_ADC7: u32 = 1u32 << 14;

/* VADC_CALIB_ABSOLUTE uses the 625mV and 1.25V reference channels.
 * VADC_CALIB_RATIOMETRIC uses the reference voltage (1.8V) and GND. */
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum vadc_calibration {
    VADC_CALIB_ABSOLUTE = 0,
    VADC_CALIB_RATIOMETRIC,
}

#[repr(C)]
pub struct vadc_linear_graph {
    pub dy: i32,
    pub dx: i32,
    pub gnd: i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum vadc_scale_fn_type {
    SCALE_DEFAULT = 0,
    SCALE_THERM_100K_PULLUP,
    SCALE_PMIC_THERM,
    SCALE_XOTHERM,
    SCALE_PMI_CHG_TEMP,
    SCALE_HW_CALIB_DEFAULT,
    SCALE_HW_CALIB_THERM_100K_PULLUP,
    SCALE_HW_CALIB_XOTHERM,
    SCALE_HW_CALIB_THERM_100K_PU_PM7,
    SCALE_HW_CALIB_PMIC_THERM,
    SCALE_HW_CALIB_PMIC_THERM_PM7,
    SCALE_HW_CALIB_PM5_CHG_TEMP,
    SCALE_HW_CALIB_PM5_SMB_TEMP,
    /* private: */
    SCALE_HW_CALIB_INVALID,
}

pub struct adc5_channels;
pub struct iio_info;
pub struct u32_fract {
    pub numerator: u32,
    pub denominator: u32,
}

#[repr(C)]
pub struct adc5_data {
    pub full_scale_code_volt: u32,
    pub full_scale_code_cur: u32,
    pub adc_chans: *const adc5_channels,
    pub info: *const iio_info,
    pub decimation: *mut u32,
    pub hw_settle_1: *mut u32,
    pub hw_settle_2: *mut u32,
}

extern "C" {
    pub fn qcom_vadc_scale(
        scaletype: vadc_scale_fn_type,
        calib_graph: *const vadc_linear_graph,
        prescale: *const u32_fract,
        absolute: bool,
        adc_code: u16,
        result_mdec: *mut i32,
    ) -> i32;

    pub fn qcom_adc5_hw_scale(
        scaletype: vadc_scale_fn_type,
        prescale_ratio: u32,
        data: *const adc5_data,
        adc_code: u16,
        result_mdec: *mut i32,
    ) -> i32;

    pub fn qcom_adc_tm5_temp_volt_scale(
        prescale_ratio: u32,
        full_scale_code_volt: u32,
        temp: i32,
    ) -> u16;

    pub fn qcom_adc_tm5_gen2_temp_res_scale(temp: i32) -> u16;
    pub fn qcom_adc5_prescaling_from_dt(num: u32, den: u32) -> i32;
    pub fn qcom_adc5_hw_settle_time_from_dt(value: u32, hw_settle: *const u32) -> i32;
    pub fn qcom_adc5_avg_samples_from_dt(value: u32) -> i32;
    pub fn qcom_adc5_decimation_from_dt(value: u32, decimation: *const u32) -> i32;
    pub fn qcom_vadc_decimation_from_dt(value: u32) -> i32;
}

#[repr(C)]
pub struct qcom_adc5_scale_type {
    pub scale_fn: Option<unsafe extern "C" fn(
        prescale: *const u32_fract,
        data: *const adc5_data,
        adc_code: u16,
        result: *mut i32,
    ) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
