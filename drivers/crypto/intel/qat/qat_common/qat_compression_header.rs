/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2022 Intel Corporation */

// C dependencies:
// #include <linux/list.h>
// #include <linux/types.h>
// #include "adf_accel_devices.h"
// #include "qat_algs_send.h"

pub const QAT_COMP_MAX_SKID: u32 = 4096;

#[repr(C)]
pub struct qat_compression_instance {
    pub dc_tx: *mut adf_etr_ring_data,
    pub dc_rx: *mut adf_etr_ring_data,
    pub accel_dev: *mut adf_accel_dev,
    pub list: list_head,
    pub state: core::ffi::c_ulong,
    pub id: core::ffi::c_int,
    pub refctr: atomic_t,
    pub backlog: qat_instance_backlog,
    pub dc_data: *mut adf_dc_data,
}

#[inline]
pub unsafe fn adf_hw_dev_has_compression(accel_dev: *mut adf_accel_dev) -> bool {
    let hw_device: *mut adf_hw_device_data = (*accel_dev).hw_device;
    let mask: u32 = !(*hw_device).accel_capabilities_mask;

    if mask & ADF_ACCEL_CAPABILITIES_COMPRESSION != 0 {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
