// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
 *            au88x0_cxtalk.c
 *
 *  Wed Nov 19 16:29:47 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.org
 ****************************************************************************/

/*
 */

// C dependency: #include "au88x0_xtalk.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;

type u32 = u32;
type xtalk_gains_t = [i16; XTGAINS_SZ];
type xtalk_dline_t = [i32; 0x20];
type xtalk_instate_t = [i16; 4];
type xtalk_state_t = [[i16; 4]; 5];
type xtalk_coefs_t = [[i16; 5]; 5];

const XTGAINS_SZ: usize = 10;

#[repr(C)]
pub struct vortex_t {
    pub mmio: *mut c_void,
}

extern "C" {
    fn hwwrite(mmio: *mut c_void, addr: u32, data: u32);
    fn hwread(mmio: *mut c_void, addr: u32) -> u32;
}

const fn s(v: u16) -> i16 {
    v as i16
}

/* Data (a whole lot of data.... ) */

static sXtalkWideKLeftEq: i16 = s(0x269C);
static sXtalkWideKRightEq: i16 = s(0x269C);
static sXtalkWideKLeftXt: i16 = s(0xF25E);
static sXtalkWideKRightXt: i16 = s(0xF25E);
static sXtalkWideShiftLeftEq: i16 = 1;
static sXtalkWideShiftRightEq: i16 = 1;
static sXtalkWideShiftLeftXt: i16 = 0;
static sXtalkWideShiftRightXt: i16 = 0;
static wXtalkWideLeftDelay: u16 = 0xd;
static wXtalkWideRightDelay: u16 = 0xd;
static sXtalkNarrowKLeftEq: i16 = s(0x468D);
static sXtalkNarrowKRightEq: i16 = s(0x468D);
static sXtalkNarrowKLeftXt: i16 = s(0xF82E);
static sXtalkNarrowKRightXt: i16 = s(0xF82E);
static sXtalkNarrowShiftLeftEq: i16 = 0x3;
static sXtalkNarrowShiftRightEq: i16 = 0x3;
static sXtalkNarrowShiftLeftXt: i16 = 0;
static sXtalkNarrowShiftRightXt: i16 = 0;
static wXtalkNarrowLeftDelay: u16 = 0x7;
static wXtalkNarrowRightDelay: u16 = 0x7;

static asXtalkGainsDefault: xtalk_gains_t = [
    s(0x4000), s(0x4000), s(0x4000), s(0x4000), s(0x4000),
    s(0x4000), s(0x4000), s(0x4000), s(0x4000), s(0x4000),
];

static asXtalkGainsTest: xtalk_gains_t = [
    s(0x7fff), s(0x8000), s(0x0000), s(0x0000), s(0x0001),
    s(0xffff), s(0x4000), s(0xc000), s(0x0002), s(0xfffe),
];

static asXtalkGains1Chan: xtalk_gains_t = [
    s(0x7FFF), 0, 0, 0, 0,
    s(0x7FFF), 0, 0, 0, 0,
];

// Input gain for 4 A3D slices. One possible input pair is left zero.
static asXtalkGainsAllChan: xtalk_gains_t = [
    s(0x7FFF), s(0x7FFF), s(0x7FFF), s(0x7FFF), 0,
    s(0x7FFF), s(0x7FFF), s(0x7FFF), s(0x7FFF), 0,
];

