/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// C dependencies retained as external Rust types:
// linux/clk-provider.h, linux/regmap.h, and linux/spinlock.h.

#[repr(C)]
pub struct visconti_pll_provider {
    pub reg_base: *mut core::ffi::c_void,
    pub node: *mut device_node,

    /* Must be last */
    pub clk_data: clk_hw_onecell_data,
}

#[macro_export]
macro_rules! VISCONTI_PLL_RATE {
    ($rate:expr, $dacen:expr, $dsmen:expr, $refdiv:expr, $intin:expr,
     $fracin:expr, $postdiv1:expr, $postdiv2:expr) => {
        visconti_pll_rate_table {
            rate: $rate,
            dacen: $dacen,
            dsmen: $dsmen,
            refdiv: $refdiv,
            intin: $intin,
            fracin: $fracin,
            postdiv1: $postdiv1,
            postdiv2: $postdiv2,
        }
    };
}

#[repr(C)]
pub struct visconti_pll_rate_table {
    pub rate: c_ulong,
    pub dacen: c_uint,
    pub dsmen: c_uint,
    pub refdiv: c_uint,
    pub intin: c_ulong,
    pub fracin: c_ulong,
    pub postdiv1: c_uint,
    pub postdiv2: c_uint,
}

#[repr(C)]
pub struct visconti_pll_info {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent: *const c_char,
    pub base_reg: c_ulong,
    pub rate_table: *const visconti_pll_rate_table,
}

/* External Linux kernel types supplied by the surrounding translation. */
pub use core::ffi::{c_char, c_uint, c_ulong};
pub use device_node;
pub use clk_hw_onecell_data;
pub use spinlock_t;

extern "C" {
    pub fn visconti_init_pll(
        np: *mut device_node,
        base: *mut core::ffi::c_void,
        nr_plls: c_ulong,
    ) -> *mut visconti_pll_provider;

    pub fn visconti_register_plls(
        ctx: *mut visconti_pll_provider,
        list: *const visconti_pll_info,
        nr_plls: c_uint,
        lock: *mut spinlock_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
