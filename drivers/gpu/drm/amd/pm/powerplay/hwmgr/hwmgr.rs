/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions.
 */

// C headers and symbols supplied by the surrounding kernel translation unit.

extern "C" {
    static mut ci_smu_funcs: pp_smumgr_func;
    static mut smu8_smu_funcs: pp_smumgr_func;
    static mut iceland_smu_funcs: pp_smumgr_func;
    static mut tonga_smu_funcs: pp_smumgr_func;
    static mut fiji_smu_funcs: pp_smumgr_func;
    static mut polaris10_smu_funcs: pp_smumgr_func;
    static mut vegam_smu_funcs: pp_smumgr_func;
    static mut vega10_smu_funcs: pp_smumgr_func;
    static mut vega12_smu_funcs: pp_smumgr_func;
    static mut smu10_smu_funcs: pp_smumgr_func;
    static mut vega20_smu_funcs: pp_smumgr_func;
    fn smu10_init_function_pointers(hwmgr: *mut pp_hwmgr) -> i32;
}

unsafe fn hwmgr_init_workload_prority(hwmgr: *mut pp_hwmgr) {
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_BOOTUP_DEFAULT as usize] = 0;
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_FULLSCREEN3D as usize] = 1;
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_POWERSAVING as usize] = 2;
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_VIDEO as usize] = 3;
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_VR as usize] = 4;
    (*hwmgr).workload_prority[PP_SMC_POWER_PROFILE_COMPUTE as usize] = 5;
    (*hwmgr).workload_setting[0] = PP_SMC_POWER_PROFILE_BOOTUP_DEFAULT;
    (*hwmgr).workload_setting[1] = PP_SMC_POWER_PROFILE_FULLSCREEN3D;
    (*hwmgr).workload_setting[2] = PP_SMC_POWER_PROFILE_POWERSAVING;
    (*hwmgr).workload_setting[3] = PP_SMC_POWER_PROFILE_VIDEO;
    (*hwmgr).workload_setting[4] = PP_SMC_POWER_PROFILE_VR;
    (*hwmgr).workload_setting[5] = PP_SMC_POWER_PROFILE_COMPUTE;
}

