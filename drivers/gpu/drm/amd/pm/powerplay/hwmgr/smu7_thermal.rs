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
 */

pub unsafe fn smu7_fan_ctrl_get_fan_speed_info(hwmgr: *mut pp_hwmgr, fan_speed_info: *mut phm_fan_speed_info) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan { return -ENODEV; }
    (*fan_speed_info).supports_percent_read = true;
    (*fan_speed_info).supports_percent_write = true;
    (*fan_speed_info).min_percent = 0;
    (*fan_speed_info).max_percent = 100;
    if PP_CAP(PHM_PlatformCaps_FanSpeedInTableIsRPM) && (*hwmgr).thermal_controller.fanInfo.ucTachometerPulsesPerRevolution != 0 {
        (*fan_speed_info).supports_rpm_read = true;
        (*fan_speed_info).supports_rpm_write = true;
        (*fan_speed_info).min_rpm = (*hwmgr).thermal_controller.fanInfo.ulMinRPM;
        (*fan_speed_info).max_rpm = (*hwmgr).thermal_controller.fanInfo.ulMaxRPM;
    } else { (*fan_speed_info).min_rpm = 0; (*fan_speed_info).max_rpm = 0; }
    0
}

pub unsafe fn smu7_fan_ctrl_get_fan_speed_pwm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan { return -ENODEV; }
    let duty100 = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL1, FMAX_DUTY100);
    let duty = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_STATUS, FDO_PWM_DUTY);
    if duty100 == 0 { return -EINVAL; }
    let mut tmp64 = duty as u64 * 255;
    tmp64 /= duty100 as u64;
    *speed = core::cmp::min(tmp64, 255) as u32;
    0
}

pub unsafe fn smu7_fan_ctrl_get_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: *mut u32) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan || (*hwmgr).thermal_controller.fanInfo.ucTachometerPulsesPerRevolution == 0 { return -ENODEV; }
    let tach_period = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_TACH_STATUS, TACH_PERIOD);
    if tach_period == 0 { return -EINVAL; }
    let crystal_clock_freq = amdgpu_asic_get_xclk((*hwmgr).adev as *mut amdgpu_device);
    *speed = 60 * crystal_clock_freq * 10000 / tach_period;
    0
}

pub unsafe fn smu7_fan_ctrl_set_static_mode(hwmgr: *mut pp_hwmgr, mode: u32) -> i32 {
    if (*hwmgr).fan_ctrl_is_in_default_mode {
        (*hwmgr).fan_ctrl_default_mode = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, FDO_PWM_MODE);
        (*hwmgr).tmin = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, TMIN);
        (*hwmgr).fan_ctrl_is_in_default_mode = false;
    }
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, TMIN, 0);
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, FDO_PWM_MODE, mode);
    0
}

pub unsafe fn smu7_fan_ctrl_set_default_mode(hwmgr: *mut pp_hwmgr) -> i32 {
    if !(*hwmgr).fan_ctrl_is_in_default_mode {
        PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, FDO_PWM_MODE, (*hwmgr).fan_ctrl_default_mode);
        PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, TMIN, (*hwmgr).tmin);
        (*hwmgr).fan_ctrl_is_in_default_mode = true;
    }
    0
}

