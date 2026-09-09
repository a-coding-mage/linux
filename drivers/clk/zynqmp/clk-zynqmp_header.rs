/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2016-2018 Xilinx
 */

// Dependencies supplied by the surrounding translation unit:
// linux/spinlock.h and linux/firmware/xlnx-zynqmp.h

/* Common Flags */
/* must be gated across rate change */
pub const ZYNQMP_CLK_SET_RATE_GATE: u32 = 1u32 << 0;
/* must be gated across re-parent */
pub const ZYNQMP_CLK_SET_PARENT_GATE: u32 = 1u32 << 1;
/* propagate rate change up one level */
pub const ZYNQMP_CLK_SET_RATE_PARENT: u32 = 1u32 << 2;
/* do not gate even if unused */
pub const ZYNQMP_CLK_IGNORE_UNUSED: u32 = 1u32 << 3;
/* don't re-parent on rate change */
pub const ZYNQMP_CLK_SET_RATE_NO_REPARENT: u32 = 1u32 << 7;
/* do not gate, ever */
pub const ZYNQMP_CLK_IS_CRITICAL: u32 = 1u32 << 11;

/* Type Flags for divider clock */
pub const ZYNQMP_CLK_DIVIDER_ONE_BASED: u32 = 1u32 << 0;
pub const ZYNQMP_CLK_DIVIDER_POWER_OF_TWO: u32 = 1u32 << 1;
pub const ZYNQMP_CLK_DIVIDER_ALLOW_ZERO: u32 = 1u32 << 2;
pub const ZYNQMP_CLK_DIVIDER_HIWORD_MASK: u32 = 1u32 << 3;
pub const ZYNQMP_CLK_DIVIDER_ROUND_CLOSEST: u32 = 1u32 << 4;
pub const ZYNQMP_CLK_DIVIDER_READ_ONLY: u32 = 1u32 << 5;
pub const ZYNQMP_CLK_DIVIDER_MAX_AT_ZERO: u32 = 1u32 << 6;

/* Type Flags for mux clock */
pub const ZYNQMP_CLK_MUX_INDEX_ONE: u32 = 1u32 << 0;
pub const ZYNQMP_CLK_MUX_INDEX_BIT: u32 = 1u32 << 1;
pub const ZYNQMP_CLK_MUX_HIWORD_MASK: u32 = 1u32 << 2;
pub const ZYNQMP_CLK_MUX_READ_ONLY: u32 = 1u32 << 3;
pub const ZYNQMP_CLK_MUX_ROUND_CLOSEST: u32 = 1u32 << 4;
pub const ZYNQMP_CLK_MUX_BIG_ENDIAN: u32 = 1u32 << 5;

#[repr(C)]
pub enum topology_type {
    TYPE_INVALID,
    TYPE_MUX,
    TYPE_PLL,
    TYPE_FIXEDFACTOR,
    TYPE_DIV1,
    TYPE_DIV2,
    TYPE_GATE,
}

/**
 * struct clock_topology - Clock topology
 * @type: Type of topology
 * @flag: Topology flags
 * @type_flag: Topology type specific flag
 * @custom_type_flag: Topology type specific custom flag
 */
#[repr(C)]
pub struct clock_topology {
    pub r#type: u32,
    pub flag: u32,
    pub type_flag: u32,
    pub custom_type_flag: u8,
}

extern "C" {
    pub fn zynqmp_clk_map_common_ccf_flags( zynqmp_flag: u32) -> ::core::ffi::c_ulong;

    pub fn zynqmp_clk_register_pll(
        name: *const ::core::ffi::c_char,
        clk_id: u32,
        parents: *const *const ::core::ffi::c_char,
        num_parents: u8,
        nodes: *const clock_topology,
    ) -> *mut clk_hw;

    pub fn zynqmp_clk_register_gate(
        name: *const ::core::ffi::c_char,
        clk_id: u32,
        parents: *const *const ::core::ffi::c_char,
        num_parents: u8,
        nodes: *const clock_topology,
    ) -> *mut clk_hw;

    pub fn zynqmp_clk_register_divider(
        name: *const ::core::ffi::c_char,
        clk_id: u32,
        parents: *const *const ::core::ffi::c_char,
        num_parents: u8,
        nodes: *const clock_topology,
    ) -> *mut clk_hw;

    pub fn zynqmp_clk_register_mux(
        name: *const ::core::ffi::c_char,
        clk_id: u32,
        parents: *const *const ::core::ffi::c_char,
        num_parents: u8,
        nodes: *const clock_topology,
    ) -> *mut clk_hw;

    pub fn zynqmp_clk_register_fixed_factor(
        name: *const ::core::ffi::c_char,
        clk_id: u32,
        parents: *const *const ::core::ffi::c_char,
        num_parents: u8,
        nodes: *const clock_topology,
    ) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
