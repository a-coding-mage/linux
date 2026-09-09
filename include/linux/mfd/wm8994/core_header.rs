/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm8994/core.h -- Core interface for WM8994
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ushort, c_void};

#[repr(C)]
pub struct wm8994_pdata {
    _private: [u8; 0],
}
pub struct device;
pub struct regmap;
pub struct regmap_irq_chip_data;
pub struct regulator_dev;
pub struct regulator_bulk_data;
pub struct irq_domain;

pub type irq_handler_t = unsafe extern "C" fn(c_int, *mut c_void) -> c_int;

extern "C" {
    pub fn regmap_read(map: *mut regmap, reg: c_ushort, val: *mut u32) -> c_int;
    pub fn regmap_write(map: *mut regmap, reg: c_ushort, val: c_ushort) -> c_int;
    pub fn regmap_bulk_read(
        map: *mut regmap,
        reg: c_ushort,
        val: *mut u16,
        count: c_int,
    ) -> c_int;
    pub fn regmap_raw_write(
        map: *mut regmap,
        reg: c_ushort,
        val: *const u16,
        count: usize,
    ) -> c_int;
    pub fn regmap_update_bits(
        map: *mut regmap,
        reg: c_ushort,
        mask: c_ushort,
        val: c_ushort,
    ) -> c_int;
    pub fn regmap_irq_get_virq(data: *mut regmap_irq_chip_data, irq: c_int) -> c_int;
    pub fn request_threaded_irq(
        irq: c_int,
        thread_fn: Option<irq_handler_t>,
        handler: Option<irq_handler_t>,
        flags: c_int,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    pub fn free_irq(irq: c_int, data: *mut c_void);
}

pub const EINVAL: c_int = 22;
pub const IRQF_TRIGGER_RISING: c_int = 0x0000_0040;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm8994_type {
    WM8994 = 0,
    WM8958 = 1,
    WM1811 = 2,
}

pub const WM8994_NUM_GPIO_REGS: c_int = 11;
pub const WM8994_NUM_LDO_REGS: c_int = 2;
pub const WM8994_NUM_IRQ_REGS: c_int = 2;

pub const WM8994_IRQ_TEMP_SHUT: c_int = 0;
pub const WM8994_IRQ_MIC1_DET: c_int = 1;
pub const WM8994_IRQ_MIC1_SHRT: c_int = 2;
pub const WM8994_IRQ_MIC2_DET: c_int = 3;
pub const WM8994_IRQ_MIC2_SHRT: c_int = 4;
pub const WM8994_IRQ_FLL1_LOCK: c_int = 5;
pub const WM8994_IRQ_FLL2_LOCK: c_int = 6;
pub const WM8994_IRQ_SRC1_LOCK: c_int = 7;
pub const WM8994_IRQ_SRC2_LOCK: c_int = 8;
pub const WM8994_IRQ_AIF1DRC1_SIG_DET: c_int = 9;
pub const WM8994_IRQ_AIF1DRC2_SIG_DET: c_int = 10;
pub const WM8994_IRQ_AIF2DRC_SIG_DET: c_int = 11;
pub const WM8994_IRQ_FIFOS_ERR: c_int = 12;
pub const WM8994_IRQ_WSEQ_DONE: c_int = 13;
pub const WM8994_IRQ_DCS_DONE: c_int = 14;
pub const WM8994_IRQ_TEMP_WARN: c_int = 15;

/* GPIOs in the chip are numbered from 1-11 */
#[inline]
pub const fn WM8994_IRQ_GPIO(x: c_int) -> c_int {
    x + WM8994_IRQ_TEMP_WARN
}

#[repr(C)]
pub struct wm8994 {
    pub pdata: wm8994_pdata,
    pub r#type: wm8994_type,
    pub revision: c_int,
    pub cust_id: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ldo_ena_always_driven: bool,
    pub gpio_base: c_int,
    pub irq_base: c_int,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
    pub edge_irq: *mut irq_domain,
    pub suspended: bool,
    pub dbvdd: *mut regulator_dev,
    pub num_supplies: c_int,
    pub supplies: *mut regulator_bulk_data,
}

/* Device I/O API */

#[inline]
pub unsafe fn wm8994_reg_read(wm8994: *mut wm8994, reg: c_ushort) -> c_int {
    let mut val: u32 = 0;
    let ret = regmap_read((*wm8994).regmap, reg, &mut val);
    if ret < 0 { ret } else { val as c_int }
}

#[inline]
pub unsafe fn wm8994_reg_write(
    wm8994: *mut wm8994,
    reg: c_ushort,
    val: c_ushort,
) -> c_int {
    regmap_write((*wm8994).regmap, reg, val)
}

#[inline]
pub unsafe fn wm8994_bulk_read(
    wm8994: *mut wm8994,
    reg: c_ushort,
    count: c_int,
    buf: *mut u16,
) -> c_int {
    regmap_bulk_read((*wm8994).regmap, reg, buf, count)
}

#[inline]
pub unsafe fn wm8994_bulk_write(
    wm8994: *mut wm8994,
    reg: c_ushort,
    count: c_int,
    buf: *const u16,
) -> c_int {
    regmap_raw_write((*wm8994).regmap, reg, buf, (count as usize) * core::mem::size_of::<u16>())
}

#[inline]
pub unsafe fn wm8994_set_bits(
    wm8994: *mut wm8994,
    reg: c_ushort,
    mask: c_ushort,
    val: c_ushort,
) -> c_int {
    regmap_update_bits((*wm8994).regmap, reg, mask, val)
}

/* Helper to save on boilerplate */
#[inline]
pub unsafe fn wm8994_request_irq(
    wm8994: *mut wm8994,
    irq: c_int,
    handler: Option<irq_handler_t>,
    name: *const c_char,
    data: *mut c_void,
) -> c_int {
    if (*wm8994).irq_data.is_null() {
        return -EINVAL;
    }
    request_threaded_irq(
        regmap_irq_get_virq((*wm8994).irq_data, irq),
        None,
        handler,
        IRQF_TRIGGER_RISING,
        name,
        data,
    )
}

#[inline]
pub unsafe fn wm8994_free_irq(wm8994: *mut wm8994, irq: c_int, data: *mut c_void) {
    if (*wm8994).irq_data.is_null() {
        return;
    }
    free_irq(regmap_irq_get_virq((*wm8994).irq_data, irq), data);
}

extern "C" {
    pub fn wm8994_irq_init(wm8994: *mut wm8994) -> c_int;
    pub fn wm8994_irq_exit(wm8994: *mut wm8994);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
