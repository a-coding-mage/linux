/* SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

pub const DEFAULT_LCD_BWIDTH: c_int = 40;
pub const DEFAULT_LCD_HWIDTH: c_int = 64;

/* External types supplied by the surrounding charlcd interface. */
#[repr(C)]
pub struct charlcd {
    _private: [u8; 0],
}

#[repr(C)]
pub enum charlcd_shift_dir {}

#[repr(C)]
pub enum charlcd_onoff {}

#[repr(C)]
pub enum charlcd_fontsize {}

#[repr(C)]
pub enum charlcd_lines {}

#[repr(C)]
pub struct hd44780_common {
    pub ifwidth: c_int, /* 4-bit or 8-bit (default) */
    pub bwidth: c_int, /* Default set by hd44780_alloc() */
    pub hwidth: c_int, /* Default set by hd44780_alloc() */
    pub hd44780_common_flags: c_ulong,
    pub write_data:
        Option<unsafe extern "C" fn(hdc: *mut hd44780_common, data: c_int)>,
    pub write_cmd:
        Option<unsafe extern "C" fn(hdc: *mut hd44780_common, cmd: c_int)>,
    /* write_cmd_raw4 is for 4-bit connected displays only */
    pub write_cmd_raw4:
        Option<unsafe extern "C" fn(hdc: *mut hd44780_common, cmd: c_int)>,
    pub hd44780: *mut c_void,
}

extern "C" {
    pub fn hd44780_common_print(lcd: *mut charlcd, c: c_int) -> c_int;
    pub fn hd44780_common_gotoxy(
        lcd: *mut charlcd,
        x: c_uint,
        y: c_uint,
    ) -> c_int;
    pub fn hd44780_common_home(lcd: *mut charlcd) -> c_int;
    pub fn hd44780_common_clear_display(lcd: *mut charlcd) -> c_int;
    pub fn hd44780_common_init_display(lcd: *mut charlcd) -> c_int;
    pub fn hd44780_common_shift_cursor(
        lcd: *mut charlcd,
        dir: charlcd_shift_dir,
    ) -> c_int;
    pub fn hd44780_common_shift_display(
        lcd: *mut charlcd,
        dir: charlcd_shift_dir,
    ) -> c_int;
    pub fn hd44780_common_display(
        lcd: *mut charlcd,
        on: charlcd_onoff,
    ) -> c_int;
    pub fn hd44780_common_cursor(
        lcd: *mut charlcd,
        on: charlcd_onoff,
    ) -> c_int;
    pub fn hd44780_common_blink(
        lcd: *mut charlcd,
        on: charlcd_onoff,
    ) -> c_int;
    pub fn hd44780_common_fontsize(
        lcd: *mut charlcd,
        size: charlcd_fontsize,
    ) -> c_int;
    pub fn hd44780_common_lines(
        lcd: *mut charlcd,
        lines: charlcd_lines,
    ) -> c_int;
    pub fn hd44780_common_redefine_char(
        lcd: *mut charlcd,
        esc: *mut c_char,
    ) -> c_int;

    pub fn hd44780_common_alloc() -> *mut charlcd;
    pub fn hd44780_common_free(lcd: *mut charlcd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
