// SPDX-License-Identifier: GPL-2.0-only
//
// Mock regmap for cs_dsp KUnit tests.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

use core::ffi::{c_int, c_uint, c_void};

// Kernel and cs_dsp types/functions are supplied by the surrounding tree.
#[repr(C)] pub struct cs_dsp_test { pub test: *mut c_void, pub dsp: *mut cs_dsp, pub saw_bus_write: bool }
#[repr(C)] pub struct cs_dsp { pub regmap: *mut regmap, pub dev: *mut c_void, pub type_: c_uint, pub rev: c_uint, pub base: c_uint }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct regmap_bus;
#[repr(C)] pub struct regmap_config;
#[repr(C)] pub struct regmap_access_table;
#[repr(C)] pub struct regmap_range;
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }

extern "C" {
    fn regcache_drop_region(map: *mut regmap, first: c_uint, last: c_uint);
    fn regmap_get_reg_stride(map: *mut regmap) -> c_int;
    fn regmap_get_val_bytes(map: *mut regmap) -> usize;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_regmap_init(dev: *mut c_void, bus: *const regmap_bus, context: *mut c_void, config: *const regmap_config) -> *mut regmap;
}

const WMFW_ADSP2: c_uint = 2;
const WMFW_HALO: c_uint = 3;

unsafe fn cs_dsp_mock_regmap_read(context: *mut c_void, _reg_buf: *const c_void, _reg_size: usize,
                                  _val_buf: *mut c_void, _val_size: usize) -> c_int {
    let _priv = context as *mut cs_dsp_test;
    // Should never get here because the regmap is cache-only
    -5 // -EIO
}

unsafe fn cs_dsp_mock_regmap_gather_write(context: *mut c_void, _reg_buf: *const c_void, _reg_size: usize,
                                          _val_buf: *const c_void, _val_size: usize) -> c_int {
    let priv_ = &mut *(context as *mut cs_dsp_test);
    priv_.saw_bus_write = true;
    // Should never get here because the regmap is cache-only
    -5 // -EIO
}

unsafe fn cs_dsp_mock_regmap_write(context: *mut c_void, _val_buf: *const c_void, _val_size: usize) -> c_int {
    let priv_ = &mut *(context as *mut cs_dsp_test);
    priv_.saw_bus_write = true;
    // Should never get here because the regmap is cache-only
    -5 // -EIO
}

static ADSP2_32BIT_REGISTER_DEFAULTS: [reg_default; 8] = [
    reg_default { reg: 0xffe00, def: 0 }, reg_default { reg: 0xffe02, def: 0 },
    reg_default { reg: 0xffe04, def: 1 }, reg_default { reg: 0xffe30, def: 0 },
    reg_default { reg: 0xffe32, def: 0 }, reg_default { reg: 0xffe34, def: 0 },
    reg_default { reg: 0xffe40, def: 0 }, reg_default { reg: 0xffe42, def: 0 },
];
static ADSP2_16BIT_REGISTER_DEFAULTS: [reg_default; 10] = [
    reg_default { reg: 0x1100, def: 0 }, reg_default { reg: 0x1101, def: 0 },
    reg_default { reg: 0x1104, def: 1 }, reg_default { reg: 0x1130, def: 0 },
    reg_default { reg: 0x1131, def: 0 }, reg_default { reg: 0x1134, def: 0 },
    reg_default { reg: 0x1140, def: 0 }, reg_default { reg: 0x1141, def: 0 },
    reg_default { reg: 0x1142, def: 0 }, reg_default { reg: 0x1143, def: 0 },
];
static HALO_REGISTER_DEFAULTS: [reg_default; 9] = [
    reg_default { reg: 0x2b80010, def: 0 }, reg_default { reg: 0x2b805c0, def: 0 },
    reg_default { reg: 0x2b805c8, def: 0 }, reg_default { reg: 0x2b805d0, def: 0 },
    reg_default { reg: 0x2b805d8, def: 0 }, reg_default { reg: 0x2bc1000, def: 0 },
    reg_default { reg: 0x2bc7000, def: 0 }, reg_default { reg: 0x25e2040, def: 0 },
    reg_default { reg: 0x25e2044, def: 0 },
];

#[no_mangle] pub static cs_dsp_mock_adsp2_32bit_sysbase: c_uint = 0xffe00;
#[no_mangle] pub static cs_dsp_mock_adsp2_16bit_sysbase: c_uint = 0x1100;
#[no_mangle] pub static cs_dsp_mock_halo_core_base: c_uint = 0x2b80000;
#[no_mangle] pub static cs_dsp_mock_halo_sysinfo_base: c_uint = 0x25e0000;

pub unsafe fn cs_dsp_mock_regmap_drop_range(priv_: *mut cs_dsp_test, first_reg: c_uint, last_reg: c_uint) {
    regcache_drop_region((*(*priv_).dsp).regmap, first_reg, last_reg);
}

pub unsafe fn cs_dsp_mock_regmap_drop_regs(priv_: *mut cs_dsp_test, first_reg: c_uint, num_regs: usize) {
    let stride = regmap_get_reg_stride((*(*priv_).dsp).regmap) as usize;
    let last = first_reg.wrapping_add((stride * num_regs.wrapping_sub(1)) as c_uint);
    cs_dsp_mock_regmap_drop_range(priv_, first_reg, last);
}

pub unsafe fn cs_dsp_mock_regmap_drop_bytes(priv_: *mut cs_dsp_test, first_reg: c_uint, num_bytes: usize) {
    let num_regs = num_bytes / regmap_get_val_bytes((*(*priv_).dsp).regmap);
    cs_dsp_mock_regmap_drop_regs(priv_, first_reg, num_regs);
}

pub unsafe fn cs_dsp_mock_regmap_drop_system_regs(priv_: *mut cs_dsp_test) {
    let dsp = &*(*priv_).dsp;
    match dsp.type_ {
        WMFW_ADSP2 if dsp.base != 0 => regcache_drop_region(dsp.regmap, dsp.base, dsp.base + 0x7c),
        WMFW_HALO if dsp.base != 0 => regcache_drop_region(dsp.regmap, dsp.base, dsp.base + 0x47000),
        _ => (),
    }
}

pub unsafe fn cs_dsp_mock_regmap_is_dirty(priv_: *mut cs_dsp_test, drop_system_regs: bool) -> bool {
    if drop_system_regs { cs_dsp_mock_regmap_drop_system_regs(priv_); }
    (*priv_).saw_bus_write = false;
    let map = (*(*priv_).dsp).regmap;
    regcache_cache_only(map, false);
    regcache_sync(map);
    regcache_cache_only(map, true);
    (*priv_).saw_bus_write
}

pub unsafe fn cs_dsp_mock_regmap_init(priv_: *mut cs_dsp_test) -> c_int {
    let dsp = &mut *(*priv_).dsp;
    dsp.regmap = devm_regmap_init(dsp.dev, core::ptr::null(), priv_ as *mut c_void, core::ptr::null());
    if dsp.regmap.is_null() { return -12; }
    regcache_cache_only(dsp.regmap, true);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
