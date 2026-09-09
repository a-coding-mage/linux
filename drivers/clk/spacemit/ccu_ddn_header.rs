/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 */

// Dependencies supplied by the Linux clock framework and ccu_common.h are
// intentionally left external to this translation.

#[repr(C)]
pub struct ccu_ddn {
    pub common: ccu_common,
    pub num_mask: ::core::ffi::c_uint,
    pub num_shift: ::core::ffi::c_uint,
    pub den_mask: ::core::ffi::c_uint,
    pub den_shift: ::core::ffi::c_uint,
    pub pre_div: ::core::ffi::c_uint,
}

// Direct translation of CLK_HW_INIT_HW; the macro and referenced framework
// symbols are provided by the translated dependencies.
macro_rules! CCU_DDN_INIT {
    ($name:ident, $parent:ident, $flags:expr) => {
        CLK_HW_INIT_HW!(stringify!($name), &$parent.common.hw, &spacemit_ccu_ddn_ops, $flags)
    };
}

macro_rules! CCU_DDN_DEFINE {
    ($name:ident, $parent:ident, $reg_ctrl:expr, $num_shift:expr, $num_width:expr,
     $den_shift:expr, $den_width:expr, $pre_div:expr, $flags:expr) => {
        static mut $name: ccu_ddn = ccu_ddn {
            common: ccu_common {
                reg_ctrl: $reg_ctrl,
                hw: clk_hw_init {
                    init: CCU_DDN_INIT!($name, $parent, $flags),
                },
            },
            num_mask: GENMASK!($num_shift + $num_width - 1, $num_shift),
            num_shift: $num_shift,
            den_mask: GENMASK!($den_shift + $den_width - 1, $den_shift),
            den_shift: $den_shift,
            pre_div: $pre_div,
        };
    };
}

#[inline]
pub unsafe fn hw_to_ccu_ddn(hw: *mut clk_hw) -> *mut ccu_ddn {
    let common: *mut ccu_common = hw_to_ccu_common(hw);
    container_of!(common, ccu_ddn, common)
}

extern "C" {
    pub static spacemit_ccu_ddn_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
