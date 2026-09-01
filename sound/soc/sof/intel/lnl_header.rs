// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2024 Intel Corporation
 */

// C header guard omitted: __SOF_INTEL_LNL_H

pub const LNL_DSP_REG_HFDSC: u32 = 0x160200; /* DSP core0 status */
pub const LNL_DSP_REG_HFDEC: u32 = 0x160204; /* DSP core0 error */

unsafe extern "C" {
    pub fn sof_lnl_set_ops(
        sdev: *mut snd_sof_dev,
        dsp_ops: *mut snd_sof_dsp_ops,
    ) -> ::core::ffi::c_int;

    pub fn lnl_dsp_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool;
    pub fn lnl_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
    pub fn lnl_sdw_check_wakeen_irq(sdev: *mut snd_sof_dev) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
