// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 */

// Translated from the Linux kernel IMX PFD clock implementation.

use core::ffi::c_void;

extern "C" {
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> i32;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: usize,
    pub best_parent_rate: usize,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const i8,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const i8,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

#[repr(C)]
pub struct clk_pfd {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub idx: u8,
}

pub const SET: usize = 0x4;
pub const CLR: usize = 0x8;
pub const OTG: usize = 0xc;

unsafe fn clk_pfd_from_hw(hw: *mut clk_hw) -> *mut clk_pfd {
    hw as *mut clk_pfd
}

unsafe extern "C" fn clk_pfd_enable(hw: *mut clk_hw) -> i32 {
    let pfd = &mut *clk_pfd_from_hw(hw);
    writel_relaxed(
        1u32 << (((pfd.idx as u32 + 1) * 8) - 1),
        pfd.reg.wrapping_add(CLR),
    );
    0
}

unsafe extern "C" fn clk_pfd_disable(hw: *mut clk_hw) {
    let pfd = &mut *clk_pfd_from_hw(hw);
    writel_relaxed(
        1u32 << (((pfd.idx as u32 + 1) * 8) - 1),
        pfd.reg.wrapping_add(SET),
    );
}

unsafe extern "C" fn clk_pfd_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pfd = &mut *clk_pfd_from_hw(hw);
    let mut tmp = parent_rate as u64;
    let frac = ((readl_relaxed(pfd.reg) >> (pfd.idx * 8)) & 0x3f) as u64;
    tmp *= 18;
    tmp /= frac;
    tmp as usize
}

unsafe extern "C" fn clk_pfd_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let req = &mut *req;
    let mut tmp = req.best_parent_rate as u64;
    tmp = tmp * 18 + (req.rate / 2) as u64;
    tmp /= req.rate as u64;
    let mut frac = tmp as u8;
    if frac < 12 {
        frac = 12;
    } else if frac > 35 {
        frac = 35;
    }
    tmp = req.best_parent_rate as u64;
    tmp *= 18;
    tmp /= frac as u64;
    req.rate = tmp as usize;
    0
}

unsafe extern "C" fn clk_pfd_set_rate(
    hw: *mut clk_hw,
    rate: usize,
    parent_rate: usize,
) -> i32 {
    let pfd = &mut *clk_pfd_from_hw(hw);
    let mut tmp = parent_rate as u64;
    tmp = tmp * 18 + (rate / 2) as u64;
    tmp /= rate as u64;
    let mut frac = tmp as u8;
    if frac < 12 {
        frac = 12;
    } else if frac > 35 {
        frac = 35;
    }
    writel_relaxed(0x3f << (pfd.idx * 8), pfd.reg.wrapping_add(CLR));
    writel_relaxed((frac as u32) << (pfd.idx * 8), pfd.reg.wrapping_add(SET));
    0
}

unsafe extern "C" fn clk_pfd_is_enabled(hw: *mut clk_hw) -> i32 {
    let pfd = &mut *clk_pfd_from_hw(hw);
    if readl_relaxed(pfd.reg) & (1 << (((pfd.idx as u32 + 1) * 8) - 1)) != 0 {
        return 0;
    }
    1
}

#[no_mangle]
pub static clk_pfd_ops: clk_ops = clk_ops {
    enable: Some(clk_pfd_enable),
    disable: Some(clk_pfd_disable),
    recalc_rate: Some(clk_pfd_recalc_rate),
    determine_rate: Some(clk_pfd_determine_rate),
    set_rate: Some(clk_pfd_set_rate),
    is_enabled: Some(clk_pfd_is_enabled),
};

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_pfd(
    name: *const i8,
    parent_name: *const i8,
    reg: *mut c_void,
    idx: u8,
) -> *mut clk_hw {
    let pfd = Box::into_raw(Box::new(clk_pfd {
        hw: clk_hw { init: core::ptr::null() },
        reg,
        idx,
    }));
    let init = Box::into_raw(Box::new(clk_init_data {
        name,
        ops: &clk_pfd_ops,
        flags: 0,
        parent_names: &parent_name,
        num_parents: 1,
    }));
    (*pfd).hw.init = init;
    let hw = &mut (*pfd).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(pfd as *mut c_void);
        return ret as isize as *mut clk_hw;
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
