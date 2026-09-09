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

// Dependencies supplied by the surrounding translation.

pub const AMD_MAX_USEC_TIMEOUT: u32 = 1_000_000; // 1000 ms

#[repr(C)]
pub struct amdgpu_ip_block {
    _private: [u8; 0],
}

/* Chip flags */
#[repr(u64)]
pub enum amd_chip_flags {
    AMD_ASIC_MASK = 0x0000ffff,
    AMD_FLAGS_MASK = 0xffff0000,
    AMD_IS_MOBILITY = 0x00010000,
    AMD_IS_APU = 0x00020000,
    AMD_IS_PX = 0x00040000,
    AMD_EXP_HW_SUPPORT = 0x00080000,
}

#[repr(u64)]
pub enum amd_apu_flags {
    AMD_APU_IS_RAVEN = 0x00000001,
    AMD_APU_IS_RAVEN2 = 0x00000002,
    AMD_APU_IS_PICASSO = 0x00000004,
    AMD_APU_IS_RENOIR = 0x00000008,
    AMD_APU_IS_GREEN_SARDINE = 0x00000010,
    AMD_APU_IS_VANGOGH = 0x00000020,
    AMD_APU_IS_CYAN_SKILLFISH2 = 0x00000040,
}

/* IP Blocks and IP block classifications are preserved as C-compatible enums. */
#[repr(i32)]
pub enum amd_ip_block_type {
    AMD_IP_BLOCK_TYPE_COMMON,
    AMD_IP_BLOCK_TYPE_GMC,
    AMD_IP_BLOCK_TYPE_IH,
    AMD_IP_BLOCK_TYPE_SMC,
    AMD_IP_BLOCK_TYPE_PSP,
    AMD_IP_BLOCK_TYPE_DCE,
    AMD_IP_BLOCK_TYPE_GFX,
    AMD_IP_BLOCK_TYPE_SDMA,
    AMD_IP_BLOCK_TYPE_UVD,
    AMD_IP_BLOCK_TYPE_VCE,
    AMD_IP_BLOCK_TYPE_ACP,
    AMD_IP_BLOCK_TYPE_VCN,
    AMD_IP_BLOCK_TYPE_MES,
    AMD_IP_BLOCK_TYPE_JPEG,
    AMD_IP_BLOCK_TYPE_VPE,
    AMD_IP_BLOCK_TYPE_UMSCH_MM,
    AMD_IP_BLOCK_TYPE_ISP,
    AMD_IP_BLOCK_TYPE_RAS,
    AMD_IP_BLOCK_TYPE_NUM,
}

#[repr(i32)]
pub enum amd_clockgating_state { AMD_CG_STATE_GATE = 0, AMD_CG_STATE_UNGATE }

#[repr(i32)]
pub enum amd_powergating_state { AMD_PG_STATE_GATE = 0, AMD_PG_STATE_UNGATE }

