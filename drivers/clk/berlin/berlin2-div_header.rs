/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 */

// Forward declaration supplied by the clock framework.
#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

pub const BERLIN2_DIV_HAS_GATE: u32 = 1u32 << 0;
pub const BERLIN2_DIV_HAS_MUX: u32 = 1u32 << 1;

macro_rules! BERLIN2_PLL_SELECT {
    ($off:expr, $sh:expr) => {
        pll_select_offs: $off,
        pll_select_shift: $sh,
    };
}

macro_rules! BERLIN2_PLL_SWITCH {
    ($off:expr, $sh:expr) => {
        pll_switch_offs: $off,
        pll_switch_shift: $sh,
    };
}

macro_rules! BERLIN2_DIV_SELECT {
    ($off:expr, $sh:expr) => {
        div_select_offs: $off,
        div_select_shift: $sh,
    };
}

macro_rules! BERLIN2_DIV_SWITCH {
    ($off:expr, $sh:expr) => {
        div_switch_offs: $off,
        div_switch_shift: $sh,
    };
}

macro_rules! BERLIN2_DIV_D3SWITCH {
    ($off:expr, $sh:expr) => {
        div3_switch_offs: $off,
        div3_switch_shift: $sh,
    };
}

macro_rules! BERLIN2_DIV_GATE {
    ($off:expr, $sh:expr) => {
        gate_offs: $off,
        gate_shift: $sh,
    };
}

macro_rules! BERLIN2_SINGLE_DIV {
    ($off:expr) => {
        BERLIN2_DIV_GATE!($off, 0),
        BERLIN2_PLL_SELECT!($off, 1),
        BERLIN2_PLL_SWITCH!($off, 4),
        BERLIN2_DIV_SWITCH!($off, 5),
        BERLIN2_DIV_D3SWITCH!($off, 6),
        BERLIN2_DIV_SELECT!($off, 7)
    };
}

#[repr(C)]
pub struct berlin2_div_map {
    pub pll_select_offs: u16,
    pub pll_switch_offs: u16,
    pub div_select_offs: u16,
    pub div_switch_offs: u16,
    pub div3_switch_offs: u16,
    pub gate_offs: u16,
    pub pll_select_shift: u8,
    pub pll_switch_shift: u8,
    pub div_select_shift: u8,
    pub div_switch_shift: u8,
    pub div3_switch_shift: u8,
    pub gate_shift: u8,
}

#[repr(C)]
pub struct berlin2_div_data {
    pub name: *const core::ffi::c_char,
    pub parent_ids: *const u8,
    pub num_parents: core::ffi::c_int,
    pub flags: core::ffi::c_ulong,
    pub map: berlin2_div_map,
    pub div_flags: u8,
}

extern "C" {
    pub fn berlin2_div_register(
        map: *const berlin2_div_map,
        base: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        div_flags: u8,
        parent_names: *const *const core::ffi::c_char,
        num_parents: core::ffi::c_int,
        flags: core::ffi::c_ulong,
        lock: *mut spinlock_t,
    ) -> *mut clk_hw;
}

// Supplied by the kernel synchronization primitives.
pub type spinlock_t = core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
