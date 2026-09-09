// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum gate clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependencies supplied by the surrounding clock-provider and gate definitions.

extern "C" {
    fn regmap_read(regmap: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(regmap: *mut regmap, reg: u32, val: u32) -> i32;
    fn udelay(usecs: u32);
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_is_enabled(hw: *mut clk_hw) -> bool;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sprd_clk_common {
    pub regmap: *mut regmap,
    pub reg: u32,
}

#[repr(C)]
pub struct sprd_gate {
    pub common: sprd_clk_common,
    pub flags: u32,
    pub enable_mask: u32,
    pub sc_offset: u32,
    pub udelay: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

extern "C" {
    fn hw_to_sprd_gate(hw: *mut clk_hw) -> *mut sprd_gate;
}

const CLK_GATE_SET_TO_DISABLE: u32 = 1 << 0;
const SPRD_GATE_NON_AON: u32 = 1 << 1;

unsafe fn clk_gate_toggle(sg: *const sprd_gate, en: bool) {
    let common = &(*sg).common;
    let mut reg: u32 = 0;
    let mut set = if (*sg).flags & CLK_GATE_SET_TO_DISABLE != 0 {
        true
    } else {
        false
    };

    set ^= en;

    regmap_read(common.regmap, common.reg, &mut reg);

    if set {
        reg |= (*sg).enable_mask;
    } else {
        reg &= !(*sg).enable_mask;
    }

    regmap_write(common.regmap, common.reg, reg);
}

unsafe fn clk_sc_gate_toggle(sg: *const sprd_gate, en: bool) {
    let common = &(*sg).common;
    let mut set = if (*sg).flags & CLK_GATE_SET_TO_DISABLE != 0 {
        1
    } else {
        0
    };
    let offset: u32;

    set ^= en as i32;

    /*
     * Each set/clear gate clock has three registers:
     * common->reg                    - base register
     * common->reg + offset           - set register
     * common->reg + 2 * offset       - clear register
     */
    offset = if set != 0 {
        (*sg).sc_offset
    } else {
        (*sg).sc_offset * 2
    };

    regmap_write(common.regmap, common.reg + offset, (*sg).enable_mask);
}

unsafe extern "C" fn sprd_gate_disable(hw: *mut clk_hw) {
    let sg = hw_to_sprd_gate(hw);

    clk_gate_toggle(sg, false);
}

unsafe extern "C" fn sprd_gate_enable(hw: *mut clk_hw) -> i32 {
    let sg = hw_to_sprd_gate(hw);

    clk_gate_toggle(sg, true);

    0
}

unsafe extern "C" fn sprd_sc_gate_disable(hw: *mut clk_hw) {
    let sg = hw_to_sprd_gate(hw);

    clk_sc_gate_toggle(sg, false);
}

unsafe extern "C" fn sprd_sc_gate_enable(hw: *mut clk_hw) -> i32 {
    let sg = hw_to_sprd_gate(hw);

    clk_sc_gate_toggle(sg, true);

    0
}

unsafe extern "C" fn sprd_pll_sc_gate_prepare(hw: *mut clk_hw) -> i32 {
    let sg = hw_to_sprd_gate(hw);

    clk_sc_gate_toggle(sg, true);
    udelay((*sg).udelay);

    0
}

unsafe extern "C" fn sprd_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let sg = hw_to_sprd_gate(hw);
    let common = &(*sg).common;
    let parent: *mut clk_hw;
    let mut reg: u32 = 0;

    if (*sg).flags & SPRD_GATE_NON_AON != 0 {
        parent = clk_hw_get_parent(hw);
        if parent.is_null() || !clk_hw_is_enabled(parent) {
            return 0;
        }
    }

    regmap_read(common.regmap, common.reg, &mut reg);

    if (*sg).flags & CLK_GATE_SET_TO_DISABLE != 0 {
        reg ^= (*sg).enable_mask;
    }

    reg &= (*sg).enable_mask;

    if reg != 0 { 1 } else { 0 }
}

#[no_mangle]
pub static sprd_gate_ops: clk_ops = clk_ops {
    disable: Some(sprd_gate_disable),
    enable: Some(sprd_gate_enable),
    is_enabled: Some(sprd_gate_is_enabled),
    unprepare: None,
    prepare: None,
};

#[no_mangle]
pub static sprd_sc_gate_ops: clk_ops = clk_ops {
    disable: Some(sprd_sc_gate_disable),
    enable: Some(sprd_sc_gate_enable),
    is_enabled: Some(sprd_gate_is_enabled),
    unprepare: None,
    prepare: None,
};

#[no_mangle]
pub static sprd_pll_sc_gate_ops: clk_ops = clk_ops {
    disable: None,
    enable: None,
    is_enabled: Some(sprd_gate_is_enabled),
    unprepare: Some(sprd_sc_gate_disable),
    prepare: Some(sprd_pll_sc_gate_prepare),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
