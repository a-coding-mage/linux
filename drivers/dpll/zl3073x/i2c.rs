// SPDX-License-Identifier: GPL-2.0-only

// Translated from the Linux kernel I2C driver implementation.  The kernel
// headers and symbols referenced below are supplied by the surrounding build.

use crate::core::{
    zl3073x_dev, zl3073x_devm_alloc, zl3073x_dev_probe, zl3073x_regmap_config,
};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const u8,
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> i32>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const core::ffi::c_void,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
    fn is_err<T>(ptr: *const T) -> bool;
    fn ptr_err<T>(ptr: *const T) -> i32;
}

unsafe extern "C" fn zl3073x_i2c_probe(client: *mut i2c_client) -> i32 {
    let dev: *mut device = unsafe { &mut (*client).dev };
    let zldev: *mut zl3073x_dev = unsafe { zl3073x_devm_alloc(dev) };
    if unsafe { is_err(zldev) } {
        return unsafe { ptr_err(zldev) };
    }

    unsafe {
        (*zldev).regmap = devm_regmap_init_i2c(
            client,
            &zl3073x_regmap_config as *const _ as *const core::ffi::c_void,
        );
    }
    if unsafe { is_err((*zldev).regmap) } {
        return unsafe {
            dev_err_probe(
                dev,
                ptr_err((*zldev).regmap),
                b"Failed to initialize regmap\0".as_ptr(),
            )
        };
    }

    unsafe { zl3073x_dev_probe(zldev) }
}

static ZL3073X_I2C_ID: [i2c_device_id; 6] = [
    i2c_device_id { name: b"zl30731\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: b"zl30732\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: b"zl30733\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: b"zl30734\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: b"zl30735\0".as_ptr(), driver_data: 0 },
    i2c_device_id { name: core::ptr::null(), driver_data: 0 },
];

// MODULE_DEVICE_TABLE(i2c, zl3073x_i2c_id);

static ZL3073X_I2C_OF_MATCH: [of_device_id; 6] = [
    of_device_id { compatible: b"microchip,zl30731\0".as_ptr() },
    of_device_id { compatible: b"microchip,zl30732\0".as_ptr() },
    of_device_id { compatible: b"microchip,zl30733\0".as_ptr() },
    of_device_id { compatible: b"microchip,zl30734\0".as_ptr() },
    of_device_id { compatible: b"microchip,zl30735\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, zl3073x_i2c_of_match);

static mut ZL3073X_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"zl3073x-i2c\0".as_ptr(),
        of_match_table: ZL3073X_I2C_OF_MATCH.as_ptr(),
    },
    probe: Some(zl3073x_i2c_probe),
    id_table: ZL3073X_I2C_ID.as_ptr(),
};

// module_i2c_driver(zl3073x_i2c_driver);
// MODULE_AUTHOR("Ivan Vecera <ivecera@redhat.com>");
// MODULE_DESCRIPTION("Microchip ZL3073x I2C driver");
// MODULE_IMPORT_NS("ZL3073X");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
