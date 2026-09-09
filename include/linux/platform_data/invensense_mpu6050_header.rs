/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Invensense, Inc.
 */

/**
 * struct inv_mpu6050_platform_data - Platform data for the mpu driver
 * @orientation: Orientation matrix of the chip (deprecated in favor of
 *              mounting matrix retrieved from device-tree)
 *
 * Contains platform specific information on how to configure the MPU6050 to
 * work on this platform.  The orientation matrices are 3x3 rotation matrices
 * that are applied to the data to rotate from the mounting orientation to the
 * platform orientation.  The values must be one of 0, 1, or -1 and each row and
 * column should have exactly 1 non-zero value.
 *
 * Deprecated in favor of mounting matrix retrieved from device-tree.
 */
#[repr(C)]
pub struct inv_mpu6050_platform_data {
    pub orientation: [i8; 9],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
