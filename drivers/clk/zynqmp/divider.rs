// SPDX-License-Identifier: GPL-2.0
/*
 * Zynq UltraScale+ MPSoC Divider support
 *
 *  Copyright (C) 2016-2019 Xilinx
 *
 * Adjustable divider clock implementation
 */

// Dependency declarations supplied by the surrounding kernel translation.

const CLK_FRAC: u16 = 1 << 13; /* has a fractional parent */
const CUSTOM_FLAG_CLK_FRAC: u8 = 1 << 0; /* has a fractional parent in custom type flag */

#[repr(C)]
pub struct ZynqmpClkDivider {
    pub hw: ClkHw,
    pub flags: u8,
    pub is_frac: bool,
    pub clk_id: u32,
    pub div_type: u32,
    pub max_div: u16,
}

#[inline]
unsafe fn to_zynqmp_clk_divider(hw: *mut ClkHw) -> *mut ZynqmpClkDivider {
    container_of!(hw, ZynqmpClkDivider, hw)
}

#[inline]
unsafe fn zynqmp_divider_get_val(parent_rate: c_ulong, rate: c_ulong, flags: u16) -> c_int {
    let (up, down): (c_ulong, c_ulong);

    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 {
        up = div_round_up_ull(parent_rate as u64, rate as u64);
        down = div_round_down_ull(parent_rate as u64, rate as u64);

        up = roundup_pow_of_two(up);
        down = rounddown_pow_of_two(down);

        let up_rate = div_round_up_ull(parent_rate as u64, up as u64);
        let down_rate = div_round_up_ull(parent_rate as u64, down as u64);

        return if rate.wrapping_sub(up_rate) <= down_rate.wrapping_sub(rate) {
            up as c_int
        } else {
            down as c_int
        };
    }

    div_round_closest(parent_rate, rate) as c_int
}

unsafe fn zynqmp_clk_divider_recalc_rate(hw: *mut ClkHw, parent_rate: c_ulong) -> c_ulong {
    let divider = to_zynqmp_clk_divider(hw);
    let clk_name = clk_hw_get_name(hw);
    let clk_id = (*divider).clk_id;
    let div_type = (*divider).div_type;
    let mut div: u32 = 0;
    let mut value: u32;

    let ret = zynqmp_pm_clock_getdivider(clk_id, &mut div);
    if ret != 0 {
        pr_debug("{}() get divider failed for {}, ret = {}\n", "zynqmp_clk_divider_recalc_rate", clk_name, ret);
    }

    value = if div_type == TYPE_DIV1 { div & 0xffff } else { div >> 16 };
    if (*divider).flags as u16 & CLK_DIVIDER_POWER_OF_TWO != 0 {
        value = 1 << value;
    }
    if value == 0 {
        warn((*divider).flags as u16 & CLK_DIVIDER_ALLOW_ZERO == 0,
             "{}: Zero divisor and CLK_DIVIDER_ALLOW_ZERO not set\n", clk_name);
        return parent_rate;
    }
    div_round_up_ull(parent_rate as u64, value as u64) as c_ulong
}

unsafe fn zynqmp_clk_divider_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> c_int {
    let divider = to_zynqmp_clk_divider(hw);
    let clk_name = clk_hw_get_name(hw);
    let clk_id = (*divider).clk_id;
    let div_type = (*divider).div_type;
    let mut bestdiv: u32;

    if (*divider).flags as u16 & CLK_DIVIDER_READ_ONLY != 0 {
        let ret = zynqmp_pm_clock_getdivider(clk_id, &mut bestdiv);
        if ret != 0 {
            pr_debug("{}() get divider failed for {}, ret = {}\n", "zynqmp_clk_divider_determine_rate", clk_name, ret);
        }
        bestdiv = if div_type == TYPE_DIV1 { bestdiv & 0xffff } else { bestdiv >> 16 };
        if (*divider).flags as u16 & CLK_DIVIDER_POWER_OF_TWO != 0 { bestdiv = 1 << bestdiv; }
        (*req).rate = div_round_up_ull((*req).best_parent_rate as u64, bestdiv as u64) as c_ulong;
        return 0;
    }

    let width = fls((*divider).max_div as u32) as u8;
    let ret = divider_determine_rate(hw, req, core::ptr::null_mut(), width, (*divider).flags as u16);
    if ret != 0 { return ret; }
    if (*divider).is_frac && clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT != 0 && (*req).rate % (*req).best_parent_rate != 0 {
        (*req).best_parent_rate = (*req).rate;
    }
    0
}

unsafe fn zynqmp_clk_divider_set_rate(hw: *mut ClkHw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let divider = to_zynqmp_clk_divider(hw);
    let clk_name = clk_hw_get_name(hw);
    let value = zynqmp_divider_get_val(parent_rate, rate, (*divider).flags as u16) as u32;
    let mut div = if (*divider).div_type == TYPE_DIV1 { (value & 0xffff) | (0xffff << 16) } else { 0xffff | (value << 16) };
    if (*divider).flags as u16 & CLK_DIVIDER_POWER_OF_TWO != 0 { div = ffs(div) as u32; }
    let ret = zynqmp_pm_clock_setdivider((*divider).clk_id, div);
    if ret != 0 { pr_debug("{}() set divider failed for {}, ret = {}\n", "zynqmp_clk_divider_set_rate", clk_name, ret); }
    ret
}

static ZYNQMP_CLK_DIVIDER_OPS: ClkOps = ClkOps {
    recalc_rate: Some(zynqmp_clk_divider_recalc_rate),
    determine_rate: Some(zynqmp_clk_divider_determine_rate),
    set_rate: Some(zynqmp_clk_divider_set_rate),
};

static ZYNQMP_CLK_DIVIDER_RO_OPS: ClkOps = ClkOps {
    recalc_rate: Some(zynqmp_clk_divider_recalc_rate),
    determine_rate: Some(zynqmp_clk_divider_determine_rate),
    set_rate: None,
};

unsafe fn zynqmp_clk_get_max_divisor(clk_id: u32, kind: u32) -> u32 {
    let mut qdata = ZynqmpPmQueryData { qid: 0, arg1: 0, arg2: 0 };
    let mut ret_payload = [0u32; PAYLOAD_ARG_CNT];
    qdata.qid = PM_QID_CLOCK_GET_MAX_DIVISOR;
    qdata.arg1 = clk_id;
    qdata.arg2 = kind;
    let ret = zynqmp_pm_query_data(qdata, ret_payload.as_mut_ptr());
    if ret != 0 { return u16::MAX as u32; }
    ret_payload[1]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