/* CG flags */
pub const AMD_CG_SUPPORT_GFX_MGCG: u64 = 1u64 << 0;
pub const AMD_CG_SUPPORT_GFX_MGLS: u64 = 1u64 << 1;
pub const AMD_CG_SUPPORT_GFX_CGCG: u64 = 1u64 << 2;
pub const AMD_CG_SUPPORT_GFX_CGLS: u64 = 1u64 << 3;
pub const AMD_CG_SUPPORT_GFX_CGTS: u64 = 1u64 << 4;
pub const AMD_CG_SUPPORT_GFX_CGTS_LS: u64 = 1u64 << 5;
pub const AMD_CG_SUPPORT_GFX_CP_LS: u64 = 1u64 << 6;
pub const AMD_CG_SUPPORT_GFX_RLC_LS: u64 = 1u64 << 7;
pub const AMD_CG_SUPPORT_MC_LS: u64 = 1u64 << 8;
pub const AMD_CG_SUPPORT_MC_MGCG: u64 = 1u64 << 9;
pub const AMD_CG_SUPPORT_SDMA_LS: u64 = 1u64 << 10;
pub const AMD_CG_SUPPORT_SDMA_MGCG: u64 = 1u64 << 11;
pub const AMD_CG_SUPPORT_BIF_LS: u64 = 1u64 << 12;
pub const AMD_CG_SUPPORT_UVD_MGCG: u64 = 1u64 << 13;
pub const AMD_CG_SUPPORT_VCE_MGCG: u64 = 1u64 << 14;
pub const AMD_CG_SUPPORT_HDP_LS: u64 = 1u64 << 15;
pub const AMD_CG_SUPPORT_HDP_MGCG: u64 = 1u64 << 16;
pub const AMD_CG_SUPPORT_ROM_MGCG: u64 = 1u64 << 17;
pub const AMD_CG_SUPPORT_DRM_LS: u64 = 1u64 << 18;
pub const AMD_CG_SUPPORT_BIF_MGCG: u64 = 1u64 << 19;
pub const AMD_CG_SUPPORT_GFX_3D_CGCG: u64 = 1u64 << 20;
pub const AMD_CG_SUPPORT_GFX_3D_CGLS: u64 = 1u64 << 21;
pub const AMD_CG_SUPPORT_DRM_MGCG: u64 = 1u64 << 22;
pub const AMD_CG_SUPPORT_DF_MGCG: u64 = 1u64 << 23;
pub const AMD_CG_SUPPORT_VCN_MGCG: u64 = 1u64 << 24;
pub const AMD_CG_SUPPORT_HDP_DS: u64 = 1u64 << 25;
pub const AMD_CG_SUPPORT_HDP_SD: u64 = 1u64 << 26;
pub const AMD_CG_SUPPORT_IH_CG: u64 = 1u64 << 27;
pub const AMD_CG_SUPPORT_ATHUB_LS: u64 = 1u64 << 28;
pub const AMD_CG_SUPPORT_ATHUB_MGCG: u64 = 1u64 << 29;
pub const AMD_CG_SUPPORT_JPEG_MGCG: u64 = 1u64 << 30;
pub const AMD_CG_SUPPORT_GFX_FGCG: u64 = 1u64 << 31;
pub const AMD_CG_SUPPORT_REPEATER_FGCG: u64 = 1u64 << 32;
pub const AMD_CG_SUPPORT_GFX_PERF_CLK: u64 = 1u64 << 33;

/* PG flags */
pub const AMD_PG_SUPPORT_GFX_PG: i32 = 1 << 0;
pub const AMD_PG_SUPPORT_GFX_SMG: i32 = 1 << 1;
pub const AMD_PG_SUPPORT_GFX_DMG: i32 = 1 << 2;
pub const AMD_PG_SUPPORT_UVD: i32 = 1 << 3;
pub const AMD_PG_SUPPORT_VCE: i32 = 1 << 4;
pub const AMD_PG_SUPPORT_CP: i32 = 1 << 5;
pub const AMD_PG_SUPPORT_GDS: i32 = 1 << 6;
pub const AMD_PG_SUPPORT_RLC_SMU_HS: i32 = 1 << 7;
pub const AMD_PG_SUPPORT_SDMA: i32 = 1 << 8;
pub const AMD_PG_SUPPORT_ACP: i32 = 1 << 9;
pub const AMD_PG_SUPPORT_SAMU: i32 = 1 << 10;
pub const AMD_PG_SUPPORT_GFX_QUICK_MG: i32 = 1 << 11;
pub const AMD_PG_SUPPORT_GFX_PIPELINE: i32 = 1 << 12;
pub const AMD_PG_SUPPORT_MMHUB: i32 = 1 << 13;
pub const AMD_PG_SUPPORT_VCN: i32 = 1 << 14;
pub const AMD_PG_SUPPORT_VCN_DPG: i32 = 1 << 15;
pub const AMD_PG_SUPPORT_ATHUB: i32 = 1 << 16;
pub const AMD_PG_SUPPORT_JPEG: i32 = 1 << 17;
pub const AMD_PG_SUPPORT_IH_SRAM_PG: i32 = 1 << 18;
pub const AMD_PG_SUPPORT_JPEG_DPG: i32 = 1 << 19;

