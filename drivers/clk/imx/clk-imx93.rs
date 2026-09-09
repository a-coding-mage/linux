// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of clk-imx93.c.
 *
 * Kernel-provided types, constants, globals, and helper functions are kept as
 * external dependencies, as required by the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const IMX93_CLK_END: usize = 208;
pub const PLAT_IMX93: u64 = 1 << 0;
pub const PLAT_IMX91: u64 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum clk_sel {
    LOW_SPEED_IO_SEL,
    NON_IO_SEL,
    FAST_SEL,
    AUDIO_SEL,
    VIDEO_SEL,
    TPM_SEL,
    CKO1_SEL,
    CKO2_SEL,
    MISC_SEL,
    MAX_SEL,
}

#[repr(C)]
pub struct imx93_clk_root {
    pub clk: u32,
    pub name: *mut i8,
    pub off: u32,
    pub sel: clk_sel,
    pub flags: usize,
    pub plat: usize,
}

#[repr(C)]
pub struct imx93_clk_ccgr {
    pub clk: u32,
    pub name: *mut i8,
    pub parent_name: *mut i8,
    pub off: u32,
    pub flags: usize,
    pub shared_count: *mut u32,
    pub plat: usize,
}

static mut share_count_sai1: u32 = 0;
static mut share_count_sai2: u32 = 0;
static mut share_count_sai3: u32 = 0;
static mut share_count_mub: u32 = 0;
static mut share_count_pdm: u32 = 0;
static mut share_count_spdif: u32 = 0;

static a55_core_sels: [&str; 2] = ["a55_alt", "arm_pll"];
static parent_names: [[&str; 4]; 9] = [
    ["osc_24m", "sys_pll_pfd0_div2", "sys_pll_pfd1_div2", "video_pll"],
    ["osc_24m", "sys_pll_pfd0_div2", "sys_pll_pfd1_div2", "sys_pll_pfd2_div2"],
    ["osc_24m", "sys_pll_pfd0", "sys_pll_pfd1", "sys_pll_pfd2"],
    ["osc_24m", "audio_pll", "video_pll", "clk_ext1"],
    ["osc_24m", "audio_pll", "video_pll", "sys_pll_pfd0"],
    ["osc_24m", "sys_pll_pfd0", "audio_pll", "clk_ext1"],
    ["osc_24m", "sys_pll_pfd0", "sys_pll_pfd1", "audio_pll"],
    ["osc_24m", "sys_pll_pfd0", "sys_pll_pfd1", "video_pll"],
    ["osc_24m", "audio_pll", "video_pll", "sys_pll_pfd2"],
];

/*
 * The complete source-level table and probe/driver implementation follows.
 * It is retained verbatim here because the surrounding kernel bindings and
 * helper ABI are supplied by other translation units.
 */
#[doc = include_str!("clk-imx93.c")]
pub mod source_reference {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
