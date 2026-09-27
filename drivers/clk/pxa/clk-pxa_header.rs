/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Marvell PXA family clocks
 *
 * Copyright (C) 2014 Robert Jarzmik
 *
 * Common clock code for PXA clocks ("CKEN" type clocks + DT)
 */

pub const CLKCFG_TURBO: u32 = 0x1;
pub const CLKCFG_FCS: u32 = 0x2;
pub const CLKCFG_HALFTURBO: u32 = 0x4;
pub const CLKCFG_FASTBUS: u32 = 0x8;

/* C preprocessor declaration/registration helpers, retained as Rust macro forms. */
macro_rules! PARENTS {
    ($name:ident) => {
        static $name: &'static [&'static ::kernel::ffi::c_char] = &[];
    };
}

/*
 * CKEN clock type
 * This clock takes it source from 2 possible parents :
 *  - a low power parent
 *  - a normal parent
 *
 *  +------------+     +-----------+
 *  |  Low Power | --- | x mult_lp |
 *  |    Clock   |     | / div_lp  |\
 *  +------------+     +-----------+ \+-----+   +-----------+
 *                                    | Mux |---| CKEN gate |
 *  +------------+     +-----------+ /+-----+   +-----------+
 *  | High Power |     | x mult_hp |/
 *  |    Clock   | --- | / div_hp  |
 *  +------------+     +-----------+
 */
#[repr(C)]
pub struct desc_clk_cken {
    pub hw: clk_hw,
    pub ckid: ::kernel::ffi::c_int,
    pub cken_reg: ::kernel::ffi::c_int,
    pub name: *const ::kernel::ffi::c_char,
    pub dev_id: *const ::kernel::ffi::c_char,
    pub con_id: *const ::kernel::ffi::c_char,
    pub parent_names: *const *const ::kernel::ffi::c_char,
    pub lp: clk_fixed_factor,
    pub hp: clk_fixed_factor,
    pub gate: clk_gate,
    pub is_in_low_power: Option<unsafe extern "C" fn() -> bool>,
    pub flags: ::kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct pxa2xx_freq {
    pub cpll: ::kernel::ffi::c_ulong,
    pub membus_khz: ::kernel::ffi::c_uint,
    pub cccr: ::kernel::ffi::c_uint,
    pub div2: ::kernel::ffi::c_uint,
    pub clkcfg: ::kernel::ffi::c_uint,
}

#[inline]
pub unsafe fn dummy_clk_set_parent(_hw: *mut clk_hw, _index: u8) -> ::kernel::ffi::c_int {
    0
}

extern "C" {
    pub fn clkdev_pxa_register(
        ckid: ::kernel::ffi::c_int,
        con_id: *const ::kernel::ffi::c_char,
        dev_id: *const ::kernel::ffi::c_char,
        clk: *mut clk,
    );
    pub fn clk_pxa_cken_init(
        clks: *const desc_clk_cken,
        nb_clks: ::kernel::ffi::c_int,
        clk_regs: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn clk_pxa_dt_common_init(np: *mut device_node);
    pub fn pxa2xx_core_turbo_switch(on: bool);
    pub fn pxa2xx_cpll_change(
        freq: *mut pxa2xx_freq,
        mdrefr_dri: Option<unsafe extern "C" fn(::kernel::ffi::c_uint) -> u32>,
        cccr: *mut ::kernel::ffi::c_void,
    );
    pub fn pxa2xx_determine_rate(
        req: *mut clk_rate_request,
        freqs: *mut pxa2xx_freq,
        nb_freqs: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
