// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * clkgen-mux.c: ST GEN-MUX Clock driver
 *
 * Copyright (C) 2014 STMicroelectronics (R&D) Limited
 *
 * Authors: Stephen Gallimore <stephen.gallimore@st.com>
 *          Pankaj Dev <pankaj.dev@st.com>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type U32 = u32;
type U8 = u8;
type CChar = c_char;
type CInt = c_int;
type CUint = c_uint;
type CULong = c_ulong;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    static mut clkgen_a9_lock: spinlock_t;

    fn of_clk_get_parent_count(np: *mut device_node) -> CUint;
    fn kcalloc(n: usize, size: usize, flags: CUint) -> *mut *const CChar;
    fn of_clk_parent_fill(
        np: *mut device_node,
        parents: *mut *const CChar,
        nparents: CUint,
    ) -> CInt;
    fn of_iomap(np: *mut device_node, index: CInt) -> *mut c_void;
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn pr_err(fmt: *const CChar, ...);
    fn pr_debug(fmt: *const CChar, ...);
    fn clk_register_mux(
        dev: *mut c_void,
        name: *const CChar,
        parent_names: *const *const CChar,
        num_parents: CInt,
        flags: CULong,
        reg: *mut c_void,
        shift: U8,
        width: U8,
        mux_flags: U8,
        lock: *mut spinlock_t,
    ) -> *mut clk;
    fn __clk_get_name(clk: *mut clk) -> *const CChar;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> CULong;
    fn kfree(ptr: *mut *const CChar);
    fn of_clk_add_provider(
        np: *mut device_node,
        get: *const c_void,
        data: *mut clk,
    ) -> CInt;
    fn of_clk_src_simple_get;
    fn iounmap(addr: *mut c_void);
}

const GFP_KERNEL: CUint = 0;
const CLK_SET_RATE_PARENT: CULong = 1 << 5;

#[repr(C)]
pub struct clkgen_mux_data {
    pub offset: U32,
    pub shift: U8,
    pub width: U8,
    pub lock: *mut spinlock_t,
    pub clk_flags: CULong,
    pub mux_flags: U8,
}

static mut stih407_a9_mux_data: clkgen_mux_data = clkgen_mux_data {
    offset: 0x1a4,
    shift: 0,
    width: 2,
    lock: unsafe { &raw mut clkgen_a9_lock },
    clk_flags: 0,
    mux_flags: 0,
};

unsafe fn clkgen_mux_get_parents(
    np: *mut device_node,
    num_parents: *mut CInt,
) -> *mut *const CChar {
    let nparents = of_clk_get_parent_count(np);
    if nparents == 0 {
        // WARN_ON(!nparents)
        return (-22isize) as *mut *const CChar;
    }

    let parents = kcalloc(
        nparents as usize,
        core::mem::size_of::<*const CChar>(),
        GFP_KERNEL,
    );
    if parents.is_null() {
        return (-12isize) as *mut *const CChar;
    }

    *num_parents = of_clk_parent_fill(np, parents, nparents);
    parents
}

unsafe fn st_of_clkgen_mux_setup(
    np: *mut device_node,
    data: *mut clkgen_mux_data,
) {
    let mut clk: *mut clk;
    let mut reg: *mut c_void;
    let parents: *mut *const CChar;
    let mut num_parents: CInt = 0;
    let parent_np: *mut device_node;

    /*
     * First check for reg property within the node to keep backward
     * compatibility, then if reg doesn't exist look at the parent node
     */
    reg = of_iomap(np, 0);
    if reg.is_null() {
        parent_np = of_get_parent(np);
        reg = of_iomap(parent_np, 0);
        of_node_put(parent_np);
        if reg.is_null() {
            pr_err(c"%s: Failed to get base address\n", c"st_of_clkgen_mux_setup\0".as_ptr());
            return;
        }
    }

    parents = clkgen_mux_get_parents(np, &mut num_parents);
    if (parents as isize) < 0 && (parents as isize) >= -4095 {
        pr_err(c"%s: Failed to get parents (%ld)\n", c"st_of_clkgen_mux_setup\0".as_ptr(), parents as isize);
        iounmap(reg);
        return;
    }

    clk = clk_register_mux(
        core::ptr::null_mut(),
        core::ptr::null(),
        parents,
        num_parents,
        (*data).clk_flags | CLK_SET_RATE_PARENT,
        reg.add((*data).offset as usize),
        (*data).shift,
        (*data).width,
        (*data).mux_flags,
        (*data).lock,
    );
    if (clk as isize) < 0 && (clk as isize) >= -4095 {
        kfree(parents);
        iounmap(reg);
        return;
    }

    pr_debug(c"%s: parent %s rate %u\n", c"__clk_get_name\0".as_ptr(), __clk_get_name(clk), __clk_get_name(clk_get_parent(clk)), clk_get_rate(clk) as CUint);

    kfree(parents);
    of_clk_add_provider(np, of_clk_src_simple_get as *const c_void, clk);
}

unsafe fn st_of_clkgen_a9_mux_setup(np: *mut device_node) {
    st_of_clkgen_mux_setup(np, &raw mut stih407_a9_mux_data);
}

// CLK_OF_DECLARE(clkgen_a9mux, "st,stih407-clkgen-a9-mux", st_of_clkgen_a9_mux_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
