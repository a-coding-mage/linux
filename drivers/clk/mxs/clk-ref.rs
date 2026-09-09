// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the Linux clock framework and the MXS clock code
// are intentionally left as external Rust items.

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64, u64) -> i32>,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: u64,
    pub best_parent_rate: u64,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ref {
    pub hw: clk_hw,
    pub reg: *mut u8,
    pub idx: u8,
}

extern "C" {
    static mut mxs_lock: core::ffi::c_void;
    static CLR: usize;
    static SET: usize;

    fn writel_relaxed(value: u32, address: *mut u8);
    fn readl_relaxed(address: *mut u8) -> u32;
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn clk_register(parent: *mut core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
}

unsafe fn to_clk_ref(hw: *mut clk_hw) -> *mut clk_ref {
    // `hw` is the first member of `struct clk_ref`.
    hw as *mut clk_ref
}

unsafe extern "C" fn clk_ref_enable(hw: *mut clk_hw) -> i32 {
    let ref_ = &mut *to_clk_ref(hw);

    writel_relaxed(
        1u32 << (((ref_.idx as u32 + 1) * 8) - 1),
        ref_.reg.add(CLR),
    );

    0
}

unsafe extern "C" fn clk_ref_disable(hw: *mut clk_hw) {
    let ref_ = &mut *to_clk_ref(hw);

    writel_relaxed(
        1u32 << (((ref_.idx as u32 + 1) * 8) - 1),
        ref_.reg.add(SET),
    );
}

unsafe extern "C" fn clk_ref_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let ref_ = &mut *to_clk_ref(hw);
    let mut tmp = parent_rate;
    let frac = ((readl_relaxed(ref_.reg) >> ((ref_.idx as u32) * 8)) & 0x3f) as u8;

    tmp *= 18;
    tmp /= frac as u64;

    tmp
}

unsafe extern "C" fn clk_ref_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let req = &mut *req;
    let parent_rate = req.best_parent_rate;
    let mut tmp = parent_rate;

    tmp = tmp * 18 + req.rate / 2;
    tmp /= req.rate;
    let frac = tmp.clamp(18, 35);

    tmp = parent_rate;
    tmp *= 18;
    tmp /= frac;

    req.rate = tmp;

    0
}

unsafe extern "C" fn clk_ref_set_rate(
    hw: *mut clk_hw,
    rate: u64,
    parent_rate: u64,
) -> i32 {
    let ref_ = &mut *to_clk_ref(hw);
    let mut flags: usize = 0;
    let mut tmp = parent_rate;
    let mut val: u32;
    let frac;
    let shift = (ref_.idx as u32) * 8;

    tmp = tmp * 18 + rate / 2;
    tmp /= rate;
    frac = tmp.clamp(18, 35) as u32;

    spin_lock_irqsave(&raw mut mxs_lock, &mut flags);

    val = readl_relaxed(ref_.reg);
    val &= !(0x3f_u32 << shift);
    val |= frac << shift;
    writel_relaxed(val, ref_.reg);

    spin_unlock_irqrestore(&raw mut mxs_lock, flags);

    0
}

static clk_ref_ops: clk_ops = clk_ops {
    enable: Some(clk_ref_enable),
    disable: Some(clk_ref_disable),
    recalc_rate: Some(clk_ref_recalc_rate),
    determine_rate: Some(clk_ref_determine_rate),
    set_rate: Some(clk_ref_set_rate),
};

pub unsafe extern "C" fn mxs_clk_ref(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    reg: *mut u8,
    idx: u8,
) -> *mut clk {
    // Allocation and error-pointer handling are provided by the kernel allocator.
    let ref_: *mut clk_ref = kzalloc_obj::<clk_ref>();
    if ref_.is_null() {
        return err_ptr(-12);
    }

    let mut init = clk_init_data {
        name,
        ops: &clk_ref_ops,
        flags: 0,
        parent_names: if !parent_name.is_null() {
            &parent_name
        } else {
            core::ptr::null()
        },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*ref_).reg = reg;
    (*ref_).idx = idx;
    (*ref_).hw.init = &mut init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*ref_).hw);
    if is_err(clk) {
        kfree(ref_ as *mut core::ffi::c_void);
    }

    clk
}

extern "C" {
    fn kzalloc_obj<T>() -> *mut T;
    fn err_ptr(error: i32) -> *mut clk;
    fn is_err(ptr: *mut clk) -> bool;
    fn kfree(ptr: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
