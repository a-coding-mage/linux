// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct clk_fixup_mux {
    pub mux: clk_mux,
    pub ops: *const clk_ops,
    pub fixup: Option<unsafe extern "C" fn(*mut u32)>,
}

#[inline]
unsafe fn to_clk_fixup_mux(hw: *mut clk_hw) -> *mut clk_fixup_mux {
    let mux = to_clk_mux(hw);
    // Equivalent to container_of(mux, struct clk_fixup_mux, mux).
    mux as *mut clk_fixup_mux
}

unsafe extern "C" fn clk_fixup_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let fixup_mux = to_clk_fixup_mux(hw);
    ((*(*fixup_mux).ops).get_parent.unwrap())(&mut (*fixup_mux).mux.hw)
}

unsafe extern "C" fn clk_fixup_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let fixup_mux = to_clk_fixup_mux(hw);
    let mux = to_clk_mux(hw);
    let mut flags: c_ulong = 0;
    let mut val: u32;

    spin_lock_irqsave((*mux).lock, &mut flags);

    val = readl((*mux).reg);
    val &= !((*mux).mask << (*mux).shift);
    val |= (index as u32) << (*mux).shift;
    ((*fixup_mux).fixup.unwrap())(&mut val);
    writel(val, (*mux).reg);

    spin_unlock_irqrestore((*mux).lock, flags);

    0
}

static clk_fixup_mux_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(clk_fixup_mux_get_parent),
    set_parent: Some(clk_fixup_mux_set_parent),
};

pub unsafe extern "C" fn imx_clk_hw_fixup_mux(
    name: *const c_char,
    reg: *mut core::ffi::c_void,
    shift: u8,
    width: u8,
    parents: *const *const c_char,
    num_parents: i32,
    fixup: Option<unsafe extern "C" fn(*mut u32)>,
) -> *mut clk_hw {
    let mut fixup_mux: *mut clk_fixup_mux;
    let hw: *mut clk_hw;
    let mut init: clk_init_data;
    let mut ret: i32;

    if fixup.is_none() {
        return ERR_PTR(-EINVAL);
    }

    fixup_mux = kzalloc(core::mem::size_of::<clk_fixup_mux>()) as *mut clk_fixup_mux;
    if fixup_mux.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &clk_fixup_mux_ops;
    init.parent_names = parents;
    init.num_parents = num_parents;
    init.flags = 0;

    (*fixup_mux).mux.reg = reg;
    (*fixup_mux).mux.shift = shift;
    (*fixup_mux).mux.mask = (1u32 << width) - 1;
    (*fixup_mux).mux.lock = &mut imx_ccm_lock;
    (*fixup_mux).mux.hw.init = &init;
    (*fixup_mux).ops = &clk_mux_ops;
    (*fixup_mux).fixup = fixup;

    hw = &mut (*fixup_mux).mux.hw;

    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(fixup_mux as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    hw
}

// External types, globals, constants, and functions supplied by included kernel headers.
extern "C" {
    static mut imx_ccm_lock: spinlock_t;
    static clk_mux_ops: clk_ops;
    fn to_clk_mux(hw: *mut clk_hw) -> *mut clk_mux;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn clk_hw_determine_rate_no_reparent(hw: *mut clk_hw, rate: *mut core::ffi::c_void) -> i32;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn ERR_PTR(error: i32) -> *mut clk_hw;
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
type c_char = i8;
type c_ulong = usize;

#[allow(non_camel_case_types)]
#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub reg: *mut core::ffi::c_void, pub shift: u8, pub mask: u32, pub lock: *mut spinlock_t }
#[repr(C)] pub struct clk_ops { pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut core::ffi::c_void) -> i32>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32> }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub parent_names: *const *const c_char, pub num_parents: i32, pub flags: u32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
