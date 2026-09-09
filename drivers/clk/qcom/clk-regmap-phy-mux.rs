// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022, Linaro Ltd.
 */

// Dependencies supplied by the kernel clock, regmap, and clk-regmap headers.

use core::ffi::{c_int, c_uint, c_void};

const PHY_MUX_MASK: c_uint = 0x3;
const PHY_MUX_PHY_SRC: c_uint = 0;
const PHY_MUX_REF_SRC: c_uint = 2;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct clk_regmap_phy_mux {
    pub clkr: clk_regmap,
    pub reg: c_uint,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
}

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn warn_on(condition: bool) -> bool;
}

#[inline]
unsafe fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap {
    hw as *mut clk_regmap
}

#[inline]
unsafe fn to_clk_regmap_phy_mux(clkr: *mut clk_regmap) -> *mut clk_regmap_phy_mux {
    clkr as *mut clk_regmap_phy_mux
}

unsafe extern "C" fn phy_mux_is_enabled(hw: *mut clk_hw) -> c_int {
    let clkr = to_clk_regmap(hw);
    let phy_mux = to_clk_regmap_phy_mux(clkr);
    let mut val: c_uint = 0;

    regmap_read((*clkr).regmap, (*phy_mux).reg, &mut val);
    val = (val & PHY_MUX_MASK) >> 0;

    warn_on(val != PHY_MUX_PHY_SRC && val != PHY_MUX_REF_SRC);

    (val == PHY_MUX_PHY_SRC) as c_int
}

unsafe extern "C" fn phy_mux_enable(hw: *mut clk_hw) -> c_int {
    let clkr = to_clk_regmap(hw);
    let phy_mux = to_clk_regmap_phy_mux(clkr);

    regmap_update_bits(
        (*clkr).regmap,
        (*phy_mux).reg,
        PHY_MUX_MASK,
        (PHY_MUX_PHY_SRC << 0) & PHY_MUX_MASK,
    )
}

unsafe extern "C" fn phy_mux_disable(hw: *mut clk_hw) {
    let clkr = to_clk_regmap(hw);
    let phy_mux = to_clk_regmap_phy_mux(clkr);

    regmap_update_bits(
        (*clkr).regmap,
        (*phy_mux).reg,
        PHY_MUX_MASK,
        (PHY_MUX_REF_SRC << 0) & PHY_MUX_MASK,
    );
}

#[no_mangle]
pub static clk_regmap_phy_mux_ops: clk_ops = clk_ops {
    enable: Some(phy_mux_enable),
    disable: Some(phy_mux_disable),
    is_enabled: Some(phy_mux_is_enabled),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
