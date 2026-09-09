/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation */

// Dependency supplied by the surrounding repository:
// #include "adf_accel_devices.h"

extern "C" {
    pub fn adf_gen4_init_vf_mig_ops(vfmig_ops: *mut qat_migdev_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
