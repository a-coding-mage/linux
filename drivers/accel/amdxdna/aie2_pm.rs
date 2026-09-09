// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding driver are intentionally left external.

const AIE2_CLK_GATING_ENABLE: u32 = 1;
const AIE2_CLK_GATING_DISABLE: u32 = 0;

unsafe fn aie2_pm_set_clk_gating(ndev: *mut amdxdna_dev_hdl, val: u32) -> i32 {
    let ret: i32;

    ret = aie2_runtime_cfg(ndev, AIE2_RT_CFG_CLK_GATING, &val as *const u32 as *mut u32);
    if ret != 0 {
        return ret;
    }

    (*ndev).clk_gating = val;
    0
}

pub unsafe fn aie2_pm_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> i32 {
    let ret: i32;

    ret = amdxdna_pm_resume_get_locked((*ndev).aie.xdna);
    if ret != 0 {
        return ret;
    }

    ret = ((*(*ndev).priv_).hw_ops).set_dpm(ndev, dpm_level);
    if ret == 0 {
        (*ndev).dpm_level = dpm_level;
    }
    amdxdna_pm_suspend_put((*ndev).aie.xdna);

    ret
}

pub unsafe fn aie2_pm_init(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let ret: i32;

    if (*ndev).dev_status != AIE2_DEV_UNINIT {
        // Resume device
        ret = ((*(*ndev).priv_).hw_ops).set_dpm(ndev, (*ndev).dpm_level);
        if ret != 0 {
            return ret;
        }

        ret = aie2_pm_set_clk_gating(ndev, (*ndev).clk_gating);
        if ret != 0 {
            return ret;
        }

        return 0;
    }

    while (*(*ndev).priv_).dpm_clk_tbl[(*ndev).max_dpm_level as usize].hclk != 0 {
        (*ndev).max_dpm_level = (*ndev).max_dpm_level.wrapping_add(1);
    }
    (*ndev).max_dpm_level = (*ndev).max_dpm_level.wrapping_sub(1);

    ret = ((*(*ndev).priv_).hw_ops).set_dpm(ndev, (*ndev).max_dpm_level);
    if ret != 0 {
        return ret;
    }
    (*ndev).dpm_level = (*ndev).max_dpm_level;

    ret = aie2_pm_set_clk_gating(ndev, AIE2_CLK_GATING_ENABLE);
    if ret != 0 {
        return ret;
    }

    (*ndev).pw_mode = POWER_MODE_DEFAULT;
    (*ndev).dft_dpm_level = 0;

    0
}

pub unsafe fn aie2_pm_set_mode(
    ndev: *mut amdxdna_dev_hdl,
    target: enum_amdxdna_power_mode_type,
) -> i32 {
    let xdna = (*ndev).aie.xdna;
    let clk_gating: u32;
    let dpm_level: u32;
    let ret: i32;

    drm_WARN_ON(&mut (*xdna).ddev, !mutex_is_locked(&mut (*xdna).dev_lock));

    if (*ndev).pw_mode == target {
        return 0;
    }

    match target {
        POWER_MODE_TURBO => {
            if (*ndev).hwctx_num != 0 {
                XDNA_ERR(xdna, "Can not set turbo when there is active hwctx");
                return -EINVAL;
            }

            clk_gating = AIE2_CLK_GATING_DISABLE;
            dpm_level = (*ndev).max_dpm_level;
        }
        POWER_MODE_HIGH => {
            clk_gating = AIE2_CLK_GATING_ENABLE;
            dpm_level = (*ndev).max_dpm_level;
        }
        POWER_MODE_DEFAULT => {
            clk_gating = AIE2_CLK_GATING_ENABLE;
            dpm_level = (*ndev).dft_dpm_level;
        }
        POWER_MODE_LOW => {
            clk_gating = AIE2_CLK_GATING_ENABLE;
            dpm_level = 0;
        }
        POWER_MODE_MEDIUM => {
            clk_gating = AIE2_CLK_GATING_ENABLE;
            dpm_level = (*ndev).max_dpm_level / 2;
        }
        _ => return -EOPNOTSUPP,
    }

    ret = aie2_pm_set_dpm(ndev, dpm_level);
    if ret != 0 {
        return ret;
    }

    ret = aie2_pm_set_clk_gating(ndev, clk_gating);
    if ret != 0 {
        return ret;
    }

    (*ndev).pw_mode = target;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
