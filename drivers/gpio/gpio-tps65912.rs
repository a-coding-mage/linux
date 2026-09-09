// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO driver for TI TPS65912x PMICs
 *
 * Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com/
 *	Andrew F. Davis <afd@ti.com>
 *
 * Based on the Arizona GPIO driver and the previous TPS65912 driver by
 * Margarita Olaya Cabrera <magi@slimlogic.co.uk>
 */

// Dependencies supplied by the kernel and TPS65912 MFD implementation.

#[repr(C)]
pub struct tps65912_gpio {
    pub gpio_chip: gpio_chip,
    pub tps: *mut tps65912,
}

extern "C" {
    pub fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    pub fn regmap_read(regmap: *mut regmap, reg: u32, val: *mut i32) -> i32;
    pub fn regmap_update_bits(regmap: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub base: i32,
    pub ngpio: u32,
    pub can_sleep: bool,
    pub parent: *mut device,
}

#[repr(C)]
pub struct tps65912 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
}

#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct platform_device_id {
    pub name: *const core::ffi::c_char,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub id_table: *const platform_device_id,
}

pub const TPS65912_GPIO1: u32 = 0;
pub const GPIO_CFG_MASK: u32 = 0;
pub const GPIO_SET_MASK: u32 = 0;
pub const GPIO_STS_MASK: u32 = 0;
pub const GPIO_LINE_DIRECTION_OUT: i32 = 0;
pub const GPIO_LINE_DIRECTION_IN: i32 = 1;
pub const GFP_KERNEL: u32 = 0;
pub const ENOMEM: i32 = 12;

unsafe extern "C" fn tps65912_gpio_get_direction(
    gc: *mut gpio_chip,
    offset: u32,
) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut tps65912_gpio;
    let mut val: i32 = 0;

    let ret = regmap_read((*(*gpio).tps).regmap, TPS65912_GPIO1.wrapping_add(offset), &mut val);
    if ret != 0 {
        return ret;
    }

    if (val as u32) & GPIO_CFG_MASK != 0 {
        GPIO_LINE_DIRECTION_OUT
    } else {
        GPIO_LINE_DIRECTION_IN
    }
}

unsafe extern "C" fn tps65912_gpio_direction_input(
    gc: *mut gpio_chip,
    offset: u32,
) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut tps65912_gpio;
    regmap_update_bits((*(*gpio).tps).regmap, TPS65912_GPIO1.wrapping_add(offset), GPIO_CFG_MASK, 0)
}

unsafe extern "C" fn tps65912_gpio_direction_output(
    gc: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut tps65912_gpio;
    let ret = regmap_update_bits(
        (*(*gpio).tps).regmap,
        TPS65912_GPIO1.wrapping_add(offset),
        GPIO_SET_MASK,
        if value != 0 { GPIO_SET_MASK } else { 0 },
    );
    if ret != 0 {
        return ret;
    }
    regmap_update_bits(
        (*(*gpio).tps).regmap,
        TPS65912_GPIO1.wrapping_add(offset),
        GPIO_CFG_MASK,
        GPIO_CFG_MASK,
    )
}

unsafe extern "C" fn tps65912_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut tps65912_gpio;
    let mut val: i32 = 0;
    let ret = regmap_read((*(*gpio).tps).regmap, TPS65912_GPIO1.wrapping_add(offset), &mut val);
    if ret != 0 {
        return ret;
    }
    if (val as u32) & GPIO_STS_MASK != 0 { 1 } else { 0 }
}

unsafe extern "C" fn tps65912_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) {
    let gpio = gpiochip_get_data(gc) as *mut tps65912_gpio;
    let _ = regmap_update_bits(
        (*(*gpio).tps).regmap,
        TPS65912_GPIO1.wrapping_add(offset),
        GPIO_SET_MASK,
        if value != 0 { GPIO_SET_MASK } else { 0 },
    );
}

static mut template_chip: gpio_chip = gpio_chip {
    label: b"tps65912-gpio\0".as_ptr() as *const core::ffi::c_char,
    owner: core::ptr::null_mut(),
    get_direction: Some(tps65912_gpio_get_direction),
    direction_input: Some(tps65912_gpio_direction_input),
    direction_output: Some(tps65912_gpio_direction_output),
    get: Some(tps65912_gpio_get),
    set: Some(tps65912_gpio_set),
    base: -1,
    ngpio: 5,
    can_sleep: true,
    parent: core::ptr::null_mut(),
};

unsafe extern "C" fn tps65912_gpio_probe(pdev: *mut platform_device) -> i32 {
    let parent = (*pdev).dev.parent;
    let tps = dev_get_drvdata(parent) as *mut tps65912;
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<tps65912_gpio>(), GFP_KERNEL)
        as *mut tps65912_gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }
    (*gpio).tps = dev_get_drvdata(parent) as *mut tps65912;
    (*gpio).gpio_chip = template_chip;
    (*gpio).gpio_chip.parent = (*tps).dev;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gpio_chip, gpio as *mut core::ffi::c_void)
}

static tps65912_gpio_id_table: [platform_device_id; 2] = [
    platform_device_id { name: b"tps65912-gpio\0".as_ptr() as *const core::ffi::c_char },
    platform_device_id { name: core::ptr::null() },
];

static mut tps65912_gpio_driver: platform_driver = platform_driver {
    driver: device_driver { name: b"tps65912-gpio\0".as_ptr() as *const core::ffi::c_char },
    probe: Some(tps65912_gpio_probe),
    id_table: tps65912_gpio_id_table.as_ptr(),
};

// Equivalent to module_platform_driver(tps65912_gpio_driver).
// MODULE_DEVICE_TABLE(platform, tps65912_gpio_id_table);
// MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
// MODULE_DESCRIPTION("TPS65912 GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
