/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Clock driver for TI Davinci PSC controllers
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

use core::ffi::{c_char, c_void};

/* Linux bitops equivalent: BIT(n) == 1U << n. */
pub const PLL_HAS_CLKMODE: u32 = 1u32 << 0; /* PLL has PLLCTL[CLKMODE] */
pub const PLL_HAS_PREDIV: u32 = 1u32 << 1; /* has prediv before PLL */
pub const PLL_PREDIV_ALWAYS_ENABLED: u32 = 1u32 << 2; /* don't clear DEN bit */
pub const PLL_PREDIV_FIXED_DIV: u32 = 1u32 << 3; /* fixed divider value */
pub const PLL_HAS_POSTDIV: u32 = 1u32 << 4; /* has postdiv after PLL */
pub const PLL_POSTDIV_ALWAYS_ENABLED: u32 = 1u32 << 5; /* don't clear DEN bit */
pub const PLL_POSTDIV_FIXED_DIV: u32 = 1u32 << 6; /* fixed divider value */
pub const PLL_HAS_EXTCLKSRC: u32 = 1u32 << 7; /* has selectable bypass */
pub const PLL_PLLM_2X: u32 = 1u32 << 8; /* PLLM value is 2x (DM365) */
pub const PLL_PREDIV_FIXED8: u32 = 1u32 << 9; /* DM355 quirk */

#[repr(C)]
pub struct davinci_pll_clk_info {
    pub name: *const c_char,
    pub unlock_reg: u32,
    pub unlock_mask: u32,
    pub pllm_mask: u32,
    pub pllm_min: u32,
    pub pllm_max: u32,
    pub pllout_min_rate: usize,
    pub pllout_max_rate: usize,
    pub flags: u32,
}

pub const SYSCLK_ARM_RATE: u32 = 1u32 << 0; /* Controls ARM rate */
pub const SYSCLK_ALWAYS_ENABLED: u32 = 1u32 << 1; /* Or bad things happen */
pub const SYSCLK_FIXED_DIV: u32 = 1u32 << 2; /* Fixed divider */

#[repr(C)]
pub struct davinci_pll_sysclk_info {
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub id: u32,
    pub ratio_width: u32,
    pub flags: u32,
}

#[macro_export]
macro_rules! SYSCLK {
    ($i:expr, $n:ident, $p:ident, $w:expr, $f:expr) => {
        static $n: $crate::davinci_pll_sysclk_info = $crate::davinci_pll_sysclk_info {
            name: concat!(stringify!($n), "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!(stringify!($p), "\0").as_ptr() as *const core::ffi::c_char,
            id: $i,
            ratio_width: $w,
            flags: $f,
        };
    };
}

#[repr(C)]
pub struct davinci_pll_obsclk_info {
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub table: *mut u32,
    pub ocsrc_mask: u32,
}

/* Opaque types supplied by the kernel and other dependencies. */
pub enum device {}
pub enum device_node {}
pub enum regmap {}
pub enum clk {}

extern "C" {
    pub fn davinci_pll_clk_register(
        dev: *mut device,
        info: *const davinci_pll_clk_info,
        parent_name: *const c_char,
        base: *mut c_void,
        cfgchip: *mut regmap,
    ) -> *mut clk;
    pub fn davinci_pll_auxclk_register(
        dev: *mut device,
        name: *const c_char,
        base: *mut c_void,
    ) -> *mut clk;
    pub fn davinci_pll_sysclkbp_clk_register(
        dev: *mut device,
        name: *const c_char,
        base: *mut c_void,
    ) -> *mut clk;
    pub fn davinci_pll_obsclk_register(
        dev: *mut device,
        info: *const davinci_pll_obsclk_info,
        base: *mut c_void,
    ) -> *mut clk;
    pub fn davinci_pll_sysclk_register(
        dev: *mut device,
        info: *const davinci_pll_sysclk_info,
        base: *mut c_void,
    ) -> *mut clk;
    pub fn of_davinci_pll_init(
        dev: *mut device,
        node: *mut device_node,
        info: *const davinci_pll_clk_info,
        obsclk_info: *const davinci_pll_obsclk_info,
        div_info: *const *const davinci_pll_sysclk_info,
        max_sysclk_id: u8,
        base: *mut c_void,
        cfgchip: *mut regmap,
    ) -> i32;

    /* Platform-specific callbacks */
    pub fn da850_pll1_init(dev: *mut device, base: *mut c_void, cfgchip: *mut regmap) -> i32;
    pub fn of_da850_pll0_init(node: *mut device_node);
    pub fn of_da850_pll1_init(dev: *mut device, base: *mut c_void, cfgchip: *mut regmap) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
