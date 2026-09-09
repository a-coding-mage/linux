/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding AMDGPU translation unit:
// amdgpu.h, hdp_v5_2.h, hdp/hdp_5_2_1_offset.h,
// hdp/hdp_5_2_1_sh_mask.h, and uapi/linux/kfd_ioctl.h.

unsafe fn hdp_v5_2_flush_hdp(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) {
    if ring.is_null() || (*ring).funcs.emit_wreg.is_none() {
        WREG32_NO_KIQ(((*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL) >> 2, 0);
        if amdgpu_sriov_vf(adev) {
            RREG32_NO_KIQ(((*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL) >> 2);
        } else {
            if (*adev).nbio.funcs.get_memsize.is_some() {
                ((*adev).nbio.funcs.get_memsize.unwrap())(adev);
            }
        }
    } else {
        amdgpu_ring_emit_wreg(ring,
            ((*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL) >> 2,
            0);
    }
}

unsafe fn hdp_v5_2_update_mem_power_gating(adev: *mut amdgpu_device, enable: bool) {
    let mut hdp_clk_cntl: u32;
    let mut hdp_mem_pwr_cntl: u32;

    if ((*adev).cg_flags & (AMD_CG_SUPPORT_HDP_LS | AMD_CG_SUPPORT_HDP_DS |
        AMD_CG_SUPPORT_HDP_SD)) == 0 { return; }

    hdp_clk_cntl = RREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL);
    hdp_mem_pwr_cntl = RREG32_SOC15!(HDP, 0, regHDP_MEM_POWER_CTRL);
    hdp_clk_cntl = REG_SET_FIELD!(hdp_clk_cntl, HDP_CLK_CNTL, ATOMIC_MEM_CLK_SOFT_OVERRIDE, 1);
    hdp_clk_cntl = REG_SET_FIELD!(hdp_clk_cntl, HDP_CLK_CNTL, RC_MEM_CLK_SOFT_OVERRIDE, 1);
    WREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL, hdp_clk_cntl);

    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_CTRL_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_LS_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_DS_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_SD_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_CTRL_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_LS_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_DS_EN, 0);
    hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_SD_EN, 0);
    WREG32_SOC15!(HDP, 0, regHDP_MEM_POWER_CTRL, hdp_mem_pwr_cntl);

    if enable {
        if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_SD) != 0 {
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_SD_EN, 1);
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_SD_EN, 1);
        } else if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_LS) != 0 {
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_LS_EN, 1);
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_LS_EN, 1);
        } else if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_DS) != 0 {
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_DS_EN, 1);
            hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_DS_EN, 1);
        }
        hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, ATOMIC_MEM_POWER_CTRL_EN, 1);
        hdp_mem_pwr_cntl = REG_SET_FIELD!(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL, RC_MEM_POWER_CTRL_EN, 1);
        WREG32_SOC15!(HDP, 0, regHDP_MEM_POWER_CTRL, hdp_mem_pwr_cntl);
    }
    hdp_clk_cntl = REG_SET_FIELD!(hdp_clk_cntl, HDP_CLK_CNTL, ATOMIC_MEM_CLK_SOFT_OVERRIDE, 0);
    hdp_clk_cntl = REG_SET_FIELD!(hdp_clk_cntl, HDP_CLK_CNTL, RC_MEM_CLK_SOFT_OVERRIDE, 0);
    WREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL, hdp_clk_cntl);
}

unsafe fn hdp_v5_2_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_MGCG) == 0 { return; }
    let mut hdp_clk_cntl = RREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL);
    let mask = HDP_CLK_CNTL__ATOMIC_MEM_CLK_SOFT_OVERRIDE_MASK |
        HDP_CLK_CNTL__RC_MEM_CLK_SOFT_OVERRIDE_MASK | HDP_CLK_CNTL__DBUS_CLK_SOFT_OVERRIDE_MASK |
        HDP_CLK_CNTL__DYN_CLK_SOFT_OVERRIDE_MASK | HDP_CLK_CNTL__XDP_REG_CLK_SOFT_OVERRIDE_MASK |
        HDP_CLK_CNTL__HDP_REG_CLK_SOFT_OVERRIDE_MASK;
    if enable { hdp_clk_cntl &= !mask; } else { hdp_clk_cntl |= mask; }
    WREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL, hdp_clk_cntl);
}

unsafe fn hdp_v5_2_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let tmp = RREG32_SOC15!(HDP, 0, regHDP_CLK_CNTL);
    let mask = HDP_CLK_CNTL__ATOMIC_MEM_CLK_SOFT_OVERRIDE_MASK | HDP_CLK_CNTL__RC_MEM_CLK_SOFT_OVERRIDE_MASK |
        HDP_CLK_CNTL__DBUS_CLK_SOFT_OVERRIDE_MASK | HDP_CLK_CNTL__DYN_CLK_SOFT_OVERRIDE_MASK |
        HDP_CLK_CNTL__XDP_REG_CLK_SOFT_OVERRIDE_MASK | HDP_CLK_CNTL__HDP_REG_CLK_SOFT_OVERRIDE_MASK;
    if (tmp & mask) == 0 { *flags |= AMD_CG_SUPPORT_HDP_MGCG; }
    let tmp = RREG32_SOC15!(HDP, 0, regHDP_MEM_POWER_CTRL);
    if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_LS_EN_MASK) != 0 { *flags |= AMD_CG_SUPPORT_HDP_LS; }
    else if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_DS_EN_MASK) != 0 { *flags |= AMD_CG_SUPPORT_HDP_DS; }
    else if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_SD_EN_MASK) != 0 { *flags |= AMD_CG_SUPPORT_HDP_SD; }
}

unsafe fn hdp_v5_2_update_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    hdp_v5_2_update_mem_power_gating(adev, enable);
    hdp_v5_2_update_medium_grain_clock_gating(adev, enable);
}

pub static hdp_v5_2_funcs: amdgpu_hdp_funcs = amdgpu_hdp_funcs {
    flush_hdp: Some(hdp_v5_2_flush_hdp),
    update_clock_gating: Some(hdp_v5_2_update_clock_gating),
    get_clock_gating_state: Some(hdp_v5_2_get_clockgating_state),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
