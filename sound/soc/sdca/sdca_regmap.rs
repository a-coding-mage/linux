// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

// Dependencies from the original C includes:
// linux/bitops.h, linux/minmax.h, linux/module.h, linux/regmap.h,
// linux/soundwire/sdw_registers.h, linux/types.h, sound/sdca.h,
// sound/sdca_function.h, sound/sdca_regmap.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong};

pub type bool_ = bool;
pub type u8_ = u8;
pub type u32_ = u32;

pub const EINVAL: c_int = 22;
pub const BITS_PER_BYTE: c_int = 8;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_init_write {
    pub addr: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct sdca_control {
    pub sel: c_uint,
    pub cn_list: c_ulonglong,
    pub mode: c_int,
    pub layers: c_uint,
    pub is_volatile: bool,
    pub deferrable: bool,
    pub nbits: c_uint,
    pub has_reset: bool,
    pub values: *mut c_uint,
    pub reset: c_uint,
    pub has_default: bool,
    pub has_fixed: bool,
}

#[repr(C)]
pub struct sdca_entity {
    pub id: c_uint,
    pub num_controls: c_int,
    pub controls: *mut sdca_control,
}

#[repr(C)]
pub struct sdca_function_data {
    pub num_entities: c_int,
    pub entities: *mut sdca_entity,
    pub desc: *mut sdca_function_desc,
    pub init_table: *mut sdca_init_write,
    pub num_init_table: c_int,
}

unsafe extern "C" {
    static SDCA_ACCESS_MODE_RW: c_int;
    static SDCA_ACCESS_MODE_RO: c_int;
    static SDCA_ACCESS_MODE_RW1S: c_int;
    static SDCA_ACCESS_MODE_RW1C: c_int;
    static SDCA_ACCESS_MODE_DUAL: c_int;
    static SDCA_ACCESS_MODE_DC: c_int;
    static SDCA_ACCESS_LAYER_DEVICE: c_uint;

    fn SDW_SDCA_CTL_ENT(reg: c_uint) -> c_uint;
    fn SDW_SDCA_CTL_CSEL(reg: c_uint) -> c_uint;
    fn SDW_SDCA_VALID_CTL(reg: c_uint) -> bool;
    fn SDW_SDCA_CTL_CNUM(reg: c_uint) -> c_uint;
    fn SDW_SDCA_NEXT_CTL(reg: c_uint) -> c_uint;
    fn SDW_SDCA_CTL(adr: c_uint, entity: c_uint, control: c_uint, cn: c_int) -> c_uint;

    fn hweight64(w: c_ulonglong) -> c_uint;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_drop_region(map: *mut regmap, min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

#[inline]
fn BIT(nr: c_uint) -> c_ulonglong {
    1u64.wrapping_shl(nr) as c_ulonglong
}

#[inline]
fn clamp_val(val: c_uint, lo: usize, hi: usize) -> c_int {
    let val = val as usize;
    let ret = if val < lo {
        lo
    } else if val > hi {
        hi
    } else {
        val
    };

    ret as c_int
}

#[inline]
fn BITS_PER_TYPE<T>(_: T) -> c_uint {
    (core::mem::size_of::<T>() * BITS_PER_BYTE as usize) as c_uint
}

unsafe fn function_find_entity(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> *mut sdca_entity {
    let mut i: c_int;

    i = 0;
    while i < (*function).num_entities {
        if SDW_SDCA_CTL_ENT(reg) == (*(*function).entities.offset(i as isize)).id {
            return (*function).entities.offset(i as isize);
        }

        i += 1;
    }

    core::ptr::null_mut()
}

unsafe fn entity_find_control(entity: *mut sdca_entity, reg: c_uint) -> *mut sdca_control {
    let mut i: c_int;

    i = 0;
    while i < (*entity).num_controls {
        if SDW_SDCA_CTL_CSEL(reg) == (*(*entity).controls.offset(i as isize)).sel {
            return (*entity).controls.offset(i as isize);
        }

        i += 1;
    }

    core::ptr::null_mut()
}

unsafe fn function_find_control(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> *mut sdca_control {
    let entity: *mut sdca_entity;

    entity = function_find_entity(function, reg);
    if entity.is_null() {
        return core::ptr::null_mut();
    }

    entity_find_control(entity, reg)
}

/**
 * sdca_regmap_readable - return if a given SDCA Control is readable
 * @function: Pointer to the Function information.
 * @reg: Register address/Control to be processed.
 *
 * Return: Returns true if the register is readable.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_readable(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> bool {
    let control: *mut sdca_control;

    if !SDW_SDCA_VALID_CTL(reg) {
        return false;
    }

    control = function_find_control(function, reg);
    if control.is_null() {
        return false;
    }

    if (BIT(SDW_SDCA_CTL_CNUM(reg)) & (*control).cn_list) == 0 {
        return false;
    }

    if (*control).mode == SDCA_ACCESS_MODE_RW
        || (*control).mode == SDCA_ACCESS_MODE_RO
        || (*control).mode == SDCA_ACCESS_MODE_RW1S
        || (*control).mode == SDCA_ACCESS_MODE_RW1C
    {
        if (SDW_SDCA_NEXT_CTL(0) & reg) != 0 {
            return false;
        }
        /* fallthrough */
    } else if (*control).mode != SDCA_ACCESS_MODE_DUAL {
        return false;
    }

    /* No access to registers marked solely for device use */
    ((*control).layers & !SDCA_ACCESS_LAYER_DEVICE) != 0
}

// EXPORT_SYMBOL_NS(sdca_regmap_readable, "SND_SOC_SDCA");

/**
 * sdca_regmap_writeable - return if a given SDCA Control is writeable
 * @function: Pointer to the Function information.
 * @reg: Register address/Control to be processed.
 *
 * Return: Returns true if the register is writeable.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_writeable(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> bool {
    let control: *mut sdca_control;

    if !SDW_SDCA_VALID_CTL(reg) {
        return false;
    }

    control = function_find_control(function, reg);
    if control.is_null() {
        return false;
    }

    if (BIT(SDW_SDCA_CTL_CNUM(reg)) & (*control).cn_list) == 0 {
        return false;
    }

    if (*control).mode == SDCA_ACCESS_MODE_RW
        || (*control).mode == SDCA_ACCESS_MODE_RW1S
        || (*control).mode == SDCA_ACCESS_MODE_RW1C
    {
        if (SDW_SDCA_NEXT_CTL(0) & reg) != 0 {
            return false;
        }
        /* fallthrough */
    } else if (*control).mode != SDCA_ACCESS_MODE_DUAL {
        return false;
    }

    /* No access to registers marked solely for device use */
    ((*control).layers & !SDCA_ACCESS_LAYER_DEVICE) != 0
}

// EXPORT_SYMBOL_NS(sdca_regmap_writeable, "SND_SOC_SDCA");

/**
 * sdca_regmap_volatile - return if a given SDCA Control is volatile
 * @function: Pointer to the Function information.
 * @reg: Register address/Control to be processed.
 *
 * Return: Returns true if the register is volatile.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_volatile(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> bool {
    let control: *mut sdca_control;

    if !SDW_SDCA_VALID_CTL(reg) {
        return false;
    }

    control = function_find_control(function, reg);
    if control.is_null() {
        return false;
    }

    (*control).is_volatile
}

// EXPORT_SYMBOL_NS(sdca_regmap_volatile, "SND_SOC_SDCA");

/**
 * sdca_regmap_deferrable - return if a given SDCA Control is deferrable
 * @function: Pointer to the Function information.
 * @reg: Register address/Control to be processed.
 *
 * Return: Returns true if the register is deferrable.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_deferrable(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> bool {
    let control: *mut sdca_control;

    if !SDW_SDCA_VALID_CTL(reg) {
        return false;
    }

    control = function_find_control(function, reg);
    if control.is_null() {
        return false;
    }

    (*control).deferrable
}

// EXPORT_SYMBOL_NS(sdca_regmap_deferrable, "SND_SOC_SDCA");

/**
 * sdca_regmap_mbq_size - return size in bytes of a given SDCA Control
 * @function: Pointer to the Function information.
 * @reg: Register address/Control to be processed.
 *
 * Return: Returns the size in bytes of the Control.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_mbq_size(
    function: *mut sdca_function_data,
    reg: c_uint,
) -> c_int {
    let control: *mut sdca_control;

    if !SDW_SDCA_VALID_CTL(reg) {
        return -EINVAL;
    }

    control = function_find_control(function, reg);
    if control.is_null() {
        return -EINVAL;
    }

    clamp_val(
        (*control).nbits / BITS_PER_BYTE as c_uint,
        core::mem::size_of::<u8_>(),
        core::mem::size_of::<u32_>(),
    )
}

// EXPORT_SYMBOL_NS(sdca_regmap_mbq_size, "SND_SOC_SDCA");

/**
 * sdca_regmap_count_constants - count the number of DisCo constant Controls
 * @dev: Pointer to the device.
 * @function: Pointer to the Function information, to be parsed.
 *
 * This function returns the number of DisCo constant Controls present
 * in a function. Typically this information will be used to populate
 * the regmap defaults array, allowing drivers to access the values of
 * DisCo constants as any other physical register.
 *
 * Return: Returns number of DisCo constant controls, or a negative error
 * code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_count_constants(
    _dev: *mut device,
    function: *mut sdca_function_data,
) -> c_int {
    let mut nconsts: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;

    i = 0;
    while i < (*function).num_entities {
        let entity: *mut sdca_entity = (*function).entities.offset(i as isize);

        j = 0;
        while j < (*entity).num_controls {
            let control: *mut sdca_control = (*entity).controls.offset(j as isize);

            if (*control).mode == SDCA_ACCESS_MODE_DC || (*control).has_reset {
                nconsts += hweight64((*control).cn_list) as c_int;
            }

            j += 1;
        }

        i += 1;
    }

    nconsts
}

// EXPORT_SYMBOL_NS(sdca_regmap_count_constants, "SND_SOC_SDCA");

/**
 * sdca_regmap_populate_constants - fill an array with DisCo constant values
 * @dev: Pointer to the device.
 * @function: Pointer to the Function information, to be parsed.
 * @consts: Pointer to the array which should be filled with the DisCo
 * constant values.
 *
 * This function will populate a regmap struct reg_default array with
 * the values of the DisCo constants for a given Function. This
 * allows to access the values of DisCo constants the same as any
 * other physical register.
 *
 * Return: Returns the number of constants populated on success, a negative
 * error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_populate_constants(
    _dev: *mut device,
    function: *mut sdca_function_data,
    consts: *mut reg_default,
) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;
    let mut l: c_int;

    i = 0;
    k = 0;
    while i < (*function).num_entities {
        let entity: *mut sdca_entity = (*function).entities.offset(i as isize);

        j = 0;
        while j < (*entity).num_controls {
            let control: *mut sdca_control = (*entity).controls.offset(j as isize);
            let mut cn: c_int;

            if (*control).mode != SDCA_ACCESS_MODE_DC && !(*control).has_reset {
                j += 1;
                continue;
            }

            l = 0;
            cn = 0;
            while (cn as c_uint) < BITS_PER_TYPE((*control).cn_list) {
                if (((*control).cn_list >> cn) & 1) != 0 {
                    (*consts.offset(k as isize)).reg = SDW_SDCA_CTL(
                        (*(*function).desc).adr,
                        (*entity).id,
                        (*control).sel,
                        cn,
                    );
                    if (*control).mode == SDCA_ACCESS_MODE_DC {
                        (*consts.offset(k as isize)).def = *(*control).values.offset(l as isize);
                    } else {
                        (*consts.offset(k as isize)).def = (*control).reset;
                    }
                    k += 1;
                    l += 1;
                }

                cn += 1;
            }

            j += 1;
        }

        i += 1;
    }

    k
}

// EXPORT_SYMBOL_NS(sdca_regmap_populate_constants, "SND_SOC_SDCA");

unsafe fn populate_control_defaults(
    dev: *mut device,
    regmap: *mut regmap,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    control: *mut sdca_control,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut cn: c_int;

    if (*control).mode == SDCA_ACCESS_MODE_DC {
        return 0;
    }

    if ((*control).layers & SDCA_ACCESS_LAYER_DEVICE) != 0 {
        return 0;
    }

    i = 0;
    cn = 0;
    while (cn as c_uint) < BITS_PER_TYPE((*control).cn_list) {
        if (((*control).cn_list >> cn) & 1) != 0 {
            let reg: c_uint;
            let mut val: c_uint = 0;

            reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, cn);

            if (*control).has_default || (*control).has_fixed {
                ret = regmap_write(regmap, reg, *(*control).values.offset(i as isize));
                if ret != 0 {
                    dev_err(
                        dev,
                        b"Failed to write default %#x: %d\n\0".as_ptr() as *const c_char,
                        reg,
                        ret,
                    );
                    return ret;
                }

                i += 1;
            } else if !(*control).is_volatile {
                if (*control).has_reset {
                    regcache_drop_region(regmap, reg, reg);
                }

                ret = regmap_read(regmap, reg, &mut val);
                if ret != 0 {
                    dev_err(
                        dev,
                        b"Failed to read initial %#x: %d\n\0".as_ptr() as *const c_char,
                        reg,
                        ret,
                    );
                    return ret;
                }
            }
        }

        cn += 1;
    }

    0
}

/**
 * sdca_regmap_write_defaults - write out DisCo defaults to device
 * @dev: Pointer to the device.
 * @regmap: Pointer to the Function register map.
 * @function: Pointer to the Function information, to be parsed.
 *
 * This function will write out to the hardware all the DisCo default and
 * fixed value controls. This will cause them to be populated into the cache,
 * and subsequent handling can be done through a cache sync. It will also
 * read any non-volatile registers that don't have defaults/fixed values to
 * populate those into the cache, this ensures they are available for reads
 * even when the device is runtime suspended.
 *
 * Return: Returns zero on success, and a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_write_defaults(
    dev: *mut device,
    regmap: *mut regmap,
    function: *mut sdca_function_data,
) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut ret: c_int;

    i = 0;
    while i < (*function).num_entities {
        let entity: *mut sdca_entity = (*function).entities.offset(i as isize);

        j = 0;
        while j < (*entity).num_controls {
            let control: *mut sdca_control = (*entity).controls.offset(j as isize);

            ret = populate_control_defaults(dev, regmap, function, entity, control);
            if ret != 0 {
                return ret;
            }

            j += 1;
        }

        i += 1;
    }

    0
}

// EXPORT_SYMBOL_NS(sdca_regmap_write_defaults, "SND_SOC_SDCA");

#[no_mangle]
pub unsafe extern "C" fn sdca_regmap_write_init(
    _dev: *mut device,
    regmap: *mut regmap,
    function: *mut sdca_function_data,
) -> c_int {
    let init: *mut sdca_init_write = (*function).init_table;
    let mut ret: c_int;
    let mut i: c_int;

    i = 0;
    while i < (*function).num_init_table {
        ret = regmap_write(
            regmap,
            (*init.offset(i as isize)).addr,
            (*init.offset(i as isize)).val,
        );
        if ret != 0 {
            return ret;
        }

        i += 1;
    }

    0
}

// EXPORT_SYMBOL_NS(sdca_regmap_write_init, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
