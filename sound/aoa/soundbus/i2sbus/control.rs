// SPDX-License-Identifier: GPL-2.0-only
/*
 * i2sbus driver -- bus control routines
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// C dependencies removed from executable Rust:
// linux/kernel.h, linux/delay.h, linux/slab.h, linux/io.h
// asm/macio.h, asm/pmac_feature.h, asm/pmac_pfunc.h, asm/keylargo.h
// "i2sbus.h"

use core::ffi::{c_char, c_int, c_void};

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct pmf_args {
    pub count: c_int,
}

#[repr(C)]
pub struct pmf_function {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct macio_chip {
    pub base: *mut c_void,
}

#[repr(C)]
pub struct macio_bus {
    pub chip: *mut macio_chip,
}

#[repr(C)]
pub struct macio_dev {
    pub bus: *mut macio_bus,
}

#[repr(C)]
pub struct of_device_dev {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct of_device {
    pub dev: of_device_dev,
}

#[repr(C)]
pub struct sound_device {
    pub ofdev: of_device,
}

#[repr(C)]
pub struct i2sbus_control {
    pub list: list_head,
    pub macio: *mut macio_chip,
}

#[repr(C)]
pub struct i2sbus_dev {
    pub item: list_head,
    pub sound: sound_device,
    pub enable: *mut pmf_function,
    pub cell_enable: *mut pmf_function,
    pub clock_enable: *mut pmf_function,
    pub cell_disable: *mut pmf_function,
    pub clock_disable: *mut pmf_function,
    pub bus_number: c_int,
}

unsafe extern "C" {
    fn kzalloc_obj_i2sbus_control() -> *mut i2sbus_control;
    fn kfree(ptr: *mut c_void);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;

    fn pmf_find_function(np: *mut device_node, name: *const c_char) -> *mut pmf_function;
    fn pmf_put_function(func: *mut pmf_function);
    fn pmf_call_one(func: *mut pmf_function, args: *mut pmf_args) -> c_int;

    fn printk(fmt: *const c_char, ...) -> c_int;

    fn MACIO_BIS(macio: *mut macio_chip, reg: c_int, bit: c_int);
    fn MACIO_BIC(macio: *mut macio_chip, reg: c_int, bit: c_int);

    static KEYLARGO_FCR1: c_int;
    static KL1_I2S0_ENABLE: c_int;
    static KL1_I2S1_ENABLE: c_int;
    static KL1_I2S0_CELL_ENABLE: c_int;
    static KL1_I2S1_CELL_ENABLE: c_int;
    static KL1_I2S0_CLK_ENABLE_BIT: c_int;
    static KL1_I2S1_CLK_ENABLE_BIT: c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_init(
    dev: *mut macio_dev,
    c: *mut *mut i2sbus_control,
) -> c_int {
    unsafe {
        *c = kzalloc_obj_i2sbus_control();
        if (*c).is_null() {
            return -ENOMEM;
        }

        INIT_LIST_HEAD(&mut (**c).list);

        (**c).macio = (*(*dev).bus).chip;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_destroy(c: *mut i2sbus_control) {
    unsafe {
        kfree(c as *mut c_void);
    }
}

/* this is serialised externally */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_add_dev(
    c: *mut i2sbus_control,
    i2sdev: *mut i2sbus_dev,
) -> c_int {
    unsafe {
        let np: *mut device_node;

        np = (*i2sdev).sound.ofdev.dev.of_node;
        (*i2sdev).enable = pmf_find_function(np, c"enable".as_ptr());
        (*i2sdev).cell_enable = pmf_find_function(np, c"cell-enable".as_ptr());
        (*i2sdev).clock_enable = pmf_find_function(np, c"clock-enable".as_ptr());
        (*i2sdev).cell_disable = pmf_find_function(np, c"cell-disable".as_ptr());
        (*i2sdev).clock_disable = pmf_find_function(np, c"clock-disable".as_ptr());

        /* if the bus number is not 0 or 1 we absolutely need to use
         * the platform functions -- there's nothing in Darwin that
         * would allow seeing a system behind what the FCRs are then,
         * and I don't want to go parsing a bunch of platform functions
         * by hand to try finding a system... */
        if (*i2sdev).bus_number != 0
            && (*i2sdev).bus_number != 1
            && ((*i2sdev).enable.is_null()
                || (*i2sdev).cell_enable.is_null()
                || (*i2sdev).clock_enable.is_null()
                || (*i2sdev).cell_disable.is_null()
                || (*i2sdev).clock_disable.is_null())
        {
            pmf_put_function((*i2sdev).enable);
            pmf_put_function((*i2sdev).cell_enable);
            pmf_put_function((*i2sdev).clock_enable);
            pmf_put_function((*i2sdev).cell_disable);
            pmf_put_function((*i2sdev).clock_disable);
            return -ENODEV;
        }

        list_add(&mut (*i2sdev).item, &mut (*c).list);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_remove_dev(
    c: *mut i2sbus_control,
    i2sdev: *mut i2sbus_dev,
) {
    unsafe {
        /* this is serialised externally */
        list_del(&mut (*i2sdev).item);
        if list_empty(&(*c).list) != 0 {
            i2sbus_control_destroy(c);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_enable(
    c: *mut i2sbus_control,
    i2sdev: *mut i2sbus_dev,
) -> c_int {
    unsafe {
        let mut args = pmf_args { count: 0 };
        let macio = (*c).macio;

        if !(*i2sdev).enable.is_null() {
            return pmf_call_one((*i2sdev).enable, &mut args);
        }

        if macio.is_null() || (*macio).base.is_null() {
            return -ENODEV;
        }

        match (*i2sdev).bus_number {
            0 => {
                /* these need to be locked or done through
                 * newly created feature calls! */
                MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S0_ENABLE);
            }
            1 => {
                MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S1_ENABLE);
            }
            _ => {
                return -ENODEV;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_cell(
    c: *mut i2sbus_control,
    i2sdev: *mut i2sbus_dev,
    enable: c_int,
) -> c_int {
    unsafe {
        let mut args = pmf_args { count: 0 };
        let macio = (*c).macio;

        match enable {
            0 => {
                if !(*i2sdev).cell_disable.is_null() {
                    return pmf_call_one((*i2sdev).cell_disable, &mut args);
                }
            }
            1 => {
                if !(*i2sdev).cell_enable.is_null() {
                    return pmf_call_one((*i2sdev).cell_enable, &mut args);
                }
            }
            _ => {
                printk(c"i2sbus: INVALID CELL ENABLE VALUE\n".as_ptr());
                return -ENODEV;
            }
        }

        if macio.is_null() || (*macio).base.is_null() {
            return -ENODEV;
        }

        match (*i2sdev).bus_number {
            0 => {
                if enable != 0 {
                    MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S0_CELL_ENABLE);
                } else {
                    MACIO_BIC(macio, KEYLARGO_FCR1, KL1_I2S0_CELL_ENABLE);
                }
            }
            1 => {
                if enable != 0 {
                    MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S1_CELL_ENABLE);
                } else {
                    MACIO_BIC(macio, KEYLARGO_FCR1, KL1_I2S1_CELL_ENABLE);
                }
            }
            _ => {
                return -ENODEV;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn i2sbus_control_clock(
    c: *mut i2sbus_control,
    i2sdev: *mut i2sbus_dev,
    enable: c_int,
) -> c_int {
    unsafe {
        let mut args = pmf_args { count: 0 };
        let macio = (*c).macio;

        match enable {
            0 => {
                if !(*i2sdev).clock_disable.is_null() {
                    return pmf_call_one((*i2sdev).clock_disable, &mut args);
                }
            }
            1 => {
                if !(*i2sdev).clock_enable.is_null() {
                    return pmf_call_one((*i2sdev).clock_enable, &mut args);
                }
            }
            _ => {
                printk(c"i2sbus: INVALID CLOCK ENABLE VALUE\n".as_ptr());
                return -ENODEV;
            }
        }

        if macio.is_null() || (*macio).base.is_null() {
            return -ENODEV;
        }

        match (*i2sdev).bus_number {
            0 => {
                if enable != 0 {
                    MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S0_CLK_ENABLE_BIT);
                } else {
                    MACIO_BIC(macio, KEYLARGO_FCR1, KL1_I2S0_CLK_ENABLE_BIT);
                }
            }
            1 => {
                if enable != 0 {
                    MACIO_BIS(macio, KEYLARGO_FCR1, KL1_I2S1_CLK_ENABLE_BIT);
                } else {
                    MACIO_BIC(macio, KEYLARGO_FCR1, KL1_I2S1_CLK_ENABLE_BIT);
                }
            }
            _ => {
                return -ENODEV;
            }
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
