// SPDX-License-Identifier: GPL-2.0
// Register map access API - SCCB support

// External Linux kernel declarations supplied by other translation units.

/**
 * sccb_is_available - Check if the adapter supports SCCB protocol
 * @adap: I2C adapter
 *
 * Return true if the I2C adapter is capable of using SCCB helper functions,
 * false otherwise.
 */
unsafe fn sccb_is_available(adap: *mut i2c_adapter) -> bool {
    let needed_funcs: u32 = I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_WRITE_BYTE_DATA;

    /*
     * If we ever want support for hardware doing SCCB natively, we will
     * introduce a sccb_xfer() callback to struct i2c_algorithm and check
     * for it here.
     */
    (i2c_get_functionality(adap) & needed_funcs) == needed_funcs
}

/**
 * regmap_sccb_read - Read data from SCCB slave device
 * @context: Device that will be interacted with
 * @reg: Register to be read from
 * @val: Pointer to store read value
 *
 * This executes the 2-phase write transmission cycle that is followed by a
 * 2-phase read transmission cycle, returning negative errno else zero on
 * success.
 */
unsafe fn regmap_sccb_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let dev = context as *mut device;
    let i2c = to_i2c_client(dev);
    let mut ret: i32;
    let mut data: i2c_smbus_data;

    i2c_lock_bus((*i2c).adapter, I2C_LOCK_SEGMENT);

    ret = __i2c_smbus_xfer(
        (*i2c).adapter,
        (*i2c).addr,
        (*i2c).flags,
        I2C_SMBUS_WRITE,
        reg,
        I2C_SMBUS_BYTE,
        core::ptr::null_mut(),
    );
    if ret < 0 {
        i2c_unlock_bus((*i2c).adapter, I2C_LOCK_SEGMENT);
        return ret;
    }

    ret = __i2c_smbus_xfer(
        (*i2c).adapter,
        (*i2c).addr,
        (*i2c).flags,
        I2C_SMBUS_READ,
        0,
        I2C_SMBUS_BYTE,
        &mut data,
    );
    if ret < 0 {
        i2c_unlock_bus((*i2c).adapter, I2C_LOCK_SEGMENT);
        return ret;
    }

    (*val) = data.byte;
    i2c_unlock_bus((*i2c).adapter, I2C_LOCK_SEGMENT);
    ret
}

/**
 * regmap_sccb_write - Write data to SCCB slave device
 * @context: Device that will be interacted with
 * @reg: Register to write to
 * @val: Value to be written
 *
 * This executes the SCCB 3-phase write transmission cycle, returning negative
 * errno else zero on success.
 */
unsafe fn regmap_sccb_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let dev = context as *mut device;
    let i2c = to_i2c_client(dev);
    i2c_smbus_write_byte_data(i2c, reg, val)
}

static regmap_sccb_bus: regmap_bus = regmap_bus {
    .reg_write: Some(regmap_sccb_write),
    .reg_read: Some(regmap_sccb_read),
};

unsafe fn regmap_get_sccb_bus(
    i2c: *mut i2c_client,
    config: *const regmap_config,
) -> *const regmap_bus {
    if (*config).val_bits == 8
        && (*config).reg_bits == 8
        && sccb_is_available((*i2c).adapter)
    {
        &regmap_sccb_bus
    } else {
        ERR_PTR(-ENOTSUPP)
    }
}

pub unsafe fn __regmap_init_sccb(
    i2c: *mut i2c_client,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let bus = regmap_get_sccb_bus(i2c, config);

    if IS_ERR(bus) {
        return ERR_CAST(bus);
    }

    __regmap_init(&mut (*i2c).dev, bus, &mut (*i2c).dev, config, lock_key, lock_name)
}

pub unsafe fn __devm_regmap_init_sccb(
    i2c: *mut i2c_client,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let bus = regmap_get_sccb_bus(i2c, config);

    if IS_ERR(bus) {
        return ERR_CAST(bus);
    }

    __devm_regmap_init(&mut (*i2c).dev, bus, &mut (*i2c).dev, config, lock_key, lock_name)
}

// EXPORT_SYMBOL_GPL(__regmap_init_sccb);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_sccb);
// MODULE_DESCRIPTION("Register map access API - SCCB support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
