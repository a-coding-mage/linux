// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type snd_ctl_elem_type_t = c_int;

const SNDRV_CTL_ELEM_TYPE_BOOLEAN: snd_ctl_elem_type_t = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: snd_ctl_elem_type_t = 2;
const REGMAP_ENDIAN_NATIVE: c_int = 0;
const REGCACHE_FLAT: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const KUNIT_PARAM_DESC_SIZE: usize = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum soc_ops_test_control_layout {
    SOC_OPS_TEST_SINGLE,
    SOC_OPS_TEST_DOUBLE,
    SOC_OPS_TEST_DOUBLE_R,
}

#[repr(C)]
struct kunit {
    priv_: *mut c_void,
    param_value: *const c_void,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
    regmap: *mut regmap,
    io_mutex: mutex,
}

#[repr(C)]
struct soc_ops_test_priv {
    test: *mut kunit,
    component: snd_soc_component,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct soc_mixer_control {
    min: c_int,
    max: c_int,
    platform_max: c_int,
    reg: c_uint,
    shift: c_uint,
    sign_bit: c_uint,
    invert: c_uint,
    rreg: c_uint,
    rshift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_id {
    name: [c_char; 44],
}

#[repr(C)]
struct snd_kcontrol {
    id: snd_ctl_elem_id,
    private_data: *mut c_void,
    private_value: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info {
    type_: snd_ctl_elem_type_t,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

type info_fn = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
type put_fn = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
type get_fn = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
struct info_test_param {
    name: *const c_char,
    func_name: *const c_char,
    layout: soc_ops_test_control_layout,
    mc: soc_mixer_control,
    info: info_fn,
    uinfo: snd_ctl_elem_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct access_test_param {
    func_name: *const c_char,
    layout: soc_ops_test_control_layout,
    mc: soc_mixer_control,
    put: put_fn,
    get: get_fn,
    init: c_uint,
    lmask: c_uint,
    rmask: c_uint,
    lreg: c_uint,
    rreg: c_uint,
    lctl: c_long,
    rctl: c_long,
    ret: c_int,
}

unsafe impl Sync for info_test_param {}
unsafe impl Sync for access_test_param {}

extern "C" {
    fn snd_soc_info_volsw(kctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int;
    fn snd_soc_info_volsw_sx(kctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int;
    fn snd_soc_put_volsw(kctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw_sx(kctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw_sx(kctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int;
    fn kunit_kzalloc(test: *mut kunit, size: size_t, flags: c_uint) -> *mut c_void;
    fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
    fn kunit_device_unregister(test: *mut kunit, dev: *mut device);
    fn devm_regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn mutex_init(lock: *mut mutex);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: size_t) -> isize;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn KUNIT_FAIL(test: *mut kunit, fmt: *const c_char, ...);
    fn KUNIT_ASSERT_FALSE(test: *mut kunit, condition: c_int);
    fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_ASSERT_GE(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_long, right: c_long);
}

const fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

const fn test_mc(
    layout: soc_ops_test_control_layout,
    xmin: c_int,
    xmax: c_int,
    xpmax: c_int,
    xsign: c_uint,
    xinvert: c_uint,
) -> soc_mixer_control {
    soc_mixer_control {
        min: xmin,
        max: xmax,
        platform_max: xpmax,
        reg: 0,
        shift: 0,
        sign_bit: xsign,
        invert: xinvert,
        rreg: if matches!(layout, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE_R) { 1 } else { 0 },
        rshift: if matches!(layout, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE) { 16 } else { 0 },
    }
}

const fn test_uinfo(
    layout: soc_ops_test_control_layout,
    ctype: snd_ctl_elem_type_t,
    cmin: c_long,
    cmax: c_long,
) -> snd_ctl_elem_info {
    snd_ctl_elem_info {
        type_: ctype,
        count: if matches!(layout, soc_ops_test_control_layout::SOC_OPS_TEST_SINGLE) { 1 } else { 2 },
        value: snd_ctl_elem_info_value {
            integer: snd_ctl_elem_info_integer { min: cmin, max: cmax },
        },
    }
}

macro_rules! info_sym {
    (volsw) => {
        snd_soc_info_volsw
    };
    (volsw_sx) => {
        snd_soc_info_volsw_sx
    };
}

macro_rules! put_sym {
    (volsw) => {
        snd_soc_put_volsw
    };
    (volsw_sx) => {
        snd_soc_put_volsw_sx
    };
}

macro_rules! get_sym {
    (volsw) => {
        snd_soc_get_volsw
    };
    (volsw_sx) => {
        snd_soc_get_volsw_sx
    };
}

macro_rules! layout {
    (SINGLE) => {
        soc_ops_test_control_layout::SOC_OPS_TEST_SINGLE
    };
    (DOUBLE) => {
        soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE
    };
    (DOUBLE_R) => {
        soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE_R
    };
}

macro_rules! elem_type {
    (BOOLEAN) => {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    };
    (INTEGER) => {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
}

macro_rules! itest {
    ($cname:expr, $clayout:ident, $ctype:ident, $cfunc:ident, $cmin:expr, $cmax:expr,
     $xmin:expr, $xmax:expr, $xpmax:expr, $xsign:expr, $xinvert:expr) => {
        info_test_param {
            name: cstr(concat!($cname, "\0").as_bytes()),
            func_name: cstr(concat!(stringify!($cfunc), "\0").as_bytes()),
            layout: layout!($clayout),
            info: info_sym!($cfunc),
            mc: test_mc(layout!($clayout), $xmin, $xmax, $xpmax, $xsign, $xinvert),
            uinfo: test_uinfo(layout!($clayout), elem_type!($ctype), $cmin, $cmax),
        }
    };
}

macro_rules! atest {
    ($clayout:ident, $cfunc:ident, $cctl:expr, $cret:expr, $cinit:expr,
     $xmask:expr, $xreg:expr, $xmin:expr, $xmax:expr, $xpmax:expr, $xsign:expr, $xinvert:expr) => {{
        const LAYOUT: soc_ops_test_control_layout = layout!($clayout);
        access_test_param {
            func_name: cstr(concat!(stringify!($cfunc), "\0").as_bytes()),
            layout: LAYOUT,
            put: put_sym!($cfunc),
            get: get_sym!($cfunc),
            mc: test_mc(LAYOUT, $xmin, $xmax, $xpmax, $xsign, $xinvert),
            lctl: $cctl,
            rctl: $cctl,
            lmask: if matches!(LAYOUT, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE) {
                ($xmask) | (($xmask) << 16)
            } else {
                $xmask
            },
            rmask: if matches!(LAYOUT, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE_R) {
                $xmask
            } else {
                0
            },
            init: if $cinit { 0xFFFF_FFFF } else { 0x0000_0000 },
            lreg: if matches!(LAYOUT, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE) {
                ($xreg) | (($xreg) << 16)
            } else {
                $xreg
            },
            rreg: if matches!(LAYOUT, soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE_R) {
                $xreg
            } else {
                0
            },
            ret: $cret,
        }
    }};
}

static all_info_test_params: &[info_test_param] = &[
    // Handling of volume control name for types
    itest!("Test Control",        SINGLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume Control", SINGLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Control",        DOUBLE_R, BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume",         DOUBLE_R, INTEGER, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume Control", DOUBLE_R, BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Control",        DOUBLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume",         DOUBLE,   INTEGER, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Volume Control", DOUBLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 0),
    itest!("Test Control",        SINGLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Volume Control", SINGLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Control",        DOUBLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Volume",         DOUBLE,   INTEGER, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Volume Control", DOUBLE,   BOOLEAN, volsw,    0,  1,   0,  1,  0, 0, 1),
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0,  2,   0,  2,  0, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw,    0,  2,   0,  2,  0, 0, 0),
    itest!("Test Volume Control", SINGLE,   INTEGER, volsw,    0,  2,   0,  2,  0, 0, 0),
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0,  1,   0,  2,  1, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw,    0,  1,   0,  2,  1, 0, 0),
    itest!("Test Volume Control", SINGLE,   INTEGER, volsw,    0,  1,   0,  2,  1, 0, 0),
    // Negative minimums
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0, 20, -10, 10,  0, 4, 0),
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0, 15, -10, 10, 15, 4, 0),
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0, 20, -10, 10,  0, 4, 1),
    itest!("Test Control",        SINGLE,   INTEGER, volsw,    0, 15, -10, 10, 15, 4, 1),
    // SX control volume control naming
    itest!("Test Control",        SINGLE,   BOOLEAN, volsw_sx, 0,  1, 0xF,  1,  0, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw_sx, 0,  1, 0xF,  1,  0, 0, 0),
    itest!("Test Volume Control", SINGLE,   BOOLEAN, volsw_sx, 0,  1, 0xF,  1,  0, 0, 0),
    itest!("Test Control",        SINGLE,   INTEGER, volsw_sx, 0,  4, 0xE,  4,  0, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw_sx, 0,  4, 0xE,  4,  0, 0, 0),
    itest!("Test Volume Control", SINGLE,   INTEGER, volsw_sx, 0,  4, 0xE,  4,  0, 0, 0),
    itest!("Test Control",        SINGLE,   INTEGER, volsw_sx, 0,  3, 0xE,  4,  3, 0, 0),
    itest!("Test Volume",         SINGLE,   INTEGER, volsw_sx, 0,  3, 0xE,  4,  3, 0, 0),
    itest!("Test Volume Control", SINGLE,   INTEGER, volsw_sx, 0,  3, 0xE,  4,  3, 0, 0),
];

static all_access_test_params: &[access_test_param] = &[
    // The C source contains the exhaustive access matrix below; entries are preserved in the same order.
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,      0,   0, false, 0x1F, 0x00,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,     20,   1, false, 0x1F, 0x14,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     25, -22, false, 0x1F, 0x00,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     15,   1, false, 0x1F, 0x0F,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20,  0, 0, 1),
    atest!(SINGLE,   volsw,      0,   1, false, 0x1F, 0x14,    0,  20,  0, 0, 1),
    atest!(SINGLE,   volsw,     20,   0, false, 0x1F, 0x00,    0,  20,  0, 0, 1),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20, 15, 0, 1),
    atest!(SINGLE,   volsw,     25, -22, false, 0x1F, 0x00,    0,  20, 15, 0, 1),
    atest!(SINGLE,   volsw,     15,   1, false, 0x1F, 0x05,    0,  20, 15, 0, 1),
    atest!(SINGLE,   volsw,     10,   1, true,  0x1F, 0x0A,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,      0,   1, true,  0x1F, 0x00,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,     20,   1, true,  0x1F, 0x14,    0,  20,  0, 0, 0),
    atest!(SINGLE,   volsw,     10,   1, true,  0x1F, 0x0A,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     25, -22, true,  0x1F, 0x00,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     15,   1, true,  0x1F, 0x0F,    0,  20, 15, 0, 0),
    atest!(SINGLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10,  0, 4, 0),
    atest!(SINGLE,   volsw,      0,   1, false, 0x1F, 0x16,  -10,  10,  0, 4, 0),
    atest!(SINGLE,   volsw,     20,   1, false, 0x1F, 0x0A,  -10,  10,  0, 4, 0),
    atest!(SINGLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(SINGLE,   volsw,     25, -22, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(SINGLE,   volsw,     15,   1, false, 0x1F, 0x05,  -10,  10, 15, 4, 0),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30,  0, 0, 0),
    atest!(SINGLE,   volsw,      0,   1, false, 0x1F, 0x0A,   10,  30,  0, 0, 0),
    atest!(SINGLE,   volsw,     20,   1, false, 0x1F, 0x1E,   10,  30,  0, 0, 0),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30, 15, 0, 0),
    atest!(SINGLE,   volsw,     25, -22, false, 0x1F, 0x00,   10,  30, 15, 0, 0),
    atest!(SINGLE,   volsw,     15,   1, false, 0x1F, 0x19,   10,  30, 15, 0, 0),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30,  0, 0, 1),
    atest!(SINGLE,   volsw,      0,   1, false, 0x1F, 0x1E,   10,  30,  0, 0, 1),
    atest!(SINGLE,   volsw,     20,   1, false, 0x1F, 0x0A,   10,  30,  0, 0, 1),
    atest!(SINGLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30, 15, 0, 1),
    atest!(SINGLE,   volsw,     25, -22, false, 0x1F, 0x00,   10,  30, 15, 0, 1),
    atest!(SINGLE,   volsw,     15,   1, false, 0x1F, 0x0F,   10,  30, 15, 0, 1),
    atest!(DOUBLE_R, volsw,     10,   1, false, 0x1F, 0x0A,    0,  20,  0, 0, 0),
    atest!(DOUBLE_R, volsw,      0,   0, false, 0x1F, 0x00,    0,  20,  0, 0, 0),
    atest!(DOUBLE_R, volsw,     20,   1, false, 0x1F, 0x14,    0,  20,  0, 0, 0),
    atest!(DOUBLE_R, volsw,     10,   1, false, 0x1F, 0x0A,    0,  20, 15, 0, 0),
    atest!(DOUBLE_R, volsw,     25, -22, false, 0x1F, 0x00,    0,  20, 15, 0, 0),
    atest!(DOUBLE_R, volsw,     15,   1, false, 0x1F, 0x0F,    0,  20, 15, 0, 0),
    atest!(DOUBLE_R, volsw,     10,   0, false, 0x1F, 0x00,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,      0,   1, false, 0x1F, 0x16,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,     20,   1, false, 0x1F, 0x0A,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,     10,   0, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     25, -22, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     15,   1, false, 0x1F, 0x05,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     10,   1, true,  0x1F, 0x00,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,      0,   1, true,  0x1F, 0x16,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,     20,   1, true,  0x1F, 0x0A,  -10,  10,  0, 4, 0),
    atest!(DOUBLE_R, volsw,     10,   1, true,  0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     25, -22, true,  0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     15,   1, true,  0x1F, 0x05,  -10,  10, 15, 4, 0),
    atest!(DOUBLE_R, volsw,     10,   1, true,  0x1F, 0x00,  -10,  10,  0, 4, 1),
    atest!(DOUBLE_R, volsw,      0,   1, true,  0x1F, 0x0A,  -10,  10,  0, 4, 1),
    atest!(DOUBLE_R, volsw,     20,   1, true,  0x1F, 0x16,  -10,  10,  0, 4, 1),
    atest!(DOUBLE_R, volsw,     10,   1, true,  0x1F, 0x00,  -10,  10, 15, 4, 1),
    atest!(DOUBLE_R, volsw,     25, -22, true,  0x1F, 0x00,  -10,  10, 15, 4, 1),
    atest!(DOUBLE_R, volsw,     15,   1, true,  0x1F, 0x1B,  -10,  10, 15, 4, 1),
    atest!(DOUBLE_R, volsw,     10,   1, false, 0x1F, 0x14,   10,  30,  0, 0, 0),
    atest!(DOUBLE_R, volsw,      0,   1, false, 0x1F, 0x0A,   10,  30,  0, 0, 0),
    atest!(DOUBLE_R, volsw,     20,   1, false, 0x1F, 0x1E,   10,  30,  0, 0, 0),
    atest!(DOUBLE_R, volsw,     10,   1, false, 0x1F, 0x14,   10,  30, 15, 0, 0),
    atest!(DOUBLE_R, volsw,     25, -22, false, 0x1F, 0x00,   10,  30, 15, 0, 0),
    atest!(DOUBLE_R, volsw,     15,   1, false, 0x1F, 0x19,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20,  0, 0, 0),
    atest!(DOUBLE,   volsw,      0,   0, false, 0x1F, 0x00,    0,  20,  0, 0, 0),
    atest!(DOUBLE,   volsw,     20,   1, false, 0x1F, 0x14,    0,  20,  0, 0, 0),
    atest!(DOUBLE,   volsw,     10,   1, false, 0x1F, 0x0A,    0,  20, 15, 0, 0),
    atest!(DOUBLE,   volsw,     25, -22, false, 0x1F, 0x00,    0,  20, 15, 0, 0),
    atest!(DOUBLE,   volsw,     15,   1, false, 0x1F, 0x0F,    0,  20, 15, 0, 0),
    atest!(DOUBLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10,  0, 4, 0),
    atest!(DOUBLE,   volsw,      0,   1, false, 0x1F, 0x16,  -10,  10,  0, 4, 0),
    atest!(DOUBLE,   volsw,     20,   1, false, 0x1F, 0x0A,  -10,  10,  0, 4, 0),
    atest!(DOUBLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE,   volsw,     25, -22, false, 0x1F, 0x00,  -10,  10, 15, 4, 0),
    atest!(DOUBLE,   volsw,     15,   1, false, 0x1F, 0x05,  -10,  10, 15, 4, 0),
    atest!(DOUBLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10,  0, 4, 1),
    atest!(DOUBLE,   volsw,      0,   1, false, 0x1F, 0x0A,  -10,  10,  0, 4, 1),
    atest!(DOUBLE,   volsw,     20,   1, false, 0x1F, 0x16,  -10,  10,  0, 4, 1),
    atest!(DOUBLE,   volsw,     10,   0, false, 0x1F, 0x00,  -10,  10, 15, 4, 1),
    atest!(DOUBLE,   volsw,     25, -22, false, 0x1F, 0x00,  -10,  10, 15, 4, 1),
    atest!(DOUBLE,   volsw,     15,   1, false, 0x1F, 0x1B,  -10,  10, 15, 4, 1),
    atest!(DOUBLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,      0,   1, false, 0x1F, 0x0A,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,     20,   1, false, 0x1F, 0x1E,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,     10,   1, false, 0x1F, 0x14,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     25, -22, false, 0x1F, 0x00,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     15,   1, false, 0x1F, 0x19,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     10,   1, true,  0x1F, 0x14,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,      0,   1, true,  0x1F, 0x0A,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,     20,   1, true,  0x1F, 0x1E,   10,  30,  0, 0, 0),
    atest!(DOUBLE,   volsw,     10,   1, true,  0x1F, 0x14,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     25, -22, true,  0x1F, 0x00,   10,  30, 15, 0, 0),
    atest!(DOUBLE,   volsw,     15,   1, true,  0x1F, 0x19,   10,  30, 15, 0, 0),
    atest!(SINGLE,   volsw_sx,   0,   1, false,  0xF, 0x0F, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   1,   0, false,  0xF, 0x00, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   2,   1, false,  0xF, 0x01, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   3,   1, false,  0xF, 0x02, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   4,   1, false,  0xF, 0x03, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   5, -22, false,  0xF, 0x00, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   0,   0, true,   0xF, 0x0F, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   1,   1, true,   0xF, 0x00, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   2,   1, true,   0xF, 0x01, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   3,   1, true,   0xF, 0x02, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   4,   1, true,   0xF, 0x03, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   5, -22, true,   0xF, 0x00, 0x0F,   4,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   0,   1, false, 0x1F, 0x03, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   1,   1, false, 0x1F, 0x02, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   2,   1, false, 0x1F, 0x01, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   3,   0, false, 0x1F, 0x00, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   4,   1, false, 0x1F, 0x0F, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   5, -22, false, 0x1F, 0x00, 0x0F,   4,  0, 0, 1),
    atest!(SINGLE,   volsw_sx,   0,   1, false, 0xFF, 0x88, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   1,   1, false, 0xFF, 0x89, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 119,   1, false, 0xFF, 0xFF, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 120,   0, false, 0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 121,   1, false, 0xFF, 0x01, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 143,   1, false, 0xFF, 0x17, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 144,   1, false, 0xFF, 0x18, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 145, -22, false, 0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   0,   1, true,  0xFF, 0x88, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx,   1,   1, true,  0xFF, 0x89, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 119,   0, true,  0xFF, 0xFF, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 120,   1, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 121,   1, true,  0xFF, 0x01, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 143,   1, true,  0xFF, 0x17, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 144,   1, true,  0xFF, 0x18, 0x88, 144,  0, 0, 0),
    atest!(SINGLE,   volsw_sx, 145, -22, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx,   0,   1, true,  0xFF, 0x88, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx,   1,   1, true,  0xFF, 0x89, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 119,   0, true,  0xFF, 0xFF, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 120,   1, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 121,   1, true,  0xFF, 0x01, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 143,   1, true,  0xFF, 0x17, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 144,   1, true,  0xFF, 0x18, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE,   volsw_sx, 145, -22, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx,   0,   1, true,  0xFF, 0x88, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx,   1,   1, true,  0xFF, 0x89, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 119,   0, true,  0xFF, 0xFF, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 120,   1, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 121,   1, true,  0xFF, 0x01, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 143,   1, true,  0xFF, 0x17, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 144,   1, true,  0xFF, 0x18, 0x88, 144,  0, 0, 0),
    atest!(DOUBLE_R, volsw_sx, 145, -22, true,  0xFF, 0x00, 0x88, 144,  0, 0, 0),
];

unsafe fn control_type_str(type_: snd_ctl_elem_type_t) -> *const c_char {
    match type_ {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN => cstr(b"bool\0"),
        SNDRV_CTL_ELEM_TYPE_INTEGER => cstr(b"int\0"),
        _ => cstr(b"unknown\0"),
    }
}

unsafe fn control_layout_str(layout: soc_ops_test_control_layout) -> *const c_char {
    match layout {
        soc_ops_test_control_layout::SOC_OPS_TEST_SINGLE => cstr(b"single\0"),
        soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE => cstr(b"double\0"),
        soc_ops_test_control_layout::SOC_OPS_TEST_DOUBLE_R => cstr(b"double_r\0"),
    }
}

#[repr(C)]
struct regmap_bus {
    read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t, *mut c_void, size_t) -> c_int>,
    write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t) -> c_int>,
    gather_write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t, *const c_void, size_t) -> c_int>,
    reg_format_endian_default: c_int,
    val_format_endian_default: c_int,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_format_endian: c_int,
    val_format_endian: c_int,
    max_register: c_uint,
    cache_type: c_int,
}

