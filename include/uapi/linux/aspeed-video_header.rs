/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2021 ASPEED Technology Inc.
 */

// Dependency supplied by the Linux V4L2 controls interface:
// `V4L2_CID_USER_ASPEED_BASE`.

/* aspeed video's input types */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum aspeed_video_input {
    VIDEO_INPUT_VGA = 0,
    VIDEO_INPUT_GFX,
    VIDEO_INPUT_MAX,
}

pub const V4L2_CID_ASPEED_HQ_MODE: u32 = V4L2_CID_USER_ASPEED_BASE + 1;
pub const V4L2_CID_ASPEED_HQ_JPEG_QUALITY: u32 = V4L2_CID_USER_ASPEED_BASE + 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
