/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Code used in the main and auxiliary Qualcomm PMIC voltage ADCs
 * of type ADC5 Gen3.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const ADC5_GEN3_HS: u32 = 0x45;
pub const ADC5_GEN3_HS_BUSY: u32 = 1 << 7;
pub const ADC5_GEN3_HS_READY: u32 = 1 << 0;
pub const ADC5_GEN3_STATUS1: u32 = 0x46;
pub const ADC5_GEN3_STATUS1_CONV_FAULT: u32 = 1 << 7;
pub const ADC5_GEN3_STATUS1_THR_CROSS: u32 = 1 << 6;
pub const ADC5_GEN3_STATUS1_EOC: u32 = 1 << 0;
pub const ADC5_GEN3_TM_EN_STS: u32 = 0x47;
pub const ADC5_GEN3_TM_HIGH_STS: u32 = 0x48;
pub const ADC5_GEN3_TM_LOW_STS: u32 = 0x49;
pub const ADC5_GEN3_EOC_STS: u32 = 0x4a;
pub const ADC5_GEN3_EOC_CHAN_0: u32 = 1 << 0;
pub const ADC5_GEN3_EOC_CLR: u32 = 0x4b;
pub const ADC5_GEN3_TM_HIGH_STS_CLR: u32 = 0x4c;
pub const ADC5_GEN3_TM_LOW_STS_CLR: u32 = 0x4d;
pub const ADC5_GEN3_CONV_ERR_CLR: u32 = 0x4e;
pub const ADC5_GEN3_CONV_ERR_CLR_REQ: u32 = 1 << 0;
pub const ADC5_GEN3_SID: u32 = 0x4f;
pub const ADC5_GEN3_SID_MASK: u32 = 0x0f;
pub const ADC5_GEN3_PERPH_CH: u32 = 0x50;
pub const ADC5_GEN3_CHAN_CONV_REQ: u32 = 1 << 7;
pub const ADC5_GEN3_TIMER_SEL: u32 = 0x51;
pub const ADC5_GEN3_TIME_IMMEDIATE: u32 = 0x1;
pub const ADC5_GEN3_DIG_PARAM: u32 = 0x52;
pub const ADC5_GEN3_DIG_PARAM_CAL_SEL_MASK: u32 = 0x30;
pub const ADC5_GEN3_DIG_PARAM_DEC_RATIO_SEL_MASK: u32 = 0x0c;
pub const ADC5_GEN3_FAST_AVG: u32 = 0x53;
pub const ADC5_GEN3_FAST_AVG_CTL_EN: u32 = 1 << 7;
pub const ADC5_GEN3_FAST_AVG_CTL_SAMPLES_MASK: u32 = 0x07;
pub const ADC5_GEN3_ADC_CH_SEL_CTL: u32 = 0x54;
pub const ADC5_GEN3_DELAY_CTL: u32 = 0x55;
pub const ADC5_GEN3_HW_SETTLE_DELAY_MASK: u32 = 0x0f;
pub const ADC5_GEN3_CH_EN: u32 = 0x56;
pub const ADC5_GEN3_HIGH_THR_INT_EN: u32 = 1 << 1;
pub const ADC5_GEN3_LOW_THR_INT_EN: u32 = 1 << 0;
pub const ADC5_GEN3_LOW_THR0: u32 = 0x57;
pub const ADC5_GEN3_LOW_THR1: u32 = 0x58;
pub const ADC5_GEN3_HIGH_THR0: u32 = 0x59;
pub const ADC5_GEN3_HIGH_THR1: u32 = 0x5a;

#[inline]
pub const fn adc5_gen3_ch_data0(channel: u32) -> u32 { 0x5c + channel * 2 }
#[inline]
pub const fn adc5_gen3_ch_data1(channel: u32) -> u32 { 0x5d + channel * 2 }

pub const ADC5_GEN3_CONV_REQ: u32 = 0xe5;
pub const ADC5_GEN3_CONV_REQ_REQ: u32 = 1 << 0;
pub const ADC5_GEN3_VIRTUAL_SID_MASK: u32 = 0xff00;
pub const ADC5_GEN3_CHANNEL_MASK: u32 = 0xff;

