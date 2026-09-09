// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

// Dependencies supplied by the surrounding kernel clock implementation.

static inline fn meson_vid_pll_div_data(clk: *mut clk_regmap) -> *mut meson_vid_pll_div_data {
    unsafe { (*clk).data as *mut meson_vid_pll_div_data }
}

/*
 * This vid_pll divided is a fully programmable fractionnal divider to
 * achieve complex video clock rates.
 *
 * Here are provided the commonly used fraction values provided by Amlogic.
 */

#[repr(C)]
struct vid_pll_div {
    shift_val: u32,
    shift_sel: u32,
    divider: u32,
    multiplier: u32,
}

const fn vid_pll_div(val: u32, sel: u32, ft: u32, fb: u32) -> vid_pll_div {
    vid_pll_div {
        shift_val: val,
        shift_sel: sel,
        divider: ft,
        multiplier: fb,
    }
}

static VID_PLL_DIV_TABLE: [vid_pll_div; 14] = [
    vid_pll_div(0x0aaa, 0, 2, 1), // 2/1  => /2
    vid_pll_div(0x5294, 2, 5, 2), // 5/2  => /2.5
    vid_pll_div(0x0db6, 0, 3, 1), // 3/1  => /3
    vid_pll_div(0x36cc, 1, 7, 2), // 7/2  => /3.5
    vid_pll_div(0x6666, 2, 15, 4), // 15/4 => /3.75
    vid_pll_div(0x0ccc, 0, 4, 1), // 4/1  => /4
    vid_pll_div(0x739c, 2, 5, 1), // 5/1  => /5
    vid_pll_div(0x0e38, 0, 6, 1), // 6/1  => /6
    vid_pll_div(0x0000, 3, 25, 4), // 25/4 => /6.25
    vid_pll_div(0x3c78, 1, 7, 1), // 7/1  => /7
    vid_pll_div(0x78f0, 2, 15, 2), // 15/2 => /7.5
    vid_pll_div(0x0fc0, 0, 12, 1), // 12/1 => /12
    vid_pll_div(0x3f80, 1, 14, 1), // 14/1 => /14
    vid_pll_div(0x7f80, 2, 15, 1), // 15/1 => /15
];

static fn _get_table_val(shift_val: u32, shift_sel: u32) -> *const vid_pll_div {
    let mut i = 0usize;
    while i < VID_PLL_DIV_TABLE.len() {
        if VID_PLL_DIV_TABLE[i].shift_val == shift_val
            && VID_PLL_DIV_TABLE[i].shift_sel == shift_sel
        {
            return &VID_PLL_DIV_TABLE[i] as *const vid_pll_div;
        }
        i += 1;
    }

    core::ptr::null()
}

unsafe fn meson_vid_pll_div_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let clk: *mut clk_regmap = to_clk_regmap(hw);
    let pll_div: *mut meson_vid_pll_div_data = meson_vid_pll_div_data(clk);
    let div: *const vid_pll_div;

    div = _get_table_val(
        meson_parm_read((*clk).map, &(*pll_div).val),
        meson_parm_read((*clk).map, &(*pll_div).sel),
    );
    if div.is_null() || (*div).divider == 0 {
        pr_debug!("{}: Invalid config value for vid_pll_div\n", "meson_vid_pll_div_recalc_rate");
        return 0;
    }

    let numerator = parent_rate.wrapping_mul((*div).multiplier as u64);
    numerator
        .wrapping_add((*div).divider as u64 - 1)
        / (*div).divider as u64
}

pub static meson_vid_pll_div_ro_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init),
    recalc_rate: Some(meson_vid_pll_div_recalc_rate),
};

// EXPORT_SYMBOL_NS_GPL(meson_vid_pll_div_ro_ops, "CLK_MESON");
// MODULE_DESCRIPTION("Amlogic video pll divider driver");
// MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
