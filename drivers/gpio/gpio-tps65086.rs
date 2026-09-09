// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015-2023 Texas Instruments Incorporated - https://www.ti.com/
 *\tAndrew Davis <afd@ti.com>
 *
 * Based on the TPS65912 driver
 */

// Dependencies supplied by the Linux GPIO, module, platform-device, and TPS65086 APIs.

#[repr(C)]
pub struct Tps65086Gpio {
    pub chip: GpioChip,
    pub tps: *mut Tps65086,
}

extern "C" {
    pub fn gpiochip_get_data(chip: *mut GpioChip) -> *mut core::ffi::c_void;
    pub fn regmap_update_bits(
        regmap: *mut Regmap,
        reg: u32,
        mask: u32,
        val: u32,
    ) -> i32;
    pub fn regmap_read(regmap: *mut Regmap, reg: u32, val: *mut i32) -> i32;
    pub fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn dev_get_drvdata(dev: *mut Device) -> *mut Tps65086;
    pub fn devm_gpiochip_add_data(
        dev: *mut Device,
        chip: *mut GpioChip,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
pub struct GpioChip {
    pub label: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    pub base: i32,
    pub ngpio: u32,
    pub can_sleep: bool,
    pub parent: *mut Device,
}

#[repr(C)]
pub struct Tps65086 {
    pub regmap: *mut Regmap,
    pub dev: *mut Device,
}

#[repr(C)]
pub struct Regmap;
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

pub const GPIO_LINE_DIRECTION_OUT: i32 = 0;
pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const GFP_KERNEL: u32 = 0;
pub const TPS65086_GPOCTRL: u32 = 0;

#[inline]
const fn bit(n: u32) -> u32 {
    1u32 << n
}

unsafe extern "C" fn tps65086_gpio_get_direction(
    _chip: *mut GpioChip,
    _offset: u32,
) -> i32 {
    /* This device is output only */
    GPIO_LINE_DIRECTION_OUT
}

unsafe extern "C" fn tps65086_gpio_direction_input(
    _chip: *mut GpioChip,
    _offset: u32,
) -> i32 {
    /* This device is output only */
    -EINVAL
}

unsafe extern "C" fn tps65086_gpio_direction_output(
    chip: *mut GpioChip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Tps65086Gpio;

    /* Set the initial value */
    regmap_update_bits(
        (*(*gpio).tps).regmap,
        TPS65086_GPOCTRL,
        bit(4u32.wrapping_add(offset)),
        if value != 0 { bit(4u32.wrapping_add(offset)) } else { 0 },
    )
}

unsafe extern "C" fn tps65086_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Tps65086Gpio;
    let mut val: i32 = 0;

    let ret = regmap_read((*(*gpio).tps).regmap, TPS65086_GPOCTRL, &mut val);
    if ret < 0 {
        return ret;
    }

    if (val as u32 & bit(4u32.wrapping_add(offset))) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn tps65086_gpio_set(
    chip: *mut GpioChip,
    offset: u32,
    value: i32,
) {
    let gpio = gpiochip_get_data(chip) as *mut Tps65086Gpio;

    let _ = regmap_update_bits(
        (*(*gpio).tps).regmap,
        TPS65086_GPOCTRL,
        bit(4u32.wrapping_add(offset)),
        if value != 0 { bit(4u32.wrapping_add(offset)) } else { 0 },
    );
}

static mut TEMPLATE_CHIP: GpioChip = GpioChip {
    label: b"tps65086-gpio\0".as_ptr() as *const core::ffi::c_char,
    owner: core::ptr::null_mut(),
    get_direction: Some(tps65086_gpio_get_direction),
    direction_input: Some(tps65086_gpio_direction_input),
    direction_output: Some(tps65086_gpio_direction_output),
    get: Some(tps65086_gpio_get),
    set: Some(tps65086_gpio_set),
    base: -1,
    ngpio: 4,
    can_sleep: true,
    parent: core::ptr::null_mut(),
};

unsafe extern "C" fn tps65086_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let gpio = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<Tps65086Gpio>(),
        GFP_KERNEL,
    ) as *mut Tps65086Gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).tps = dev_get_drvdata((*pdev).dev.parent);
    (*gpio).chip = TEMPLATE_CHIP;
    (*gpio).chip.parent = (*gpio).tps.as_ref().unwrap().dev;

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).chip, gpio as *mut core::ffi::c_void)
}

// Platform device ID table: { .name = "tps65086-gpio" }, { /* sentinel */ }.
// MODULE_DEVICE_TABLE(platform, tps65086_gpio_id_table);
// module_platform_driver(tps65086_gpio_driver);
// MODULE_AUTHOR("Andrew Davis <afd@ti.com>");
// MODULE_DESCRIPTION("TPS65086 GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
