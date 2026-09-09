// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the ICST307 VCO clock found in the ARM Reference designs.
 * We wrap the custom interface from <asm/hardware/icst.h> into the generic
 * clock framework.
 *
 * Copyright (C) 2012-2015 Linus Walleij
 *
 * TODO: when all ARM reference designs are migrated to generic clocks, the
 * ICST clock code from the ARM tree should probably be merged into this
 * file.
 */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const VERSATILE_LOCK_VAL: u32 = 0xA05F;
const VERSATILE_AUX_OSC_BITS: u32 = 0x7FFFF;
const INTEGRATOR_AP_CM_BITS: u32 = 0xFF;
const INTEGRATOR_AP_SYS_BITS: u32 = 0xFF;
const INTEGRATOR_CP_CM_CORE_BITS: u32 = 0x7FF;
const INTEGRATOR_CP_CM_MEM_BITS: u32 = 0x7FF000;
const INTEGRATOR_AP_PCI_25_33_MHZ: u32 = 1 << 8;

#[repr(C)]
pub struct clk_icst {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub vcoreg_off: u32,
    pub lockreg_off: u32,
    pub params: *mut icst_params,
    pub rate: c_ulong,
    pub ctype: icst_control_type,
}

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct clk_init_data {
    pub name: *const c_char, pub ops: *const clk_ops, pub flags: u32,
    pub parent_names: *const *const c_char, pub num_parents: u8,
}
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub val_bits: u32, pub reg_stride: u32 }
#[repr(C)] pub struct icst_vco { pub v: u32, pub r: u32, pub s: u32 }
#[repr(C)] pub struct icst_params {
    pub vco_max: u32, pub vco_min: u32, pub vd_min: u32, pub vd_max: u32,
    pub rd_min: u32, pub rd_max: u32, pub s2div: Option<unsafe extern "C" fn(u32) -> u32>,
    pub idx2s: Option<unsafe extern "C" fn(u32) -> u32>, pub ref_: c_ulong,
}
#[repr(C)] pub struct clk_icst_desc { pub vco_offset: u32, pub lock_offset: u32, pub params: *const icst_params }
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}
#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq)] pub enum icst_control_type {
    ICST_VERSATILE, ICST_INTEGRATOR_AP_CM, ICST_INTEGRATOR_AP_SYS,
    ICST_INTEGRATOR_AP_PCI, ICST_INTEGRATOR_CP_CM_CORE, ICST_INTEGRATOR_CP_CM_MEM,
}

extern "C" {
    fn regmap_read(*mut regmap, u32, *mut u32) -> c_int;
    fn regmap_write(*mut regmap, u32, u32) -> c_int;
    fn regmap_update_bits(*mut regmap, u32, u32, u32) -> c_int;
    fn icst_hz(*const icst_params, icst_vco) -> c_ulong;
    fn icst_hz_to_vco(*const icst_params, c_ulong) -> icst_vco;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn vco_get(icst: *mut clk_icst, vco: *mut icst_vco) -> c_int {
    let mut val = 0u32; let ret = regmap_read((*icst).map, (*icst).vcoreg_off, &mut val); if ret != 0 { return ret; }
    match (*icst).ctype {
        icst_control_type::ICST_INTEGRATOR_AP_CM => { (*vco).v = val & INTEGRATOR_AP_CM_BITS; (*vco).r = 22; (*vco).s = 1; }
        icst_control_type::ICST_INTEGRATOR_AP_SYS => { (*vco).v = val & INTEGRATOR_AP_SYS_BITS; (*vco).r = 46; (*vco).s = 3; }
        icst_control_type::ICST_INTEGRATOR_AP_PCI => { let d = (val & INTEGRATOR_AP_PCI_25_33_MHZ) != 0; (*vco).v = if d {17} else {14}; (*vco).r = if d {22} else {14}; (*vco).s = 1; }
        icst_control_type::ICST_INTEGRATOR_CP_CM_CORE => { (*vco).v = val & 0xff; (*vco).r = 22; (*vco).s = (val >> 8) & 7; }
        icst_control_type::ICST_INTEGRATOR_CP_CM_MEM => { (*vco).v = (val >> 12) & 0xff; (*vco).r = 22; (*vco).s = (val >> 20) & 7; }
        _ => { (*vco).v = val & 0x1ff; (*vco).r = (val >> 9) & 0x7f; (*vco).s = (val >> 16) & 3; }
    } 0
}

unsafe fn vco_set(icst: *mut clk_icst, vco: icst_vco) -> c_int {
    let (mask, val) = match (*icst).ctype {
        icst_control_type::ICST_INTEGRATOR_AP_CM => (INTEGRATOR_AP_CM_BITS, vco.v & 0xff),
        icst_control_type::ICST_INTEGRATOR_AP_SYS => (INTEGRATOR_AP_SYS_BITS, vco.v & 0xff),
        icst_control_type::ICST_INTEGRATOR_CP_CM_CORE => (INTEGRATOR_CP_CM_CORE_BITS, (vco.v & 0xff) | (vco.s << 8)),
        icst_control_type::ICST_INTEGRATOR_CP_CM_MEM => (INTEGRATOR_CP_CM_MEM_BITS, ((vco.v & 0xff) << 12) | (vco.s << 20)),
        _ => (VERSATILE_AUX_OSC_BITS, vco.v | (vco.r << 9) | (vco.s << 16)),
    };
    let mut ret = regmap_write((*icst).map, (*icst).lockreg_off, VERSATILE_LOCK_VAL); if ret != 0 { return ret; }
    ret = regmap_update_bits((*icst).map, (*icst).vcoreg_off, mask, val); if ret != 0 { return ret; }
    regmap_write((*icst).map, (*icst).lockreg_off, 0)
}

unsafe extern "C" {
    pub fn icst_clk_setup(dev: *mut device, desc: *const clk_icst_desc, name: *const c_char, parent_name: *const c_char, map: *mut regmap, ctype: icst_control_type) -> *mut clk;
    pub fn icst_clk_register(dev: *mut device, desc: *const clk_icst_desc, name: *const c_char, parent_name: *const c_char, base: *mut c_void) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
