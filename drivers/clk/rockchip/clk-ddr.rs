// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Rockchip Electronics Co. Ltd.
 * Author: Lin Huang <hl@rock-chips.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Declarations supplied by the surrounding kernel sources.
extern "C" {
    fn arm_smccc_smc(
        a0: c_ulong, a1: c_ulong, a2: c_ulong, a3: c_ulong,
        a4: c_ulong, a5: c_ulong, a6: c_ulong, a7: c_ulong,
        res: *mut arm_smccc_res,
    );
    fn readl(addr: *const c_void) -> u32;
    fn clk_register(parent: *mut clk, hw: *mut clk_hw) -> *mut clk;
    fn kfree(ptr: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub ops: *const clk_ops,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
}

#[repr(C)]
pub struct arm_smccc_res {
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
}

#[repr(C)]
pub struct clk;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

pub const ROCKCHIP_SIP_DRAM_FREQ: c_ulong = 0;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_RATE: c_ulong = 0;
pub const ROCKCHIP_SIP_CONFIG_DRAM_GET_RATE: c_ulong = 0;
pub const ROCKCHIP_SIP_CONFIG_DRAM_ROUND_RATE: c_ulong = 0;
pub const ROCKCHIP_DDRCLK_SIP: c_int = 0;
pub const CLK_SET_RATE_NO_REPARENT: c_ulong = 0;

#[repr(C)]
pub struct rockchip_ddrclk {
    pub hw: clk_hw,
    pub reg_base: *mut c_void,
    pub mux_offset: c_int,
    pub mux_shift: c_int,
    pub mux_width: c_int,
    pub div_shift: c_int,
    pub div_width: c_int,
    pub ddr_flag: c_int,
    pub lock: *mut spinlock_t,
}

unsafe fn to_rockchip_ddrclk_hw(hw: *mut clk_hw) -> *mut rockchip_ddrclk {
    (hw as *mut u8).sub(core::mem::offset_of!(rockchip_ddrclk, hw)) as *mut rockchip_ddrclk
}

unsafe extern "C" fn rockchip_ddrclk_sip_set_rate(
    hw: *mut clk_hw,
    drate: c_ulong,
    _prate: c_ulong,
) -> c_int {
    let ddrclk = &mut *to_rockchip_ddrclk_hw(hw);
    let mut flags: c_ulong = 0;
    let mut res = arm_smccc_res { a0: 0, a1: 0, a2: 0, a3: 0 };

    spin_lock_irqsave(ddrclk.lock, &mut flags);
    arm_smccc_smc(
        ROCKCHIP_SIP_DRAM_FREQ, drate, 0,
        ROCKCHIP_SIP_CONFIG_DRAM_SET_RATE,
        0, 0, 0, 0, &mut res,
    );
    spin_unlock_irqrestore(ddrclk.lock, flags);

    res.a0 as c_int
}

unsafe extern "C" fn rockchip_ddrclk_sip_recalc_rate(
    _hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    let mut res = arm_smccc_res { a0: 0, a1: 0, a2: 0, a3: 0 };

    arm_smccc_smc(
        ROCKCHIP_SIP_DRAM_FREQ, 0, 0,
        ROCKCHIP_SIP_CONFIG_DRAM_GET_RATE,
        0, 0, 0, 0, &mut res,
    );

    res.a0
}

unsafe extern "C" fn rockchip_ddrclk_sip_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let mut res = arm_smccc_res { a0: 0, a1: 0, a2: 0, a3: 0 };

    arm_smccc_smc(
        ROCKCHIP_SIP_DRAM_FREQ, (*req).rate, 0,
        ROCKCHIP_SIP_CONFIG_DRAM_ROUND_RATE,
        0, 0, 0, 0, &mut res,
    );

    (*req).rate = res.a0;
    0
}

unsafe extern "C" fn rockchip_ddrclk_get_parent(hw: *mut clk_hw) -> u8 {
    let ddrclk = &*to_rockchip_ddrclk_hw(hw);
    let mut val = readl(ddrclk.reg_base.add(ddrclk.mux_offset as usize))
        >> ddrclk.mux_shift;
    val &= ((1u32 << (ddrclk.mux_width - 1)) - 1) as u32;
    val as u8
}

pub static rockchip_ddrclk_sip_ops: clk_ops = clk_ops {
    recalc_rate: Some(rockchip_ddrclk_sip_recalc_rate),
    set_rate: Some(rockchip_ddrclk_sip_set_rate),
    determine_rate: Some(rockchip_ddrclk_sip_determine_rate),
    get_parent: Some(rockchip_ddrclk_get_parent),
};

pub unsafe extern "C" fn rockchip_clk_register_ddrclk(
    name: *const c_char,
    flags: c_int,
    parent_names: *const *const c_char,
    num_parents: u8,
    mux_offset: c_int,
    mux_shift: c_int,
    mux_width: c_int,
    div_shift: c_int,
    div_width: c_int,
    ddr_flag: c_int,
    reg_base: *mut c_void,
    lock: *mut spinlock_t,
) -> *mut clk {
    let ddrclk = Box::into_raw(Box::new(core::mem::zeroed::<rockchip_ddrclk>()));
    if ddrclk.is_null() {
        return (-12isize) as *mut clk;
    }

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = flags as c_ulong;
    init.flags |= CLK_SET_RATE_NO_REPARENT;

    match ddr_flag {
        ROCKCHIP_DDRCLK_SIP => init.ops = &rockchip_ddrclk_sip_ops,
        _ => {
            pr_err(b"%s: unsupported ddrclk type %d\n\0".as_ptr() as *const c_char,
                   b"rockchip_clk_register_ddrclk\0".as_ptr() as *const c_char, ddr_flag);
            kfree(ddrclk as *mut c_void);
            return (-22isize) as *mut clk;
        }
    }

    (*ddrclk).reg_base = reg_base;
    (*ddrclk).lock = lock;
    (*ddrclk).hw.init = &init;
    (*ddrclk).mux_offset = mux_offset;
    (*ddrclk).mux_shift = mux_shift;
    (*ddrclk).mux_width = mux_width;
    (*ddrclk).div_shift = div_shift;
    (*ddrclk).div_width = div_width;
    (*ddrclk).ddr_flag = ddr_flag;

    let clk = clk_register(core::ptr::null_mut(), &mut (*ddrclk).hw);
    if (clk as isize) < 0 {
        kfree(ddrclk as *mut c_void);
    }
    clk
}

extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
