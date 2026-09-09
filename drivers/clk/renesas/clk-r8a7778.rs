// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7778 Core CPG Clocks
 *
 * Copyright (C) 2014  Ulrich Hecht
 */

// Translated from the Linux kernel implementation.  The declarations below
// are supplied by the surrounding kernel bindings.

#[repr(C)]
struct R8a7778Rate {
    plla_mult: ::core::ffi::c_ulong,
    pllb_mult: ::core::ffi::c_ulong,
}

/* PLL multipliers per bits 11, 12, and 18 of MODEMR */
static R8A7778_RATES: [R8a7778Rate; 8] = [
    R8a7778Rate { plla_mult: 21, pllb_mult: 21 },
    R8a7778Rate { plla_mult: 24, pllb_mult: 24 },
    R8a7778Rate { plla_mult: 28, pllb_mult: 28 },
    R8a7778Rate { plla_mult: 32, pllb_mult: 32 },
    R8a7778Rate { plla_mult: 0, pllb_mult: 0 },
    R8a7778Rate { plla_mult: 24, pllb_mult: 21 },
    R8a7778Rate { plla_mult: 28, pllb_mult: 21 },
    R8a7778Rate { plla_mult: 32, pllb_mult: 24 },
];

#[repr(C)]
struct R8a7778Div {
    name: *const ::core::ffi::c_char,
    div: [u32; 4],
}

/* Clock dividers per bits 1 and 2 of MODEMR */
static R8A7778_DIVS: [R8a7778Div; 6] = [
    R8a7778Div { name: b"b\0".as_ptr() as *const _, div: [12, 12, 16, 18] },
    R8a7778Div { name: b"out\0".as_ptr() as *const _, div: [12, 12, 16, 18] },
    R8a7778Div { name: b"p\0".as_ptr() as *const _, div: [16, 12, 16, 12] },
    R8a7778Div { name: b"s\0".as_ptr() as *const _, div: [4, 3, 4, 3] },
    R8a7778Div { name: b"s1\0".as_ptr() as *const _, div: [8, 6, 8, 6] },
    R8a7778Div { name: ::core::ptr::null(), div: [0; 4] },
];

static mut CPG_MODE_RATES: u32 = 0;
static mut CPG_MODE_DIVS: u32 = 0;

extern "C" {
    type DeviceNode;
    type Clk;

    fn strcmp(a: *const ::core::ffi::c_char, b: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn of_clk_get_parent_name(np: *mut DeviceNode, index: u32) -> *const ::core::ffi::c_char;
    fn clk_register_fixed_factor(
        dev: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        parent_name: *const ::core::ffi::c_char,
        flags: u32,
        mult: ::core::ffi::c_ulong,
        div: ::core::ffi::c_ulong,
    ) -> *mut Clk;
    fn rcar_rst_read_mode_pins(mode: *mut u32) -> ::core::ffi::c_int;
    fn of_property_count_strings(np: *mut DeviceNode, propname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn of_property_read_string_index(
        np: *mut DeviceNode,
        propname: *const ::core::ffi::c_char,
        index: u32,
        output: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn of_clk_add_provider(np: *mut DeviceNode, get: *const ::core::ffi::c_void, data: *mut ClkOnecellData) -> ::core::ffi::c_int;
    fn cpg_mstp_add_clk_domain(np: *mut DeviceNode);
    fn kzalloc(size: usize, flags: u32) -> *mut ::core::ffi::c_void;
}

unsafe fn r8a7778_cpg_register_clock(np: *mut DeviceNode, name: *const ::core::ffi::c_char) -> *mut Clk {
    if strcmp(name, b"plla\0".as_ptr() as *const _) == 0 {
        return clk_register_fixed_factor(::core::ptr::null_mut(), b"plla\0".as_ptr() as *const _,
            of_clk_get_parent_name(np, 0), 0, R8A7778_RATES[CPG_MODE_RATES as usize].plla_mult, 1);
    } else if strcmp(name, b"pllb\0".as_ptr() as *const _) == 0 {
        return clk_register_fixed_factor(::core::ptr::null_mut(), b"pllb\0".as_ptr() as *const _,
            of_clk_get_parent_name(np, 0), 0, R8A7778_RATES[CPG_MODE_RATES as usize].pllb_mult, 1);
    } else {
        let mut i = 0usize;
        while i < R8A7778_DIVS.len() {
            if !R8A7778_DIVS[i].name.is_null() && strcmp(name, R8A7778_DIVS[i].name) == 0 {
                return clk_register_fixed_factor(::core::ptr::null_mut(), R8A7778_DIVS[i].name,
                    b"plla\0".as_ptr() as *const _, 0, 1,
                    R8A7778_DIVS[i].div[CPG_MODE_DIVS as usize] as ::core::ffi::c_ulong);
            }
            i += 1;
        }
    }

    (-22isize) as *mut Clk
}

#[repr(C)]
struct ClkOnecellData {
    clks: *mut *mut Clk,
    clk_num: u32,
}

unsafe fn r8a7778_cpg_clocks_init(np: *mut DeviceNode) {
    let mut mode: u32 = 0;

    if rcar_rst_read_mode_pins(&mut mode) != 0 {
        return;
    }

    if mode & (1u32 << 19) == 0 {
        ::core::hint::unreachable_unchecked();
    }

    CPG_MODE_RATES = (((mode & (1u32 << 18)) != 0) as u32) << 2
        | (((mode & (1u32 << 12)) != 0) as u32) << 1
        | ((mode & (1u32 << 11)) != 0) as u32;
    CPG_MODE_DIVS = (((mode & (1u32 << 2)) != 0) as u32) << 1
        | ((mode & (1u32 << 1)) != 0) as u32;

    let num_clks = of_property_count_strings(np, b"clock-output-names\0".as_ptr() as *const _);
    if num_clks < 0 {
        return;
    }

    // The C source intentionally leaks these allocations on failure.
    let data = kzalloc(::core::mem::size_of::<ClkOnecellData>(), 0) as *mut ClkOnecellData;
    let clks = kzalloc((num_clks as usize) * ::core::mem::size_of::<*mut Clk>(), 0) as *mut *mut Clk;
    if data.is_null() || clks.is_null() {
        return;
    }

    (*data).clks = clks;
    (*data).clk_num = num_clks as u32;

    let mut i = 0u32;
    while i < num_clks as u32 {
        let mut name: *const ::core::ffi::c_char = ::core::ptr::null();
        of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const _, i, &mut name);

        let clk = r8a7778_cpg_register_clock(np, name);
        if !clk.is_null() {
            *clks.add(i as usize) = clk;
        }
        i += 1;
    }

    of_clk_add_provider(np, ::core::ptr::null(), data);
    cpg_mstp_add_clk_domain(np);
}

// CLK_OF_DECLARE(r8a7778_cpg_clks, "renesas,r8a7778-cpg-clocks",
//                r8a7778_cpg_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
