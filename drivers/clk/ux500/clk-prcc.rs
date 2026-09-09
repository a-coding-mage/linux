// SPDX-License-Identifier: GPL-2.0-only
/*
 * PRCC clock implementation for ux500 platform.
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Author: Ulf Hansson <ulf.hansson@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const PRCC_PCKEN: usize = 0x000;
const PRCC_PCKDIS: usize = 0x004;
const PRCC_KCKEN: usize = 0x008;
const PRCC_KCKDIS: usize = 0x00C;
const PRCC_PCKSR: usize = 0x010;
const PRCC_KCKSR: usize = 0x014;

#[repr(C)]
pub struct clk_prcc {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub cg_sel: u32,
    pub is_enabled: i32,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: usize,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: usize,
}

#[repr(C)]
pub struct clk;

type resource_size_t = usize;

unsafe extern "C" {
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn readl(address: *mut core::ffi::c_void) -> u32;
    fn cpu_relax();
    fn ioremap(physical_base: resource_size_t, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(address: *mut core::ffi::c_void);
    fn clk_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
    fn kfree(pointer: *mut clk_prcc);
    fn pr_err(format: *const core::ffi::c_char, ...);
}

unsafe fn clk_prcc_from_hw(hw: *mut clk_hw) -> *mut clk_prcc {
    hw as *mut clk_prcc
}

/* PRCC clock operations. */

unsafe extern "C" fn clk_prcc_pclk_enable(hw: *mut clk_hw) -> i32 {
    let clk = &mut *clk_prcc_from_hw(hw);

    writel(clk.cg_sel, clk.base.add(PRCC_PCKEN));
    while (readl(clk.base.add(PRCC_PCKSR)) & clk.cg_sel) == 0 {
        cpu_relax();
    }

    clk.is_enabled = 1;
    0
}

unsafe extern "C" fn clk_prcc_pclk_disable(hw: *mut clk_hw) {
    let clk = &mut *clk_prcc_from_hw(hw);

    writel(clk.cg_sel, clk.base.add(PRCC_PCKDIS));
    clk.is_enabled = 0;
}

unsafe extern "C" fn clk_prcc_kclk_enable(hw: *mut clk_hw) -> i32 {
    let clk = &mut *clk_prcc_from_hw(hw);

    writel(clk.cg_sel, clk.base.add(PRCC_KCKEN));
    while (readl(clk.base.add(PRCC_KCKSR)) & clk.cg_sel) == 0 {
        cpu_relax();
    }

    clk.is_enabled = 1;
    0
}

unsafe extern "C" fn clk_prcc_kclk_disable(hw: *mut clk_hw) {
    let clk = &mut *clk_prcc_from_hw(hw);

    writel(clk.cg_sel, clk.base.add(PRCC_KCKDIS));
    clk.is_enabled = 0;
}

unsafe extern "C" fn clk_prcc_is_enabled(hw: *mut clk_hw) -> i32 {
    (*clk_prcc_from_hw(hw)).is_enabled
}

static CLK_PRCC_PCLK_OPS: clk_ops = clk_ops {
    enable: Some(clk_prcc_pclk_enable),
    disable: Some(clk_prcc_pclk_disable),
    is_enabled: Some(clk_prcc_is_enabled),
};

static CLK_PRCC_KCLK_OPS: clk_ops = clk_ops {
    enable: Some(clk_prcc_kclk_enable),
    disable: Some(clk_prcc_kclk_disable),
    is_enabled: Some(clk_prcc_is_enabled),
};

unsafe fn clk_reg_prcc(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    phy_base: resource_size_t,
    cg_sel: u32,
    flags: usize,
    clk_prcc_ops: *const clk_ops,
) -> *mut clk {
    if name.is_null() {
        return core::ptr::null_mut();
    }

    let clk = Box::into_raw(Box::new(clk_prcc {
        hw: clk_hw { init: core::ptr::null_mut() },
        base: ioremap(phy_base, 4096),
        cg_sel,
        is_enabled: 1,
    }));
    if (*clk).base.is_null() {
        kfree(clk);
        return core::ptr::null_mut();
    }

    let parent_names = if parent_name.is_null() {
        core::ptr::null()
    } else {
        &parent_name as *const *const core::ffi::c_char
    };
    let init = Box::into_raw(Box::new(clk_init_data {
        name,
        ops: clk_prcc_ops,
        flags,
        parent_names,
        num_parents: if parent_name.is_null() { 0 } else { 1 },
    }));
    (*clk).hw.init = init;

    let clk_reg = clk_register(core::ptr::null_mut(), &mut (*clk).hw);
    if !clk_reg.is_null() {
        return clk_reg;
    }

    iounmap((*clk).base);
    kfree(clk);
    core::ptr::null_mut()
}

pub unsafe extern "C" fn clk_reg_prcc_pclk(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    phy_base: resource_size_t,
    cg_sel: u32,
    flags: usize,
) -> *mut clk {
    clk_reg_prcc(name, parent_name, phy_base, cg_sel, flags, &CLK_PRCC_PCLK_OPS)
}

pub unsafe extern "C" fn clk_reg_prcc_kclk(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    phy_base: resource_size_t,
    cg_sel: u32,
    flags: usize,
) -> *mut clk {
    clk_reg_prcc(name, parent_name, phy_base, cg_sel, flags, &CLK_PRCC_KCLK_OPS)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
