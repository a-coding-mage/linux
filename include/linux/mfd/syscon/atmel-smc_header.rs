/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Atmel SMC (Static Memory Controller) register offsets and bit definitions.
 *
 * Copyright (C) 2014 Atmel
 * Copyright (C) 2014 Free Electrons
 *
 * Author: Boris Brezillon <boris.brezillon@free-electrons.com>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[inline]
pub const fn atmel_smc_setup(cs: u32) -> u32 { cs * 0x10 }

#[inline]
pub unsafe fn atmel_hsmc_setup(layout: *const atmel_hsmc_reg_layout, cs: u32) -> u32 {
    (*layout).timing_regs_offset + cs * 0x14
}

#[inline]
pub const fn atmel_smc_pulse(cs: u32) -> u32 { cs * 0x10 + 0x4 }

#[inline]
pub unsafe fn atmel_hsmc_pulse(layout: *const atmel_hsmc_reg_layout, cs: u32) -> u32 {
    (*layout).timing_regs_offset + cs * 0x14 + 0x4
}

#[inline]
pub const fn atmel_smc_cycle(cs: u32) -> u32 { cs * 0x10 + 0x8 }

#[inline]
pub unsafe fn atmel_hsmc_cycle(layout: *const atmel_hsmc_reg_layout, cs: u32) -> u32 {
    (*layout).timing_regs_offset + cs * 0x14 + 0x8
}

pub const ATMEL_SMC_NWE_SHIFT: u32 = 0;
pub const ATMEL_SMC_NCS_WR_SHIFT: u32 = 8;
pub const ATMEL_SMC_NRD_SHIFT: u32 = 16;
pub const ATMEL_SMC_NCS_RD_SHIFT: u32 = 24;

#[inline]
pub const fn atmel_smc_mode(cs: u32) -> u32 { cs * 0x10 + 0xc }

#[inline]
pub unsafe fn atmel_hsmc_mode(layout: *const atmel_hsmc_reg_layout, cs: u32) -> u32 {
    (*layout).timing_regs_offset + cs * 0x14 + 0x10
}

pub const ATMEL_SMC_MODE_READMODE_MASK: u32 = 1 << 0;
pub const ATMEL_SMC_MODE_READMODE_NCS: u32 = 0 << 0;
pub const ATMEL_SMC_MODE_READMODE_NRD: u32 = 1 << 0;
pub const ATMEL_SMC_MODE_WRITEMODE_MASK: u32 = 1 << 1;
pub const ATMEL_SMC_MODE_WRITEMODE_NCS: u32 = 0 << 1;
pub const ATMEL_SMC_MODE_WRITEMODE_NWE: u32 = 1 << 1;
pub const ATMEL_SMC_MODE_EXNWMODE_MASK: u32 = ((1 << (5 - 4 + 1)) - 1) << 4;
pub const ATMEL_SMC_MODE_EXNWMODE_DISABLE: u32 = 0 << 4;
pub const ATMEL_SMC_MODE_EXNWMODE_FROZEN: u32 = 2 << 4;
pub const ATMEL_SMC_MODE_EXNWMODE_READY: u32 = 3 << 4;
pub const ATMEL_SMC_MODE_BAT_MASK: u32 = 1 << 8;
pub const ATMEL_SMC_MODE_BAT_SELECT: u32 = 0 << 8;
pub const ATMEL_SMC_MODE_BAT_WRITE: u32 = 1 << 8;
pub const ATMEL_SMC_MODE_DBW_MASK: u32 = ((1 << (13 - 12 + 1)) - 1) << 12;
pub const ATMEL_SMC_MODE_DBW_8: u32 = 0 << 12;
pub const ATMEL_SMC_MODE_DBW_16: u32 = 1 << 12;
pub const ATMEL_SMC_MODE_DBW_32: u32 = 2 << 12;
pub const ATMEL_SMC_MODE_TDF_MASK: u32 = ((1 << (19 - 16 + 1)) - 1) << 16;
#[inline]
pub const fn atmel_smc_mode_tdf(x: u32) -> u32 { (x - 1) << 16 }
pub const ATMEL_SMC_MODE_TDF_MAX: u32 = 16;
pub const ATMEL_SMC_MODE_TDF_MIN: u32 = 1;
pub const ATMEL_SMC_MODE_TDFMODE_OPTIMIZED: u32 = 1 << 20;
pub const ATMEL_SMC_MODE_PMEN: u32 = 1 << 24;
pub const ATMEL_SMC_MODE_PS_MASK: u32 = ((1 << (29 - 28 + 1)) - 1) << 28;
pub const ATMEL_SMC_MODE_PS_4: u32 = 0 << 28;
pub const ATMEL_SMC_MODE_PS_8: u32 = 1 << 28;
pub const ATMEL_SMC_MODE_PS_16: u32 = 2 << 28;
pub const ATMEL_SMC_MODE_PS_32: u32 = 3 << 28;

