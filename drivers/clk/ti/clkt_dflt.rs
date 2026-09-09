// SPDX-License-Identifier: GPL-2.0-only
/*
 * Default clock type
 *
 * Copyright (C) 2005-2008, 2015 Texas Instruments, Inc.
 * Copyright (C) 2004-2010 Nokia Corporation
 *
 * Contacts:
 * Richard Woodruff <r-woodruff2@ti.com>
 * Paul Walmsley
 * Tero Kristo <t-kristo@ti.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const MAX_MODULE_ENABLE_WAIT: i32 = 100000;
const CM_FCLKEN: u32 = 0x0000;
const CM_ICLKEN: u32 = 0x0010;

/* External types and operations are provided by the translated kernel tree. */
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct clk_omap_reg {
    pub offset: u32,
    pub index: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct clk_hw {
    pub clk: *mut core::ffi::c_void,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct clk_hw_omap_ops {
    pub find_idlest: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8, *mut u8)>,
    pub find_companion: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8)>,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct clk_hw_omap {
    pub hw: clk_hw,
    pub enable_reg: clk_omap_reg,
    pub enable_bit: u8,
    pub flags: u32,
    pub clkdm: *mut core::ffi::c_void,
    pub clkdm_name: *const core::ffi::c_char,
    pub ops: *const clk_hw_omap_ops,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct ti_clk_ll_ops_type {
    pub clk_readl: unsafe extern "C" fn(*const clk_omap_reg) -> u32,
    pub clk_writel: unsafe extern "C" fn(u32, *const clk_omap_reg),
    pub cm_split_idlest_reg: unsafe extern "C" fn(*const clk_omap_reg, *mut i16, *mut u8) -> i32,
    pub cm_wait_module_ready: unsafe extern "C" fn(u8, i16, u8, u8),
    pub clkdm_clk_enable: unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32,
    pub clkdm_clk_disable: unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}

extern "C" {
    static ti_clk_ll_ops: *const ti_clk_ll_ops_type;
    fn udelay(usecs: u32);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn clk_hw_get_name(hw: *const clk_hw) -> *const core::ffi::c_char;
    fn ti_clk_get_features() -> *const ti_clk_features;
}

#[repr(C)]
struct ti_clk_features {
    pub flags: u32,
    pub cm_idlest_val: u8,
}

const TI_CLK_DISABLE_CLKDM_CONTROL: u32 = 1 << 0;
const INVERT_ENABLE: u32 = 1 << 0;

unsafe fn to_clk_hw_omap(hw: *mut clk_hw) -> *mut clk_hw_omap {
    hw as *mut clk_hw_omap
}

unsafe fn _wait_idlest_generic(
    _clk: *mut clk_hw_omap,
    reg: *mut clk_omap_reg,
    mask: u32,
    idlest: u8,
    _name: *const core::ffi::c_char,
) -> i32 {
    let mut i: i32 = 0;
    let ena: u32 = if idlest != 0 { 0 } else { mask };

    for n in 0..MAX_MODULE_ENABLE_WAIT {
        i = n;
        if ((*ti_clk_ll_ops).clk_readl)(reg) & mask == ena {
            break;
        }
        udelay(1);
    }

    i32::from(i < MAX_MODULE_ENABLE_WAIT)
}

unsafe fn _omap2_module_wait_ready(clk: *mut clk_hw_omap) {
    let mut companion_reg = core::mem::MaybeUninit::<clk_omap_reg>::uninit();
    let mut idlest_reg = core::mem::MaybeUninit::<clk_omap_reg>::uninit();
    let mut other_bit = 0u8;
    let mut idlest_bit = 0u8;
    let mut idlest_val = 0u8;
    let mut idlest_reg_id = 0u8;
    let mut prcm_mod = 0i16;

    let ops = (*clk).ops;
    if !ops.is_null() {
        if let Some(find_companion) = (*ops).find_companion {
            find_companion(clk, companion_reg.as_mut_ptr(), &mut other_bit);
            if ((*ti_clk_ll_ops).clk_readl(companion_reg.as_ptr()) & (1u32 << other_bit)) == 0 {
                return;
            }
        }
    }

    if let Some(find_idlest) = (*ops).find_idlest {
        find_idlest(clk, idlest_reg.as_mut_ptr(), &mut idlest_bit, &mut idlest_val);
    }
    let r = ((*ti_clk_ll_ops).cm_split_idlest_reg)(
        idlest_reg.as_ptr(), &mut prcm_mod, &mut idlest_reg_id,
    );
    if r != 0 {
        _wait_idlest_generic(clk, idlest_reg.as_mut_ptr(), 1u32 << idlest_bit, idlest_val, clk_hw_get_name(&(*clk).hw));
    } else {
        ((*ti_clk_ll_ops).cm_wait_module_ready)(0, prcm_mod, idlest_reg_id, idlest_bit);
    }
}

pub unsafe extern "C" fn omap2_clk_dflt_find_companion(
    clk: *mut clk_hw_omap,
    other_reg: *mut clk_omap_reg,
    other_bit: *mut u8,
) {
    *other_reg = (*clk).enable_reg;
    (*other_reg).offset ^= CM_FCLKEN ^ CM_ICLKEN;
    *other_bit = (*clk).enable_bit;
}

pub unsafe extern "C" fn omap2_clk_dflt_find_idlest(
    clk: *mut clk_hw_omap,
    idlest_reg: *mut clk_omap_reg,
    idlest_bit: *mut u8,
    idlest_val: *mut u8,
) {
    *idlest_reg = (*clk).enable_reg;
    (*idlest_reg).offset &= !0xf0;
    (*idlest_reg).offset |= 0x20;
    *idlest_bit = (*clk).enable_bit;
    *idlest_val = (*ti_clk_get_features()).cm_idlest_val;
}

pub unsafe extern "C" fn omap2_dflt_clk_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw);
    let clkdm_control = ((*ti_clk_get_features()).flags & TI_CLK_DISABLE_CLKDM_CONTROL) == 0;
    if clkdm_control && !(*clk).clkdm.is_null() {
        let ret = ((*ti_clk_ll_ops).clkdm_clk_enable)((*clk).clkdm, (*hw).clk);
        if ret != 0 { return ret; }
    }
    let mut v = ((*ti_clk_ll_ops).clk_readl)(&(*clk).enable_reg);
    if (*clk).flags & INVERT_ENABLE != 0 { v &= !(1u32 << (*clk).enable_bit); }
    else { v |= 1u32 << (*clk).enable_bit; }
    ((*ti_clk_ll_ops).clk_writel)(v, &(*clk).enable_reg);
    let _ = ((*ti_clk_ll_ops).clk_readl)(&(*clk).enable_reg);
    if !(*clk).ops.is_null() && (*(*clk).ops).find_idlest.is_some() { _omap2_module_wait_ready(clk); }
    0
}

