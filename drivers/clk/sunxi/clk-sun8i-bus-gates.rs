// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Jens Kuske <jenskuske@gmail.com>
 *
 * Based on clk-simple-gates.c, which is:
 * Copyright 2015 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Kernel declarations supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct clk_onecell_data {
    pub clks: *mut *mut clk,
    pub clk_num: c_uint,
}
#[repr(C)]
pub struct resource { pub start: usize }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

extern "C" {
    fn of_io_request_and_map(node: *mut device_node, index: c_int, name: *const c_char) -> *mut c_void;
    fn of_node_full_name(node: *mut device_node) -> *const c_char;
    fn of_property_match_string(node: *mut device_node, property: *const c_char, value: *const c_char) -> c_int;
    fn of_clk_get_parent_name(node: *mut device_node, index: c_int) -> *const c_char;
    fn kmalloc_obj_clk_onecell_data() -> *mut clk_onecell_data;
    fn kzalloc_clk_array(count: usize) -> *mut *mut clk;
    fn of_property_count_u32_elems(node: *mut device_node, property: *const c_char) -> c_int;
    fn of_property_read_u32_index(node: *mut device_node, property: *const c_char, index: c_int, value: *mut c_int) -> c_int;
    fn of_property_read_string_index(node: *mut device_node, property: *const c_char, index: c_int, value: *mut *const c_char) -> c_int;
    fn of_clk_add_provider(node: *mut device_node, get: *const c_void, data: *mut clk_onecell_data) -> c_int;
    fn of_clk_src_onecell_get() -> !;
    fn clk_register_gate(dev: *mut c_void, name: *const c_char, parent: *const c_char,
                         flags: c_uint, reg: *mut c_void, bit: u8, gate_flags: c_uint,
                         lock: *mut spinlock_t) -> *mut clk;
    fn iounmap(addr: *mut c_void);
    fn of_address_to_resource(node: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    fn resource_size(res: *const resource) -> usize;
    fn release_mem_region(start: usize, size: usize);
    fn kfree(ptr: *mut c_void);
    fn warn_on(condition: bool);
    static mut gates_lock: spinlock_t;
}

unsafe fn sun8i_h3_bus_gates_init(node: *mut device_node) {
    static NAMES: [&[u8]; 4] = [b"ahb1\0", b"ahb2\0", b"apb1\0", b"apb2\0"];
    const AHB1: usize = 0;
    const AHB2: usize = 1;
    const APB1: usize = 2;
    const APB2: usize = 3;
    const PARENT_MAX: usize = 4;
    let mut parents: [*const c_char; PARENT_MAX] = [core::ptr::null(); PARENT_MAX];
    let mut res = resource { start: 0 };
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() { return; }

    for i in 0..NAMES.len() {
        let idx = of_property_match_string(node, b"clock-names\0".as_ptr() as *const c_char,
                                           NAMES[i].as_ptr() as *const c_char);
        if idx < 0 { return; }
        parents[i] = of_clk_get_parent_name(node, idx);
    }

    let clk_data = kmalloc_obj_clk_onecell_data();
    if clk_data.is_null() { iounmap(reg); of_address_to_resource(node, 0, &mut res); release_mem_region(res.start, resource_size(&res)); return; }

    let mut number = of_property_count_u32_elems(node, b"clock-indices\0".as_ptr() as *const c_char);
    of_property_read_u32_index(node, b"clock-indices\0".as_ptr() as *const c_char, number - 1, &mut number);
    (*clk_data).clks = kzalloc_clk_array((number + 1) as usize);
    if (*clk_data).clks.is_null() { kfree(clk_data as *mut c_void); iounmap(reg); of_address_to_resource(node, 0, &mut res); release_mem_region(res.start, resource_size(&res)); return; }

    let mut i = 0;
    // Equivalent to of_property_for_each_u32(node, "clock-indices", index).
    let mut index = 0;
    while i < number {
        let mut clk_name: *const c_char = core::ptr::null();
        of_property_read_string_index(node, b"clock-output-names\0".as_ptr() as *const c_char, i, &mut clk_name);
        let clk_parent = if index == 17 || (index >= 29 && index <= 31) { AHB2 }
            else if index <= 63 || index >= 128 { AHB1 }
            else if index >= 64 && index <= 95 { APB1 }
            else if index >= 96 && index <= 127 { APB2 }
            else { warn_on(true); index += 1; continue; };
        let clk_reg = (reg as usize + 4 * (index / 32)) as *mut c_void;
        let clk_bit = (index % 32) as u8;
        *(*clk_data).clks.add(index as usize) = clk_register_gate(core::ptr::null_mut(), clk_name, parents[clk_parent], 0, clk_reg, clk_bit, 0, &mut gates_lock);
        i += 1;
        index += 1;
    }
    (*clk_data).clk_num = (number + 1) as c_uint;
    of_clk_add_provider(node, of_clk_src_onecell_get as *const c_void, clk_data);
}

// CLK_OF_DECLARE(sun8i_h3_bus_gates, "allwinner,sun8i-h3-bus-gates-clk", sun8i_h3_bus_gates_init);
// CLK_OF_DECLARE(sun8i_a83t_bus_gates, "allwinner,sun8i-a83t-bus-gates-clk", sun8i_h3_bus_gates_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
