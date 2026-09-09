// SPDX-License-Identifier: GPL-2.0+
/*
 * gpiolib support for Wolfson WM8994
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct wm8994_gpio {
    wm8994: *mut wm8994,
    gpio_chip: gpio_chip,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn wm8994_set_bits(wm8994: *mut wm8994, reg: u32, mask: u32, val: u32) -> i32;
    fn wm8994_reg_read(wm8994: *mut wm8994, reg: u32) -> i32;
    fn pinconf_to_config_param(config: u64) -> u32;
    fn regmap_irq_get_virq(data: *mut core::ffi::c_void, offset: u32) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut wm8994_pdata;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)] struct wm8994 { pub r#type: u32, pub irq_data: *mut core::ffi::c_void, pub dev: *mut device }
#[repr(C)] struct wm8994_pdata { pub gpio_base: i32 }
#[repr(C)] struct device;
#[repr(C)] struct platform_device { pub dev: device }
#[repr(C)] struct seq_file;
#[repr(C)] struct gpio_chip {
    pub label: *const u8, pub owner: *mut core::ffi::c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, u32, u64) -> i32>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub dbg_show: Option<unsafe extern "C" fn(*mut seq_file, *mut gpio_chip)>,
    pub can_sleep: bool, pub ngpio: u32, pub parent: *mut device, pub base: i32,
}
#[repr(C)] struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
#[repr(C)] struct driver { pub name: *const u8 }

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENOTSUPP: i32 = 524;
const WM8958: u32 = 1;
const WM8994_GPIO_1: u32 = 0;
const WM8994_GPIO_MAX: u32 = 12;
const WM8994_GPN_DIR: u32 = 1;
const WM8994_GPN_LVL: u32 = 2;
const WM8994_GPN_PU: u32 = 4;
const WM8994_GPN_PD: u32 = 8;
const WM8994_GPN_POL: u32 = 16;
const WM8994_GPN_OP_CFG: u32 = 32;
const WM8994_GPN_OP_CFG_MASK: u32 = 32;
const WM8994_GPN_FN_MASK: u32 = 0xff;
const PIN_CONFIG_DRIVE_OPEN_DRAIN: u32 = 1;
const PIN_CONFIG_DRIVE_PUSH_PULL: u32 = 2;

unsafe extern "C" fn wm8994_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut wm8994_gpio;
    let wm = (*gpio).wm8994;
    if (*wm).r#type == WM8958 && matches!(offset, 1 | 2 | 3 | 4 | 6) { return -EINVAL; }
    0
}

unsafe extern "C" fn wm8994_gpio_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    wm8994_set_bits(wm, WM8994_GPIO_1 + offset, WM8994_GPN_DIR, WM8994_GPN_DIR)
}

unsafe extern "C" fn wm8994_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    let ret = wm8994_reg_read(wm, WM8994_GPIO_1 + offset);
    if ret < 0 { ret } else if (ret as u32 & WM8994_GPN_LVL) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn wm8994_gpio_direction_out(chip: *mut gpio_chip, offset: u32, mut value: i32) -> i32 {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    if value != 0 { value = WM8994_GPN_LVL as i32; }
    wm8994_set_bits(wm, WM8994_GPIO_1 + offset, WM8994_GPN_DIR | WM8994_GPN_LVL, value as u32)
}

unsafe extern "C" fn wm8994_gpio_set(chip: *mut gpio_chip, offset: u32, mut value: i32) {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    if value != 0 { value = WM8994_GPN_LVL as i32; }
    let _ = wm8994_set_bits(wm, WM8994_GPIO_1 + offset, WM8994_GPN_LVL, value as u32);
}

unsafe extern "C" fn wm8994_gpio_set_config(chip: *mut gpio_chip, offset: u32, config: u64) -> i32 {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => wm8994_set_bits(wm, WM8994_GPIO_1 + offset, WM8994_GPN_OP_CFG_MASK, WM8994_GPN_OP_CFG),
        PIN_CONFIG_DRIVE_PUSH_PULL => wm8994_set_bits(wm, WM8994_GPIO_1 + offset, WM8994_GPN_OP_CFG_MASK, 0),
        _ => -ENOTSUPP,
    }
}

unsafe extern "C" fn wm8994_gpio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    let wm = (*(gpiochip_get_data(chip) as *mut wm8994_gpio)).wm8994;
    regmap_irq_get_virq((*wm).irq_data, offset)
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn wm8994_gpio_fn(fn_: u16) -> &'static str {
    match fn_ as u32 {
        0 => "pin-specific", 1 => "GPIO", 2 => "SDOUT", 3 => "IRQ", 4 => "Temperature",
        5 => "MICBIAS1 detect", 6 => "MICBIAS1 short", 7 => "MICBIAS2 detect", 8 => "MICBIAS2 short",
        9 => "FLL1 lock", 10 => "FLL2 lock", 11 => "SRC1 lock", 12 => "SRC2 lock",
        13 => "DRC1 activity", 14 => "DRC2 activity", 15 => "DRC3 activity", 16 => "Write sequencer",
        17 => "FIFO error", 18 => "OPCLK", 19 => "Thermal warning", 20 => "DC servo",
        21 => "FLL1 output", 22 => "FLL1 output", _ => "Unknown",
    }
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn wm8994_gpio_dbg_show(_s: *mut seq_file, _chip: *mut gpio_chip) {
    // The source reports each GPIO's label, direction, pulls, polarity,
    // output configuration, function name, and register value through seq_file.
}

static mut TEMPLATE_CHIP: gpio_chip = gpio_chip {
    label: b"wm8994\0".as_ptr(), owner: core::ptr::null_mut(),
    request: Some(wm8994_gpio_request), direction_input: Some(wm8994_gpio_direction_in), get: Some(wm8994_gpio_get),
    direction_output: Some(wm8994_gpio_direction_out), set: Some(wm8994_gpio_set), set_config: Some(wm8994_gpio_set_config),
    to_irq: Some(wm8994_gpio_to_irq), dbg_show: None, can_sleep: true, ngpio: 0, parent: core::ptr::null_mut(), base: 0,
};

unsafe extern "C" fn wm8994_gpio_probe(pdev: *mut platform_device) -> i32 {
    let wm = dev_get_drvdata(&mut (*pdev).dev as *mut device) as *mut wm8994;
    let pdata = dev_get_platdata((*wm).dev);
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<wm8994_gpio>(), 0) as *mut wm8994_gpio;
    if gpio.is_null() { return -ENOMEM; }
    (*gpio).wm8994 = wm;
    (*gpio).gpio_chip = TEMPLATE_CHIP;
    (*gpio).gpio_chip.ngpio = WM8994_GPIO_MAX;
    (*gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*gpio).gpio_chip.base = if !pdata.is_null() && (*pdata).gpio_base != 0 { (*pdata).gpio_base } else { -1 };
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gpio_chip, gpio as *mut core::ffi::c_void)
}

static mut WM8994_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"wm8994-gpio\0".as_ptr() }, probe: Some(wm8994_gpio_probe),
};

unsafe extern "C" fn wm8994_gpio_init() -> i32 { platform_driver_register(&mut WM8994_GPIO_DRIVER) }
unsafe extern "C" fn wm8994_gpio_exit() { platform_driver_unregister(&mut WM8994_GPIO_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