static asXtalkGainsZeros: xtalk_gains_t = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static alXtalkDlineZeros: xtalk_dline_t = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static alXtalkDlineTest: xtalk_dline_t = [
    0x0000fc18, 0x0fff03e8, 0x000186a0, 0xfffe7960u32 as i32,
    1, 0xffffffffu32 as i32, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

static asXtalkInStateZeros: xtalk_instate_t = [
    0, 0, 0, 0,
];

static asXtalkInStateTest: xtalk_instate_t = [
    s(0x0080), s(0xff80), s(0x0001), s(0xffff),
];

static asXtalkOutStateZeros: xtalk_state_t = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

static sDiamondKLeftEq: i16 = s(0x401d);
static sDiamondKRightEq: i16 = s(0x401d);
static sDiamondKLeftXt: i16 = s(0xF90E);
static sDiamondKRightXt: i16 = s(0xF90E);
static sDiamondShiftLeftEq: i16 = 1;
static sDiamondShiftRightEq: i16 = 1;
static sDiamondShiftLeftXt: i16 = 0;
static sDiamondShiftRightXt: i16 = 0;
static wDiamondLeftDelay: u16 = 0xb;
static wDiamondRightDelay: u16 = 0xb;

static asXtalkWideCoefsLeftEq: xtalk_coefs_t = [
    [s(0xEC4C), s(0xDCE9), s(0xFDC2), s(0xFEEC), 0],
    [s(0x5F60), s(0xCBCB), s(0xFC26), s(0x0305), 0],
    [s(0x340B), s(0xe8f5), s(0x236c), s(0xe40d), 0],
    [s(0x76d5), s(0xc78d), s(0x05ac), s(0xfa5b), 0],
    [s(0x7F04), s(0xC0FA), s(0x0263), s(0xFDA2), 0],
];
static asXtalkWideCoefsRightEq: xtalk_coefs_t = [
    [s(0xEC4C), s(0xDCE9), s(0xFDC2), s(0xFEEC), 0],
    [s(0x5F60), s(0xCBCB), s(0xFC26), s(0x0305), 0],
    [s(0x340B), s(0xe8f5), s(0x236c), s(0xe40d), 0],
    [s(0x76d5), s(0xc78d), s(0x05ac), s(0xfa5b), 0],
    [s(0x7F04), s(0xC0FA), s(0x0263), s(0xFDA2), 0],
];
static asXtalkWideCoefsLeftXt: xtalk_coefs_t = [
    [s(0x55c6), s(0xc97b), s(0x005b), s(0x0047), 0],
    [s(0x6a60), s(0xca20), s(0xffc6), s(0x0040), 0],
    [s(0x6411), s(0xd711), s(0xfca1), s(0x0190), 0],
    [s(0x77dc), s(0xc79e), s(0xffb8), s(0x000a), 0],
    [0, 0, 0, 0, 0],
];
static asXtalkWideCoefsRightXt: xtalk_coefs_t = [
    [s(0x55c6), s(0xc97b), s(0x005b), s(0x0047), 0],
    [s(0x6a60), s(0xca20), s(0xffc6), s(0x0040), 0],
    [s(0x6411), s(0xd711), s(0xfca1), s(0x0190), 0],
    [s(0x77dc), s(0xc79e), s(0xffb8), s(0x000a), 0],
    [0, 0, 0, 0, 0],
];
static asXtalkNarrowCoefsLeftEq: xtalk_coefs_t = [
    [s(0x50B5), s(0xD07C), s(0x026D), s(0xFD21), 0],
    [s(0x460F), s(0xE44F), s(0xF75E), s(0xEFA6), 0],
    [s(0x556D), s(0xDCAB), s(0x2098), s(0xF0F2), 0],
    [s(0x7E03), s(0xC1F0), s(0x007D), s(0xFF89), 0],
    [s(0x383E), s(0xFD9D), s(0xB278), s(0x4547), 0],
];

static asXtalkNarrowCoefsRightEq: xtalk_coefs_t = [
    [s(0x50B5), s(0xD07C), s(0x026D), s(0xFD21), 0],
    [s(0x460F), s(0xE44F), s(0xF75E), s(0xEFA6), 0],
    [s(0x556D), s(0xDCAB), s(0x2098), s(0xF0F2), 0],
    [s(0x7E03), s(0xC1F0), s(0x007D), s(0xFF89), 0],
    [s(0x383E), s(0xFD9D), s(0xB278), s(0x4547), 0],
];

static asXtalkNarrowCoefsLeftXt: xtalk_coefs_t = [
    [s(0x3CB2), s(0xDF49), s(0xF6EA), s(0x095B), 0],
    [s(0x6777), s(0xC915), s(0xFEAF), s(0x00B1), 0],
    [s(0x7762), s(0xC7D9), s(0x025B), s(0xFDA6), 0],
    [s(0x6B7A), s(0xD2AA), s(0xF2FB), s(0x0B64), 0],
    [0, 0, 0, 0, 0],
];

static asXtalkNarrowCoefsRightXt: xtalk_coefs_t = [
    [s(0x3CB2), s(0xDF49), s(0xF6EA), s(0x095B), 0],
    [s(0x6777), s(0xC915), s(0xFEAF), s(0x00B1), 0],
    [s(0x7762), s(0xC7D9), s(0x025B), s(0xFDA6), 0],
    [s(0x6B7A), s(0xD2AA), s(0xF2FB), s(0x0B64), 0],
    [0, 0, 0, 0, 0],
];

static asXtalkCoefsZeros: xtalk_coefs_t = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asXtalkCoefsPipe: xtalk_coefs_t = [
    [0, 0, s(0x0FA0), 0, 0],
    [0, 0, s(0x0FA0), 0, 0],
    [0, 0, s(0x0FA0), 0, 0],
    [0, 0, s(0x0FA0), 0, 0],
    [0, 0, s(0x1180), 0, 0],
];
static asXtalkCoefsNegPipe: xtalk_coefs_t = [
    [0, 0, s(0xF380), 0, 0],
    [0, 0, s(0xF380), 0, 0],
    [0, 0, s(0xF380), 0, 0],
    [0, 0, s(0xF380), 0, 0],
    [0, 0, s(0xF200), 0, 0],
];

static asXtalkCoefsNumTest: xtalk_coefs_t = [
    [0, 0, s(0xF380), s(0x8000), s(0x6D60)],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asXtalkCoefsDenTest: xtalk_coefs_t = [
    [s(0xC000), s(0x2000), s(0x4000), 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asXtalkOutStateTest: xtalk_state_t = [
    [s(0x7FFF), s(0x0004), s(0xFFFC), 0],
    [s(0xFE00), s(0x0008), s(0xFFF8), s(0x4000)],
    [s(0x0200), s(0x0010), s(0xFFF0), s(0xC000)],
    [s(0x8000), s(0x0020), s(0xFFE0), 0],
    [0, 0, 0, 0],
];

static asDiamondCoefsLeftEq: xtalk_coefs_t = [
    [s(0x0F1E), s(0x2D05), s(0xF8E3), s(0x07C8), 0],
    [s(0x45E2), s(0xCA51), s(0x0448), s(0xFCE7), 0],
    [s(0xA93E), s(0xDBD5), s(0x022C), s(0x028A), 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asDiamondCoefsRightEq: xtalk_coefs_t = [
    [s(0x0F1E), s(0x2D05), s(0xF8E3), s(0x07C8), 0],
    [s(0x45E2), s(0xCA51), s(0x0448), s(0xFCE7), 0],
    [s(0xA93E), s(0xDBD5), s(0x022C), s(0x028A), 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asDiamondCoefsLeftXt: xtalk_coefs_t = [
    [s(0x3B50), s(0xFE08), s(0xF959), s(0x0060), 0],
    [s(0x9FCB), s(0xD8F1), s(0x00A2), s(0x003A), 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static asDiamondCoefsRightXt: xtalk_coefs_t = [
    [s(0x3B50), s(0xFE08), s(0xF959), s(0x0060), 0],
    [s(0x9FCB), s(0xD8F1), s(0x00A2), s(0x003A), 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

/**/
/* XTalk EQ and XT */

unsafe fn vortex_XtalkHw_SetLeftEQ(vortex: *mut vortex_t, arg_0: i16, arg_4: i16, coefs: &xtalk_coefs_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x24200 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24204 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24208 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x2420c + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24210 + (i as u32) * 0x24, coefs[i][4] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24538, (arg_0 as u16) as u32);
    hwwrite((*vortex).mmio, 0x2453C, (arg_4 as u16) as u32);
}

unsafe fn vortex_XtalkHw_SetRightEQ(vortex: *mut vortex_t, arg_0: i16, arg_4: i16, coefs: &xtalk_coefs_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x242b4 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242b8 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242bc + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242c0 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242c4 + (i as u32) * 0x24, coefs[i][4] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24540, (arg_0 as u16) as u32);
    hwwrite((*vortex).mmio, 0x24544, (arg_4 as u16) as u32);
}

unsafe fn vortex_XtalkHw_SetLeftXT(vortex: *mut vortex_t, arg_0: i16, arg_4: i16, coefs: &xtalk_coefs_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x24368 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x2436c + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24370 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24374 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24378 + (i as u32) * 0x24, coefs[i][4] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24548, (arg_0 as u16) as u32);
    hwwrite((*vortex).mmio, 0x2454C, (arg_4 as u16) as u32);
}

unsafe fn vortex_XtalkHw_SetRightXT(vortex: *mut vortex_t, arg_0: i16, arg_4: i16, coefs: &xtalk_coefs_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x2441C + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24420 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24424 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24428 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
        hwwrite((*vortex).mmio, 0x2442C + (i as u32) * 0x24, coefs[i][4] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24550, (arg_0 as u16) as u32);
    hwwrite((*vortex).mmio, 0x24554, (arg_4 as u16) as u32);
}

unsafe fn vortex_XtalkHw_SetLeftEQStates(vortex: *mut vortex_t, arg_0: &xtalk_instate_t, coefs: &xtalk_state_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x24214 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24218 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x2421C + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24220 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x244F8, arg_0[0] as u16 as u32);
    hwwrite((*vortex).mmio, 0x244FC, arg_0[1] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24500, arg_0[2] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24504, arg_0[3] as u16 as u32);
}

unsafe fn vortex_XtalkHw_SetRightEQStates(vortex: *mut vortex_t, arg_0: &xtalk_instate_t, coefs: &xtalk_state_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x242C8 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242CC + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x242D0 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x244D4 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24508, arg_0[0] as u16 as u32);
    hwwrite((*vortex).mmio, 0x2450C, arg_0[1] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24510, arg_0[2] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24514, arg_0[3] as u16 as u32);
}

unsafe fn vortex_XtalkHw_SetLeftXTStates(vortex: *mut vortex_t, arg_0: &xtalk_instate_t, coefs: &xtalk_state_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x2437C + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24380 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24384 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24388 + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24518, arg_0[0] as u16 as u32);
    hwwrite((*vortex).mmio, 0x2451C, arg_0[1] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24520, arg_0[2] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24524, arg_0[3] as u16 as u32);
}

unsafe fn vortex_XtalkHw_SetRightXTStates(vortex: *mut vortex_t, arg_0: &xtalk_instate_t, coefs: &xtalk_state_t) {
    for i in 0..5usize {
        hwwrite((*vortex).mmio, 0x24430 + (i as u32) * 0x24, coefs[i][0] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24434 + (i as u32) * 0x24, coefs[i][1] as u16 as u32);
        hwwrite((*vortex).mmio, 0x24438 + (i as u32) * 0x24, coefs[i][2] as u16 as u32);
        hwwrite((*vortex).mmio, 0x2443C + (i as u32) * 0x24, coefs[i][3] as u16 as u32);
    }
    hwwrite((*vortex).mmio, 0x24528, arg_0[0] as u16 as u32);
    hwwrite((*vortex).mmio, 0x2452C, arg_0[1] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24530, arg_0[2] as u16 as u32);
    hwwrite((*vortex).mmio, 0x24534, arg_0[3] as u16 as u32);
}

// C #if 0 block omitted from active Rust: disabled getter helpers for EQ, XT, states, gains,
// delay, dline, control register, and sample rate used hwread to mirror the setters.

/* Gains */

unsafe fn vortex_XtalkHw_SetGains(vortex: *mut vortex_t, gains: &xtalk_gains_t) {
    for i in 0..XTGAINS_SZ {
        hwwrite((*vortex).mmio, 0x244D0 + ((i as u32) * 4), gains[i] as u16 as u32);
    }
}

unsafe fn vortex_XtalkHw_SetGainsAllChan(vortex: *mut vortex_t) {
    vortex_XtalkHw_SetGains(vortex, &asXtalkGainsAllChan);
}

/* Delay parameters */

unsafe fn vortex_XtalkHw_SetDelay(vortex: *mut vortex_t, right: u16, left: u16) {
    let mut esp0: u32 = 0;

    esp0 &= 0x1FFFFFFF;
    esp0 |= 0xA0000000;
    esp0 = (esp0 & 0xffffE0ff) | (((right & 0x1F) as u32) << 8);
    esp0 = (esp0 & 0xfffc1fff) | (((left & 0x1F) as u32) << 0xd);

    hwwrite((*vortex).mmio, 0x24660, esp0);
}

unsafe fn vortex_XtalkHw_SetLeftDline(vortex: *mut vortex_t, dline: &xtalk_dline_t) {
    for i in 0..0x20usize {
        hwwrite((*vortex).mmio, 0x24000 + ((i as u32) << 2), (dline[i] as u32) & 0xffff);
        hwwrite((*vortex).mmio, 0x24080 + ((i as u32) << 2), (dline[i] >> 0x10) as u32);
    }
}

unsafe fn vortex_XtalkHw_SetRightDline(vortex: *mut vortex_t, dline: &xtalk_dline_t) {
    for i in 0..0x20usize {
        hwwrite((*vortex).mmio, 0x24100 + ((i as u32) << 2), (dline[i] as u32) & 0xffff);
        hwwrite((*vortex).mmio, 0x24180 + ((i as u32) << 2), (dline[i] >> 0x10) as u32);
    }
}

/* Control/Global stuff */

unsafe fn vortex_XtalkHw_SetSampleRate(vortex: *mut vortex_t, sr: u32) {
    let mut temp: u32;

    temp = (hwread((*vortex).mmio, 0x24660) & 0x1FFFFFFF) | 0xC0000000;
    temp = (temp & 0xffffff07) | ((sr & 0x1f) << 3);
    hwwrite((*vortex).mmio, 0x24660, temp);
}

unsafe fn vortex_XtalkHw_Enable(vortex: *mut vortex_t) {
    let mut temp: u32;

    temp = (hwread((*vortex).mmio, 0x24660) & 0x1FFFFFFF) | 0xC0000000;
    temp |= 1;
    hwwrite((*vortex).mmio, 0x24660, temp);
}

unsafe fn vortex_XtalkHw_Disable(vortex: *mut vortex_t) {
    let mut temp: u32;

    temp = (hwread((*vortex).mmio, 0x24660) & 0x1FFFFFFF) | 0xC0000000;
    temp &= 0xfffffffe;
    hwwrite((*vortex).mmio, 0x24660, temp);
}

unsafe fn vortex_XtalkHw_ZeroIO(vortex: *mut vortex_t) {
    for i in 0..20u32 {
        hwwrite((*vortex).mmio, 0x24600 + (i << 2), 0);
    }
    for i in 0..4u32 {
        hwwrite((*vortex).mmio, 0x24650 + (i << 2), 0);
    }
}

unsafe fn vortex_XtalkHw_ZeroState(vortex: *mut vortex_t) {
    vortex_XtalkHw_ZeroIO(vortex); // inlined

    vortex_XtalkHw_SetLeftEQ(vortex, 0, 0, &asXtalkCoefsZeros);
    vortex_XtalkHw_SetRightEQ(vortex, 0, 0, &asXtalkCoefsZeros);

    vortex_XtalkHw_SetLeftXT(vortex, 0, 0, &asXtalkCoefsZeros);
    vortex_XtalkHw_SetRightXT(vortex, 0, 0, &asXtalkCoefsZeros);

    vortex_XtalkHw_SetGains(vortex, &asXtalkGainsZeros); // inlined

    vortex_XtalkHw_SetDelay(vortex, 0, 0); // inlined

    vortex_XtalkHw_SetLeftDline(vortex, &alXtalkDlineZeros); // inlined
    vortex_XtalkHw_SetRightDline(vortex, &alXtalkDlineZeros); // inlined
    vortex_XtalkHw_SetLeftDline(vortex, &alXtalkDlineZeros); // inlined
    vortex_XtalkHw_SetRightDline(vortex, &alXtalkDlineZeros); // inlined

    vortex_XtalkHw_SetLeftEQStates(vortex, &asXtalkInStateZeros, &asXtalkOutStateZeros);
    vortex_XtalkHw_SetRightEQStates(vortex, &asXtalkInStateZeros, &asXtalkOutStateZeros);
    vortex_XtalkHw_SetLeftXTStates(vortex, &asXtalkInStateZeros, &asXtalkOutStateZeros);
    vortex_XtalkHw_SetRightXTStates(vortex, &asXtalkInStateZeros, &asXtalkOutStateZeros);
}

unsafe fn vortex_XtalkHw_ProgramPipe(vortex: *mut vortex_t) {
    vortex_XtalkHw_SetLeftEQ(vortex, 0, 1, &asXtalkCoefsPipe);
    vortex_XtalkHw_SetRightEQ(vortex, 0, 1, &asXtalkCoefsPipe);
    vortex_XtalkHw_SetLeftXT(vortex, 0, 0, &asXtalkCoefsZeros);
    vortex_XtalkHw_SetRightXT(vortex, 0, 0, &asXtalkCoefsZeros);

    vortex_XtalkHw_SetDelay(vortex, 0, 0); // inlined
}

unsafe fn vortex_XtalkHw_ProgramXtalkWide(vortex: *mut vortex_t) {
    vortex_XtalkHw_SetLeftEQ(
        vortex,
        sXtalkWideKLeftEq,
        sXtalkWideShiftLeftEq,
        &asXtalkWideCoefsLeftEq,
    );
    vortex_XtalkHw_SetRightEQ(
        vortex,
        sXtalkWideKRightEq,
        sXtalkWideShiftRightEq,
        &asXtalkWideCoefsRightEq,
    );
    vortex_XtalkHw_SetLeftXT(
        vortex,
        sXtalkWideKLeftXt,
        sXtalkWideShiftLeftXt,
        &asXtalkWideCoefsLeftXt,
    );
    vortex_XtalkHw_SetRightXT(
        vortex,
        sXtalkWideKLeftXt,
        sXtalkWideShiftLeftXt,
        &asXtalkWideCoefsLeftXt,
    );

    vortex_XtalkHw_SetDelay(vortex, wXtalkWideRightDelay, wXtalkWideLeftDelay); // inlined
}

unsafe fn vortex_XtalkHw_ProgramXtalkNarrow(vortex: *mut vortex_t) {
    vortex_XtalkHw_SetLeftEQ(
        vortex,
        sXtalkNarrowKLeftEq,
        sXtalkNarrowShiftLeftEq,
        &asXtalkNarrowCoefsLeftEq,
    );
    vortex_XtalkHw_SetRightEQ(
        vortex,
        sXtalkNarrowKRightEq,
        sXtalkNarrowShiftRightEq,
        &asXtalkNarrowCoefsRightEq,
    );
    vortex_XtalkHw_SetLeftXT(
        vortex,
        sXtalkNarrowKLeftXt,
        sXtalkNarrowShiftLeftXt,
        &asXtalkNarrowCoefsLeftXt,
    );
    vortex_XtalkHw_SetRightXT(
        vortex,
        sXtalkNarrowKLeftXt,
        sXtalkNarrowShiftLeftXt,
        &asXtalkNarrowCoefsLeftXt,
    );

    vortex_XtalkHw_SetDelay(vortex, wXtalkNarrowRightDelay, wXtalkNarrowLeftDelay); // inlined
}

unsafe fn vortex_XtalkHw_ProgramDiamondXtalk(vortex: *mut vortex_t) {
    //sDiamondKLeftEq,sDiamondKRightXt,asDiamondCoefsLeftEq
    vortex_XtalkHw_SetLeftEQ(
        vortex,
        sDiamondKLeftEq,
        sDiamondShiftLeftEq,
        &asDiamondCoefsLeftEq,
    );
    vortex_XtalkHw_SetRightEQ(
        vortex,
        sDiamondKRightEq,
        sDiamondShiftRightEq,
        &asDiamondCoefsRightEq,
    );
    vortex_XtalkHw_SetLeftXT(
        vortex,
        sDiamondKLeftXt,
        sDiamondShiftLeftXt,
        &asDiamondCoefsLeftXt,
    );
    vortex_XtalkHw_SetRightXT(
        vortex,
        sDiamondKLeftXt,
        sDiamondShiftLeftXt,
        &asDiamondCoefsLeftXt,
    );

    vortex_XtalkHw_SetDelay(vortex, wDiamondRightDelay, wDiamondLeftDelay); // inlined
}

unsafe fn vortex_XtalkHw_init(vortex: *mut vortex_t) {
    vortex_XtalkHw_ZeroState(vortex);
}

/* End of file */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
