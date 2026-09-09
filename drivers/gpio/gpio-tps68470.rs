// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO driver for TPS68470 PMIC
 *
 * Copyright (C) 2017 Intel Corporation
 *
 * Authors:
 *	Antti Laakso <antti.laakso@intel.com>
 *	Tianshu Qiu <tian.shu.qiu@intel.com>
 *	Jian Xu Zheng <jian.xu.zheng@intel.com>
 *	Yuning Pu <yuning.pu@intel.com>
 */

// Linux kernel dependencies supplied by other translation units.

const TPS68470_N_LOGIC_OUTPUT: usize = 3;
const TPS68470_N_REGULAR_GPIO: usize = 7;
const TPS68470_N_GPIO: usize = TPS68470_N_LOGIC_OUTPUT + TPS68470_N_REGULAR_GPIO;

#[repr(C)]
struct Tps68470GpioData {
    tps68470_regmap: *mut Regmap,
    gc: GpioChip,
}

extern "C" {
    type Regmap;
    type Device;
    type PlatformDevice;
    type Module;

    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut Tps68470GpioData;
    fn regmap_read(regmap: *mut Regmap, reg: u32, val: *mut i32) -> i32;
    fn regmap_update_bits(regmap: *mut Regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut Device) -> *mut Regmap;
    fn devm_gpiochip_add_data(dev: *mut Device, gc: *mut GpioChip, data: *mut Tps68470GpioData) -> i32;
    fn device_property_present(dev: *mut Device, name: *const u8) -> bool;
}

#[repr(C)]
struct GpioChip {
    label: *const u8,
    owner: *mut Module,
    direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    can_sleep: bool,
    names: *const *const u8,
    ngpio: u32,
    base: i32,
    parent: *mut Device,
}

const TPS68470_REG_GPDO: u32 = 0;
const TPS68470_REG_SGPO: u32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const TPS68470_GPIO_MODE_MASK: i32 = 0;
const TPS68470_GPIO_MODE_OUT_CMOS: i32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const THIS_MODULE: *mut Module = core::ptr::null_mut();

extern "C" {
    fn tps68470_gpio_ctl_reg_a(offset: u32) -> u32;
}

unsafe fn tps68470_gpio_get(gc: *mut GpioChip, mut offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let regmap = gpio.tps68470_regmap;
    let mut reg = TPS68470_REG_GPDO;
    let mut val: i32 = 0;

    if offset >= TPS68470_N_REGULAR_GPIO as u32 {
        offset -= TPS68470_N_REGULAR_GPIO as u32;
        reg = TPS68470_REG_SGPO;
    }

    let ret = regmap_read(regmap, reg, &mut val);
    if ret != 0 {
        dev_err(gpio.gc.parent, b"reg 0x%x read failed\0".as_ptr(), TPS68470_REG_SGPO);
        return ret;
    }
    if (val & (1_i32.wrapping_shl(offset))) != 0 { 1 } else { 0 }
}

unsafe fn tps68470_gpio_get_direction(gc: *mut GpioChip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let regmap = gpio.tps68470_regmap;
    let mut val: i32 = 0;

    /* rest are always outputs */
    if offset >= TPS68470_N_REGULAR_GPIO as u32 { return GPIO_LINE_DIRECTION_OUT; }
    let ret = regmap_read(regmap, tps68470_gpio_ctl_reg_a(offset), &mut val);
    if ret != 0 {
        dev_err(gpio.gc.parent, b"reg 0x%x read failed\0".as_ptr(), tps68470_gpio_ctl_reg_a(offset));
        return ret;
    }
    val &= TPS68470_GPIO_MODE_MASK;
    if val >= TPS68470_GPIO_MODE_OUT_CMOS { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn tps68470_gpio_set(gc: *mut GpioChip, mut offset: u32, value: i32) {
    let gpio = &mut *gpiochip_get_data(gc);
    let mut reg = TPS68470_REG_GPDO;
    if offset >= TPS68470_N_REGULAR_GPIO as u32 { reg = TPS68470_REG_SGPO; offset -= TPS68470_N_REGULAR_GPIO as u32; }
    let bit = 1_u32.wrapping_shl(offset);
    let _ = regmap_update_bits(gpio.tps68470_regmap, reg, bit, if value != 0 { bit } else { 0 });
}

unsafe fn tps68470_gpio_output(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let ret = tps68470_gpio_set_result(gc, offset, value);
    if ret != 0 { return ret; }
    if offset >= TPS68470_N_REGULAR_GPIO as u32 { return 0; }
    regmap_update_bits(gpio.tps68470_regmap, tps68470_gpio_ctl_reg_a(offset), TPS68470_GPIO_MODE_MASK as u32, TPS68470_GPIO_MODE_OUT_CMOS as u32)
}

unsafe fn tps68470_gpio_set_result(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    tps68470_gpio_set(gc, offset, value); 0
}

unsafe fn tps68470_gpio_input(gc: *mut GpioChip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    if offset >= TPS68470_N_REGULAR_GPIO as u32 { return -EINVAL; }
    regmap_update_bits(gpio.tps68470_regmap, tps68470_gpio_ctl_reg_a(offset), TPS68470_GPIO_MODE_MASK as u32, 0)
}

unsafe fn tps68470_enable_i2c_daisy_chain(gc: *mut GpioChip) -> i32 {
    let ret = tps68470_gpio_input(gc, 1);
    if ret != 0 { return ret; }
    tps68470_gpio_input(gc, 2)
}

static TPS68470_NAMES: [&[u8]; TPS68470_N_GPIO] = [b"gpio.0", b"gpio.1", b"gpio.2", b"gpio.3", b"gpio.4", b"gpio.5", b"gpio.6", b"s_enable", b"s_idle", b"s_resetn"];

extern "C" {
    fn platform_device_dev(pdev: *mut PlatformDevice) -> *mut Device;
}

unsafe fn tps68470_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let pdev_dev = platform_device_dev(pdev);
    let gpio = devm_kzalloc(pdev_dev, core::mem::size_of::<Tps68470GpioData>(), GFP_KERNEL)
        as *mut Tps68470GpioData;
    if gpio.is_null() { return -ENOMEM; }

    (*gpio).tps68470_regmap = dev_get_drvdata(core::ptr::null_mut());
    (*gpio).gc.label = b"tps68470-gpio\0".as_ptr();
    (*gpio).gc.owner = THIS_MODULE;
    (*gpio).gc.direction_input = Some(tps68470_gpio_input);
    (*gpio).gc.direction_output = Some(tps68470_gpio_output);
    (*gpio).gc.get = Some(tps68470_gpio_get);
    (*gpio).gc.get_direction = Some(tps68470_gpio_get_direction);
    (*gpio).gc.set = None;
    (*gpio).gc.can_sleep = true;
    (*gpio).gc.ngpio = TPS68470_N_GPIO as u32;
    (*gpio).gc.base = -1;
    (*gpio).gc.parent = pdev_dev;

    let mut ret = devm_gpiochip_add_data(pdev_dev, &mut (*gpio).gc, gpio);
    if ret != 0 { return ret; }
    if device_property_present(pdev_dev, b"daisy-chain-enable\0".as_ptr()) {
        ret = tps68470_enable_i2c_daisy_chain(&mut (*gpio).gc);
    }
    ret
}

// static struct platform_driver tps68470_gpio_driver and module_platform_driver()
// are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
