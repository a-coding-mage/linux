// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum divider clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Dependency supplied by the Linux clock-provider headers and div.h.

use core::ffi::{c_int, c_uint, c_ulong};

extern "C" {
    fn divider_determine_rate(
        hw: *mut crate::clk_hw,
        req: *mut crate::clk_rate_request,
        table: *const c_ulong,
        width: c_uint,
        flags: c_uint,
    ) -> c_int;
    fn divider_recalc_rate(
        hw: *mut crate::clk_hw,
        parent_rate: c_ulong,
        val: c_ulong,
        table: *const c_ulong,
        flags: c_uint,
        width: c_uint,
    ) -> c_ulong;
    fn divider_get_val(
        rate: c_ulong,
        parent_rate: c_ulong,
        table: *const c_ulong,
        width: c_uint,
        flags: c_uint,
    ) -> c_ulong;
    fn regmap_read(regmap: *mut core::ffi::c_void, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(regmap: *mut core::ffi::c_void, reg: c_uint, val: c_uint) -> c_int;
    fn hw_to_sprd_div(hw: *mut crate::clk_hw) -> *mut crate::sprd_div;
}

#[inline]
unsafe fn sprd_div_determine_rate(
    hw: *mut crate::clk_hw,
    req: *mut crate::clk_rate_request,
) -> c_int {
    let cd = &mut *hw_to_sprd_div(hw);
    divider_determine_rate(
        &mut cd.common.hw,
        req,
        core::ptr::null(),
        cd.div.width,
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn sprd_div_helper_recalc_rate(
    common: *mut crate::sprd_clk_common,
    div: *const crate::sprd_div_internal,
    parent_rate: c_ulong,
) -> c_ulong {
    let common_ref = &*common;
    let div_ref = &*div;
    let mut reg: c_uint = 0;

    regmap_read(
        common_ref.regmap,
        common_ref.reg.wrapping_add(div_ref.offset),
        &mut reg,
    );
    let mut val = (reg >> div_ref.shift) as c_ulong;
    val &= (1u64.wrapping_shl(div_ref.width) - 1) as c_ulong;

    divider_recalc_rate(
        &common_ref.hw as *const _ as *mut _,
        parent_rate,
        val,
        core::ptr::null(),
        0,
        div_ref.width,
    )
}

#[inline]
unsafe fn sprd_div_recalc_rate(hw: *mut crate::clk_hw, parent_rate: c_ulong) -> c_ulong {
    let cd = &mut *hw_to_sprd_div(hw);
    sprd_div_helper_recalc_rate(&mut cd.common, &cd.div, parent_rate)
}

#[no_mangle]
pub unsafe extern "C" fn sprd_div_helper_set_rate(
    common: *const crate::sprd_clk_common,
    div: *const crate::sprd_div_internal,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let common_ref = &*common;
    let div_ref = &*div;
    let val = divider_get_val(rate, parent_rate, core::ptr::null(), div_ref.width, 0);
    let mut reg: c_uint = 0;

    regmap_read(
        common_ref.regmap,
        common_ref.reg.wrapping_add(div_ref.offset),
        &mut reg,
    );
    let mask = (1u64.wrapping_shl(div_ref.width + div_ref.shift) - 1)
        & !(1u64.wrapping_shl(div_ref.shift) - 1);
    reg &= !(mask as c_uint);

    regmap_write(
        common_ref.regmap,
        common_ref.reg.wrapping_add(div_ref.offset),
        reg | ((val << div_ref.shift) as c_uint),
    );

    0
}

#[inline]
unsafe fn sprd_div_set_rate(
    hw: *mut crate::clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let cd = &mut *hw_to_sprd_div(hw);
    sprd_div_helper_set_rate(&cd.common, &cd.div, rate, parent_rate)
}

#[no_mangle]
pub static sprd_div_ops: crate::clk_ops = crate::clk_ops {
    recalc_rate: Some(sprd_div_recalc_rate),
    determine_rate: Some(sprd_div_determine_rate),
    set_rate: Some(sprd_div_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