pub unsafe fn hwmgr_early_init(hwmgr: *mut pp_hwmgr) -> i32 {
    if hwmgr.is_null() { return -EINVAL; }
    (*hwmgr).usec_timeout = AMD_MAX_USEC_TIMEOUT;
    (*hwmgr).pp_table_version = PP_TABLE_V1;
    (*hwmgr).dpm_level = AMD_DPM_FORCED_LEVEL_AUTO;
    (*hwmgr).request_dpm_level = AMD_DPM_FORCED_LEVEL_AUTO;
    hwmgr_init_default_caps(hwmgr);
    hwmgr_set_user_specify_caps(hwmgr);
    (*hwmgr).fan_ctrl_is_in_default_mode = true;
    hwmgr_init_workload_prority(hwmgr);
    (*hwmgr).gfxoff_state_changed_by_workload = false;
    let adev = (*hwmgr).adev;
    match (*hwmgr).chip_family {
        AMDGPU_FAMILY_CI => {
            (*adev).pm.pp_feature &= !PP_GFXOFF_MASK;
            (*hwmgr).smumgr_funcs = &mut ci_smu_funcs;
            ci_set_asic_special_caps(hwmgr);
            (*hwmgr).feature_mask &= !(PP_VBI_TIME_SUPPORT_MASK | PP_ENABLE_GFX_CG_THRU_SMU | PP_GFXOFF_MASK);
            (*hwmgr).pp_table_version = PP_TABLE_V0;
            (*hwmgr).od_enabled = false;
            if (*hwmgr).chip_id == CHIP_BONAIRE && (*adev).pdev.subsystem_vendor == 0x106B {
                (*adev).pm.pp_feature &= !PP_MCLK_DPM_MASK;
                (*hwmgr).feature_mask &= !PP_MCLK_DPM_MASK;
            }
            smu7_init_function_pointers(hwmgr);
        }
        AMDGPU_FAMILY_CZ => {
            (*adev).pm.pp_feature &= !PP_GFXOFF_MASK;
            (*hwmgr).od_enabled = false;
            (*hwmgr).smumgr_funcs = &mut smu8_smu_funcs;
            (*hwmgr).feature_mask &= !PP_GFXOFF_MASK;
            smu8_init_function_pointers(hwmgr);
        }
        AMDGPU_FAMILY_VI => {
            (*adev).pm.pp_feature &= !PP_GFXOFF_MASK;
            (*hwmgr).feature_mask &= !PP_GFXOFF_MASK;
            match (*hwmgr).chip_id {
                CHIP_TOPAZ => { (*hwmgr).smumgr_funcs=&mut iceland_smu_funcs; topaz_set_asic_special_caps(hwmgr); (*hwmgr).feature_mask &= !(PP_VBI_TIME_SUPPORT_MASK|PP_ENABLE_GFX_CG_THRU_SMU); (*hwmgr).pp_table_version=PP_TABLE_V0; (*hwmgr).od_enabled=false; }
                CHIP_TONGA => { (*hwmgr).smumgr_funcs=&mut tonga_smu_funcs; tonga_set_asic_special_caps(hwmgr); (*hwmgr).feature_mask &= !PP_VBI_TIME_SUPPORT_MASK; }
                CHIP_FIJI => { (*hwmgr).smumgr_funcs=&mut fiji_smu_funcs; fiji_set_asic_special_caps(hwmgr); (*hwmgr).feature_mask &= !(PP_VBI_TIME_SUPPORT_MASK|PP_ENABLE_GFX_CG_THRU_SMU); }
                CHIP_POLARIS11 | CHIP_POLARIS10 | CHIP_POLARIS12 => { (*hwmgr).smumgr_funcs=&mut polaris10_smu_funcs; polaris_set_asic_special_caps(hwmgr); (*hwmgr).feature_mask &= !PP_UVD_HANDSHAKE_MASK; }
                CHIP_VEGAM => { (*hwmgr).smumgr_funcs=&mut vegam_smu_funcs; polaris_set_asic_special_caps(hwmgr); (*hwmgr).feature_mask &= !PP_UVD_HANDSHAKE_MASK; }
                _ => return -EINVAL,
            }
            smu7_init_function_pointers(hwmgr);
        }
        AMDGPU_FAMILY_AI => match (*hwmgr).chip_id {
            CHIP_VEGA10 => { (*adev).pm.pp_feature &= !PP_GFXOFF_MASK; (*hwmgr).feature_mask &= !PP_GFXOFF_MASK; (*hwmgr).smumgr_funcs=&mut vega10_smu_funcs; vega10_hwmgr_init(hwmgr); }
            CHIP_VEGA12 => { (*hwmgr).smumgr_funcs=&mut vega12_smu_funcs; vega12_hwmgr_init(hwmgr); }
            CHIP_VEGA20 => { (*adev).pm.pp_feature &= !PP_GFXOFF_MASK; (*hwmgr).feature_mask &= !PP_GFXOFF_MASK; (*hwmgr).smumgr_funcs=&mut vega20_smu_funcs; vega20_hwmgr_init(hwmgr); }
            _ => return -EINVAL,
        },
        AMDGPU_FAMILY_RV => match (*hwmgr).chip_id { CHIP_RAVEN => { (*hwmgr).od_enabled=false; (*hwmgr).smumgr_funcs=&mut smu10_smu_funcs; smu10_init_function_pointers(hwmgr); }, _ => return -EINVAL },
        _ => return -EINVAL,
    }
    0
}

pub unsafe fn hwmgr_sw_init(hwmgr: *mut pp_hwmgr) -> i32 {
    if hwmgr.is_null() || (*hwmgr).smumgr_funcs.is_null() || (*(*hwmgr).smumgr_funcs).smu_init.is_none() { return -EINVAL; }
    phm_register_irq_handlers(hwmgr);
    ((*(*hwmgr).smumgr_funcs).smu_init.unwrap())(hwmgr)
}
pub unsafe fn hwmgr_sw_fini(hwmgr: *mut pp_hwmgr) -> i32 { if !hwmgr.is_null() && !(*hwmgr).smumgr_funcs.is_null() && (*(*hwmgr).smumgr_funcs).smu_fini.is_some() { ((*(*hwmgr).smumgr_funcs).smu_fini.unwrap())(hwmgr); } 0 }

