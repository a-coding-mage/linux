/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * linux/include/video/mmp_disp.h
 * Header file for Marvell MMP Display Controller
 *
 * Copyright (C) 2012 Marvell Technology Group Ltd.
 * Authors: Zhou Zhu <zzhu3@marvell.com>
 */

use core::ffi::{c_char, c_void};

// Supplied by the Linux/kernel dependencies in the original header.
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum Pixfmt {
    PIXFMT_UYVY = 0,
    PIXFMT_VYUY,
    PIXFMT_YUYV,
    PIXFMT_YUV422P,
    PIXFMT_YVU422P,
    PIXFMT_YUV420P,
    PIXFMT_YVU420P,
    PIXFMT_RGB565 = 0x100,
    PIXFMT_BGR565,
    PIXFMT_RGB1555,
    PIXFMT_BGR1555,
    PIXFMT_RGB888PACK,
    PIXFMT_BGR888PACK,
    PIXFMT_RGB888UNPACK,
    PIXFMT_BGR888UNPACK,
    PIXFMT_RGBA888,
    PIXFMT_BGRA888,
    PIXFMT_RGB666,
    PIXFMT_PSEUDOCOLOR = 0x200,
}

pub const fn pixfmt_to_stride(pix_fmt: i32) -> i32 {
    match pix_fmt {
        0x100 | 0x101 | 0x102 | 0x103 | 0 | 1 | 2 => 2,
        0x106 | 0x107 | 0x108 | 0x109 => 4,
        0x104 | 0x105 => 3,
        3 | 4 | 5 | 6 | 0x200 => 1,
        _ => 0,
    }
}

#[repr(C)]
pub struct mmp_win {
    pub xsrc: u16,
    pub ysrc: u16,
    pub xdst: u16,
    pub ydst: u16,
    pub xpos: u16,
    pub ypos: u16,
    pub left_crop: u16,
    pub right_crop: u16,
    pub up_crop: u16,
    pub bottom_crop: u16,
    pub pix_fmt: i32,
    pub pitch: [u32; 3],
}

#[repr(C)]
pub struct mmp_addr {
    pub phys: [u32; 6],
}

#[repr(C)]
pub struct mmp_mode {
    pub name: *const c_char,
    pub refresh: u32,
    pub xres: u32,
    pub yres: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub hsync_invert: u32,
    pub vsync_invert: u32,
    pub invert_pixclock: u32,
    pub pixclock_freq: u32,
    pub pix_fmt_out: i32,
}

pub struct mmp_path;
pub struct mmp_overlay;
pub struct mmp_panel;

pub const MMP_OFF: i32 = 0;
pub const MMP_ON: i32 = 1;

pub fn stat_name(stat: i32) -> &'static [u8] {
    match stat {
        MMP_OFF => b"OFF\0",
        MMP_ON => b"ON\0",
        _ => b"UNKNOWNSTAT\0",
    }
}

#[repr(C)]
pub struct mmp_overlay_ops {
    pub set_fetch: Option<unsafe extern "C" fn(*mut mmp_overlay, i32)>,
    pub set_onoff: Option<unsafe extern "C" fn(*mut mmp_overlay, i32)>,
    pub set_win: Option<unsafe extern "C" fn(*mut mmp_overlay, *mut mmp_win)>,
    pub set_addr: Option<unsafe extern "C" fn(*mut mmp_overlay, *mut mmp_addr) -> i32>,
}

#[repr(C)]
pub struct mmp_overlay {
    pub id: i32,
    pub name: *const c_char,
    pub path: *mut mmp_path,
    pub dmafetch_id: i32,
    pub addr: mmp_addr,
    pub win: mmp_win,
    pub open_count: i32,
    pub status: i32,
    pub access_ok: mutex,
    pub ops: *const mmp_overlay_ops,
}

pub const PANELTYPE_ACTIVE: i32 = 0;
pub const PANELTYPE_SMART: i32 = 1;
pub const PANELTYPE_TV: i32 = 2;
pub const PANELTYPE_DSI_CMD: i32 = 3;
pub const PANELTYPE_DSI_VIDEO: i32 = 4;

#[repr(C)]
pub struct mmp_panel {
    pub node: list_head,
    pub name: *const c_char,
    pub plat_path_name: *const c_char,
    pub dev: *mut device,
    pub panel_type: i32,
    pub plat_data: *mut c_void,
    pub get_modelist: Option<unsafe extern "C" fn(*mut mmp_panel, *mut *mut mmp_mode) -> i32>,
    pub set_mode: Option<unsafe extern "C" fn(*mut mmp_panel, *mut mmp_mode)>,
    pub set_onoff: Option<unsafe extern "C" fn(*mut mmp_panel, i32)>,
}

