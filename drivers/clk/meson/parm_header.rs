/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2015 Endless Mobile, Inc.
 * Author: Carlo Caione <carlo@endlessm.com>
 */

// Dependency intent: linux/bits.h and linux/regmap.h provide GENMASK and regmap APIs.

macro_rules! PMASK {
    ($width:expr) => {
        (((1 as u32) << (($width) - 1)) - 1) | ((1 as u32) << (($width) - 1))
    };
}

macro_rules! SETPMASK {
    ($width:expr, $shift:expr) => {
        (((1 as u32) << (($shift) + ($width) - 1)) - 1
            | ((1 as u32) << (($shift) + ($width) - 1)))
            & !(((1 as u32) << ($shift)) - 1)
    };
}

macro_rules! CLRPMASK {
    ($width:expr, $shift:expr) => {
        !SETPMASK!($width, $shift)
    };
}

macro_rules! PARM_GET {
    ($width:expr, $shift:expr, $reg:expr) => {
        (($reg & SETPMASK!($width, $shift)) >> ($shift))
    };
}

macro_rules! PARM_SET {
    ($width:expr, $shift:expr, $reg:expr, $val:expr) => {
        (($reg & CLRPMASK!($width, $shift)) | (($val) << ($shift)))
    };
}

macro_rules! MESON_PARM_APPLICABLE {
    ($p:expr) => {
        (($p).width != 0)
    };
}

#[repr(C)]
pub struct parm {
    pub reg_off: u16,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

extern "C" {
    pub fn regmap_read(map: *mut regmap, reg: u16, val: *mut u32) -> i32;
    pub fn regmap_update_bits(map: *mut regmap, reg: u16, mask: u32, val: u32) -> i32;
}

#[inline]
pub unsafe fn meson_parm_read(map: *mut regmap, p: *mut parm) -> u32 {
    let mut val: u32 = 0;

    regmap_read(map, (*p).reg_off, &mut val);
    PARM_GET!((*p).width as u32, (*p).shift as u32, val)
}

#[inline]
pub unsafe fn meson_parm_write(map: *mut regmap, p: *mut parm, val: u32) {
    regmap_update_bits(
        map,
        (*p).reg_off,
        SETPMASK!((*p).width as u32, (*p).shift as u32),
        val << (*p).shift,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