// Feature-mask enums retain the source values and names.
#[repr(u32)]
pub enum PP_FEATURE_MASK {
    PP_SCLK_DPM_MASK = 0x1, PP_MCLK_DPM_MASK = 0x2, PP_PCIE_DPM_MASK = 0x4,
    PP_SCLK_DEEP_SLEEP_MASK = 0x8, PP_POWER_CONTAINMENT_MASK = 0x10,
    PP_UVD_HANDSHAKE_MASK = 0x20, PP_SMC_VOLTAGE_CONTROL_MASK = 0x40,
    PP_VBI_TIME_SUPPORT_MASK = 0x80, PP_ULV_MASK = 0x100,
    PP_ENABLE_GFX_CG_THRU_SMU = 0x200, PP_CLOCK_STRETCH_MASK = 0x400,
    PP_OD_FUZZY_FAN_CONTROL_MASK = 0x800, PP_SOCCLK_DPM_MASK = 0x1000,
    PP_DCEFCLK_DPM_MASK = 0x2000, PP_OVERDRIVE_MASK = 0x4000,
    PP_GFXOFF_MASK = 0x8000, PP_ACG_MASK = 0x10000, PP_STUTTER_MODE = 0x20000,
    PP_AVFS_MASK = 0x40000, PP_GFX_DCS_MASK = 0x80000,
}

#[repr(u32)]
pub enum amd_harvest_ip_mask { AMD_HARVEST_IP_VCN_MASK = 0x1, AMD_HARVEST_IP_JPEG_MASK = 0x2, AMD_HARVEST_IP_DMU_MASK = 0x4 }

#[repr(u32)]
pub enum DC_FEATURE_MASK {
    DC_FBC_MASK = 1 << 0, DC_MULTI_MON_PP_MCLK_SWITCH_MASK = 1 << 1,
    DC_DISABLE_FRACTIONAL_PWM_MASK = 1 << 2, DC_PSR_MASK = 1 << 3,
    DC_EDP_NO_POWER_SEQUENCING = 1 << 4, DC_DISABLE_LTTPR_DP1_4A = 1 << 5,
    DC_DISABLE_LTTPR_DP2_0 = 1 << 6, DC_PSR_ALLOW_SMU_OPT = 1 << 7,
    DC_PSR_ALLOW_MULTI_DISP_OPT = 1 << 8, DC_REPLAY_MASK = 1 << 9,
    DC_FRL_MASK = 1 << 10,
}

#[repr(u32)]
pub enum DC_DEBUG_MASK {
    DC_DISABLE_PIPE_SPLIT=0x1, DC_DISABLE_STUTTER=0x2, DC_DISABLE_DSC=0x4,
    DC_DISABLE_CLOCK_GATING=0x8, DC_DISABLE_PSR=0x10,
    DC_FORCE_SUBVP_MCLK_SWITCH=0x20, DC_DISABLE_MPO=0x40,
    DC_ENABLE_DPIA_TRACE=0x80, DC_ENABLE_DML2=0x100, DC_DISABLE_PSR_SU=0x200,
    DC_DISABLE_REPLAY=0x400, DC_DISABLE_IPS=0x800, DC_DISABLE_IPS_DYNAMIC=0x1000,
    DC_DISABLE_IPS2_DYNAMIC=0x2000, DC_FORCE_IPS_ENABLE=0x4000,
    DC_DISABLE_ACPI_EDID=0x8000, DC_DISABLE_HDMI_CEC=0x10000,
    DC_DISABLE_SUBVP_FAMS=0x20000, DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE=0x40000,
    DC_HDCP_LC_FORCE_FW_ENABLE=0x80000, DC_HDCP_LC_ENABLE_SW_FALLBACK=0x100000,
    DC_SKIP_DETECTION_LT=0x200000,
}

pub enum amd_dpm_forced_level {}

#[repr(C)]
pub struct drm_printer { _private: [u8; 0] }

#[repr(C)]
pub struct amd_ip_funcs {
    pub name: *mut core::ffi::c_char,
    pub early_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub late_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub sw_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub sw_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub early_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub hw_init: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub hw_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub late_fini: Option<unsafe extern "C" fn(*mut amdgpu_ip_block)>,
    pub prepare_suspend: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub complete: Option<unsafe extern "C" fn(*mut amdgpu_ip_block)>,
    pub is_idle: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> bool>,
    pub wait_for_idle: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub soft_reset: Option<unsafe extern "C" fn(*mut amdgpu_ip_block) -> i32>,
    pub set_clockgating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, amd_clockgating_state) -> i32>,
    pub set_powergating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, amd_powergating_state) -> i32>,
    pub get_clockgating_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, *mut u64)>,
    pub dump_ip_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block)>,
    pub print_ip_state: Option<unsafe extern "C" fn(*mut amdgpu_ip_block, *mut drm_printer)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
