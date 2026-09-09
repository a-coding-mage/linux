/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

const REG_HDP_CLK_CNTL_V6_1: u32 = 0xd5;
const REG_HDP_CLK_CNTL_V6_1_BASE_IDX: u32 = 0;

unsafe fn hdp_v6_0_update_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let mut hdp_clk_cntl: u32;
    let mut hdp_mem_pwr_cntl: u32;

    if ((*adev).cg_flags & (AMD_CG_SUPPORT_HDP_LS |
        AMD_CG_SUPPORT_HDP_DS | AMD_CG_SUPPORT_HDP_SD)) == 0 { return; }

    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(6, 1, 0) {
        hdp_clk_cntl = RREG32_SOC15(HDP, 0, REG_HDP_CLK_CNTL_V6_1);
    } else {
        hdp_clk_cntl = RREG32_SOC15(HDP, 0, regHDP_CLK_CNTL);
    }
    hdp_mem_pwr_cntl = RREG32_SOC15(HDP, 0, regHDP_MEM_POWER_CTRL);

    /* Before doing clock/power mode switch, forced on IPH & RC clock */
    hdp_clk_cntl = REG_SET_FIELD(hdp_clk_cntl, HDP_CLK_CNTL,
        RC_MEM_CLK_SOFT_OVERRIDE, 1);
    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(6, 1, 0) {
        WREG32_SOC15(HDP, 0, REG_HDP_CLK_CNTL_V6_1, hdp_clk_cntl);
    } else { WREG32_SOC15(HDP, 0, regHDP_CLK_CNTL, hdp_clk_cntl); }

    /* disable clock and power gating before any changing */
    for field in [ATOMIC_MEM_POWER_CTRL_EN, ATOMIC_MEM_POWER_LS_EN,
        ATOMIC_MEM_POWER_DS_EN, ATOMIC_MEM_POWER_SD_EN,
        RC_MEM_POWER_CTRL_EN, RC_MEM_POWER_LS_EN,
        RC_MEM_POWER_DS_EN, RC_MEM_POWER_SD_EN] {
        hdp_mem_pwr_cntl = REG_SET_FIELD(hdp_mem_pwr_cntl,
            HDP_MEM_POWER_CTRL, field, 0);
    }
    WREG32_SOC15(HDP, 0, regHDP_MEM_POWER_CTRL, hdp_mem_pwr_cntl);

    /* Already disabled above. The actions below are for "enabled" only */
    if enable {
        /* only one clock gating mode (LS/DS/SD) can be enabled */
        let field = if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_SD) != 0 {
            Some(ATOMIC_MEM_POWER_SD_EN)
        } else if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_LS) != 0 {
            Some(ATOMIC_MEM_POWER_LS_EN)
        } else if ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_DS) != 0 {
            Some(ATOMIC_MEM_POWER_DS_EN)
        } else { None };
        if let Some(field) = field {
            hdp_mem_pwr_cntl = REG_SET_FIELD(hdp_mem_pwr_cntl,
                HDP_MEM_POWER_CTRL, field, 1);
            let rc_field = match field {
                ATOMIC_MEM_POWER_SD_EN => RC_MEM_POWER_SD_EN,
                ATOMIC_MEM_POWER_LS_EN => RC_MEM_POWER_LS_EN,
                _ => RC_MEM_POWER_DS_EN,
            };
            hdp_mem_pwr_cntl = REG_SET_FIELD(hdp_mem_pwr_cntl,
                HDP_MEM_POWER_CTRL, rc_field, 1);
        }

        /* confirmed that IPH_MEM_POWER_CTRL_EN and RC_MEM_POWER_CTRL_EN have to
         * be set for SRAM LS/DS/SD */
        if ((*adev).cg_flags & (AMD_CG_SUPPORT_HDP_LS | AMD_CG_SUPPORT_HDP_DS |
            AMD_CG_SUPPORT_HDP_SD)) != 0 {
            hdp_mem_pwr_cntl = REG_SET_FIELD(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL,
                ATOMIC_MEM_POWER_CTRL_EN, 1);
            hdp_mem_pwr_cntl = REG_SET_FIELD(hdp_mem_pwr_cntl, HDP_MEM_POWER_CTRL,
                RC_MEM_POWER_CTRL_EN, 1);
            WREG32_SOC15(HDP, 0, regHDP_MEM_POWER_CTRL, hdp_mem_pwr_cntl);
        }
    }

    /* disable IPH & RC clock override after clock/power mode changing */
    hdp_clk_cntl = REG_SET_FIELD(hdp_clk_cntl, HDP_CLK_CNTL,
        RC_MEM_CLK_SOFT_OVERRIDE, 0);
    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(6, 1, 0) {
        WREG32_SOC15(HDP, 0, REG_HDP_CLK_CNTL_V6_1, hdp_clk_cntl);
    } else { WREG32_SOC15(HDP, 0, regHDP_CLK_CNTL, hdp_clk_cntl); }
}

unsafe fn hdp_v6_0_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    /* AMD_CG_SUPPORT_HDP_LS/DS/SD */
    let tmp = RREG32_SOC15(HDP, 0, regHDP_MEM_POWER_CTRL);
    if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_LS_EN_MASK) != 0 {
        *flags |= AMD_CG_SUPPORT_HDP_LS;
    } else if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_DS_EN_MASK) != 0 {
        *flags |= AMD_CG_SUPPORT_HDP_DS;
    } else if (tmp & HDP_MEM_POWER_CTRL__ATOMIC_MEM_POWER_SD_EN_MASK) != 0 {
        *flags |= AMD_CG_SUPPORT_HDP_SD;
    }
}

const hdp_v6_0_funcs: amdgpu_hdp_funcs = amdgpu_hdp_funcs {
    flush_hdp: amdgpu_hdp_generic_flush,
    update_clock_gating: hdp_v6_0_update_clock_gating,
    get_clock_gating_state: hdp_v6_0_get_clockgating_state,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
