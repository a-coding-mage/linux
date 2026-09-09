/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2008 Freescale Semiconductor, Inc. All Rights Reserved.
 *
 * Freescale DIU Frame Buffer device driver
 *
 * Authors: Hongjun Chen <hong-jun.chen@freescale.com>
 *          Paul Widmer <paul.widmer@freescale.com>
 *          Srikanth Srinivasan <srikanth.srinivasan@freescale.com>
 *          York Sun <yorksun@freescale.com>
 *
 * Based on imxfb.c Copyright (C) 2004 S.Hauer, Pengutronix
 */

// C dependency: linux/types.h

#[repr(C)]
pub struct mfb_chroma_key {
    pub enable: core::ffi::c_int,
    pub red_max: u8,
    pub green_max: u8,
    pub blue_max: u8,
    pub red_min: u8,
    pub green_min: u8,
    pub blue_min: u8,
}

#[repr(C)]
pub struct aoi_display_offset {
    pub x_aoi_d: i32,
    pub y_aoi_d: i32,
}

// ioctl definitions: _IOW/_IOR('M', number, type)
// MFB_SET_CHROMA_KEY = _IOW('M', 1, struct mfb_chroma_key)
// MFB_SET_BRIGHTNESS = _IOW('M', 3, u8)
// MFB_SET_ALPHA = _IOW('M', 0, u8)
// MFB_GET_ALPHA = _IOR('M', 0, u8)
// MFB_SET_AOID = _IOW('M', 4, struct aoi_display_offset)
// MFB_GET_AOID = _IOR('M', 4, struct aoi_display_offset)
// MFB_SET_PIXFMT = _IOW('M', 8, u32)
// MFB_GET_PIXFMT = _IOR('M', 8, u32)

/*
 * The MPC5121 BSP comes with a gamma_set utility that initializes the
 * gamma table. Unfortunately, it uses bad values for the IOCTL commands,
 * but there's nothing we can do about it now. These ioctls are only
 * supported on the MPC5121.
 */
// MFB_SET_GAMMA = _IOW('M', 1, u8)
// MFB_GET_GAMMA = _IOR('M', 1, u8)

/*
 * The original definitions of MFB_SET_PIXFMT and MFB_GET_PIXFMT used the
 * wrong value for 'size' field of the ioctl. The current macros above use the
 * right size, but we still need to provide backwards compatibility, at least
 * for a while.
 */
pub const MFB_SET_PIXFMT_OLD: u32 = 0x8001_4d08;
pub const MFB_GET_PIXFMT_OLD: u32 = 0x4001_4d08;

// The following declarations are present when __KERNEL__ is defined.

/* These are the fields of area descriptor (in DDR memory) for every plane. */
#[repr(C, packed)]
pub struct diu_ad {
    // Word 0 (32-bit) in DDR memory: hard coded pixel format.
    pub pix_fmt: __be32,
    // Word 1 (32-bit) in DDR memory.
    pub addr: __le32,
    // Word 2 (32-bit) in DDR memory.
    pub src_size_g_alpha: __le32,
    // Word 3 (32-bit) in DDR memory.
    pub aoi_size: __le32,
    // Word 4 (32-bit) in DDR memory.
    pub offset_xyi: __le32,
    // Word 5 (32-bit) in DDR memory.
    pub offset_xyd: __le32,
    // Word 6 (32-bit) in DDR memory.
    pub ckmax_r: u8,
    pub ckmax_g: u8,
    pub ckmax_b: u8,
    pub res9: u8,
    // Word 7 (32-bit) in DDR memory.
    pub ckmin_r: u8,
    pub ckmin_g: u8,
    pub ckmin_b: u8,
    pub res10: u8,
    // Word 8 (32-bit) in DDR memory.
    pub next_ad: __le32,
    // Word 9 (32-bit) in DDR memory, just for 64-bit aligned.
    pub paddr: u32,
}

/* DIU register map. */
#[repr(C, packed)]
pub struct diu {
    pub desc: [__be32; 3],
    pub gamma: __be32,
    pub palette: __be32,
    pub cursor: __be32,
    pub curs_pos: __be32,
    pub diu_mode: __be32,
    pub bgnd: __be32,
    pub bgnd_wb: __be32,
    pub disp_size: __be32,
    pub wb_size: __be32,
    pub wb_mem_addr: __be32,
    pub hsyn_para: __be32,
    pub vsyn_para: __be32,
    pub syn_pol: __be32,
    pub thresholds: __be32,
    pub int_status: __be32,
    pub int_mask: __be32,
    pub colorbar: [__be32; 8],
    pub filling: __be32,
    pub plut: __be32,
}

/* Modes of operation of DIU. The DIU supports five modes, but the driver
 * only supports modes 0 and 1. */
pub const MFB_MODE0: u32 = 0; // DIU off
pub const MFB_MODE1: u32 = 1; // All three planes output to display

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
