// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO support for Cirrus Logic Madera codecs
 *
 * Copyright (C) 2015-2018 Cirrus Logic
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut c_void,
    ) -> c_int;
    fn gpiochip_add_pin_range(
        chip: *mut gpio_chip,
        pctl_name: *const c_char,
        gpio_offset: c_uint,
        pin_offset: c_uint,
        npins: c_uint,
    ) -> c_int;
    fn gpiochip_generic_request(chip: *mut gpio_chip, offset: c_uint) -> c_int;
    fn gpiochip_generic_free(chip: *mut gpio_chip, offset: c_uint);
    fn gpiochip_generic_config(chip: *mut gpio_chip, offset: c_uint, config: c_uint) -> c_int;
}

#[repr(C)]
pub struct madera {
    pub regmap: *mut regmap,
    pub pdata: madera_pdata,
    pub r#type: c_uint,
}

#[repr(C)]
pub struct madera_pdata {
    pub gpio_base: c_int,
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
pub struct gpio_chip {
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint)>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int)>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_uint) -> c_int>,
    pub can_sleep: bool,
    pub parent: *mut device,
    pub ngpio: c_uint,
    pub base: c_int,
}

#[repr(C)]
pub struct madera_gpio {
    pub madera: *mut madera,
    pub gpio_chip: gpio_chip,
}

const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GPIO_LINE_DIRECTION_OUT: c_int = 0;

unsafe extern "C" fn madera_gpio_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let madera_gpio = gpiochip_get_data(chip) as *mut madera_gpio;
    let madera = (*madera_gpio).madera;
    let reg_offset = 2u32.wrapping_mul(offset);
    let mut val = 0u32;
    let ret = regmap_read((*madera).regmap, MADERA_GPIO1_CTRL_2.wrapping_add(reg_offset), &mut val);
    if ret < 0 { return ret; }
    if val & MADERA_GP1_DIR_MASK != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn madera_gpio_direction_in(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let madera_gpio = gpiochip_get_data(chip) as *mut madera_gpio;
    let madera = (*madera_gpio).madera;
    let reg_offset = 2u32.wrapping_mul(offset);
    regmap_update_bits((*madera).regmap, MADERA_GPIO1_CTRL_2.wrapping_add(reg_offset), MADERA_GP1_DIR_MASK, MADERA_GP1_DIR)
}

unsafe extern "C" fn madera_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let madera_gpio = gpiochip_get_data(chip) as *mut madera_gpio;
    let madera = (*madera_gpio).madera;
    let reg_offset = 2u32.wrapping_mul(offset);
    let mut val = 0u32;
    let ret = regmap_read((*madera).regmap, MADERA_GPIO1_CTRL_1.wrapping_add(reg_offset), &mut val);
    if ret < 0 { return ret; }
    if val & MADERA_GP1_LVL_MASK != 0 { 1 } else { 0 }
}

unsafe extern "C" fn madera_gpio_direction_out(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let madera_gpio = gpiochip_get_data(chip) as *mut madera_gpio;
    let madera = (*madera_gpio).madera;
    let reg_offset = 2u32.wrapping_mul(offset);
    let reg_val = if value != 0 { MADERA_GP1_LVL } else { 0 };
    let ret = regmap_update_bits((*madera).regmap, MADERA_GPIO1_CTRL_2.wrapping_add(reg_offset), MADERA_GP1_DIR_MASK, 0);
    if ret < 0 { return ret; }
    regmap_update_bits((*madera).regmap, MADERA_GPIO1_CTRL_1.wrapping_add(reg_offset), MADERA_GP1_LVL_MASK, reg_val)
}

unsafe extern "C" fn madera_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) {
    let madera_gpio = gpiochip_get_data(chip) as *mut madera_gpio;
    let madera = (*madera_gpio).madera;
    let reg_offset = 2u32.wrapping_mul(offset);
    let reg_val = if value != 0 { MADERA_GP1_LVL } else { 0 };
    let _ = regmap_update_bits((*madera).regmap, MADERA_GPIO1_CTRL_1.wrapping_add(reg_offset), MADERA_GP1_LVL_MASK, reg_val);
}

