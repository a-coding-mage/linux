/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel clock implementation:
// linux/clk-provider.h, ccu_common.h, ccu_div.h, and ccu_mult.h.

/*
 * struct ccu_nkm - Definition of an N-K-M clock
 *
 * Clocks based on the formula parent * N * K / M
 */
#[repr(C)]
pub struct ccu_nkm {
    pub enable: u32,
    pub lock: u32,

    pub n: ccu_mult_internal,
    pub k: ccu_mult_internal,
    pub m: ccu_div_internal,
    pub mux: ccu_mux_internal,

    pub fixed_post_div: ::core::ffi::c_uint,
    pub max_m_n_ratio: ::core::ffi::c_ulong,
    pub min_parent_m_ratio: ::core::ffi::c_ulong,

    pub common: ccu_common,
}

#[macro_export]
macro_rules! SUNXI_CCU_NKM_WITH_MUX_GATE_LOCK {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $nshift:expr, $nwidth:expr,
     $kshift:expr, $kwidth:expr,
     $mshift:expr, $mwidth:expr,
     $muxshift:expr, $muxwidth:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct: ccu_nkm = ccu_nkm {
            enable: $gate,
            lock: $lock,
            k: _SUNXI_CCU_MULT!($kshift, $kwidth),
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            common: ccu_common {
                reg: $reg,
                hw: hw {
                    init: CLK_HW_INIT_PARENTS!($name, $parents, &ccu_nkm_ops, $flags),
                },
            },
            ..::core::default::Default::default()
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_NKM_WITH_GATE_LOCK {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr,
     $nshift:expr, $nwidth:expr,
     $kshift:expr, $kwidth:expr,
     $mshift:expr, $mwidth:expr,
     $gate:expr, $lock:expr, $flags:expr) => {
        let $struct: ccu_nkm = ccu_nkm {
            enable: $gate,
            lock: $lock,
            k: _SUNXI_CCU_MULT!($kshift, $kwidth),
            n: _SUNXI_CCU_MULT!($nshift, $nwidth),
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            common: ccu_common {
                reg: $reg,
                hw: hw {
                    init: CLK_HW_INIT!($name, $parent, &ccu_nkm_ops, $flags),
                },
            },
            ..::core::default::Default::default()
        };
    };
}

#[inline]
pub unsafe fn hw_to_ccu_nkm(hw: *mut clk_hw) -> *mut ccu_nkm {
    let common: *mut ccu_common = hw_to_ccu_common(hw);
    // Equivalent to C's container_of(common, struct ccu_nkm, common).
    (common as *mut u8).sub(::core::mem::offset_of!(ccu_nkm, common)) as *mut ccu_nkm
}

extern "C" {
    pub static ccu_nkm_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
