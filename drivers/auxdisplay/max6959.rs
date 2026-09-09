// SPDX-License-Identifier: GPL-2.0
/*
 * MAX6958/6959 7-segment LED display controller
 * Datasheet:
 * https://www.analog.com/media/en/technical-documentation/data-sheets/MAX6958-MAX6959.pdf
 *
 * Copyright (c) 2024, Intel Corporation.
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const REG_DECODE_MODE: u32 = 0x01;
const REG_INTENSITY: u32 = 0x02;
const REG_SCAN_LIMIT: u32 = 0x03;
const REG_CONFIGURATION: u32 = 0x04;
const REG_CONFIGURATION_S_BIT: u32 = 1 << 0;

#[inline]
const fn reg_digit(x: u32) -> u32 { 0x20 + x }

const REG_DIGIT0: u32 = 0x20;
const REG_DIGIT1: u32 = 0x21;
const REG_DIGIT2: u32 = 0x22;
const REG_DIGIT3: u32 = 0x23;
const REG_SEGMENTS: u32 = 0x24;
const REG_MAX: u32 = REG_SEGMENTS;

#[repr(C)]
pub struct max6959_priv {
    pub linedisp: linedisp,
    pub work: delayed_work,
    pub regmap: *mut regmap,
}

extern "C" {
    fn container_of<T, U>(ptr: *mut T, member: *const U) -> *mut U;
    fn bitrev8(value: u8) -> u8;
    fn map_to_seg7(map: *const map_seg7, value: i8) -> u8;
    fn regmap_bulk_write(map: *mut regmap, reg: u32, val: *const u8, count: usize) -> i32;
    fn schedule_delayed_work(work: *mut delayed_work, delay: u64) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn regmap_assign_bits(map: *mut regmap, reg: u32, mask: u32, val: bool) -> i32;
    fn regmap_get_device(map: *mut regmap) -> *mut device;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn linedisp_register(ld: *mut linedisp, dev: *mut device, width: u32, ops: *const linedisp_ops) -> i32;
    fn linedisp_unregister(ld: *mut linedisp);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut core::ffi::c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut max6959_priv;
    fn dev_get_drvdata(dev: *mut device) -> *mut max6959_priv;
}

#[repr(C)] pub struct linedisp { pub map: *mut linedisp_map, pub buf: *mut i8 }
#[repr(C)] pub struct linedisp_map { pub map: map_union }
#[repr(C)] pub struct map_union { pub seg7: map_seg7 }
#[repr(C)] pub struct map_seg7 { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct regmap_config { pub reg_bits: u8, pub val_bits: u8, pub max_register: u32, pub cache_type: u32 }
#[repr(C)] pub struct linedisp_ops { pub get_map_type: Option<unsafe extern "C" fn(*mut linedisp) -> i32>, pub update: Option<unsafe extern "C" fn(*mut linedisp)> }

const LINEDISP_MAP_SEG7: i32 = 1;
const REGCACHE_MAPLE: u32 = 0;

unsafe extern "C" fn max6959_disp_update(work: *mut work_struct) {
    let priv_ = container_of(work, core::ptr::addr_of!((*core::ptr::null_mut::<max6959_priv>()).work.work), core::ptr::null_mut());
    let linedisp = &mut (*priv_).linedisp;
    let map = (*linedisp).map;
    let mut s = (*linedisp).buf;
    let mut buf = [0u8; 4];

    // Map segments according to datasheet
    for digit in buf.iter_mut() {
        *digit = bitrev8(map_to_seg7(&(*map).map.seg7, *s as i8)) >> 1;
        s = s.add(1);
    }
    regmap_bulk_write((*priv_).regmap, reg_digit(0), buf.as_ptr(), buf.len());
}

unsafe extern "C" fn max6959_linedisp_get_map_type(linedisp: *mut linedisp) -> i32 {
    let priv_ = container_of(linedisp, core::ptr::null(), core::ptr::null());
    INIT_DELAYED_WORK(&mut (*priv_).work, max6959_disp_update);
    LINEDISP_MAP_SEG7
}

unsafe extern "C" fn max6959_linedisp_update(linedisp: *mut linedisp) {
    let priv_ = container_of(linedisp, core::ptr::null(), core::ptr::null());
    schedule_delayed_work(&mut (*priv_).work, 0);
}

static max6959_linedisp_ops: linedisp_ops = linedisp_ops { get_map_type: Some(max6959_linedisp_get_map_type), update: Some(max6959_linedisp_update) };

unsafe fn max6959_enable(priv_: *mut max6959_priv, enable: bool) -> i32 { regmap_assign_bits((*priv_).regmap, REG_CONFIGURATION, REG_CONFIGURATION_S_BIT, enable) }
unsafe extern "C" fn max6959_power_off(priv_: *mut core::ffi::c_void) { max6959_enable(priv_ as *mut max6959_priv, false); }
unsafe fn max6959_power_on(priv_: *mut max6959_priv) -> i32 {
    let dev = regmap_get_device((*priv_).regmap);
    let ret = max6959_enable(priv_, true);
    if ret != 0 { return ret; }
    devm_add_action_or_reset(dev, max6959_power_off, priv_ as *mut core::ffi::c_void)
}

static max6959_regmap_config: regmap_config = regmap_config { reg_bits: 8, val_bits: 8, max_register: REG_MAX, cache_type: REGCACHE_MAPLE };

unsafe extern "C" fn max6959_i2c_probe(client: *mut i2c_client) -> i32 {
    let dev = &mut (*client).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<max6959_priv>(), 0) as *mut max6959_priv;
    if priv_.is_null() { return -12; }
    (*priv_).regmap = devm_regmap_init_i2c(client, &max6959_regmap_config);
    let ret = max6959_power_on(priv_);
    if ret != 0 { return ret; }
    let ret = linedisp_register(&mut (*priv_).linedisp, dev, 4, &max6959_linedisp_ops);
    if ret != 0 { return ret; }
    i2c_set_clientdata(client, priv_ as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn max6959_i2c_remove(client: *mut i2c_client) {
    let priv_ = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&mut (*priv_).work);
    linedisp_unregister(&mut (*priv_).linedisp);
}

unsafe extern "C" fn max6959_suspend(dev: *mut device) -> i32 { max6959_enable(dev_get_drvdata(dev), false) }
unsafe extern "C" fn max6959_resume(dev: *mut device) -> i32 { max6959_enable(dev_get_drvdata(dev), true) }

// DEFINE_SIMPLE_DEV_PM_OPS(max6959_pm_ops, max6959_suspend, max6959_resume);
// MODULE_DEVICE_TABLE(i2c, max6959_i2c_id);
// MODULE_DEVICE_TABLE(of, max6959_of_table);
// module_i2c_driver(max6959_i2c_driver);
// MODULE_DESCRIPTION("MAX6958/6959 7-segment LED controller");
// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("LINEDISP");

extern "C" { fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
