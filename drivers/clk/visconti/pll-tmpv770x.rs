// SPDX-License-Identifier: GPL-2.0-only
/*
 * Toshiba Visconti PLL controller
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/of_address.h, linux/slab.h,
// dt-bindings/clock/toshiba,tmpv770x.h, and pll.h.

/* Must be equal to the last pll ID increased by one */
const PLLS_NR: usize = TMPV770X_PLL_PIIMGERPLL as usize + 1;

static tmpv770x_pll_lock: spinlock_t = DEFINE_SPINLOCK!();

static pipll0_rates: [visconti_pll_rate_table; 4] = [
    VISCONTI_PLL_RATE!(840000000, 0x1, 0x0, 0x1, 0x54, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE!(780000000, 0x1, 0x0, 0x1, 0x4e, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE!(600000000, 0x1, 0x0, 0x1, 0x3c, 0x000000, 0x2, 0x1),
    visconti_pll_rate_table { /* sentinel */ },
];

static piddrcpll_rates: [visconti_pll_rate_table; 3] = [
    VISCONTI_PLL_RATE!(780000000, 0x1, 0x0, 0x1, 0x4e, 0x000000, 0x2, 0x1),
    VISCONTI_PLL_RATE!(760000000, 0x1, 0x0, 0x1, 0x4c, 0x000000, 0x2, 0x1),
    visconti_pll_rate_table { /* sentinel */ },
];

static pivoifpll_rates: [visconti_pll_rate_table; 8] = [
    VISCONTI_PLL_RATE!(165000000, 0x1, 0x0, 0x1, 0x42, 0x000000, 0x4, 0x2),
    VISCONTI_PLL_RATE!(148500000, 0x1, 0x1, 0x1, 0x3b, 0x666666, 0x4, 0x2),
    VISCONTI_PLL_RATE!(96000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x2),
    VISCONTI_PLL_RATE!(74250000, 0x1, 0x1, 0x1, 0x3b, 0x666666, 0x4, 0x4),
    VISCONTI_PLL_RATE!(54000000, 0x1, 0x0, 0x1, 0x36, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE!(48000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE!(35750000, 0x1, 0x1, 0x1, 0x32, 0x0ccccc, 0x7, 0x4),
    visconti_pll_rate_table { /* sentinel */ },
];

static piimgerpll_rates: [visconti_pll_rate_table; 5] = [
    VISCONTI_PLL_RATE!(165000000, 0x1, 0x0, 0x1, 0x42, 0x000000, 0x4, 0x2),
    VISCONTI_PLL_RATE!(96000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x2),
    VISCONTI_PLL_RATE!(54000000, 0x1, 0x0, 0x1, 0x36, 0x000000, 0x5, 0x4),
    VISCONTI_PLL_RATE!(48000000, 0x1, 0x0, 0x1, 0x30, 0x000000, 0x5, 0x4),
    visconti_pll_rate_table { /* sentinel */ },
];

static pll_info: [visconti_pll_info; 4] = [
    visconti_pll_info { id: TMPV770X_PLL_PIPLL0, name: c"pipll0", parent_name: c"osc2-clk", offset: 0x0, rate_table: &pipll0_rates },
    visconti_pll_info { id: TMPV770X_PLL_PIDDRCPLL, name: c"piddrcpll", parent_name: c"osc2-clk", offset: 0x500, rate_table: &piddrcpll_rates },
    visconti_pll_info { id: TMPV770X_PLL_PIVOIFPLL, name: c"pivoifpll", parent_name: c"osc2-clk", offset: 0x600, rate_table: &pivoifpll_rates },
    visconti_pll_info { id: TMPV770X_PLL_PIIMGERPLL, name: c"piimgerpll", parent_name: c"osc2-clk", offset: 0x700, rate_table: &piimgerpll_rates },
];

unsafe fn tmpv770x_setup_plls(np: *mut device_node) {
    let reg_base: *mut core::ffi::c_void = of_iomap(np, 0);
    if reg_base.is_null() { return; }

    let ctx = visconti_init_pll(np, reg_base, PLLS_NR);
    if IS_ERR!(ctx) {
        iounmap(reg_base);
        return;
    }

    (*ctx).clk_data.hws[TMPV770X_PLL_PIPLL1 as usize] =
        clk_hw_register_fixed_rate(core::ptr::null_mut(), c"pipll1", core::ptr::null(), 0, 600000000);
    (*ctx).clk_data.hws[TMPV770X_PLL_PIDNNPLL as usize] =
        clk_hw_register_fixed_rate(core::ptr::null_mut(), c"pidnnpll", core::ptr::null(), 0, 500000000);
    (*ctx).clk_data.hws[TMPV770X_PLL_PIETHERPLL as usize] =
        clk_hw_register_fixed_rate(core::ptr::null_mut(), c"pietherpll", core::ptr::null(), 0, 500000000);

    visconti_register_plls(ctx, pll_info.as_ptr(), pll_info.len(), &tmpv770x_pll_lock);
}

CLK_OF_DECLARE!(tmpv770x_plls, c"toshiba,tmpv7708-pipllct", tmpv770x_setup_plls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
