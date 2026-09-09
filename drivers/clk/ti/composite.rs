// SPDX-License-Identifier: GPL-2.0-only
/* TI composite clock support */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided types and functions are declared externally by the surrounding tree.
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct clk_parent_data { pub index: c_uint }
#[repr(C)] pub struct clk_rate_request { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    static ti_clk_divider_ops: clk_ops;
    static ti_clk_mux_ops: clk_ops;
    fn omap2_dflt_clk_enable(hw: *mut clk_hw) -> c_int;
    fn omap2_dflt_clk_disable(hw: *mut clk_hw);
    fn omap2_dflt_clk_is_enabled(hw: *mut clk_hw) -> c_int;
    fn of_parse_phandle_with_args(node: *mut device_node, prop: *const c_char, cells: *const c_char, i: c_int, out: *mut of_phandle_args) -> c_int;
    fn ti_clk_retry_init(node: *mut device_node, hw: *mut clk_hw, f: unsafe extern "C" fn(*mut c_void, *mut device_node));
    fn ti_dt_clk_name(node: *mut device_node) -> *const c_char;
    fn clk_register_composite_pdata(parent: *mut c_void, name: *const c_char, parents: *mut clk_parent_data, n: c_int, mux: *mut clk_hw, mux_ops: *const clk_ops, div: *mut clk_hw, div_ops: *const clk_ops, gate: *mut clk_hw, gate_ops: *const clk_ops, flags: c_ulong) -> *mut clk;
    fn ti_clk_add_alias(clk: *mut clk, name: *const c_char) -> c_int;
    fn clk_unregister(clk: *mut clk);
    fn of_clk_add_provider(node: *mut device_node, get: *const c_void, data: *mut clk);
    fn of_clk_src_simple_get(_: *mut device_node, _: *const c_void) -> *mut clk;
    fn of_clk_get_parent_count(node: *mut device_node) -> c_uint;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
}

#[repr(C)] struct of_phandle_args { pub np: *mut device_node, _rest: [u8; 0] }
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const CLK_COMPONENT_TYPE_MAX: usize = 3;
const GFP_KERNEL: c_uint = 0;

unsafe extern "C" fn ti_composite_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    (ti_clk_divider_ops.recalc_rate.unwrap())(hw, parent_rate)
}
unsafe extern "C" fn ti_composite_determine_rate(_: *mut clk_hw, _: *mut clk_rate_request) -> c_int { -EINVAL }
unsafe extern "C" fn ti_composite_set_rate(_: *mut clk_hw, _: c_ulong, _: c_ulong) -> c_int { -EINVAL }

static TI_COMPOSITE_DIVIDER_OPS: clk_ops = clk_ops { recalc_rate: Some(ti_composite_recalc_rate), determine_rate: Some(ti_composite_determine_rate), set_rate: Some(ti_composite_set_rate), enable: None, disable: None, is_enabled: None };
static TI_COMPOSITE_GATE_OPS: clk_ops = clk_ops { recalc_rate: None, determine_rate: None, set_rate: None, enable: Some(omap2_dflt_clk_enable), disable: Some(omap2_dflt_clk_disable), is_enabled: Some(omap2_dflt_clk_is_enabled) };

#[repr(C)] struct component_clk { num_parents: c_int, parent_data: *mut clk_parent_data, node: *mut device_node, typ: c_int, hw: *mut clk_hw, link: list_head }
static COMPONENT_CLK_TYPES: [&[u8]; 3] = [b"gate\0", b"divider\0", b"mux\0"];
static mut COMPONENT_CLKS: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn get_component_node(node: *mut device_node, i: c_int) -> *mut device_node { let mut a = of_phandle_args { np: core::ptr::null_mut(), _rest: [] }; if of_parse_phandle_with_args(node, b"clocks\0".as_ptr() as _, b"#clock-cells\0".as_ptr() as _, i, &mut a) != 0 { core::ptr::null_mut() } else { a.np } }
unsafe fn lookup_component(_: *mut device_node) -> *mut component_clk { core::ptr::null_mut() }

#[repr(C)] struct clk_hw_omap_comp { hw: clk_hw, comp_nodes: [*mut device_node; 3], comp_clks: [*mut component_clk; 3] }
unsafe fn get_hw(clk: *mut clk_hw_omap_comp, idx: usize) -> *mut clk_hw { if clk.is_null() || (*clk).comp_clks[idx].is_null() { core::ptr::null_mut() } else { (*(*clk).comp_clks[idx]).hw } }

unsafe extern "C" fn register_composite(user: *mut c_void, node: *mut device_node) {
    let cclk = user as *mut clk_hw_omap_comp; let mut num_parents = 0; let mut parent_data = core::ptr::null_mut();
    for i in 0..CLK_COMPONENT_TYPE_MAX { if (*cclk).comp_nodes[i].is_null() { continue; } let comp = lookup_component((*cclk).comp_nodes[i]); if comp.is_null() { return; } let t = (*comp).typ as usize; if !(*cclk).comp_clks[t].is_null() { break; } (*cclk).comp_clks[t] = comp; (*cclk).comp_nodes[i] = core::ptr::null_mut(); }
    for i in (0..CLK_COMPONENT_TYPE_MAX).rev() { let comp = (*cclk).comp_clks[i]; if !comp.is_null() && (*comp).num_parents != 0 { num_parents = (*comp).num_parents; parent_data = (*comp).parent_data; break; } }
    if num_parents == 0 { kfree(cclk as _); return; }
    let name = ti_dt_clk_name(node); let clk = clk_register_composite_pdata(core::ptr::null_mut(), name, parent_data, num_parents, get_hw(cclk, 2), &ti_clk_mux_ops, get_hw(cclk, 1), &TI_COMPOSITE_DIVIDER_OPS, get_hw(cclk, 0), &TI_COMPOSITE_GATE_OPS, 0);
    if !clk.is_null() { if ti_clk_add_alias(clk, name) != 0 { clk_unregister(clk); } else { of_clk_add_provider(node, of_clk_src_simple_get as *const c_void, clk); } }
    for i in 0..CLK_COMPONENT_TYPE_MAX { let comp = (*cclk).comp_clks[i]; if !comp.is_null() { list_del(&mut (*comp).link); kfree((*comp).parent_data as _); kfree(comp as _); } } kfree(cclk as _);
}

unsafe extern "C" fn of_ti_composite_clk_setup(node: *mut device_node) { let n = of_clk_get_parent_count(node); if n == 0 { return; } let c = kzalloc(core::mem::size_of::<clk_hw_omap_comp>(), GFP_KERNEL) as *mut clk_hw_omap_comp; if c.is_null() { return; } for i in 0..n as usize { (*c).comp_nodes[i] = get_component_node(node, i as c_int); } register_composite(&mut (*c).hw as *mut _ as _, node); }

pub unsafe extern "C" fn ti_clk_add_component(node: *mut device_node, hw: *mut clk_hw, typ: c_int) -> c_int { let n = of_clk_get_parent_count(node); if n == 0 { return -EINVAL; } let p = kcalloc(n as usize, core::mem::size_of::<clk_parent_data>(), GFP_KERNEL) as *mut clk_parent_data; if p.is_null() { return -ENOMEM; } for i in 0..n { (*p.add(i as usize)).index = i; } let c = kzalloc(core::mem::size_of::<component_clk>(), GFP_KERNEL) as *mut component_clk; if c.is_null() { kfree(p as _); return -ENOMEM; } (*c).num_parents = n as c_int; (*c).parent_data = p; (*c).hw = hw; (*c).node = node; (*c).typ = typ; list_add(&mut (*c).link, &mut COMPONENT_CLKS); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
