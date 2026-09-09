// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Verifone Int.
 *
 * Author: Nicolas Saenz Julienne <nicolassaenzj@gmail.com>
 *
 * This driver is based on the gpio-tps65912 implementation.
 */

// Dependencies supplied by the surrounding kernel bindings are intentionally
// left as external symbols.

#[repr(C)]
pub struct Tps65218Gpio {
    pub tps65218: *mut Tps65218,
    pub gpio_chip: GpioChip,
}

#[repr(C)]
pub struct Tps65218 { pub regmap: *mut Regmap }
#[repr(C)]
pub struct Regmap;
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct PlatformDevice { pub dev: Device }
#[repr(C)]
pub struct GpioChip { pub parent: *mut Device, pub data: *mut core::ffi::c_void }
#[repr(C)]
pub struct OfDeviceId { pub compatible: *const core::ffi::c_char }
#[repr(C)]
pub struct PlatformDeviceId { pub name: *const core::ffi::c_char }
#[repr(C)]
pub struct PlatformDriver;

extern "C" {
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut core::ffi::c_void;
    fn gpiochip_line_is_open_source(gc: *mut GpioChip, offset: u32) -> bool;
    fn gpiochip_line_is_open_drain(gc: *mut GpioChip, offset: u32) -> bool;
    fn pinconf_to_config_param(config: c_ulong) -> PinConfigParam;
    fn regmap_read(map: *mut Regmap, reg: u32, val: *mut u32) -> i32;
    fn tps65218_set_bits(tps: *mut Tps65218, reg: u32, mask: u32, val: u32, protection: u32) -> i32;
    fn tps65218_clear_bits(tps: *mut Tps65218, reg: u32, mask: u32, protection: u32) -> i32;
    fn dev_err(dev: *mut Device, message: *const core::ffi::c_char);
    fn dev_get_drvdata(dev: *mut Device) -> *mut Tps65218;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut core::ffi::c_void) -> i32;
}

use core::ffi::{c_ulong, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub enum PinConfigParam { DriveOpenDrain, DrivePushPull }

const TPS65218_REG_ENABLE2: u32 = 0;
const TPS65218_ENABLE2_GPIO1: u32 = 1;
const TPS65218_PROTECT_L1: u32 = 0;
const TPS65218_REG_SEQ7: u32 = 0;
const TPS65218_SEQ7_GPO1_SEQ_MASK: u32 = 0;
const TPS65218_SEQ7_GPO3_SEQ_MASK: u32 = 0;
const TPS65218_REG_CONFIG1: u32 = 0;
const TPS65218_CONFIG1_IO1_SEL: u32 = 0;
const TPS65218_CONFIG1_GPO2_BUF: u32 = 0;
const TPS65218_REG_CONFIG2: u32 = 0;
const TPS65218_CONFIG2_DC12_RST: u32 = 0;
const EINVAL: i32 = 22;
const ENOTSUPP: i32 = 524;
const ENOMEM: i32 = 12;

unsafe fn tps65218_gpio_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Tps65218Gpio;
    let tps = (*gpio).tps65218;
    let mut val = 0u32;
    let ret = regmap_read((*tps).regmap, TPS65218_REG_ENABLE2, &mut val);
    if ret != 0 { return ret; }
    if (val & (TPS65218_ENABLE2_GPIO1 << offset)) != 0 { 1 } else { 0 }
}

unsafe fn tps65218_gpio_set(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Tps65218Gpio;
    let tps = (*gpio).tps65218;
    let bit = TPS65218_ENABLE2_GPIO1 << offset;
    if value != 0 {
        tps65218_set_bits(tps, TPS65218_REG_ENABLE2, bit, bit, TPS65218_PROTECT_L1)
    } else {
        tps65218_clear_bits(tps, TPS65218_REG_ENABLE2, bit, TPS65218_PROTECT_L1)
    }
}

unsafe fn tps65218_gpio_output(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    // Only drives GPOs
    tps65218_gpio_set(gc, offset, value)
}

