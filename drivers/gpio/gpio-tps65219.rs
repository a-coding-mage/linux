// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO driver for TI TPS65214/TPS65215/TPS65219 PMICs
 *
 * Copyright (C) 2022, 2025 Texas Instruments Incorporated - http://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel bindings are intentionally
// referenced here rather than reimplemented in this translation unit.

const TPS65219_GPIO0_DIR_MASK: i32 = 1 << 3;
const TPS65214_GPIO0_DIR_MASK: i32 = 1 << 1;
const TPS6521X_GPIO0_OFFSET: i32 = 2;
const TPS6521X_GPIO0_IDX: u32 = 0;

/*
 * TPS65214 GPIO mapping
 * Linux gpio offset 0 -> GPIO (pin16) -> bit_offset 2
 * Linux gpio offset 1 -> GPO1 (pin9 ) -> bit_offset 0
 *
 * TPS65215 & TPS65219 GPIO mapping
 * Linux gpio offset 0 -> GPIO (pin16) -> bit_offset 2
 * Linux gpio offset 1 -> GPO1 (pin8 ) -> bit_offset 0
 * Linux gpio offset 2 -> GPO2 (pin17) -> bit_offset 1
 */

#[repr(C)]
pub struct tps65219_gpio {
    pub change_dir: Option<unsafe extern "C" fn(*mut gpio_chip, u32, u32) -> i32>,
    pub gpio_chip: gpio_chip,
    pub tps: *mut tps65219,
}

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut tps65219_gpio;
    fn regmap_read(regmap: *mut regmap, reg: u32, val: *mut i32) -> i32;
    fn regmap_update_bits(regmap: *mut regmap, reg: u32, mask: i32, val: i32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_get_device_id(pdev: *mut platform_device) -> *mut platform_device_id;
    fn dev_get_drvdata(dev: *mut device) -> *mut tps65219;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
}

#[repr(C)] pub struct gpio_chip { _private: [u8; 0] }
#[repr(C)] pub struct tps65219 { pub regmap: *mut regmap, pub dev: *mut device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct platform_device_id { pub name: *const core::ffi::c_char, pub driver_data: usize }

const GPIO_LINE_DIRECTION_IN: i32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;
const TPS65219_REG_GENERAL_CONFIG: u32 = 0;
const TPS65219_REG_MFP_1_CONFIG: u32 = 0;
const TPS65219_REG_MFP_CTRL: u32 = 0;
const TPS65219_MFP_GPIO_STATUS_MASK: u32 = 0;
const ENOTSUPP: i32 = 524;
const ENOMEM: i32 = 12;
const ENODATA: i32 = 61;
const TPS65214: usize = 0;
const TPS65219: usize = 1;

unsafe fn tps65214_gpio_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let mut val = 0;
    if offset != TPS6521X_GPIO0_IDX { return GPIO_LINE_DIRECTION_OUT; }
    let ret = regmap_read(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_GENERAL_CONFIG, &mut val);
    if ret != 0 { return ret; }
    if (val & TPS65214_GPIO0_DIR_MASK) == 0 { 1 } else { 0 }
}

unsafe fn tps65219_gpio_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let mut val = 0;
    if offset != TPS6521X_GPIO0_IDX { return GPIO_LINE_DIRECTION_OUT; }
    let ret = regmap_read(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_MFP_1_CONFIG, &mut val);
    if ret != 0 { return ret; }
    if (val & TPS65219_GPIO0_DIR_MASK) != 0 { 1 } else { 0 }
}

unsafe fn tps65219_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let dev = gpio.tps.as_ref().unwrap().dev;
    let mut val = 0;
    if offset != TPS6521X_GPIO0_IDX { dev_err(dev, b"GPIO%d is output only, cannot get\0".as_ptr() as _, offset); return -ENOTSUPP; }
    let ret = regmap_read(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_MFP_CTRL, &mut val);
    if ret != 0 { return ret; }
    let ret = if (val & (1 << TPS65219_MFP_GPIO_STATUS_MASK)) != 0 { 1 } else { 0 };
    dev_warn(dev, b"GPIO%d = %d, MULTI_DEVICE_ENABLE, not a standard GPIO\0".as_ptr() as _, offset, ret);
    if tps65219_gpio_get_direction(gc, offset) == GPIO_LINE_DIRECTION_OUT { return -ENOTSUPP; }
    ret
}

