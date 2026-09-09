// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 Linaro Ltd.
 * Author: Pi-Cheng Chen <pi-cheng.chen@linaro.org>
 */

use core::ffi::c_void;

// Kernel dependencies supplied by the surrounding translation unit.
#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const u8,
    pub ops: *const clk_ops,
    pub parent_names: *const *const u8,
    pub num_parents: u8,
    pub flags: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut c_void) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
}

#[repr(C)]
pub struct mtk_composite {
    pub name: *const u8,
    pub parent_names: *const *const u8,
    pub num_parents: u8,
    pub flags: u32,
    pub mux_reg: u32,
    pub mux_shift: u8,
    pub mux_width: u8,
    pub id: usize,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw_onecell_data {
    pub hws: *mut *mut clk_hw,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn clk_hw_determine_rate_no_reparent(hw: *mut clk_hw, rate: *mut c_void) -> i32;
    fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn device_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn ptr_err(ptr: *mut c_void) -> i32;
    fn is_err(ptr: *const c_void) -> bool;
    fn is_err_or_null(ptr: *const c_void) -> bool;
}

#[repr(C)]
struct mtk_clk_cpumux {
    hw: clk_hw,
    regmap: *mut regmap,
    reg: u32,
    mask: u32,
    shift: u8,
}

unsafe fn to_mtk_clk_cpumux(hw: *mut clk_hw) -> *mut mtk_clk_cpumux {
    (hw as *mut u8).sub(core::mem::offset_of!(mtk_clk_cpumux, hw)) as *mut mtk_clk_cpumux
}

unsafe extern "C" fn clk_cpumux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_mtk_clk_cpumux(hw);
    let mut val: u32 = 0;
    regmap_read((*mux).regmap, (*mux).reg, &mut val);
    val >>= (*mux).shift;
    val &= (*mux).mask;
    val as u8
}

unsafe extern "C" fn clk_cpumux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux = to_mtk_clk_cpumux(hw);
    let val = (index as u32).wrapping_shl((*mux).shift as u32);
    let mask = (*mux).mask.wrapping_shl((*mux).shift as u32);
    regmap_update_bits((*mux).regmap, (*mux).reg, mask, val)
}

static CLK_CPUMUX_OPS: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(clk_cpumux_get_parent),
    set_parent: Some(clk_cpumux_set_parent),
};

unsafe fn mtk_clk_register_cpumux(
    dev: *mut device,
    mux: *const mtk_composite,
    regmap: *mut regmap,
) -> *mut clk_hw {
    let cpumux = kzalloc(core::mem::size_of::<mtk_clk_cpumux>()) as *mut mtk_clk_cpumux;
    if cpumux.is_null() {
        return (-12isize) as *mut clk_hw;
    }

    let mut init = clk_init_data {
        name: (*mux).name,
        ops: &CLK_CPUMUX_OPS,
        parent_names: (*mux).parent_names,
        num_parents: (*mux).num_parents,
        flags: (*mux).flags,
    };

    (*cpumux).reg = (*mux).mux_reg;
    (*cpumux).shift = (*mux).mux_shift;
    (*cpumux).mask = (1u32.wrapping_shl((*mux).mux_width as u32)).wrapping_sub(1);
    (*cpumux).regmap = regmap;
    (*cpumux).hw.init = &mut init;

    let ret = clk_hw_register(dev, &mut (*cpumux).hw);
    if ret != 0 {
        kfree(cpumux as *mut c_void);
        return (ret as isize) as *mut clk_hw;
    }
    &mut (*cpumux).hw
}

unsafe fn mtk_clk_unregister_cpumux(hw: *mut clk_hw) {
    if hw.is_null() {
        return;
    }
    let cpumux = to_mtk_clk_cpumux(hw);
    clk_hw_unregister(hw);
    kfree(cpumux as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn mtk_clk_register_cpumuxes(
    dev: *mut device,
    node: *mut device_node,
    clks: *const mtk_composite,
    num: i32,
    clk_data: *mut clk_hw_onecell_data,
) -> i32 {
    let regmap = device_node_to_regmap(node);
    if is_err(regmap as *const c_void) {
        return ptr_err(regmap as *mut c_void);
    }

    let mut i = 0;
    while i < num {
        let mux = clks.add(i as usize);
        let slot = (*clk_data).hws.add((*mux).id);
        if !is_err_or_null((*slot) as *const c_void) {
            i += 1;
            continue;
        }
        let hw = mtk_clk_register_cpumux(dev, mux, regmap);
        if is_err(hw as *const c_void) {
            while i > 0 {
                i -= 1;
                let prev = clks.add(i as usize);
                let prev_slot = (*clk_data).hws.add((*prev).id);
                if is_err_or_null((*prev_slot) as *const c_void) {
                    continue;
                }
                mtk_clk_unregister_cpumux(*prev_slot);
                *prev_slot = (-2isize) as *mut clk_hw;
            }
            return ptr_err(hw as *mut c_void);
        }
        *slot = hw;
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mtk_clk_unregister_cpumuxes(
    clks: *const mtk_composite,
    num: i32,
    clk_data: *mut clk_hw_onecell_data,
) {
    let mut i = num;
    while i > 0 {
        i -= 1;
        let mux = clks.add(i as usize);
        let slot = (*clk_data).hws.add((*mux).id);
        if is_err_or_null((*slot) as *const c_void) {
            continue;
        }
        mtk_clk_unregister_cpumux(*slot);
        *slot = (-2isize) as *mut clk_hw;
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
