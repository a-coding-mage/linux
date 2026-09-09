// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Altera Corporation. All rights reserved
 */

// Dependencies supplied by the surrounding kernel translation.

const SYSMGR_SDMMCGRP_CTRL_OFFSET: u32 = 0x28;

unsafe fn streq(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> bool {
    // C strcmp(a, b) == 0; strcmp is supplied externally.
    unsafe { strcmp(a, b) == 0 }
}

unsafe fn to_socfpga_gate_clk(p: *mut clk_hw) -> *mut socfpga_gate_clk {
    // container_of(p, struct socfpga_gate_clk, hw.hw)
    unsafe { (p as *mut u8).sub(core::mem::offset_of!(socfpga_gate_clk, hw.hw)) as *mut socfpga_gate_clk }
}

unsafe extern "C" {
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(p: *mut core::ffi::c_void);
    fn warn_on(condition: bool) -> bool;
    fn of_property_read_u32_array(node: *mut device_node, name: *const core::ffi::c_char, out: *mut u32, count: usize) -> i32;
    fn of_property_read_u32(node: *mut device_node, name: *const core::ffi::c_char, out: *mut u32) -> i32;
    fn of_property_read_string(node: *mut device_node, name: *const core::ffi::c_char, out: *mut *const core::ffi::c_char) -> i32;
    fn of_clk_parent_fill(node: *mut device_node, parents: *mut *const core::ffi::c_char, max: usize) -> u8;
    fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn of_clk_add_hw_provider(node: *mut device_node, get: unsafe extern "C" fn(*mut device_node, *const *mut clk_hw) -> *mut clk_hw, hw: *mut clk_hw) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

unsafe extern "C" {
    static mut clk_mgr_a10_base_addr: *mut u8;
    static mut gateclk_ops: clk_ops;
    static clk_gate_ops: clk_gate_ops_type;
}

#[repr(C)]
struct clk_ops {
    recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
}

#[repr(C)]
struct clk_gate_ops_type {
    enable: unsafe extern "C" fn(*mut clk_hw) -> i32,
    disable: unsafe extern "C" fn(*mut clk_hw),
}

#[repr(C)]
struct clk_hw { _opaque: [u8; 0] }
#[repr(C)]
struct device { _opaque: [u8; 0] }
#[repr(C)]
struct device_node { name: *const core::ffi::c_char }
#[repr(C)]
struct clk_init_data {
    name: *const core::ffi::c_char,
    ops: *const clk_ops,
    flags: u32,
    num_parents: u8,
    parent_names: *const *const core::ffi::c_char,
}
#[repr(C)]
struct socfpga_gate_clk {
    hw: socfpga_gate_hw,
    fixed_div: u32,
    div_reg: *mut u8,
    shift: u32,
    width: u32,
}
#[repr(C)]
struct socfpga_gate_hw { hw: clk_hw, reg: *mut u8, bit_idx: u32, init: *const clk_init_data }

const SOCFPGA_MAX_PARENTS: usize = 4;

unsafe extern "C" fn socfpga_gate_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let socfpgaclk = unsafe { to_socfpga_gate_clk(hwclk) };
    let mut div: u32 = 1;
    let mut val: u32;

    unsafe {
        if (*socfpgaclk).fixed_div != 0 {
            div = (*socfpgaclk).fixed_div;
        } else if !(*socfpgaclk).div_reg.is_null() {
            val = readl((*socfpgaclk).div_reg as *const core::ffi::c_void) >> (*socfpgaclk).shift;
            val &= (1u32 << (*socfpgaclk).width).wrapping_sub(1);
            div = 1u32 << val;
        }
    }
    parent_rate / div as usize
}

unsafe fn __socfpga_gate_init(node: *mut device_node, ops: *const clk_ops) {
    let mut clk_gate = [0u32; 2];
    let mut div_reg = [0u32; 3];
    let mut fixed_div = 0u32;
    let mut hw_clk: *mut clk_hw;
    let socfpga_clk = unsafe { kzalloc_obj::<socfpga_gate_clk>() };
    let mut clk_name = unsafe { (*node).name };
    let mut parent_name = [core::ptr::null(); SOCFPGA_MAX_PARENTS];
    let mut init: clk_init_data;
    let mut rc: i32;

    if unsafe { warn_on(socfpga_clk.is_null()) } { return; }
    unsafe {
        rc = of_property_read_u32_array(node, b"clk-gate\0".as_ptr() as _, clk_gate.as_mut_ptr(), 2);
        if rc != 0 { clk_gate[0] = 0; }
        if clk_gate[0] != 0 {
            (*socfpga_clk).hw.reg = clk_mgr_a10_base_addr.add(clk_gate[0] as usize);
            (*socfpga_clk).hw.bit_idx = clk_gate[1];
            gateclk_ops.enable = Some(clk_gate_ops.enable);
            gateclk_ops.disable = Some(clk_gate_ops.disable);
        }
        rc = of_property_read_u32(node, b"fixed-divider\0".as_ptr() as _, &mut fixed_div);
        (*socfpga_clk).fixed_div = if rc != 0 { 0 } else { fixed_div };
        rc = of_property_read_u32_array(node, b"div-reg\0".as_ptr() as _, div_reg.as_mut_ptr(), 3);
        if rc == 0 {
            (*socfpga_clk).div_reg = clk_mgr_a10_base_addr.add(div_reg[0] as usize);
            (*socfpga_clk).shift = div_reg[1]; (*socfpga_clk).width = div_reg[2];
        } else { (*socfpga_clk).div_reg = core::ptr::null_mut(); }
        of_property_read_string(node, b"clock-output-names\0".as_ptr() as _, &mut clk_name);
        init = clk_init_data { name: clk_name, ops, flags: 0, num_parents: of_clk_parent_fill(node, parent_name.as_mut_ptr(), SOCFPGA_MAX_PARENTS), parent_names: parent_name.as_ptr() };
        (*socfpga_clk).hw.init = &init;
        hw_clk = &mut (*socfpga_clk).hw.hw;
        rc = clk_hw_register(core::ptr::null_mut(), hw_clk);
        if rc != 0 { pr_err(b"Could not register clock:%s\n\0".as_ptr() as _, clk_name); kfree(socfpga_clk as _); return; }
        rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw_clk);
        if rc != 0 { pr_err(b"Could not register clock provider for node:%s\n\0".as_ptr() as _, clk_name); clk_hw_unregister(hw_clk); kfree(socfpga_clk as _); }
    }
}

pub unsafe extern "C" fn socfpga_a10_gate_init(node: *mut device_node) {
    unsafe { __socfpga_gate_init(node, &gateclk_ops); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
