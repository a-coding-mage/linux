// SPDX-License-Identifier: GPL-2.0+
/*
 * This driver provides regmap to access to analog part of audio codec
 * found on Allwinner A23, A31s, A33, H3 and A64 Socs
 *
 * Copyright 2016 Chen-Yu Tsai <wens@csie.org>
 * Copyright (C) 2018 Vasily Khoruzhick <anarsoul@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub reg_read: Option<
        unsafe extern "C" fn(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int,
    >,
    pub reg_write:
        Option<unsafe extern "C" fn(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int>,
    pub fast_io: bool,
    pub max_register: c_uint,
}

unsafe extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
}

/* Analog control register access bits */
const ADDA_PR: u32 = 0x0; /* PRCM base + 0x1c0 */
const ADDA_PR_RESET: u32 = 1u32 << 28;
const ADDA_PR_WRITE: u32 = 1u32 << 24;
const ADDA_PR_ADDR_SHIFT: u32 = 16;
const ADDA_PR_ADDR_MASK: u32 = 0x1f;
const ADDA_PR_DATA_IN_SHIFT: u32 = 8;
const ADDA_PR_DATA_IN_MASK: u32 = 0xff;
const ADDA_PR_DATA_OUT_SHIFT: u32 = 0;
const ADDA_PR_DATA_OUT_MASK: u32 = 0xff;

/* regmap access bits */
unsafe extern "C" fn adda_reg_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let base: *mut c_void = context as *mut c_void;
    let mut tmp: u32;

    /* De-assert reset */
    unsafe {
        writel(readl(base as *const c_void) | ADDA_PR_RESET, base);
    }

    /* Clear write bit */
    unsafe {
        writel(readl(base as *const c_void) & !ADDA_PR_WRITE, base);
    }

    /* Set register address */
    unsafe {
        tmp = readl(base as *const c_void);
    }
    tmp &= !(ADDA_PR_ADDR_MASK << ADDA_PR_ADDR_SHIFT);
    tmp |= ((reg as u32) & ADDA_PR_ADDR_MASK) << ADDA_PR_ADDR_SHIFT;
    unsafe {
        writel(tmp, base);
    }

    /* Read back value */
    unsafe {
        *val = (readl(base as *const c_void) & ADDA_PR_DATA_OUT_MASK) as c_uint;
    }

    0
}

unsafe extern "C" fn adda_reg_write(
    context: *mut c_void,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    let base: *mut c_void = context as *mut c_void;
    let mut tmp: u32;

    /* De-assert reset */
    unsafe {
        writel(readl(base as *const c_void) | ADDA_PR_RESET, base);
    }

    /* Set register address */
    unsafe {
        tmp = readl(base as *const c_void);
    }
    tmp &= !(ADDA_PR_ADDR_MASK << ADDA_PR_ADDR_SHIFT);
    tmp |= ((reg as u32) & ADDA_PR_ADDR_MASK) << ADDA_PR_ADDR_SHIFT;
    unsafe {
        writel(tmp, base);
    }

    /* Set data to write */
    unsafe {
        tmp = readl(base as *const c_void);
    }
    tmp &= !(ADDA_PR_DATA_IN_MASK << ADDA_PR_DATA_IN_SHIFT);
    tmp |= ((val as u32) & ADDA_PR_DATA_IN_MASK) << ADDA_PR_DATA_IN_SHIFT;
    unsafe {
        writel(tmp, base);
    }

    /* Set write bit to signal a write */
    unsafe {
        writel(readl(base as *const c_void) | ADDA_PR_WRITE, base);
    }

    /* Clear write bit */
    unsafe {
        writel(readl(base as *const c_void) & !ADDA_PR_WRITE, base);
    }

    0
}

static ADDA_PR_REGMAP_CFG_NAME: &[u8; 8] = b"adda-pr\0";

static ADDA_PR_REGMAP_CFG: regmap_config = regmap_config {
    name: ADDA_PR_REGMAP_CFG_NAME.as_ptr() as *const c_char,
    reg_bits: 5,
    reg_stride: 1,
    val_bits: 8,
    reg_read: Some(adda_reg_read),
    reg_write: Some(adda_reg_write),
    fast_io: true,
    max_register: 31,
};

#[no_mangle]
pub unsafe extern "C" fn sun8i_adda_pr_regmap_init(
    dev: *mut device,
    base: *mut c_void,
) -> *mut regmap {
    unsafe { devm_regmap_init(dev, core::ptr::null(), base, &ADDA_PR_REGMAP_CFG) }
}

/* EXPORT_SYMBOL_GPL(sun8i_adda_pr_regmap_init); */

/* MODULE_DESCRIPTION("Allwinner analog audio codec regmap driver"); */
/* MODULE_AUTHOR("Vasily Khoruzhick <anarsoul@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:sunxi-adda-pr"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
