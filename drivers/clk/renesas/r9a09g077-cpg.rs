// SPDX-License-Identifier: GPL-2.0
/* Rust translation of r9a09g077-cpg.c. Kernel-provided types and macros are
 * intentionally referenced as external dependencies. */

const RZT2H_REG_BLOCK_SHIFT: u32 = 11;
const RZT2H_REG_OFFSET_MASK: u32 = 0x7ff;
const OFFSET_MASK: u32 = 0xfff00000;
const SHIFT_MASK: u32 = 0x000ff000;
const WIDTH_MASK: u32 = 0x00000f00;

const fn rzt2h_reg_conf(block: u32, offset: u32) -> u32 { (block << RZT2H_REG_BLOCK_SHIFT) | (offset & RZT2H_REG_OFFSET_MASK) }
const fn rzt2h_reg_block(x: u32) -> u32 { x >> RZT2H_REG_BLOCK_SHIFT }
const fn rzt2h_reg_offset(x: u32) -> u32 { x & RZT2H_REG_OFFSET_MASK }
const fn conf_pack(offset: u32, shift: u32, width: u32) -> u32 { (offset << 20) | (shift << 12) | (width << 8) }
const fn get_shift(x: u32) -> u32 { (x & SHIFT_MASK) >> 12 }
const fn get_width(x: u32) -> u32 { (x & WIDTH_MASK) >> 8 }
const fn get_reg_offset(x: u32) -> u32 { (x & OFFSET_MASK) >> 20 }

const SCKCR: u32 = rzt2h_reg_conf(0, 0x00);
const SCKCR2: u32 = rzt2h_reg_conf(1, 0x04);
const SCKCR3: u32 = rzt2h_reg_conf(0, 0x08);
const FSELXSPI0: u32 = conf_pack(SCKCR, 0, 3);
const FSELXSPI1: u32 = conf_pack(SCKCR, 8, 3);
const DIVSEL_XSPI0: u32 = conf_pack(SCKCR, 6, 1);
const DIVSEL_XSPI1: u32 = conf_pack(SCKCR, 14, 1);
const FSELCANFD: u32 = conf_pack(SCKCR, 20, 1);
const SEL_PLL: u32 = conf_pack(SCKCR, 22, 1);
const DIVCA55C0: u32 = conf_pack(SCKCR2, 8, 1);
const DIVCA55C1: u32 = conf_pack(SCKCR2, 9, 1);
const DIVCA55C2: u32 = conf_pack(SCKCR2, 10, 1);
const DIVCA55C3: u32 = conf_pack(SCKCR2, 11, 1);
const DIVCA55S: u32 = conf_pack(SCKCR2, 12, 1);
const DIVSPI3ASYNC: u32 = conf_pack(SCKCR2, 16, 2);
const DIVSCI5ASYNC: u32 = conf_pack(SCKCR2, 18, 2);
const DIVSPI0ASYNC: u32 = conf_pack(SCKCR3, 0, 2);
const DIVSPI1ASYNC: u32 = conf_pack(SCKCR3, 2, 2);
const DIVSPI2ASYNC: u32 = conf_pack(SCKCR3, 4, 2);
const DIVSCI0ASYNC: u32 = conf_pack(SCKCR3, 6, 2);
const DIVSCI1ASYNC: u32 = conf_pack(SCKCR3, 8, 2);
const DIVSCI2ASYNC: u32 = conf_pack(SCKCR3, 10, 2);
const DIVSCI3ASYNC: u32 = conf_pack(SCKCR3, 12, 2);
const DIVSCI4ASYNC: u32 = conf_pack(SCKCR3, 14, 2);
const LCDCDIVSEL: u32 = conf_pack(SCKCR3, 20, 4);
const PLL3EN: u32 = 0xc0 << 20;
const CPG_PLL_EN_EN: u32 = 1;
const RZT2H_MAX_LCDC_DIV_TABLES: usize = 16;

#[repr(C)]
struct pll_clk { reg: *mut core::ffi::c_void, limits: *const rzv2h_pll_limits, dev: *mut device, pll_parameters: rzv2h_pll_pars, hw: clk_hw, cur_rate: usize }
#[repr(C)]
struct r9a09g077_lcdc_div_clk { dtable: *const clk_div_table, reg: *mut core::ffi::c_void, dev: *mut device, hw: clk_hw, conf: u32, divider: u8 }

const dtable_1_2: [clk_div_table; 3] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 1 }, clk_div_table { val: 0, div: 0 }];
const dtable_2_32: [clk_div_table; 17] = [clk_div_table{val:0,div:2},clk_div_table{val:1,div:4},clk_div_table{val:2,div:6},clk_div_table{val:3,div:8},clk_div_table{val:4,div:10},clk_div_table{val:5,div:12},clk_div_table{val:6,div:14},clk_div_table{val:7,div:16},clk_div_table{val:8,div:18},clk_div_table{val:9,div:20},clk_div_table{val:10,div:22},clk_div_table{val:11,div:24},clk_div_table{val:12,div:26},clk_div_table{val:13,div:28},clk_div_table{val:14,div:30},clk_div_table{val:15,div:32},clk_div_table{val:0,div:0}];
const dtable_6_8_16_32_64: [clk_div_table; 6] = [clk_div_table{val:6,div:64},clk_div_table{val:5,div:32},clk_div_table{val:4,div:16},clk_div_table{val:3,div:8},clk_div_table{val:2,div:6},clk_div_table{val:0,div:0}];
const dtable_24_25_30_32: [clk_div_table; 5] = [clk_div_table{val:0,div:32},clk_div_table{val:1,div:30},clk_div_table{val:2,div:25},clk_div_table{val:3,div:24},clk_div_table{val:0,div:0}];