#[inline]
pub unsafe fn atmel_hsmc_timings(layout: *const atmel_hsmc_reg_layout, cs: u32) -> u32 {
    (*layout).timing_regs_offset + cs * 0x14 + 0xc
}
pub const ATMEL_HSMC_TIMINGS_OCMS: u32 = 1 << 12;
#[inline]
pub const fn atmel_hsmc_timings_rbnsel(x: u32) -> u32 { x << 28 }
pub const ATMEL_HSMC_TIMINGS_NFSEL: u32 = 1 << 31;
pub const ATMEL_HSMC_TIMINGS_TCLR_SHIFT: u32 = 0;
pub const ATMEL_HSMC_TIMINGS_TADL_SHIFT: u32 = 4;
pub const ATMEL_HSMC_TIMINGS_TAR_SHIFT: u32 = 8;
pub const ATMEL_HSMC_TIMINGS_TRR_SHIFT: u32 = 16;
pub const ATMEL_HSMC_TIMINGS_TWB_SHIFT: u32 = 24;

#[repr(C)]
pub struct atmel_hsmc_reg_layout {
    pub timing_regs_offset: u32,
}

/**
 * struct atmel_smc_cs_conf - SMC CS config as described in the datasheet.
 * @setup: NCS/NWE/NRD setup timings (not applicable to at91rm9200)
 * @pulse: NCS/NWE/NRD pulse timings (not applicable to at91rm9200)
 * @cycle: NWE/NRD cycle timings (not applicable to at91rm9200)
 * @timings: advanced NAND related timings (only applicable to HSMC)
 * @mode: all kind of config parameters (see the fields definition above).
 *\t  The mode fields are different on at91rm9200
 */
#[repr(C)]
pub struct atmel_smc_cs_conf {
    pub setup: u32,
    pub pulse: u32,
    pub cycle: u32,
    pub timings: u32,
    pub mode: u32,
}

extern "C" {
    pub fn atmel_smc_cs_conf_init(conf: *mut atmel_smc_cs_conf);
    pub fn atmel_smc_cs_conf_set_timing(conf: *mut atmel_smc_cs_conf, shift: u32, ncycles: u32) -> c_int;
    pub fn atmel_smc_cs_conf_set_setup(conf: *mut atmel_smc_cs_conf, shift: u32, ncycles: u32) -> c_int;
    pub fn atmel_smc_cs_conf_set_pulse(conf: *mut atmel_smc_cs_conf, shift: u32, ncycles: u32) -> c_int;
    pub fn atmel_smc_cs_conf_set_cycle(conf: *mut atmel_smc_cs_conf, shift: u32, ncycles: u32) -> c_int;
    pub fn atmel_smc_cs_conf_apply(regmap: *mut regmap, cs: c_int, conf: *const atmel_smc_cs_conf);
    pub fn atmel_hsmc_cs_conf_apply(regmap: *mut regmap, reglayout: *const atmel_hsmc_reg_layout, cs: c_int, conf: *const atmel_smc_cs_conf);
    pub fn atmel_smc_cs_conf_get(regmap: *mut regmap, cs: c_int, conf: *mut atmel_smc_cs_conf);
    pub fn atmel_hsmc_cs_conf_get(regmap: *mut regmap, reglayout: *const atmel_hsmc_reg_layout, cs: c_int, conf: *mut atmel_smc_cs_conf);
    pub fn atmel_hsmc_get_reg_layout(np: *mut device_node) -> *const atmel_hsmc_reg_layout;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
