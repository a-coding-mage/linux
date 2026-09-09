// SPDX-License-Identifier: GPL-2.0
/*
 * MVEBU Core divider clock
 *
 * Copyright (C) 2013 Marvell
 *
 * Ezequiel Garcia <ezequiel.garcia@free-electrons.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const CORE_CLK_DIV_RATIO_MASK: u32 = 0xff;

#[repr(C)]
struct clk_corediv_desc {
    mask: u32,
    offset: u32,
    fieldbit: u32,
}

#[repr(C)]
struct clk_corediv_soc_desc {
    descs: *const clk_corediv_desc,
    ndescs: usize,
    ops: clk_ops,
    ratio_reload: u32,
    enable_bit_offset: u32,
    ratio_offset: usize,
}

#[repr(C)]
struct clk_corediv {
    hw: clk_hw,
    reg: *mut u8,
    desc: *const clk_corediv_desc,
    soc_desc: *const clk_corediv_soc_desc,
    lock: spinlock_t,
}

static mut clk_data: clk_onecell_data = clk_onecell_data { clk_num: 0, clks: core::ptr::null_mut() };

static mvebu_corediv_desc: [clk_corediv_desc; 1] = [
    clk_corediv_desc { mask: 0x3f, offset: 8, fieldbit: 1 }, // NAND clock
];

static mv98dx3236_corediv_desc: [clk_corediv_desc; 1] = [
    clk_corediv_desc { mask: 0x0f, offset: 6, fieldbit: 27 }, // NAND clock
];

unsafe fn clk_corediv_is_enabled(hwclk: *mut clk_hw) -> i32 {
    let corediv = container_of_corediv(hwclk);
    let soc_desc = &*(*corediv).soc_desc;
    let desc = &*(*corediv).desc;
    let enable_mask = (1u32 << desc.fieldbit) << soc_desc.enable_bit_offset;
    if core::ptr::read_volatile((*corediv).reg as *const u32) & enable_mask != 0 { 1 } else { 0 }
}

unsafe fn clk_corediv_enable(hwclk: *mut clk_hw) -> i32 {
    let corediv = container_of_corediv(hwclk);
    let soc_desc = &*(*corediv).soc_desc;
    let desc = &*(*corediv).desc;
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut (*corediv).lock, &mut flags);
    let mut reg = core::ptr::read_volatile((*corediv).reg as *const u32);
    reg |= (1u32 << desc.fieldbit) << soc_desc.enable_bit_offset;
    core::ptr::write_volatile((*corediv).reg as *mut u32, reg);
    spin_unlock_irqrestore(&mut (*corediv).lock, flags);
    0
}

unsafe fn clk_corediv_disable(hwclk: *mut clk_hw) {
    let corediv = container_of_corediv(hwclk);
    let soc_desc = &*(*corediv).soc_desc;
    let desc = &*(*corediv).desc;
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut (*corediv).lock, &mut flags);
    let mut reg = core::ptr::read_volatile((*corediv).reg as *const u32);
    reg &= !((1u32 << desc.fieldbit) << soc_desc.enable_bit_offset);
    core::ptr::write_volatile((*corediv).reg as *mut u32, reg);
    spin_unlock_irqrestore(&mut (*corediv).lock, flags);
}

unsafe fn clk_corediv_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let corediv = container_of_corediv(hwclk);
    let soc_desc = &*(*corediv).soc_desc;
    let desc = &*(*corediv).desc;
    let reg = core::ptr::read_volatile((*corediv).reg.add(soc_desc.ratio_offset) as *const u32);
    let div = (reg >> desc.offset) & desc.mask;
    parent_rate / div as usize
}

unsafe fn clk_corediv_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    // Valid ratio are 1:4, 1:5, 1:6 and 1:8
    let mut div = (*req).best_parent_rate / (*req).rate;
    if div < 4 { div = 4; } else if div > 6 { div = 8; }
    (*req).rate = (*req).best_parent_rate / div;
    0
}

unsafe fn clk_corediv_set_rate(hwclk: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let corediv = container_of_corediv(hwclk);
    let soc_desc = &*(*corediv).soc_desc;
    let desc = &*(*corediv).desc;
    let mut flags: usize = 0;
    let div = parent_rate / rate;
    spin_lock_irqsave(&mut (*corediv).lock, &mut flags);
    let ratio = (*corediv).reg.add(soc_desc.ratio_offset) as *mut u32;
    let mut reg = core::ptr::read_volatile(ratio as *const u32);
    reg &= !(desc.mask << desc.offset);
    reg |= (div as u32 & desc.mask) << desc.offset;
    core::ptr::write_volatile(ratio, reg);
    reg = core::ptr::read_volatile((*corediv).reg as *const u32) | (1u32 << desc.fieldbit);
    core::ptr::write_volatile((*corediv).reg as *mut u32, reg);
    reg = core::ptr::read_volatile((*corediv).reg as *const u32) | soc_desc.ratio_reload;
    core::ptr::write_volatile((*corediv).reg as *mut u32, reg);
    udelay(1000);
    reg &= !(CORE_CLK_DIV_RATIO_MASK | soc_desc.ratio_reload);
    core::ptr::write_volatile((*corediv).reg as *mut u32, reg);
    udelay(1000);
    spin_unlock_irqrestore(&mut (*corediv).lock, flags);
    0
}

// External kernel types and helpers referenced by the translated implementation.
extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn udelay(usecs: u32);
}

unsafe fn container_of_corediv(hw: *mut clk_hw) -> *mut clk_corediv {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_corediv, hw)) as *mut clk_corediv
}

#[repr(C)] struct clk_hw { init: *const clk_init_data }
#[repr(C)] struct clk_init_data { num_parents: u32, parent_names: *const *const i8, name: *const i8, ops: *const clk_ops, flags: u32 }
#[repr(C)] struct clk_rate_request { rate: usize, best_parent_rate: usize }
#[repr(C)] struct clk_onecell_data { clk_num: usize, clks: *mut *mut clk }
#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct spinlock_t { _private: [u8; 0] }
#[repr(C)] struct clk_ops {
    enable: Option<unsafe fn(*mut clk_hw) -> i32>, disable: Option<unsafe fn(*mut clk_hw)>,
    is_enabled: Option<unsafe fn(*mut clk_hw) -> i32>, recalc_rate: Option<unsafe fn(*mut clk_hw, usize) -> usize>,
    determine_rate: Option<unsafe fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    set_rate: Option<unsafe fn(*mut clk_hw, usize, usize) -> i32>,
}

static armada370_corediv_soc: clk_corediv_soc_desc = clk_corediv_soc_desc { descs: mvebu_corediv_desc.as_ptr(), ndescs: 1, ops: corediv_ops(true), ratio_reload: 1 << 8, enable_bit_offset: 24, ratio_offset: 0x8 };
static armada380_corediv_soc: clk_corediv_soc_desc = clk_corediv_soc_desc { descs: mvebu_corediv_desc.as_ptr(), ndescs: 1, ops: corediv_ops(true), ratio_reload: 1 << 8, enable_bit_offset: 16, ratio_offset: 0x4 };
static armada375_corediv_soc: clk_corediv_soc_desc = clk_corediv_soc_desc { descs: mvebu_corediv_desc.as_ptr(), ndescs: 1, ops: corediv_ops(false), ratio_reload: 1 << 8, enable_bit_offset: 0, ratio_offset: 0x4 };
static mv98dx3236_corediv_soc: clk_corediv_soc_desc = clk_corediv_soc_desc { descs: mv98dx3236_corediv_desc.as_ptr(), ndescs: 1, ops: corediv_ops(false), ratio_reload: 1 << 10, enable_bit_offset: 0, ratio_offset: 0x8 };

const fn corediv_ops(with_enable: bool) -> clk_ops { clk_ops { enable: if with_enable { Some(clk_corediv_enable) } else { None }, disable: if with_enable { Some(clk_corediv_disable) } else { None }, is_enabled: if with_enable { Some(clk_corediv_is_enabled) } else { None }, recalc_rate: Some(clk_corediv_recalc_rate), determine_rate: Some(clk_corediv_determine_rate), set_rate: Some(clk_corediv_set_rate) } }

unsafe fn mvebu_corediv_clk_init(_node: *mut device_node, _soc_desc: *const clk_corediv_soc_desc) { }
unsafe fn armada370_corediv_clk_init(node: *mut device_node) { mvebu_corediv_clk_init(node, &armada370_corediv_soc); }
unsafe fn armada375_corediv_clk_init(node: *mut device_node) { mvebu_corediv_clk_init(node, &armada375_corediv_soc); }
unsafe fn armada380_corediv_clk_init(node: *mut device_node) { mvebu_corediv_clk_init(node, &armada380_corediv_soc); }
unsafe fn mv98dx3236_corediv_clk_init(node: *mut device_node) { mvebu_corediv_clk_init(node, &mv98dx3236_corediv_soc); }

#[repr(C)] struct device_node { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
