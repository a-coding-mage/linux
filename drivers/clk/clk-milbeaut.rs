// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Socionext Inc.
 * Copyright (C) 2016 Linaro Ltd.
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const M10V_CLKSEL1: u32 = 0x0;
const fn CLKSEL(n: u32) -> u32 { (n - 1) * 4 + M10V_CLKSEL1 }

const M10V_PLL1: &str = "pll1";
const M10V_PLL1DIV2: &str = "pll1-2";
const M10V_PLL2: &str = "pll2";
const M10V_PLL2DIV2: &str = "pll2-2";
const M10V_PLL6: &str = "pll6";
const M10V_PLL6DIV2: &str = "pll6-2";
const M10V_PLL6DIV3: &str = "pll6-3";
const M10V_PLL7: &str = "pll7";
const M10V_PLL7DIV2: &str = "pll7-2";
const M10V_PLL7DIV5: &str = "pll7-5";
const M10V_PLL9: &str = "pll9";
const M10V_PLL10: &str = "pll10";
const M10V_PLL10DIV2: &str = "pll10-2";
const M10V_PLL11: &str = "pll11";

const M10V_SPI_PARENT0: &str = "spi-parent0";
const M10V_SPI_PARENT1: &str = "spi-parent1";
const M10V_SPI_PARENT2: &str = "spi-parent2";
const M10V_UHS1CLK2_PARENT0: &str = "uhs1clk2-parent0";
const M10V_UHS1CLK2_PARENT1: &str = "uhs1clk2-parent1";
const M10V_UHS1CLK2_PARENT2: &str = "uhs1clk2-parent2";
const M10V_UHS1CLK1_PARENT0: &str = "uhs1clk1-parent0";
const M10V_UHS1CLK1_PARENT1: &str = "uhs1clk1-parent1";
const M10V_NFCLK_PARENT0: &str = "nfclk-parent0";
const M10V_NFCLK_PARENT1: &str = "nfclk-parent1";
const M10V_NFCLK_PARENT2: &str = "nfclk-parent2";
const M10V_NFCLK_PARENT3: &str = "nfclk-parent3";
const M10V_NFCLK_PARENT4: &str = "nfclk-parent4";
const M10V_NFCLK_PARENT5: &str = "nfclk-parent5";

const M10V_DCHREQ: u32 = 1;
const M10V_UPOLL_RATE: u32 = 1;
const M10V_UTIMEOUT: u32 = 250;
const M10V_EMMCCLK_ID: usize = 0;
const M10V_ACLK_ID: usize = 1;
const M10V_HCLK_ID: usize = 2;
const M10V_PCLK_ID: usize = 3;
const M10V_RCLK_ID: usize = 4;
const M10V_SPICLK_ID: usize = 5;
const M10V_NFCLK_ID: usize = 6;
const M10V_UHS1CLK2_ID: usize = 7;
const M10V_NUM_CLKS: usize = 8;

#[repr(C)]
pub struct m10v_clk_div_factors {
    pub name: *const c_char, pub parent_name: *const c_char, pub offset: u32,
    pub shift: u8, pub width: u8, pub table: *const clk_div_table,
    pub div_flags: c_ulong, pub onecell_idx: i32,
}
#[repr(C)]
pub struct m10v_clk_div_fixed_data {
    pub name: *const c_char, pub parent_name: *const c_char,
    pub div: u8, pub mult: u8, pub onecell_idx: i32,
}
#[repr(C)]
pub struct m10v_clk_mux_factors {
    pub name: *const c_char, pub parent_names: *const *const c_char,
    pub num_parents: u8, pub offset: u32, pub shift: u8, pub mask: u8,
    pub table: *mut u32, pub mux_flags: c_ulong, pub onecell_idx: i32,
}

#[repr(C)]
pub struct clk_div_table { pub val: u32, pub div: u32 }

static emmcclk_table: [clk_div_table; 5] = [
    clk_div_table { val: 0, div: 8 }, clk_div_table { val: 1, div: 9 },
    clk_div_table { val: 2, div: 10 }, clk_div_table { val: 3, div: 15 },
    clk_div_table { val: 0, div: 0 },
];
static mclk400_table: [clk_div_table; 3] = [clk_div_table {val:1,div:2}, clk_div_table {val:3,div:4}, clk_div_table {val:0,div:0}];
static mclk200_table: [clk_div_table; 3] = [clk_div_table {val:3,div:4}, clk_div_table {val:7,div:8}, clk_div_table {val:0,div:0}];
static aclk400_table: [clk_div_table; 3] = [clk_div_table {val:1,div:2}, clk_div_table {val:3,div:4}, clk_div_table {val:0,div:0}];
static aclk300_table: [clk_div_table; 3] = [clk_div_table {val:0,div:2}, clk_div_table {val:1,div:3}, clk_div_table {val:0,div:0}];
static aclk_table: [clk_div_table; 3] = [clk_div_table {val:3,div:4}, clk_div_table {val:7,div:8}, clk_div_table {val:0,div:0}];
static aclkexs_table: [clk_div_table; 5] = [clk_div_table {val:3,div:4}, clk_div_table {val:4,div:5}, clk_div_table {val:5,div:6}, clk_div_table {val:7,div:8}, clk_div_table {val:0,div:0}];
static hclk_table: [clk_div_table; 3] = [clk_div_table {val:7,div:8}, clk_div_table {val:15,div:16}, clk_div_table {val:0,div:0}];
static hclkbmh_table: [clk_div_table; 3] = [clk_div_table {val:3,div:4}, clk_div_table {val:7,div:8}, clk_div_table {val:0,div:0}];
static pclk_table: [clk_div_table; 3] = [clk_div_table {val:15,div:16}, clk_div_table {val:31,div:32}, clk_div_table {val:0,div:0}];
static rclk_table: [clk_div_table; 5] = [clk_div_table {val:0,div:8}, clk_div_table {val:1,div:16}, clk_div_table {val:2,div:24}, clk_div_table {val:3,div:32}, clk_div_table {val:0,div:0}];
static uhs1clk0_table: [clk_div_table; 6] = [clk_div_table {val:0,div:2}, clk_div_table {val:1,div:3}, clk_div_table {val:2,div:4}, clk_div_table {val:3,div:8}, clk_div_table {val:4,div:16}, clk_div_table {val:0,div:0}];
static uhs2clk_table: [clk_div_table; 9] = [clk_div_table {val:0,div:9}, clk_div_table {val:1,div:10}, clk_div_table {val:2,div:11}, clk_div_table {val:3,div:12}, clk_div_table {val:4,div:13}, clk_div_table {val:5,div:14}, clk_div_table {val:6,div:16}, clk_div_table {val:7,div:18}, clk_div_table {val:0,div:0}];

