// SPDX-License-Identifier: GPL-2.0+
//
// Gateworks I2C PLD GPIO expander
//
// Copyright (C) 2019 Linus Walleij <linus.walleij@linaro.org>
//
// Based on code and know-how from the OpenWrt driver:
// Copyright (C) 2009 Gateworks Corporation
// Authors: Chris Lang, Imre Kaloz

// C dependencies: linux/bits.h, linux/kernel.h, linux/slab.h,
// linux/gpio/driver.h, linux/i2c.h, linux/module.h

#[repr(C)]
pub struct gw_pld {
    pub chip: gpio_chip,
    pub client: *mut i2c_client,
    pub out: u8,
}

extern "C" {
    pub static THIS_MODULE: *mut module;
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn i2c_smbus_write_byte(client: *mut i2c_client, value: u8) -> i32;
    fn i2c_smbus_read_byte(client: *mut i2c_client) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut core::ffi::c_void);
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

// External kernel types and constants supplied by the surrounding translation.
#[repr(C)] pub struct gpio_chip { pub base: i32, pub can_sleep: bool, pub parent: *mut device, pub owner: *mut module, pub label: *const core::ffi::c_char, pub ngpio: u32, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)> }
#[repr(C)] pub struct i2c_client { pub dev: device, pub flags: u16 }
#[repr(C)] pub struct device;
#[repr(C)] pub struct module;
#[repr(C)] pub struct i2c_device_id;
#[repr(C)] pub struct of_device_id;
#[repr(C)] pub struct i2c_driver;

const GFP_KERNEL: u32 = 0;
const I2C_M_IGNORE_NAK: u16 = 0;

unsafe extern "C" fn gw_pld_input8(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gw = gpiochip_get_data(gc) as *mut gw_pld;
    (*gw).out |= 1u8.wrapping_shl(offset);
    i2c_smbus_write_byte((*gw).client, (*gw).out)
}

unsafe extern "C" fn gw_pld_get8(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gw = gpiochip_get_data(gc) as *mut gw_pld;
    let val = i2c_smbus_read_byte((*gw).client);
    if val < 0 { 0 } else { ((val & (1i32 << offset)) != 0) as i32 }
}

unsafe extern "C" fn gw_pld_output8(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gw = gpiochip_get_data(gc) as *mut gw_pld;
    if value != 0 { (*gw).out |= 1u8.wrapping_shl(offset); }
    else { (*gw).out &= !(1u8.wrapping_shl(offset)); }
    i2c_smbus_write_byte((*gw).client, (*gw).out)
}

unsafe extern "C" fn gw_pld_set8(gc: *mut gpio_chip, offset: u32, value: i32) {
    let _ = gw_pld_output8(gc, offset, value);
}

unsafe extern "C" fn gw_pld_probe(client: *mut i2c_client) -> i32 {
    let dev = &mut (*client).dev as *mut device;
    let gw = devm_kzalloc(dev, core::mem::size_of::<gw_pld>(), GFP_KERNEL) as *mut gw_pld;
    if gw.is_null() { return -12; }

    (*gw).chip.base = -1;
    (*gw).chip.can_sleep = true;
    (*gw).chip.parent = dev;
    (*gw).chip.owner = THIS_MODULE;
    (*gw).chip.label = dev_name(dev);
    (*gw).chip.ngpio = 8;
    (*gw).chip.direction_input = Some(gw_pld_input8);
    (*gw).chip.get = Some(gw_pld_get8);
    (*gw).chip.direction_output = Some(gw_pld_output8);
    (*gw).chip.set = Some(gw_pld_set8);
    (*gw).client = client;

    (*client).flags |= I2C_M_IGNORE_NAK;
    (*gw).out = 0xFF;
    i2c_set_clientdata(client, gw as *mut core::ffi::c_void);
    let ret = devm_gpiochip_add_data(dev, &mut (*gw).chip, gw as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    dev_info(dev, b"registered Gateworks PLD GPIO device\0".as_ptr() as *const core::ffi::c_char);
    0
}

// The following driver tables and module registration are supplied as C-compatible
// declarations; their exact kernel helper expansion is build-time dependent.
#[no_mangle] pub static gw_pld_id: [i2c_device_id; 2] = unsafe { core::mem::zeroed() };
#[no_mangle] pub static gw_pld_dt_ids: [of_device_id; 2] = unsafe { core::mem::zeroed() };
#[no_mangle] pub static mut gw_pld_driver: i2c_driver = unsafe { core::mem::zeroed() };

// MODULE_DEVICE_TABLE(i2c, gw_pld_id);
// MODULE_DEVICE_TABLE(of, gw_pld_dt_ids);
// module_i2c_driver(gw_pld_driver);
// MODULE_DESCRIPTION("Gateworks I2C PLD GPIO expander");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
