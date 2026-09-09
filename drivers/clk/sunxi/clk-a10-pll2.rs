// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 * Emilio López <emilio@elopez.com.ar>
 *
 * Copyright 2015 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Kernel, device-tree, and clock-provider declarations are supplied externally.

const SUN4I_PLL2_ENABLE: u32 = 31;
const SUN4I_PLL2_PRE_DIV_SHIFT: u32 = 0;
const SUN4I_PLL2_PRE_DIV_WIDTH: u32 = 5;
const SUN4I_PLL2_PRE_DIV_MASK: u32 = (1u32 << SUN4I_PLL2_PRE_DIV_WIDTH) - 1;
const SUN4I_PLL2_N_SHIFT: u32 = 8;
const SUN4I_PLL2_N_WIDTH: u32 = 7;
const SUN4I_PLL2_N_MASK: u32 = (1u32 << SUN4I_PLL2_N_WIDTH) - 1;
const SUN4I_PLL2_POST_DIV_SHIFT: u32 = 26;
const SUN4I_PLL2_POST_DIV_WIDTH: u32 = 4;
const SUN4I_PLL2_POST_DIV_MASK: u32 = (1u32 << SUN4I_PLL2_POST_DIV_WIDTH) - 1;
const SUN4I_PLL2_POST_DIV_VALUE: u32 = 4;
const SUN4I_PLL2_OUTPUTS: usize = 4;

