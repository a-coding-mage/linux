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

unsafe fn smu7_enable_disable_uvd_dpm(hwmgr: *mut pp_hwmgr, enable: bool) -> i32 {
    smum_send_msg_to_smc(hwmgr, if enable { PPSMC_MSG_UVDDPM_Enable } else { PPSMC_MSG_UVDDPM_Disable }, core::ptr::null_mut())
}

unsafe fn smu7_enable_disable_vce_dpm(hwmgr: *mut pp_hwmgr, enable: bool) -> i32 {
    smum_send_msg_to_smc(hwmgr, if enable { PPSMC_MSG_VCEDPM_Enable } else { PPSMC_MSG_VCEDPM_Disable }, core::ptr::null_mut())
}

unsafe fn smu7_update_uvd_dpm(hwmgr: *mut pp_hwmgr, bgate: bool) -> i32 {
    if !bgate { smum_update_smc_table(hwmgr, SMU_UVD_TABLE); }
    smu7_enable_disable_uvd_dpm(hwmgr, !bgate)
}

unsafe fn smu7_update_vce_dpm(hwmgr: *mut pp_hwmgr, bgate: bool) -> i32 {
    if !bgate { smum_update_smc_table(hwmgr, SMU_VCE_TABLE); }
    smu7_enable_disable_vce_dpm(hwmgr, !bgate)
}

unsafe fn smu7_powerdown_uvd(hwmgr: *mut pp_hwmgr) -> i32 {
    if phm_cf_want_uvd_power_gating(hwmgr) { return smum_send_msg_to_smc(hwmgr, PPSMC_MSG_UVDPowerOFF, core::ptr::null_mut()); }
    0
}

unsafe fn smu7_powerup_uvd(hwmgr: *mut pp_hwmgr) -> i32 {
    if phm_cf_want_uvd_power_gating(hwmgr) {
        let value = if phm_cap_enabled((*(*hwmgr).platform_descriptor.platformCaps), PHM_PlatformCaps_UVDDynamicPowerGating) { 1 } else { 0 };
        return smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_MSG_UVDPowerON, value, core::ptr::null_mut());
    }
    0
}

unsafe fn smu7_powerdown_vce(hwmgr: *mut pp_hwmgr) -> i32 {
    if phm_cf_want_vce_power_gating(hwmgr) { return smum_send_msg_to_smc(hwmgr, PPSMC_MSG_VCEPowerOFF, core::ptr::null_mut()); }
    0
}

unsafe fn smu7_powerup_vce(hwmgr: *mut pp_hwmgr) -> i32 {
    if phm_cf_want_vce_power_gating(hwmgr) { return smum_send_msg_to_smc(hwmgr, PPSMC_MSG_VCEPowerON, core::ptr::null_mut()); }
    0
}

pub unsafe fn smu7_disable_clock_power_gating(hwmgr: *mut pp_hwmgr) -> i32 {
    let data = (*hwmgr).backend as *mut smu7_hwmgr;
    (*data).uvd_power_gated = false;
    (*data).vce_power_gated = false;
    smu7_powerup_uvd(hwmgr);
    smu7_powerup_vce(hwmgr);
    0
}

pub unsafe fn smu7_powergate_uvd(hwmgr: *mut pp_hwmgr, bgate: bool) {
    let data = (*hwmgr).backend as *mut smu7_hwmgr;
    (*data).uvd_power_gated = bgate;
    if bgate {
        amdgpu_device_ip_set_powergating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_UVD, AMD_PG_STATE_GATE);
        amdgpu_device_ip_set_clockgating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_UVD, AMD_CG_STATE_GATE);
        smu7_update_uvd_dpm(hwmgr, true);
        smu7_powerdown_uvd(hwmgr);
    } else {
        smu7_powerup_uvd(hwmgr);
        amdgpu_device_ip_set_clockgating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_UVD, AMD_CG_STATE_UNGATE);
        amdgpu_device_ip_set_powergating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_UVD, AMD_PG_STATE_UNGATE);
        smu7_update_uvd_dpm(hwmgr, false);
    }
}

