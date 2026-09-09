/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Intel Corporation
 */

// Dependency supplied by ivpu_drv.h in the original C header.
pub struct ivpu_device;

unsafe extern "C" {
    pub fn ivpu_sysfs_init(vdev: *mut ivpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
