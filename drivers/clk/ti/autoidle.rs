// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI clock autoidle support
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct clk_ti_autoidle {
    pub reg: clk_omap_reg,
    pub shift: u8,
    pub flags: u8,
    pub name: *const core::ffi::c_char,
    pub node: list_head,
}

const AUTOIDLE_LOW: u8 = 0x1;

static mut autoidle_clks: list_head = list_head_init();
static mut autoidle_spinlock: spinlock_t = spinlock_init();

unsafe fn _omap2_clk_deny_idle(clk: *mut clk_hw_omap) -> i32 {
    if !(*clk).ops.is_null() && (*(*clk).ops).deny_idle.is_some() {
        let mut irqflags: c_ulong = 0;
        spin_lock_irqsave(&raw mut autoidle_spinlock, &mut irqflags);
        (*clk).autoidle_count += 1;
        if (*clk).autoidle_count == 1 {
            ((*(*clk).ops).deny_idle.unwrap())(clk);
        }
        spin_unlock_irqrestore(&raw mut autoidle_spinlock, irqflags);
    }
    0
}

unsafe fn _omap2_clk_allow_idle(clk: *mut clk_hw_omap) -> i32 {
    if !(*clk).ops.is_null() && (*(*clk).ops).allow_idle.is_some() {
        let mut irqflags: c_ulong = 0;
        spin_lock_irqsave(&raw mut autoidle_spinlock, &mut irqflags);
        (*clk).autoidle_count -= 1;
        if (*clk).autoidle_count == 0 {
            ((*(*clk).ops).allow_idle.unwrap())(clk);
        }
        spin_unlock_irqrestore(&raw mut autoidle_spinlock, irqflags);
    }
    0
}

pub unsafe fn omap2_clk_deny_idle(clk: *mut clk) -> i32 {
    if clk.is_null() { return -EINVAL; }
    let hw = __clk_get_hw(clk);
    if omap2_clk_is_hw_omap(hw) {
        return _omap2_clk_deny_idle(to_clk_hw_omap(hw));
    }
    -EINVAL
}

pub unsafe fn omap2_clk_allow_idle(clk: *mut clk) -> i32 {
    if clk.is_null() { return -EINVAL; }
    let hw = __clk_get_hw(clk);
    if omap2_clk_is_hw_omap(hw) {
        return _omap2_clk_allow_idle(to_clk_hw_omap(hw));
    }
    -EINVAL
}

unsafe fn _allow_autoidle(clk: *mut clk_ti_autoidle) {
    let mut val = ((*ti_clk_ll_ops).clk_readl)(&raw mut (*clk).reg);
    if (*clk).flags & AUTOIDLE_LOW != 0 {
        val &= !(1u32 << (*clk).shift);
    } else {
        val |= 1u32 << (*clk).shift;
    }
    ((*ti_clk_ll_ops).clk_writel)(val, &raw mut (*clk).reg);
}

unsafe fn _deny_autoidle(clk: *mut clk_ti_autoidle) {
    let mut val = ((*ti_clk_ll_ops).clk_readl)(&raw mut (*clk).reg);
    if (*clk).flags & AUTOIDLE_LOW != 0 {
        val |= 1u32 << (*clk).shift;
    } else {
        val &= !(1u32 << (*clk).shift);
    }
    ((*ti_clk_ll_ops).clk_writel)(val, &raw mut (*clk).reg);
}

unsafe fn _clk_generic_allow_autoidle_all() {
    let mut c: *mut clk_ti_autoidle = core::ptr::null_mut();
    list_for_each_entry(&mut c, &raw mut autoidle_clks, node) {
        _allow_autoidle(c);
    }
}

unsafe fn _clk_generic_deny_autoidle_all() {
    let mut c: *mut clk_ti_autoidle = core::ptr::null_mut();
    list_for_each_entry(&mut c, &raw mut autoidle_clks, node) {
        _deny_autoidle(c);
    }
}

pub unsafe fn of_ti_clk_autoidle_setup(node: *mut device_node) -> i32 {
    let mut shift: u32 = 0;
    if of_property_read_u32(node, b"ti,autoidle-shift\0".as_ptr() as _, &mut shift) != 0 {
        return 0;
    }
    let clk = kzalloc_obj::<clk_ti_autoidle>();
    if clk.is_null() { return -ENOMEM; }
    (*clk).shift = shift as u8;
    (*clk).name = ti_dt_clk_name(node);
    let ret = ti_clk_get_reg_addr(node, 0, &mut (*clk).reg);
    if ret != 0 { kfree(clk as *mut core::ffi::c_void); return ret; }
    if of_property_read_bool(node, b"ti,invert-autoidle-bit\0".as_ptr() as _) {
        (*clk).flags |= AUTOIDLE_LOW;
    }
    list_add(&mut (*clk).node, &raw mut autoidle_clks);
    0
}

pub unsafe fn omap2_clk_enable_autoidle_all() -> i32 {
    let ret = omap2_clk_for_each(Some(_omap2_clk_allow_idle));
    if ret != 0 { return ret; }
    _clk_generic_allow_autoidle_all();
    0
}

pub unsafe fn omap2_clk_disable_autoidle_all() -> i32 {
    let ret = omap2_clk_for_each(Some(_omap2_clk_deny_idle));
    if ret != 0 { return ret; }
    _clk_generic_deny_autoidle_all();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
