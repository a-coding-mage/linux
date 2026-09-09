// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2014 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Linux kernel dependencies supplied by other translation units.

const SUN8I_MBUS_ENABLE: u32 = 31;
const SUN8I_MBUS_MUX_SHIFT: u32 = 24;
const SUN8I_MBUS_MUX_MASK: u32 = 0x3;
const SUN8I_MBUS_DIV_SHIFT: u32 = 0;
const SUN8I_MBUS_DIV_WIDTH: u32 = 3;
const SUN8I_MBUS_MAX_PARENTS: u32 = 4;

extern "C" {
    static mut sun8i_a23_mbus_lock: spinlock_t;
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct resource {
    pub start: u64,
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_divider {
    pub hw: clk_hw,
    pub reg: *mut ::core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_gate {
    pub hw: clk_hw,
    pub reg: *mut ::core::ffi::c_void,
    pub bit_idx: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_mux {
    pub hw: clk_hw,
    pub reg: *mut ::core::ffi::c_void,
    pub shift: u8,
    pub mask: u32,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

extern "C" {
    fn of_clk_get_parent_count(node: *mut device_node) -> i32;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut *const ::core::ffi::c_char;
    fn of_io_request_and_map(
        node: *mut device_node,
        index: i32,
        name: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn of_node_full_name(node: *mut device_node) -> *const ::core::ffi::c_char;
    fn kzalloc(size: usize, flags: u32) -> *mut ::core::ffi::c_void;
    fn of_property_read_string(
        node: *mut device_node,
        propname: *const ::core::ffi::c_char,
        out: *mut *const ::core::ffi::c_char,
    ) -> i32;
    fn of_clk_parent_fill(
        node: *mut device_node,
        parents: *mut *const ::core::ffi::c_char,
        num_parents: i32,
    ) -> i32;
    fn clk_register_composite(
        dev: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        parents: *mut *const ::core::ffi::c_char,
        num_parents: i32,
        mux_hw: *mut clk_hw,
        mux_ops: *const ::core::ffi::c_void,
        div_hw: *mut clk_hw,
        div_ops: *const ::core::ffi::c_void,
        gate_hw: *mut clk_hw,
        gate_ops: *const ::core::ffi::c_void,
        flags: u32,
    ) -> *mut clk;
    fn of_clk_add_provider(node: *mut device_node, get: *const ::core::ffi::c_void, clk: *mut clk) -> i32;
    fn clk_unregister(clk: *mut clk);
    fn iounmap(addr: *mut ::core::ffi::c_void);
    fn of_address_to_resource(node: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn resource_size(res: *const resource) -> u64;
    fn release_mem_region(start: u64, size: u64);
    fn kfree(ptr: *mut ::core::ffi::c_void);
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    static clk_mux_ops: ::core::ffi::c_void;
    static clk_divider_ops: ::core::ffi::c_void;
    static clk_gate_ops: ::core::ffi::c_void;
    static of_clk_src_simple_get: ::core::ffi::c_void;
}

const GFP_KERNEL: u32 = 0;
const CLK_IS_CRITICAL: u32 = 1 << 11;

#[inline(never)]
pub unsafe extern "C" fn sun8i_a23_mbus_setup(node: *mut device_node) {
    let num_parents = of_clk_get_parent_count(node);
    let mut parents: *mut *const ::core::ffi::c_char = kcalloc(
        num_parents as usize,
        core::mem::size_of::<*const ::core::ffi::c_char>(),
        GFP_KERNEL,
    );
    let mut clk_name = (*node).name;
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    let mut div: *mut clk_divider;
    let mut gate: *mut clk_gate;
    let mut mux: *mut clk_mux;
    let clk: *mut clk;
    let reg: *mut ::core::ffi::c_void;
    let err: i32;

    if parents.is_null() { return; }

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg as isize == -1 {
        pr_err(b"Could not get registers for sun8i-mbus-clk\0".as_ptr() as _);
        goto_err_free_parents(parents);
        return;
    }

    div = kzalloc(core::mem::size_of::<clk_divider>(), GFP_KERNEL) as *mut clk_divider;
    if div.is_null() { goto_err_unmap(reg, node, res.as_mut_ptr(), parents); return; }
    mux = kzalloc(core::mem::size_of::<clk_mux>(), GFP_KERNEL) as *mut clk_mux;
    if mux.is_null() { kfree(div as _); goto_err_unmap(reg, node, res.as_mut_ptr(), parents); return; }
    gate = kzalloc(core::mem::size_of::<clk_gate>(), GFP_KERNEL) as *mut clk_gate;
    if gate.is_null() { kfree(mux as _); kfree(div as _); goto_err_unmap(reg, node, res.as_mut_ptr(), parents); return; }

    of_property_read_string(node, b"clock-output-names\0".as_ptr() as _, &mut clk_name);
    of_clk_parent_fill(node, parents, num_parents);

    (*gate).reg = reg; (*gate).bit_idx = SUN8I_MBUS_ENABLE as u8; (*gate).lock = &mut sun8i_a23_mbus_lock;
    (*div).reg = reg; (*div).shift = SUN8I_MBUS_DIV_SHIFT as u8; (*div).width = SUN8I_MBUS_DIV_WIDTH as u8; (*div).lock = &mut sun8i_a23_mbus_lock;
    (*mux).reg = reg; (*mux).shift = SUN8I_MBUS_MUX_SHIFT as u8; (*mux).mask = SUN8I_MBUS_MUX_MASK; (*mux).lock = &mut sun8i_a23_mbus_lock;

    clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents, num_parents, &mut (*mux).hw, &clk_mux_ops, &mut (*div).hw, &clk_divider_ops, &mut (*gate).hw, &clk_gate_ops, CLK_IS_CRITICAL);
    if clk as isize == -1 { kfree(gate as _); kfree(mux as _); kfree(div as _); goto_err_unmap(reg, node, res.as_mut_ptr(), parents); return; }
    err = of_clk_add_provider(node, &of_clk_src_simple_get, clk);
    if err != 0 { clk_unregister(clk); kfree(gate as _); kfree(mux as _); kfree(div as _); goto_err_unmap(reg, node, res.as_mut_ptr(), parents); return; }
    kfree(parents as _);
}

unsafe fn goto_err_free_parents(parents: *mut *const ::core::ffi::c_char) { kfree(parents as _); }
unsafe fn goto_err_unmap(reg: *mut ::core::ffi::c_void, node: *mut device_node, res: *mut resource, parents: *mut *const ::core::ffi::c_char) {
    iounmap(reg); of_address_to_resource(node, 0, res); release_mem_region((*res).start, resource_size(res)); kfree(parents as _);
}

// CLK_OF_DECLARE(sun8i_a23_mbus, "allwinner,sun8i-a23-mbus-clk", sun8i_a23_mbus_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
