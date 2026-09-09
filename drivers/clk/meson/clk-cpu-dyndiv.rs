// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

// Dependencies supplied by the Linux clock-provider, module, regmap, and
// Meson CPU dynamic-divider interfaces.

#[inline]
unsafe fn meson_clk_cpu_dyndiv_data(
    clk: *mut clk_regmap,
) -> *mut meson_clk_cpu_dyndiv_data {
    (*clk).data as *mut meson_clk_cpu_dyndiv_data
}

unsafe fn meson_clk_cpu_dyndiv_recalc_rate(
    hw: *mut clk_hw,
    prate: c_ulong,
) -> c_ulong {
    let clk: *mut clk_regmap = to_clk_regmap(hw);
    let data: *mut meson_clk_cpu_dyndiv_data = meson_clk_cpu_dyndiv_data(clk);

    divider_recalc_rate(
        hw,
        prate,
        meson_parm_read((*clk).map, &(*data).div),
        core::ptr::null_mut(),
        0,
        (*data).div.width,
    )
}

unsafe fn meson_clk_cpu_dyndiv_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let clk: *mut clk_regmap = to_clk_regmap(hw);
    let data: *mut meson_clk_cpu_dyndiv_data = meson_clk_cpu_dyndiv_data(clk);

    divider_determine_rate(hw, req, core::ptr::null_mut(), (*data).div.width, 0)
}

unsafe fn meson_clk_cpu_dyndiv_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let clk: *mut clk_regmap = to_clk_regmap(hw);
    let data: *mut meson_clk_cpu_dyndiv_data = meson_clk_cpu_dyndiv_data(clk);
    let mut val: c_uint;
    let ret: c_int;

    ret = divider_get_val(rate, parent_rate, core::ptr::null_mut(), (*data).div.width, 0);
    if ret < 0 {
        return ret;
    }

    val = (ret as c_uint) << (*data).div.shift;

    /* Write the SYS_CPU_DYN_ENABLE bit before changing the divider */
    meson_parm_write((*clk).map, &(*data).dyn_, 1);

    /* Update the divider while removing the SYS_CPU_DYN_ENABLE bit */
    regmap_update_bits(
        (*clk).map,
        (*data).div.reg_off,
        SETPMASK((*data).div.width, (*data).div.shift)
            | SETPMASK((*data).dyn_.width, (*data).dyn_.shift),
        val,
    )
}

pub static meson_clk_cpu_dyndiv_ops: clk_ops = clk_ops {
    .init = Some(clk_regmap_init),
    .recalc_rate = Some(meson_clk_cpu_dyndiv_recalc_rate),
    .determine_rate = Some(meson_clk_cpu_dyndiv_determine_rate),
    .set_rate = Some(meson_clk_cpu_dyndiv_set_rate),
};

// EXPORT_SYMBOL_NS_GPL(meson_clk_cpu_dyndiv_ops, "CLK_MESON");
// MODULE_DESCRIPTION("Amlogic CPU Dynamic Clock divider");
// MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
