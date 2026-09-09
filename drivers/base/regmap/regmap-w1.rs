// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - W1 (1-Wire) support
//
// Copyright (c) 2017 Radioavionica Corporation
// Author: Alex A. Mihaylov <minimumlaw@rambler.ru>

// External kernel dependencies supplied by other translation units.

const W1_CMD_READ_DATA: u8 = 0x69;
const W1_CMD_WRITE_DATA: u8 = 0x6C;

/*
 * 1-Wire slaves registers with addess 8 bit and data 8 bit
 */

unsafe fn w1_reg_a8_v8_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 255 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_READ_DATA);
        w1_write_8((*sl).master, reg as u8);
        *val = w1_read_8((*sl).master) as u32;
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

unsafe fn w1_reg_a8_v8_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 255 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_WRITE_DATA);
        w1_write_8((*sl).master, reg as u8);
        w1_write_8((*sl).master, val as u8);
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

/*
 * 1-Wire slaves registers with addess 8 bit and data 16 bit
 */

unsafe fn w1_reg_a8_v16_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 255 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_READ_DATA);
        w1_write_8((*sl).master, reg as u8);
        *val = w1_read_8((*sl).master) as u32;
        *val |= (w1_read_8((*sl).master) as u32) << 8;
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

unsafe fn w1_reg_a8_v16_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 255 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_WRITE_DATA);
        w1_write_8((*sl).master, reg as u8);
        w1_write_8((*sl).master, (val & 0x00FF) as u8);
        w1_write_8((*sl).master, ((val >> 8) & 0x00FF) as u8);
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

/*
 * 1-Wire slaves registers with addess 16 bit and data 16 bit
 */

unsafe fn w1_reg_a16_v16_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 65535 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_READ_DATA);
        w1_write_8((*sl).master, (reg & 0x00FF) as u8);
        w1_write_8((*sl).master, ((reg >> 8) & 0x00FF) as u8);
        *val = w1_read_8((*sl).master) as u32;
        *val |= (w1_read_8((*sl).master) as u32) << 8;
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

unsafe fn w1_reg_a16_v16_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let dev = context as *mut device;
    let sl = container_of!(dev, w1_slave, dev);
    let mut ret: i32 = 0;

    if reg > 65535 {
        return -EINVAL;
    }

    mutex_lock(&mut (*(*sl).master).bus_mutex);
    if w1_reset_select_slave(sl) == 0 {
        w1_write_8((*sl).master, W1_CMD_WRITE_DATA);
        w1_write_8((*sl).master, (reg & 0x00FF) as u8);
        w1_write_8((*sl).master, ((reg >> 8) & 0x00FF) as u8);
        w1_write_8((*sl).master, (val & 0x00FF) as u8);
        w1_write_8((*sl).master, ((val >> 8) & 0x00FF) as u8);
    } else {
        ret = -ENODEV;
    }
    mutex_unlock(&mut (*(*sl).master).bus_mutex);

    ret
}

/* Various types of supported bus addressing */

static regmap_bus regmap_w1_bus_a8_v8 = regmap_bus {
    reg_read: Some(w1_reg_a8_v8_read),
    reg_write: Some(w1_reg_a8_v8_write),
};

static regmap_bus regmap_w1_bus_a8_v16 = regmap_bus {
    reg_read: Some(w1_reg_a8_v16_read),
    reg_write: Some(w1_reg_a8_v16_write),
};

static regmap_bus regmap_w1_bus_a16_v16 = regmap_bus {
    reg_read: Some(w1_reg_a16_v16_read),
    reg_write: Some(w1_reg_a16_v16_write),
};

unsafe fn regmap_get_w1_bus(w1_dev: *mut device, config: *const regmap_config) -> *const regmap_bus {
    let _ = w1_dev;
    if (*config).reg_bits == 8 && (*config).val_bits == 8 {
        return &raw const regmap_w1_bus_a8_v8;
    }
    if (*config).reg_bits == 8 && (*config).val_bits == 16 {
        return &raw const regmap_w1_bus_a8_v16;
    }
    if (*config).reg_bits == 16 && (*config).val_bits == 16 {
        return &raw const regmap_w1_bus_a16_v16;
    }
    ERR_PTR(-ENOTSUPP)
}

unsafe fn __regmap_init_w1(
    w1_dev: *mut device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let bus = regmap_get_w1_bus(w1_dev, config);
    if IS_ERR(bus) {
        return ERR_CAST(bus);
    }
    __regmap_init(w1_dev, bus, w1_dev as *mut core::ffi::c_void, config, lock_key, lock_name)
}

unsafe fn __devm_regmap_init_w1(
    w1_dev: *mut device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const core::ffi::c_char,
) -> *mut regmap {
    let bus = regmap_get_w1_bus(w1_dev, config);
    if IS_ERR(bus) {
        return ERR_CAST(bus);
    }
    __devm_regmap_init(w1_dev, bus, w1_dev as *mut core::ffi::c_void, config, lock_key, lock_name)
}

// EXPORT_SYMBOL_GPL(__regmap_init_w1);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_w1);
// MODULE_DESCRIPTION("Register map access API - W1 (1-Wire) support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