pub unsafe extern "C" fn omap2_dflt_clk_disable(hw: *mut clk_hw) {
    let clk = to_clk_hw_omap(hw);
    let mut v = ((*ti_clk_ll_ops).clk_readl)(&(*clk).enable_reg);
    if (*clk).flags & INVERT_ENABLE != 0 { v |= 1u32 << (*clk).enable_bit; }
    else { v &= !(1u32 << (*clk).enable_bit); }
    ((*ti_clk_ll_ops).clk_writel)(v, &(*clk).enable_reg);
    if ((*ti_clk_get_features()).flags & TI_CLK_DISABLE_CLKDM_CONTROL) == 0 && !(*clk).clkdm.is_null() {
        ((*ti_clk_ll_ops).clkdm_clk_disable)((*clk).clkdm, (*hw).clk);
    }
}

pub unsafe extern "C" fn omap2_dflt_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw);
    let mut v = ((*ti_clk_ll_ops).clk_readl)(&(*clk).enable_reg);
    if (*clk).flags & INVERT_ENABLE != 0 { v ^= 1u32 << (*clk).enable_bit; }
    if v & (1u32 << (*clk).enable_bit) != 0 { 1 } else { 0 }
}

#[repr(C)]
pub struct clk_hw_omap_ops_export {
    pub find_idlest: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8, *mut u8)>,
    pub find_companion: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8)>,
}

pub static clkhwops_wait: clk_hw_omap_ops_export = clk_hw_omap_ops_export {
    find_idlest: Some(omap2_clk_dflt_find_idlest),
    find_companion: Some(omap2_clk_dflt_find_companion),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
