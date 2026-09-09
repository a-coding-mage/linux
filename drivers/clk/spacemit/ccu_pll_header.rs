/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 */

// Dependency intent: linux/clk-provider.h and ccu_common.h supply the
// referenced clock, common-CCU, and helper definitions.

/**
 * Structure mapping between PLL rate and register configuration.
 *
 * @rate:   PLL rate
 * @swcr1:  Value of register PLLx_SW1_CTRL.
 * @swcr2:  Value of register PLLAx_SW2_CTRL.
 * @swcr3:  Value of register PLLx_SW3_CTRL.
 *
 * See the source tables for the registers used in PPL/PPLA clocks.
 */
#[repr(C)]
pub struct ccu_pll_rate_tbl {
    pub rate: ::core::ffi::c_ulong,
    pub swcr1: u32,
    pub swcr2: u32,
    pub swcr3: u32,
}

#[repr(C)]
pub struct ccu_pll_config {
    pub rate_tbl: *const ccu_pll_rate_tbl,
    pub tbl_num: u32,
    pub reg_lock: u32,
    pub mask_lock: u32,
}

#[repr(C)]
pub struct ccu_pll {
    pub common: ccu_common,
    pub config: ccu_pll_config,
}

#[macro_export]
macro_rules! CCU_PLL_RATE {
    ($rate:expr, $swcr1:expr, $swcr3:expr) => {
        ccu_pll_rate_tbl {
            rate: $rate,
            swcr1: $swcr1,
            swcr2: 0,
            swcr3: $swcr3,
        }
    };
}

#[macro_export]
macro_rules! CCU_PLLA_RATE {
    ($rate:expr, $swcr1:expr, $swcr2:expr, $swcr3:expr) => {
        ccu_pll_rate_tbl {
            rate: $rate,
            swcr1: $swcr1,
            swcr2: $swcr2,
            swcr3: $swcr3,
        }
    };
}

#[macro_export]
macro_rules! CCU_PLL_CONFIG {
    ($table:expr, $reg_lock:expr, $mask_lock:expr) => {
        ccu_pll_config {
            rate_tbl: ($table).as_ptr(),
            tbl_num: ($table).len() as u32,
            reg_lock: $reg_lock,
            mask_lock: $mask_lock,
        }
    };
}

// C preprocessor build-time initialization macro; retained as a Rust macro.
#[macro_export]
macro_rules! CCU_PLL_COMMON_HWINIT {
    ($name:ident, $ops:expr, $flags:expr) => {
        &clk_init_data {
            name: stringify!($name),
            ops: $ops,
            parent_data: &clk_parent_data { index: 0 },
            num_parents: 1,
            flags: $flags,
        }
    };
}

// The following definition macros preserve the source construction pattern;
// their fields depend on the external ccu_common and clock-provider types.
#[macro_export]
macro_rules! CCU_PLL_X_DEFINE {
    ($name:ident, $table:expr, $reg_swcr1:expr, $reg_swcr2:expr,
     $reg_swcr3:expr, $reg_lock:expr, $mask_lock:expr, $ops:expr, $flags:expr) => {
        static mut $name: ccu_pll = ccu_pll {
            config: CCU_PLL_CONFIG!($table, $reg_lock, $mask_lock),
            common: ccu_common {
                reg_swcr1: $reg_swcr1,
                reg_swcr2: $reg_swcr2,
                reg_swcr3: $reg_swcr3,
                hw: clk_hw {
                    init: CCU_PLL_COMMON_HWINIT!($name, $ops, $flags),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! CCU_PLL_DEFINE {
    ($name:ident, $table:expr, $reg_swcr1:expr, $reg_swcr3:expr,
     $reg_lock:expr, $mask_lock:expr, $flags:expr) => {
        CCU_PLL_X_DEFINE!($name, $table, $reg_swcr1, 0, $reg_swcr3,
                          $reg_lock, $mask_lock, &spacemit_ccu_pll_ops, $flags);
    };
}

#[macro_export]
macro_rules! CCU_PLLA_DEFINE {
    ($name:ident, $table:expr, $reg_swcr1:expr, $reg_swcr2:expr,
     $reg_swcr3:expr, $reg_lock:expr, $mask_lock:expr, $flags:expr) => {
        CCU_PLL_X_DEFINE!($name, $table, $reg_swcr1, $reg_swcr2, $reg_swcr3,
                          $reg_lock, $mask_lock, &spacemit_ccu_plla_ops, $flags);
    };
}

#[inline]
pub unsafe fn hw_to_ccu_pll(hw: *mut clk_hw) -> *mut ccu_pll {
    let common = hw_to_ccu_common(hw);
    container_of!(common, ccu_pll, common)
}

unsafe extern "C" {
    pub static spacemit_ccu_pll_ops: clk_ops;
    pub static spacemit_ccu_plla_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
