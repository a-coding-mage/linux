// SPDX-License-Identifier: GPL-2.0
/*
 * Digital I/O driver for Technologic Systems I2C FPGA Core
 *
 * Copyright (C) 2015, 2018 Technologic Systems
 * Copyright (C) 2016 Savoir-Faire Linux
 */

// C dependencies: linux/gpio/driver.h, linux/i2c.h, linux/module.h,
// linux/property.h, and linux/regmap.h.

const DEFAULT_PIN_NUMBER: u32 = 32;
/*
 * Register bits used by the GPIO device
 * Some boards, such as TS-7970 do not have a separate input bit
 */
const TS4900_GPIO_OE: u32 = 0x01;
const TS4900_GPIO_OUT: u32 = 0x02;
const TS4900_GPIO_IN: u32 = 0x04;
const TS7970_GPIO_IN: u32 = 0x02;

#[repr(C)]
struct ts4900_gpio_priv {
    regmap: *mut regmap,
    gpio_chip: gpio_chip,
    input_bit: u32,
}

#[repr(C)]
struct regmap_config {
    reg_bits: u32,
    val_bits: u32,
}

#[repr(C)]
struct gpio_chip {
    label: *const core::ffi::c_char,
    owner: *mut core::ffi::c_void,
    get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    base: i32,
    can_sleep: bool,
    ngpio: u32,
    parent: *mut device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct device;
#[repr(C)]
struct regmap;
#[repr(C)]
struct i2c_driver;
#[repr(C)]
struct i2c_device_id;

const GPIO_LINE_DIRECTION_IN: i32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut ts4900_gpio_priv;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn device_property_read_u32(dev: *mut device, name: *const core::ffi::c_char, val: *mut u32) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut core::ffi::c_void);
    fn ptr_err(ptr: *mut regmap) -> i32;
}

static ts4900_regmap_config: regmap_config = regmap_config { reg_bits: 16, val_bits: 8 };

unsafe extern "C" fn ts4900_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip);
    let mut reg = 0u32;
    regmap_read(priv_.regmap, offset, &mut reg);
    if reg & TS4900_GPIO_OE != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn ts4900_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip);
    /* Only clear the OE bit here, requires a RMW. Prevents a potential issue
     * with OE and DAT getting to the physical pin at different times. */
    regmap_update_bits(priv_.regmap, offset, TS4900_GPIO_OE, 0)
}

unsafe extern "C" fn ts4900_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip);
    let mut reg = 0u32;
    let ret: i32;
    /* If changing from an input to an output, first set DAT and then OE. */
    regmap_read(priv_.regmap, offset, &mut reg);
    if reg & TS4900_GPIO_OE == 0 {
        if value != 0 { reg = TS4900_GPIO_OUT; } else { reg &= !TS4900_GPIO_OUT; }
        regmap_write(priv_.regmap, offset, reg);
    }
    if value != 0 { ret = regmap_write(priv_.regmap, offset, TS4900_GPIO_OE | TS4900_GPIO_OUT); }
    else { ret = regmap_write(priv_.regmap, offset, TS4900_GPIO_OE); }
    ret
}

unsafe extern "C" fn ts4900_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip);
    let mut reg = 0u32;
    regmap_read(priv_.regmap, offset, &mut reg);
    if reg & priv_.input_bit != 0 { 1 } else { 0 }
}

unsafe extern "C" fn ts4900_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let priv_ = &mut *gpiochip_get_data(chip);
    if value != 0 { regmap_update_bits(priv_.regmap, offset, TS4900_GPIO_OUT, TS4900_GPIO_OUT); }
    else { regmap_update_bits(priv_.regmap, offset, TS4900_GPIO_OUT, 0); }
}

static template_chip: gpio_chip = gpio_chip {
    label: b"ts4900-gpio\0".as_ptr() as *const _,
    owner: core::ptr::null_mut(),
    get_direction: Some(ts4900_gpio_get_direction),
    direction_input: Some(ts4900_gpio_direction_input),
    direction_output: Some(ts4900_gpio_direction_output),
    get: Some(ts4900_gpio_get),
    set: Some(ts4900_gpio_set),
    base: -1,
    can_sleep: true,
    ngpio: 0,
    parent: core::ptr::null_mut(),
};

static ts4900_gpio_of_match_table: [of_device_id; 3] = [
    of_device_id { compatible: b"technologic,ts4900-gpio\0".as_ptr() as *const _, data: TS4900_GPIO_IN as *const _ },
    of_device_id { compatible: b"technologic,ts7970-gpio\0".as_ptr() as *const _, data: TS7970_GPIO_IN as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn ts4900_gpio_probe(client: *mut i2c_client) -> i32 {
    let mut ngpio: u32 = 0;
    if device_property_read_u32(&mut (*client).dev, b"ngpios\0".as_ptr() as *const _, &mut ngpio) != 0 {
        ngpio = DEFAULT_PIN_NUMBER;
    }
    let priv_ = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<ts4900_gpio_priv>(), 0) as *mut ts4900_gpio_priv;
    if priv_.is_null() { return -12; }
    (*priv_).gpio_chip = template_chip;
    (*priv_).gpio_chip.label = b"ts4900-gpio\0".as_ptr() as *const _;
    (*priv_).gpio_chip.ngpio = ngpio;
    (*priv_).gpio_chip.parent = &mut (*client).dev;
    (*priv_).input_bit = device_get_match_data(&mut (*client).dev) as usize as u32;
    (*priv_).regmap = devm_regmap_init_i2c(client, &ts4900_regmap_config);
    if (*priv_).regmap.is_null() { return ptr_err((*priv_).regmap); }
    let ret = devm_gpiochip_add_data(&mut (*client).dev, &mut (*priv_).gpio_chip, priv_ as *mut _);
    if ret < 0 { return ret; }
    i2c_set_clientdata(client, priv_ as *mut _);
    0
}

static ts4900_gpio_driver: *mut i2c_driver = core::ptr::null_mut();

#[repr(C)]
struct ts4900_gpio_id_entry {
    name: *const core::ffi::c_char,
}

static ts4900_gpio_id_table: [ts4900_gpio_id_entry; 2] = [
    ts4900_gpio_id_entry { name: b"ts4900-gpio\0".as_ptr() as *const _ },
    ts4900_gpio_id_entry { name: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, ts4900_gpio_of_match_table);
// MODULE_DEVICE_TABLE(i2c, ts4900_gpio_id_table);
// module_i2c_driver(ts4900_gpio_driver);
// MODULE_AUTHOR("Technologic Systems");
// MODULE_DESCRIPTION("GPIO interface for Technologic Systems I2C-FPGA core");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
