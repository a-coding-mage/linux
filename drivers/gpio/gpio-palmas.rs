// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI Palma series PMIC's GPIO driver.
 *
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 *
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

// External Linux kernel, GPIO, PALMAS, device-tree, and platform-driver
// declarations are supplied by other translation units.

#[repr(C)]
pub struct palmas_gpio {
    pub gpio_chip: gpio_chip,
    pub palmas: *mut palmas,
}

#[repr(C)]
pub struct palmas_device_data {
    pub ngpio: c_int,
}

unsafe extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn palmas_read(palmas: *mut palmas, base: c_int, reg: c_uint, val: *mut c_uint) -> c_int;
    fn palmas_write(palmas: *mut palmas, base: c_int, reg: c_uint, val: c_uint) -> c_int;
    fn palmas_update_bits(
        palmas: *mut palmas,
        base: c_int,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn palmas_irq_get_virq(palmas: *mut palmas, irq: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
pub struct gpio_chip {
    pub owner: *mut module,
    pub label: *const core::ffi::c_char,
    pub ngpio: c_uint,
    pub can_sleep: bool,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int)>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub parent: *mut device,
    pub base: c_int,
}

#[repr(C)]
pub struct palmas;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct palmas_platform_data {
    pub gpio_base: c_int,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

type c_int = i32;
type c_uint = u32;

const PALMAS_GPIO_BASE: c_int = 0;
const PALMAS_GPIO_DATA_DIR: c_uint = 0;
const PALMAS_GPIO_DATA_DIR2: c_uint = 1;
const PALMAS_GPIO_DATA_OUT: c_uint = 2;
const PALMAS_GPIO_DATA_OUT2: c_uint = 3;
const PALMAS_GPIO_DATA_IN: c_uint = 4;
const PALMAS_GPIO_DATA_IN2: c_uint = 5;
const PALMAS_GPIO_SET_DATA_OUT: c_uint = 6;
const PALMAS_GPIO_SET_DATA_OUT2: c_uint = 7;
const PALMAS_GPIO_CLEAR_DATA_OUT: c_uint = 8;
const PALMAS_GPIO_CLEAR_DATA_OUT2: c_uint = 9;
const PALMAS_GPIO_0_IRQ: c_int = 0;
const GPIO_LINE_DIRECTION_OUT: c_int = 0;
const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

unsafe fn bit(offset: c_uint) -> c_uint { 1u32.wrapping_shl(offset) }

pub unsafe extern "C" fn palmas_gpio_get(gc: *mut gpio_chip, mut offset: c_uint) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let palmas = (*pg).palmas;
    let mut val = 0u32;
    let mut reg: c_uint;
    let gpio16 = offset / 8;
    offset %= 8;
    reg = if gpio16 != 0 { PALMAS_GPIO_DATA_DIR2 } else { PALMAS_GPIO_DATA_DIR };
    let mut ret = palmas_read(palmas, PALMAS_GPIO_BASE, reg, &mut val);
    if ret < 0 { return ret; }
    reg = if val & bit(offset) != 0 {
        if gpio16 != 0 { PALMAS_GPIO_DATA_OUT2 } else { PALMAS_GPIO_DATA_OUT }
    } else if gpio16 != 0 { PALMAS_GPIO_DATA_IN2 } else { PALMAS_GPIO_DATA_IN };
    ret = palmas_read(palmas, PALMAS_GPIO_BASE, reg, &mut val);
    if ret < 0 { return ret; }
    if val & bit(offset) != 0 { 1 } else { 0 }
}

pub unsafe extern "C" fn palmas_gpio_set(gc: *mut gpio_chip, mut offset: c_uint, value: c_int) {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let gpio16 = offset / 8;
    offset %= 8;
    let reg = if gpio16 != 0 {
        if value != 0 { PALMAS_GPIO_SET_DATA_OUT2 } else { PALMAS_GPIO_CLEAR_DATA_OUT2 }
    } else if value != 0 { PALMAS_GPIO_SET_DATA_OUT } else { PALMAS_GPIO_CLEAR_DATA_OUT };
    let _ = palmas_write((*pg).palmas, PALMAS_GPIO_BASE, reg, bit(offset));
}

pub unsafe extern "C" fn palmas_gpio_output(gc: *mut gpio_chip, mut offset: c_uint, value: c_int) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let palmas = (*pg).palmas;
    let gpio16 = offset / 8;
    offset %= 8;
    let reg = if gpio16 != 0 { PALMAS_GPIO_DATA_DIR2 } else { PALMAS_GPIO_DATA_DIR };
    let ret = palmas_gpio_set_ret(gc, offset, value);
    if ret != 0 { return ret; }
    palmas_update_bits(palmas, PALMAS_GPIO_BASE, reg, bit(offset), bit(offset))
}

