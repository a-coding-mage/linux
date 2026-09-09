/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Copyright (C) 1996-2000 Vojtech Pavlik
 *
 *  Sponsored by SuSE
 */
/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

// Dependencies supplied by the corresponding Linux UAPI headers are intentionally
// referenced rather than reimplemented here.

pub const JS_VERSION: u32 = 0x020100;

pub const JS_EVENT_BUTTON: u8 = 0x01; // button pressed/released
pub const JS_EVENT_AXIS: u8 = 0x02; // joystick moved
pub const JS_EVENT_INIT: u8 = 0x80; // initial state of device

#[repr(C)]
pub struct js_event {
    pub time: u32,   // event timestamp in milliseconds
    pub value: i16,  // value
    pub type_: u8,   // event type
    pub number: u8,  // axis/button number
}

// IOCTL commands for joystick driver. _IOR, _IOW, _IOC and the referenced
// Linux input constants are supplied by the corresponding UAPI dependencies.
pub const JSIOCGVERSION: _ = _IOR('j', 0x01, u32); // get driver version
pub const JSIOCGAXES: _ = _IOR('j', 0x11, u8); // get number of axes
pub const JSIOCGBUTTONS: _ = _IOR('j', 0x12, u8); // get number of buttons

#[macro_export]
macro_rules! JSIOCGNAME {
    ($len:expr) => { _IOC(_IOC_READ, 'j', 0x13, $len) };
}

pub const JSIOCSCORR: _ = _IOW('j', 0x21, js_corr); // set correction values
pub const JSIOCGCORR: _ = _IOR('j', 0x22, js_corr); // get correction values
pub const JSIOCSAXMAP: _ = _IOW('j', 0x31, [u8; ABS_CNT]); // set axis mapping
pub const JSIOCGAXMAP: _ = _IOR('j', 0x32, [u8; ABS_CNT]); // get axis mapping
pub const JSIOCSBTNMAP: _ = _IOW('j', 0x33, [u16; KEY_MAX - BTN_MISC + 1]); // set button mapping
pub const JSIOCGBTNMAP: _ = _IOR('j', 0x34, [u16; KEY_MAX - BTN_MISC + 1]); // get button mapping

pub const JS_CORR_NONE: u8 = 0x00; // returns raw values
pub const JS_CORR_BROKEN: u8 = 0x01; // broken line

#[repr(C)]
pub struct js_corr {
    pub coef: [i32; 8],
    pub prec: i16,
    pub type_: u16,
}

pub const JS_TRUE: i32 = 1;
pub const JS_FALSE: i32 = 0;
pub const JS_X_0: i32 = 0x01;
pub const JS_Y_0: i32 = 0x02;
pub const JS_X_1: i32 = 0x04;
pub const JS_Y_1: i32 = 0x08;
pub const JS_MAX: i32 = 2;

pub const JS_DEF_TIMEOUT: i32 = 0x1300;
pub const JS_DEF_CORR: i32 = 0;
pub const JS_DEF_TIMELIMIT: i64 = 10;

pub const JS_SET_CAL: i32 = 1;
pub const JS_GET_CAL: i32 = 2;
pub const JS_SET_TIMEOUT: i32 = 3;
pub const JS_GET_TIMEOUT: i32 = 4;
pub const JS_SET_TIMELIMIT: i32 = 5;
pub const JS_GET_TIMELIMIT: i32 = 6;
pub const JS_GET_ALL: i32 = 7;
pub const JS_SET_ALL: i32 = 8;

#[repr(C)]
pub struct JS_DATA_TYPE {
    pub buttons: i32,
    pub x: i32,
    pub y: i32,
}

pub const JS_RETURN: usize = core::mem::size_of::<JS_DATA_TYPE>();

#[repr(C)]
pub struct JS_DATA_SAVE_TYPE_32 {
    pub JS_TIMEOUT: i32,
    pub BUSY: i32,
    pub JS_EXPIRETIME: i32,
    pub JS_TIMELIMIT: i32,
    pub JS_SAVE: JS_DATA_TYPE,
    pub JS_CORR: JS_DATA_TYPE,
}

#[repr(C)]
pub struct JS_DATA_SAVE_TYPE_64 {
    pub JS_TIMEOUT: i32,
    pub BUSY: i32,
    pub JS_EXPIRETIME: i64,
    pub JS_TIMELIMIT: i64,
    pub JS_SAVE: JS_DATA_TYPE,
    pub JS_CORR: JS_DATA_TYPE,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
