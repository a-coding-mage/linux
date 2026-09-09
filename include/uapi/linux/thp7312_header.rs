/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * THine THP7312 user space header file.
 *
 * Copyright (C) 2021 THine Electronics, Inc.
 * Copyright (C) 2023 Ideas on Board Oy
 */

// Dependency: V4L2_CID_USER_THP7312_BASE is supplied by the V4L2 controls API.

pub const V4L2_CID_THP7312_LOW_LIGHT_COMPENSATION: u32 =
    V4L2_CID_USER_THP7312_BASE + 0x01;
pub const V4L2_CID_THP7312_AUTO_FOCUS_METHOD: u32 =
    V4L2_CID_USER_THP7312_BASE + 0x02;
pub const V4L2_CID_THP7312_NOISE_REDUCTION_AUTO: u32 =
    V4L2_CID_USER_THP7312_BASE + 0x03;
pub const V4L2_CID_THP7312_NOISE_REDUCTION_ABSOLUTE: u32 =
    V4L2_CID_USER_THP7312_BASE + 0x04;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