unsafe fn palmas_gpio_set_ret(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let gpio16 = offset / 8;
    let reg = if gpio16 != 0 { if value != 0 { PALMAS_GPIO_SET_DATA_OUT2 } else { PALMAS_GPIO_CLEAR_DATA_OUT2 } } else { if value != 0 { PALMAS_GPIO_SET_DATA_OUT } else { PALMAS_GPIO_CLEAR_DATA_OUT } };
    palmas_write((*pg).palmas, PALMAS_GPIO_BASE, reg, bit(offset % 8))
}

pub unsafe extern "C" fn palmas_gpio_input(gc: *mut gpio_chip, mut offset: c_uint) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let gpio16 = offset / 8;
    offset %= 8;
    let reg = if gpio16 != 0 { PALMAS_GPIO_DATA_DIR2 } else { PALMAS_GPIO_DATA_DIR };
    palmas_update_bits((*pg).palmas, PALMAS_GPIO_BASE, reg, bit(offset), 0)
}

pub unsafe extern "C" fn palmas_gpio_get_direction(gc: *mut gpio_chip, mut offset: c_uint) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    let gpio16 = offset / 8;
    offset %= 8;
    let reg = if gpio16 != 0 { PALMAS_GPIO_DATA_DIR2 } else { PALMAS_GPIO_DATA_DIR };
    let mut val = 0u32;
    let ret = palmas_read((*pg).palmas, PALMAS_GPIO_BASE, reg, &mut val);
    if ret != 0 { return ret; }
    if val & bit(offset) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

pub unsafe extern "C" fn palmas_gpio_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let pg = gpiochip_get_data(gc) as *mut palmas_gpio;
    palmas_irq_get_virq((*pg).palmas, PALMAS_GPIO_0_IRQ + offset as c_int)
}

pub static palmas_dev_data: palmas_device_data = palmas_device_data { ngpio: 8 };
pub static tps80036_dev_data: palmas_device_data = palmas_device_data { ngpio: 16 };

pub static of_palmas_gpio_match: [of_device_id; 5] = [
    of_device_id { compatible: b"ti,palmas-gpio\0".as_ptr() as *const _, data: &palmas_dev_data as *const _ as *const _ },
    of_device_id { compatible: b"ti,tps65913-gpio\0".as_ptr() as *const _, data: &palmas_dev_data as *const _ as *const _ },
    of_device_id { compatible: b"ti,tps65914-gpio\0".as_ptr() as *const _, data: &palmas_dev_data as *const _ as *const _ },
    of_device_id { compatible: b"ti,tps80036-gpio\0".as_ptr() as *const _, data: &tps80036_dev_data as *const _ as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe extern "C" fn palmas_gpio_probe(pdev: *mut platform_device) -> c_int {
    let palmas = dev_get_drvdata((*pdev).dev.parent());
    let mut dev_data = of_device_get_match_data(&mut (*pdev).dev) as *const palmas_device_data;
    if dev_data.is_null() { dev_data = &palmas_dev_data; }
    let pg = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<palmas_gpio>(), GFP_KERNEL) as *mut palmas_gpio;
    if pg.is_null() { return -ENOMEM; }
    (*pg).palmas = palmas as *mut palmas;
    (*pg).gpio_chip.ngpio = (*dev_data).ngpio as c_uint;
    (*pg).gpio_chip.can_sleep = true;
    (*pg).gpio_chip.direction_input = Some(palmas_gpio_input);
    (*pg).gpio_chip.direction_output = Some(palmas_gpio_output);
    (*pg).gpio_chip.get_direction = Some(palmas_gpio_get_direction);
    (*pg).gpio_chip.to_irq = Some(palmas_gpio_to_irq);
    (*pg).gpio_chip.parent = &mut (*pdev).dev;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*pg).gpio_chip, pg as *mut _)
}

impl device {
    unsafe fn parent(&mut self) -> *mut device { core::ptr::null_mut() }
}

pub static mut palmas_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: b"palmas-gpio\0".as_ptr() as *const _, of_match_table: of_palmas_gpio_match.as_ptr() },
    probe: Some(palmas_gpio_probe),
};

pub unsafe extern "C" fn palmas_gpio_init() -> c_int { platform_driver_register(&mut palmas_gpio_driver) }
pub unsafe extern "C" fn palmas_gpio_exit() { platform_driver_unregister(&mut palmas_gpio_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
