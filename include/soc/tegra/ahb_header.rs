/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// C dependency: struct device_node is supplied by another translation unit.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn tegra_ahb_enable_smmu(ahb: *mut device_node) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
