// SPDX-License-Identifier: GPL-2.0
/*
 * SAMA5D2 PIOBU GPIO controller
 *
 * Copyright (C) 2018 Microchip Technology Inc. and its subsidiaries
 *
 * Author: Andrei Stefanescu <andrei.stefanescu@microchip.com>
 */

/* Dependencies supplied by the Linux kernel bindings. */

const PIOBU_NUM: u32 = 8;
const PIOBU_REG_SIZE: u32 = 4;

/*
 * backup mode protection register for tamper detection
 * normal mode protection register for tamper detection
 * wakeup signal generation
 */
const PIOBU_BMPR: u32 = 0x7c;
const PIOBU_NMPR: u32 = 0x80;
const PIOBU_WKPR: u32 = 0x90;

const PIOBU_BASE: u32 = 0x18; /* PIOBU offset from SECUMOD base register address. */
const PIOBU_DET_OFFSET: u32 = 16;

/* In the datasheet this bit is called OUTPUT */
const PIOBU_DIRECTION: u32 = 1 << 8;
const PIOBU_OUT: u32 = 1 << 8;
const PIOBU_IN: u32 = 0;

const PIOBU_SOD: u32 = 1 << 9;
const PIOBU_PDS: u32 = 1 << 10;

const PIOBU_HIGH: u32 = 1 << 9;
const PIOBU_LOW: u32 = 0;

#[repr(C)]
pub struct gpio_chip {
    pub label: *const core::ffi::c_char,
    pub parent: *mut device,
    pub owner: *mut module,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub base: i32,
    pub ngpio: u32,
    pub can_sleep: i32,
}

#[repr(C)]
pub struct sama5d2_piobu {
    pub chip: gpio_chip,
    pub regmap: *mut regmap,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device {
    pub dev: device,
    pub name: *const core::ffi::c_char,
}

extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

const GFP_KERNEL: u32 = 0;
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;

unsafe fn sama5d2_piobu_setup_pin(chip: *mut gpio_chip, pin: u32) -> i32 {
    let piobu = chip as *mut sama5d2_piobu;
    let mask = 1u32 << (PIOBU_DET_OFFSET + pin);
    let mut ret = regmap_update_bits((*piobu).regmap, PIOBU_BMPR, mask, 0);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*piobu).regmap, PIOBU_NMPR, mask, 0);
    if ret != 0 { return ret; }
    regmap_update_bits((*piobu).regmap, PIOBU_WKPR, mask, 0)
}

unsafe fn sama5d2_piobu_write_value(chip: *mut gpio_chip, pin: u32, mask: u32, value: u32) -> i32 {
    let piobu = chip as *mut sama5d2_piobu;
    let reg = PIOBU_BASE + pin * PIOBU_REG_SIZE;
    regmap_update_bits((*piobu).regmap, reg, mask, value)
}

unsafe fn sama5d2_piobu_read_value(chip: *mut gpio_chip, pin: u32, mask: u32) -> i32 {
    let piobu = chip as *mut sama5d2_piobu;
    let reg = PIOBU_BASE + pin * PIOBU_REG_SIZE;
    let mut val = 0u32;
    let ret = regmap_read((*piobu).regmap, reg, &mut val);
    if ret < 0 { return ret; }
    (val & mask) as i32
}

unsafe extern "C" fn sama5d2_piobu_get_direction(chip: *mut gpio_chip, pin: u32) -> i32 {
    let ret = sama5d2_piobu_read_value(chip, pin, PIOBU_DIRECTION);
    if ret < 0 { return ret; }
    if ret == PIOBU_IN as i32 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn sama5d2_piobu_direction_input(chip: *mut gpio_chip, pin: u32) -> i32 {
    sama5d2_piobu_write_value(chip, pin, PIOBU_DIRECTION, PIOBU_IN)
}

unsafe extern "C" fn sama5d2_piobu_direction_output(chip: *mut gpio_chip, pin: u32, value: i32) -> i32 {
    let mut val = PIOBU_OUT;
    if value != 0 { val |= PIOBU_HIGH; }
    sama5d2_piobu_write_value(chip, pin, PIOBU_DIRECTION | PIOBU_SOD, val)
}

/* if pin is input, read value from PDS else read from SOD */
unsafe extern "C" fn sama5d2_piobu_get(chip: *mut gpio_chip, pin: u32) -> i32 {
    let mut ret = sama5d2_piobu_get_direction(chip, pin);
    if ret == GPIO_LINE_DIRECTION_IN {
        ret = sama5d2_piobu_read_value(chip, pin, PIOBU_PDS);
    } else if ret == GPIO_LINE_DIRECTION_OUT {
        ret = sama5d2_piobu_read_value(chip, pin, PIOBU_SOD);
    }
    if ret < 0 { return ret; }
    if ret != 0 { 1 } else { 0 }
}

unsafe extern "C" fn sama5d2_piobu_set(chip: *mut gpio_chip, pin: u32, mut value: i32) {
    value = if value == 0 { PIOBU_LOW as i32 } else { PIOBU_HIGH as i32 };
    let _ = sama5d2_piobu_write_value(chip, pin, PIOBU_SOD, value as u32);
}

unsafe extern "C" fn sama5d2_piobu_probe(pdev: *mut platform_device) -> i32 {
    let piobu = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<sama5d2_piobu>(), GFP_KERNEL) as *mut sama5d2_piobu;
    if piobu.is_null() { return -12; }
    (*piobu).chip.label = (*pdev).name;
    (*piobu).chip.parent = &mut (*pdev).dev;
    (*piobu).chip.get_direction = Some(sama5d2_piobu_get_direction);
    (*piobu).chip.direction_input = Some(sama5d2_piobu_direction_input);
    (*piobu).chip.direction_output = Some(sama5d2_piobu_direction_output);
    (*piobu).chip.get = Some(sama5d2_piobu_get);
    (*piobu).chip.set = Some(sama5d2_piobu_set);
    (*piobu).chip.base = -1;
    (*piobu).chip.ngpio = PIOBU_NUM;
    (*piobu).chip.can_sleep = 0;
    (*piobu).regmap = syscon_node_to_regmap(core::ptr::null_mut());
    if (*piobu).regmap.is_null() { return -19; }
    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*piobu).chip, piobu as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    let mut i = 0u32;
    while i < PIOBU_NUM {
        let ret = sama5d2_piobu_setup_pin(&mut (*piobu).chip, i);
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

#[no_mangle]
pub static mut sama5d2_piobu_driver_probe: unsafe extern "C" fn(*mut platform_device) -> i32 = sama5d2_piobu_probe;

/* module_platform_driver(sama5d2_piobu_driver); */
/* MODULE_LICENSE("GPL v2"); MODULE_DESCRIPTION("SAMA5D2 PIOBU controller driver");
 * MODULE_AUTHOR("Andrei Stefanescu <andrei.stefanescu@microchip.com>"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
