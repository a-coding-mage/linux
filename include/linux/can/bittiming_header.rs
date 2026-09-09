/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2020 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
 * Copyright (c) 2021-2025 Vincent Mailhol <mailhol@kernel.org>
 */

use core::mem::ManuallyDrop;

pub const CAN_SYNC_SEG: u32 = 1;
pub const CAN_BITRATE_UNSET: u32 = 0;
pub const CAN_BITRATE_UNKNOWN: u32 = !0u32;

pub const CAN_CTRLMODE_FD_TDC_MASK: u32 = CAN_CTRLMODE_TDC_AUTO | CAN_CTRLMODE_TDC_MANUAL;
pub const CAN_CTRLMODE_XL_TDC_MASK: u32 = CAN_CTRLMODE_XL_TDC_AUTO | CAN_CTRLMODE_XL_TDC_MANUAL;
pub const CAN_CTRLMODE_TDC_AUTO_MASK: u32 = CAN_CTRLMODE_TDC_AUTO | CAN_CTRLMODE_XL_TDC_AUTO;
pub const CAN_CTRLMODE_TDC_MANUAL_MASK: u32 = CAN_CTRLMODE_TDC_MANUAL | CAN_CTRLMODE_XL_TDC_MANUAL;

#[repr(C)]
pub struct can_tdc {
    pub tdcv: u32,
    pub tdco: u32,
    pub tdcf: u32,
}

pub const CAN_PWM_DECODE_NS: u32 = 5;
pub const CAN_PWM_NS_MAX: u32 = 205 - CAN_PWM_DECODE_NS;

#[repr(C)]
pub struct can_tdc_const {
    pub tdcv_min: u32,
    pub tdcv_max: u32,
    pub tdco_min: u32,
    pub tdco_max: u32,
    pub tdcf_min: u32,
    pub tdcf_max: u32,
}

#[repr(C)]
pub struct can_pwm {
    pub pwms: u32,
    pub pwml: u32,
    pub pwmo: u32,
}

#[repr(C)]
pub struct can_pwm_const {
    pub pwms_min: u32,
    pub pwms_max: u32,
    pub pwml_min: u32,
    pub pwml_max: u32,
    pub pwmo_min: u32,
    pub pwmo_max: u32,
}

#[repr(C)]
pub union data_bittiming_params__bindgen_ty_1 {
    pub tdc: ManuallyDrop<can_tdc>,
    pub pwm: ManuallyDrop<can_pwm>,
}

#[repr(C)]
pub struct data_bittiming_params {
    pub data_bittiming_const: *const can_bittiming_const,
    pub data_bittiming: can_bittiming,
    pub tdc_const: *const can_tdc_const,
    pub pwm_const: *const can_pwm_const,
    pub __bindgen_anon_1: data_bittiming_params__bindgen_ty_1,
    pub data_bitrate_const: *const u32,
    pub data_bitrate_const_cnt: core::ffi::c_uint,
    pub do_set_data_bittiming: Option<unsafe extern "C" fn(dev: *mut net_device) -> core::ffi::c_int>,
    pub do_get_auto_tdcv: Option<unsafe extern "C" fn(dev: *const net_device, tdcv: *mut u32) -> core::ffi::c_int>,
}

#[cfg(CONFIG_CAN_CALC_BITTIMING)]
extern "C" {
    pub fn can_calc_bittiming(dev: *const net_device, bt: *mut can_bittiming,
                              btc: *const can_bittiming_const, extack: *mut netlink_ext_ack) -> core::ffi::c_int;
    pub fn can_calc_tdco(tdc: *mut can_tdc, tdc_const: *const can_tdc_const,
                         dbt: *const can_bittiming, tdc_mask: u32,
                         ctrlmode: *mut u32, ctrlmode_supported: u32);
    pub fn can_calc_pwm(dev: *mut net_device, extack: *mut netlink_ext_ack) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_CAN_CALC_BITTIMING))]
pub unsafe fn can_calc_bittiming(_dev: *const net_device, _bt: *mut can_bittiming,
                                 _btc: *const can_bittiming_const, extack: *mut netlink_ext_ack) -> core::ffi::c_int {
    NL_SET_ERR_MSG(extack, "bit-timing calculation not available\n");
    -EINVAL
}

#[cfg(not(CONFIG_CAN_CALC_BITTIMING))]
pub unsafe fn can_calc_tdco(_tdc: *mut can_tdc, _tdc_const: *const can_tdc_const,
                            _dbt: *const can_bittiming, _tdc_mask: u32,
                            _ctrlmode: *mut u32, _ctrlmode_supported: u32) {}

#[cfg(not(CONFIG_CAN_CALC_BITTIMING))]
pub unsafe fn can_calc_pwm(_dev: *mut net_device, extack: *mut netlink_ext_ack) -> core::ffi::c_int {
    NL_SET_ERR_MSG(extack, "bit-timing calculation not available: manually provide PWML and PWMS\n");
    -EINVAL
}

extern "C" {
    pub fn can_sjw_set_default(bt: *mut can_bittiming);
    pub fn can_sjw_check(dev: *const net_device, bt: *const can_bittiming,
                         btc: *const can_bittiming_const, extack: *mut netlink_ext_ack) -> core::ffi::c_int;
    pub fn can_get_bittiming(dev: *const net_device, bt: *mut can_bittiming,
                             btc: *const can_bittiming_const, bitrate_const: *const u32,
                             bitrate_const_cnt: core::ffi::c_uint, extack: *mut netlink_ext_ack) -> core::ffi::c_int;
    pub fn can_validate_pwm_bittiming(dev: *const net_device, pwm: *const can_pwm,
                                      extack: *mut netlink_ext_ack) -> core::ffi::c_int;
}

pub unsafe fn can_get_relative_tdco(dbt_params: *const data_bittiming_params) -> i32 {
    let dbt = &(*dbt_params).data_bittiming;
    let sample_point_in_tc = (CAN_SYNC_SEG + dbt.prop_seg + dbt.phase_seg1) * dbt.brp;
    (*dbt_params).__bindgen_anon_1.tdc.tdco as i32 - sample_point_in_tc as i32
}

pub unsafe fn can_bit_time(bt: *const can_bittiming) -> core::ffi::c_uint {
    CAN_SYNC_SEG + (*bt).prop_seg + (*bt).phase_seg1 + (*bt).phase_seg2
}

pub unsafe fn can_bit_time_tqmin(bt: *const can_bittiming) -> core::ffi::c_uint {
    can_bit_time(bt) * (*bt).brp
}

pub unsafe fn can_tqmin_to_ns(tqmin: u32, clock_freq: u32) -> u32 {
    DIV_U64_ROUND_CLOSEST(mul_u32_u32(tqmin, NSEC_PER_SEC), clock_freq)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
