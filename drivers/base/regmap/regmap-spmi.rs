// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - SPMI support
//
// Copyright (c) 2012-2013, The Linux Foundation. All rights reserved.
//
// Based on regmap-i2c.c:
// Copyright 2011 Wolfson Microelectronics plc
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

use core::ffi::c_void;

extern "C" {
    fn spmi_register_read(context: *mut c_void, addr: u8, val: *mut u8) -> i32;
    fn spmi_register_zero_write(context: *mut c_void, val: u8) -> i32;
    fn spmi_register_write(context: *mut c_void, addr: u8, val: u8) -> i32;
    fn spmi_ext_register_read(context: *mut c_void, addr: u16, val: *mut u8, len: usize) -> i32;
    fn spmi_ext_register_readl(context: *mut c_void, addr: u16, val: *mut u8, len: usize) -> i32;
    fn spmi_ext_register_write(context: *mut c_void, addr: u16, val: *const u8, len: usize) -> i32;
    fn spmi_ext_register_writel(context: *mut c_void, addr: u16, val: *const u8, len: usize) -> i32;
    fn __regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut c_void,
                     config: *const regmap_config, lock_key: *mut lock_class_key,
                     lock_name: *const i8) -> *mut regmap;
    fn __devm_regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut c_void,
                          config: *const regmap_config, lock_key: *mut lock_class_key,
                          lock_name: *const i8) -> *mut regmap;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct spmi_device { pub dev: device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }

pub const REGMAP_ENDIAN_NATIVE: i32 = 0;

#[repr(C)]
pub struct regmap_bus {
    pub read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *mut c_void, usize) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> i32>,
    pub gather_write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *const c_void, usize) -> i32>,
    pub reg_format_endian_default: i32,
    pub val_format_endian_default: i32,
}

unsafe extern "C" fn regmap_spmi_base_read(context: *mut c_void, reg: *const c_void,
                                             reg_size: usize, val: *mut c_void,
                                             mut val_size: usize) -> i32 {
    debug_assert!(reg_size == 1);
    let mut addr = *(reg as *const u8);
    let mut err = 0;
    while val_size != 0 && err == 0 {
        err = spmi_register_read(context, addr, val as *mut u8);
        addr = addr.wrapping_add(1);
        val = val.add(1);
        val_size -= 1;
    }
    err
}

unsafe extern "C" fn regmap_spmi_base_gather_write(context: *mut c_void, reg: *const c_void,
                                                     reg_size: usize, val: *const c_void,
                                                     mut val_size: usize) -> i32 {
    let mut data = val as *const u8;
    let mut addr = *(reg as *const u8);
    debug_assert!(reg_size == 1);
    let mut err = 0;
    if addr == 0 && val_size != 0 {
        err = spmi_register_zero_write(context, *data);
        if err != 0 { return err; }
        data = data.add(1); addr = addr.wrapping_add(1); val_size -= 1;
    }
    while val_size != 0 {
        err = spmi_register_write(context, addr, *data);
        if err != 0 { break; }
        data = data.add(1); addr = addr.wrapping_add(1); val_size -= 1;
    }
    err
}

unsafe extern "C" fn regmap_spmi_base_write(context: *mut c_void, data: *const c_void,
                                              count: usize) -> i32 {
    debug_assert!(count >= 1);
    regmap_spmi_base_gather_write(context, data, 1, (data as *const u8).add(1) as *const c_void, count - 1)
}

static REGMAP_SPMI_BASE: regmap_bus = regmap_bus { read: Some(regmap_spmi_base_read), write: Some(regmap_spmi_base_write), gather_write: Some(regmap_spmi_base_gather_write), reg_format_endian_default: REGMAP_ENDIAN_NATIVE, val_format_endian_default: REGMAP_ENDIAN_NATIVE };

unsafe extern "C" fn regmap_spmi_ext_read(context: *mut c_void, reg: *const c_void, reg_size: usize, val: *mut c_void, mut val_size: usize) -> i32 {
    debug_assert!(reg_size == 2); let mut addr = *(reg as *const u16); let mut val = val as *mut u8; let mut err = 0;
    while addr <= 0xff && val_size != 0 { let len = core::cmp::min(val_size, 16); err = spmi_ext_register_read(context, addr, val, len); if err != 0 { return err; } addr = addr.wrapping_add(len as u16); val = val.add(len); val_size -= len; }
    while val_size != 0 { let len = core::cmp::min(val_size, 8); err = spmi_ext_register_readl(context, addr, val, len); if err != 0 { return err; } addr = addr.wrapping_add(len as u16); val = val.add(len); val_size -= len; } err
}

unsafe extern "C" fn regmap_spmi_ext_gather_write(context: *mut c_void, reg: *const c_void, reg_size: usize, val: *const c_void, mut val_size: usize) -> i32 {
    debug_assert!(reg_size == 2); let mut addr = *(reg as *const u16); let mut val = val as *const u8;
    while addr <= 0xff && val_size != 0 { let len = core::cmp::min(val_size, 16); let err = spmi_ext_register_write(context, addr, val, len); if err != 0 { return err; } addr = addr.wrapping_add(len as u16); val = val.add(len); val_size -= len; }
    while val_size != 0 { let len = core::cmp::min(val_size, 8); let err = spmi_ext_register_writel(context, addr, val, len); if err != 0 { return err; } addr = addr.wrapping_add(len as u16); val = val.add(len); val_size -= len; } 0
}

unsafe extern "C" fn regmap_spmi_ext_write(context: *mut c_void, data: *const c_void, count: usize) -> i32 { debug_assert!(count >= 2); regmap_spmi_ext_gather_write(context, data, 2, (data as *const u8).add(2) as *const c_void, count - 2) }

static REGMAP_SPMI_EXT: regmap_bus = regmap_bus { read: Some(regmap_spmi_ext_read), write: Some(regmap_spmi_ext_write), gather_write: Some(regmap_spmi_ext_gather_write), reg_format_endian_default: REGMAP_ENDIAN_NATIVE, val_format_endian_default: REGMAP_ENDIAN_NATIVE };

pub unsafe extern "C" fn __regmap_init_spmi_base(sdev: *mut spmi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap { __regmap_init(&mut (*sdev).dev, &REGMAP_SPMI_BASE, sdev as *mut c_void, config, lock_key, lock_name) }
pub unsafe extern "C" fn __devm_regmap_init_spmi_base(sdev: *mut spmi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap { __devm_regmap_init(&mut (*sdev).dev, &REGMAP_SPMI_BASE, sdev as *mut c_void, config, lock_key, lock_name) }
pub unsafe extern "C" fn __regmap_init_spmi_ext(sdev: *mut spmi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap { __regmap_init(&mut (*sdev).dev, &REGMAP_SPMI_EXT, sdev as *mut c_void, config, lock_key, lock_name) }
pub unsafe extern "C" fn __devm_regmap_init_spmi_ext(sdev: *mut spmi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap { __devm_regmap_init(&mut (*sdev).dev, &REGMAP_SPMI_EXT, sdev as *mut c_void, config, lock_key, lock_name) }

// MODULE_DESCRIPTION("Register map access API - SPMI support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
