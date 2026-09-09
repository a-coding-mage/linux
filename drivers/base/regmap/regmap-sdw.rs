// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2015-17 Intel Corporation.

// Dependencies supplied by the surrounding kernel/regmap/SoundWire code.

unsafe fn regmap_sdw_write(context: *mut core::ffi::c_void,
                           val_buf: *const core::ffi::c_void,
                           val_size: usize) -> i32 {
    let dev = context as *mut device;
    let slave = unsafe { dev_to_sdw_dev(dev) };
    /* First word of buffer contains the destination address */
    let addr = u32::from_le(unsafe { *(val_buf as *const u32) });
    let val = val_buf as *const u8;

    unsafe {
        sdw_nwrite_no_pm(slave, addr, val_size - core::mem::size_of::<u32>(),
                         val.add(core::mem::size_of::<u32>()))
    }
}

unsafe fn regmap_sdw_gather_write(context: *mut core::ffi::c_void,
                                   reg_buf: *const core::ffi::c_void,
                                   _reg_size: usize,
                                   val_buf: *const core::ffi::c_void,
                                   val_size: usize) -> i32 {
    let dev = context as *mut device;
    let slave = unsafe { dev_to_sdw_dev(dev) };
    let addr = u32::from_le(unsafe { *(reg_buf as *const u32) });

    unsafe { sdw_nwrite_no_pm(slave, addr, val_size, val_buf as *const u8) }
}

unsafe fn regmap_sdw_read(context: *mut core::ffi::c_void,
                           reg_buf: *const core::ffi::c_void,
                           _reg_size: usize,
                           val_buf: *mut core::ffi::c_void,
                           val_size: usize) -> i32 {
    let dev = context as *mut device;
    let slave = unsafe { dev_to_sdw_dev(dev) };
    let addr = u32::from_le(unsafe { *(reg_buf as *const u32) });

    unsafe { sdw_nread_no_pm(slave, addr, val_size, val_buf as *mut u8) }
}

static regmap_sdw: regmap_bus = regmap_bus {
    write: Some(regmap_sdw_write),
    gather_write: Some(regmap_sdw_gather_write),
    read: Some(regmap_sdw_read),
    reg_format_endian_default: REGMAP_ENDIAN_LITTLE,
    val_format_endian_default: REGMAP_ENDIAN_LITTLE,
};

unsafe fn regmap_sdw_config_check(config: *const regmap_config) -> i32 {
    /* Register addresses are 32 bits wide */
    if unsafe { (*config).reg_bits } != 32 {
        return -ENOTSUPP;
    }

    if unsafe { (*config).pad_bits } != 0 {
        return -ENOTSUPP;
    }

    /* Only bulk writes are supported not multi-register writes */
    if unsafe { (*config).can_multi_write } {
        return -ENOTSUPP;
    }

    0
}

unsafe fn __regmap_init_sdw(sdw: *mut sdw_slave,
                            config: *const regmap_config,
                            lock_key: *mut lock_class_key,
                            lock_name: *const core::ffi::c_char) -> *mut regmap {
    let ret = unsafe { regmap_sdw_config_check(config) };
    if ret != 0 {
        return unsafe { ERR_PTR(ret) };
    }

    unsafe {
        __regmap_init(&mut (*sdw).dev, &regmap_sdw,
                      &mut (*sdw).dev, config, lock_key, lock_name)
    }
}

unsafe fn __devm_regmap_init_sdw(sdw: *mut sdw_slave,
                                 config: *const regmap_config,
                                 lock_key: *mut lock_class_key,
                                 lock_name: *const core::ffi::c_char) -> *mut regmap {
    let ret = unsafe { regmap_sdw_config_check(config) };
    if ret != 0 {
        return unsafe { ERR_PTR(ret) };
    }

    unsafe {
        __devm_regmap_init(&mut (*sdw).dev, &regmap_sdw,
                           &mut (*sdw).dev, config, lock_key, lock_name)
    }
}

// EXPORT_SYMBOL_GPL(__regmap_init_sdw);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_sdw);

// MODULE_DESCRIPTION("regmap SoundWire Module");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
