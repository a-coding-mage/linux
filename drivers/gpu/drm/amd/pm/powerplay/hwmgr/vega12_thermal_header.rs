/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

// Dependency supplied by hwmgr.h.

#[repr(C)]
pub struct vega12_temperature {
    pub edge_temp: u16,
    pub hot_spot_temp: u16,
    pub hbm_temp: u16,
    pub vr_soc_temp: u16,
    pub vr_mem_temp: u16,
    pub liquid1_temp: u16,
    pub liquid2_temp: u16,
    pub plx_temp: u16,
}

pub const VEGA12_THERMAL_HIGH_ALERT_MASK: i32 = 0x1;
pub const VEGA12_THERMAL_LOW_ALERT_MASK: i32 = 0x2;

pub const VEGA12_THERMAL_MINIMUM_TEMP_READING: i32 = -256;
pub const VEGA12_THERMAL_MAXIMUM_TEMP_READING: i32 = 255;

pub const VEGA12_THERMAL_MINIMUM_ALERT_TEMP: i32 = 0;
pub const VEGA12_THERMAL_MAXIMUM_ALERT_TEMP: i32 = 255;

pub const FDO_PWM_MODE_STATIC: i32 = 1;
pub const FDO_PWM_MODE_STATIC_RPM: i32 = 5;

extern "C" {
    pub fn vega12_thermal_get_temperature(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_thermal_stop_thermal_controller(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_fan_ctrl_get_fan_speed_info(
        hwmgr: *mut pp_hwmgr,
        fan_speed_info: *mut phm_fan_speed_info,
    ) -> i32;
    pub fn vega12_fan_ctrl_reset_fan_speed_to_default(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_fan_ctrl_get_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> i32;
    pub fn vega12_fan_ctrl_stop_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_thermal_disable_alert(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_fan_ctrl_start_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn vega12_start_thermal_controller(
        hwmgr: *mut pp_hwmgr,
        range: *mut PP_TemperatureRange,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
