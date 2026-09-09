/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding kernel translation unit.

pub const TEMP_RANGE_MIN: i32 = 0;
pub const TEMP_RANGE_MAX: i32 = 80 * 1000;

macro_rules! PHM_FUNC_CHECK {
    ($hw:expr) => {
        if ($hw).is_null() || unsafe { (*$hw).hwmgr_func.is_null() } { return -EINVAL; }
    };
}

pub unsafe fn phm_setup_asic(hwmgr: *mut pp_hwmgr) -> i32 {
    PHM_FUNC_CHECK!(hwmgr);
    if !(*(*hwmgr).hwmgr_func).asic_setup.is_none() { return ((*(*hwmgr).hwmgr_func).asic_setup.unwrap())(hwmgr); }
    0
}
pub unsafe fn phm_power_down_asic(hwmgr: *mut pp_hwmgr) -> i32 {
    PHM_FUNC_CHECK!(hwmgr);
    if !(*(*hwmgr).hwmgr_func).power_off_asic.is_none() { return ((*(*hwmgr).hwmgr_func).power_off_asic.unwrap())(hwmgr); }
    0
}
pub unsafe fn phm_set_power_state(hwmgr: *mut pp_hwmgr, pcurrent_state: *const pp_hw_power_state, pnew_power_state: *const pp_hw_power_state) -> i32 {
    let mut states = phm_set_power_state_input { pcurrent_state, pnew_state: pnew_power_state };
    PHM_FUNC_CHECK!(hwmgr);
    if !(*(*hwmgr).hwmgr_func).power_state_set.is_none() { return ((*(*hwmgr).hwmgr_func).power_state_set.unwrap())(hwmgr, &mut states); }
    0
}
pub unsafe fn phm_enable_dynamic_state_management(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut ret = -EINVAL;
    PHM_FUNC_CHECK!(hwmgr);
    let adev = (*hwmgr).adev;
    if !(*hwmgr).pp_one_vf && smum_is_dpm_running(hwmgr) && !amdgpu_passthrough(adev) && (*adev).in_suspend && (*adev).asic_type != CHIP_RAVEN { pr_info!("dpm has been enabled\n"); return 0; }
    if !(*(*hwmgr).hwmgr_func).dynamic_state_management_enable.is_none() { ret = ((*(*hwmgr).hwmgr_func).dynamic_state_management_enable.unwrap())(hwmgr); }
    ret
}
pub unsafe fn phm_disable_dynamic_state_management(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut ret = -EINVAL;
    PHM_FUNC_CHECK!(hwmgr);
    if !(*hwmgr).not_vf { return 0; }
    if !smum_is_dpm_running(hwmgr) { pr_info!("dpm has been disabled\n"); return 0; }
    if !(*(*hwmgr).hwmgr_func).dynamic_state_management_disable.is_none() { ret = ((*(*hwmgr).hwmgr_func).dynamic_state_management_disable.unwrap())(hwmgr); }
    ret
}
pub unsafe fn phm_force_dpm_levels(hwmgr: *mut pp_hwmgr, level: amd_dpm_forced_level) -> i32 { let mut ret=0; PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).force_dpm_level.is_none() { ret=((*(*hwmgr).hwmgr_func).force_dpm_level.unwrap())(hwmgr,level); } ret }

pub unsafe fn phm_apply_state_adjust_rules(hwmgr:*mut pp_hwmgr, adjusted_ps:*mut pp_power_state, current_ps:*const pp_power_state)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).apply_state_adjust_rules.is_none() { return ((*(*hwmgr).hwmgr_func).apply_state_adjust_rules.unwrap())(hwmgr,adjusted_ps,current_ps); } 0 }
pub unsafe fn phm_apply_clock_adjust_rules(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).apply_clocks_adjust_rules.is_none() { return ((*(*hwmgr).hwmgr_func).apply_clocks_adjust_rules.unwrap())(hwmgr); } 0 }
pub unsafe fn phm_disable_clock_power_gatings(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).disable_clock_power_gating.is_none() { return ((*(*hwmgr).hwmgr_func).disable_clock_power_gating.unwrap())(hwmgr); } 0 }
pub unsafe fn phm_pre_display_configuration_changed(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).pre_display_config_changed.is_none() { ((*(*hwmgr).hwmgr_func).pre_display_config_changed.unwrap())(hwmgr); } 0 }
pub unsafe fn phm_display_configuration_changed(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).display_config_changed.is_none() { ((*(*hwmgr).hwmgr_func).display_config_changed.unwrap())(hwmgr); } 0 }
pub unsafe fn phm_notify_smc_display_config_after_ps_adjustment(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).notify_smc_display_config_after_ps_adjustment.is_none() { ((*(*hwmgr).hwmgr_func).notify_smc_display_config_after_ps_adjustment.unwrap())(hwmgr); } 0 }
pub unsafe fn phm_stop_thermal_controller(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*hwmgr).not_vf{return 0;} if (*(*hwmgr).hwmgr_func).stop_thermal_controller.is_none(){return -EINVAL;} ((*(*hwmgr).hwmgr_func).stop_thermal_controller.unwrap())(hwmgr) }
pub unsafe fn phm_register_irq_handlers(hwmgr:*mut pp_hwmgr)->i32 { PHM_FUNC_CHECK!(hwmgr); if !(*(*hwmgr).hwmgr_func).register_irq_handlers.is_none(){return ((*(*hwmgr).hwmgr_func).register_irq_handlers.unwrap())(hwmgr);} 0 }

