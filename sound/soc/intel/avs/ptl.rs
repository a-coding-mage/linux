// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2024-2025 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C dependencies:
// #include <sound/hdaudio_ext.h>
// #include "avs.h"
// #include "debug.h"
// #include "registers.h"
// #include "trace.h"

use core::ffi::c_int;

const MTL_HfDSSGBL_BASE: u32 = 0x1000;
const MTL_REG_HfDSSCS: u32 = MTL_HfDSSGBL_BASE + 0x0;
const MTL_HfDSSCS_SPA: u32 = 1u32 << 16;
const MTL_HfDSSCS_CPA: u32 = 1u32 << 24;

const MTL_DSPCS_BASE: u32 = 0x178D00;
const MTL_REG_DSPCCTL: u32 = MTL_DSPCS_BASE + 0x4;
const MTL_DSPCCTL_OSEL: u32 = ((1u32 << (25 - 24 + 1)) - 1) << 24;
const MTL_DSPCCTL_OSEL_HOST: u32 = 1u32 << 25;

unsafe fn avs_ptl_core_power_on(adev: *mut avs_dev) -> c_int {
    let mut reg: u32 = 0;
    let mut ret: c_int;

    /* Power up DSP domain. */
    snd_hdac_adsp_updatel(
        adev,
        MTL_REG_HfDSSCS,
        MTL_HfDSSCS_SPA,
        MTL_HfDSSCS_SPA,
    );
    trace_avs_dsp_core_op(1, AVS_MAIN_CORE_MASK, c"power dsp".as_ptr(), true);

    ret = snd_hdac_adsp_readl_poll(
        adev,
        MTL_REG_HfDSSCS,
        &mut reg,
        (reg & MTL_HfDSSCS_CPA) == MTL_HfDSSCS_CPA,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US,
    );
    if ret != 0 {
        dev_err(
            (*adev).dev,
            c"power on domain dsp failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    /* Prevent power gating of DSP domain. */
    snd_hdac_adsp_updatel(
        adev,
        MTL_REG_HfPWRCTL2,
        MTL_HfPWRCTL2_WPDSPHPxPG,
        MTL_HfPWRCTL2_WPDSPHPxPG,
    );
    trace_avs_dsp_core_op(1, AVS_MAIN_CORE_MASK, c"prevent dsp PG".as_ptr(), true);

    ret = snd_hdac_adsp_readl_poll(
        adev,
        MTL_REG_HfPWRSTS2,
        &mut reg,
        (reg & MTL_HfPWRSTS2_DSPHPxPGS) == MTL_HfPWRSTS2_DSPHPxPGS,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US,
    );

    /* Set ownership to HOST. */
    snd_hdac_adsp_updatel(
        adev,
        MTL_REG_DSPCCTL,
        MTL_DSPCCTL_OSEL,
        MTL_DSPCCTL_OSEL_HOST,
    );
    ret
}

unsafe fn avs_ptl_core_power_off(adev: *mut avs_dev) -> c_int {
    let mut reg: u32 = 0;

    /* Allow power gating of DSP domain. No STS polling as HOST is only one of its users. */
    snd_hdac_adsp_updatel(adev, MTL_REG_HfPWRCTL2, MTL_HfPWRCTL2_WPDSPHPxPG, 0);
    trace_avs_dsp_core_op(0, AVS_MAIN_CORE_MASK, c"allow dsp pg".as_ptr(), false);

    /* Power down DSP domain. */
    snd_hdac_adsp_updatel(adev, MTL_REG_HfDSSCS, MTL_HfDSSCS_SPA, 0);
    trace_avs_dsp_core_op(0, AVS_MAIN_CORE_MASK, c"power dsp".as_ptr(), false);

    snd_hdac_adsp_readl_poll(
        adev,
        MTL_REG_HfDSSCS,
        &mut reg,
        (reg & MTL_HfDSSCS_CPA) == 0,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US,
    )
}

unsafe fn avs_ptl_core_power(adev: *mut avs_dev, mut core_mask: u32, power: bool) -> c_int {
    core_mask &= AVS_MAIN_CORE_MASK;
    if core_mask == 0 {
        return 0;
    }

    if power {
        return avs_ptl_core_power_on(adev);
    }
    avs_ptl_core_power_off(adev)
}

pub static avs_ptl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_ptl_core_power),
    reset: Some(avs_mtl_core_reset),
    stall: Some(avs_lnl_core_stall),
    dsp_interrupt: Some(avs_mtl_dsp_interrupt),
    int_control: Some(avs_mtl_interrupt_control),
    load_basefw: Some(avs_hda_load_basefw),
    load_lib: Some(avs_hda_load_library),
    transfer_mods: Some(avs_hda_transfer_modules),
    log_buffer_offset: Some(avs_icl_log_buffer_offset),
    log_buffer_status: Some(avs_apl_log_buffer_status),
    coredump: Some(avs_apl_coredump),
    d0ix_toggle: Some(avs_icl_d0ix_toggle),
    set_d0ix: Some(avs_icl_set_d0ix),
    /* AVS_SET_ENABLE_LOGS_OP(icl) */
    enable_logs: Some(avs_icl_enable_logs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
