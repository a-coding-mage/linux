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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Kernel declarations and pp_psm.h supply the types and external symbols used here.

pub unsafe fn psm_init_power_state_table(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut result: i32;
    let mut i: u32;
    let mut state: *mut pp_power_state;
    let mut size: i32;
    let table_entries: i32;

    if (*(*hwmgr).hwmgr_func).get_num_of_pp_table_entries.is_none() { return 0; }
    if (*(*hwmgr).hwmgr_func).get_power_state_size.is_none() { return 0; }
    table_entries = ((*(*hwmgr).hwmgr_func).get_num_of_pp_table_entries.unwrap())(hwmgr);
    size = ((*(*hwmgr).hwmgr_func).get_power_state_size.unwrap())(hwmgr)
        + core::mem::size_of::<pp_power_state>() as i32;
    if table_entries <= 0 || size == 0 {
        pr_warn!("Please check whether power state management is supported on this asic\n");
        (*hwmgr).num_ps = 0; (*hwmgr).ps_size = 0; return 0;
    }
    (*hwmgr).num_ps = table_entries; (*hwmgr).ps_size = size;
    (*hwmgr).ps = kcalloc(table_entries as usize, size as usize, GFP_KERNEL);
    if (*hwmgr).ps.is_null() { return -ENOMEM; }
    (*hwmgr).request_ps = kzalloc(size as usize, GFP_KERNEL);
    if (*hwmgr).request_ps.is_null() { kfree((*hwmgr).ps as *mut _); (*hwmgr).ps = core::ptr::null_mut(); return -ENOMEM; }
    (*hwmgr).current_ps = kzalloc(size as usize, GFP_KERNEL);
    if (*hwmgr).current_ps.is_null() {
        kfree((*hwmgr).request_ps as *mut _); kfree((*hwmgr).ps as *mut _);
        (*hwmgr).request_ps = core::ptr::null_mut(); (*hwmgr).ps = core::ptr::null_mut(); return -ENOMEM;
    }
    state = (*hwmgr).ps;
    for i in 0..table_entries as u32 {
        result = ((*(*hwmgr).hwmgr_func).get_pp_table_entry.unwrap())(hwmgr, i, state);
        if result != 0 {
            kfree((*hwmgr).current_ps as *mut _); kfree((*hwmgr).request_ps as *mut _); kfree((*hwmgr).ps as *mut _);
            (*hwmgr).current_ps = core::ptr::null_mut(); (*hwmgr).request_ps = core::ptr::null_mut(); (*hwmgr).ps = core::ptr::null_mut(); return -EINVAL;
        }
        if (*state).classification.flags & PP_StateClassificationFlag_Boot != 0 {
            (*hwmgr).boot_ps = state;
            core::ptr::copy_nonoverlapping(state as *const u8, (*hwmgr).current_ps as *mut u8, size as usize);
            core::ptr::copy_nonoverlapping(state as *const u8, (*hwmgr).request_ps as *mut u8, size as usize);
        }
        (*state).id = i + 1;
        if (*state).classification.flags & PP_StateClassificationFlag_Uvd != 0 { (*hwmgr).uvd_ps = state; }
        state = ((state as usize) + size as usize) as *mut pp_power_state;
    }
    0
}

pub unsafe fn psm_fini_power_state_table(hwmgr: *mut pp_hwmgr) -> i32 {
    if hwmgr.is_null() { return -EINVAL; }
    if (*hwmgr).ps.is_null() { return 0; }
    kfree((*hwmgr).current_ps as *mut _); kfree((*hwmgr).request_ps as *mut _); kfree((*hwmgr).ps as *mut _);
    (*hwmgr).request_ps = core::ptr::null_mut(); (*hwmgr).ps = core::ptr::null_mut(); (*hwmgr).current_ps = core::ptr::null_mut(); 0
}

