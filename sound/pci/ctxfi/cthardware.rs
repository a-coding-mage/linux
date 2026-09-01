// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthardware.c
 *
 * @Brief
 * This file contains the implementation of hardware access methord.
 *
 * @Author	Liu Chun
 * @Date 	Jun 26 2008
 */

// C dependencies:
// #include "cthardware.h"
// #include "cthw20k1.h"
// #include "cthw20k2.h"
// #include <linux/bug.h>

use core::ffi::c_int;

pub const ENODEV: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CHIPTYP {
    ATC20K1,
    ATC20K2,
}

use CHIPTYP::{ATC20K1, ATC20K2};

pub type CTCARDS = c_int;

#[repr(C)]
pub struct pci_dev {
    pub device: u16,
}

#[repr(C)]
pub struct hw {
    pub pci: *mut pci_dev,
    pub chip_type: CHIPTYP,
    pub model: CTCARDS,
}

unsafe extern "C" {
    pub fn create_20k1_hw_obj(rhw: *mut *mut hw) -> c_int;
    pub fn create_20k2_hw_obj(rhw: *mut *mut hw) -> c_int;
    pub fn destroy_20k1_hw_obj(hw: *mut hw) -> c_int;
    pub fn destroy_20k2_hw_obj(hw: *mut hw) -> c_int;

    // C macro from <linux/bug.h>; provided externally in the translated build.
    pub fn WARN_ON(condition: c_int) -> c_int;
}

pub unsafe fn create_hw_obj(
    pci: *mut pci_dev,
    chip_type: CHIPTYP,
    model: CTCARDS,
    rhw: *mut *mut hw,
) -> c_int {
    let err: c_int;

    match chip_type {
        ATC20K1 => {
            err = unsafe { create_20k1_hw_obj(rhw) };
        }
        ATC20K2 => {
            err = unsafe { create_20k2_hw_obj(rhw) };
        }
        _ => {
            err = -ENODEV;
        }
    }
    if err != 0 {
        return err;
    }

    unsafe {
        (**rhw).pci = pci;
        (**rhw).chip_type = chip_type;
        (**rhw).model = model;
    }

    0
}

pub unsafe fn destroy_hw_obj(hw: *mut hw) -> c_int {
    let err: c_int;

    match unsafe { (*(*hw).pci).device } {
        0x0005 => {
            /* 20k1 device */
            err = unsafe { destroy_20k1_hw_obj(hw) };
        }
        0x000B => {
            /* 20k2 device */
            err = unsafe { destroy_20k2_hw_obj(hw) };
        }
        _ => {
            err = -ENODEV;
        }
    }

    err
}

pub unsafe fn get_field(data: u32, field: u32) -> u32 {
    let mut i: c_int;

    if unsafe { WARN_ON((field == 0) as c_int) } != 0 {
        return 0;
    }
    /* @field should always be greater than 0 */
    i = 0;
    while (field & (1u32 << i)) == 0 {
        i += 1;
    }

    (data & field) >> i
}

pub unsafe fn set_field(data: *mut u32, field: u32, value: u32) {
    let mut i: c_int;

    if unsafe { WARN_ON((field == 0) as c_int) } != 0 {
        return;
    }
    /* @field should always be greater than 0 */
    i = 0;
    while (field & (1u32 << i)) == 0 {
        i += 1;
    }

    unsafe {
        *data = (*data & !field) | ((value << i) & field);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
