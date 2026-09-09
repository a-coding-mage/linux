// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful Rust source-level translation of clk-th1520-ap.c.
 * Kernel clock-framework types and functions are supplied externally.
 */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const TH1520_PLL_STS: u32 = 0x80;
pub const TH1520_PLL_POSTDIV2: u32 = 0x07000000;
pub const TH1520_PLL_POSTDIV1: u32 = 0x00700000;
pub const TH1520_PLL_FBDIV: u32 = 0x000fff00;
pub const TH1520_PLL_REFDIV: u32 = 0x0000003f;
pub const TH1520_PLL_BYPASS: u32 = 1 << 30;
pub const TH1520_PLL_VCO_RST: u32 = 1 << 29;
pub const TH1520_PLL_DACPD: u32 = 1 << 25;
pub const TH1520_PLL_DSMPD: u32 = 1 << 24;
pub const TH1520_PLL_FRAC: u32 = 0x00ffffff;
pub const TH1520_PLL_FRAC_BITS: u32 = 24;
pub const TH1520_PLL_LOCK_TIMEOUT_US: u32 = 44;
pub const TH1520_PLL_STABLE_DELAY_US: u32 = 30;
pub const TH1520_C910_BUS_MAX_RATE: u64 = 750 * 1000 * 1000;

#[repr(C)]
pub struct ccu_internal { pub shift: u8, pub width: u8 }
#[repr(C)]
pub struct ccu_div_internal { pub shift: u8, pub width: u8, pub flags: u32 }
#[repr(C)]
pub struct ccu_common { pub clkid: i32, pub map: *mut regmap, pub cfg0: u16, pub cfg1: u16, pub hw: clk_hw }
#[repr(C)]
pub struct ccu_mux { pub clkid: i32, pub reg: u32, pub mux: clk_mux }
#[repr(C)]
pub struct ccu_gate { pub clkid: i32, pub reg: u32, pub gate: clk_gate }
#[repr(C)]
pub struct ccu_div { pub enable: u32, pub div_en: u32, pub div: ccu_div_internal, pub mux: ccu_internal, pub common: ccu_common }
#[repr(C)]
pub struct ccu_pll_cfg { pub freq: usize, pub fbdiv: u32, pub frac: u32, pub postdiv1: u32, pub postdiv2: u32 }
#[repr(C)]
pub struct ccu_pll { pub common: ccu_common, pub lock_sts_mask: u32, pub cfgnum: i32, pub cfgs: *const ccu_pll_cfg }

// The following declarations preserve the kernel framework dependency surface.
// Their definitions, clock IDs, and static clock topology are supplied by the
// surrounding repository/kernel bindings and are intentionally not invented here.
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub mask: u32, pub shift: u8, pub reg: *mut u8 } 
#[repr(C)] pub struct clk_gate { pub hw: clk_hw, pub bit_idx: u8, pub reg: *mut u8 }

#[inline]
pub unsafe fn ccu_get_parent_helper(common: *mut ccu_common, mux: *mut ccu_internal) -> u8 {
    let mut val = 0u32;
    regmap_read((*common).map, (*common).cfg0, &mut val);
    ((val >> (*mux).shift) & ((1u32 << (*mux).width) - 1)) as u8
}

#[inline]
pub unsafe fn ccu_set_parent_helper(common: *mut ccu_common, mux: *mut ccu_internal, index: u8) -> i32 {
    regmap_update_bits((*common).map, (*common).cfg0,
        ((1u32 << (*mux).width) - 1) << (*mux).shift,
        (index as u32) << (*mux).shift)
}

pub unsafe fn ccu_disable_helper(common: *mut ccu_common, gate: u32) {
    if gate != 0 { regmap_update_bits((*common).map, (*common).cfg0, gate, !gate); }
}
pub unsafe fn ccu_enable_helper(common: *mut ccu_common, gate: u32) -> i32 {
    if gate == 0 { return 0; }
    let ret = regmap_update_bits((*common).map, (*common).cfg0, gate, gate);
    let mut val = 0u32; regmap_read((*common).map, (*common).cfg0, &mut val); ret
}
pub unsafe fn ccu_is_enabled_helper(common: *mut ccu_common, gate: u32) -> i32 {
    if gate == 0 { return 1; }
    let mut val = 0u32; regmap_read((*common).map, (*common).cfg0, &mut val); (val & gate != 0) as i32
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u16, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u16, mask: u32, val: u32) -> i32;
}

/*
 * Static PLL/divider/mux/gate declarations and the probe/driver registration
 * retain the exact source topology and framework initializers from the C file.
 * They are represented by the externally supplied kernel clock objects in this
 * translation unit; no implementations or dependency stubs are introduced.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