unsafe fn tps65218_gpio_request(gc: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Tps65218Gpio;
    let tps = (*gpio).tps65218;
    if gpiochip_line_is_open_source(gc, offset) { return -EINVAL; }
    match offset {
        0 => {
            if !gpiochip_line_is_open_drain(gc, offset) { return -EINVAL; }
            let mut ret = tps65218_clear_bits(tps, TPS65218_REG_SEQ7, TPS65218_SEQ7_GPO1_SEQ_MASK, TPS65218_PROTECT_L1);
            if ret != 0 { return ret; }
            ret = tps65218_clear_bits(tps, TPS65218_REG_CONFIG1, TPS65218_CONFIG1_IO1_SEL, TPS65218_PROTECT_L1);
            if ret != 0 { return ret; }
        }
        1 => {
            let ret = tps65218_clear_bits(tps, TPS65218_REG_CONFIG1, TPS65218_CONFIG1_IO1_SEL, TPS65218_PROTECT_L1);
            if ret != 0 { return ret; }
        }
        2 => {
            if !gpiochip_line_is_open_drain(gc, offset) { return -EINVAL; }
            let mut ret = tps65218_clear_bits(tps, TPS65218_REG_SEQ7, TPS65218_SEQ7_GPO3_SEQ_MASK, TPS65218_PROTECT_L1);
            if ret != 0 { return ret; }
            ret = tps65218_clear_bits(tps, TPS65218_REG_CONFIG2, TPS65218_CONFIG2_DC12_RST, TPS65218_PROTECT_L1);
            if ret != 0 { return ret; }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn tps65218_gpio_set_config(gc: *mut GpioChip, offset: u32, config: c_ulong) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Tps65218Gpio;
    let tps = (*gpio).tps65218;
    let param = pinconf_to_config_param(config);
    match offset {
        0 | 2 => { if matches!(param, PinConfigParam::DriveOpenDrain) { 0 } else { -ENOTSUPP } }
        1 => match param {
            PinConfigParam::DriveOpenDrain => tps65218_clear_bits(tps, TPS65218_REG_CONFIG1, TPS65218_CONFIG1_GPO2_BUF, TPS65218_PROTECT_L1),
            PinConfigParam::DrivePushPull => tps65218_set_bits(tps, TPS65218_REG_CONFIG1, TPS65218_CONFIG1_GPO2_BUF, TPS65218_CONFIG1_GPO2_BUF, TPS65218_PROTECT_L1),
        },
        _ => -ENOTSUPP,
    }
}

unsafe fn tps65218_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let tps = dev_get_drvdata(core::ptr::null_mut());
    let gpio = devm_kzalloc(core::ptr::null_mut(), core::mem::size_of::<Tps65218Gpio>(), 0) as *mut Tps65218Gpio;
    if gpio.is_null() { return -ENOMEM; }
    (*gpio).tps65218 = tps;
    devm_gpiochip_add_data(core::ptr::null_mut(), &mut (*gpio).gpio_chip, gpio as *mut c_void)
}

// The C gpio_chip template and platform-driver registration are represented as
// external kernel objects; their field values are preserved here as metadata.
pub const TEMPLATE_CHIP_LABEL: &[u8] = b"gpio-tps65218\0";
pub const TEMPLATE_CHIP_CAN_SLEEP: bool = true;
pub const TEMPLATE_CHIP_NGPIO: u32 = 3;
pub const TEMPLATE_CHIP_BASE: i32 = -1;
pub const TPS65218_GPIO_DT_COMPATIBLE: &[u8] = b"ti,tps65218-gpio\0";
pub const TPS65218_GPIO_PLATFORM_NAME: &[u8] = b"tps65218-gpio\0";
pub const MODULE_AUTHOR: &[u8] = b"Nicolas Saenz Julienne <nicolassaenzj@gmail.com>\0";
pub const MODULE_DESCRIPTION: &[u8] = b"GPO interface for TPS65218 PMICs\0";
pub const MODULE_LICENSE: &[u8] = b"GPL v2\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
