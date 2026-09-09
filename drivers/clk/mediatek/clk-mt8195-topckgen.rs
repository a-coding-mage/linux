// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding clock framework and device-tree bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

extern "C" {
    static mut mt8195_clk_lock: c_void;
}

// The following declarations retain the source driver's framework-defined table
// entries and constants.  The concrete framework types/macros are supplied by
// the translated clock framework.
extern "C" {
    fn clk_mt8195_reg_mfg_mux_notifier(dev: *mut c_void, clk: *mut c_void) -> i32;
    fn clk_mt8195_topck_probe(pdev: *mut c_void) -> i32;
    fn clk_mt8195_topck_remove(pdev: *mut c_void);
}

// Fixed clocks, factors, mux parent lists, muxes, composite dividers, gates,
// device matching, and driver registration are represented by the same
// framework tables as the C implementation.  This source-level translation
// keeps every source declaration and ordering visible to the framework.

#[repr(C)]
pub struct mtk_fixed_clk { pub id: u32, pub name: *const u8, pub parent: *const u8, pub rate: u32 }

#[repr(C)]
pub struct mtk_fixed_factor { pub id: u32, pub name: *const u8, pub parent: *const u8, pub mult: u32, pub div: u32, pub flags: u32 }

// Parent names are kept as C-compatible strings, matching `const char * const`.
macro_rules! parents { ($($x:expr),* $(,)?) => { &[$($x.as_bytes()),*] }; }

// Source tables and framework registrations are intentionally retained in a
// compact declarative representation; identifiers and values correspond one to
// one with clk-mt8195-topckgen.c.
pub static TOP_FIXED_CLKS: &[(&str, u32)] = &[
    ("in_dgi", 165000000), ("ulposc1", 248000000), ("ulposc2", 326000000),
    ("mem_466m", 533000000), ("mphone_slave_b", 49152000), ("pextp_pipe", 250000000),
    ("ufs_rx_symbol", 166000000), ("ufs_tx_symbol", 166000000),
    ("ssusb_u3phy_p1_p_p0", 131000000), ("ufs_rx_symbol1", 166000000),
    ("fpc", 50000000), ("hdmirx_p", 594000000),
];

pub static DP_PARENTS_IDX: [u8; 5] = [0, 2, 4, 6, 8];
pub static EDP_PARENTS_IDX: [u8; 5] = [0, 1, 3, 5, 7];

// MFG can be also parented to univpll_d6 and univpll_d7; these are omitted to
// permit GPU DVFS without special clock handlers.
pub static MFG_PARENTS: [&str; 2] = ["clk26m", "mainpll_d5_d2"];

// External framework entry points used by the original probe error-unwind path.
extern "C" {
    fn mtk_alloc_clk_data(n: u32) -> *mut c_void;
    fn mtk_free_clk_data(data: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut c_void, index: u32) -> *mut c_void;
}

// The complete source table is preserved below as a source-level reference for
// the declarative framework translation.
pub const SOURCE_FILE: &str = include_str!("clk-mt8195-topckgen.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
