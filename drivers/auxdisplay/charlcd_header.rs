/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Character LCD driver for Linux
 *
 * Copyright (C) 2000-2008, Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2016-2017 Glider bvba
 */

pub const LCD_FLAG_B: u32 = 0x0004; /* Blink on */
pub const LCD_FLAG_C: u32 = 0x0008; /* Cursor on */
pub const LCD_FLAG_D: u32 = 0x0010; /* Display on */
pub const LCD_FLAG_F: u32 = 0x0020; /* Large font mode */
pub const LCD_FLAG_N: u32 = 0x0040; /* 2-rows mode */
pub const LCD_FLAG_L: u32 = 0x0080; /* Backlight enabled */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_onoff {
    CHARLCD_OFF = 0,
    CHARLCD_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_shift_dir {
    CHARLCD_SHIFT_LEFT = 0,
    CHARLCD_SHIFT_RIGHT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_fontsize {
    CHARLCD_FONTSIZE_SMALL = 0,
    CHARLCD_FONTSIZE_LARGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_lines {
    CHARLCD_LINES_1 = 0,
    CHARLCD_LINES_2,
}

#[repr(C)]
pub struct charlcd_ops {
    pub backlight: Option<unsafe extern "C" fn(lcd: *mut charlcd, on: charlcd_onoff)>,
    pub print: Option<unsafe extern "C" fn(lcd: *mut charlcd, c: i32) -> i32>,
    pub gotoxy: Option<unsafe extern "C" fn(lcd: *mut charlcd, x: u32, y: u32) -> i32>,
    pub home: Option<unsafe extern "C" fn(lcd: *mut charlcd) -> i32>,
    pub clear_display: Option<unsafe extern "C" fn(lcd: *mut charlcd) -> i32>,
    pub init_display: Option<unsafe extern "C" fn(lcd: *mut charlcd) -> i32>,
    pub shift_cursor: Option<unsafe extern "C" fn(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32>,
    pub shift_display: Option<unsafe extern "C" fn(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32>,
    pub display: Option<unsafe extern "C" fn(lcd: *mut charlcd, on: charlcd_onoff) -> i32>,
    pub cursor: Option<unsafe extern "C" fn(lcd: *mut charlcd, on: charlcd_onoff) -> i32>,
    pub blink: Option<unsafe extern "C" fn(lcd: *mut charlcd, on: charlcd_onoff) -> i32>,
    pub fontsize: Option<unsafe extern "C" fn(lcd: *mut charlcd, size: charlcd_fontsize) -> i32>,
    pub lines: Option<unsafe extern "C" fn(lcd: *mut charlcd, lines: charlcd_lines) -> i32>,
    pub redefine_char: Option<unsafe extern "C" fn(lcd: *mut charlcd, esc: *mut i8) -> i32>,
}

#[repr(C)]
pub struct charlcd {
    pub ops: *const charlcd_ops,
    pub char_conv: *const u8, /* Optional */

    pub height: i32,
    pub width: i32,

    /* Contains the LCD X and Y offset */
    pub addr: charlcd_addr,

    pub drvdata: *mut core::ffi::c_void, /* Set by charlcd_alloc() */
}

#[repr(C)]
pub struct charlcd_addr {
    pub x: usize,
    pub y: usize,
}

extern "C" {
    pub fn charlcd_backlight(lcd: *mut charlcd, on: charlcd_onoff);

    pub fn charlcd_alloc(drvdata_size: u32) -> *mut charlcd;
    pub fn charlcd_free(lcd: *mut charlcd);

    pub fn charlcd_register(lcd: *mut charlcd) -> i32;
    pub fn charlcd_unregister(lcd: *mut charlcd) -> i32;

    pub fn charlcd_poke(lcd: *mut charlcd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
