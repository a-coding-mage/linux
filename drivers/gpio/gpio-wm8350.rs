// SPDX-License-Identifier: GPL-2.0+
/*
 * gpiolib support for Wolfson WM835x PMICs
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 */

// Dependencies supplied by the Linux kernel and WM8350 support files.

#[repr(C)]
pub struct wm8350_gpio_data {
    pub wm8350: *mut wm8350,
    pub gpio_chip: gpio_chip,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut wm8350_gpio_data;
    fn wm8350_set_bits(wm8350: *mut wm8350, reg: i32, mask: i32) -> i32;
    fn wm8350_reg_read(wm8350: *mut wm8350, reg: i32) -> i32;
    fn wm8350_clear_bits(wm8350: *mut wm8350, reg: i32, mask: i32) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut wm8350;
    fn dev_get_platdata(dev: *mut device) -> *mut wm8350_platform_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut wm8350_gpio_data,
    ) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
pub struct wm8350 {
    pub dev: *mut device,
    pub irq_base: i32,
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct wm8350_platform_data {
    pub gpio_base: i32,
}

pub type gpio_direction_input_fn = unsafe extern "C" fn(*mut gpio_chip, u32) -> i32;
pub type gpio_get_fn = unsafe extern "C" fn(*mut gpio_chip, u32) -> i32;
pub type gpio_direction_output_fn = unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32;
pub type gpio_set_fn = unsafe extern "C" fn(*mut gpio_chip, u32, i32);
pub type gpio_to_irq_fn = unsafe extern "C" fn(*mut gpio_chip, u32) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_chip {
    pub label: *const u8,
    pub owner: *const core::ffi::c_void,
    pub direction_input: Option<gpio_direction_input_fn>,
    pub get: Option<gpio_get_fn>,
    pub direction_output: Option<gpio_direction_output_fn>,
    pub set: Option<gpio_set_fn>,
    pub to_irq: Option<gpio_to_irq_fn>,
    pub can_sleep: bool,
    pub ngpio: u32,
    pub parent: *mut device,
    pub base: i32,
}

#[repr(C)]
pub struct wm8350_driver_info {
    pub name: *const u8,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: wm8350_driver_info,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

const WM8350_GPIO_CONFIGURATION_I_O: i32 = 0;
const WM8350_GPIO_LEVEL: i32 = 0;
const WM8350_IRQ_GPIO: i32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

unsafe extern "C" fn wm8350_gpio_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm8350_gpio = gpiochip_get_data(chip);
    let wm8350 = (*wm8350_gpio).wm8350;
    wm8350_set_bits(wm8350, WM8350_GPIO_CONFIGURATION_I_O, 1i32.wrapping_shl(offset))
}

unsafe extern "C" fn wm8350_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm8350_gpio = gpiochip_get_data(chip);
    let wm8350 = (*wm8350_gpio).wm8350;
    let ret = wm8350_reg_read(wm8350, WM8350_GPIO_LEVEL);
    if ret < 0 { return ret; }
    if ret & 1i32.wrapping_shl(offset) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn wm8350_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let wm8350_gpio = gpiochip_get_data(chip);
    let wm8350 = (*wm8350_gpio).wm8350;
    if value != 0 {
        let _ = wm8350_set_bits(wm8350, WM8350_GPIO_LEVEL, 1i32.wrapping_shl(offset));
    } else {
        let _ = wm8350_clear_bits(wm8350, WM8350_GPIO_LEVEL, 1i32.wrapping_shl(offset));
    }
}

unsafe extern "C" fn wm8350_gpio_direction_out(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let wm8350_gpio = gpiochip_get_data(chip);
    let wm8350 = (*wm8350_gpio).wm8350;
    let ret = wm8350_clear_bits(wm8350, WM8350_GPIO_CONFIGURATION_I_O, 1i32.wrapping_shl(offset));
    if ret < 0 { return ret; }
    // Don't have an atomic direction/value setup
    if value != 0 { wm8350_set_bits(wm8350, WM8350_GPIO_LEVEL, 1i32.wrapping_shl(offset)) }
    else { wm8350_clear_bits(wm8350, WM8350_GPIO_LEVEL, 1i32.wrapping_shl(offset)) }
}

unsafe extern "C" fn wm8350_gpio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm8350_gpio = gpiochip_get_data(chip);
    let wm8350 = (*wm8350_gpio).wm8350;
    // WM8350 IRQ GPIO macro is supplied by the WM8350 headers.
    let irq_base = *(wm8350 as *mut i32);
    if irq_base == 0 { return -EINVAL; }
    irq_base + WM8350_IRQ_GPIO + offset as i32
}

static mut template_chip: gpio_chip = gpio_chip {
    label: b"wm8350\0".as_ptr(), owner: core::ptr::null(),
    direction_input: Some(wm8350_gpio_direction_in), get: Some(wm8350_gpio_get),
    direction_output: Some(wm8350_gpio_direction_out), set: Some(wm8350_gpio_set), to_irq: Some(wm8350_gpio_to_irq),
    can_sleep: true, ngpio: 0, parent: core::ptr::null_mut(), base: 0,
};

unsafe extern "C" fn wm8350_gpio_probe(pdev: *mut platform_device) -> i32 {
    let wm8350 = dev_get_drvdata((*pdev).dev.parent);
    let pdata = dev_get_platdata((*wm8350).dev);
    let wm8350_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<wm8350_gpio_data>(), GFP_KERNEL) as *mut wm8350_gpio_data;
    if wm8350_gpio.is_null() { return -ENOMEM; }
    (*wm8350_gpio).wm8350 = wm8350;
    (*wm8350_gpio).gpio_chip = template_chip;
    (*wm8350_gpio).gpio_chip.ngpio = 13;
    (*wm8350_gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*wm8350_gpio).gpio_chip.base = if !pdata.is_null() && (*pdata).gpio_base != 0 { (*pdata).gpio_base } else { -1 };
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*wm8350_gpio).gpio_chip, wm8350_gpio)
}

static mut wm8350_gpio_driver: platform_driver = platform_driver {
    driver: wm8350_driver_info { name: b"wm8350-gpio\0".as_ptr(), probe: Some(wm8350_gpio_probe) },
    probe: Some(wm8350_gpio_probe),
};

#[no_mangle]
pub unsafe extern "C" fn wm8350_gpio_init() -> i32 { platform_driver_register(&mut wm8350_gpio_driver) }

#[no_mangle]
pub unsafe extern "C" fn wm8350_gpio_exit() { platform_driver_unregister(&mut wm8350_gpio_driver); }

// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_DESCRIPTION("GPIO interface for WM8350 PMICs");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:wm8350-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
