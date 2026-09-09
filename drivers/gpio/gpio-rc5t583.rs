// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for RICOH583 power management chip.
 *
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 * Author: Laxman dewangan <ldewangan@nvidia.com>
 *
 * Based on code
 *	Copyright (C) 2011 RICOH COMPANY,LTD
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct rc5t583_gpio {
    pub gpio_chip: gpio_chip,
    pub rc5t583: *mut rc5t583,
}

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn rc5t583_read(parent: *mut device, reg: u8, val: *mut u8) -> i32;
    fn rc5t583_set_bits(parent: *mut device, reg: u8, mask: u8) -> i32;
    fn rc5t583_clear_bits(parent: *mut device, reg: u8, mask: u8) -> i32;
    fn dev_get_drvdata(parent: *mut device) -> *mut rc5t583;
    fn dev_get_platdata(dev: *mut device) -> *mut rc5t583_platform_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct rc5t583 {
    pub dev: *mut device,
    pub irq_base: i32,
}

#[repr(C)]
pub struct rc5t583_platform_data {
    pub gpio_base: i32,
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const u8,
    pub owner: *mut core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut gpio_chip, u32)>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub ngpio: u32,
    pub can_sleep: bool,
    pub parent: *mut device,
    pub base: i32,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
}

const RC5T583_GPIO_MON_IOIN: u8 = 0;
const RC5T583_GPIO_IOOUT: u8 = 0;
const RC5T583_GPIO_IOSEL: u8 = 0;
const RC5T583_GPIO_PGSEL: u8 = 0;
const RC5T583_MAX_GPIO: u32 = 0;
const RC5T583_IRQ_GPIO0: i32 = 0;
const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

#[inline]
fn bit(offset: u32) -> u8 {
    1u8.wrapping_shl(offset)
}

unsafe extern "C" fn rc5t583_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    let parent = (*(*rc5t583_gpio).rc5t583).dev;
    let mut val: u8 = 0;
    let ret = rc5t583_read(parent, RC5T583_GPIO_MON_IOIN, &mut val);
    if ret < 0 {
        return ret;
    }
    if (val & bit(offset)) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn rc5t583_gpio_set(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    let parent = (*(*rc5t583_gpio).rc5t583).dev;
    if val != 0 {
        rc5t583_set_bits(parent, RC5T583_GPIO_IOOUT, bit(offset))
    } else {
        rc5t583_clear_bits(parent, RC5T583_GPIO_IOOUT, bit(offset))
    }
}

unsafe extern "C" fn rc5t583_gpio_dir_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    let parent = (*(*rc5t583_gpio).rc5t583).dev;
    let ret = rc5t583_clear_bits(parent, RC5T583_GPIO_IOSEL, bit(offset));
    if ret < 0 { return ret; }
    // Set pin to gpio mode
    rc5t583_clear_bits(parent, RC5T583_GPIO_PGSEL, bit(offset))
}

unsafe extern "C" fn rc5t583_gpio_dir_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    let parent = (*(*rc5t583_gpio).rc5t583).dev;
    let ret = rc5t583_gpio_set(gc, offset, value);
    if ret != 0 { return ret; }
    let ret = rc5t583_set_bits(parent, RC5T583_GPIO_IOSEL, bit(offset));
    if ret < 0 { return ret; }
    // Set pin to gpio mode
    rc5t583_clear_bits(parent, RC5T583_GPIO_PGSEL, bit(offset))
}

unsafe extern "C" fn rc5t583_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    if offset < RC5T583_MAX_GPIO {
        (*(*rc5t583_gpio).rc5t583).irq_base + RC5T583_IRQ_GPIO0 + offset as i32
    } else { -EINVAL }
}

unsafe extern "C" fn rc5t583_gpio_free(gc: *mut gpio_chip, offset: u32) {
    let rc5t583_gpio = gpiochip_get_data(gc) as *mut rc5t583_gpio;
    let parent = (*(*rc5t583_gpio).rc5t583).dev;
    let _ = rc5t583_set_bits(parent, RC5T583_GPIO_PGSEL, bit(offset));
}

unsafe extern "C" fn rc5t583_gpio_probe(pdev: *mut platform_device) -> i32 {
    let rc5t583 = dev_get_drvdata((*pdev).dev.parent);
    let pdata = dev_get_platdata((*rc5t583).dev);
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rc5t583_gpio>(), GFP_KERNEL)
        as *mut rc5t583_gpio;
    if gpio.is_null() { return -ENOMEM; }
    (*gpio).gpio_chip.label = b"gpio-rc5t583\0".as_ptr();
    (*gpio).gpio_chip.owner = core::ptr::null_mut();
    (*gpio).gpio_chip.free = Some(rc5t583_gpio_free);
    (*gpio).gpio_chip.direction_input = Some(rc5t583_gpio_dir_input);
    (*gpio).gpio_chip.direction_output = Some(rc5t583_gpio_dir_output);
    (*gpio).gpio_chip.set = Some(rc5t583_gpio_set);
    (*gpio).gpio_chip.get = Some(rc5t583_gpio_get);
    (*gpio).gpio_chip.to_irq = Some(rc5t583_gpio_to_irq);
    (*gpio).gpio_chip.ngpio = RC5T583_MAX_GPIO;
    (*gpio).gpio_chip.can_sleep = true;
    (*gpio).rc5t583 = rc5t583;
    (*gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*gpio).gpio_chip.base = -1;
    if !pdata.is_null() && (*pdata).gpio_base != 0 { (*gpio).gpio_chip.base = (*pdata).gpio_base; }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gpio_chip, gpio as *mut core::ffi::c_void)
}

static mut rc5t583_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: b"rc5t583-gpio\0".as_ptr() },
    probe: Some(rc5t583_gpio_probe),
};

unsafe extern "C" fn rc5t583_gpio_init() -> i32 {
    platform_driver_register(&mut rc5t583_gpio_driver)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
