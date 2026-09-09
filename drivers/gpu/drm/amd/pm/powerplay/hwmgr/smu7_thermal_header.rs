/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied by hwmgr.h in the original C header.

pub const SMU7_THERMAL_HIGH_ALERT_MASK: u32 = 0x1;
pub const SMU7_THERMAL_LOW_ALERT_MASK: u32 = 0x2;

pub const SMU7_THERMAL_MINIMUM_TEMP_READING: i32 = -256;
pub const SMU7_THERMAL_MAXIMUM_TEMP_READING: i32 = 255;

pub const SMU7_THERMAL_MINIMUM_ALERT_TEMP: i32 = 0;
pub const SMU7_THERMAL_MAXIMUM_ALERT_TEMP: i32 = 255;

pub const FDO_PWM_MODE_STATIC: u32 = 1;
pub const FDO_PWM_MODE_STATIC_RPM: u32 = 5;

unsafe extern "C" {
    pub fn smu7_thermal_get_temperature(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_thermal_stop_thermal_controller(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_fan_ctrl_get_fan_speed_info(
        hwmgr: *mut pp_hwmgr,
        fan_speed_info: *mut phm_fan_speed_info,
    ) -> i32;
    pub fn smu7_fan_ctrl_get_fan_speed_pwm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> i32;
    pub fn smu7_fan_ctrl_set_default_mode(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_fan_ctrl_set_static_mode(hwmgr: *mut pp_hwmgr, mode: u32) -> i32;
    pub fn smu7_fan_ctrl_set_fan_speed_pwm(hwmgr: *mut pp_hwmgr, speed: u32) -> i32;
    pub fn smu7_fan_ctrl_reset_fan_speed_to_default(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_thermal_ctrl_uninitialize_thermal_controller(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_fan_ctrl_set_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: u32) -> i32;
    pub fn smu7_fan_ctrl_get_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> i32;
    pub fn smu7_fan_ctrl_stop_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_thermal_disable_alert(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_fan_ctrl_start_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32;
    pub fn smu7_start_thermal_controller(
        hwmgr: *mut pp_hwmgr,
        temperature_range: *mut PP_TemperatureRange,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
