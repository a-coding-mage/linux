/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependencies supplied by the original Linux headers and CCU headers:
// linux/bitops.h, linux/clk-provider.h, ccu_common.h, ccu_div.h,
// ccu_mult.h, ccu_mux.h

/*
 * struct ccu_mp - Definition of an M-P clock
 *
 * Clocks based on the formula parent >> P / M
 */
#[repr(C)]
struct ccu_mp {
    enable: u32,

    m: ccu_div_internal,
    p: ccu_div_internal,
    mux: ccu_mux_internal,

    fixed_post_div: ::core::ffi::c_uint,

    common: ccu_common,
}

macro_rules! SUNXI_CCU_MP_WITH_MUX_GATE_POSTDIV {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $postdiv:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            fixed_post_div: $postdiv,
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_FIXED_POSTDIV,
                hw: clk_hw_init_parents!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

macro_rules! SUNXI_CCU_MP_DATA_WITH_MUX_GATE_POSTDIV {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $postdiv:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            fixed_post_div: $postdiv,
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_FIXED_POSTDIV,
                hw: clk_hw_init_parents_data!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

macro_rules! SUNXI_CCU_MP_WITH_MUX_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            common: ccu_common {
                reg: $reg,
                hw: clk_hw_init_parents!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

macro_rules! SUNXI_CCU_MP_WITH_MUX {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $flags:expr) => {
        SUNXI_CCU_MP_WITH_MUX_GATE!($struct, $name, $parents, $reg,
            $mshift, $mwidth, $pshift, $pwidth, $muxshift, $muxwidth, 0, $flags);
    };
}

macro_rules! SUNXI_CCU_MP_MUX_GATE_POSTDIV_DUALDIV {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $postdiv:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            fixed_post_div: $postdiv,
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_FIXED_POSTDIV | CCU_FEATURE_DUAL_DIV,
                hw: clk_hw_init_parents_data!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

macro_rules! SUNXI_CCU_MP_DATA_WITH_MUX_GATE_FEAT {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $flags:expr, $features:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            common: ccu_common {
                reg: $reg,
                features: $features,
                hw: clk_hw_init_parents_data!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

macro_rules! SUNXI_CCU_MP_DATA_WITH_MUX_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $flags:expr) => {
        SUNXI_CCU_MP_DATA_WITH_MUX_GATE_FEAT!($struct, $name, $parents, $reg,
            $mshift, $mwidth, $pshift, $pwidth, $muxshift, $muxwidth,
            $gate, $flags, 0);
    };
}

macro_rules! SUNXI_CCU_DUALDIV_MUX_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $flags:expr) => {
        SUNXI_CCU_MP_DATA_WITH_MUX_GATE_FEAT!($struct, $name, $parents, $reg,
            $mshift, $mwidth, $pshift, $pwidth, $muxshift, $muxwidth,
            $gate, $flags, CCU_FEATURE_DUAL_DIV);
    };
}

macro_rules! SUNXI_CCU_MP_DATA_WITH_MUX {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $flags:expr) => {
        SUNXI_CCU_MP_DATA_WITH_MUX_GATE!($struct, $name, $parents, $reg,
            $mshift, $mwidth, $pshift, $pwidth, $muxshift, $muxwidth, 0, $flags);
    };
}

macro_rules! SUNXI_CCU_MP_HW_WITH_MUX_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $pshift:expr, $pwidth:expr,
     $muxshift:expr, $muxwidth:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: $gate,
            m: _SUNXI_CCU_DIV!($mshift, $mwidth),
            p: _SUNXI_CCU_DIV!($pshift, $pwidth),
            mux: _SUNXI_CCU_MUX!($muxshift, $muxwidth),
            common: ccu_common {
                reg: $reg,
                hw: clk_hw_init_parents_hw!($name, $parents, &ccu_mp_ops, $flags),
            },
        };
    };
}

unsafe fn hw_to_ccu_mp(hw: *mut clk_hw) -> *mut ccu_mp {
    let common: *mut ccu_common = hw_to_ccu_common(hw);
    container_of!(common, ccu_mp, common)
}

extern "C" {
    static ccu_mp_ops: clk_ops;
}

/*
 * Special class of M-P clock that supports MMC timing modes
 *
 * Since the MMC clock registers all follow the same layout, we can
 * simplify the macro for this particular case. In addition, as
 * switching modes also affects the output clock rate, we need to
 * have CLK_GET_RATE_NOCACHE for all these types of clocks.
 */

macro_rules! SUNXI_CCU_MP_MMC_WITH_MUX_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $flags:expr) => {
        let $struct = ccu_mp {
            enable: 1u32 << 31,
            m: _SUNXI_CCU_DIV!(0, 4),
            p: _SUNXI_CCU_DIV!(16, 2),
            mux: _SUNXI_CCU_MUX!(24, 2),
            common: ccu_common {
                reg: $reg,
                features: CCU_FEATURE_MMC_TIMING_SWITCH,
                hw: clk_hw_init_parents!($name, $parents, &ccu_mp_mmc_ops,
                    CLK_GET_RATE_NOCACHE | $flags),
            },
        };
    };
}

extern "C" {
    static ccu_mp_mmc_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