unsafe fn psm_get_ui_state(hwmgr: *mut pp_hwmgr, ui_label: PP_StateUILabel, state_id: *mut usize) -> i32 {
    let mut state = (*hwmgr).ps;
    for _ in 0..(*hwmgr).num_ps { if (*state).classification.ui_label & ui_label != 0 { *state_id = (*state).id as usize; return 0; } state = ((state as usize) + (*hwmgr).ps_size as usize) as *mut pp_power_state; } -EINVAL
}
unsafe fn psm_get_state_by_classification(hwmgr: *mut pp_hwmgr, flag: PP_StateClassificationFlag, state_id: *mut usize) -> i32 {
    let mut state = (*hwmgr).ps;
    for _ in 0..(*hwmgr).num_ps { if (*state).classification.flags & flag != 0 { *state_id = (*state).id as usize; return 0; } state = ((state as usize) + (*hwmgr).ps_size as usize) as *mut pp_power_state; } -EINVAL
}
unsafe fn psm_set_states(hwmgr: *mut pp_hwmgr, state_id: usize) -> i32 {
    let mut state = (*hwmgr).ps;
    for _ in 0..(*hwmgr).num_ps { if (*state).id as usize == state_id { core::ptr::copy_nonoverlapping(state as *const u8, (*hwmgr).request_ps as *mut u8, (*hwmgr).ps_size as usize); return 0; } state = ((state as usize) + (*hwmgr).ps_size as usize) as *mut pp_power_state; } -EINVAL
}

pub unsafe fn psm_set_boot_states(hwmgr: *mut pp_hwmgr) -> i32 { let mut id=0usize; if (*hwmgr).ps.is_null(){return 0;} if psm_get_state_by_classification(hwmgr,PP_StateClassificationFlag_Boot,&mut id)==0 { psm_set_states(hwmgr,id) } else {-EINVAL} }
pub unsafe fn psm_set_performance_states(hwmgr: *mut pp_hwmgr) -> i32 { let mut id=0usize; if (*hwmgr).ps.is_null(){return 0;} if psm_get_ui_state(hwmgr,PP_StateUILabel_Performance,&mut id)==0 { psm_set_states(hwmgr,id) } else {-EINVAL} }

pub unsafe fn psm_set_user_performance_state(hwmgr:*mut pp_hwmgr, mut label_id:PP_StateUILabel, state:*mut *mut pp_power_state)->i32 {
    if (*hwmgr).ps.is_null(){return 0;} *state=(*hwmgr).ps;
    'restart: loop { for _ in 0..(*hwmgr).num_ps { if (**state).classification.ui_label & label_id != 0{return 0;} *state=(((*state) as usize)+(*hwmgr).ps_size as usize) as *mut pp_power_state; }
        match label_id { PP_StateUILabel_Battery|PP_StateUILabel_Balanced => {label_id=PP_StateUILabel_Performance; continue 'restart}, _=>return -EINVAL }
    }
}

// The remaining state-management helpers are supplied by the powerplay implementation.
unsafe fn power_state_management(hwmgr:*mut pp_hwmgr,new_ps:*mut pp_power_state){ let requested=if !new_ps.is_null(){new_ps}else{(*hwmgr).request_ps}; let current=(*hwmgr).current_ps; phm_apply_state_adjust_rules(hwmgr,requested,current); let mut equal=false; if !current.is_null(){phm_check_states_equal(hwmgr,&(*current).hardware,&(*requested).hardware,&mut equal);} if !equal||phm_check_smc_update_required_for_display_configuration(hwmgr){phm_set_power_state(hwmgr,&(*current).hardware,&(*requested).hardware); core::ptr::copy_nonoverlapping((*hwmgr).request_ps as *const u8,(*hwmgr).current_ps as *mut u8,(*hwmgr).ps_size as usize);} }

pub unsafe fn psm_adjust_power_state_dynamic(hwmgr:*mut pp_hwmgr,skip_display_settings:bool,new_ps:*mut pp_power_state)->i32{if (*hwmgr).not_vf{if !skip_display_settings{phm_display_configuration_changed(hwmgr);}if !(*hwmgr).ps.is_null(){power_state_management(hwmgr,new_ps)}else{phm_apply_clock_adjust_rules(hwmgr)}if !skip_display_settings{phm_notify_smc_display_config_after_ps_adjustment(hwmgr);}}if phm_force_dpm_levels(hwmgr,(*hwmgr).request_dpm_level)==0{(*hwmgr).dpm_level=(*hwmgr).request_dpm_level;}if (*hwmgr).dpm_level!=AMD_DPM_FORCED_LEVEL_MANUAL{let index=fls((*hwmgr).workload_mask);let index=if index>0&&index<=Workload_Policy_Max{index-1}else{0};let workload=[(*hwmgr).workload_setting[index as usize]];if (*hwmgr).power_profile_mode!=workload[0]&&!(*hwmgr).hwmgr_func.set_power_profile_mode.is_none(){(*(*hwmgr).hwmgr_func).set_power_profile_mode.unwrap()(hwmgr,workload.as_ptr(),0);}}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
