/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// C dependencies:
// #include <linux/types.h>
// #include "adf_accel_devices.h"
// #include "adf_common_drv.h"

pub const ADF_GEN2_ERRSOU3: u32 = 0x3A000 + 0x0C;
pub const ADF_GEN2_ERRSOU5: u32 = 0x3A000 + 0xD8;
pub const ADF_GEN2_ERRMSK3: u32 = 0x3A000 + 0x1C;
pub const ADF_GEN2_ERRMSK5: u32 = 0x3A000 + 0xDC;

// CONFIG_PCI_IOV is a build-time C configuration condition.
#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn adf_gen2_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops);
    pub fn adf_gen2_init_vf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops);
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn adf_gen2_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops) {
    (*pfvf_ops).enable_comms = Some(adf_pfvf_comms_disabled);
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn adf_gen2_init_vf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops) {
    (*pfvf_ops).enable_comms = Some(adf_pfvf_comms_disabled);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
