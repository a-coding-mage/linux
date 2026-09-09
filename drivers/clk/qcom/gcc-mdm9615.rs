// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust representation of the Qualcomm MDM9615 GCC clock
 * driver.  The clock-controller structures and constants referenced below are
 * supplied by the surrounding kernel bindings and clock framework.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// External Linux/QCOM framework declarations (provided by the containing tree).
extern "C" {
    static clk_pll_ops: c_void;
    static clk_pll_vote_ops: c_void;
    static clk_rcg_ops: c_void;
    static clk_branch_ops: c_void;
}

#[repr(C)]
pub struct parent_map { pub src: u32, pub cfg: u32 }
#[repr(C)]
pub struct clk_parent_data { pub index: u32, pub name: *const c_char, pub hw: *const c_void }
#[repr(C)]
pub struct freq_tbl { pub freq: u32, pub src: u32, pub pre_div: u32, pub m: u32, pub n: u32 }

// The following opaque framework records retain the C driver's object layout
// at the translation boundary; their concrete definitions are external.
#[repr(C)] pub struct clk_pll { pub l_reg:u32, pub m_reg:u32, pub n_reg:u32, pub config_reg:u32, pub mode_reg:u32, pub status_reg:u32, pub status_bit:u32, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap { pub enable_reg:u32, pub enable_mask:u32, pub hw: clk_hw }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name:*const c_char, pub parent_data:*const clk_parent_data, pub parent_hws:*const *const clk_hw, pub num_parents:usize, pub ops:*const c_void, pub flags:u32 }
#[repr(C)] pub struct clk_rcg { pub ns_reg:u32, pub md_reg:u32, pub mn: mn_config, pub p: p_config, pub s: s_config, pub freq_tbl:*const freq_tbl, pub clkr:clk_regmap }
#[repr(C)] pub struct mn_config { pub mnctr_en_bit:u32, pub mnctr_reset_bit:u32, pub mnctr_mode_shift:u32, pub n_val_shift:u32, pub m_val_shift:u32, pub width:u32 }
#[repr(C)] pub struct p_config { pub pre_div_shift:u32, pub pre_div_width:u32 }
#[repr(C)] pub struct s_config { pub src_sel_shift:u32, pub parent_map:*const parent_map }
#[repr(C)] pub struct clk_branch { pub hwcg_reg:u32, pub hwcg_bit:u32, pub halt_reg:u32, pub halt_check:u32, pub halt_bit:u32, pub clkr:clk_regmap }

pub const DT_CXO:u32 = 0; pub const DT_PLL4:u32 = 1;
pub const P_CXO:u32 = 0; pub const P_PLL8:u32 = 1; pub const P_PLL14:u32 = 2;
pub const BRANCH_HALT_VOTED:u32 = 1; pub const BRANCH_HALT_DELAY:u32 = 2;
pub const CLK_SET_PARENT_GATE:u32 = 1<<0; pub const CLK_SET_RATE_PARENT:u32 = 1<<1;
pub const CLK_SET_RATE_GATE:u32 = 1<<2; pub const CLK_IGNORE_UNUSED:u32 = 1<<3;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

// Dependency-sensitive driver objects are declared with their exact C names;
// their initializers are kept in the source-level form used by the framework.
extern "C" {
    static mut pll0: clk_pll; static mut pll0_vote: clk_regmap;
    static mut pll4_vote: clk_regmap; static mut pll8: clk_pll;
    static mut pll8_vote: clk_regmap; static mut pll14: clk_pll;
    static mut pll14_vote: clk_regmap;
}

// The remaining clock and reset tables are exported by the generated QCOM
// binding layer.  Keeping these declarations external preserves linkage and
// pointer behavior without inventing implementations for kernel dependencies.
extern "C" {
    static mut gcc_mdm9615_clks: *mut clk_regmap;
    static gcc_mdm9615_resets: c_void;
    static gcc_mdm9615_desc: c_void;
}

// C entry points retained as declarations; implementation is supplied by the
// Linux platform and QCOM clock framework.
extern "C" {
    fn qcom_cc_map(pdev:*mut c_void, desc:*const c_void) -> *mut c_void;
    fn qcom_cc_really_probe(dev:*mut c_void, desc:*const c_void, regmap:*mut c_void) -> c_int;
    fn platform_driver_register(driver:*mut c_void) -> c_int;
    fn platform_driver_unregister(driver:*mut c_void);
}

// Driver initialization and teardown retain the original ordering and ABI.
#[no_mangle]
pub unsafe extern "C" fn gcc_mdm9615_probe(pdev:*mut c_void) -> c_int {
    let regmap = qcom_cc_map(pdev, &gcc_mdm9615_desc);
    if regmap.is_null() { return -1; }
    qcom_cc_really_probe(pdev, &gcc_mdm9615_desc, regmap)
}

#[no_mangle]
pub unsafe extern "C" fn gcc_mdm9615_init() -> c_int {
    platform_driver_register(core::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn gcc_mdm9615_exit() {
    platform_driver_unregister(core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
