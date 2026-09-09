// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 *
 * Emilio López <emilio@elopez.com.ar>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub type __iomem = c_void;

pub const SUN4I_CODEC_GATE: u32 = 31;

pub const CLK_SET_RATE_PARENT: u32 = 1 << 2;

unsafe extern "C" {
    fn of_io_request_and_map(
        node: *mut device_node,
        index: c_int,
        name: *const c_char,
    ) -> *mut __iomem;
    fn of_node_full_name(node: *mut device_node) -> *const c_char;
    fn of_property_read_string(
        node: *mut device_node,
        property: *const c_char,
        out: *mut *const c_char,
    ) -> c_int;
    fn of_clk_get_parent_name(node: *mut device_node, index: c_int) -> *const c_char;
    fn clk_register_gate(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        reg: *mut __iomem,
        bit_idx: u8,
        flags2: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn of_clk_add_provider(
        node: *mut device_node,
        get: unsafe extern "C" fn(*mut device_node, *mut c_void, u32) -> *mut clk,
        data: *mut clk,
    ) -> c_int;
    fn of_clk_src_simple_get(
        node: *mut device_node,
        data: *mut c_void,
        index: u32,
    ) -> *mut clk;
    fn is_err(ptr: *const c_void) -> bool;
}

unsafe extern "C" fn sun4i_codec_clk_setup(node: *mut device_node) {
    let mut clk: *mut clk;
    let mut clk_name: *const c_char = (*node).name;
    let parent_name: *const c_char;
    let reg: *mut __iomem;

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if is_err(reg.cast()) {
        return;
    }

    of_property_read_string(
        node,
        b"clock-output-names\0".as_ptr().cast(),
        &mut clk_name,
    );
    parent_name = of_clk_get_parent_name(node, 0);

    clk = clk_register_gate(
        core::ptr::null_mut(),
        clk_name,
        parent_name,
        CLK_SET_RATE_PARENT,
        reg,
        SUN4I_CODEC_GATE as u8,
        0,
        core::ptr::null_mut(),
    );

    if !is_err(clk.cast()) {
        of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
}

// CLK_OF_DECLARE(sun4i_codec, "allwinner,sun4i-a10-codec-clk",
//                sun4i_codec_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
