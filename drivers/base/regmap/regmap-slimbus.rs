// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017, Linaro Ltd.

// Dependencies supplied by the Linux regmap, SLIMbus, module, and internal
// headers are referenced here but are not defined in this translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct slim_device {
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

type RegmapWrite = unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> c_int;
type RegmapRead = unsafe extern "C" fn(
    *mut c_void,
    *const c_void,
    usize,
    *mut c_void,
    usize,
) -> c_int;

#[repr(C)]
struct regmap_bus {
    write: Option<RegmapWrite>,
    read: Option<RegmapRead>,
    reg_format_endian_default: c_int,
    val_format_endian_default: c_int,
}

// REGMAP_ENDIAN_LITTLE
const REGMAP_ENDIAN_LITTLE: c_int = 1;
const ENOTSUPP: c_int = 524;

unsafe extern "C" {
    fn slim_write(
        sdev: *mut slim_device,
        reg: u16,
        count: usize,
        val: *const u8,
    ) -> c_int;
    fn slim_read(sdev: *mut slim_device, reg: u16, count: usize, val: *mut c_void) -> c_int;
    fn __regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        bus_context: *mut slim_device,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const c_char,
    ) -> *mut regmap;
    fn __devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        bus_context: *mut slim_device,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const c_char,
    ) -> *mut regmap;
}

unsafe extern "C" fn regmap_slimbus_write(
    context: *mut c_void,
    data: *const c_void,
    count: usize,
) -> c_int {
    let sdev = context as *mut slim_device;

    slim_write(
        sdev,
        *(data as *const u16),
        count.wrapping_sub(2),
        (data as *const u8).add(2),
    )
}

unsafe extern "C" fn regmap_slimbus_read(
    context: *mut c_void,
    reg: *const c_void,
    _reg_size: usize,
    val: *mut c_void,
    val_size: usize,
) -> c_int {
    let sdev = context as *mut slim_device;

    slim_read(sdev, *(reg as *const u16), val_size, val)
}

static REGMAP_SLIMBUS_BUS: regmap_bus = regmap_bus {
    write: Some(regmap_slimbus_write),
    read: Some(regmap_slimbus_read),
    reg_format_endian_default: REGMAP_ENDIAN_LITTLE,
    val_format_endian_default: REGMAP_ENDIAN_LITTLE,
};

unsafe fn regmap_get_slimbus(
    _slim: *mut slim_device,
    config: *const regmap_config,
) -> *const regmap_bus {
    if (*config).val_bits == 8 && (*config).reg_bits == 16 {
        &REGMAP_SLIMBUS_BUS
    } else {
        core::ptr::with_exposed_provenance::<regmap_bus>(ENOTSUPP as usize)
    }
}

#[no_mangle]
pub unsafe extern "C" fn __regmap_init_slimbus(
    slimbus: *mut slim_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const c_char,
) -> *mut regmap {
    let bus = regmap_get_slimbus(slimbus, config);

    if (bus as usize) <= usize::MAX - 4095 {
        return bus as *mut regmap;
    }

    __regmap_init(
        &mut (*slimbus).dev,
        bus,
        slimbus,
        config,
        lock_key,
        lock_name,
    )
}

#[no_mangle]
pub unsafe extern "C" fn __devm_regmap_init_slimbus(
    slimbus: *mut slim_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const c_char,
) -> *mut regmap {
    let bus = regmap_get_slimbus(slimbus, config);

    if (bus as usize) <= usize::MAX - 4095 {
        return bus as *mut regmap;
    }

    __devm_regmap_init(
        &mut (*slimbus).dev,
        bus,
        slimbus,
        config,
        lock_key,
        lock_name,
    )
}

// EXPORT_SYMBOL_GPL(__regmap_init_slimbus);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_slimbus);
// MODULE_DESCRIPTION("Register map access API - SLIMbus support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
