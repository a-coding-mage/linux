/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1999-2007 H. Peter Anvin - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

#[repr(C)]
pub struct far_ptr {
    pub off: u16,
    pub seg: u16,
}

/* VESA General Information table */
#[repr(C, packed)]
pub struct vesa_general_info {
    pub signature: u32,       /* 0 Magic number = "VESA" */
    pub version: u16,         /* 4 */
    pub vendor_string: far_ptr, /* 6 */
    pub capabilities: u32,    /* 10 */
    pub video_mode_ptr: far_ptr, /* 14 */
    pub total_memory: u16,     /* 18 */

    pub reserved: [u8; 236],   /* 20 */
}

pub const VESA_MAGIC: u32 = 'V' as u32 + (('E' as u32) << 8)
    + (('S' as u32) << 16) + (('A' as u32) << 24);

#[repr(C, packed)]
pub struct vesa_mode_info {
    pub mode_attr: u16,       /* 0 */
    pub win_attr: [u8; 2],    /* 2 */
    pub win_grain: u16,      /* 4 */
    pub win_size: u16,       /* 6 */
    pub win_seg: [u16; 2],   /* 8 */
    pub win_scheme: far_ptr, /* 12 */
    pub logical_scan: u16,   /* 16 */

    pub h_res: u16,          /* 18 */
    pub v_res: u16,          /* 20 */
    pub char_width: u8,      /* 22 */
    pub char_height: u8,     /* 23 */
    pub memory_planes: u8,   /* 24 */
    pub bpp: u8,             /* 25 */
    pub banks: u8,           /* 26 */
    pub memory_layout: u8,   /* 27 */
    pub bank_size: u8,       /* 28 */
    pub image_planes: u8,    /* 29 */
    pub page_function: u8,   /* 30 */

    pub rmask: u8,           /* 31 */
    pub rpos: u8,            /* 32 */
    pub gmask: u8,           /* 33 */
    pub gpos: u8,            /* 34 */
    pub bmask: u8,           /* 35 */
    pub bpos: u8,            /* 36 */
    pub resv_mask: u8,       /* 37 */
    pub resv_pos: u8,        /* 38 */
    pub dcm_info: u8,        /* 39 */

    pub lfb_ptr: u32,        /* 40 Linear frame buffer address */
    pub offscreen_ptr: u32,  /* 44 Offscreen memory address */
    pub offscreen_size: u16, /* 48 */

    pub reserved: [u8; 206], /* 50 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