pub unsafe fn smu7_fan_ctrl_start_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut result;
    if PP_CAP(PHM_PlatformCaps_ODFuzzyFanControlSupport) {
        result = smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_StartFanControl, FAN_CONTROL_FUZZY, core::ptr::null_mut());
        if PP_CAP(PHM_PlatformCaps_FanSpeedInTableIsRPM) { (*hwmgr).hwmgr_func.set_max_fan_rpm_output(hwmgr, (*hwmgr).thermal_controller.advanceFanControlParameters.usMaxFanRPM); }
        else { (*hwmgr).hwmgr_func.set_max_fan_pwm_output(hwmgr, (*hwmgr).thermal_controller.advanceFanControlParameters.usMaxFanPWM); }
    } else { result = smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_StartFanControl, FAN_CONTROL_TABLE, core::ptr::null_mut()); }
    if result == 0 && (*hwmgr).thermal_controller.advanceFanControlParameters.ucTargetTemperature != 0 { result = smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_MSG_SetFanTemperatureTarget, (*hwmgr).thermal_controller.advanceFanControlParameters.ucTargetTemperature, core::ptr::null_mut()); }
    if result == 0 && ((*hwmgr).chip_id == CHIP_POLARIS10 || (*hwmgr).chip_id == CHIP_POLARIS11 || (*hwmgr).chip_id == CHIP_POLARIS12) && (*hwmgr).thermal_controller.advanceFanControlParameters.ucEnableZeroRPM && !PP_CAP(PHM_PlatformCaps_customThermalManagement) { result = smum_send_msg_to_smc(hwmgr, PPSMC_MSG_EnableZeroRpm, core::ptr::null_mut()); }
    (*hwmgr).fan_ctrl_enabled = true; result
}

pub unsafe fn smu7_fan_ctrl_stop_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32 { (*hwmgr).fan_ctrl_enabled = false; smum_send_msg_to_smc(hwmgr, PPSMC_StopFanControl, core::ptr::null_mut()) }

pub unsafe fn smu7_fan_ctrl_set_fan_speed_pwm(hwmgr: *mut pp_hwmgr, mut speed: u32) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan { return 0; }
    speed = core::cmp::min(speed, 255);
    if PP_CAP(PHM_PlatformCaps_MicrocodeFanControl) { smu7_fan_ctrl_stop_smc_fan_control(hwmgr); }
    let duty100 = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL1, FMAX_DUTY100);
    if duty100 == 0 { return -EINVAL; }
    let duty = (speed as u64 * duty100 as u64 / 255) as u32;
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL0, FDO_STATIC_DUTY, duty);
    smu7_fan_ctrl_set_static_mode(hwmgr, FDO_PWM_MODE_STATIC)
}

pub unsafe fn smu7_fan_ctrl_reset_fan_speed_to_default(hwmgr: *mut pp_hwmgr) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan { return 0; }
    if PP_CAP(PHM_PlatformCaps_MicrocodeFanControl) { let mut result = smu7_fan_ctrl_set_static_mode(hwmgr, FDO_PWM_MODE_STATIC); if result == 0 { result = smu7_fan_ctrl_start_smc_fan_control(hwmgr); } result } else { smu7_fan_ctrl_set_default_mode(hwmgr) }
}

pub unsafe fn smu7_fan_ctrl_set_fan_speed_rpm(hwmgr: *mut pp_hwmgr, speed: u32) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.bNoFan || (*hwmgr).thermal_controller.fanInfo.ucTachometerPulsesPerRevolution == 0 || speed == 0 || speed > u32::MAX / 8 || speed < (*hwmgr).thermal_controller.fanInfo.ulMinRPM || speed > (*hwmgr).thermal_controller.fanInfo.ulMaxRPM { return -EINVAL; }
    if PP_CAP(PHM_PlatformCaps_MicrocodeFanControl) { smu7_fan_ctrl_stop_smc_fan_control(hwmgr); }
    let crystal_clock_freq = amdgpu_asic_get_xclk((*hwmgr).adev as *mut amdgpu_device);
    let tach_period = 60 * crystal_clock_freq * 10000 / (8 * speed);
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_TACH_CTRL, TARGET_PERIOD, tach_period);
    smu7_fan_ctrl_set_static_mode(hwmgr, FDO_PWM_MODE_STATIC_RPM)
}

pub unsafe fn smu7_thermal_get_temperature(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut temp = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_MULT_THERMAL_STATUS, CTF_TEMP);
    if temp & 0x200 != 0 { temp = SMU7_THERMAL_MAXIMUM_TEMP_READING; } else { temp &= 0x1ff; }
    temp * PP_TEMPERATURE_UNITS_PER_CENTIGRADES
}

