/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Toshiba Visconti clock controller
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/mfd/syscon.h, linux/clk-provider.h, linux/of.h,
// linux/of_address.h, linux/delay.h, linux/regmap.h, linux/slab.h,
// linux/string.h, linux/io.h, linux/spinlock.h, and "reset.h".

#[repr(C)]
pub struct visconti_clk_provider {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
pub struct visconti_clk_gate_table {
    pub id: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: u8,
    pub ckon_offset: u32,
    pub ckoff_offset: u32,
    pub ck_idx: u8,
    pub div: ::core::ffi::c_uint,
    pub rs_id: u8,
}

#[repr(C)]
pub struct visconti_fixed_clk {
    pub id: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub parent: *const ::core::ffi::c_char,
    pub flag: ::core::ffi::c_ulong,
    pub mult: ::core::ffi::c_uint,
    pub div: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct visconti_clk_gate {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub ckon_offset: u32,
    pub ckoff_offset: u32,
    pub ck_idx: u8,
    pub flags: u8,
    pub rson_offset: u32,
    pub rsoff_offset: u32,
    pub rs_idx: u8,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub fn visconti_init_clk(
        dev: *mut device,
        regmap: *mut regmap,
        nr_clks: ::core::ffi::c_ulong,
    ) -> *mut visconti_clk_provider;

    pub fn visconti_clk_register_gates(
        data: *mut visconti_clk_provider,
        clks: *const visconti_clk_gate_table,
        num_gate: ::core::ffi::c_int,
        reset: *const visconti_reset_data,
        lock: *mut spinlock_t,
    ) -> ::core::ffi::c_int;
}

pub const NO_RESET: u32 = 0xFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
