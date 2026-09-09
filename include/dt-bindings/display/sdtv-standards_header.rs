/* SPDX-License-Identifier: GPL-2.0-only OR X11 */
/*
 * Copyright 2019 Pengutronix, Marco Felsch <kernel@pengutronix.de>
 */

/*
 * Attention: Keep the SDTV_STD_* bit definitions in sync with
 * include/uapi/linux/videodev2.h V4L2_STD_* bit definitions.
 */
/* One bit for each standard */
pub const SDTV_STD_PAL_B: u32 = 0x00000001;
pub const SDTV_STD_PAL_B1: u32 = 0x00000002;
pub const SDTV_STD_PAL_G: u32 = 0x00000004;
pub const SDTV_STD_PAL_H: u32 = 0x00000008;
pub const SDTV_STD_PAL_I: u32 = 0x00000010;
pub const SDTV_STD_PAL_D: u32 = 0x00000020;
pub const SDTV_STD_PAL_D1: u32 = 0x00000040;
pub const SDTV_STD_PAL_K: u32 = 0x00000080;

pub const SDTV_STD_PAL: u32 = SDTV_STD_PAL_B
    | SDTV_STD_PAL_B1
    | SDTV_STD_PAL_G
    | SDTV_STD_PAL_H
    | SDTV_STD_PAL_I
    | SDTV_STD_PAL_D
    | SDTV_STD_PAL_D1
    | SDTV_STD_PAL_K;

pub const SDTV_STD_PAL_M: u32 = 0x00000100;
pub const SDTV_STD_PAL_N: u32 = 0x00000200;
pub const SDTV_STD_PAL_Nc: u32 = 0x00000400;
pub const SDTV_STD_PAL_60: u32 = 0x00000800;

pub const SDTV_STD_NTSC_M: u32 = 0x00001000; /* BTSC */
pub const SDTV_STD_NTSC_M_JP: u32 = 0x00002000; /* EIA-J */
pub const SDTV_STD_NTSC_443: u32 = 0x00004000;
pub const SDTV_STD_NTSC_M_KR: u32 = 0x00008000; /* FM A2 */

pub const SDTV_STD_NTSC: u32 = SDTV_STD_NTSC_M | SDTV_STD_NTSC_M_JP | SDTV_STD_NTSC_M_KR;

pub const SDTV_STD_SECAM_B: u32 = 0x00010000;
pub const SDTV_STD_SECAM_D: u32 = 0x00020000;
pub const SDTV_STD_SECAM_G: u32 = 0x00040000;
pub const SDTV_STD_SECAM_H: u32 = 0x00080000;
pub const SDTV_STD_SECAM_K: u32 = 0x00100000;
pub const SDTV_STD_SECAM_K1: u32 = 0x00200000;
pub const SDTV_STD_SECAM_L: u32 = 0x00400000;
pub const SDTV_STD_SECAM_LC: u32 = 0x00800000;

pub const SDTV_STD_SECAM: u32 = SDTV_STD_SECAM_B
    | SDTV_STD_SECAM_D
    | SDTV_STD_SECAM_G
    | SDTV_STD_SECAM_H
    | SDTV_STD_SECAM_K
    | SDTV_STD_SECAM_K1
    | SDTV_STD_SECAM_L
    | SDTV_STD_SECAM_LC;

/* Standards for Countries with 60Hz Line frequency */
pub const SDTV_STD_525_60: u32 =
    SDTV_STD_PAL_M | SDTV_STD_PAL_60 | SDTV_STD_NTSC | SDTV_STD_NTSC_443;

/* Standards for Countries with 50Hz Line frequency */
pub const SDTV_STD_625_50: u32 = SDTV_STD_PAL | SDTV_STD_PAL_N | SDTV_STD_PAL_Nc | SDTV_STD_SECAM;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
