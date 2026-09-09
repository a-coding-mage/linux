// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO driver for Analog Devices ADP5520 MFD PMICs
 *
 * Copyright 2009 Analog Devices Inc.
 */

// Kernel dependencies supplied by the surrounding tree.

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: i32,
    pub name: *const u8,
}

#[repr(C)]
pub struct gpio_chip {
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub can_sleep: bool,
    pub base: i32,
    pub ngpio: u32,
    pub label: *const u8,
    pub owner: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct adp5520_gpio_platform_data {
    pub gpio_en_mask: u8,
    pub gpio_start: i32,
    pub gpio_pullup_mask: u8,
}

const ADP5520_MAXGPIOS: usize = 8;
const ID_ADP5520: i32 = 0;
const ADP5520_GPIO_OUT: u8 = 0;
const ADP5520_GPIO_IN: u8 = 0;
const ADP5520_GPIO_CFG_1: u8 = 0;
const ADP5520_GPIO_CFG_2: u8 = 0;
const ADP5520_GPIO_C3: u8 = 0;
const ADP5520_GPIO_R3: u8 = 0;
const ADP5520_C3_MODE: u8 = 0;
const ADP5520_R3_MODE: u8 = 0;
const ADP5520_LED_CONTROL: u8 = 0;
const ADP5520_GPIO_PULLUP: u8 = 0;

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn adp5520_read(master: *mut device, reg: u8, value: *mut u8) -> i32;
    fn adp5520_set_bits(master: *mut device, reg: u8, mask: u8) -> i32;
    fn adp5520_clr_bits(master: *mut device, reg: u8, mask: u8) -> i32;
    fn dev_get_platdata(dev: *mut device) -> *mut adp5520_gpio_platform_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn dev_err(dev: *mut device, message: *const u8);
}

#[repr(C)]
struct adp5520_gpio {
    master: *mut device,
    gpio_chip: gpio_chip,
    lut: [u8; ADP5520_MAXGPIOS],
    output: usize,
}

unsafe extern "C" fn adp5520_gpio_get_value(chip: *mut gpio_chip, off: u32) -> i32 {
    let dev = gpiochip_get_data(chip) as *mut adp5520_gpio;
    let mut reg_val: u8 = 0;

    /* There are dedicated registers for GPIO IN/OUT. */
    if ((*dev).output & (1usize << off)) != 0 {
        adp5520_read((*dev).master, ADP5520_GPIO_OUT, &mut reg_val);
    } else {
        adp5520_read((*dev).master, ADP5520_GPIO_IN, &mut reg_val);
    }

    if (reg_val & (*dev).lut[off as usize]) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn adp5520_gpio_set_value(chip: *mut gpio_chip, off: u32, val: i32) {
    let dev = gpiochip_get_data(chip) as *mut adp5520_gpio;
    if val != 0 {
        adp5520_set_bits((*dev).master, ADP5520_GPIO_OUT, (*dev).lut[off as usize]);
    } else {
        adp5520_clr_bits((*dev).master, ADP5520_GPIO_OUT, (*dev).lut[off as usize]);
    }
}

unsafe extern "C" fn adp5520_gpio_direction_input(chip: *mut gpio_chip, off: u32) -> i32 {
    let dev = gpiochip_get_data(chip) as *mut adp5520_gpio;
    (*dev).output &= !(1usize << off);
    adp5520_clr_bits((*dev).master, ADP5520_GPIO_CFG_2, (*dev).lut[off as usize])
}

unsafe extern "C" fn adp5520_gpio_direction_output(chip: *mut gpio_chip, off: u32, val: i32) -> i32 {
    let dev = gpiochip_get_data(chip) as *mut adp5520_gpio;
    let mut ret = 0;
    (*dev).output |= 1usize << off;
    if val != 0 {
        ret |= adp5520_set_bits((*dev).master, ADP5520_GPIO_OUT, (*dev).lut[off as usize]);
    } else {
        ret |= adp5520_clr_bits((*dev).master, ADP5520_GPIO_OUT, (*dev).lut[off as usize]);
    }
    ret |= adp5520_set_bits((*dev).master, ADP5520_GPIO_CFG_2, (*dev).lut[off as usize]);
    ret
}

unsafe extern "C" fn adp5520_gpio_probe(pdev: *mut platform_device) -> i32 {
    let pdata = dev_get_platdata(&mut (*pdev).dev);
    if pdata.is_null() { return -19; }
    if (*pdev).id != ID_ADP5520 { return -19; }

    let dev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<adp5520_gpio>(), 0) as *mut adp5520_gpio;
    if dev.is_null() { return -12; }
    (*dev).master = (*pdev).dev.parent;

    let mut gpios = 0usize;
    for i in 0..ADP5520_MAXGPIOS {
        if ((*pdata).gpio_en_mask & (1u8 << i)) != 0 {
            (*dev).lut[gpios] = 1u8 << i;
            gpios += 1;
        }
    }
    if gpios < 1 { return -22; }

    (*dev).gpio_chip.direction_input = Some(adp5520_gpio_direction_input);
    (*dev).gpio_chip.direction_output = Some(adp5520_gpio_direction_output);
    (*dev).gpio_chip.get = Some(adp5520_gpio_get_value);
    (*dev).gpio_chip.set = Some(adp5520_gpio_set_value);
    (*dev).gpio_chip.can_sleep = true;
    (*dev).gpio_chip.base = (*pdata).gpio_start;
    (*dev).gpio_chip.ngpio = gpios as u32;
    (*dev).gpio_chip.label = (*pdev).name;

    let mut ret = adp5520_clr_bits((*dev).master, ADP5520_GPIO_CFG_1, (*pdata).gpio_en_mask);
    let mut ctl_mask = 0u8;
    if ((*pdata).gpio_en_mask & ADP5520_GPIO_C3) != 0 { ctl_mask |= ADP5520_C3_MODE; }
    if ((*pdata).gpio_en_mask & ADP5520_GPIO_R3) != 0 { ctl_mask |= ADP5520_R3_MODE; }
    if ctl_mask != 0 { ret = adp5520_set_bits((*dev).master, ADP5520_LED_CONTROL, ctl_mask); }
    ret |= adp5520_set_bits((*dev).master, ADP5520_GPIO_PULLUP, (*pdata).gpio_pullup_mask);
    if ret != 0 { return ret; }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*dev).gpio_chip, dev as *mut core::ffi::c_void)
}

// module_platform_driver(adp5520_gpio_driver);
// MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
// MODULE_DESCRIPTION("GPIO ADP5520 Driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:adp5520-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