pub unsafe fn smu7_powergate_vce(hwmgr: *mut pp_hwmgr, bgate: bool) {
    let data = (*hwmgr).backend as *mut smu7_hwmgr;
    (*data).vce_power_gated = bgate;
    if bgate {
        amdgpu_device_ip_set_powergating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_VCE, AMD_PG_STATE_GATE);
        amdgpu_device_ip_set_clockgating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_VCE, AMD_CG_STATE_GATE);
        smu7_update_vce_dpm(hwmgr, true);
        smu7_powerdown_vce(hwmgr);
    } else {
        smu7_powerup_vce(hwmgr);
        amdgpu_device_ip_set_clockgating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_VCE, AMD_CG_STATE_UNGATE);
        amdgpu_device_ip_set_powergating_state((*hwmgr).adev, AMD_IP_BLOCK_TYPE_VCE, AMD_PG_STATE_UNGATE);
        smu7_update_vce_dpm(hwmgr, false);
    }
}

pub unsafe fn smu7_update_clock_gatings(hwmgr: *mut pp_hwmgr, msg_id: *const u32) -> i32 {
    if (*hwmgr).feature_mask & PP_ENABLE_GFX_CG_THRU_SMU == 0 { return 0; }
    let mut msg: PPSMC_Msg;
    let mut value: u32;
    let group = (*msg_id & PP_GROUP_MASK) >> PP_GROUP_SHIFT;
    match group {
        PP_GROUP_GFX => match (*msg_id & PP_BLOCK_MASK) >> PP_BLOCK_SHIFT {
            PP_BLOCK_GFX_CG => {
                if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_CGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } }
                if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_CGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } }
            }
            PP_BLOCK_GFX_3D => {
                if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_3DCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } }
                if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_3DLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } }
            }
            PP_BLOCK_GFX_RLC => { if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_RLC_LS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_GFX_CP => { if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_GFX_CP_LS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_GFX_MG => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_CPF_MGCG_MASK | CG_RLC_MGCG_MASK | CG_GFX_OTHERS_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            _ => return -EINVAL,
        },
        PP_GROUP_SYS => match (*msg_id & PP_BLOCK_MASK) >> PP_BLOCK_SHIFT {
            PP_BLOCK_SYS_BIF => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_BIF_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_BIF_MGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_SYS_MC => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_MC_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_MC_MGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_SYS_DRM => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_DRM_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_DRM_MGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_SYS_HDP => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_HDP_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_HDP_MGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_SYS_SDMA => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_SDMA_MGCG_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } if PP_STATE_SUPPORT_LS & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_LS != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_SDMA_MGLS_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            PP_BLOCK_SYS_ROM => { if PP_STATE_SUPPORT_CG & *msg_id != 0 { msg = if (*msg_id & PP_STATE_MASK) & PP_STATE_CG != 0 { PPSMC_MSG_EnableClockGatingFeature } else { PPSMC_MSG_DisableClockGatingFeature }; value = CG_SYS_ROM_MASK; if smum_send_msg_to_smc_with_parameter(hwmgr, msg, value, core::ptr::null_mut()) != 0 { return -EINVAL; } } }
            _ => return -EINVAL,
        },
        _ => return -EINVAL,
    }
    0
}

/* This function is for Polaris11 only for now,
 * Powerplay will only control the static per CU Power Gating.
 * Dynamic per CU Power Gating will be done in gfx.
 */
pub unsafe fn smu7_powergate_gfx(hwmgr: *mut pp_hwmgr, enable: bool) -> i32 {
    let adev = (*hwmgr).adev;
    if enable { smum_send_msg_to_smc_with_parameter(hwmgr, PPSMC_MSG_GFX_CU_PG_ENABLE, (*adev).gfx.cu_info.number, core::ptr::null_mut()) }
    else { smum_send_msg_to_smc(hwmgr, PPSMC_MSG_GFX_CU_PG_DISABLE, core::ptr::null_mut()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
