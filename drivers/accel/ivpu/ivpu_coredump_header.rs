/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Translated from ivpu_coredump.h.
// The original includes provide these types and functions.

#[cfg(CONFIG_DEV_COREDUMP)]
extern "C" {
    pub fn ivpu_dev_coredump(vdev: *mut crate::ivpu_device);
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
#[inline]
pub unsafe fn ivpu_dev_coredump(vdev: *mut crate::ivpu_device) {
    let mut p = crate::drm_info_printer((*vdev).drm.dev);

    crate::ivpu_fw_log_print(vdev, false, &mut p);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
