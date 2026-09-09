/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * osd.h - DEPRECATED On Screen Display API
 *
 * NOTE: should not be used on future drivers
 *
 * Copyright (C) 2001 Ralph  Metzler <ralph@convergence.de>
 *                  & Marcus Metzler <marcus@convergence.de>
 *                    for convergence integrated media GmbH
 */

use core::ffi::{c_long, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OSD_Command {
    /* All functions return -2 on "not open" */
    OSD_Close = 1, /* () */
    /* Disables OSD and releases the buffers; returns 0 on success */
    OSD_Open, /* (x0,y0,x1,y1,BitPerPixel[2/4/8](color&0x0F),mix[0..15](color&0xF0)) */
    /* Opens OSD with this size and bit depth; returns 0 on success, -1 on DRAM allocation error, -2 on "already open" */
    OSD_Show, /* () */
    /* enables OSD mode; returns 0 on success */
    OSD_Hide, /* () */
    /* disables OSD mode; returns 0 on success */
    OSD_Clear, /* () */
    /* Sets all pixel to color 0; returns 0 on success */
    OSD_Fill, /* (color) */
    /* Sets all pixel to color <col>; returns 0 on success */
    OSD_SetColor, /* (color,R{x0},G{y0},B{x1},opacity{y1}) */
    /* set palette entry <num> to <r,g,b>, <mix> and <trans> apply; returns 0 on success, -1 on error */
    OSD_SetPalette, /* (firstcolor{color},lastcolor{x0},data) */
    /* Set a number of entries in the palette */
    OSD_SetTrans, /* (transparency{color}) */
    /* Sets transparency of mixed pixel (0..15); returns 0 on success */
    OSD_SetPixel, /* (x0,y0,color) */
    /* sets pixel <x>,<y> to color number <col>; returns 0 on success, -1 on error */
    OSD_GetPixel, /* (x0,y0) */
    /* returns color number of pixel <x>,<y>, or -1 */
    OSD_SetRow, /* (x0,y0,x1,data) */
    /* fills pixels x0,y through x1,y with the content of data[] */
    OSD_SetBlock, /* (x0,y0,x1,y1,increment{color},data) */
    /* fills pixels x0,y0 through x1,y1 with the content of data[] */
    OSD_FillRow, /* (x0,y0,x1,color) */
    /* fills pixels x0,y through x1,y with the color <col> */
    OSD_FillBlock, /* (x0,y0,x1,y1,color) */
    /* fills pixels x0,y0 through x1,y1 with the color <col> */
    OSD_Line, /* (x0,y0,x1,y1,color) */
    /* draw a line from x0,y0 to x1,y1 with the color <col> */
    OSD_Query, /* (x0,y0,x1,y1,xasp{color}}), yasp=11 */
    /* fills parameters with the picture dimensions and the pixel aspect ratio */
    OSD_Test, /* () */
    /* draws a test picture. for debugging purposes only; TODO: remove "test" in final version */
    OSD_Text, /* (x0,y0,size,color,text) */
    OSD_SetWindow, /* (x0) set window with number 0<x0<8 as current */
    OSD_MoveWindow, /* move current window to (x0, y0) */
    OSD_OpenRaw, /* Open other types of OSD windows */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osd_cmd_t {
    pub cmd: OSD_Command,
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub color: i32,
    pub data: *mut c_void,
}

/* OSD_OpenRaw: set 'color' to desired window type */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum osd_raw_window_t {
    OSD_BITMAP1,
    OSD_BITMAP2,
    OSD_BITMAP4,
    OSD_BITMAP8,
    OSD_BITMAP1HR,
    OSD_BITMAP2HR,
    OSD_BITMAP4HR,
    OSD_BITMAP8HR,
    OSD_YCRCB422,
    OSD_YCRCB444,
    OSD_YCRCB444HR,
    OSD_VIDEOTSIZE,
    OSD_VIDEOHSIZE,
    OSD_VIDEOQSIZE,
    OSD_VIDEODSIZE,
    OSD_VIDEOTHSIZE,
    OSD_VIDEOTQSIZE,
    OSD_VIDEOTDSIZE,
    OSD_VIDEONSIZE,
    OSD_CURSOR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osd_cap_t {
    pub cmd: i32,
    /* OSD_CAP_MEMSIZE: memory size */
    pub val: c_long,
}

/* Linux ioctl encoding; the exact ABI value depends on target C layout. */
pub const OSD_SEND_CMD: u32 = 0x4000_6fA0 | ((core::mem::size_of::<osd_cmd_t>() as u32) << 16);
pub const OSD_GET_CAPABILITY: u32 = 0x8000_6fA1 | ((core::mem::size_of::<osd_cap_t>() as u32) << 16);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
