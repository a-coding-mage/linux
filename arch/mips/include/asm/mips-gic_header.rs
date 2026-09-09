/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

/* This header requires asm/mips-cps.h in the C implementation. */

use core::ffi::c_void;

pub const MIPS_GIC_SHARED_OFS: usize = 0x00000;
pub const MIPS_GIC_SHARED_SZ: usize = 0x08000;
pub const MIPS_GIC_LOCAL_OFS: usize = 0x08000;
pub const MIPS_GIC_LOCAL_SZ: usize = 0x04000;
pub const MIPS_GIC_REDIR_OFS: usize = 0x0c000;
pub const MIPS_GIC_REDIR_SZ: usize = 0x04000;
pub const MIPS_GIC_USER_OFS: usize = 0x10000;
pub const MIPS_GIC_USER_SZ: usize = 0x10000;

extern "C" {
    pub static mut mips_gic_base: *mut c_void;
    pub static mips_cm_is64: bool;
    pub fn gic_get_c0_compare_int() -> i32;
    pub fn gic_get_c0_perfcount_int() -> i32;
    pub fn gic_get_c0_fdc_int() -> i32;
}

/* C accessor-generator macros. The CPS_ACCESSOR_* definitions are supplied by
 * asm/mips-cps.h and expand these declarations into MMIO accessors. */
#[macro_export]
macro_rules! GIC_ACCESSOR_RO { ($sz:expr, $off:expr, $name:ident) => {}; }
#[macro_export]
macro_rules! GIC_ACCESSOR_RW { ($sz:expr, $off:expr, $name:ident) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RO { ($sz:expr, $off:expr, $name:ident) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RW { ($sz:expr, $off:expr, $name:ident) => {}; }
#[macro_export]
macro_rules! GIC_ACCESSOR_RO_INTR_REG { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_ACCESSOR_RW_INTR_REG { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RO_INTR_REG { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RW_INTR_REG { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_ACCESSOR_RO_INTR_BIT { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_ACCESSOR_RW_INTR_BIT { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RO_INTR_BIT { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! GIC_VX_ACCESSOR_RW_INTR_BIT { ($($arg:tt)*) => {}; }

pub const GIC_CONFIG_COUNTSTOP: u32 = 1u32 << 28;
pub const GIC_CONFIG_COUNTBITS: u32 = 0x0f << 24;
pub const GIC_CONFIG_NUMINTERRUPTS: u32 = 0xff << 16;
pub const GIC_CONFIG_PVPS: u32 = 0x7f;
pub const GIC_POL_ACTIVE_LOW: u32 = 0;
pub const GIC_POL_ACTIVE_HIGH: u32 = 1;
pub const GIC_POL_FALLING_EDGE: u32 = 0;
pub const GIC_POL_RISING_EDGE: u32 = 1;
pub const GIC_TRIG_LEVEL: u32 = 0;
pub const GIC_TRIG_EDGE: u32 = 1;
pub const GIC_DUAL_SINGLE: u32 = 0;
pub const GIC_DUAL_DUAL: u32 = 1;
pub const GIC_WEDGE_RW: u32 = 1u32 << 31;
pub const GIC_WEDGE_INTR: u32 = 0xff;
pub const GIC_MAP_PIN_MAP_TO_PIN: u32 = 1u32 << 31;
pub const GIC_MAP_PIN_MAP_TO_NMI: u32 = 1u32 << 30;
pub const GIC_MAP_PIN_MAP: u32 = 0x3f;
pub const GIC_VX_CTL_FDC_ROUTABLE: u32 = 1u32 << 4;
pub const GIC_VX_CTL_SWINT_ROUTABLE: u32 = 1u32 << 3;
pub const GIC_VX_CTL_PERFCNT_ROUTABLE: u32 = 1u32 << 2;
pub const GIC_VX_CTL_TIMER_ROUTABLE: u32 = 1u32 << 1;
pub const GIC_VX_CTL_EIC: u32 = 1;
pub const GIC_VX_OTHER_VPNUM: u32 = 0x3f;
pub const GIC_VX_IDENT_VPNUM: u32 = 0x3f;

GIC_ACCESSOR_RW!(32, 0x000, config);
GIC_ACCESSOR_RW!(64, 0x010, counter);
GIC_ACCESSOR_RW!(32, 0x010, counter_32l);
GIC_ACCESSOR_RW!(32, 0x014, counter_32h);
GIC_ACCESSOR_RW_INTR_BIT!(0x100, pol);
GIC_ACCESSOR_RW_INTR_BIT!(0x180, trig);
GIC_ACCESSOR_RW_INTR_BIT!(0x200, dual);
GIC_ACCESSOR_RW!(32, 0x280, wedge);
GIC_ACCESSOR_RW_INTR_BIT!(0x300, rmask);
GIC_ACCESSOR_RW_INTR_BIT!(0x380, smask);
GIC_ACCESSOR_RO_INTR_BIT!(0x400, mask);
GIC_ACCESSOR_RO_INTR_BIT!(0x480, pend);
GIC_ACCESSOR_RW_INTR_REG!(32, 0x500, 0x4, map_pin);
GIC_ACCESSOR_RW_INTR_REG!(32, 0x2000, 0x20, map_vp);
GIC_VX_ACCESSOR_RW!(32, 0x000, ctl);
GIC_VX_ACCESSOR_RO!(32, 0x004, pend);
GIC_VX_ACCESSOR_RO!(32, 0x008, mask);
GIC_VX_ACCESSOR_RW!(32, 0x00c, rmask);
GIC_VX_ACCESSOR_RW!(32, 0x010, smask);
GIC_VX_ACCESSOR_RW_INTR_REG!(32, 0x040, 0x4, map);
GIC_VX_ACCESSOR_RW!(32, 0x040, wd_map);
GIC_VX_ACCESSOR_RW!(32, 0x044, compare_map);
GIC_VX_ACCESSOR_RW!(32, 0x048, timer_map);
GIC_VX_ACCESSOR_RW!(32, 0x04c, fdc_map);
GIC_VX_ACCESSOR_RW!(32, 0x050, perfctr_map);
GIC_VX_ACCESSOR_RW!(32, 0x054, swint0_map);
GIC_VX_ACCESSOR_RW!(32, 0x058, swint1_map);
GIC_VX_ACCESSOR_RW!(32, 0x080, other);
GIC_VX_ACCESSOR_RO!(32, 0x088, ident);
GIC_VX_ACCESSOR_RW!(64, 0x0a0, compare);
GIC_VX_ACCESSOR_RW_INTR_REG!(32, 0x100, 0x4, eic_shadow_set);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mips_gic_local_interrupt {
    GIC_LOCAL_INT_WD,
    GIC_LOCAL_INT_COMPARE,
    GIC_LOCAL_INT_TIMER,
    GIC_LOCAL_INT_PERFCTR,
    GIC_LOCAL_INT_SWINT0,
    GIC_LOCAL_INT_SWINT1,
    GIC_LOCAL_INT_FDC,
    GIC_NUM_LOCAL_INTRS,
}

#[inline]
pub unsafe fn mips_gic_present() -> bool {
    /* IS_ENABLED(CONFIG_MIPS_GIC) is a build-time C configuration condition. */
    !mips_gic_base.is_null()
}

#[inline]
pub fn mips_gic_vx_map_reg(intr: mips_gic_local_interrupt) -> u32 {
    let intr = intr as u32;
    if intr <= GIC_LOCAL_INT_TIMER as u32 { return intr; }
    if intr == GIC_LOCAL_INT_FDC as u32 { return GIC_LOCAL_INT_TIMER as u32 + 1; }
    intr + 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
