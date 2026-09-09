// SPDX-License-Identifier: GPL-2.0

// Translated from the Linux kernel implementation. The referenced kernel
// types, functions, and constants are supplied by the surrounding bindings.

use core::ffi::{c_char, c_void};

const REGVAL_MASK: u32 = 0xffff;
const REGNUM_C22_MASK: u32 = 0x1f;
// Clause-45 mask includes the device type (5 bit) and actual register number (16 bit).
const REGNUM_C45_MASK: u32 = 0x1f_ffff;
const REGMAP_MDIO_C45_DEVAD_SHIFT: u32 = 16;
const REGMAP_MDIO_C45_REGNUM_MASK: u32 = 0xffff;
const ENXIO: i32 = 6;
const EOPNOTSUPP: i32 = 95;

#[repr(C)]
pub struct mdio_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub val_bits: u32,
}

#[repr(C)]
pub struct lock_class_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

type RegWrite = unsafe extern "C" fn(*mut c_void, u32, u32) -> i32;
type RegRead = unsafe extern "C" fn(*mut c_void, u32, *mut u32) -> i32;

#[repr(C)]
pub struct regmap_bus {
    pub reg_write: Option<RegWrite>,
    pub reg_read: Option<RegRead>,
}

unsafe extern "C" {
    fn mdiodev_read(mdio_dev: *mut mdio_device, reg: u32) -> i32;
    fn mdiodev_write(mdio_dev: *mut mdio_device, reg: u32, val: u32) -> i32;
    fn mdiodev_c45_read(mdio_dev: *mut mdio_device, devad: u32, reg: u32) -> i32;
    fn mdiodev_c45_write(
        mdio_dev: *mut mdio_device,
        devad: u32,
        reg: u32,
        val: u32,
    ) -> i32;
    fn __regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        context: *mut c_void,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const c_char,
    ) -> *mut regmap;
    fn __devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        context: *mut c_void,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const c_char,
    ) -> *mut regmap;
}

unsafe fn regmap_mdio_c22_read(context: *mut c_void, reg: u32, val: *mut u32) -> i32 {
    let mdio_dev = context as *mut mdio_device;

    if (reg & !REGNUM_C22_MASK) != 0 {
        return -ENXIO;
    }

    let ret = mdiodev_read(mdio_dev, reg);
    if ret < 0 {
        return ret;
    }

    *val = (ret as u32) & REGVAL_MASK;
    0
}

unsafe fn regmap_mdio_c22_write(context: *mut c_void, reg: u32, val: u32) -> i32 {
    let mdio_dev = context as *mut mdio_device;

    if (reg & !REGNUM_C22_MASK) != 0 {
        return -ENXIO;
    }

    mdiodev_write(mdio_dev, reg, val)
}

static REGMAP_MDIO_C22_BUS: regmap_bus = regmap_bus {
    reg_write: Some(regmap_mdio_c22_write),
    reg_read: Some(regmap_mdio_c22_read),
};

unsafe fn regmap_mdio_c45_read(context: *mut c_void, mut reg: u32, val: *mut u32) -> i32 {
    let mdio_dev = context as *mut mdio_device;

    if (reg & !REGNUM_C45_MASK) != 0 {
        return -ENXIO;
    }

    let devad = reg >> REGMAP_MDIO_C45_DEVAD_SHIFT;
    reg &= REGMAP_MDIO_C45_REGNUM_MASK;

    let ret = mdiodev_c45_read(mdio_dev, devad, reg);
    if ret < 0 {
        return ret;
    }

    *val = (ret as u32) & REGVAL_MASK;
    0
}

unsafe fn regmap_mdio_c45_write(context: *mut c_void, mut reg: u32, val: u32) -> i32 {
    let mdio_dev = context as *mut mdio_device;

    if (reg & !REGNUM_C45_MASK) != 0 {
        return -ENXIO;
    }

    let devad = reg >> REGMAP_MDIO_C45_DEVAD_SHIFT;
    reg &= REGMAP_MDIO_C45_REGNUM_MASK;

    mdiodev_c45_write(mdio_dev, devad, reg, val)
}

static REGMAP_MDIO_C45_BUS: regmap_bus = regmap_bus {
    reg_write: Some(regmap_mdio_c45_write),
    reg_read: Some(regmap_mdio_c45_read),
};

pub unsafe extern "C" fn __regmap_init_mdio(
    mdio_dev: *mut mdio_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const c_char,
) -> *mut regmap {
    let bus: *const regmap_bus;

    if (*config).reg_bits == 5 && (*config).val_bits == 16 {
        bus = &REGMAP_MDIO_C22_BUS;
    } else if (*config).reg_bits == 21 && (*config).val_bits == 16 {
        bus = &REGMAP_MDIO_C45_BUS;
    } else {
        return (-EOPNOTSUPP) as isize as *mut regmap;
    }

    __regmap_init(
        &mut (*mdio_dev).dev,
        bus,
        mdio_dev as *mut c_void,
        config,
        lock_key,
        lock_name,
    )
}

pub unsafe extern "C" fn __devm_regmap_init_mdio(
    mdio_dev: *mut mdio_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const c_char,
) -> *mut regmap {
    let bus: *const regmap_bus;

    if (*config).reg_bits == 5 && (*config).val_bits == 16 {
        bus = &REGMAP_MDIO_C22_BUS;
    } else if (*config).reg_bits == 21 && (*config).val_bits == 16 {
        bus = &REGMAP_MDIO_C45_BUS;
    } else {
        return (-EOPNOTSUPP) as isize as *mut regmap;
    }

    __devm_regmap_init(
        &mut (*mdio_dev).dev,
        bus,
        mdio_dev as *mut c_void,
        config,
        lock_key,
        lock_name,
    )
}

// EXPORT_SYMBOL_GPL(__regmap_init_mdio);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_mdio);
// MODULE_AUTHOR("Sander Vanheule <sander@svanheule.net>");
// MODULE_DESCRIPTION("regmap MDIO Module");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
