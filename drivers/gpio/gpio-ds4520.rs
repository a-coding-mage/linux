// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2023 Analog Devices, Inc.
 * Driver for the DS4520 I/O Expander
 */

// Dependencies supplied by the surrounding kernel Rust environment.

const DS4520_PULLUP0: u32 = 0xF0;
const DS4520_IO_CONTROL0: u32 = 0xF2;
const DS4520_IO_STATUS0: u32 = 0xF8;

static DS4520_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 8,
    val_bits: 8,
};

unsafe fn ds4520_gpio_probe(client: *mut I2cClient) -> i32 {
    let mut config = core::mem::zeroed::<GpioRegmapConfig>();
    let dev: *mut Device = &mut (*client).dev;
    let mut regmap: *mut Regmap;
    let mut base: u32 = 0;
    let ret: i32;

    ret = device_property_read_u32(dev, c"reg".as_ptr(), &mut base);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Missing 'reg' property.\n".as_ptr());
    }

    regmap = devm_regmap_init_i2c(client, &DS4520_REGMAP_CONFIG);
    if is_err(regmap) {
        return dev_err_probe(
            dev,
            ptr_err(regmap),
            c"Failed to allocate register map\n".as_ptr(),
        );
    }

    config.regmap = regmap;
    config.parent = dev;

    config.reg_dat_base = base.wrapping_add(DS4520_IO_STATUS0);
    config.reg_set_base = base.wrapping_add(DS4520_PULLUP0);
    config.reg_dir_out_base = base.wrapping_add(DS4520_IO_CONTROL0);

    ptr_err_or_zero(devm_gpio_regmap_register(dev, &config))
}

static DS4520_GPIO_OF_MATCH_TABLE: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: c"adi,ds4520-gpio".as_ptr(),
    },
    OfDeviceId::default(),
];

// MODULE_DEVICE_TABLE(of, ds4520_gpio_of_match_table);

static DS4520_GPIO_ID_TABLE: [I2cDeviceId; 2] = [
    I2cDeviceId {
        name: c"ds4520-gpio".as_ptr(),
    },
    I2cDeviceId::default(),
];

// MODULE_DEVICE_TABLE(i2c, ds4520_gpio_id_table);

static mut DS4520_GPIO_DRIVER: I2cDriver = I2cDriver {
    driver: Driver {
        name: c"ds4520-gpio".as_ptr(),
        of_match_table: &DS4520_GPIO_OF_MATCH_TABLE,
    },
    probe: Some(ds4520_gpio_probe),
    id_table: &DS4520_GPIO_ID_TABLE,
};

// module_i2c_driver(ds4520_gpio_driver);

// MODULE_DESCRIPTION("DS4520 I/O Expander");
// MODULE_AUTHOR("Okan Sahin <okan.sahin@analog.com>");
// MODULE_LICENSE("GPL");

extern "C" {
    type Device;
    type GpioRegmapConfig;
    type I2cClient;
    type I2cDeviceId;
    type I2cDriver;
    type OfDeviceId;
    type Regmap;
    type RegmapConfig;

    fn device_property_read_u32(dev: *mut Device, propname: *const core::ffi::c_char, out_value: *mut u32) -> i32;
    fn dev_err_probe(dev: *mut Device, err: i32, fmt: *const core::ffi::c_char) -> i32;
    fn devm_regmap_init_i2c(client: *mut I2cClient, config: *const RegmapConfig) -> *mut Regmap;
    fn is_err(ptr: *mut Regmap) -> bool;
    fn ptr_err(ptr: *mut Regmap) -> i32;
    fn devm_gpio_regmap_register(dev: *mut Device, config: *const GpioRegmapConfig) -> *mut core::ffi::c_void;
    fn ptr_err_or_zero(ptr: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
