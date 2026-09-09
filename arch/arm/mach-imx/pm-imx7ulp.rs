// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP
 *   Author: Dong Aisheng <aisheng.dong@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const SMC_PMCTRL: usize = 0x10;
const BP_PMCTRL_PSTOPO: u32 = 16;
const PSTOPO_PSTOP3: u32 = 0x3;
const PSTOPO_PSTOP2: u32 = 0x2;
const PSTOPO_PSTOP1: u32 = 0x1;
const BP_PMCTRL_RUNM: u32 = 8;
const RUNM_RUN: u32 = 0;
const BP_PMCTRL_STOPM: u32 = 0;
const STOPM_STOP: u32 = 0;

const BM_PMCTRL_PSTOPO: u32 = 3 << BP_PMCTRL_PSTOPO;
const BM_PMCTRL_RUNM: u32 = 3 << BP_PMCTRL_RUNM;
const BM_PMCTRL_STOPM: u32 = 7 << BP_PMCTRL_STOPM;

static mut smc1_base: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn imx7ulp_set_lpm(mode: ulp_cpu_pwr_mode) -> i32 {
    let mut val: u32 = readl_relaxed((smc1_base as *mut u8).add(SMC_PMCTRL) as *const u32);

    /* clear all */
    val &= !(BM_PMCTRL_RUNM | BM_PMCTRL_STOPM | BM_PMCTRL_PSTOPO);

    match mode {
        ULP_PM_RUN => {
            /* system/bus clock enabled */
            val |= PSTOPO_PSTOP3 << BP_PMCTRL_PSTOPO;
        }
        ULP_PM_WAIT => {
            /* system clock disabled, bus clock enabled */
            val |= PSTOPO_PSTOP2 << BP_PMCTRL_PSTOPO;
        }
        ULP_PM_STOP => {
            /* system/bus clock disabled */
            val |= PSTOPO_PSTOP1 << BP_PMCTRL_PSTOPO;
        }
        _ => return -EINVAL,
    }

    writel_relaxed(val, (smc1_base as *mut u8).add(SMC_PMCTRL) as *mut u32);

    0
}

pub unsafe fn imx7ulp_pm_init() {
    let np: *mut device_node;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        "fsl,imx7ulp-smc1" as *const str,
    );
    smc1_base = of_iomap(np, 0);
    of_node_put(np);
    WARN_ON(smc1_base.is_null());

    imx7ulp_set_lpm(ULP_PM_RUN);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