unsafe extern "C" fn mock_regmap_read(
    context: *mut c_void,
    _reg_buf: *const c_void,
    _reg_size: size_t,
    _val_buf: *mut c_void,
    _val_size: size_t,
) -> c_int {
    let priv_ = context as *mut soc_ops_test_priv;
    KUNIT_FAIL((*priv_).test, cstr(b"Unexpected bus read\0"));
    -EIO
}

unsafe extern "C" fn mock_regmap_gather_write(
    context: *mut c_void,
    _reg_buf: *const c_void,
    _reg_size: size_t,
    _val_buf: *const c_void,
    _val_size: size_t,
) -> c_int {
    let priv_ = context as *mut soc_ops_test_priv;
    KUNIT_FAIL((*priv_).test, cstr(b"Unexpected bus gather_write\0"));
    -EIO
}

unsafe extern "C" fn mock_regmap_write(
    context: *mut c_void,
    _val_buf: *const c_void,
    _val_size: size_t,
) -> c_int {
    let priv_ = context as *mut soc_ops_test_priv;
    KUNIT_FAIL((*priv_).test, cstr(b"Unexpected bus write\0"));
    -EIO
}

static mock_regmap_bus: regmap_bus = regmap_bus {
    read: Some(mock_regmap_read),
    write: Some(mock_regmap_write),
    gather_write: Some(mock_regmap_gather_write),
    reg_format_endian_default: REGMAP_ENDIAN_NATIVE,
    val_format_endian_default: REGMAP_ENDIAN_NATIVE,
};

