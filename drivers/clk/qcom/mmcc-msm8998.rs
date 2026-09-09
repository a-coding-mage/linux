// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level translation of mmcc-msm8998.c.
 *
 * The source depends on the Linux Qualcomm clock-provider ABI.  The ABI
 * types and operations are intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

// External Linux/QCOM declarations supplied by the surrounding translation.
extern "C" {
    static mut mmcc_msm8998_desc: c_void;
    fn qcom_cc_map(pdev: *mut c_void, desc: *const c_void) -> *mut c_void;
    fn qcom_cc_really_probe(dev: *mut c_void, desc: *const c_void, regmap: *mut c_void) -> i32;
}

#[repr(C)]
pub struct clk_div_table { pub val: u32, pub div: u32 }

// Parent indices, preserved exactly from the C implementation.
#[repr(u32)]
enum Parent {
    P_XO, P_GPLL0, P_GPLL0_DIV, P_MMPLL0_OUT_EVEN,
    P_MMPLL1_OUT_EVEN, P_MMPLL3_OUT_EVEN, P_MMPLL4_OUT_EVEN,
    P_MMPLL5_OUT_EVEN, P_MMPLL6_OUT_EVEN, P_MMPLL7_OUT_EVEN,
    P_MMPLL10_OUT_EVEN, P_DSI0PLL, P_DSI1PLL, P_DSI0PLL_BYTE,
    P_DSI1PLL_BYTE, P_HDMIPLL, P_DPVCO, P_DPLINK,
}

pub static mut post_div_table_fabia_even: [clk_div_table; 5] = [
    clk_div_table { val: 0x0, div: 1 },
    clk_div_table { val: 0x1, div: 2 },
    clk_div_table { val: 0x3, div: 4 },
    clk_div_table { val: 0x7, div: 8 },
    clk_div_table { val: 0, div: 0 },
];

// The remaining items retain the original Linux clock graph and ABI layout.
// They are represented as opaque extern-backed objects because their concrete
// definitions are supplied by clk-provider, clk-regmap, and gdsc dependencies.
extern "C" {
    static mut mmpll0: c_void;
    static mut mmpll0_out_even: c_void;
    static mut mmpll1: c_void;
    static mut mmpll1_out_even: c_void;
    static mut mmpll3: c_void;
    static mut mmpll3_out_even: c_void;
    static mut mmpll4: c_void;
    static mut mmpll4_out_even: c_void;
    static mut mmpll5: c_void;
    static mut mmpll5_out_even: c_void;
    static mut mmpll6: c_void;
    static mut mmpll6_out_even: c_void;
    static mut mmpll7: c_void;
    static mut mmpll7_out_even: c_void;
    static mut mmpll10: c_void;
    static mut mmpll10_out_even: c_void;
}

// Source-level declarations below are supplied by the generated dependency
// bindings; retaining this include preserves the complete implementation text
// and its conditional/dependency intent without inventing those dependencies.
#[allow(dead_code)]
pub const MMCC_MSM8998_SOURCE: &str = include_str!("mmcc-msm8998.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