unsafe fn smu7_thermal_set_temperature_range(hwmgr: *mut pp_hwmgr, low_temp: i32, high_temp: i32) -> i32 {
    let mut low = SMU7_THERMAL_MINIMUM_ALERT_TEMP * PP_TEMPERATURE_UNITS_PER_CENTIGRADES;
    let mut high = SMU7_THERMAL_MAXIMUM_ALERT_TEMP * PP_TEMPERATURE_UNITS_PER_CENTIGRADES;
    if low < low_temp { low = low_temp; } if high > high_temp { high = high_temp; } if low > high { return -EINVAL; }
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, DIG_THERM_INTH, high / PP_TEMPERATURE_UNITS_PER_CENTIGRADES);
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, DIG_THERM_INTL, low / PP_TEMPERATURE_UNITS_PER_CENTIGRADES);
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_CTRL, DIG_THERM_DPM, high / PP_TEMPERATURE_UNITS_PER_CENTIGRADES); 0
}

unsafe fn smu7_thermal_initialize(hwmgr: *mut pp_hwmgr) -> i32 {
    if (*hwmgr).thermal_controller.fanInfo.ucTachometerPulsesPerRevolution != 0 { PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_TACH_CTRL, EDGE_PER_REV, (*hwmgr).thermal_controller.fanInfo.ucTachometerPulsesPerRevolution - 1); }
    PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_FDO_CTRL2, TACH_PWM_RESP_RATE, 0x28); 0
}

unsafe fn smu7_thermal_enable_alert(hwmgr: *mut pp_hwmgr) { let mut alert = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, THERM_INT_MASK); alert &= !(SMU7_THERMAL_HIGH_ALERT_MASK | SMU7_THERMAL_LOW_ALERT_MASK); PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, THERM_INT_MASK, alert); smum_send_msg_to_smc(hwmgr, PPSMC_MSG_Thermal_Cntl_Enable, core::ptr::null_mut()); }

pub unsafe fn smu7_thermal_disable_alert(hwmgr: *mut pp_hwmgr) -> i32 { let mut alert = PHM_READ_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, THERM_INT_MASK); alert |= SMU7_THERMAL_HIGH_ALERT_MASK | SMU7_THERMAL_LOW_ALERT_MASK; PHM_WRITE_VFPF_INDIRECT_FIELD((*hwmgr).device, CGS_IND_REG__SMC, CG_THERMAL_INT, THERM_INT_MASK, alert); smum_send_msg_to_smc(hwmgr, PPSMC_MSG_Thermal_Cntl_Disable, core::ptr::null_mut()) }

pub unsafe fn smu7_thermal_stop_thermal_controller(hwmgr: *mut pp_hwmgr) -> i32 { let result = smu7_thermal_disable_alert(hwmgr); if !(*hwmgr).thermal_controller.fanInfo.bNoFan { smu7_fan_ctrl_set_default_mode(hwmgr); } result }

unsafe fn smu7_thermal_start_smc_fan_control(hwmgr: *mut pp_hwmgr) -> i32 { if PP_CAP(PHM_PlatformCaps_MicrocodeFanControl) { smu7_fan_ctrl_start_smc_fan_control(hwmgr); smu7_fan_ctrl_set_static_mode(hwmgr, FDO_PWM_MODE_STATIC); } 0 }

pub unsafe fn smu7_start_thermal_controller(hwmgr: *mut pp_hwmgr, range: *mut PP_TemperatureRange) -> i32 {
    if range.is_null() { return -EINVAL; }
    smu7_thermal_initialize(hwmgr); if smu7_thermal_set_temperature_range(hwmgr, (*range).min, (*range).max) != 0 { return -EINVAL; }
    smu7_thermal_enable_alert(hwmgr); if smum_thermal_avfs_enable(hwmgr) != 0 { return -EINVAL; }
    smum_thermal_setup_fan_table(hwmgr); smu7_thermal_start_smc_fan_control(hwmgr); 0
}

pub unsafe fn smu7_thermal_ctrl_uninitialize_thermal_controller(hwmgr: *mut pp_hwmgr) -> i32 { if !(*hwmgr).thermal_controller.fanInfo.bNoFan { smu7_fan_ctrl_set_default_mode(hwmgr); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
