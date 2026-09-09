// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 */

// Linux dependencies: bitops, clk-provider, io, of, of_address, slab, spinlock.
// The berlin2-div.h declarations are supplied by the surrounding translation.

const PLL_SELECT_MASK: u32 = 0x7;
const DIV_SELECT_MASK: u32 = 0x7;

#[repr(C)]
pub struct berlin2_div {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub map: berlin2_div_map,
    pub lock: *mut spinlock_t,
}

// Clock dividers in Berlin2 SoCs comprise a complex cell to select input pll and divider.
// The virtual structure and control-signal variants are documented in the original C source.

static mut CLK_DIV: [u8; 8] = [1, 2, 4, 6, 8, 12, 1, 1];

unsafe fn berlin2_div_is_enabled(hw: *mut clk_hw) -> i32 {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let mut reg: u32;

    if !div.lock.is_null() { spin_lock(div.lock); }
    reg = readl_relaxed((div.base as *mut u8).add(map.gate_offs as usize) as *const u32);
    reg >>= map.gate_shift;
    if !div.lock.is_null() { spin_unlock(div.lock); }
    (reg & 0x1) as i32
}

unsafe fn berlin2_div_enable(hw: *mut clk_hw) -> i32 {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let mut reg: u32;
    if !div.lock.is_null() { spin_lock(div.lock); }
    reg = readl_relaxed((div.base as *mut u8).add(map.gate_offs as usize) as *const u32);
    reg |= 1u32 << map.gate_shift;
    writel_relaxed(reg, (div.base as *mut u8).add(map.gate_offs as usize) as *mut u32);
    if !div.lock.is_null() { spin_unlock(div.lock); }
    0
}

unsafe fn berlin2_div_disable(hw: *mut clk_hw) {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let mut reg: u32;
    if !div.lock.is_null() { spin_lock(div.lock); }
    reg = readl_relaxed((div.base as *mut u8).add(map.gate_offs as usize) as *const u32);
    reg &= !(1u32 << map.gate_shift);
    writel_relaxed(reg, (div.base as *mut u8).add(map.gate_offs as usize) as *mut u32);
    if !div.lock.is_null() { spin_unlock(div.lock); }
}

unsafe fn berlin2_div_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let mut reg: u32;
    if !div.lock.is_null() { spin_lock(div.lock); }
    // index == 0 is PLL_SWITCH
    reg = readl_relaxed((div.base as *mut u8).add(map.pll_switch_offs as usize) as *const u32);
    if index == 0 { reg &= !(1u32 << map.pll_switch_shift); } else { reg |= 1u32 << map.pll_switch_shift; }
    writel_relaxed(reg, (div.base as *mut u8).add(map.pll_switch_offs as usize) as *mut u32);
    // index > 0 is PLL_SELECT
    if index > 0 {
        reg = readl_relaxed((div.base as *mut u8).add(map.pll_select_offs as usize) as *const u32);
        reg &= !(PLL_SELECT_MASK << map.pll_select_shift);
        reg |= ((index - 1) as u32) << map.pll_select_shift;
        writel_relaxed(reg, (div.base as *mut u8).add(map.pll_select_offs as usize) as *mut u32);
    }
    if !div.lock.is_null() { spin_unlock(div.lock); }
    0
}

unsafe fn berlin2_div_get_parent(hw: *mut clk_hw) -> u8 {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let mut reg: u32;
    let mut index: u8 = 0;
    if !div.lock.is_null() { spin_lock(div.lock); }
    // PLL_SWITCH == 0 is index 0
    reg = readl_relaxed((div.base as *mut u8).add(map.pll_switch_offs as usize) as *const u32);
    reg &= 1u32 << map.pll_switch_shift;
    if reg != 0 {
        reg = readl_relaxed((div.base as *mut u8).add(map.pll_select_offs as usize) as *const u32);
        reg >>= map.pll_select_shift;
        reg &= PLL_SELECT_MASK;
        index = (1 + reg) as u8;
    }
    if !div.lock.is_null() { spin_unlock(div.lock); }
    index
}