unsafe fn tps65219_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let bit = if offset == TPS6521X_GPIO0_IDX { TPS6521X_GPIO0_OFFSET } else { offset as i32 - 1 };
    let mask = 1 << bit;
    regmap_update_bits(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_GENERAL_CONFIG, mask, if value != 0 { mask } else { 0 })
}

unsafe fn tps65219_gpio_change_direction(gc: *mut gpio_chip, offset: u32, direction: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    dev_err(gpio.tps.as_ref().unwrap().dev, b"GPIO%d direction set by NVM, change to %u failed, not allowed by specification\n\0".as_ptr() as _, offset, direction);
    -ENOTSUPP
}

unsafe fn tps65214_gpio_change_direction(gc: *mut gpio_chip, offset: u32, direction: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    let dev = gpio.tps.as_ref().unwrap().dev;
    let mut val = 0;
    let mut ret = regmap_read(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_MFP_1_CONFIG, &mut val);
    if ret != 0 { return ret; }
    ret = if (val & TPS65219_GPIO0_DIR_MASK) != 0 { 1 } else { 0 };
    if ret != 0 { dev_err(dev, b"GPIO%d configured as VSEL, not GPIO\n\0".as_ptr() as _, offset); }
    ret = regmap_update_bits(gpio.tps.as_ref().unwrap().regmap, TPS65219_REG_GENERAL_CONFIG, TPS65214_GPIO0_DIR_MASK, direction as i32);
    if ret != 0 { dev_err(dev, b"Fail to change direction to %u for GPIO%d.\n\0".as_ptr() as _, direction, offset); }
    ret
}

unsafe fn tps65219_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    if offset != TPS6521X_GPIO0_IDX { dev_err(gpio.tps.as_ref().unwrap().dev, b"GPIO%d is output only, cannot change to input\n\0".as_ptr() as _, offset); return -ENOTSUPP; }
    if tps65219_gpio_get_direction(gc, offset) == GPIO_LINE_DIRECTION_IN { return 0; }
    (gpio.change_dir.unwrap())(gc, offset, GPIO_LINE_DIRECTION_IN as u32)
}

unsafe fn tps65219_gpio_direction_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = &mut *gpiochip_get_data(gc);
    tps65219_gpio_set(gc, offset, value);
    if offset != TPS6521X_GPIO0_IDX { return 0; }
    if tps65219_gpio_get_direction(gc, offset) == GPIO_LINE_DIRECTION_OUT { return 0; }
    (gpio.change_dir.unwrap())(gc, offset, GPIO_LINE_DIRECTION_OUT as u32)
}

unsafe fn tps65219_gpio_probe(pdev: *mut platform_device) -> i32 {
    let chip = (*platform_get_device_id(pdev)).driver_data;
    let tps = dev_get_drvdata((*pdev).dev.parent);
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<tps65219_gpio>(), 0) as *mut tps65219_gpio;
    if gpio.is_null() { return -ENOMEM; }
    if chip == TPS65214 {
        core::ptr::write(&mut (*gpio).gpio_chip, core::mem::zeroed());
        (*gpio).change_dir = Some(tps65214_gpio_change_direction);
    } else if chip == TPS65219 {
        core::ptr::write(&mut (*gpio).gpio_chip, core::mem::zeroed());
        (*gpio).change_dir = Some(tps65219_gpio_change_direction);
    } else { return -ENODATA; }
    (*gpio).tps = tps;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gpio_chip, gpio as *mut core::ffi::c_void)
}

#[repr(C)]
pub struct tps6521x_gpio_id { pub name: *const core::ffi::c_char, pub driver_data: usize }

// Platform ID table, module metadata, and driver registration supplied by the
// kernel module framework.
#[no_mangle]
pub static tps6521x_gpio_id_table: [tps6521x_gpio_id; 3] = [
    tps6521x_gpio_id { name: b"tps65214-gpio\0".as_ptr() as _, driver_data: TPS65214 },
    tps6521x_gpio_id { name: b"tps65219-gpio\0".as_ptr() as _, driver_data: TPS65219 },
    tps6521x_gpio_id { name: core::ptr::null(), driver_data: 0 },
];

extern "C" {
    static mut tps65214_template_chip: gpio_chip;
    static mut tps65219_template_chip: gpio_chip;
    static mut tps65219_gpio_driver: core::ffi::c_void;
}

// MODULE_AUTHOR("Jonathan Cormier <jcormier@criticallink.com>");
// MODULE_DESCRIPTION("TPS65214/TPS65215/TPS65219 GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