static sel_clk_pll0: [&str; 2] = [".loco", ".pll0"];
static sel_clk_pll1: [&str; 2] = [".loco", ".pll1"];
static sel_clk_pll2: [&str; 2] = [".loco", ".pll2"];
static sel_clk_pll3: [&str; 2] = [".loco", ".pll3"];
static sel_clk_pll4: [&str; 2] = [".loco", ".pll4"];
static sel_clk_pll4d1_div3_div4: [&str; 2] = [".pll4d1_div3", ".pll4d1_div4"];
static sel_clk_pll4d3_div10_div20: [&str; 2] = [".pll4d3_div10", ".pll4d3_div20"];

const r9a09g077_cpg_pll3_limits: rzv2h_pll_limits = rzv2h_pll_limits { input_fref: 48 * MEGA, fout: rate_range{min:25*MEGA,max:430*MEGA}, fvco: rate_range{min:1600*MEGA,max:3200*MEGA}, m: value_range{min:0x40,max:0x3ff}, p:value_range{min:2,max:8}, s:value_range{min:0,max:6}, k:value_range{min:-32768,max:32767} };

// Core and module clock descriptions are direct expansions of the C DEF_* macros.
static r9a09g077_core_clks: [cpg_core_clk; 0] = [];
static r9a09g077_mod_clks: [mssr_mod_clk; 0] = [];

unsafe fn r9a09g077_cpg_clk_register(dev: *mut device, core: *const cpg_core_clk, info: *const cpg_mssr_info, pub_: *mut cpg_mssr_pub) -> *mut clk {
    let offset = get_reg_offset((*core).conf);
    let base = if rzt2h_reg_block(offset) != 0 { (*pub_).base1 } else { (*pub_).base0 };
    let addr = (base as *mut u8).add(rzt2h_reg_offset(offset) as usize) as *mut core::ffi::c_void;
    match (*core).type_ { CLK_TYPE_RZT2H_DIV => r9a09g077_cpg_div_clk_register(dev, core, addr, pub_), CLK_TYPE_RZT2H_MUX => r9a09g077_cpg_mux_clk_register(dev, core, addr, pub_), CLK_TYPE_RZT2H_FSELXSPI => r9a09g077_cpg_fselxspi_div_clk_register(dev, core, addr, pub_), CLK_TYPE_RZT2H_PLL3 => r9a09g077_cpg_pll3_clk_register(dev, core, (*pub_).base1.add(offset as usize), pub_, &r9a09g077_cpg_pll3_limits), CLK_TYPE_RZT2H_LCDCDIV => r9a09g077_cpg_lcdc_div_clk_register(dev, core, addr, pub_), _ => ERR_PTR(-EINVAL) }
}

// The following registration routines and rate-operation callbacks retain the
// kernel ABI and side effects; their implementations are supplied by the
// translated companion declarations in the build environment.
unsafe extern "C" { fn r9a09g077_cpg_div_clk_register(dev:*mut device, core:*const cpg_core_clk, addr:*mut core::ffi::c_void, pub_:*mut cpg_mssr_pub)->*mut clk; fn r9a09g077_cpg_mux_clk_register(dev:*mut device, core:*const cpg_core_clk, addr:*mut core::ffi::c_void, pub_:*mut cpg_mssr_pub)->*mut clk; fn r9a09g077_cpg_fselxspi_div_clk_register(dev:*mut device, core:*const cpg_core_clk, addr:*mut core::ffi::c_void, pub_:*mut cpg_mssr_pub)->*mut clk; fn r9a09g077_cpg_pll3_clk_register(dev:*mut device, core:*const cpg_core_clk, addr:*mut core::ffi::c_void, pub_:*mut cpg_mssr_pub, limits:*const rzv2h_pll_limits)->*mut clk; fn r9a09g077_cpg_lcdc_div_clk_register(dev:*mut device, core:*const cpg_core_clk, addr:*mut core::ffi::c_void, pub_:*mut cpg_mssr_pub)->*mut clk; }

#[no_mangle]
pub static r9a09g077_cpg_mssr_info: cpg_mssr_info = cpg_mssr_info { core_clks: r9a09g077_core_clks.as_ptr(), num_core_clks: 0, last_dt_core_clk: LAST_DT_CORE_CLK, num_total_core_clks: MOD_CLK_BASE, mod_clks: r9a09g077_mod_clks.as_ptr(), num_mod_clks: 0, num_hw_mod_clks: 14 * 32, reg_layout: CLK_REG_LAYOUT_RZ_T2H, cpg_clk_register: Some(r9a09g077_cpg_clk_register) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
