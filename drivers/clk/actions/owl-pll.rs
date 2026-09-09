// SPDX-License-Identifier: GPL-2.0+
//
// OWL pll clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependencies supplied by the surrounding kernel translation.

unsafe fn owl_pll_calculate_mul(pll_hw: *mut owl_pll_hw, rate: c_ulong) -> u32 {
    let mut mul: u32;

    mul = div_round_closest(rate, (*pll_hw).bfreq);
    if mul < (*pll_hw).min_mul {
        mul = (*pll_hw).min_mul;
    } else if mul > (*pll_hw).max_mul {
        mul = (*pll_hw).max_mul;
    }

    mul & mul_mask(pll_hw)
}

unsafe fn _get_table_rate(table: *const clk_pll_table, val: c_uint) -> c_ulong {
    let mut clkt = table;

    while (*clkt).rate != 0 {
        if (*clkt).val == val {
            return (*clkt).rate;
        }
        clkt = clkt.add(1);
    }

    0
}

unsafe fn _get_pll_table(
    mut table: *const clk_pll_table,
    rate: c_ulong,
) -> *const clk_pll_table {
    let mut clkt = table;

    while (*clkt).rate != 0 {
        if (*clkt).rate == rate {
            table = clkt;
            break;
        } else if (*clkt).rate < rate {
            table = clkt;
        }
        clkt = clkt.add(1);
    }

    table
}

unsafe fn owl_pll_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let pll = hw_to_owl_pll(hw);
    let pll_hw = &mut (*pll).pll_hw;
    let clkt: *const clk_pll_table;
    let mut mul: u32;

    if !pll_hw.table.is_null() {
        clkt = _get_pll_table(pll_hw.table, (*req).rate);
        (*req).rate = (*clkt).rate;

        return 0;
    }

    /* fixed frequency */
    if pll_hw.width == 0 {
        (*req).rate = pll_hw.bfreq;

        return 0;
    }

    mul = owl_pll_calculate_mul(pll_hw, (*req).rate);

    (*req).rate = pll_hw.bfreq * mul as c_ulong;

    0
}

unsafe fn owl_pll_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let pll = hw_to_owl_pll(hw);
    let pll_hw = &mut (*pll).pll_hw;
    let common = &(*pll).common;
    let mut val: u32 = 0;

    if !pll_hw.table.is_null() {
        regmap_read(common.regmap, pll_hw.reg, &mut val);

        val >>= pll_hw.shift;
        val &= mul_mask(pll_hw);

        return _get_table_rate(pll_hw.table, val);
    }

    /* fixed frequency */
    if pll_hw.width == 0 {
        return pll_hw.bfreq;
    }

    regmap_read(common.regmap, pll_hw.reg, &mut val);

    val >>= pll_hw.shift;
    val &= mul_mask(pll_hw);

    pll_hw.bfreq * val as c_ulong
}

unsafe fn owl_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    let pll = hw_to_owl_pll(hw);
    let pll_hw = &mut (*pll).pll_hw;
    let common = &(*pll).common;
    let mut reg: u32 = 0;

    regmap_read(common.regmap, pll_hw.reg, &mut reg);

    if (reg & (1u32 << pll_hw.bit_idx)) != 0 { 1 } else { 0 }
}

unsafe fn owl_pll_set(
    common: *const owl_clk_common,
    pll_hw: *const owl_pll_hw,
    enable: bool,
) {
    let mut reg: u32 = 0;

    regmap_read((*common).regmap, (*pll_hw).reg, &mut reg);

    if enable {
        reg |= 1u32 << (*pll_hw).bit_idx;
    } else {
        reg &= !(1u32 << (*pll_hw).bit_idx);
    }

    regmap_write((*common).regmap, (*pll_hw).reg, reg);
}

unsafe fn owl_pll_enable(hw: *mut clk_hw) -> c_int {
    let pll = hw_to_owl_pll(hw);
    let common = &(*pll).common;

    owl_pll_set(common, &(*pll).pll_hw, true);

    0
}

unsafe fn owl_pll_disable(hw: *mut clk_hw) {
    let pll = hw_to_owl_pll(hw);
    let common = &(*pll).common;

    owl_pll_set(common, &(*pll).pll_hw, false);
}

unsafe fn owl_pll_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    let pll = hw_to_owl_pll(hw);
    let pll_hw = &mut (*pll).pll_hw;
    let common = &(*pll).common;
    let clkt: *const clk_pll_table;
    let val: u32;
    let mut reg: u32 = 0;

    /* fixed frequency */
    if pll_hw.width == 0 {
        return 0;
    }

    if !pll_hw.table.is_null() {
        clkt = _get_pll_table(pll_hw.table, rate);
        val = (*clkt).val;
    } else {
        val = owl_pll_calculate_mul(pll_hw, rate);
    }

    regmap_read(common.regmap, pll_hw.reg, &mut reg);

    reg &= !mul_mask(pll_hw);
    reg |= val << pll_hw.shift;

    regmap_write(common.regmap, pll_hw.reg, reg);

    udelay(pll_hw.delay);

    0
}

static owl_pll_ops: clk_ops = clk_ops {
    enable: Some(owl_pll_enable),
    disable: Some(owl_pll_disable),
    is_enabled: Some(owl_pll_is_enabled),
    determine_rate: Some(owl_pll_determine_rate),
    recalc_rate: Some(owl_pll_recalc_rate),
    set_rate: Some(owl_pll_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
