/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2025 Intel Corporation
 */

unsafe extern "C" {
    pub fn sof_nvl_set_ops(
        sdev: *mut snd_sof_dev,
        dsp_ops: *mut snd_sof_dsp_ops,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