pub unsafe fn phm_start_thermal_controller(hwmgr:*mut pp_hwmgr)->i32 {
    let mut ret=0;
    let mut range=PP_TemperatureRange { min:TEMP_RANGE_MIN,max:TEMP_RANGE_MAX,edge_emergency_max:TEMP_RANGE_MAX,hotspot_min:TEMP_RANGE_MIN,hotspot_crit_max:TEMP_RANGE_MAX,hotspot_emergency_max:TEMP_RANGE_MAX,mem_min:TEMP_RANGE_MIN,mem_crit_max:TEMP_RANGE_MAX,mem_emergency_max:TEMP_RANGE_MAX,sw_ctf_threshold:0 };
    let adev=(*hwmgr).adev;
    if !(*hwmgr).not_vf{return 0;}
    if !(*(*hwmgr).hwmgr_func).get_thermal_temperature_range.is_none(){((*(*hwmgr).hwmgr_func).get_thermal_temperature_range.unwrap())(hwmgr,&mut range);}
    if phm_cap_enabled((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_ThermalController) && !(*(*hwmgr).hwmgr_func).start_thermal_controller.is_none(){ret=((*(*hwmgr).hwmgr_func).start_thermal_controller.unwrap())(hwmgr,&mut range);}
    (*adev).pm.dpm.thermal.min_temp=range.min; (*adev).pm.dpm.thermal.max_temp=range.max; (*adev).pm.dpm.thermal.max_edge_emergency_temp=range.edge_emergency_max; (*adev).pm.dpm.thermal.min_hotspot_temp=range.hotspot_min; (*adev).pm.dpm.thermal.max_hotspot_crit_temp=range.hotspot_crit_max; (*adev).pm.dpm.thermal.max_hotspot_emergency_temp=range.hotspot_emergency_max; (*adev).pm.dpm.thermal.min_mem_temp=range.mem_min; (*adev).pm.dpm.thermal.max_mem_crit_temp=range.mem_crit_max; (*adev).pm.dpm.thermal.max_mem_emergency_temp=range.mem_emergency_max; (*adev).pm.dpm.thermal.sw_ctf_threshold=range.sw_ctf_threshold; ret
}

pub unsafe fn phm_check_smc_update_required_for_display_configuration(hwmgr:*mut pp_hwmgr)->bool { if hwmgr.is_null()||(*hwmgr).hwmgr_func.is_null()||(*hwmgr).pp_one_vf{return false;} if (*(*hwmgr).hwmgr_func).check_smc_update_required_for_display_configuration.is_none(){return false;} ((*(*hwmgr).hwmgr_func).check_smc_update_required_for_display_configuration.unwrap())(hwmgr) }
pub unsafe fn phm_check_states_equal(hwmgr:*mut pp_hwmgr,pstate1:*const pp_hw_power_state,pstate2:*const pp_hw_power_state,equal:*mut bool)->i32 { PHM_FUNC_CHECK!(hwmgr); if (*(*hwmgr).hwmgr_func).check_states_equal.is_none(){return -EINVAL;} ((*(*hwmgr).hwmgr_func).check_states_equal.unwrap())(hwmgr,pstate1,pstate2,equal) }

pub unsafe fn phm_store_dal_configuration_data(hwmgr:*mut pp_hwmgr,display_config:*const amd_pp_display_configuration)->i32 { let mut number=0; PHM_FUNC_CHECK!(hwmgr); if display_config.is_null(){return -EINVAL;} if !(*(*hwmgr).hwmgr_func).set_min_deep_sleep_dcefclk.is_none(){((*(*hwmgr).hwmgr_func).set_min_deep_sleep_dcefclk.unwrap())(hwmgr,(*display_config).min_dcef_deep_sleep_set_clk);} for index in 0..(*display_config).num_path_including_non_display {if (*display_config).displays[index as usize].controller_id!=0{number+=1;}} if !(*(*hwmgr).hwmgr_func).set_active_display_count.is_none(){((*(*hwmgr).hwmgr_func).set_active_display_count.unwrap())(hwmgr,number);} if (*(*hwmgr).hwmgr_func).store_cc6_data.is_none(){return -EINVAL;} ((*(*hwmgr).hwmgr_func).store_cc6_data.unwrap())(hwmgr,(*display_config).cpu_pstate_separation_time,(*display_config).cpu_cc6_disable,(*display_config).cpu_pstate_disable,(*display_config).nb_pstate_switch_disable); 0 }
pub unsafe fn phm_set_cpu_power_state(hwmgr:*mut pp_hwmgr)->i32 {PHM_FUNC_CHECK!(hwmgr);if !(*(*hwmgr).hwmgr_func).set_cpu_power_state.is_none(){return ((*(*hwmgr).hwmgr_func).set_cpu_power_state.unwrap())(hwmgr);}0}
pub unsafe fn phm_get_performance_level(hwmgr:*mut pp_hwmgr,state:*const pp_hw_power_state,designation:PHM_PerformanceLevelDesignation,index:u32,level:*mut PHM_PerformanceLevel)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_performance_level.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_performance_level.unwrap())(hwmgr,state,designation,index,level)}

