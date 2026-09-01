/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2025 Intel Corporation
 */

// Header guard __SOF_INTEL_PTL_H omitted in Rust.

// Depends on external BIT/GENMASK macro semantics from the original C headers.
pub const PTL_MICPVCP_DDZE_FORCED: u32 = 1u32 << 16;
pub const PTL_MICPVCP_DDZE_ENABLED: u32 = 1u32 << 17;
pub const PTL_MICPVCP_DDZLS_SDW: u32 = ((u32::MAX) << 20) & ((u32::MAX) >> (31 - 26));

#[inline]
pub const fn PTL_MICPVCP_GET_SDW_MASK(x: u32) -> u32 {
    ((x) & PTL_MICPVCP_DDZLS_SDW) >> 20
}

unsafe extern "C" {
    pub fn sof_ptl_set_ops(
        sdev: *mut snd_sof_dev,
        dsp_ops: *mut snd_sof_dsp_ops,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
