/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da9052 declarations for DA9052 PMICs.
 *
 * Copyright(c) 2011 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/* Common - HWMON Channel Definations */
pub const DA9052_ADC_VDDOUT: i32 = 0;
pub const DA9052_ADC_ICH: i32 = 1;
pub const DA9052_ADC_TBAT: i32 = 2;
pub const DA9052_ADC_VBAT: i32 = 3;
pub const DA9052_ADC_IN4: i32 = 4;
pub const DA9052_ADC_IN5: i32 = 5;
pub const DA9052_ADC_IN6: i32 = 6;
pub const DA9052_ADC_TSI: i32 = 7;
pub const DA9052_ADC_TJUNC: i32 = 8;
pub const DA9052_ADC_VBBAT: i32 = 9;

/* TSI channel has its own 4 channel mux */
pub const DA9052_ADC_TSI_XP: i32 = 70;
pub const DA9052_ADC_TSI_XN: i32 = 71;
pub const DA9052_ADC_TSI_YP: i32 = 72;
pub const DA9052_ADC_TSI_YN: i32 = 73;

pub const DA9052_IRQ_DCIN: i32 = 0;
pub const DA9052_IRQ_VBUS: i32 = 1;
pub const DA9052_IRQ_DCINREM: i32 = 2;
pub const DA9052_IRQ_VBUSREM: i32 = 3;
pub const DA9052_IRQ_VDDLOW: i32 = 4;
pub const DA9052_IRQ_ALARM: i32 = 5;
pub const DA9052_IRQ_SEQRDY: i32 = 6;
pub const DA9052_IRQ_COMP1V2: i32 = 7;
pub const DA9052_IRQ_NONKEY: i32 = 8;
pub const DA9052_IRQ_IDFLOAT: i32 = 9;
pub const DA9052_IRQ_IDGND: i32 = 10;
pub const DA9052_IRQ_CHGEND: i32 = 11;
pub const DA9052_IRQ_TBAT: i32 = 12;
pub const DA9052_IRQ_ADC_EOM: i32 = 13;
pub const DA9052_IRQ_PENDOWN: i32 = 14;
pub const DA9052_IRQ_TSIREADY: i32 = 15;
pub const DA9052_IRQ_GPI0: i32 = 16;
pub const DA9052_IRQ_GPI1: i32 = 17;
pub const DA9052_IRQ_GPI2: i32 = 18;
pub const DA9052_IRQ_GPI3: i32 = 19;
pub const DA9052_IRQ_GPI4: i32 = 20;
pub const DA9052_IRQ_GPI5: i32 = 21;
pub const DA9052_IRQ_GPI6: i32 = 22;
pub const DA9052_IRQ_GPI7: i32 = 23;
pub const DA9052_IRQ_GPI8: i32 = 24;
pub const DA9052_IRQ_GPI9: i32 = 25;
pub const DA9052_IRQ_GPI10: i32 = 26;
pub const DA9052_IRQ_GPI11: i32 = 27;
pub const DA9052_IRQ_GPI12: i32 = 28;
pub const DA9052_IRQ_GPI13: i32 = 29;
pub const DA9052_IRQ_GPI14: i32 = 30;
pub const DA9052_IRQ_GPI15: i32 = 31;

#[repr(i32)]
pub enum da9052_chip_id {
    DA9052,
    DA9053_AA,
    DA9053_BA,
    DA9053_BB,
    DA9053_BC,
}

pub enum da9052_pdata {}

#[repr(C)]
pub struct da9052 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub auxadc_lock: mutex,
    pub done: completion,
    pub irq_base: i32,
    pub irq_data: *mut regmap_irq_chip_data,
    pub chip_id: u8,
    pub chip_irq: i32,
    pub fault_log: i32,
    pub fix_io: Option<unsafe extern "C" fn(*mut da9052, u8) -> i32>,
}