static MADERA_LABEL: &[u8] = b"madera\0";
static mut madera_gpio_chip: gpio_chip = gpio_chip {
    label: MADERA_LABEL.as_ptr() as *const c_char,
    owner: core::ptr::null_mut(),
    request: Some(gpiochip_generic_request),
    free: Some(gpiochip_generic_free),
    get_direction: Some(madera_gpio_get_direction),
    direction_input: Some(madera_gpio_direction_in),
    get: Some(madera_gpio_get),
    direction_output: Some(madera_gpio_direction_out),
    set: Some(madera_gpio_set),
    set_config: Some(gpiochip_generic_config),
    can_sleep: true,
    parent: core::ptr::null_mut(), ngpio: 0, base: 0,
};

// The remaining platform-driver registration and chip-variant constants are supplied by the kernel build environment.
extern "C" {
    static mut MADERA_GPIO1_CTRL_1: c_uint;
    static mut MADERA_GPIO1_CTRL_2: c_uint;
    static mut MADERA_GP1_DIR_MASK: c_uint;
    static mut MADERA_GP1_DIR: c_uint;
    static mut MADERA_GP1_LVL_MASK: c_uint;
    static mut MADERA_GP1_LVL: c_uint;
    static CS47L15: c_uint;
    static CS47L35: c_uint;
    static CS47L85: c_uint;
    static WM1840: c_uint;
    static CS47L90: c_uint;
    static CS47L91: c_uint;
    static CS42L92: c_uint;
    static CS47L92: c_uint;
    static CS47L93: c_uint;
    static CS47L15_NUM_GPIOS: c_uint;
    static CS47L35_NUM_GPIOS: c_uint;
    static CS47L85_NUM_GPIOS: c_uint;
    static CS47L90_NUM_GPIOS: c_uint;
    static CS47L92_NUM_GPIOS: c_uint;
}

unsafe extern "C" fn madera_gpio_probe(pdev: *mut platform_device) -> c_int {
    let madera = dev_get_drvdata((*pdev).dev.parent) as *mut madera;
    let pdata = &mut (*madera).pdata;
    let madera_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<madera_gpio>(), 0) as *mut madera_gpio;
    if madera_gpio.is_null() { return -12; }
    (*madera_gpio).madera = madera;
    (*madera_gpio).gpio_chip = madera_gpio_chip;
    (*madera_gpio).gpio_chip.parent = (*pdev).dev.parent;
    (*madera_gpio).gpio_chip.ngpio = if (*madera).r#type == CS47L15 {
        CS47L15_NUM_GPIOS
    } else if (*madera).r#type == CS47L35 {
        CS47L35_NUM_GPIOS
    } else if (*madera).r#type == CS47L85 || (*madera).r#type == WM1840 {
        CS47L85_NUM_GPIOS
    } else if (*madera).r#type == CS47L90 || (*madera).r#type == CS47L91 {
        CS47L90_NUM_GPIOS
    } else if (*madera).r#type == CS42L92 || (*madera).r#type == CS47L92 || (*madera).r#type == CS47L93 {
        CS47L92_NUM_GPIOS
    } else {
        return -22;
    };
    (*madera_gpio).gpio_chip.base = if pdata.gpio_base != 0 { pdata.gpio_base } else { -1 };
    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*madera_gpio).gpio_chip, madera_gpio as *mut c_void);
    if ret < 0 { return ret; }
    let name = b"madera-pinctrl\0";
    let ret = gpiochip_add_pin_range(&mut (*madera_gpio).gpio_chip, name.as_ptr() as *const c_char, 0, 0, (*madera_gpio).gpio_chip.ngpio);
    if ret != 0 { return ret; }
    0
}

// Equivalent of module_platform_driver(madera_gpio_driver).
// MODULE_SOFTDEP("pre: pinctrl-madera");
// MODULE_DESCRIPTION("GPIO interface for Madera codecs");
// MODULE_AUTHOR("Nariman Poushin <nariman@opensource.cirrus.com>");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:madera-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
