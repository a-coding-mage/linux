/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// C dependencies: linux/clk-provider.h, ccu_common.h, ccu_div.h,
// ccu_frac.h, ccu_mult.h, and ccu_sdm.h.

/*
 * struct ccu_nm - Definition of an N-M clock
 *
 * Clocks based on the formula parent * N / M
 */
#[repr(C)]
pub struct ccu_nm {
    pub enable: u32,
    pub lock: u32,

    pub n: ccu_mult_internal,
    pub m: ccu_div_internal,
    pub frac: ccu_frac_internal,
    pub sdm: ccu_sdm_internal,

    pub fixed_post_div: ::core::ffi::c_uint,
    pub min_rate: ::core::ffi::c_uint,
    pub max_rate: ::core::ffi::c_uint,

    pub common: ccu_common,
}

macro_rules! SUNXI_CCU_NM_WITH_SDM_GATE_LOCK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr, $mshift:expr, $mwidth:expr,
     $sdm_table:expr, $sdm_en:expr, $sdm_reg:expr, $sdm_reg_en:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct = ccu_nm {
            enable: $gate,
            lock: $lock,
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            frac: Default::default(),
            sdm: _SUNXI_CCU_SDM!($sdm_table, $sdm_en, $sdm_reg, $sdm_reg_en),
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_SIGMA_DELTA_MOD,
                hw: CLK_HW_INIT!($name, $parent, &ccu_nm_ops, $flags),
                ..Default::default()
            },
            ..Default::default()
        };
    };
}

macro_rules! SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr, $mshift:expr, $mwidth:expr,
     $frac_en:expr, $frac_sel:expr, $frac_rate_0:expr, $frac_rate_1:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct = ccu_nm {
            enable: $gate,
            lock: $lock,
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            frac: _SUNXI_CCU_FRAC!($frac_en, $frac_sel, $frac_rate_0, $frac_rate_1),
            sdm: Default::default(),
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_FRACTIONAL,
                hw: CLK_HW_INIT!($name, $parent, &ccu_nm_ops, $flags),
                ..Default::default()
            },
            ..Default::default()
        };
    };
}

macro_rules! SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $min_rate:expr,
     $nshift:expr, $nwidth:expr, $mshift:expr, $mwidth:expr,
     $frac_en:expr, $frac_sel:expr, $frac_rate_0:expr, $frac_rate_1:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let mut $struct = ccu_nm {
            min_rate: $min_rate,
            ..SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!($struct, $name, $parent, $reg,
                $nshift, $nwidth, $mshift, $mwidth, $frac_en, $frac_sel,
                $frac_rate_0, $frac_rate_1, $gate, $lock, $flags)
        };
    };
}

macro_rules! SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX_FEAT {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $min_rate:expr, $max_rate:expr,
     $nshift:expr, $nwidth:expr, $mshift:expr, $mwidth:expr,
     $frac_en:expr, $frac_sel:expr, $frac_rate_0:expr, $frac_rate_1:expr,
     $gate:expr, $lock:expr, $flags:expr, $features:expr) => {
        let $struct = ccu_nm {
            enable: $gate, lock: $lock,
            n: _SUNXI_CCU_MULT!($nshift, $nwidth), m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            frac: _SUNXI_CCU_FRAC!($frac_en, $frac_sel, $frac_rate_0, $frac_rate_1),
            min_rate: $min_rate, max_rate: $max_rate, sdm: Default::default(),
            common: ccu_common { reg: $reg, features: $features,
                hw: CLK_HW_INIT!($name, $parent, &ccu_nm_ops, $flags), ..Default::default() },
            ..Default::default()
        };
    };
}

macro_rules! SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX {
    ($($args:tt)*) => { SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX_FEAT!($($args)*, CCU_FEATURE_FRACTIONAL) };
}

macro_rules! SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX_CLOSEST {
    ($($args:tt)*) => { SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK_MIN_MAX_FEAT!($($args)*, CCU_FEATURE_FRACTIONAL | CCU_FEATURE_CLOSEST_RATE) };
}

macro_rules! SUNXI_CCU_NM_WITH_GATE_LOCK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr, $mshift:expr, $mwidth:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct = ccu_nm {
            enable: $gate, lock: $lock,
            n: _SUNXI_CCU_MULT!($nshift, $nwidth), m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            frac: Default::default(), sdm: Default::default(),
            common: ccu_common { reg: $reg,
                hw: CLK_HW_INIT!($name, $parent, &ccu_nm_ops, $flags), ..Default::default() },
            ..Default::default()
        };
    };
}

pub unsafe fn hw_to_ccu_nm(hw: *mut clk_hw) -> *mut ccu_nm {
    let common: *mut ccu_common = hw_to_ccu_common(hw);
    container_of!(common, ccu_nm, common)
}

extern "C" {
    pub static ccu_nm_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