extern "C" {
    pub fn da9052_adc_manual_read(da9052: *mut da9052, channel: u8) -> i32;
    pub fn da9052_adc_read_temp(da9052: *mut da9052) -> i32;

    pub fn regmap_read(map: *mut regmap, reg: u8, val: *mut i32) -> i32;
    pub fn regmap_write(map: *mut regmap, reg: u8, val: u8) -> i32;
    pub fn regmap_update_bits(map: *mut regmap, reg: u8, mask: u8, val: u8) -> i32;

    pub fn da9052_device_init(da9052: *mut da9052, chip_id: u8) -> i32;
    pub fn da9052_device_exit(da9052: *mut da9052);
    pub static da9052_regmap_config: regmap_config;
    pub fn da9052_irq_init(da9052: *mut da9052) -> i32;
    pub fn da9052_irq_exit(da9052: *mut da9052) -> i32;
    pub fn da9052_request_irq(da9052: *mut da9052, irq: i32, name: *mut i8,
                              handler: irq_handler_t, data: *mut core::ffi::c_void) -> i32;
    pub fn da9052_free_irq(da9052: *mut da9052, irq: i32, data: *mut core::ffi::c_void);
    pub fn da9052_enable_irq(da9052: *mut da9052, irq: i32) -> i32;
    pub fn da9052_disable_irq(da9052: *mut da9052, irq: i32) -> i32;
    pub fn da9052_disable_irq_nosync(da9052: *mut da9052, irq: i32) -> i32;
}

#[inline]
pub unsafe fn da9052_reg_read(da9052: *mut da9052, reg: u8) -> i32 {
    let mut val = 0i32;
    let mut ret = regmap_read((*da9052).regmap, reg, &mut val);
    if ret < 0 { return ret; }
    if let Some(fix_io) = (*da9052).fix_io {
        ret = fix_io(da9052, reg);
        if ret < 0 { return ret; }
    }
    val
}

#[inline]
pub unsafe fn da9052_reg_write(da9052: *mut da9052, reg: u8, val: u8) -> i32 {
    let mut ret = regmap_write((*da9052).regmap, reg, val);
    if ret < 0 { return ret; }
    if let Some(fix_io) = (*da9052).fix_io {
        ret = fix_io(da9052, reg);
        if ret < 0 { return ret; }
    }
    ret
}

#[inline]
pub unsafe fn da9052_group_read(da9052: *mut da9052, reg: u8, reg_cnt: u32, val: *mut u8) -> i32 {
    let mut ret;
    let mut tmp = 0u32;
    let mut i = 0u32;
    while i < reg_cnt {
        ret = regmap_read((*da9052).regmap, reg.wrapping_add(i as u8), &mut tmp as *mut u32 as *mut i32);
        *val.add(i as usize) = tmp as u8;
        if ret < 0 { return ret; }
        i += 1;
    }
    ret = 0;
    if let Some(fix_io) = (*da9052).fix_io {
        ret = fix_io(da9052, reg);
        if ret < 0 { return ret; }
    }
    ret
}

#[inline]
pub unsafe fn da9052_group_write(da9052: *mut da9052, reg: u8, reg_cnt: u32, val: *mut u8) -> i32 {
    let mut ret = 0;
    let mut i = 0u32;
    while i < reg_cnt {
        ret = regmap_write((*da9052).regmap, reg.wrapping_add(i as u8), *val.add(i as usize));
        if ret < 0 { return ret; }
        i += 1;
    }
    if let Some(fix_io) = (*da9052).fix_io {
        ret = fix_io(da9052, reg);
        if ret < 0 { return ret; }
    }
    ret
}

#[inline]
pub unsafe fn da9052_reg_update(da9052: *mut da9052, reg: u8, bit_mask: u8, reg_val: u8) -> i32 {
    let mut ret = regmap_update_bits((*da9052).regmap, reg, bit_mask, reg_val);
    if ret < 0 { return ret; }
    if let Some(fix_io) = (*da9052).fix_io {
        ret = fix_io(da9052, reg);
        if ret < 0 { return ret; }
    }
    ret
}

// External kernel types and interrupt handler type are supplied by dependencies.
extern "C" {
    pub type device;
    pub type regmap;
    pub type mutex;
    pub type completion;
    pub type regmap_irq_chip_data;
    pub type regmap_config;
    pub type irq_handler_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
