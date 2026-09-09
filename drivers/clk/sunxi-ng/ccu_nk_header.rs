/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependencies supplied by the corresponding clock-provider headers:
// linux/clk-provider.h, ccu_common.h, ccu_div.h, and ccu_mult.h.

/*
 * struct ccu_nk - Definition of an N-K clock
 *
 * Clocks based on the formula parent * N * K
 */
#[repr(C)]
pub struct ccu_nk {
    pub reg: u16,
    pub enable: u32,
    pub lock: u32,

    pub n: ccu_mult_internal,
    pub k: ccu_mult_internal,

    pub fixed_post_div: ::core::ffi::c_uint,

    pub common: ccu_common,
}

#[macro_export]
macro_rules! SUNXI_CCU_NK_WITH_GATE_LOCK_POSTDIV {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr,
     $kshift:expr, $kwidth:expr,
     $gate:expr, $lock:expr, $postdiv:expr,
     $flags:expr) => {
        let $struct: ccu_nk = ccu_nk {
            enable: $gate,
            lock: $lock,
            k: _SUNXI_CCU_MULT!($kshift, $kwidth),
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            fixed_post_div: $postdiv,
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_FIXED_POSTDIV,
                hw: clk_hw_init!($name, $parent, &ccu_nk_ops, $flags),
                ..::core::default::Default::default()
            },
        };
    };
}

pub unsafe fn hw_to_ccu_nk(hw: *mut clk_hw) -> *mut ccu_nk {
    let common: *mut ccu_common = hw_to_ccu_common(hw);

    container_of!(common, ccu_nk, common)
}

unsafe extern "C" {
    pub static ccu_nk_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
