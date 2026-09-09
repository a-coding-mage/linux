/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * KXCJK-1013 3-axis accelerometer Interface
 * Copyright (c) 2014, Intel Corporation.
 */

// Dependency intent: equivalent of <linux/iio/iio.h>.

#[repr(C)]
pub struct kxcjk_1013_platform_data {
    pub active_high_intr: bool,
    pub orientation: iio_mount_matrix,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
