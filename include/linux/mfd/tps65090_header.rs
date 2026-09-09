/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Core driver interface for TI TPS65090 PMIC family
 *
 * Copyright (C) 2012 NVIDIA Corporation
 */

// Dependency intent from the original header: linux/irq.h and linux/regmap.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(i32)]
pub enum Tps65090Irq {
    TPS65090_IRQ_INTERRUPT,
    TPS65090_IRQ_VAC_STATUS_CHANGE,
    TPS65090_IRQ_VSYS_STATUS_CHANGE,
    TPS65090_IRQ_BAT_STATUS_CHANGE,
    TPS65090_IRQ_CHARGING_STATUS_CHANGE,
    TPS65090_IRQ_CHARGING_COMPLETE,
    TPS65090_IRQ_OVERLOAD_DCDC1,
    TPS65090_IRQ_OVERLOAD_DCDC2,
    TPS65090_IRQ_OVERLOAD_DCDC3,
    TPS65090_IRQ_OVERLOAD_FET1,
    TPS65090_IRQ_OVERLOAD_FET2,
    TPS65090_IRQ_OVERLOAD_FET3,
    TPS65090_IRQ_OVERLOAD_FET4,
    TPS65090_IRQ_OVERLOAD_FET5,
    TPS65090_IRQ_OVERLOAD_FET6,
    TPS65090_IRQ_OVERLOAD_FET7,
}

#[repr(i32)]
pub enum Tps65090RegulatorId {
    TPS65090_REGULATOR_DCDC1,
    TPS65090_REGULATOR_DCDC2,
    TPS65090_REGULATOR_DCDC3,
    TPS65090_REGULATOR_FET1,
    TPS65090_REGULATOR_FET2,
    TPS65090_REGULATOR_FET3,
    TPS65090_REGULATOR_FET4,
    TPS65090_REGULATOR_FET5,
    TPS65090_REGULATOR_FET6,
    TPS65090_REGULATOR_FET7,
    TPS65090_REGULATOR_LDO1,
    TPS65090_REGULATOR_LDO2,
    TPS65090_REGULATOR_MAX,
}

pub const TPS65090_REG_INTR_STS: u32 = 0x00;
pub const TPS65090_REG_INTR_STS2: u32 = 0x01;
pub const TPS65090_REG_INTR_MASK: u32 = 0x02;
pub const TPS65090_REG_INTR_MASK2: u32 = 0x03;
pub const TPS65090_REG_CG_CTRL0: u32 = 0x04;
pub const TPS65090_REG_CG_CTRL1: u32 = 0x05;
pub const TPS65090_REG_CG_CTRL2: u32 = 0x06;
pub const TPS65090_REG_CG_CTRL3: u32 = 0x07;
pub const TPS65090_REG_CG_CTRL4: u32 = 0x08;
pub const TPS65090_REG_CG_CTRL5: u32 = 0x09;
pub const TPS65090_REG_CG_STATUS1: u32 = 0x0a;
pub const TPS65090_REG_CG_STATUS2: u32 = 0x0b;
pub const TPS65090_REG_AD_OUT1: u32 = 0x17;
pub const TPS65090_REG_AD_OUT2: u32 = 0x18;
pub const TPS65090_MAX_REG: u32 = TPS65090_REG_AD_OUT2;
pub const TPS65090_NUM_REGS: u32 = TPS65090_MAX_REG + 1;

#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct regmap_irq_chip_data;
#[repr(C)]
pub struct regulator_init_data;

#[repr(C)]
pub struct tps65090 {
    pub dev: *mut device,
    pub rmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
}

#[repr(C)]
pub struct tps65090_regulator_plat_data {
    pub reg_init_data: *mut regulator_init_data,
    pub enable_ext_control: bool,
    pub gpiod: *mut gpio_desc,
    pub overcurrent_wait_valid: bool,
    pub overcurrent_wait: c_int,
}

#[repr(C)]
pub struct tps65090_platform_data {
    pub irq_base: c_int,
    pub supplied_to: *mut *mut c_char,
    pub num_supplicants: usize,
    pub enable_low_current_chrg: c_int,
    pub reg_pdata: [*mut tps65090_regulator_plat_data; TPS65090_REGULATOR_MAX as usize],
}

extern "C" {
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    pub fn regmap_write(map: *mut regmap, reg: c_int, val: u8) -> c_int;
    pub fn regmap_read(map: *mut regmap, reg: c_int, val: *mut u32) -> c_int;
    pub fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: u32, val: u32) -> c_int;
}

#[inline]
pub unsafe fn tps65090_write(dev: *mut device, reg: c_int, val: u8) -> c_int {
    let tps = dev_get_drvdata(dev) as *mut tps65090;
    regmap_write((*tps).rmap, reg, val)
}

#[inline]
pub unsafe fn tps65090_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int {
    let tps = dev_get_drvdata(dev) as *mut tps65090;
    let mut temp_val = 0u32;
    let ret = regmap_read((*tps).rmap, reg, &mut temp_val);
    if ret == 0 {
        *val = temp_val as u8;
    }
    ret
}

#[inline]
pub unsafe fn tps65090_set_bits(dev: *mut device, reg: c_int, bit_num: u8) -> c_int {
    let tps = dev_get_drvdata(dev) as *mut tps65090;
    regmap_update_bits((*tps).rmap, reg, 1u32 << bit_num, !0u32)
}

#[inline]
pub unsafe fn tps65090_clr_bits(dev: *mut device, reg: c_int, bit_num: u8) -> c_int {
    let tps = dev_get_drvdata(dev) as *mut tps65090;
    regmap_update_bits((*tps).rmap, reg, 1u32 << bit_num, 0u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