#[repr(C)]
pub struct mmp_path_ops {
    pub check_status: Option<unsafe extern "C" fn(*mut mmp_path) -> i32>,
    pub get_overlay: Option<unsafe extern "C" fn(*mut mmp_path, i32) -> *mut mmp_overlay>,
    pub get_modelist: Option<unsafe extern "C" fn(*mut mmp_path, *mut *mut mmp_mode) -> i32>,
    pub set_mode: Option<unsafe extern "C" fn(*mut mmp_path, *mut mmp_mode)>,
    pub set_onoff: Option<unsafe extern "C" fn(*mut mmp_path, i32)>,
}

pub const PATH_OUT_PARALLEL: i32 = 0;
pub const PATH_OUT_DSI: i32 = 1;
pub const PATH_OUT_HDMI: i32 = 2;

#[repr(C)]
pub struct mmp_path {
    pub node: list_head,
    pub dev: *mut device,
    pub id: i32,
    pub name: *const c_char,
    pub output_type: i32,
    pub panel: *mut mmp_panel,
    pub plat_data: *mut c_void,
    pub mode: mmp_mode,
    pub open_count: i32,
    pub status: i32,
    pub access_ok: mutex,
    pub ops: mmp_path_ops,
    pub overlay_num: i32,
    pub overlays: [mmp_overlay; 0],
}

unsafe extern "C" {
    pub fn mmp_get_path(name: *const c_char) -> *mut mmp_path;
}

pub unsafe fn mmp_path_set_mode(path: *mut mmp_path, mode: *mut mmp_mode) {
    if !path.is_null() {
        if let Some(f) = (*path).ops.set_mode { f(path, mode); }
    }
}
pub unsafe fn mmp_path_set_onoff(path: *mut mmp_path, status: i32) {
    if !path.is_null() {
        if let Some(f) = (*path).ops.set_onoff { f(path, status); }
    }
}
pub unsafe fn mmp_path_get_modelist(path: *mut mmp_path, modelist: *mut *mut mmp_mode) -> i32 {
    if !path.is_null() { if let Some(f) = (*path).ops.get_modelist { return f(path, modelist); } }
    0
}
pub unsafe fn mmp_path_get_overlay(path: *mut mmp_path, overlay_id: i32) -> *mut mmp_overlay {
    if !path.is_null() { if let Some(f) = (*path).ops.get_overlay { return f(path, overlay_id); } }
    core::ptr::null_mut()
}
pub unsafe fn mmp_overlay_set_fetch(overlay: *mut mmp_overlay, fetch_id: i32) {
    if !overlay.is_null() { if let Some(f) = (*(*overlay).ops).set_fetch { f(overlay, fetch_id); } }
}
pub unsafe fn mmp_overlay_set_onoff(overlay: *mut mmp_overlay, status: i32) {
    if !overlay.is_null() { if let Some(f) = (*(*overlay).ops).set_onoff { f(overlay, status); } }
}
pub unsafe fn mmp_overlay_set_win(overlay: *mut mmp_overlay, win: *mut mmp_win) {
    if !overlay.is_null() { if let Some(f) = (*(*overlay).ops).set_win { f(overlay, win); } }
}
pub unsafe fn mmp_overlay_set_addr(overlay: *mut mmp_overlay, addr: *mut mmp_addr) -> i32 {
    if !overlay.is_null() { if let Some(f) = (*(*overlay).ops).set_addr { return f(overlay, addr); } }
    0
}

#[repr(C)]
pub struct mmp_path_info {
    pub name: *const c_char,
    pub dev: *mut device,
    pub id: i32,
    pub output_type: i32,
    pub overlay_num: i32,
    pub set_mode: Option<unsafe extern "C" fn(*mut mmp_path, *mut mmp_mode)>,
    pub set_onoff: Option<unsafe extern "C" fn(*mut mmp_path, i32)>,
    pub overlay_ops: *const mmp_overlay_ops,
    pub plat_data: *mut c_void,
}

unsafe extern "C" {
    pub fn mmp_register_path(info: *mut mmp_path_info) -> *mut mmp_path;
    pub fn mmp_unregister_path(path: *mut mmp_path);
    pub fn mmp_register_panel(panel: *mut mmp_panel);
    pub fn mmp_unregister_panel(panel: *mut mmp_panel);
}

#[repr(C)]
pub struct mmp_buffer_driver_mach_info {
    pub name: *const c_char,
    pub path_name: *const c_char,
    pub overlay_id: i32,
    pub dmafetch_id: i32,
    pub default_pixfmt: i32,
}

#[repr(C)]
pub struct mmp_mach_path_config {
    pub name: *const c_char,
    pub overlay_num: i32,
    pub output_type: i32,
    pub path_config: u32,
    pub link_config: u32,
    pub dsi_rbswap: u32,
}

#[repr(C)]
pub struct mmp_mach_plat_info {
    pub name: *const c_char,
    pub clk_name: *const c_char,
    pub path_num: i32,
    pub paths: *mut mmp_mach_path_config,
}

#[repr(C)]
pub struct mmp_mach_panel_info {
    pub name: *const c_char,
    pub plat_set_onoff: Option<unsafe extern "C" fn(i32)>,
    pub plat_path_name: *const c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
