/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/device.h, linux/clk-provider.h, and linux/regmap.h

use core::ffi::c_void;

/**
 * struct clk_regmap - regmap backed clock
 *
 * @hw:     handle between common and hardware-specific interfaces
 * @map:    pointer to the regmap structure controlling the clock
 * @data:   data specific to the clock type
 *
 * Clock which is controlled by regmap backed registers. The actual type of
 * of the clock is controlled by the clock_ops and data.
 */
#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub data: *mut c_void,
}

pub unsafe fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap {
    // `hw` is the first member of clk_regmap, matching container_of().
    hw as *mut clk_regmap
}

/* clk_regmap init op to get and cache regmap from the controllers */
extern "C" {
    pub fn clk_regmap_init(hw: *mut clk_hw) -> core::ffi::c_int;
}

/**
 * struct clk_regmap_gate_data - regmap backed gate specific data
 *
 * @offset:  offset of the register controlling gate
 * @bit_idx: single bit controlling gate
 * @flags:   hardware-specific flags
 *
 * Flags:
 * Same as clk_gate except CLK_GATE_HIWORD_MASK which is ignored
 */
#[repr(C)]
pub struct clk_regmap_gate_data {
    pub offset: core::ffi::c_uint,
    pub bit_idx: u8,
    pub flags: u8,
}

pub unsafe fn clk_get_regmap_gate_data(
    clk: *mut clk_regmap,
) -> *mut clk_regmap_gate_data {
    (*clk).data as *mut clk_regmap_gate_data
}

extern "C" {
    pub static clk_regmap_gate_ops: clk_ops;
    pub static clk_regmap_gate_ro_ops: clk_ops;
}

/**
 * struct clk_regmap_div_data - regmap backed adjustable divider specific data
 *
 * @offset:  offset of the register controlling the divider
 * @shift:   shift to the divider bit field
 * @width:   width of the divider bit field
 * @table:   array of value/divider pairs, last entry should have div = 0
 *
 * Flags:
 * Same as clk_divider except CLK_DIVIDER_HIWORD_MASK which is ignored
 */
#[repr(C)]
pub struct clk_regmap_div_data {
    pub offset: core::ffi::c_uint,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub table: *const clk_div_table,
}

pub unsafe fn clk_get_regmap_div_data(
    clk: *mut clk_regmap,
) -> *mut clk_regmap_div_data {
    (*clk).data as *mut clk_regmap_div_data
}

extern "C" {
    pub static clk_regmap_divider_ops: clk_ops;
    pub static clk_regmap_divider_ro_ops: clk_ops;
}

/**
 * struct clk_regmap_mux_data - regmap backed multiplexer clock specific data
 *
 * @hw:      handle between common and hardware-specific interfaces
 * @offset:  offset of theregister controlling multiplexer
 * @table:   array of parent indexed register values
 * @shift:   shift to multiplexer bit field
 * @mask:    mask of mutliplexer bit field
 * @flags:   hardware-specific flags
 *
 * Flags:
 * Same as clk_divider except CLK_MUX_HIWORD_MASK which is ignored
 */
#[repr(C)]
pub struct clk_regmap_mux_data {
    pub offset: core::ffi::c_uint,
    pub table: *mut u32,
    pub mask: u32,
    pub shift: u8,
    pub flags: u8,
}

pub unsafe fn clk_get_regmap_mux_data(
    clk: *mut clk_regmap,
) -> *mut clk_regmap_mux_data {
    (*clk).data as *mut clk_regmap_mux_data
}

extern "C" {
    pub static clk_regmap_mux_ops: clk_ops;
    pub static clk_regmap_mux_ro_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