pub unsafe fn hwmgr_hw_init(hwmgr: *mut pp_hwmgr) -> i32 {
    let mut ret=0; (*hwmgr).pp_one_vf=amdgpu_sriov_is_pp_one_vf((*hwmgr).adev); (*hwmgr).pm_en=amdgpu_dpm && ((*hwmgr).not_vf || (*hwmgr).pp_one_vf); if !(*hwmgr).pm_en{return 0;}
    if (*hwmgr).pptable_func.is_null() || (*(*hwmgr).pptable_func).pptable_init.is_none() || (*hwmgr).hwmgr_func.is_null() || (*(*hwmgr).hwmgr_func).backend_init.is_none() { (*hwmgr).pm_en=false; return 0; }
    ret=((*(*hwmgr).pptable_func).pptable_init.unwrap())(hwmgr); if ret!=0{return ret;}
    (*(*hwmgr).adev).pm.no_fan=(*hwmgr).thermal_controller.fanInfo.bNoFan;
    ret=((*(*hwmgr).hwmgr_func).backend_init.unwrap())(hwmgr); if ret!=0 { if (*(*hwmgr).pptable_func).pptable_fini.is_some(){((*(*hwmgr).pptable_func).pptable_fini.unwrap())(hwmgr);} return ret; }
    if (*hwmgr).dyn_state.max_clock_voltage_on_dc.sclk==0 || (*hwmgr).dyn_state.max_clock_voltage_on_dc.mclk==0 {(*hwmgr).dyn_state.max_clock_voltage_on_dc=(*hwmgr).dyn_state.max_clock_voltage_on_ac;}
    ret=psm_init_power_state_table(hwmgr); if ret!=0{return ret;} ret=phm_setup_asic(hwmgr); if ret!=0{return ret;} ret=phm_enable_dynamic_state_management(hwmgr); if ret!=0{return ret;} ret=phm_start_thermal_controller(hwmgr); ret|=psm_set_performance_states(hwmgr); if ret!=0{return ret;} (*(*hwmgr).adev).pm.dpm_enabled=true; 0
}
pub unsafe fn hwmgr_hw_fini(hwmgr:*mut pp_hwmgr)->i32 { if hwmgr.is_null()||!(*hwmgr).pm_en||!(*hwmgr).not_vf{return 0;} phm_stop_thermal_controller(hwmgr); psm_set_boot_states(hwmgr); psm_adjust_power_state_dynamic(hwmgr,true,core::ptr::null_mut()); phm_disable_dynamic_state_management(hwmgr); phm_disable_clock_power_gatings(hwmgr); if (*hwmgr).hwmgr_func.is_null()==false&&(*(*hwmgr).hwmgr_func).backend_fini.is_some(){((*(*hwmgr).hwmgr_func).backend_fini.unwrap())(hwmgr);} if (*hwmgr).pptable_func.is_null()==false&&(*(*hwmgr).pptable_func).pptable_fini.is_some(){((*(*hwmgr).pptable_func).pptable_fini.unwrap())(hwmgr);} psm_fini_power_state_table(hwmgr) }
pub unsafe fn hwmgr_suspend(hwmgr:*mut pp_hwmgr)->i32 { if hwmgr.is_null()||!(*hwmgr).pm_en||!(*hwmgr).not_vf{return 0;} phm_disable_smc_firmware_ctf(hwmgr); let mut r=psm_set_boot_states(hwmgr); if r!=0{return r;} r=psm_adjust_power_state_dynamic(hwmgr,true,core::ptr::null_mut()); if r!=0{return r;} phm_power_down_asic(hwmgr) }
pub unsafe fn hwmgr_resume(hwmgr:*mut pp_hwmgr)->i32 { if hwmgr.is_null(){return -EINVAL;} if !(*hwmgr).not_vf||!(*hwmgr).pm_en{return 0;} let mut r=phm_setup_asic(hwmgr); if r!=0{return r;} r=phm_enable_dynamic_state_management(hwmgr); if r!=0{return r;} r=phm_start_thermal_controller(hwmgr); r|=psm_set_performance_states(hwmgr); if r!=0{return r;} psm_adjust_power_state_dynamic(hwmgr,false,core::ptr::null_mut()) }

unsafe fn power_state_convert(state: amd_pm_state_type) -> PP_StateUILabel { match state { POWER_STATE_TYPE_BATTERY=>PP_StateUILabel_Battery, POWER_STATE_TYPE_BALANCED=>PP_StateUILabel_Balanced, POWER_STATE_TYPE_PERFORMANCE=>PP_StateUILabel_Performance, _=>PP_StateUILabel_None } }
pub unsafe fn hwmgr_handle_task(hwmgr:*mut pp_hwmgr, task_id:amd_pp_task, user_state:*mut amd_pm_state_type)->i32 { if hwmgr.is_null(){return -EINVAL;} match task_id { AMD_PP_TASK_DISPLAY_CONFIG_CHANGE=>{if !(*hwmgr).not_vf{return 0;} let mut r=phm_pre_display_configuration_changed(hwmgr); if r!=0{return r;} r=phm_set_cpu_power_state(hwmgr); if r!=0{return r;} r=psm_set_performance_states(hwmgr); if r!=0{return r;} psm_adjust_power_state_dynamic(hwmgr,false,core::ptr::null_mut())}, AMD_PP_TASK_ENABLE_USER_STATE=>{if !(*hwmgr).not_vf{return 0;} if user_state.is_null(){return -EINVAL;} let mut ps=core::ptr::null_mut(); let mut r=psm_set_user_performance_state(hwmgr,power_state_convert(*user_state),&mut ps); if r!=0{return r;} psm_adjust_power_state_dynamic(hwmgr,true,ps)}, AMD_PP_TASK_COMPLETE_INIT|AMD_PP_TASK_READJUST_POWER_STATE=>psm_adjust_power_state_dynamic(hwmgr,true,core::ptr::null_mut()), _=>0 } }

