/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Synopsys, Inc. (www.synopsys.com)
 *
 * Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>
 */

// DSP-related saved registers - need to be saved only when you are
// scheduled out.
// Structure field names correspond to auxiliary register definitions for
// automatic offset calculation in DSP_AUX_SAVE_RESTORE macros.
#[repr(C)]
pub struct dsp_callee_regs {
    pub ACC0_GLO: core::ffi::c_ulong,
    pub ACC0_GHI: core::ffi::c_ulong,
    pub DSP_BFLY0: core::ffi::c_ulong,
    pub DSP_FFT_CTRL: core::ffi::c_ulong,

    // Preserved from CONFIG_ARC_DSP_AGU_USERSPACE. Enable the corresponding
    // Rust feature to include these fields.
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_AP0: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_AP1: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_AP2: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_AP3: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_OS0: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_OS1: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_MOD0: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_MOD1: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_MOD2: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_ARC_DSP_AGU_USERSPACE")]
    pub AGU_MOD3: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