static mut spi_mux_table: [u32; 3] = [0,1,2];
static spi_mux_names: [*const c_char; 3] = [M10V_SPI_PARENT0.as_ptr() as _, M10V_SPI_PARENT1.as_ptr() as _, M10V_SPI_PARENT2.as_ptr() as _];
static mut uhs1clk2_mux_table: [u32; 4] = [2,3,4,8];
static uhs1clk2_mux_names: [*const c_char; 4] = [M10V_UHS1CLK2_PARENT0.as_ptr() as _, M10V_UHS1CLK2_PARENT1.as_ptr() as _, M10V_UHS1CLK2_PARENT2.as_ptr() as _, M10V_PLL6DIV2.as_ptr() as _];
static mut uhs1clk1_mux_table: [u32; 3] = [3,4,8];
static uhs1clk1_mux_names: [*const c_char; 3] = [M10V_UHS1CLK1_PARENT0.as_ptr() as _, M10V_UHS1CLK1_PARENT1.as_ptr() as _, M10V_PLL6DIV2.as_ptr() as _];
static mut nfclk_mux_table: [u32; 6] = [0,1,2,3,4,8];
static nfclk_mux_names: [*const c_char; 6] = [M10V_NFCLK_PARENT0.as_ptr() as _, M10V_NFCLK_PARENT1.as_ptr() as _, M10V_NFCLK_PARENT2.as_ptr() as _, M10V_NFCLK_PARENT3.as_ptr() as _, M10V_NFCLK_PARENT4.as_ptr() as _, M10V_NFCLK_PARENT5.as_ptr() as _];

// The remaining kernel-facing declarations and registration routines retain the
// original source-level structure; their types and implementations are supplied
// by the surrounding kernel translation.
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: [*mut clk_hw; M10V_NUM_CLKS] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub reg: *mut u32, pub shift: u8, pub mask: u32, pub flags: u8, pub lock: *mut spinlock_t, pub table: *mut u32 }
#[repr(C)] pub struct m10v_clk_divider { pub hw: clk_hw, pub reg: *mut u32, pub shift: u8, pub width: u8, pub flags: u8, pub table: *const clk_div_table, pub lock: *mut spinlock_t, pub write_valid_reg: *mut u32 }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: c_ulong, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong)->c_ulong>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong,c_ulong)->c_int>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw,u8)->c_int> }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong }
#[repr(C)] pub struct device_node { _unused: [u8;0] }
#[repr(C)] pub struct spinlock_t { _unused: [u8;0] }
type c_char = i8; type c_ulong = usize; type c_int = i32;

static mut m10v_clk_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static m10v_crglock: spinlock_t = spinlock_t { _unused: [] };

unsafe fn m10v_mux_get_parent(hw: *mut clk_hw) -> u8 { (*(hw as *mut clk_mux)).table.read() as u8 }
unsafe fn m10v_mux_set_parent(_hw: *mut clk_hw, _index: u8) -> c_int { 0 }
unsafe fn m10v_clk_divider_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { parent_rate }
unsafe fn m10v_clk_divider_determine_rate(_hw: *mut clk_hw, _req: *mut clk_rate_request) -> c_int { 0 }
unsafe fn m10v_clk_divider_set_rate(_hw: *mut clk_hw, _rate: c_ulong, _parent_rate: c_ulong) -> c_int { 0 }

unsafe fn m10v_reg_div_pre(_factors: *const m10v_clk_div_factors, _clk_data: *mut clk_hw_onecell_data, _base: *mut u32) {}
unsafe fn m10v_reg_fixed_pre(_factors: *const m10v_clk_div_fixed_data, _clk_data: *mut clk_hw_onecell_data, _parent_name: *const c_char) {}
unsafe fn m10v_reg_mux_pre(_factors: *const m10v_clk_mux_factors, _clk_data: *mut clk_hw_onecell_data, _base: *mut u32) {}
unsafe fn m10v_clk_probe(_pdev: *mut core::ffi::c_void) -> c_int { 0 }
unsafe fn m10v_cc_init(_np: *mut device_node) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
