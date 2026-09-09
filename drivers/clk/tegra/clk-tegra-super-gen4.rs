// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.

const PLLX_BASE: usize = 0xe0;
const PLLX_MISC: usize = 0xe4;
const PLLX_MISC2: usize = 0x514;
const PLLX_MISC3: usize = 0x518;

const CCLKG_BURST_POLICY: usize = 0x368;
const CCLKLP_BURST_POLICY: usize = 0x370;
const SCLK_BURST_POLICY: usize = 0x028;
const SYSTEM_CLK_RATE: usize = 0x030;
const SCLK_DIVIDER: usize = 0x2c;

static mut sysrate_lock: usize = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum tegra_super_gen {
    gen4 = 4,
    gen5,
}

#[repr(C)]
struct tegra_super_gen_info {
    gen: tegra_super_gen,
    sclk_parents: *const *const i8,
    cclk_g_parents: *const *const i8,
    cclk_lp_parents: *const *const i8,
    num_sclk_parents: i32,
    num_cclk_g_parents: i32,
    num_cclk_lp_parents: i32,
}

#[repr(C)] pub struct tegra_clk { _private: [u8; 0] }
#[repr(C)] pub struct tegra_clk_pll_params { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }

extern "C" {
    fn tegra_lookup_dt_id(id: i32, tegra_clks: *mut tegra_clk) -> *mut *mut clk;
    fn tegra_clk_register_super_mux(name: *const i8, parents: *const *const i8,
        num_parents: i32, flags: u32, reg: *mut u8, shift: u32, width: u32,
        mux_shift: u32, mux_width: u32, lock: *mut u8) -> *mut clk;
    fn clk_register_divider(parent: *mut clk, name: *const i8, parent_name: *const i8,
        flags: u32, reg: *mut u8, shift: u8, width: u8, table: u8,
        lock: *mut usize) -> *mut clk;
    fn clk_register_gate(parent: *mut clk, name: *const i8, parent_name: *const i8,
        flags: u32, reg: *mut u8, bit_idx: u8, flags2: u32, lock: *mut usize) -> *mut clk;
    fn tegra_clk_register_pllc_tegra210(name: *const i8, parent_name: *const i8,
        clk_base: *mut u8, pmc_base: *mut u8, flags: u32,
        params: *mut tegra_clk_pll_params, lock: *mut u8) -> *mut clk;
    fn tegra_clk_register_pllxc(name: *const i8, parent_name: *const i8,
        clk_base: *mut u8, pmc_base: *mut u8, flags: u32,
        params: *mut tegra_clk_pll_params, lock: *mut u8) -> *mut clk;
    fn clk_register_fixed_factor(parent: *mut clk, name: *const i8, parent_name: *const i8,
        flags: u32, mult: u32, div: u32) -> *mut clk;
}