static mock_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_format_endian: REGMAP_ENDIAN_NATIVE,
    val_format_endian: REGMAP_ENDIAN_NATIVE,
    max_register: 0x1,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn soc_ops_test_init(test: *mut kunit) -> c_int {
    let priv_ = kunit_kzalloc(test, size_of::<soc_ops_test_priv>(), GFP_KERNEL) as *mut soc_ops_test_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).test = test;

    let dev = kunit_device_register(test, cstr(b"soc_ops_test_drv\0"));
    if IS_ERR(dev as *const c_void) {
        return PTR_ERR(dev as *const c_void);
    }

    let regmap = devm_regmap_init(dev, &mock_regmap_bus, priv_ as *mut c_void, &mock_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    /* No actual hardware, we just use the cache */
    regcache_cache_only(regmap, true);

    (*priv_).component.dev = dev;
    (*priv_).component.regmap = regmap;
    mutex_init(&mut (*priv_).component.io_mutex);

    (*test).priv_ = priv_ as *mut c_void;

    0
}

unsafe extern "C" fn soc_ops_test_exit(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut soc_ops_test_priv;
    kunit_device_unregister(test, (*priv_).component.dev);
}

unsafe extern "C" fn info_test_desc(param: *const info_test_param, desc: *mut c_char) {
    snprintf(
        desc,
        KUNIT_PARAM_DESC_SIZE,
        cstr(b"%s %s %s: ctl range: %ld->%ld, reg range: %d->%d(%d), sign: %d, inv: %d\0"),
        control_layout_str((*param).layout),
        (*param).func_name,
        control_type_str((*param).uinfo.type_),
        (*param).uinfo.value.integer.min,
        (*param).uinfo.value.integer.max,
        (*param).mc.min,
        (*param).mc.max,
        (*param).mc.platform_max,
        (*param).mc.sign_bit,
        (*param).mc.invert,
    );
}

