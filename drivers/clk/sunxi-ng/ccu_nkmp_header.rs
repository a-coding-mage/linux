/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/clk-provider.h>, "ccu_common.h", "ccu_div.h", and "ccu_mult.h".

/*
 * struct ccu_nkmp - Definition of an N-K-M-P clock
 *
 * Clocks based on the formula parent * N * K >> P / M
 */
#[repr(C)]
pub struct ccu_nkmp {
    pub enable: u32,
    pub lock: u32,

    pub n: ccu_mult_internal,
    pub k: ccu_mult_internal,
    pub m: ccu_div_internal,
    pub p: ccu_div_internal,

    pub fixed_post_div: ::core::ffi::c_uint,
    pub max_rate: ::core::ffi::c_uint,

    pub common: ccu_common,
}

#[macro_export]
macro_rules! SUNXI_CCU_NKMP_WITH_GATE_LOCK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr,
     $kshift:expr, $kwidth:expr,
     $mshift:expr, $mwidth:expr,
     $pshift:expr, $pwidth:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct = ccu_nkmp {
            enable: $gate,
            lock: $lock,
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            k: _SUNXI_CCU_MULT!($kshift, $kwidth),
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            common: ccu_common {
                reg: $reg,
                hw: clk_hw_init!($name, $parent, &ccu_nkmp_ops, $flags),
            },
        };
    };
}

pub unsafe fn hw_to_ccu_nkmp(hw: *mut clk_hw) -> *mut ccu_nkmp {
    let common: *mut ccu_common = hw_to_ccu_common(hw);

    container_of!(common, ccu_nkmp, common)
}

extern "C" {
    pub static ccu_nkmp_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
