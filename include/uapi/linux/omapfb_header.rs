/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * File: include/linux/omapfb.h
 *
 * Framebuffer driver for TI OMAP boards
 *
 * Copyright (C) 2004 Nokia Corporation
 * Author: Imre Deak <imre.deak@nokia.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

// Dependencies supplied by the Linux framebuffer, ioctl, and types headers
// are intentionally referenced rather than implemented here.

/* IOCTL commands. */
macro_rules! OMAP_IOW { ($num:expr, $dtype:ty) => { _IOW(b'O', $num, $dtype) }; }
macro_rules! OMAP_IOR { ($num:expr, $dtype:ty) => { _IOR(b'O', $num, $dtype) }; }
macro_rules! OMAP_IOWR { ($num:expr, $dtype:ty) => { _IOWR(b'O', $num, $dtype) }; }
macro_rules! OMAP_IO { ($num:expr) => { _IO(b'O', $num) }; }

pub const OMAPFB_MIRROR: _ = OMAP_IOW!(31, core::ffi::c_int);
pub const OMAPFB_SYNC_GFX: _ = OMAP_IO!(37);
pub const OMAPFB_VSYNC: _ = OMAP_IO!(38);
pub const OMAPFB_SET_UPDATE_MODE: _ = OMAP_IOW!(40, core::ffi::c_int);
pub const OMAPFB_GET_CAPS: _ = OMAP_IOR!(42, omapfb_caps);
pub const OMAPFB_GET_UPDATE_MODE: _ = OMAP_IOW!(43, core::ffi::c_int);
pub const OMAPFB_LCD_TEST: _ = OMAP_IOW!(45, core::ffi::c_int);
pub const OMAPFB_CTRL_TEST: _ = OMAP_IOW!(46, core::ffi::c_int);
pub const OMAPFB_UPDATE_WINDOW_OLD: _ = OMAP_IOW!(47, omapfb_update_window_old);
pub const OMAPFB_SET_COLOR_KEY: _ = OMAP_IOW!(50, omapfb_color_key);
pub const OMAPFB_GET_COLOR_KEY: _ = OMAP_IOW!(51, omapfb_color_key);
pub const OMAPFB_SETUP_PLANE: _ = OMAP_IOW!(52, omapfb_plane_info);
pub const OMAPFB_QUERY_PLANE: _ = OMAP_IOW!(53, omapfb_plane_info);
pub const OMAPFB_UPDATE_WINDOW: _ = OMAP_IOW!(54, omapfb_update_window);
pub const OMAPFB_SETUP_MEM: _ = OMAP_IOW!(55, omapfb_mem_info);
pub const OMAPFB_QUERY_MEM: _ = OMAP_IOW!(56, omapfb_mem_info);
pub const OMAPFB_WAITFORVSYNC: _ = OMAP_IO!(57);
pub const OMAPFB_MEMORY_READ: _ = OMAP_IOR!(58, omapfb_memory_read);
pub const OMAPFB_GET_OVERLAY_COLORMODE: _ = OMAP_IOR!(59, omapfb_ovl_colormode);
pub const OMAPFB_WAITFORGO: _ = OMAP_IO!(60);
pub const OMAPFB_GET_VRAM_INFO: _ = OMAP_IOR!(61, omapfb_vram_info);
pub const OMAPFB_SET_TEARSYNC: _ = OMAP_IOW!(62, omapfb_tearsync_info);
pub const OMAPFB_GET_DISPLAY_INFO: _ = OMAP_IOR!(63, omapfb_display_info);

pub const OMAPFB_CAPS_GENERIC_MASK: u32 = 0x00000fff;
pub const OMAPFB_CAPS_LCDC_MASK: u32 = 0x00fff000;
pub const OMAPFB_CAPS_PANEL_MASK: u32 = 0xff000000;
pub const OMAPFB_CAPS_MANUAL_UPDATE: u32 = 0x00001000;
pub const OMAPFB_CAPS_TEARSYNC: u32 = 0x00002000;
pub const OMAPFB_CAPS_PLANE_RELOCATE_MEM: u32 = 0x00004000;
pub const OMAPFB_CAPS_PLANE_SCALE: u32 = 0x00008000;
pub const OMAPFB_CAPS_WINDOW_PIXEL_DOUBLE: u32 = 0x00010000;
pub const OMAPFB_CAPS_WINDOW_SCALE: u32 = 0x00020000;
pub const OMAPFB_CAPS_WINDOW_OVERLAY: u32 = 0x00040000;
pub const OMAPFB_CAPS_WINDOW_ROTATE: u32 = 0x00080000;
pub const OMAPFB_CAPS_SET_BACKLIGHT: u32 = 0x01000000;

