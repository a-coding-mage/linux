// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7779 Core CPG Clocks
 *
 * Copyright (C) 2013, 2014 Horms Solutions Ltd.
 *
 * Contact: Simon Horman <horms@verge.net.au>
 */

// Linux kernel headers and dt-bindings are external dependencies.

const CPG_NUM_CLOCKS: usize = R8A7779_CLK_OUT as usize + 1;

#[repr(C)]
struct CpgClkConfig {
    z_mult: c_uint,
    z_div: c_uint,
    zs_and_s_div: c_uint,
    s1_div: c_uint,
    p_div: c_uint,
    b_and_out_div: c_uint,
}

static CPG_CLK_CONFIGS: [CpgClkConfig; 4] = [
    CpgClkConfig { z_mult: 1, z_div: 2, zs_and_s_div: 8, s1_div: 16, p_div: 32, b_and_out_div: 24 },
    CpgClkConfig { z_mult: 2, z_div: 3, zs_and_s_div: 6, s1_div: 12, p_div: 24, b_and_out_div: 24 },
    CpgClkConfig { z_mult: 1, z_div: 2, zs_and_s_div: 8, s1_div: 16, p_div: 32, b_and_out_div: 32 },
    CpgClkConfig { z_mult: 2, z_div: 3, zs_and_s_div: 6, s1_div: 12, p_div: 24, b_and_out_div: 36 },
];

static CPG_PLLA_MULT: [c_uint; 4] = [42, 48, 56, 64];

const fn cpg_clk_config_index(md: u32) -> usize {
    (((md & ((1 << 2) | (1 << 1))) >> 1) as usize)
}

const fn cpg_plla_mult_index(md: u32) -> usize {
    (((md & ((1 << 12) | (1 << 11))) >> 11) as usize)
}

type c_uint = u32;
type c_int = i32;
type c_long = i64;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ClkOnecellData {
    pub clks: *mut *mut Clk,
    pub clk_num: c_uint,
}

extern "C" {
    static R8A7779_CLK_OUT: c_uint;

    fn strcmp(a: *const u8, b: *const u8) -> c_int;
    fn of_clk_get_parent_name(np: *mut DeviceNode, index: c_uint) -> *const u8;
    fn clk_register_fixed_factor(
        dev: *mut core::ffi::c_void,
        name: *const u8,
        parent_name: *const u8,
        flags: c_uint,
        mult: c_uint,
        div: c_uint,
    ) -> *mut Clk;
    fn rcar_rst_read_mode_pins(mode: *mut u32) -> c_int;
    fn of_property_count_strings(np: *mut DeviceNode, property: *const u8) -> c_int;
    fn kzalloc_obj<T>() -> *mut T;
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn of_property_read_string_index(
        np: *mut DeviceNode,
        property: *const u8,
        index: c_uint,
        output: *mut *const u8,
    ) -> c_int;
    fn of_clk_add_provider(
        np: *mut DeviceNode,
        get: *const core::ffi::c_void,
        data: *mut ClkOnecellData,
    ) -> c_int;
    fn of_clk_src_onecell_get() -> !;
    fn cpg_mstp_add_clk_domain(np: *mut DeviceNode);
    fn pr_err(format: *const u8, ...);
}

const EINVAL: c_int = 22;

unsafe fn r8a7779_cpg_register_clock(
    np: *mut DeviceNode,
    config: *const CpgClkConfig,
    plla_mult: c_uint,
    name: *const u8,
) -> *mut Clk {
    let mut parent_name = b"plla\0".as_ptr();
    let mut mult: c_uint = 1;
    let mut div: c_uint = 1;

    if strcmp(name, b"plla\0".as_ptr()) == 0 {
        parent_name = of_clk_get_parent_name(np, 0);
        mult = plla_mult;
    } else if strcmp(name, b"z\0".as_ptr()) == 0 {
        div = (*config).z_div;
        mult = (*config).z_mult;
    } else if strcmp(name, b"zs\0".as_ptr()) == 0 || strcmp(name, b"s\0".as_ptr()) == 0 {
        div = (*config).zs_and_s_div;
    } else if strcmp(name, b"s1\0".as_ptr()) == 0 {
        div = (*config).s1_div;
    } else if strcmp(name, b"p\0".as_ptr()) == 0 {
        div = (*config).p_div;
    } else if strcmp(name, b"b\0".as_ptr()) == 0 || strcmp(name, b"out\0".as_ptr()) == 0 {
        div = (*config).b_and_out_div;
    } else {
        return (-EINVAL as isize) as *mut Clk;
    }

    clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, div)
}

unsafe fn r8a7779_cpg_clocks_init(np: *mut DeviceNode) {
    let config: *const CpgClkConfig;
    let data: *mut ClkOnecellData;
    let clks: *mut *mut Clk;
    let mut mode: u32 = 0;
    let mut plla_mult: c_uint;
    let num_clks: c_int;

    if rcar_rst_read_mode_pins(&mut mode) != 0 {
        return;
    }

    num_clks = of_property_count_strings(np, b"clock-output-names\0".as_ptr());
    if num_clks < 0 {
        pr_err(b"%s: failed to count clocks\n\0".as_ptr());
        return;
    }

    data = kzalloc_obj::<ClkOnecellData>();
    clks = kzalloc_objs::<*mut Clk>(CPG_NUM_CLOCKS);
    if data.is_null() || clks.is_null() {
        /* We're leaking memory on purpose, there's no point in cleaning
         * up as the system won't boot anyway.
         */
        return;
    }

    (*data).clks = clks;
    (*data).clk_num = num_clks as c_uint;

    config = &CPG_CLK_CONFIGS[cpg_clk_config_index(mode)];
    plla_mult = CPG_PLLA_MULT[cpg_plla_mult_index(mode)];

    let mut i: c_uint = 0;
    while i < num_clks as c_uint {
        let mut name: *const u8 = core::ptr::null();
        let clk: *mut Clk;

        of_property_read_string_index(np, b"clock-output-names\0".as_ptr(), i, &mut name);

        clk = r8a7779_cpg_register_clock(np, config, plla_mult, name);
        if (clk as isize) == (-EINVAL as isize) {
            pr_err(b"%s: failed to register %pOFn %s clock (%ld)\n\0".as_ptr());
        } else {
            *clks.add(i as usize) = clk;
        }

        i += 1;
    }

    of_clk_add_provider(np, of_clk_src_onecell_get as *const core::ffi::c_void, data);
    cpg_mstp_add_clk_domain(np);
}

// CLK_OF_DECLARE(r8a7779_cpg_clks, "renesas,r8a7779-cpg-clocks",
//                r8a7779_cpg_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
