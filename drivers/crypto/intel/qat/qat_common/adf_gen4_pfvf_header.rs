/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// Translated from adf_gen4_pfvf.h.
// Dependencies supplied by the surrounding translation unit:
// adf_accel_devices.h, adf_common_drv.h

// #ifdef CONFIG_PCI_IOV
#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn adf_gen4_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops);
}

// #else
#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn adf_gen4_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops) {
    (*pfvf_ops).enable_comms = adf_pfvf_comms_disabled;
}

// External types and symbols referenced by the original header are supplied by
// the corresponding translated dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
