/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 Clock Management (CM) register definitions
 *
 * Copyright (C) 2007-2009 Texas Instruments, Inc.
 * Copyright (C) 2007-2010 Nokia Corporation
 * Paul Walmsley
 *
 * The CM hardware modules on the OMAP2/3 are quite similar to each
 * other.  The CM modules/instances on OMAP4 are quite different, so
 * they are handled in a separate file.
 */

/* Dependency intent: symbols from cm.h and linux/io.h are supplied externally. */

/* Module specific CM register offsets from CM_BASE + domain offset. */
pub const CM_FCLKEN: u32 = 0x0000;
pub const CM_FCLKEN1: u32 = CM_FCLKEN;
pub const CM_CLKEN: u32 = CM_FCLKEN;
pub const CM_ICLKEN: u32 = 0x0010;
pub const CM_ICLKEN1: u32 = CM_ICLKEN;
pub const CM_ICLKEN2: u32 = 0x0014;
pub const CM_ICLKEN3: u32 = 0x0018;
pub const CM_IDLEST: u32 = 0x0020;
pub const CM_IDLEST1: u32 = CM_IDLEST;
pub const CM_IDLEST2: u32 = 0x0024;
pub const OMAP2430_CM_IDLEST3: u32 = 0x0028;
pub const CM_AUTOIDLE: u32 = 0x0030;
pub const CM_AUTOIDLE1: u32 = CM_AUTOIDLE;
pub const CM_AUTOIDLE2: u32 = 0x0034;
pub const CM_AUTOIDLE3: u32 = 0x0038;
pub const CM_CLKSEL: u32 = 0x0040;
pub const CM_CLKSEL1: u32 = CM_CLKSEL;
pub const CM_CLKSEL2: u32 = 0x0044;
pub const OMAP2_CM_CLKSTCTRL: u32 = 0x0048;

/* External MMIO and CM-base declarations supplied by other translation units. */
extern "C" {
    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn __ffs(value: u32) -> u32;
}

#[repr(C)]
pub struct CmBase {
    pub va: *mut u8,
}

extern "C" {
    pub static cm_base: CmBase;
}

pub unsafe fn omap2_cm_read_mod_reg(module: i16, idx: u16) -> u32 {
    let addr = cm_base.va.offset(module as isize + idx as isize);
    readl_relaxed(addr as *const core::ffi::c_void)
}

pub unsafe fn omap2_cm_write_mod_reg(val: u32, module: i16, idx: u16) {
    let addr = cm_base.va.offset(module as isize + idx as isize);
    writel_relaxed(val, addr as *mut core::ffi::c_void);
}

/* Read-modify-write a register in a CM module. Caller must lock. */
pub unsafe fn omap2_cm_rmw_mod_reg_bits(mask: u32, bits: u32, module: i16, idx: i16) -> u32 {
    let mut v = omap2_cm_read_mod_reg(module, idx as u16);
    v &= !mask;
    v |= bits;
    omap2_cm_write_mod_reg(v, module, idx as u16);
    v
}

/* Read a CM register, AND it, and shift the result down to bit 0. */
pub unsafe fn omap2_cm_read_mod_bits_shift(domain: i16, idx: i16, mask: u32) -> u32 {
    let mut v = omap2_cm_read_mod_reg(domain, idx as u16);
    v &= mask;
    v >>= __ffs(mask);
    v
}

pub unsafe fn omap2_cm_set_mod_reg_bits(bits: u32, module: i16, idx: i16) -> u32 {
    omap2_cm_rmw_mod_reg_bits(bits, bits, module, idx)
}

pub unsafe fn omap2_cm_clear_mod_reg_bits(bits: u32, module: i16, idx: i16) -> u32 {
    omap2_cm_rmw_mod_reg_bits(bits, 0x0, module, idx)
}

/* CM register bits shared between 24XX and 3430. */
pub const OMAP_CLKSEL_GFX_SHIFT: u32 = 0;
pub const OMAP_CLKSEL_GFX_MASK: u32 = 0x7 << 0;
pub const OMAP_CLKSEL_GFX_WIDTH: u32 = 3;
pub const OMAP_EN_GFX_SHIFT: u32 = 0;
pub const OMAP_EN_GFX_MASK: u32 = 1 << 0;
pub const OMAP_ST_GFX_MASK: u32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