pub unsafe fn hwmgr_init_default_caps(hwmgr:*mut pp_hwmgr) { phm_cap_unset((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_PCIEPerformanceRequest); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_UVDDPM); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_VCEDPM); /* CONFIG_ACPI conditional preserved from C source. */ if amdgpu_acpi_is_pcie_performance_request_supported((*hwmgr).adev){phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_PCIEPerformanceRequest);} phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_DynamicPatchPowerState); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_EnableSMU7ThermalManagement); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_DynamicPowerManagement); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_SMC); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_DynamicUVDState); phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_FanSpeedInTableIsRPM); }
pub unsafe fn hwmgr_set_user_specify_caps(hwmgr:*mut pp_hwmgr)->i32 { if (*hwmgr).feature_mask&PP_SCLK_DEEP_SLEEP_MASK!=0{phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_SclkDeepSleep);}else{phm_cap_unset((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_SclkDeepSleep);} if (*hwmgr).feature_mask&PP_POWER_CONTAINMENT_MASK!=0{phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_PowerContainment);phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_CAC);}else{phm_cap_unset((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_PowerContainment);phm_cap_unset((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_CAC);} if (*hwmgr).feature_mask&PP_OVERDRIVE_MASK!=0{(*hwmgr).od_enabled=true;} 0 }

unsafe fn cap_set_special(hwmgr:*mut pp_hwmgr, set:&[u32], unset:&[u32])->i32 { for c in set {phm_cap_set((*hwmgr).platform_descriptor.platformCaps,*c);} for c in unset {phm_cap_unset((*hwmgr).platform_descriptor.platformCaps,*c);} 0 }
pub unsafe fn polaris_set_asic_special_caps(hwmgr:*mut pp_hwmgr)->i32 { let mut s=[PHM_PlatformCaps_EVV,PHM_PlatformCaps_SQRamping,PHM_PlatformCaps_RegulatorHot,PHM_PlatformCaps_MemorySpreadSpectrumSupport,PHM_PlatformCaps_EngineSpreadSpectrumSupport,PHM_PlatformCaps_AutomaticDCTransition]; cap_set_special(hwmgr,&s,&[]); if (((*hwmgr).chip_id==CHIP_POLARIS11)&&!(*hwmgr).is_kicker)||(*hwmgr).chip_id==CHIP_POLARIS12{phm_cap_set((*hwmgr).platform_descriptor.platformCaps,PHM_PlatformCaps_SPLLShutdownSupport);} if (*hwmgr).chip_id!=CHIP_POLARIS11{cap_set_special(hwmgr,&[PHM_PlatformCaps_DBRamping,PHM_PlatformCaps_TDRamping,PHM_PlatformCaps_TCPRamping],&[]);} 0 }
pub unsafe fn fiji_set_asic_special_caps(hwmgr:*mut pp_hwmgr)->i32 {cap_set_special(hwmgr,&[PHM_PlatformCaps_EVV],&[PHM_PlatformCaps_SQRamping,PHM_PlatformCaps_DBRamping,PHM_PlatformCaps_TDRamping,PHM_PlatformCaps_TCPRamping])}
pub unsafe fn tonga_set_asic_special_caps(hwmgr:*mut pp_hwmgr)->i32 {cap_set_special(hwmgr,&[PHM_PlatformCaps_EVV],&[PHM_PlatformCaps_SQRamping,PHM_PlatformCaps_DBRamping,PHM_PlatformCaps_TDRamping,PHM_PlatformCaps_TCPRamping,PHM_PlatformCaps_UVDPowerGating,PHM_PlatformCaps_VCEPowerGating])}
pub unsafe fn topaz_set_asic_special_caps(hwmgr:*mut pp_hwmgr)->i32 {fiji_set_asic_special_caps(hwmgr)}
pub unsafe fn ci_set_asic_special_caps(hwmgr:*mut pp_hwmgr)->i32 {cap_set_special(hwmgr,&[PHM_PlatformCaps_MemorySpreadSpectrumSupport,PHM_PlatformCaps_EngineSpreadSpectrumSupport],&[PHM_PlatformCaps_SQRamping,PHM_PlatformCaps_DBRamping,PHM_PlatformCaps_TDRamping,PHM_PlatformCaps_TCPRamping])}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