pub unsafe fn phm_get_clock_info(hwmgr:*mut pp_hwmgr,state:*const pp_hw_power_state,pclock_info:*mut pp_clock_info,designation:PHM_PerformanceLevelDesignation)->i32 { let mut level=PHM_PerformanceLevel::default(); PHM_FUNC_CHECK!(hwmgr); if state.is_null()||pclock_info.is_null(){return -EINVAL;} let mut result=phm_get_performance_level(hwmgr,state,PHM_PerformanceLevelDesignation_Activity,0,&mut level); if result!=0{return result;} (*pclock_info).min_mem_clk=level.memory_clock;(*pclock_info).min_eng_clk=level.coreClock;(*pclock_info).min_bus_bandwidth=level.nonLocalMemoryFreq*level.nonLocalMemoryWidth; result=phm_get_performance_level(hwmgr,state,designation,(*hwmgr).platform_descriptor.hardwareActivityPerformanceLevels-1,&mut level);if result!=0{return result;}(*pclock_info).max_mem_clk=level.memory_clock;(*pclock_info).max_eng_clk=level.coreClock;(*pclock_info).max_bus_bandwidth=level.nonLocalMemoryFreq*level.nonLocalMemoryWidth;0 }
pub unsafe fn phm_get_current_shallow_sleep_clocks(hwmgr:*mut pp_hwmgr,state:*const pp_hw_power_state,clock_info:*mut pp_clock_info)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_current_shallow_sleep_clocks.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_current_shallow_sleep_clocks.unwrap())(hwmgr,state,clock_info)}
pub unsafe fn phm_get_clock_by_type(hwmgr:*mut pp_hwmgr,typ:amd_pp_clock_type,clocks:*mut amd_pp_clocks)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_clock_by_type.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_clock_by_type.unwrap())(hwmgr,typ,clocks)}
pub unsafe fn phm_get_clock_by_type_with_latency(hwmgr:*mut pp_hwmgr,typ:amd_pp_clock_type,clocks:*mut pp_clock_levels_with_latency)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_clock_by_type_with_latency.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_clock_by_type_with_latency.unwrap())(hwmgr,typ,clocks)}
pub unsafe fn phm_get_clock_by_type_with_voltage(hwmgr:*mut pp_hwmgr,typ:amd_pp_clock_type,clocks:*mut pp_clock_levels_with_voltage)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_clock_by_type_with_voltage.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_clock_by_type_with_voltage.unwrap())(hwmgr,typ,clocks)}
pub unsafe fn phm_set_watermarks_for_clocks_ranges(hwmgr:*mut pp_hwmgr,clock_ranges:*mut core::ffi::c_void)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).set_watermarks_for_clocks_ranges.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).set_watermarks_for_clocks_ranges.unwrap())(hwmgr,clock_ranges)}
pub unsafe fn phm_display_clock_voltage_request(hwmgr:*mut pp_hwmgr,clock:*mut pp_display_clock_request)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).display_clock_voltage_request.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).display_clock_voltage_request.unwrap())(hwmgr,clock)}
pub unsafe fn phm_get_max_high_clocks(hwmgr:*mut pp_hwmgr,clocks:*mut amd_pp_simple_clock_info)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).get_max_high_clocks.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).get_max_high_clocks.unwrap())(hwmgr,clocks)}
pub unsafe fn phm_disable_smc_firmware_ctf(hwmgr:*mut pp_hwmgr)->i32 {PHM_FUNC_CHECK!(hwmgr);if !(*hwmgr).not_vf{return 0;}if (*(*hwmgr).hwmgr_func).disable_smc_firmware_ctf.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).disable_smc_firmware_ctf.unwrap())(hwmgr)}
pub unsafe fn phm_set_active_display_count(hwmgr:*mut pp_hwmgr,count:u32)->i32 {PHM_FUNC_CHECK!(hwmgr);if (*(*hwmgr).hwmgr_func).set_active_display_count.is_none(){return -EINVAL;}((*(*hwmgr).hwmgr_func).set_active_display_count.unwrap())(hwmgr,count)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
