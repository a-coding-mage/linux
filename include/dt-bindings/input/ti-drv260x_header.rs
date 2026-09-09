/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * DRV260X haptics driver family
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 *
 * Copyright:   (C) 2014 Texas Instruments, Inc.
 */

// Calibration Types
pub const DRV260X_LRA_MODE: u32 = 0x00;
pub const DRV260X_LRA_NO_CAL_MODE: u32 = 0x01;
pub const DRV260X_ERM_MODE: u32 = 0x02;

// Library Selection
pub const DRV260X_LIB_EMPTY: u32 = 0x00;
pub const DRV260X_ERM_LIB_A: u32 = 0x01;
pub const DRV260X_ERM_LIB_B: u32 = 0x02;
pub const DRV260X_ERM_LIB_C: u32 = 0x03;
pub const DRV260X_ERM_LIB_D: u32 = 0x04;
pub const DRV260X_ERM_LIB_E: u32 = 0x05;
pub const DRV260X_LIB_LRA: u32 = 0x06;
pub const DRV260X_ERM_LIB_F: u32 = 0x07;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