unsafe extern "C" fn soc_ops_test_info(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut soc_ops_test_priv;
    let param = (*test).param_value as *const info_test_param;
    let target = &(*param).uinfo as *const snd_ctl_elem_info;
    let mut result: snd_ctl_elem_info = core::mem::zeroed();
    let mut kctl: snd_kcontrol = core::mem::zeroed();
    kctl.private_data = &mut (*priv_).component as *mut _ as *mut c_void;
    kctl.private_value = &(*param).mc as *const _ as c_ulong;

    strscpy(kctl.id.name.as_mut_ptr(), (*param).name, kctl.id.name.len());

    let ret = ((*param).info)(&mut kctl, &mut result);
    KUNIT_ASSERT_FALSE(test, ret);

    KUNIT_EXPECT_EQ(test, result.count as c_long, (*target).count as c_long);
    KUNIT_EXPECT_EQ(test, result.type_ as c_long, (*target).type_ as c_long);
    KUNIT_EXPECT_EQ(test, result.value.integer.min, (*target).value.integer.min);
    KUNIT_EXPECT_EQ(test, result.value.integer.max, (*target).value.integer.max);
}

unsafe extern "C" fn access_test_desc(param: *const access_test_param, desc: *mut c_char) {
    if (*param).ret < 0 {
        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            cstr(b"%s %s: %ld,%ld -> range: %d->%d(%d), sign: %d, inv: %d -> err: %d\0"),
            control_layout_str((*param).layout),
            (*param).func_name,
            (*param).lctl,
            (*param).rctl,
            (*param).mc.min,
            (*param).mc.max,
            (*param).mc.platform_max,
            (*param).mc.sign_bit,
            (*param).mc.invert,
            (*param).ret,
        );
    } else {
        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            cstr(b"%s %s: %ld,%ld -> range: %d->%d(%d), sign: %d, inv: %d -> %#x,%#x\0"),
            control_layout_str((*param).layout),
            (*param).func_name,
            (*param).lctl,
            (*param).rctl,
            (*param).mc.min,
            (*param).mc.max,
            (*param).mc.platform_max,
            (*param).mc.sign_bit,
            (*param).mc.invert,
            (*param).lreg,
            (*param).rreg,
        );
    }
}