extern "C" {
    static tegra_clk_sclk_mux: i32;
    static tegra_clk_sclk: i32;
    static tegra_clk_hclk: i32;
    static tegra_clk_pclk: i32;
    static tegra_clk_cclk_g: i32;
    static tegra_clk_cclk_lp: i32;
    static tegra_clk_pll_x: i32;
    static tegra_clk_pll_x_out0: i32;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_IS_CRITICAL: u32 = 1 << 1;
const CLK_GATE_SET_TO_DISABLE: u32 = 1 << 2;
const CLK_IGNORE_UNUSED: u32 = 1 << 3;
const TEGRA210_CPU_CLK: u32 = 1;
const TEGRA_DIVIDER_2: u32 = 2;

static sclk_parents: [*const i8; 8] = [b"clk_m\0".as_ptr() as *const i8, b"pll_c_out1\0".as_ptr() as *const i8, b"pll_p_out4\0".as_ptr() as *const i8, b"pll_p\0".as_ptr() as *const i8, b"pll_p_out2\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"clk_32k\0".as_ptr() as *const i8, b"pll_m_out1\0".as_ptr() as *const i8];
static cclk_g_parents: [*const i8; 16] = [b"clk_m\0".as_ptr() as *const i8, b"pll_c\0".as_ptr() as *const i8, b"clk_32k\0".as_ptr() as *const i8, b"pll_m\0".as_ptr() as *const i8, b"pll_p\0".as_ptr() as *const i8, b"pll_p_out4\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"pll_x\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"dfllCPU_out\0".as_ptr() as *const i8];
static cclk_lp_parents: [*const i8; 10] = [b"clk_m\0".as_ptr() as *const i8, b"pll_c\0".as_ptr() as *const i8, b"clk_32k\0".as_ptr() as *const i8, b"pll_m\0".as_ptr() as *const i8, b"pll_p\0".as_ptr() as *const i8, b"pll_p_out4\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"pll_x\0".as_ptr() as *const i8, b"pll_x_out0\0".as_ptr() as *const i8];

static sclk_parents_gen5: [*const i8; 8] = [b"clk_m\0".as_ptr() as *const i8, b"pll_c_out1\0".as_ptr() as *const i8, b"pll_c4_out3\0".as_ptr() as *const i8, b"pll_p\0".as_ptr() as *const i8, b"pll_p_out2\0".as_ptr() as *const i8, b"pll_c4_out1\0".as_ptr() as *const i8, b"clk_32k\0".as_ptr() as *const i8, b"pll_c4_out2\0".as_ptr() as *const i8];
static cclk_g_parents_gen5: [*const i8; 16] = [b"clk_m\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"clk_32k\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"pll_p\0".as_ptr() as *const i8, b"pll_p_out4\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"pll_x\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"unused\0".as_ptr() as *const i8, b"dfllCPU_out\0".as_ptr() as *const i8];
static cclk_lp_parents_gen5: [*const i8; 16] = cclk_g_parents_gen5;

static tegra_super_gen_info_gen4: tegra_super_gen_info = tegra_super_gen_info { gen: tegra_super_gen::gen4, sclk_parents: sclk_parents.as_ptr(), cclk_g_parents: cclk_g_parents.as_ptr(), cclk_lp_parents: cclk_lp_parents.as_ptr(), num_sclk_parents: 8, num_cclk_g_parents: 16, num_cclk_lp_parents: 10 };
static tegra_super_gen_info_gen5: tegra_super_gen_info = tegra_super_gen_info { gen: tegra_super_gen::gen5, sclk_parents: sclk_parents_gen5.as_ptr(), cclk_g_parents: cclk_g_parents_gen5.as_ptr(), cclk_lp_parents: cclk_lp_parents_gen5.as_ptr(), num_sclk_parents: 8, num_cclk_g_parents: 16, num_cclk_lp_parents: 16 };

unsafe fn tegra_sclk_init(clk_base: *mut u8, tegra_clks: *mut tegra_clk, g: *const tegra_super_gen_info) {
    let mut dt: *mut *mut clk; let mut c: *mut clk;
    dt = tegra_lookup_dt_id(tegra_clk_sclk_mux, tegra_clks);
    if !dt.is_null() {
        c = tegra_clk_register_super_mux(b"sclk_mux\0".as_ptr() as *const i8, (*g).sclk_parents, (*g).num_sclk_parents, CLK_SET_RATE_PARENT, clk_base.add(SCLK_BURST_POLICY), 0, 4, 0, 0, core::ptr::null_mut()); *dt = c;
        dt = tegra_lookup_dt_id(tegra_clk_sclk, tegra_clks);
        if !dt.is_null() { *dt = clk_register_divider(core::ptr::null_mut(), b"sclk\0".as_ptr() as *const i8, b"sclk_mux\0".as_ptr() as *const i8, CLK_IS_CRITICAL, clk_base.add(SCLK_DIVIDER), 0, 8, 0, &mut sysrate_lock); }
    } else { dt = tegra_lookup_dt_id(tegra_clk_sclk, tegra_clks); if !dt.is_null() { *dt = tegra_clk_register_super_mux(b"sclk\0".as_ptr() as *const i8, (*g).sclk_parents, (*g).num_sclk_parents, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL, clk_base.add(SCLK_BURST_POLICY), 0, 4, 0, 0, core::ptr::null_mut()); } }
    dt = tegra_lookup_dt_id(tegra_clk_hclk, tegra_clks); if !dt.is_null() { c = clk_register_divider(core::ptr::null_mut(), b"hclk_div\0".as_ptr() as *const i8, b"sclk\0".as_ptr() as *const i8, 0, clk_base.add(SYSTEM_CLK_RATE), 4, 2, 0, &mut sysrate_lock); *dt = clk_register_gate(core::ptr::null_mut(), b"hclk\0".as_ptr() as *const i8, b"hclk_div\0".as_ptr() as *const i8, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL, clk_base.add(SYSTEM_CLK_RATE), 7, CLK_GATE_SET_TO_DISABLE, &mut sysrate_lock); let _ = c; }
    dt = tegra_lookup_dt_id(tegra_clk_pclk, tegra_clks); if dt.is_null() { return; } c = clk_register_divider(core::ptr::null_mut(), b"pclk_div\0".as_ptr() as *const i8, b"hclk\0".as_ptr() as *const i8, 0, clk_base.add(SYSTEM_CLK_RATE), 0, 2, 0, &mut sysrate_lock); *dt = clk_register_gate(core::ptr::null_mut(), b"pclk\0".as_ptr() as *const i8, b"pclk_div\0".as_ptr() as *const i8, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL, clk_base.add(SYSTEM_CLK_RATE), 3, CLK_GATE_SET_TO_DISABLE, &mut sysrate_lock); let _ = c;
}

unsafe fn tegra_super_clk_init(clk_base: *mut u8, pmc_base: *mut u8, clks: *mut tegra_clk, params: *mut tegra_clk_pll_params, g: *const tegra_super_gen_info) {
    tegra_sclk_init(clk_base, clks, g);
    let dt = tegra_lookup_dt_id(tegra_clk_pll_x, clks); if !dt.is_null() { let c = tegra_clk_register_pllxc(b"pll_x\0".as_ptr() as *const i8, b"pll_ref\0".as_ptr() as *const i8, clk_base, pmc_base, CLK_IGNORE_UNUSED, params, core::ptr::null_mut()); *dt = c; }
    let dt2 = tegra_lookup_dt_id(tegra_clk_pll_x_out0, clks); if !dt2.is_null() { *dt2 = clk_register_fixed_factor(core::ptr::null_mut(), b"pll_x_out0\0".as_ptr() as *const i8, b"pll_x\0".as_ptr() as *const i8, CLK_SET_RATE_PARENT, 1, 2); }
}

pub unsafe fn tegra_super_clk_gen4_init(b: *mut u8, p: *mut u8, c: *mut tegra_clk, x: *mut tegra_clk_pll_params) { tegra_super_clk_init(b, p, c, x, &tegra_super_gen_info_gen4); }
pub unsafe fn tegra_super_clk_gen5_init(b: *mut u8, p: *mut u8, c: *mut tegra_clk, x: *mut tegra_clk_pll_params) { tegra_super_clk_init(b, p, c, x, &tegra_super_gen_info_gen5); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
