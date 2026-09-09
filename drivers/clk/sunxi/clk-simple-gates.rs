// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct clk_onecell_data {
    pub clks: *mut *mut clk,
    pub clk_num: c_int,
}
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct resource { pub start: usize, _rest: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

static mut gates_lock: spinlock_t = spinlock_t { _private: [] };

extern "C" {
    fn of_io_request_and_map(node: *mut device_node, index: c_int,
                             name: *const c_char) -> *mut c_void;
    fn is_err(ptr: *const c_void) -> bool;
    fn of_node_full_name(node: *mut device_node) -> *const c_char;
    fn of_clk_get_parent_name(node: *mut device_node, index: c_int) -> *const c_char;
    fn kmalloc_clk_onecell_data() -> *mut clk_onecell_data;
    fn kfree(ptr: *mut c_void);
    fn of_property_count_u32_elems(node: *mut device_node, name: *const c_char) -> c_int;
    fn of_property_read_u32_index(node: *mut device_node, name: *const c_char,
                                   index: c_int, value: *mut c_int) -> c_int;
    fn kzalloc_clk_array(count: usize) -> *mut *mut clk;
    fn of_property_read_string_index(node: *mut device_node, name: *const c_char,
                                     index: c_int, value: *mut *const c_char) -> c_int;
    fn clk_register_gate(dev: *mut c_void, name: *const c_char, parent: *const c_char,
                         flags: u32, reg: *mut c_void, bit_idx: u8,
                         clk_flags: u8, lock: *mut spinlock_t) -> *mut clk;
    fn warn_on(condition: bool);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn of_clk_add_provider(node: *mut device_node, get: *const c_void, data: *mut clk_onecell_data) -> c_int;
    fn of_clk_src_onecell_get() -> !;
    fn iounmap(addr: *mut c_void);
    fn of_address_to_resource(node: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    fn resource_size(res: *const resource) -> usize;
    fn release_mem_region(start: usize, size: usize);
}

unsafe fn sunxi_simple_gates_setup(node: *mut device_node,
                                   protected: *const c_int, nprotected: c_int) {
    let mut clk_data: *mut clk_onecell_data;
    let clk_parent: *const c_char;
    let clk_name: *const c_char = core::ptr::null();
    let mut res = resource { start: 0, _rest: [] };
    let clk_reg: *mut c_void;
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if is_err(reg) { return; }

    clk_parent = of_clk_get_parent_name(node, 0);
    clk_data = kmalloc_clk_onecell_data();
    if clk_data.is_null() { iounmap(reg); of_address_to_resource(node, 0, &mut res); release_mem_region(res.start, resource_size(&res)); return; }

    let mut number = of_property_count_u32_elems(node, b"clock-indices\0".as_ptr() as *const c_char);
    of_property_read_u32_index(node, b"clock-indices\0".as_ptr() as *const c_char,
                                number - 1, &mut number);
    (*clk_data).clks = kzalloc_clk_array((number + 1) as usize);
    if (*clk_data).clks.is_null() {
        kfree(clk_data as *mut c_void); iounmap(reg); of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res)); return;
    }

    let mut i = 0;
    let mut index: c_int = 0;
    while i < number {
        of_property_read_u32_index(node, b"clock-indices\0".as_ptr() as *const c_char, i, &mut index);
        of_property_read_string_index(node, b"clock-output-names\0".as_ptr() as *const c_char, i, &mut (clk_name as *mut _));
        clk_reg = (reg as usize + 4 * (index as usize / 32)) as *mut c_void;
        let clk_bit = (index as u8) % 32;
        (*clk_data).clks.add(index as usize).write(clk_register_gate(core::ptr::null_mut(), clk_name, clk_parent, 0, clk_reg, clk_bit, 0, &mut gates_lock));
        if !protected.is_null() {
            let mut j = 0;
            while j < nprotected {
                if *protected.add(j as usize) == index {
                    clk_prepare_enable(*(*clk_data).clks.add(index as usize));
                }
                j += 1;
            }
        }
        i += 1;
    }
    (*clk_data).clk_num = number + 1;
    of_clk_add_provider(node, of_clk_src_onecell_get as *const c_void, clk_data);
}

unsafe fn sunxi_simple_gates_init(node: *mut device_node) { sunxi_simple_gates_setup(node, core::ptr::null(), 0); }

// CLK_OF_DECLARE registrations (all invoke sunxi_simple_gates_init):
// sun4i_a10_gates, sun4i_a10_apb0, sun4i_a10_apb1, sun4i_a10_axi,
// sun5i_a10s_apb0, sun5i_a10s_apb1, sun5i_a13_apb0, sun5i_a13_apb1,
// sun6i_a31_ahb1, sun6i_a31_apb1, sun6i_a31_apb2, sun7i_a20_apb0,
// sun7i_a20_apb1, sun8i_a23_ahb1, sun8i_a23_apb1, sun8i_a23_apb2,
// sun8i_a33_ahb1, sun8i_a83t_apb0, sun9i_a80_ahb0, sun9i_a80_ahb1,
// sun9i_a80_ahb2, sun9i_a80_apb0, sun9i_a80_apb1, sun9i_a80_apbs.

static sun4i_a10_ahb_critical_clocks: [c_int; 1] = [14]; // ahb_sdram
unsafe fn sun4i_a10_ahb_init(node: *mut device_node) {
    sunxi_simple_gates_setup(node, sun4i_a10_ahb_critical_clocks.as_ptr(), 1);
}
// CLK_OF_DECLARE: sun4i_a10_ahb, sun5i_a10s_ahb, sun5i_a13_ahb, sun7i_a20_ahb.

static sun4i_a10_dram_critical_clocks: [c_int; 1] = [15]; // dram_output
unsafe fn sun4i_a10_dram_init(node: *mut device_node) {
    sunxi_simple_gates_setup(node, sun4i_a10_dram_critical_clocks.as_ptr(), 1);
}
// CLK_OF_DECLARE: sun4i_a10_dram.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