// ADC channels for PMIC5 Gen3
pub const ADC5_GEN3_REF_GND: u32 = 0x00;
pub const ADC5_GEN3_1P25VREF: u32 = 0x01;
pub const ADC5_GEN3_DIE_TEMP: u32 = 0x03;
pub const ADC5_GEN3_USB_SNS_V_16: u32 = 0x11;
pub const ADC5_GEN3_VIN_DIV16_MUX: u32 = 0x12;
pub const ADC5_GEN3_VPH_PWR: u32 = 0x8e;
pub const ADC5_GEN3_VBAT_SNS_QBG: u32 = 0x8f;
// 100k pull-up channels
pub const ADC5_GEN3_AMUX1_THM_100K_PU: u32 = 0x44;
pub const ADC5_GEN3_AMUX2_THM_100K_PU: u32 = 0x45;
pub const ADC5_GEN3_AMUX3_THM_100K_PU: u32 = 0x46;
pub const ADC5_GEN3_AMUX4_THM_100K_PU: u32 = 0x47;
pub const ADC5_GEN3_AMUX5_THM_100K_PU: u32 = 0x48;
pub const ADC5_GEN3_AMUX6_THM_100K_PU: u32 = 0x49;
pub const ADC5_GEN3_AMUX1_GPIO_100K_PU: u32 = 0x4a;
pub const ADC5_GEN3_AMUX2_GPIO_100K_PU: u32 = 0x4b;
pub const ADC5_GEN3_AMUX3_GPIO_100K_PU: u32 = 0x4c;
pub const ADC5_GEN3_AMUX4_GPIO_100K_PU: u32 = 0x4d;
pub const ADC5_MAX_CHANNEL: u32 = 0xc0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum adc5_cal_method { ADC5_NO_CAL = 0, ADC5_RATIOMETRIC_CAL, ADC5_ABSOLUTE_CAL }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum adc5_time_select { MEAS_INT_DISABLE = 0, MEAS_INT_IMMEDIATE, MEAS_INT_50MS, MEAS_INT_100MS, MEAS_INT_1S, MEAS_INT_NONE }

#[repr(C)]
pub struct adc5_sdam_data { pub base_addr: u16, pub irq_name: *const core::ffi::c_char, pub irq: i32 }

#[repr(C)]
pub struct adc5_device_data { pub regmap: *mut regmap, pub base: *mut adc5_sdam_data, pub num_sdams: i32 }

#[repr(C)]
pub struct adc5_channel_common_prop {
    pub channel: u32,
    pub cal_method: adc5_cal_method,
    pub decimation: u32,
    pub sid: u32,
    pub label: *const core::ffi::c_char,
    pub prescale: u32,
    pub hw_settle_time_us: u32,
    pub avg_samples: u32,
    pub scale_fn_type: vadc_scale_fn_type,
}

#[repr(C)]
pub struct tm5_aux_dev_wrapper {
    pub aux_dev: auxiliary_device,
    pub dev_data: *mut adc5_device_data,
    pub tm_props: *mut adc5_channel_common_prop,
    pub n_tm_channels: u32,
}

extern "C" {
    pub fn adc5_gen3_read(adc: *mut adc5_device_data, sdam_index: u32, offset: u16, data: *mut u8, len: i32) -> i32;
    pub fn adc5_gen3_write(adc: *mut adc5_device_data, sdam_index: u32, offset: u16, data: *mut u8, len: i32) -> i32;
    pub fn adc5_gen3_poll_wait_hs(adc: *mut adc5_device_data, sdam_index: u32) -> i32;
    pub fn adc5_gen3_update_dig_param(prop: *mut adc5_channel_common_prop, data: *mut u8);
    pub fn adc5_gen3_status_clear(adc: *mut adc5_device_data, sdam_index: i32, offset: u16, val: *mut u8, len: i32) -> i32;
    pub fn adc5_gen3_mutex_lock(dev: *mut device);
    pub fn adc5_gen3_mutex_unlock(dev: *mut device);
    pub fn adc5_gen3_get_scaled_reading(dev: *mut device, common_props: *mut adc5_channel_common_prop, val: *mut i32) -> i32;
    pub fn adc5_gen3_therm_code_to_temp(dev: *mut device, common_props: *mut adc5_channel_common_prop, code: u16, val: *mut i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
