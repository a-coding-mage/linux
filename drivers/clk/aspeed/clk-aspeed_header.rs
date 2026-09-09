/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Structures used by ASPEED clock drivers
 *
 * Copyright 2019 IBM Corp.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/clk-provider.h, linux/kernel.h, linux/reset-controller.h,
// and linux/spinlock.h.

#[repr(C)]
pub struct clk_div_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_controller_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

/**
 * struct aspeed_gate_data - Aspeed gated clocks
 * @clock_idx: bit used to gate this clock in the clock register
 * @reset_idx: bit used to reset this IP in the reset register. -1 if no
 *             reset is required when enabling the clock
 * @name: the clock name
 * @parent_name: the name of the parent clock
 * @flags: standard clock framework flags
 */
#[repr(C)]
pub struct aspeed_gate_data {
    pub clock_idx: u8,
    pub reset_idx: i8,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub flags: core::ffi::c_ulong,
}

/**
 * struct aspeed_clk_gate - Aspeed specific clk_gate structure
 * @hw: handle between common and hardware-specific interfaces
 * @reg: register controlling gate
 * @clock_idx: bit used to gate this clock in the clock register
 * @reset_idx: bit used to reset this IP in the reset register. -1 if no
 *  reset is required when enabling the clock
 * @flags: hardware-specific flags
 * @lock: register lock
 *
 * Some of the clocks in the Aspeed SoC must be put in reset before enabling.
 * This modified version of clk_gate allows an optional reset bit to be
 * specified.
 */
#[repr(C)]
pub struct aspeed_clk_gate {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub clock_idx: u8,
    pub reset_idx: i8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

#[macro_export]
macro_rules! to_aspeed_clk_gate {
    ($hw:expr) => {
        (($hw as *mut u8).sub(core::mem::offset_of!($crate::aspeed_clk_gate, hw))
            as *mut $crate::aspeed_clk_gate)
    };
}

/**
 * struct aspeed_reset - Aspeed reset controller
 * @map: regmap to access the containing system controller
 * @rcdev: reset controller device
 */
#[repr(C)]
pub struct aspeed_reset {
    pub map: *mut regmap,
    pub rcdev: reset_controller_dev,
}

#[macro_export]
macro_rules! to_aspeed_reset {
    ($p:expr) => {
        (($p as *mut u8).sub(core::mem::offset_of!($crate::aspeed_reset, rcdev))
            as *mut $crate::aspeed_reset)
    };
}

/**
 * struct aspeed_clk_soc_data - Aspeed SoC specific divisor information
 * @div_table: Common divider lookup table
 * @eclk_div_table: Divider lookup table for ECLK
 * @mac_div_table: Divider lookup table for MAC (Ethernet) clocks
 * @calc_pll: Callback to maculate common PLL settings
 */
#[repr(C)]
pub struct aspeed_clk_soc_data {
    pub div_table: *const clk_div_table,
    pub eclk_div_table: *const clk_div_table,
    pub mac_div_table: *const clk_div_table,
    pub calc_pll: Option<unsafe extern "C" fn(
        name: *const core::ffi::c_char,
        val: u32,
    ) -> *mut clk_hw>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
