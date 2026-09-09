// SPDX-License-Identifier: GPL-2.0
/* SAMA7G5 PMC code, translated from sama7g5.c. */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct clk_pll_layout { pub mul_mask: u32, pub frac_mask: u32, pub div_mask: u32, pub endiv_mask: u32, pub mul_shift: u8, pub frac_shift: u8, pub div_shift: u8, pub endiv_shift: u8 }
#[repr(C)]
pub struct clk_range { pub min: u64, pub max: u64 }
#[repr(C)]
pub struct clk_pll_characteristics { pub input: clk_range, pub num_output: usize, pub output: *const clk_range, pub core_output: *const clk_range, pub acr: u32 }
#[repr(C)]
pub struct clk_hw;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct sama7g5_pll { pub n: *const c_char, pub l: *const clk_pll_layout, pub c: *const clk_pll_characteristics, pub hw: *mut clk_hw, pub f: c_ulong, pub p: sama7g5_pll_parent, pub t: u8, pub eid: u8, pub safe_div: u8 }
pub type c_ulong = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sama7g5_pll_parent { SAMA7G5_PLL_PARENT_MAINCK, SAMA7G5_PLL_PARENT_MAIN_XTAL, SAMA7G5_PLL_PARENT_FRACCK }
pub const PLL_ID_CPU: usize = 0; pub const PLL_ID_SYS: usize = 1; pub const PLL_ID_DDR: usize = 2; pub const PLL_ID_IMG: usize = 3; pub const PLL_ID_BAUD: usize = 4; pub const PLL_ID_AUDIO: usize = 5; pub const PLL_ID_ETH: usize = 6; pub const PLL_ID_MAX: usize = 7;
pub const PLL_COMPID_FRAC: usize = 0; pub const PLL_COMPID_DIV0: usize = 1; pub const PLL_COMPID_DIV1: usize = 2; pub const PLL_COMPID_MAX: usize = 3;
pub const PLL_TYPE_FRAC: u8 = 0; pub const PLL_TYPE_DIV: u8 = 1;
pub const PMC_MAIN: usize = 0; pub const PMC_MCK: usize = 1; pub const PMC_CPUPLL: usize = 2; pub const PMC_SYSPLL: usize = 3; pub const PMC_AUDIOPMCPLL: usize = 4; pub const PMC_AUDIOIOPLL: usize = 5; pub const PMC_UTMI: usize = 6;
pub const CLK_IS_CRITICAL: usize = 1 << 0; pub const CLK_SET_RATE_PARENT: usize = 1 << 1; pub const CLK_SET_RATE_GATE: usize = 1 << 2; pub const CLK_SET_PARENT_GATE: usize = 1 << 3; pub const CLK_GET_RATE_NOCACHE: usize = 1 << 4;

static PLL_LAYOUT_FRAC: clk_pll_layout = clk_pll_layout { mul_mask: 0xff000000, frac_mask: 0x003fffff, div_mask: 0, endiv_mask: 0, mul_shift: 24, frac_shift: 0, div_shift: 0, endiv_shift: 0 };
static PLL_LAYOUT_DIVPMC: clk_pll_layout = clk_pll_layout { mul_mask: 0, frac_mask: 0, div_mask: 0xff, endiv_mask: 1 << 29, mul_shift: 0, frac_shift: 0, div_shift: 0, endiv_shift: 29 };
static PLL_LAYOUT_DIVIO: clk_pll_layout = clk_pll_layout { mul_mask: 0, frac_mask: 0, div_mask: 0xff000, endiv_mask: 1 << 30, mul_shift: 0, frac_shift: 0, div_shift: 12, endiv_shift: 30 };
static CPU_OUTPUTS: [clk_range; 1] = [clk_range { min: 2343750, max: 1000000002 }];
static PLL_OUTPUTS: [clk_range; 1] = [clk_range { min: 2343750, max: 1200000000 }];
static CORE_OUTPUTS: [clk_range; 1] = [clk_range { min: 600000000, max: 1200000000 }];
static CPU_CHARS: clk_pll_characteristics = clk_pll_characteristics { input: clk_range { min: 12000000, max: 50000000 }, num_output: 1, output: CPU_OUTPUTS.as_ptr(), core_output: CORE_OUTPUTS.as_ptr(), acr: 0x00070010 };
static PLL_CHARS: clk_pll_characteristics = clk_pll_characteristics { input: clk_range { min: 12000000, max: 50000000 }, num_output: 1, output: PLL_OUTPUTS.as_ptr(), core_output: CORE_OUTPUTS.as_ptr(), acr: 0x00070010 };

macro_rules! pll { ($n:literal, $p:expr, $l:expr, $c:expr, $t:expr, $f:expr, $eid:expr, $safe:expr) => { sama7g5_pll { n: concat!($n, "\0").as_ptr() as *const c_char, p: $p, l: &$l, c: &$c, hw: core::ptr::null_mut(), t: $t, f: $f, eid: $eid, safe_div: $safe } } }
static mut SAMA7G5_PLLS: [[sama7g5_pll; PLL_COMPID_MAX]; PLL_ID_MAX] = [
 [pll!("cpupll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, PLL_LAYOUT_FRAC, CPU_CHARS, PLL_TYPE_FRAC, CLK_IS_CRITICAL, 0, 0), pll!("cpupll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, CPU_CHARS, PLL_TYPE_DIV, CLK_IS_CRITICAL|CLK_SET_RATE_PARENT, 2, 15), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
 [pll!("syspll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_IS_CRITICAL|CLK_SET_RATE_GATE, 0, 0), pll!("syspll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_IS_CRITICAL|CLK_SET_RATE_GATE, 3, 0), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
 [pll!("ddrpll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_IS_CRITICAL|CLK_SET_RATE_GATE, 0, 0), pll!("ddrpll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_IS_CRITICAL|CLK_SET_RATE_GATE, 0, 0), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
 [pll!("imgpll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_SET_RATE_GATE, 0, 0), pll!("imgpll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT, 0, 0), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
 [pll!("baudpll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_SET_RATE_GATE, 0, 0), pll!("baudpll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT, 0, 0), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
 [pll!("audiopll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAIN_XTAL, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_SET_RATE_GATE, 0, 0), pll!("audiopll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT, 4, 0), pll!("audiopll_diviock", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVIO, PLL_CHARS, PLL_TYPE_DIV, CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT, 5, 0) ],
 [pll!("ethpll_fracck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAIN_XTAL, PLL_LAYOUT_FRAC, PLL_CHARS, PLL_TYPE_FRAC, CLK_SET_RATE_GATE, 0, 0), pll!("ethpll_divpmcck", sama7g5_pll_parent::SAMA7G5_PLL_PARENT_FRACCK, PLL_LAYOUT_DIVPMC, PLL_CHARS, PLL_TYPE_DIV, CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT, 0, 0), sama7g5_pll { n: core::ptr::null(), l: core::ptr::null(), c: core::ptr::null(), hw: core::ptr::null_mut(), f: 0, p: sama7g5_pll_parent::SAMA7G5_PLL_PARENT_MAINCK, t: 0, eid: 0, safe_div: 0 } ],
];

// The remaining registration data and setup retain the source's external PMC API.
extern "C" { fn sama7g5_pmc_setup(np: *mut device_node); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
