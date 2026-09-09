// SPDX-License-Identifier: GPL-2.0-only
/*
 * FXL6408 GPIO driver
 *
 * Copyright 2023 Toradex
 *
 * Author: Emanuele Ghidoli <emanuele.ghidoli@toradex.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

const FXL6408_REG_DEVICE_ID: u32 = 0x01;
const FXL6408_MF_FAIRCHILD: u32 = 0b101;
const FXL6408_MF_SHIFT: u32 = 5;

/* Bits set here indicate that the GPIO is an output. */
const FXL6408_REG_IO_DIR: u32 = 0x03;

/* Bits set here, when the corresponding bit of IO_DIR is set, drive
 * the output high instead of low. */
const FXL6408_REG_OUTPUT: u32 = 0x05;

/* Bits here make the output High-Z, instead of the OUTPUT value. */
const FXL6408_REG_OUTPUT_HIGH_Z: u32 = 0x07;

/* Returns the current status (1 = HIGH) of the input pins. */
const FXL6408_REG_INPUT_STATUS: u32 = 0x0f;

/*
 * Return the current interrupt status
 * This bit is HIGH if input GPIO != default state (register 09h).
 * The flag is cleared after being read (bit returns to 0).
 * The input must go back to default state and change again before this flag is raised again.
 */
const FXL6408_REG_INT_STS: u32 = 0x13;

const FXL6408_NGPIO: u32 = 8;

#[repr(C)]
pub struct regmap_range {
    pub range_min: u32,
    pub range_max: u32,
}

#[repr(C)]
pub struct regmap_access_table {
    pub yes_ranges: *const regmap_range,
    pub n_yes_ranges: usize,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
    pub volatile_table: *const regmap_access_table,
    pub cache_type: u32,
    pub num_reg_defaults_raw: u32,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)]
pub struct gpio_regmap_config {
    pub parent: *mut device,
    pub ngpio: u32,
    pub reg_dat_base: usize,
    pub reg_set_base: usize,
    pub reg_dir_out_base: usize,
    pub ngpio_per_reg: u32,
    pub regmap: *mut regmap,
}
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut i32) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8, ...) -> i32;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut regmap);
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn devm_gpio_regmap_register(dev: *mut device, config: *mut gpio_regmap_config) -> *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut regmap;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> i32;
}

static RD_RANGE: [regmap_range; 3] = [
    regmap_range { range_min: FXL6408_REG_DEVICE_ID, range_max: FXL6408_REG_DEVICE_ID },
    regmap_range { range_min: FXL6408_REG_IO_DIR, range_max: FXL6408_REG_OUTPUT },
    regmap_range { range_min: FXL6408_REG_INPUT_STATUS, range_max: FXL6408_REG_INPUT_STATUS },
];
static WR_RANGE: [regmap_range; 3] = [
    regmap_range { range_min: FXL6408_REG_DEVICE_ID, range_max: FXL6408_REG_DEVICE_ID },
    regmap_range { range_min: FXL6408_REG_IO_DIR, range_max: FXL6408_REG_OUTPUT },
    regmap_range { range_min: FXL6408_REG_OUTPUT_HIGH_Z, range_max: FXL6408_REG_OUTPUT_HIGH_Z },
];
static VOLATILE_RANGE: [regmap_range; 2] = [
    regmap_range { range_min: FXL6408_REG_DEVICE_ID, range_max: FXL6408_REG_DEVICE_ID },
    regmap_range { range_min: FXL6408_REG_INPUT_STATUS, range_max: FXL6408_REG_INPUT_STATUS },
];

static RD_TABLE: regmap_access_table = regmap_access_table { yes_ranges: RD_RANGE.as_ptr(), n_yes_ranges: RD_RANGE.len() };
static WR_TABLE: regmap_access_table = regmap_access_table { yes_ranges: WR_RANGE.as_ptr(), n_yes_ranges: WR_RANGE.len() };
static VOLATILE_TABLE: regmap_access_table = regmap_access_table { yes_ranges: VOLATILE_RANGE.as_ptr(), n_yes_ranges: VOLATILE_RANGE.len() };

static REGMAP: regmap_config = regmap_config {
    reg_bits: 8, val_bits: 8, max_register: FXL6408_REG_INT_STS,
    wr_table: &WR_TABLE, rd_table: &RD_TABLE, volatile_table: &VOLATILE_TABLE,
    cache_type: 0, num_reg_defaults_raw: FXL6408_REG_INT_STS + 1,
};

unsafe fn fxl6408_identify(dev: *mut device, map: *mut regmap) -> i32 {
    let mut val: i32 = 0;
    let ret = regmap_read(map, FXL6408_REG_DEVICE_ID, &mut val);
    if ret != 0 { return dev_err_probe(dev, ret, b"error reading DEVICE_ID\0".as_ptr()); }
    if ((val as u32) >> FXL6408_MF_SHIFT) != FXL6408_MF_FAIRCHILD {
        return dev_err_probe(dev, -19, b"invalid device id 0x%02x\0".as_ptr(), val);
    }
    0
}

unsafe fn fxl6408_resume(dev: *mut device) -> i32 {
    let map = dev_get_drvdata(dev);
    regcache_mark_dirty(map);
    regcache_sync(map)
}

unsafe fn fxl6408_probe(client: *mut i2c_client) -> i32 {
    let dev = &mut (*client).dev as *mut device;
    let mut gpio_config = gpio_regmap_config {
        parent: dev,
        ngpio: FXL6408_NGPIO,
        reg_dat_base: FXL6408_REG_INPUT_STATUS as usize,
        reg_set_base: FXL6408_REG_OUTPUT as usize,
        reg_dir_out_base: FXL6408_REG_IO_DIR as usize,
        ngpio_per_reg: FXL6408_NGPIO,
        regmap: core::ptr::null_mut(),
    };

    gpio_config.regmap = devm_regmap_init_i2c(client, &REGMAP);
    if gpio_config.regmap.is_null() {
        return dev_err_probe(dev, -12, b"failed to allocate register map\0".as_ptr());
    }

    let ret = fxl6408_identify(dev, gpio_config.regmap);
    if ret != 0 { return ret; }

    i2c_set_clientdata(client, gpio_config.regmap);

    /* Disable High-Z of outputs, so that our OUTPUT updates actually take effect. */
    let ret = regmap_write(gpio_config.regmap, FXL6408_REG_OUTPUT_HIGH_Z, 0);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"failed to write 'output high Z' register\0".as_ptr());
    }

    let result = devm_gpio_regmap_register(dev, &mut gpio_config);
    if result.is_null() { 0 } else { 0 }
}

// Device-tree, I2C device ID, PM, and module registration metadata:
// compatible = "fcs,fxl6408"; device name = "fxl6408";
// module_i2c_driver(fxl6408_driver);
// MODULE_AUTHOR("Emanuele Ghidoli <emanuele.ghidoli@toradex.com>");
// MODULE_DESCRIPTION("FXL6408 GPIO driver");
// MODULE_LICENSE("GPL");

// The remaining driver registration and PM metadata are provided by the kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
