/* SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * include/linux/v4l2-common.h
 *
 * Common V4L2 and V4L2 subdev definitions.
 *
 * Users are advised to include this file either through videodev2.h
 * (V4L2) or through v4l2-subdev.h (V4L2 subdev) rather than to refer
 * to this file directly.
 *
 * Copyright (C) 2012 Nokia Corporation
 * Contact: Sakari Ailus <sakari.ailus@iki.fi>
 */

/* linux/types.h supplies the __u8 and __u32 types used below. */

/*
 *
 * Selection interface definitions
 *
 */

/* Current cropping area */
pub const V4L2_SEL_TGT_CROP: u32 = 0x0000;
/* Default cropping area */
pub const V4L2_SEL_TGT_CROP_DEFAULT: u32 = 0x0001;
/* Cropping bounds */
pub const V4L2_SEL_TGT_CROP_BOUNDS: u32 = 0x0002;
/* Native frame size */
pub const V4L2_SEL_TGT_NATIVE_SIZE: u32 = 0x0003;
/* Current composing area */
pub const V4L2_SEL_TGT_COMPOSE: u32 = 0x0100;
/* Default composing area */
pub const V4L2_SEL_TGT_COMPOSE_DEFAULT: u32 = 0x0101;
/* Composing bounds */
pub const V4L2_SEL_TGT_COMPOSE_BOUNDS: u32 = 0x0102;
/* Current composing area plus all padding pixels */
pub const V4L2_SEL_TGT_COMPOSE_PADDED: u32 = 0x0103;

/* Selection flags */
pub const V4L2_SEL_FLAG_GE: u32 = 1 << 0;
pub const V4L2_SEL_FLAG_LE: u32 = 1 << 1;
pub const V4L2_SEL_FLAG_KEEP_CONFIG: u32 = 1 << 2;

#[repr(C)]
pub struct v4l2_edid {
    pub pad: u32,
    pub start_block: u32,
    pub blocks: u32,
    pub reserved: [u32; 5],
    pub edid: *mut u8,
}

/*
 * The following backward-compatibility definitions are present only when
 * building outside the kernel (__KERNEL__ is not defined in the C header).
 * Rust cfg selection for the surrounding build is intentionally left to the
 * consumer of this translation.
 */
#[cfg(not(feature = "kernel"))]
pub const V4L2_SEL_TGT_CROP_ACTIVE: u32 = V4L2_SEL_TGT_CROP;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SEL_TGT_COMPOSE_ACTIVE: u32 = V4L2_SEL_TGT_COMPOSE;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_TGT_CROP_ACTUAL: u32 = V4L2_SEL_TGT_CROP;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_TGT_COMPOSE_ACTUAL: u32 = V4L2_SEL_TGT_COMPOSE;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_TGT_CROP_BOUNDS: u32 = V4L2_SEL_TGT_CROP_BOUNDS;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_TGT_COMPOSE_BOUNDS: u32 = V4L2_SEL_TGT_COMPOSE_BOUNDS;

/* Backward compatibility flag definitions --- to be removed. */
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_FLAG_SIZE_GE: u32 = V4L2_SEL_FLAG_GE;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_FLAG_SIZE_LE: u32 = V4L2_SEL_FLAG_LE;
#[cfg(not(feature = "kernel"))]
pub const V4L2_SUBDEV_SEL_FLAG_KEEP_CONFIG: u32 = V4L2_SEL_FLAG_KEEP_CONFIG;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