/* Values from DSP must map to lower 16-bits */
pub const OMAPFB_FORMAT_MASK: u32 = 0x00ff;
pub const OMAPFB_FORMAT_FLAG_DOUBLE: u32 = 0x0100;
pub const OMAPFB_FORMAT_FLAG_TEARSYNC: u32 = 0x0200;
pub const OMAPFB_FORMAT_FLAG_FORCE_VSYNC: u32 = 0x0400;
pub const OMAPFB_FORMAT_FLAG_ENABLE_OVERLAY: u32 = 0x0800;
pub const OMAPFB_FORMAT_FLAG_DISABLE_OVERLAY: u32 = 0x1000;
pub const OMAPFB_MEMTYPE_SDRAM: u32 = 0;
pub const OMAPFB_MEMTYPE_SRAM: u32 = 1;
pub const OMAPFB_MEMTYPE_MAX: u32 = 1;
pub const OMAPFB_MEM_IDX_ENABLED: u32 = 0x80;
pub const OMAPFB_MEM_IDX_MASK: u32 = 0x7f;

#[repr(u32)]
pub enum omapfb_color_format {
    OMAPFB_COLOR_RGB565 = 0,
    OMAPFB_COLOR_YUV422,
    OMAPFB_COLOR_YUV420,
    OMAPFB_COLOR_CLUT_8BPP,
    OMAPFB_COLOR_CLUT_4BPP,
    OMAPFB_COLOR_CLUT_2BPP,
    OMAPFB_COLOR_CLUT_1BPP,
    OMAPFB_COLOR_RGB444,
    OMAPFB_COLOR_YUY422,
    OMAPFB_COLOR_ARGB16,
    OMAPFB_COLOR_RGB24U,
    OMAPFB_COLOR_RGB24P,
    OMAPFB_COLOR_ARGB32,
    OMAPFB_COLOR_RGBA32,
    OMAPFB_COLOR_RGBX32,
}

#[repr(C)]
pub struct omapfb_update_window { pub x: __u32, pub y: __u32, pub width: __u32, pub height: __u32, pub format: __u32, pub out_x: __u32, pub out_y: __u32, pub out_width: __u32, pub out_height: __u32, pub reserved: [__u32; 8] }
#[repr(C)]
pub struct omapfb_update_window_old { pub x: __u32, pub y: __u32, pub width: __u32, pub height: __u32, pub format: __u32 }

#[repr(u32)] pub enum omapfb_plane { OMAPFB_PLANE_GFX = 0, OMAPFB_PLANE_VID1, OMAPFB_PLANE_VID2 }
#[repr(u32)] pub enum omapfb_channel_out { OMAPFB_CHANNEL_OUT_LCD = 0, OMAPFB_CHANNEL_OUT_DIGIT }
#[repr(C)] pub struct omapfb_plane_info { pub pos_x: __u32, pub pos_y: __u32, pub enabled: __u8, pub channel_out: __u8, pub mirror: __u8, pub mem_idx: __u8, pub out_width: __u32, pub out_height: __u32, pub reserved2: [__u32; 12] }
#[repr(C)] pub struct omapfb_mem_info { pub size: __u32, pub type_: __u8, pub reserved: [__u8; 3] }
#[repr(C)] pub struct omapfb_caps { pub ctrl: __u32, pub plane_color: __u32, pub wnd_color: __u32 }
#[repr(u32)] pub enum omapfb_color_key_type { OMAPFB_COLOR_KEY_DISABLED = 0, OMAPFB_COLOR_KEY_GFX_DST, OMAPFB_COLOR_KEY_VID_SRC }
#[repr(C)] pub struct omapfb_color_key { pub channel_out: __u8, pub background: __u32, pub trans_key: __u32, pub key_type: __u8 }
#[repr(u32)] pub enum omapfb_update_mode { OMAPFB_UPDATE_DISABLED = 0, OMAPFB_AUTO_UPDATE, OMAPFB_MANUAL_UPDATE }
#[repr(C)] pub struct omapfb_memory_read { pub x: __u16, pub y: __u16, pub w: __u16, pub h: __u16, pub buffer_size: usize, pub buffer: *mut core::ffi::c_void }
#[repr(C)] pub struct omapfb_ovl_colormode { pub overlay_idx: __u8, pub mode_idx: __u8, pub bits_per_pixel: __u32, pub nonstd: __u32, pub red: fb_bitfield, pub green: fb_bitfield, pub blue: fb_bitfield, pub transp: fb_bitfield }
#[repr(C)] pub struct omapfb_vram_info { pub total: __u32, pub free: __u32, pub largest_free_block: __u32, pub reserved: [__u32; 5] }
#[repr(C)] pub struct omapfb_tearsync_info { pub enabled: __u8, pub reserved1: [__u8; 3], pub line: __u16, pub reserved2: __u16 }
#[repr(C)] pub struct omapfb_display_info { pub xres: __u16, pub yres: __u16, pub width: __u32, pub height: __u32, pub reserved: [__u32; 5] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
