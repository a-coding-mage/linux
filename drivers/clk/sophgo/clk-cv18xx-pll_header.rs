/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// Dependency supplied by the surrounding clock implementation:
// #include "clk-cv18xx-common.h"

#[repr(C)]
pub struct cv1800_clk_pll_limit {
    pub pre_div: cv1800_clk_pll_limit_range,
    pub div: cv1800_clk_pll_limit_range,
    pub post_div: cv1800_clk_pll_limit_range,
    pub ictrl: cv1800_clk_pll_limit_range,
    pub mode: cv1800_clk_pll_limit_range,
}

#[repr(C)]
pub struct cv1800_clk_pll_limit_range {
    pub min: u8,
    pub max: u8,
}

#[macro_export]
macro_rules! _CV1800_PLL_LIMIT {
    ($min:expr, $max:expr) => {
        cv1800_clk_pll_limit_range { min: $min, max: $max }
    };
}

#[macro_export]
macro_rules! for_each_pll_limit_range {
    ($var:ident, $restrict:expr, $body:block) => {
        for $var in $restrict.min..=$restrict.max $body
    };
}

#[repr(C)]
pub struct cv1800_clk_pll_synthesizer {
    pub en: cv1800_clk_regbit,
    pub clk_half: cv1800_clk_regbit,
    pub ctrl: u32,
    pub set: u32,
}

pub const _PLL_PRE_DIV_SEL_FIELD: u32 = 0x0000007f;
pub const _PLL_POST_DIV_SEL_FIELD: u32 = 0x00007f00;
pub const _PLL_SEL_MODE_FIELD: u32 = 0x00018000;
pub const _PLL_DIV_SEL_FIELD: u32 = 0x00fe0000;
pub const _PLL_ICTRL_FIELD: u32 = 0x07000000;
pub const _PLL_ALL_FIELD_MASK: u32 = _PLL_PRE_DIV_SEL_FIELD
    | _PLL_POST_DIV_SEL_FIELD
    | _PLL_SEL_MODE_FIELD
    | _PLL_DIV_SEL_FIELD
    | _PLL_ICTRL_FIELD;

#[macro_export]
macro_rules! PLL_COPY_REG {
    ($dest:expr, $src:expr) => {
        (($dest & !_PLL_ALL_FIELD_MASK) | ($src & _PLL_ALL_FIELD_MASK))
    };
}

#[macro_export]
macro_rules! PLL_GET_PRE_DIV_SEL { ($reg:expr) => { (($reg & _PLL_PRE_DIV_SEL_FIELD) >> 0) }; }
#[macro_export]
macro_rules! PLL_GET_POST_DIV_SEL { ($reg:expr) => { (($reg & _PLL_POST_DIV_SEL_FIELD) >> 8) }; }
#[macro_export]
macro_rules! PLL_GET_SEL_MODE { ($reg:expr) => { (($reg & _PLL_SEL_MODE_FIELD) >> 15) }; }
#[macro_export]
macro_rules! PLL_GET_DIV_SEL { ($reg:expr) => { (($reg & _PLL_DIV_SEL_FIELD) >> 17) }; }
#[macro_export]
macro_rules! PLL_GET_ICTRL { ($reg:expr) => { (($reg & _PLL_ICTRL_FIELD) >> 24) }; }

#[macro_export]
macro_rules! PLL_SET_PRE_DIV_SEL { ($reg:expr, $val:expr) => { _CV1800_SET_FIELD!($reg, $val, _PLL_PRE_DIV_SEL_FIELD) }; }
#[macro_export]
macro_rules! PLL_SET_POST_DIV_SEL { ($reg:expr, $val:expr) => { _CV1800_SET_FIELD!($reg, $val, _PLL_POST_DIV_SEL_FIELD) }; }
#[macro_export]
macro_rules! PLL_SET_SEL_MODE { ($reg:expr, $val:expr) => { _CV1800_SET_FIELD!($reg, $val, _PLL_SEL_MODE_FIELD) }; }
#[macro_export]
macro_rules! PLL_SET_DIV_SEL { ($reg:expr, $val:expr) => { _CV1800_SET_FIELD!($reg, $val, _PLL_DIV_SEL_FIELD) }; }
#[macro_export]
macro_rules! PLL_SET_ICTRL { ($reg:expr, $val:expr) => { _CV1800_SET_FIELD!($reg, $val, _PLL_ICTRL_FIELD) }; }

#[repr(C)]
pub struct cv1800_clk_pll {
    pub common: cv1800_clk_common,
    pub pll_reg: u32,
    pub pll_pwd: cv1800_clk_regbit,
    pub pll_status: cv1800_clk_regbit,
    pub pll_limit: *const cv1800_clk_pll_limit,
    pub pll_syn: *mut cv1800_clk_pll_synthesizer,
}

// C initializers CV1800_INTEGRAL_PLL and CV1800_FACTIONAL_PLL are preserved as
// Rust macros; CV1800_CLK_COMMON and CV1800_CLK_BIT are supplied externally.
#[macro_export]
macro_rules! CV1800_INTEGRAL_PLL {
    ($name:ident, $parent:expr, $pll_reg:expr, $pll_pwd_reg:expr, $pll_pwd_shift:expr,
     $pll_status_reg:expr, $pll_status_shift:expr, $pll_limit:expr, $flags:expr) => {
        let $name = cv1800_clk_pll {
            common: CV1800_CLK_COMMON!(stringify!($name), $parent, &cv1800_clk_ipll_ops, $flags),
            pll_reg: $pll_reg,
            pll_pwd: CV1800_CLK_BIT!($pll_pwd_reg, $pll_pwd_shift),
            pll_status: CV1800_CLK_BIT!($pll_status_reg, $pll_status_shift),
            pll_limit: $pll_limit,
            pll_syn: core::ptr::null_mut(),
        };
    };
}

#[macro_export]
macro_rules! CV1800_FACTIONAL_PLL {
    ($name:ident, $parent:expr, $pll_reg:expr, $pll_pwd_reg:expr, $pll_pwd_shift:expr,
     $pll_status_reg:expr, $pll_status_shift:expr, $pll_limit:expr, $pll_syn:expr, $flags:expr) => {
        let $name = cv1800_clk_pll {
            common: CV1800_CLK_COMMON!(stringify!($name), $parent, &cv1800_clk_fpll_ops, $flags),
            pll_reg: $pll_reg,
            pll_pwd: CV1800_CLK_BIT!($pll_pwd_reg, $pll_pwd_shift),
            pll_status: CV1800_CLK_BIT!($pll_status_reg, $pll_status_shift),
            pll_limit: $pll_limit,
            pll_syn: $pll_syn,
        };
    };
}

extern "C" {
    pub static cv1800_clk_ipll_ops: clk_ops;
    pub static cv1800_clk_fpll_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
