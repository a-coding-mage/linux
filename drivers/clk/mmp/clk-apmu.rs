// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmp AXI peripharal clock operation source file
 *
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct ClkApmu {
    pub hw: ClkHw,
    pub base: *mut core::ffi::c_void,
    pub rst_mask: u32,
    pub enable_mask: u32,
    pub lock: *mut SpinlockT,
}

#[repr(C)]
pub struct ClkHw {
    pub init: *mut ClkInitData,
}

#[repr(C)]
pub struct ClkInitData {
    pub name: *const core::ffi::c_char,
    pub ops: *const ClkOps,
    pub flags: u32,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct ClkOps {
    pub enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
}

#[repr(C)]
pub struct Clk;

#[repr(C)]
pub struct SpinlockT;

const CLK_SET_RATE_PARENT: u32 = 1 << 2;

unsafe extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut SpinlockT, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut SpinlockT, flags: usize);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn clk_register(hw: *mut core::ffi::c_void, init: *mut ClkHw) -> *mut Clk;
    fn is_err<T>(ptr: *mut T) -> bool;
}

unsafe fn to_clk_apmu(hw: *mut ClkHw) -> *mut ClkApmu {
    // Equivalent to container_of(hw, struct clk_apmu, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(ClkApmu, hw)) as *mut ClkApmu
}

unsafe extern "C" fn clk_apmu_enable(hw: *mut ClkHw) -> i32 {
    let apmu = to_clk_apmu(hw);
    let mut data: u32;
    let mut flags: usize = 0;

    if !(*apmu).lock.is_null() {
        spin_lock_irqsave((*apmu).lock, &mut flags);
    }

    data = readl_relaxed((*apmu).base) | (*apmu).enable_mask;
    writel_relaxed(data, (*apmu).base);

    if !(*apmu).lock.is_null() {
        spin_unlock_irqrestore((*apmu).lock, flags);
    }

    0
}

unsafe extern "C" fn clk_apmu_disable(hw: *mut ClkHw) {
    let apmu = to_clk_apmu(hw);
    let data: u32;
    let mut flags: usize = 0;

    if !(*apmu).lock.is_null() {
        spin_lock_irqsave((*apmu).lock, &mut flags);
    }

    data = readl_relaxed((*apmu).base) & !(*apmu).enable_mask;
    writel_relaxed(data, (*apmu).base);

    if !(*apmu).lock.is_null() {
        spin_unlock_irqrestore((*apmu).lock, flags);
    }
}

static CLK_APMU_OPS: ClkOps = ClkOps {
    enable: Some(clk_apmu_enable),
    disable: Some(clk_apmu_disable),
};

#[no_mangle]
pub unsafe extern "C" fn mmp_clk_register_apmu(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    base: *mut core::ffi::c_void,
    enable_mask: u32,
    lock: *mut SpinlockT,
) -> *mut Clk {
    let apmu: *mut ClkApmu = kzalloc_obj();
    let clk: *mut Clk;
    let mut init: ClkInitData;

    if apmu.is_null() {
        return core::ptr::null_mut();
    }

    init = ClkInitData {
        name,
        ops: &CLK_APMU_OPS,
        flags: CLK_SET_RATE_PARENT,
        parent_names: if !parent_name.is_null() {
            &parent_name
        } else {
            core::ptr::null()
        },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*apmu).base = base;
    (*apmu).enable_mask = enable_mask;
    (*apmu).lock = lock;
    (*apmu).hw.init = &mut init;

    clk = clk_register(core::ptr::null_mut(), &mut (*apmu).hw);

    if is_err(clk) {
        kfree(apmu as *mut core::ffi::c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
