// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_char;

type U8 = u8;
type CInt = i32;
type CUInt = u32;
type CULong = usize;

#[repr(C)]
pub struct ClkHw {
    pub init: *const ClkInitData,
}

#[repr(C)]
pub struct ClkDivider {
    pub hw: ClkHw,
    pub reg: *mut core::ffi::c_void,
    pub shift: U8,
    pub width: U8,
    pub lock: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ClkMux {
    pub hw: ClkHw,
    pub reg: *mut core::ffi::c_void,
    pub shift: U8,
    pub mask: CUInt,
    pub lock: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ClkRateRequest {
    _private: [u8; 0],
}

pub type RecalcRate = unsafe extern "C" fn(*mut ClkHw, CULong) -> CULong;
pub type DetermineRate = unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> CInt;
pub type SetRate = unsafe extern "C" fn(*mut ClkHw, CULong, CULong) -> CInt;
pub type GetParent = unsafe extern "C" fn(*mut ClkHw) -> U8;
pub type SetParent = unsafe extern "C" fn(*mut ClkHw, U8) -> CInt;

#[repr(C)]
pub struct ClkOps {
    pub recalc_rate: Option<RecalcRate>,
    pub determine_rate: Option<DetermineRate>,
    pub set_rate: Option<SetRate>,
    pub get_parent: Option<GetParent>,
    pub set_parent: Option<SetParent>,
}

#[repr(C)]
pub struct ClkInitData {
    pub name: *const c_char,
    pub ops: *const ClkOps,
    pub flags: CUInt,
    pub parent_names: *const *const c_char,
    pub num_parents: CUInt,
}

extern "C" {
    static mut jiffies: CULong;
    static mut imx_ccm_lock: core::ffi::c_void;
    static clk_divider_ops: ClkOps;
    static clk_mux_ops: ClkOps;
    fn clk_hw_determine_rate_no_reparent(hw: *mut ClkHw, req: *mut ClkRateRequest) -> CInt;

    fn msecs_to_jiffies(msecs: CUInt) -> CULong;
    fn readl_relaxed(reg: *mut core::ffi::c_void) -> CUInt;
    fn time_after(a: CULong, b: CULong) -> bool;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut ClkHw) -> CInt;
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[no_mangle]
pub static clk_busy_divider_ops: ClkOps = ClkOps {
    recalc_rate: Some(clk_busy_divider_recalc_rate),
    determine_rate: Some(clk_busy_divider_determine_rate),
    set_rate: Some(clk_busy_divider_set_rate),
    get_parent: None,
    set_parent: None,
};

#[no_mangle]
pub static clk_busy_mux_ops: ClkOps = ClkOps {
    recalc_rate: None,
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    set_rate: None,
    get_parent: Some(clk_busy_mux_get_parent),
    set_parent: Some(clk_busy_mux_set_parent),
};

const ENOMEM: CInt = 12;
const ETIMEDOUT: CInt = 110;
const CLK_SET_RATE_PARENT: CUInt = 1 << 2;
const CLK_IS_CRITICAL: CUInt = 1 << 11;

unsafe fn clk_busy_wait(reg: *mut core::ffi::c_void, shift: U8) -> CInt {
    let timeout = jiffies.wrapping_add(msecs_to_jiffies(10));

    while readl_relaxed(reg) & (1u32 << shift) != 0 {
        if time_after(jiffies, timeout) {
            return -ETIMEDOUT;
        }
    }

    0
}

#[repr(C)]
pub struct ClkBusyDivider {
    pub div: ClkDivider,
    pub div_ops: *const ClkOps,
    pub reg: *mut core::ffi::c_void,
    pub shift: U8,
}

unsafe fn to_clk_busy_divider(hw: *mut ClkHw) -> *mut ClkBusyDivider {
    hw as *mut ClkBusyDivider
}

unsafe extern "C" fn clk_busy_divider_recalc_rate(hw: *mut ClkHw, parent_rate: CULong) -> CULong {
    let busy = &mut *to_clk_busy_divider(hw);
    ((*busy.div_ops).recalc_rate.unwrap())(&mut busy.div.hw, parent_rate)
}

unsafe extern "C" fn clk_busy_divider_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> CInt {
    let busy = &mut *to_clk_busy_divider(hw);
    ((*busy.div_ops).determine_rate.unwrap())(&mut busy.div.hw, req)
}

unsafe extern "C" fn clk_busy_divider_set_rate(hw: *mut ClkHw, rate: CULong, parent_rate: CULong) -> CInt {
    let busy = &mut *to_clk_busy_divider(hw);
    let mut ret = ((*busy.div_ops).set_rate.unwrap())(&mut busy.div.hw, rate, parent_rate);
    if ret == 0 {
        ret = clk_busy_wait(busy.reg, busy.shift);
    }
    ret
}

#[repr(C)]
pub struct ClkBusyMux {
    pub mux: ClkMux,
    pub mux_ops: *const ClkOps,
    pub reg: *mut core::ffi::c_void,
    pub shift: U8,
}

unsafe fn to_clk_busy_mux(hw: *mut ClkHw) -> *mut ClkBusyMux {
    hw as *mut ClkBusyMux
}

unsafe extern "C" fn clk_busy_mux_get_parent(hw: *mut ClkHw) -> U8 {
    let busy = &mut *to_clk_busy_mux(hw);
    ((*busy.mux_ops).get_parent.unwrap())(&mut busy.mux.hw)
}

unsafe extern "C" fn clk_busy_mux_set_parent(hw: *mut ClkHw, index: U8) -> CInt {
    let busy = &mut *to_clk_busy_mux(hw);
    let mut ret = ((*busy.mux_ops).set_parent.unwrap())(&mut busy.mux.hw, index);
    if ret == 0 {
        ret = clk_busy_wait(busy.reg, busy.shift);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_busy_divider(
    name: *const c_char,
    parent_name: *const c_char,
    reg: *mut core::ffi::c_void,
    shift: U8,
    width: U8,
    busy_reg: *mut core::ffi::c_void,
    busy_shift: U8,
) -> *mut ClkHw {
    let busy = Box::new(ClkBusyDivider {
        div: ClkDivider {
            hw: ClkHw { init: core::ptr::null() },
            reg: core::ptr::null_mut(),
            shift: 0,
            width: 0,
            lock: core::ptr::null_mut(),
        },
        div_ops: core::ptr::addr_of!(clk_divider_ops),
        reg: core::ptr::null_mut(),
        shift: 0,
    });
    let busy = Box::into_raw(busy);
    if busy.is_null() {
        return (-ENOMEM as isize) as *mut ClkHw;
    }

    (*busy).reg = busy_reg;
    (*busy).shift = busy_shift;
    (*busy).div.reg = reg;
    (*busy).div.shift = shift;
    (*busy).div.width = width;
    (*busy).div.lock = core::ptr::addr_of_mut!(imx_ccm_lock);

    let init = Box::new(ClkInitData {
        name,
        ops: core::ptr::addr_of!(clk_busy_divider_ops),
        flags: CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
        parent_names: &parent_name,
        num_parents: 1,
    });
    (*busy).div.hw.init = Box::into_raw(init);
    let hw = &mut (*busy).div.hw as *mut ClkHw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(busy as *mut core::ffi::c_void);
        return (ret as isize) as *mut ClkHw;
    }
    hw
}

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_busy_mux(
    name: *const c_char,
    reg: *mut core::ffi::c_void,
    shift: U8,
    width: U8,
    busy_reg: *mut core::ffi::c_void,
    busy_shift: U8,
    parent_names: *const *const c_char,
    num_parents: CInt,
) -> *mut ClkHw {
    let busy = Box::into_raw(Box::new(ClkBusyMux {
        mux: ClkMux {
            hw: ClkHw { init: core::ptr::null() },
            reg: core::ptr::null_mut(),
            shift: 0,
            mask: 0,
            lock: core::ptr::null_mut(),
        },
        mux_ops: core::ptr::addr_of!(clk_mux_ops),
        reg: core::ptr::null_mut(),
        shift: 0,
    }));
    if busy.is_null() {
        return (-ENOMEM as isize) as *mut ClkHw;
    }
    (*busy).reg = busy_reg;
    (*busy).shift = busy_shift;
    (*busy).mux.reg = reg;
    (*busy).mux.shift = shift;
    (*busy).mux.mask = (1u32 << width) - 1;
    (*busy).mux.lock = core::ptr::addr_of_mut!(imx_ccm_lock);

    let init = Box::new(ClkInitData {
        name,
        ops: core::ptr::addr_of!(clk_busy_mux_ops),
        flags: CLK_IS_CRITICAL,
        parent_names,
        num_parents: num_parents as CUInt,
    });
    (*busy).mux.hw.init = Box::into_raw(init);
    let hw = &mut (*busy).mux.hw as *mut ClkHw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(busy as *mut core::ffi::c_void);
        return (ret as isize) as *mut ClkHw;
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
