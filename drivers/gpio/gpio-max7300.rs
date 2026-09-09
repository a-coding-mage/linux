// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009 Wolfram Sang, Pengutronix
 *
 * Check max730x.c for further details.
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub adapter: *mut i2c_adapter,
    _private: [u8; 0],
}

#[repr(C)]
pub struct max7301 {
    pub read: Option<unsafe extern "C" fn(*mut device, u32) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut device, u32, u32) -> c_int>,
    pub dev: *mut device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, reg: u32, val: u32) -> c_int;
    fn i2c_smbus_read_byte_data(client: *mut i2c_client, reg: u32) -> c_int;
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: u64) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn __max730x_probe(ts: *mut max7301) -> c_int;
    fn __max730x_remove(dev: *mut device);
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
}

const I2C_FUNC_SMBUS_BYTE_DATA: u64 = 1 << 2;
const GFP_KERNEL: u32 = 0;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;

unsafe extern "C" fn max7300_i2c_write(
    dev: *mut device,
    reg: u32,
    val: u32,
) -> c_int {
    let client: *mut i2c_client = to_i2c_client(dev);

    i2c_smbus_write_byte_data(client, reg, val)
}

unsafe extern "C" fn max7300_i2c_read(dev: *mut device, reg: u32) -> c_int {
    let client: *mut i2c_client = to_i2c_client(dev);

    i2c_smbus_read_byte_data(client, reg)
}

unsafe extern "C" fn max7300_probe(client: *mut i2c_client) -> c_int {
    let mut ts: *mut max7301;

    if !i2c_check_functionality((*client).adapter, I2C_FUNC_SMBUS_BYTE_DATA) {
        return -EIO;
    }

    ts = devm_kzalloc(
        &mut (*client).dev,
        core::mem::size_of::<max7301>(),
        GFP_KERNEL,
    ) as *mut max7301;
    if ts.is_null() {
        return -ENOMEM;
    }

    (*ts).read = Some(max7300_i2c_read);
    (*ts).write = Some(max7300_i2c_write);
    (*ts).dev = &mut (*client).dev;

    __max730x_probe(ts)
}

unsafe extern "C" fn max7300_remove(client: *mut i2c_client) {
    __max730x_remove(&mut (*client).dev);
}

static mut max7300_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"max7300\0\0\0\0\0\0\0\0\0\0\0\0\0",
        _private: [],
    },
    i2c_device_id {
        name: [0; 20],
        _private: [],
    },
];

static mut max7300_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"max7300\0".as_ptr() as *const c_char,
    },
    probe: Some(max7300_probe),
    remove: Some(max7300_remove),
    id_table: unsafe { max7300_id.as_ptr() },
};

unsafe extern "C" fn max7300_init() -> c_int {
    i2c_add_driver(&raw mut max7300_driver)
}

unsafe extern "C" fn max7300_exit() {
    i2c_del_driver(&raw mut max7300_driver);
}

// MODULE_DEVICE_TABLE(i2c, max7300_id);
// subsys_initcall(max7300_init);
// module_exit(max7300_exit);
// MODULE_AUTHOR("Wolfram Sang");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("MAX7300 GPIO-Expander");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