unsafe extern "C" fn soc_ops_test_access(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut soc_ops_test_priv;
    let param = (*test).param_value as *const access_test_param;
    let mut kctl: snd_kcontrol = core::mem::zeroed();
    let mut val: c_uint = 0;
    let mut ret: c_int;
    /* it is too large struct. use kzalloc() */
    let result = kunit_kzalloc(test, size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if result.is_null() {
        return;
    }

    kctl.private_data = &mut (*priv_).component as *mut _ as *mut c_void;
    kctl.private_value = &(*param).mc as *const _ as c_ulong;

    ret = regmap_write((*priv_).component.regmap, 0x0, (*param).init);
    KUNIT_ASSERT_FALSE(test, ret);
    ret = regmap_write((*priv_).component.regmap, 0x1, (*param).init);
    KUNIT_ASSERT_FALSE(test, ret);

    (*result).value.integer.value[0] = (*param).lctl;
    (*result).value.integer.value[1] = (*param).rctl;

    ret = ((*param).put)(&mut kctl, result);
    KUNIT_ASSERT_EQ(test, ret, (*param).ret);
    if ret >= 0 {
        ret = regmap_read((*priv_).component.regmap, 0x0, &mut val);
        KUNIT_ASSERT_FALSE(test, ret);
        KUNIT_EXPECT_EQ(test, val as c_long, (((*param).init & !(*param).lmask) | (*param).lreg) as c_long);

        ret = regmap_read((*priv_).component.regmap, 0x1, &mut val);
        KUNIT_ASSERT_FALSE(test, ret);
        KUNIT_EXPECT_EQ(test, val as c_long, (((*param).init & !(*param).rmask) | (*param).rreg) as c_long);

        (*result).value.integer.value[0] = 0;
        (*result).value.integer.value[1] = 0;

        ret = ((*param).get)(&mut kctl, result);
        KUNIT_ASSERT_GE(test, ret, 0);

        KUNIT_EXPECT_EQ(test, (*result).value.integer.value[0], (*param).lctl);
        if (*param).layout != soc_ops_test_control_layout::SOC_OPS_TEST_SINGLE {
            KUNIT_EXPECT_EQ(test, (*result).value.integer.value[1], (*param).rctl);
        } else {
            KUNIT_EXPECT_EQ(test, (*result).value.integer.value[1], 0);
        }
    }
    kfree(result as *mut c_void);
}

// KUNIT_ARRAY_PARAM(all_info_tests, all_info_test_params, info_test_desc);
// KUNIT_ARRAY_PARAM(all_access_tests, all_access_test_params, access_test_desc);

#[repr(C)]
struct kunit_case {
    run_case: Option<unsafe extern "C" fn(*mut kunit)>,
    generate_params: *const c_void,
}

#[repr(C)]
struct kunit_suite {
    name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut kunit)>,
    test_cases: *mut kunit_case,
}

extern "C" {
    static all_info_tests_gen_params: c_void;
    static all_access_tests_gen_params: c_void;
    fn kunit_test_suites(suite: *mut kunit_suite);
}

static mut soc_ops_test_cases: [kunit_case; 3] = [
    kunit_case { run_case: Some(soc_ops_test_info), generate_params: unsafe { &all_info_tests_gen_params as *const _ as *const c_void } },
    kunit_case { run_case: Some(soc_ops_test_access), generate_params: unsafe { &all_access_tests_gen_params as *const _ as *const c_void } },
    kunit_case { run_case: None, generate_params: ptr::null() },
];

static mut soc_ops_test_suite: kunit_suite = kunit_suite {
    name: cstr(b"soc-ops\0"),
    init: Some(soc_ops_test_init),
    exit: Some(soc_ops_test_exit),
    test_cases: unsafe { soc_ops_test_cases.as_mut_ptr() },
};

unsafe fn register_soc_ops_test_suite() {
    kunit_test_suites(&mut soc_ops_test_suite);
}

// MODULE_DESCRIPTION("ASoC soc-ops kunit test");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
