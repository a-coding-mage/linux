/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit.

static mut df_v1_7_channel_number: [u32; 9] = [1, 2, 0, 4, 0, 8, 0, 16, 2];

unsafe fn df_v1_7_sw_init(adev: *mut amdgpu_device) {
    (*adev).df.hash_status.hash_64k = false;
    (*adev).df.hash_status.hash_2m = false;
    (*adev).df.hash_status.hash_1g = false;
}

unsafe fn df_v1_7_sw_fini(_adev: *mut amdgpu_device) {}

unsafe fn df_v1_7_enable_broadcast_mode(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp: u32;

    if enable {
        tmp = RREG32_SOC15(DF, 0, mmFabricConfigAccessControl);
        tmp &= !FabricConfigAccessControl__CfgRegInstAccEn_MASK;
        WREG32_SOC15(DF, 0, mmFabricConfigAccessControl, tmp);
    } else {
        WREG32_SOC15(
            DF,
            0,
            mmFabricConfigAccessControl,
            mmFabricConfigAccessControl_DEFAULT,
        );
    }
}

unsafe fn df_v1_7_get_fb_channel_number(adev: *mut amdgpu_device) -> u32 {
    let mut tmp = RREG32_SOC15(DF, 0, mmDF_CS_AON0_DramBaseAddress0);
    tmp &= DF_CS_AON0_DramBaseAddress0__IntLvNumChan_MASK;
    tmp >>= DF_CS_AON0_DramBaseAddress0__IntLvNumChan__SHIFT;
    tmp
}

unsafe fn df_v1_7_get_hbm_channel_number(adev: *mut amdgpu_device) -> u32 {
    let mut fb_channel_number: i32 = ((*adev).df.funcs.get_fb_channel_number)(adev) as i32;
    if fb_channel_number >= 9 {
        fb_channel_number = 0;
    }
    df_v1_7_channel_number[fb_channel_number as usize]
}

unsafe fn df_v1_7_update_medium_grain_clock_gating(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    let mut tmp: u32;

    // Put DF on broadcast mode
    ((*adev).df.funcs.enable_broadcast_mode)(adev, true);

    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_DF_MGCG) != 0 {
        tmp = RREG32_SOC15(DF, 0, mmDF_PIE_AON0_DfGlobalClkGater);
        tmp &= !DF_PIE_AON0_DfGlobalClkGater__MGCGMode_MASK;
        tmp |= DF_V1_7_MGCG_ENABLE_15_CYCLE_DELAY;
        WREG32_SOC15(DF, 0, mmDF_PIE_AON0_DfGlobalClkGater, tmp);
    } else {
        tmp = RREG32_SOC15(DF, 0, mmDF_PIE_AON0_DfGlobalClkGater);
        tmp &= !DF_PIE_AON0_DfGlobalClkGater__MGCGMode_MASK;
        tmp |= DF_V1_7_MGCG_DISABLE;
        WREG32_SOC15(DF, 0, mmDF_PIE_AON0_DfGlobalClkGater, tmp);
    }

    // Exit broadcast mode
    ((*adev).df.funcs.enable_broadcast_mode)(adev, false);
}

unsafe fn df_v1_7_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    // AMD_CG_SUPPORT_DF_MGCG
    let tmp = RREG32_SOC15(DF, 0, mmDF_PIE_AON0_DfGlobalClkGater);
    if (tmp & DF_V1_7_MGCG_ENABLE_15_CYCLE_DELAY) != 0 {
        *flags |= AMD_CG_SUPPORT_DF_MGCG as u64;
    }
}

unsafe fn df_v1_7_enable_ecc_force_par_wr_rmw(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    WREG32_FIELD15(DF, 0, DF_CS_AON0_CoherentSlaveModeCtrlA0, ForceParWrRMW, enable);
}

const df_v1_7_funcs: amdgpu_df_funcs = amdgpu_df_funcs {
    sw_init: Some(df_v1_7_sw_init),
    sw_fini: Some(df_v1_7_sw_fini),
    enable_broadcast_mode: Some(df_v1_7_enable_broadcast_mode),
    get_fb_channel_number: Some(df_v1_7_get_fb_channel_number),
    get_hbm_channel_number: Some(df_v1_7_get_hbm_channel_number),
    update_medium_grain_clock_gating: Some(df_v1_7_update_medium_grain_clock_gating),
    get_clockgating_state: Some(df_v1_7_get_clockgating_state),
    enable_ecc_force_par_wr_rmw: Some(df_v1_7_enable_ecc_force_par_wr_rmw),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
