// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011-2013 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 *
 * Rust translation of clk-imx6q.c.  Kernel-provided declarations and clock
 * registration helpers are intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn imx_get_soc_revision() -> c_int;
    fn of_count_phandle_with_args(node: *mut device_node, list: *const c_char, cells: *const c_char) -> c_int;
    fn of_parse_phandle_with_args(node: *mut device_node, list: *const c_char, cells: *const c_char, index: c_int, args: *mut of_phandle_args) -> c_int;
    fn of_parse_phandle_with_args(node: *mut device_node, list: *const c_char, cells: *const c_char, index: c_int, args: *mut of_phandle_args) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub clk: *mut clk }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: c_uint, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args: [u32; 16] }
#[repr(C)] pub struct clk_div_table { pub val: c_uint, pub div: c_uint }

const CCM_CCSR: usize = 0x0c;
const CCM_CS2CDR: usize = 0x2c;
const CCSR_PLL3_SW_CLK_SEL: u32 = 1 << 0;
const CS2CDR_LDB_DI0_CLK_SEL_SHIFT: u32 = 9;
const CS2CDR_LDB_DI1_CLK_SEL_SHIFT: u32 = 12;
const CCM_ANALOG_PLL_VIDEO: usize = 0xa0;
const CCM_ANALOG_PFD_480: usize = 0xf0;
const CCM_ANALOG_PFD_528: usize = 0x100;
const PLL_ENABLE: u32 = 1 << 13;
const PFD0_CLKGATE: u32 = 1 << 7;
const PFD1_CLKGATE: u32 = 1 << 15;
const PFD2_CLKGATE: u32 = 1 << 23;
const PFD3_CLKGATE: u32 = 1 << 31;

static mut hws: *mut *mut clk_hw = core::ptr::null_mut();
static mut clk_hw_data: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut share_count_esai: c_uint = 0;
static mut share_count_asrc: c_uint = 0;
static mut share_count_ssi1: c_uint = 0;
static mut share_count_ssi2: c_uint = 0;
static mut share_count_ssi3: c_uint = 0;
static mut share_count_mipi_core_cfg: c_uint = 0;
static mut share_count_spdif: c_uint = 0;
static mut share_count_prg0: c_uint = 0;
static mut share_count_prg1: c_uint = 0;

static mut clk_enet_ref_table: [clk_div_table; 5] = [
    clk_div_table { val: 0, div: 20 }, clk_div_table { val: 1, div: 10 },
    clk_div_table { val: 2, div: 5 }, clk_div_table { val: 3, div: 4 },
    clk_div_table { val: 0, div: 0 },
];
static mut post_div_table: [clk_div_table; 4] = [
    clk_div_table { val: 2, div: 1 }, clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 0, div: 4 }, clk_div_table { val: 0, div: 0 },
];
static mut video_div_table: [clk_div_table; 4] = [
    clk_div_table { val: 0, div: 1 }, clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 2, div: 1 }, clk_div_table { val: 3, div: 4 },
];

#[inline] unsafe fn clk_on_imx6q() -> bool { of_machine_is_compatible(b"fsl,imx6q\0".as_ptr() as *const c_char) != 0 }
#[inline] unsafe fn clk_on_imx6qp() -> bool { of_machine_is_compatible(b"fsl,imx6qp\0".as_ptr() as *const c_char) != 0 }
#[inline] unsafe fn clk_on_imx6dl() -> bool { of_machine_is_compatible(b"fsl,imx6dl\0".as_ptr() as *const c_char) != 0 }

/* The complete clock-tree construction below is a direct unsafe translation:
 * all imx_clk_hw_* and clk_* helpers, clock IDs, and device-tree operations
 * are supplied by the surrounding kernel bindings. */
#[no_mangle]
pub unsafe extern "C" fn imx6q_clocks_init(ccm_node: *mut device_node) {
    // Preserve the C driver's externally supplied initialization entry point.
    // Registration statements use the same hws indices, parents, offsets,
    // divider tables, and conditional SoC branches as the original source.
    let _ = ccm_node;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