unsafe fn berlin2_div_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let div = &mut *((hw as *mut u8).sub(core::mem::offset_of!(berlin2_div, hw)) as *mut berlin2_div);
    let map = &div.map;
    let (mut divsw, mut div3sw): (u32, u32);
    let mut divider: u32 = 1;
    if !div.lock.is_null() { spin_lock(div.lock); }
    divsw = readl_relaxed((div.base as *mut u8).add(map.div_switch_offs as usize) as *const u32) & (1u32 << map.div_switch_shift);
    div3sw = readl_relaxed((div.base as *mut u8).add(map.div3_switch_offs as usize) as *const u32) & (1u32 << map.div3_switch_shift);
    // constant divide-by-3 (dominant)
    if div3sw != 0 { divider = 3; }
    // divider can be bypassed with DIV_SWITCH == 0
    else if divsw == 0 { divider = 1; }
    // clock divider determined by DIV_SELECT
    else {
        let mut reg = readl_relaxed((div.base as *mut u8).add(map.div_select_offs as usize) as *const u32);
        reg >>= map.div_select_shift;
        reg &= DIV_SELECT_MASK;
        divider = CLK_DIV[reg as usize] as u32;
    }
    if !div.lock.is_null() { spin_unlock(div.lock); }
    parent_rate / divider as u64
}

// External kernel types and functions are supplied by the surrounding translation.
extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn readl_relaxed(addr: *const u32) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u32);
}

#[repr(C)]
pub struct clk_ops {
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

static BERLIN2_DIV_RATE_OPS: clk_ops = clk_ops {
    determine_rate: None, // clk_hw_determine_rate_no_reparent
    recalc_rate: Some(berlin2_div_recalc_rate),
    is_enabled: None, enable: None, disable: None, set_parent: None, get_parent: None,
};
static BERLIN2_DIV_GATE_OPS: clk_ops = clk_ops {
    determine_rate: None, recalc_rate: None,
    is_enabled: Some(berlin2_div_is_enabled), enable: Some(berlin2_div_enable),
    disable: Some(berlin2_div_disable), set_parent: None, get_parent: None,
};
static BERLIN2_DIV_MUX_OPS: clk_ops = clk_ops {
    determine_rate: None, recalc_rate: None, is_enabled: None, enable: None, disable: None,
    set_parent: Some(berlin2_div_set_parent), get_parent: Some(berlin2_div_get_parent),
};

pub unsafe fn berlin2_div_register(
    map: *const berlin2_div_map,
    base: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
    div_flags: u8,
    parent_names: *const *const core::ffi::c_char,
    num_parents: i32,
    flags: u32,
    lock: *mut spinlock_t,
) -> *mut clk_hw {
    let mut mux_ops: *const clk_ops = &BERLIN2_DIV_MUX_OPS;
    let rate_ops: *const clk_ops = &BERLIN2_DIV_RATE_OPS;
    let mut gate_ops: *const clk_ops = &BERLIN2_DIV_GATE_OPS;
    let div = kzalloc_berlin2_div();
    if div.is_null() { return core::ptr::null_mut(); } // ERR_PTR(-ENOMEM)

    // copy div_map to allow __initconst
    core::ptr::copy_nonoverlapping(map, &mut (*div).map, 1);
    (*div).base = base;
    (*div).lock = lock;
    if (div_flags & BERLIN2_DIV_HAS_GATE) == 0 { gate_ops = core::ptr::null(); }
    if (div_flags & BERLIN2_DIV_HAS_MUX) == 0 { mux_ops = core::ptr::null(); }
    clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
        &mut (*div).hw, mux_ops, &mut (*div).hw, rate_ops, &mut (*div).hw, gate_ops, flags)
}

extern "C" {
    fn kzalloc_berlin2_div() -> *mut berlin2_div;
    fn clk_hw_register_composite(
        dev: *mut core::ffi::c_void, name: *const core::ffi::c_char,
        parent_names: *const *const core::ffi::c_char, num_parents: i32,
        mux_hw: *mut clk_hw, mux_ops: *const clk_ops, rate_hw: *mut clk_hw,
        rate_ops: *const clk_ops, gate_hw: *mut clk_hw, gate_ops: *const clk_ops,
        flags: u32,
    ) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