extern "C" {
    static mut sun4i_a10_pll2_lock: core::ffi::c_void;
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_onecell_data { pub clks: *mut *mut clk, pub clk_num: u32 }
#[repr(C)] pub struct clk_multiplier { pub hw: core::ffi::c_void, pub reg: *mut core::ffi::c_void, pub shift: u32, pub width: u32, pub flags: u32, pub lock: *mut core::ffi::c_void }
#[repr(C)] pub struct clk_gate { pub hw: core::ffi::c_void, pub reg: *mut core::ffi::c_void, pub bit_idx: u32, pub lock: *mut core::ffi::c_void }
extern "C" {
    fn of_io_request_and_map(n: *mut device_node, index: i32, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn of_node_full_name(n: *mut device_node) -> *const core::ffi::c_char;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn iounmap(p: *mut core::ffi::c_void);
    fn of_clk_get_parent_name(n: *mut device_node, index: i32) -> *const core::ffi::c_char;
    fn clk_register_divider(a: *mut core::ffi::c_void, name: *const core::ffi::c_char, parent: *const core::ffi::c_char, flags: u32, reg: *mut core::ffi::c_void, shift: u32, width: u32, div_flags: u32, lock: *mut core::ffi::c_void) -> *mut clk;
    fn clk_register_composite(a: *mut core::ffi::c_void, name: *const core::ffi::c_char, parent: *const *const core::ffi::c_char, parents: usize, mux: *mut core::ffi::c_void, mux_ops: *mut core::ffi::c_void, mult: *mut core::ffi::c_void, mult_ops: *mut core::ffi::c_void, gate: *mut core::ffi::c_void, gate_ops: *mut core::ffi::c_void, flags: u32) -> *mut clk;
    fn clk_register_fixed_factor(a: *mut core::ffi::c_void, name: *const core::ffi::c_char, parent: *const core::ffi::c_char, flags: u32, mult: u32, div: u32) -> *mut clk;
    fn clk_unregister_divider(c: *mut clk);
    fn __clk_get_name(c: *mut clk) -> *const core::ffi::c_char;
    fn readl(reg: *mut core::ffi::c_void) -> u32;
    fn writel(val: u32, reg: *mut core::ffi::c_void);
    fn of_property_read_string_index(n: *mut device_node, name: *const core::ffi::c_char, index: u32, out: *mut *const core::ffi::c_char) -> i32;
    fn of_clk_add_provider(n: *mut device_node, get: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32;
    fn IS_ERR(p: *mut core::ffi::c_void) -> bool;
    fn pr_err(s: *const core::ffi::c_char);
    fn WARN_ON(v: bool) -> bool;
}

const CLK_DIVIDER_ONE_BASED: u32 = 1 << 0;
const CLK_DIVIDER_ALLOW_ZERO: u32 = 1 << 1;
const CLK_MULTIPLIER_ZERO_BYPASS: u32 = 1 << 0;
const CLK_MULTIPLIER_ROUND_CLOSEST: u32 = 1 << 1;
const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const SUN4I_A10_PLL2_1X: u32 = 0;
const SUN4I_A10_PLL2_2X: u32 = 1;
const SUN4I_A10_PLL2_4X: u32 = 2;
const SUN4I_A10_PLL2_8X: u32 = 3;

unsafe fn sun4i_pll2_setup(node: *mut device_node, post_div_offset: u32) {
    let mut clk_name = *(node as *mut *const core::ffi::c_char);
    let mut parent: *const core::ffi::c_char;
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) { return; }
    let clk_data = kzalloc(core::mem::size_of::<clk_onecell_data>(), 0) as *mut clk_onecell_data;
    if clk_data.is_null() { iounmap(reg); return; }
    let clks = kzalloc(core::mem::size_of::<*mut clk>() * SUN4I_PLL2_OUTPUTS, 0) as *mut *mut clk;
    if clks.is_null() { kfree(clk_data as *mut _); iounmap(reg); return; }
    parent = of_clk_get_parent_name(node, 0);
    let prediv = clk_register_divider(core::ptr::null_mut(), b"pll2-prediv\0".as_ptr() as _, parent, 0, reg, SUN4I_PLL2_PRE_DIV_SHIFT, SUN4I_PLL2_PRE_DIV_WIDTH, CLK_DIVIDER_ONE_BASED | CLK_DIVIDER_ALLOW_ZERO, &mut sun4i_a10_pll2_lock);
    if IS_ERR(prediv as *mut _) { pr_err(b"Couldn't register the prediv clock\0".as_ptr() as _); kfree(clks as _); kfree(clk_data as _); iounmap(reg); return; }
    let gate = kzalloc(core::mem::size_of::<clk_gate>(), 0) as *mut clk_gate;
    if gate.is_null() { clk_unregister_divider(prediv); kfree(clks as _); kfree(clk_data as _); iounmap(reg); return; }
    (*gate).reg = reg; (*gate).bit_idx = SUN4I_PLL2_ENABLE; (*gate).lock = &mut sun4i_a10_pll2_lock;
    let mult = kzalloc(core::mem::size_of::<clk_multiplier>(), 0) as *mut clk_multiplier;
    if mult.is_null() { kfree(gate as _); clk_unregister_divider(prediv); kfree(clks as _); kfree(clk_data as _); iounmap(reg); return; }
    (*mult).reg = reg; (*mult).shift = SUN4I_PLL2_N_SHIFT; (*mult).width = 7; (*mult).flags = CLK_MULTIPLIER_ZERO_BYPASS | CLK_MULTIPLIER_ROUND_CLOSEST; (*mult).lock = &mut sun4i_a10_pll2_lock;
    parent = __clk_get_name(prediv);
    let base = clk_register_composite(core::ptr::null_mut(), b"pll2-base\0".as_ptr() as _, &parent, 1, core::ptr::null_mut(), core::ptr::null_mut(), &mut (*mult).hw, core::ptr::null_mut(), &mut (*gate).hw, core::ptr::null_mut(), CLK_SET_RATE_PARENT);
    if IS_ERR(base as *mut _) { pr_err(b"Couldn't register the base multiplier clock\0".as_ptr() as _); kfree(mult as _); kfree(gate as _); clk_unregister_divider(prediv); kfree(clks as _); kfree(clk_data as _); iounmap(reg); return; }
    parent = __clk_get_name(base);
    let mut val = readl(reg); val &= !(SUN4I_PLL2_POST_DIV_MASK << SUN4I_PLL2_POST_DIV_SHIFT); val |= (SUN4I_PLL2_POST_DIV_VALUE - post_div_offset) << SUN4I_PLL2_POST_DIV_SHIFT; writel(val, reg);
    let names = [SUN4I_A10_PLL2_1X, SUN4I_A10_PLL2_2X, SUN4I_A10_PLL2_4X, SUN4I_A10_PLL2_8X];
    let factors = [(1, 4), (1, 2), (1, 1), (2, 1)];
    for i in 0..SUN4I_PLL2_OUTPUTS { of_property_read_string_index(node, b"clock-output-names\0".as_ptr() as _, names[i], &mut clk_name); *clks.add(i) = clk_register_fixed_factor(core::ptr::null_mut(), clk_name, parent, CLK_SET_RATE_PARENT, factors[i].0, factors[i].1); WARN_ON(IS_ERR(*clks.add(i) as *mut _)); }
    (*clk_data).clks = clks; (*clk_data).clk_num = SUN4I_PLL2_OUTPUTS as u32; of_clk_add_provider(node, core::ptr::null_mut(), clk_data as _);
}

unsafe fn sun4i_a10_pll2_setup(node: *mut device_node) { sun4i_pll2_setup(node, 0); }
unsafe fn sun5i_a13_pll2_setup(node: *mut device_node) { sun4i_pll2_setup(node, 1); }

// CLK_OF_DECLARE(sun4i_a10_pll2, "allwinner,sun4i-a10-pll2-clk", sun4i_a10_pll2_setup);
// CLK_OF_DECLARE(sun5i_a13_pll2, "allwinner,sun5i-a13-pll2-clk", sun5i_a13_pll2_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
