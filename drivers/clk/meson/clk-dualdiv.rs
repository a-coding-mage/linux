// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017 BayLibre, SAS
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/*
 * The AO Domain embeds a dual/divider to generate a more precise
 * 32,768KHz clock for low-power suspend mode and CEC.
 *     ______   ______
 *    |      | |      |
 *    | Div1 |-| Cnt1 |
 *   /|______| |______|\
 * -|  ______   ______  X--> Out
 *   \|      | |      |/
 *    | Div2 |-| Cnt2 |
 *    |______| |______|
 *
 * The dividing can be switched to single or dual, with a counter
 * for each divider to set when the switching is done.
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn meson_clk_dualdiv_data(clk: *mut clk_regmap) -> *mut meson_clk_dualdiv_data {
    (*clk).data as *mut meson_clk_dualdiv_data
}

unsafe fn __dualdiv_param_to_rate(
    parent_rate: libc::c_ulong,
    p: *const meson_clk_dualdiv_param,
) -> libc::c_ulong {
    if !(*p).dual {
        return DIV_ROUND_CLOSEST(parent_rate, (*p).n1);
    }

    DIV_ROUND_CLOSEST(
        parent_rate.wrapping_mul((*p).m1.wrapping_add((*p).m2)),
        (*p).n1
            .wrapping_mul((*p).m1)
            .wrapping_add((*p).n2.wrapping_mul((*p).m2)),
    )
}

unsafe fn meson_clk_dualdiv_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let clk = to_clk_regmap(hw);
    let dualdiv = meson_clk_dualdiv_data(clk);
    let mut setting: meson_clk_dualdiv_param = core::mem::zeroed();

    setting.dual = meson_parm_read((*clk).map, &(*dualdiv).dual);
    setting.n1 = meson_parm_read((*clk).map, &(*dualdiv).n1).wrapping_add(1);
    setting.m1 = meson_parm_read((*clk).map, &(*dualdiv).m1).wrapping_add(1);
    setting.n2 = meson_parm_read((*clk).map, &(*dualdiv).n2).wrapping_add(1);
    setting.m2 = meson_parm_read((*clk).map, &(*dualdiv).m2).wrapping_add(1);

    __dualdiv_param_to_rate(parent_rate, &setting)
}

unsafe fn __dualdiv_get_setting(
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
    dualdiv: *mut meson_clk_dualdiv_data,
) -> *const meson_clk_dualdiv_param {
    let table = (*dualdiv).table;
    let mut best: libc::c_ulong = 0;
    let mut now: libc::c_ulong = 0;
    let mut best_i: usize = 0;

    if table.is_null() {
        return core::ptr::null();
    }

    let mut i: usize = 0;
    while (*table.add(i)).n1 != 0 {
        now = __dualdiv_param_to_rate(parent_rate, table.add(i));

        /* If we get an exact match, don't bother any further */
        if now == rate {
            return table.add(i);
        } else if now.abs_diff(rate) < best.abs_diff(rate) {
            best = now;
            best_i = i;
        }
        i += 1;
    }

    table.add(best_i)
}

unsafe fn meson_clk_dualdiv_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> libc::c_int {
    let clk = to_clk_regmap(hw);
    let dualdiv = meson_clk_dualdiv_data(clk);
    let setting = __dualdiv_get_setting((*req).rate, (*req).best_parent_rate, dualdiv);

    if !setting.is_null() {
        (*req).rate = __dualdiv_param_to_rate((*req).best_parent_rate, setting);
    } else {
        (*req).rate = meson_clk_dualdiv_recalc_rate(hw, (*req).best_parent_rate);
    }

    0
}

unsafe fn meson_clk_dualdiv_set_rate(
    hw: *mut clk_hw,
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
) -> libc::c_int {
    let clk = to_clk_regmap(hw);
    let dualdiv = meson_clk_dualdiv_data(clk);
    let setting = __dualdiv_get_setting(rate, parent_rate, dualdiv);

    if setting.is_null() {
        return -EINVAL;
    }

    meson_parm_write((*clk).map, &(*dualdiv).dual, (*setting).dual);
    meson_parm_write((*clk).map, &(*dualdiv).n1, (*setting).n1.wrapping_sub(1));
    meson_parm_write((*clk).map, &(*dualdiv).m1, (*setting).m1.wrapping_sub(1));
    meson_parm_write((*clk).map, &(*dualdiv).n2, (*setting).n2.wrapping_sub(1));
    meson_parm_write((*clk).map, &(*dualdiv).m2, (*setting).m2.wrapping_sub(1));

    0
}

pub static meson_clk_dualdiv_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init),
    recalc_rate: Some(meson_clk_dualdiv_recalc_rate),
    determine_rate: Some(meson_clk_dualdiv_determine_rate),
    set_rate: Some(meson_clk_dualdiv_set_rate),
};

pub static meson_clk_dualdiv_ro_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init),
    recalc_rate: Some(meson_clk_dualdiv_recalc_rate),
    ..core::mem::zeroed()
};

// EXPORT_SYMBOL_NS_GPL(meson_clk_dualdiv_ops, "CLK_MESON");
// EXPORT_SYMBOL_NS_GPL(meson_clk_dualdiv_ro_ops, "CLK_MESON");
// MODULE_DESCRIPTION("Amlogic dual divider driver");
// MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
