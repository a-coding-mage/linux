// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Google, Inc.
 */

// Dependencies supplied by the Linux clock, device-tree, allocator, and local
// clock-provider interfaces are intentionally left external.

use core::ffi::c_void;
use core::ptr;

extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_clk_add_provider(
        node: *mut device_node,
        get: Option<unsafe extern "C" fn(*mut device_node, *const c_void) -> *mut clk>,
        data: *mut clk_onecell_data,
    ) -> i32;
    fn clk_register_gate(
        dev: *mut device,
        name: *const i8,
        parent: *const i8,
        flags: u32,
        reg: *mut u8,
        shift: u8,
        gate_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn clk_register_mux(
        dev: *mut device,
        name: *const i8,
        parents: *const *const i8,
        num_parents: u8,
        flags: u32,
        reg: *mut u8,
        shift: u8,
        width: u8,
        mux_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn clk_register_divider(
        dev: *mut device,
        name: *const i8,
        parent: *const i8,
        flags: u32,
        reg: *mut u8,
        shift: u8,
        width: u8,
        div_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn clk_register_fixed_factor(
        dev: *mut device,
        name: *const i8,
        parent: *const i8,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn __clk_get_name(clk: *mut clk) -> *const i8;
    fn get_count_order(num: u8) -> u8;
    fn of_clk_src_onecell_get(node: *mut device_node, data: *const c_void) -> *mut clk;
}

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }

#[repr(C)]
pub struct clk_onecell_data {
    pub clks: *mut *mut clk,
    pub clk_num: u32,
}

#[repr(C)]
pub struct pistachio_clk_provider {
    pub clk_data: clk_onecell_data,
    pub node: *mut device_node,
    pub base: *mut u8,
}

#[repr(C)]
pub struct pistachio_gate {
    pub name: *const i8,
    pub parent: *const i8,
    pub reg: usize,
    pub shift: u8,
    pub id: usize,
}

#[repr(C)]
pub struct pistachio_mux {
    pub name: *const i8,
    pub parents: *const *const i8,
    pub num_parents: u8,
    pub reg: usize,
    pub shift: u8,
    pub id: usize,
}

#[repr(C)]
pub struct pistachio_div {
    pub name: *const i8,
    pub parent: *const i8,
    pub reg: usize,
    pub width: u8,
    pub div_flags: u8,
    pub id: usize,
}

#[repr(C)]
pub struct pistachio_fixed_factor {
    pub name: *const i8,
    pub parent: *const i8,
    pub div: u32,
    pub id: usize,
}

const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const CLK_SET_RATE_NO_REPARENT: u32 = 1 << 6;

pub unsafe extern "C" fn pistachio_clk_alloc_provider(
    node: *mut device_node,
    num_clks: u32,
) -> *mut pistachio_clk_provider {
    let p = kzalloc(core::mem::size_of::<pistachio_clk_provider>(), 0)
        as *mut pistachio_clk_provider;
    if p.is_null() { return p; }

    (*p).clk_data.clks = kzalloc(
        core::mem::size_of::<*mut clk>() * num_clks as usize, 0,
    ) as *mut *mut clk;
    if (*p).clk_data.clks.is_null() {
        kfree(p as *mut c_void);
        return ptr::null_mut();
    }
    (*p).clk_data.clk_num = num_clks;
    (*p).node = node;
    (*p).base = of_iomap(node, 0);
    if (*p).base.is_null() {
        kfree((*p).clk_data.clks as *mut c_void);
        kfree(p as *mut c_void);
        return ptr::null_mut();
    }
    p
}

pub unsafe extern "C" fn pistachio_clk_register_provider(p: *mut pistachio_clk_provider) {
    for i in 0..(*p).clk_data.clk_num {
        let clk = *(*p).clk_data.clks.add(i as usize);
        if (clk as isize) < 0 { /* IS_ERR: external kernel pointer predicate */ }
    }
    of_clk_add_provider((*p).node, Some(of_clk_src_onecell_get), &mut (*p).clk_data);
}

pub unsafe extern "C" fn pistachio_clk_register_gate(p: *mut pistachio_clk_provider, gate: *mut pistachio_gate, num: u32) {
    for i in 0..num as usize {
        let g = &*gate.add(i);
        let clk = clk_register_gate(ptr::null_mut(), g.name, g.parent, CLK_SET_RATE_PARENT,
            (*p).base.add(g.reg), g.shift, 0, ptr::null_mut());
        *(*p).clk_data.clks.add(g.id) = clk;
    }
}

pub unsafe extern "C" fn pistachio_clk_register_mux(p: *mut pistachio_clk_provider, mux: *mut pistachio_mux, num: u32) {
    for i in 0..num as usize {
        let m = &*mux.add(i);
        let clk = clk_register_mux(ptr::null_mut(), m.name, m.parents, m.num_parents,
            CLK_SET_RATE_NO_REPARENT, (*p).base.add(m.reg), m.shift,
            get_count_order(m.num_parents), 0, ptr::null_mut());
        *(*p).clk_data.clks.add(m.id) = clk;
    }
}

pub unsafe extern "C" fn pistachio_clk_register_div(p: *mut pistachio_clk_provider, div: *mut pistachio_div, num: u32) {
    for i in 0..num as usize {
        let d = &*div.add(i);
        let clk = clk_register_divider(ptr::null_mut(), d.name, d.parent, 0,
            (*p).base.add(d.reg), 0, d.width, d.div_flags, ptr::null_mut());
        *(*p).clk_data.clks.add(d.id) = clk;
    }
}

pub unsafe extern "C" fn pistachio_clk_register_fixed_factor(p: *mut pistachio_clk_provider, ff: *mut pistachio_fixed_factor, num: u32) {
    for i in 0..num as usize {
        let f = &*ff.add(i);
        let clk = clk_register_fixed_factor(ptr::null_mut(), f.name, f.parent, 0, 1, f.div);
        *(*p).clk_data.clks.add(f.id) = clk;
    }
}

pub unsafe extern "C" fn pistachio_clk_force_enable(p: *mut pistachio_clk_provider, clk_ids: *mut u32, num: u32) {
    for i in 0..num as usize {
        let clk = *(*p).clk_data.clks.add(*clk_ids.add(i) as usize);
        if (clk as isize) < 0 { continue; }
        let _err = clk_prepare_enable(clk);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
