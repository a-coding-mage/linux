// SPDX-License-Identifier: GPL-2.0+
/*
 * Amlogic Meson8 DDR clock controller
 *
 * Copyright (C) 2019 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
 */

// Dependencies supplied by the surrounding kernel clock-controller code.

const AM_DDR_PLL_CNTL: u32 = 0x00;
const AM_DDR_PLL_CNTL1: u32 = 0x04;
const AM_DDR_PLL_CNTL2: u32 = 0x08;
const AM_DDR_PLL_CNTL3: u32 = 0x0c;
const AM_DDR_PLL_CNTL4: u32 = 0x10;
const AM_DDR_PLL_STS: u32 = 0x14;
const DDR_CLK_CNTL: u32 = 0x18;
const DDR_CLK_STS: u32 = 0x1c;

static mut meson8_ddr_pll_dco: clk_regmap = clk_regmap {
    data: &meson_clk_pll_data {
        en: meson_reg_field { reg_off: AM_DDR_PLL_CNTL, shift: 30, width: 1 },
        m: meson_reg_field { reg_off: AM_DDR_PLL_CNTL, shift: 0, width: 9 },
        n: meson_reg_field { reg_off: AM_DDR_PLL_CNTL, shift: 9, width: 5 },
        l: meson_reg_field { reg_off: AM_DDR_PLL_CNTL, shift: 31, width: 1 },
        rst: meson_reg_field { reg_off: AM_DDR_PLL_CNTL, shift: 29, width: 1 },
    },
    hw: clk_hw_init {
        init: &clk_init_data {
            name: "ddr_pll_dco",
            ops: &meson_clk_pll_ro_ops,
            parent_data: &clk_parent_data { fw_name: "xtal" },
            num_parents: 1,
        },
    },
};

static mut meson8_ddr_pll: clk_regmap = clk_regmap {
    data: &clk_regmap_div_data {
        offset: AM_DDR_PLL_CNTL,
        shift: 16,
        width: 2,
        flags: CLK_DIVIDER_POWER_OF_TWO,
    },
    hw: clk_hw_init {
        init: &clk_init_data {
            name: "ddr_pll",
            ops: &clk_regmap_divider_ro_ops,
            parent_hws: &[unsafe { &meson8_ddr_pll_dco.hw as *const _ }],
            num_parents: 1,
        },
    },
};

static mut meson8_ddr_hw_clks: [*mut clk_hw; 2] = [
    DDR_CLKID_DDR_PLL_DCO as usize as *mut clk_hw,
    DDR_CLKID_DDR_PLL as usize as *mut clk_hw,
];

static meson8_ddr_clkc_data: meson_clkc_data = meson_clkc_data {
    hw_clks: meson_clkc_hw_clks {
        hws: unsafe { &meson8_ddr_hw_clks as *const _ },
        num: 2,
    },
};

static meson8_ddr_clkc_match_table: [of_device_id; 3] = [
    of_device_id {
        compatible: "amlogic,meson8-ddr-clkc",
        data: &meson8_ddr_clkc_data,
    },
    of_device_id {
        compatible: "amlogic,meson8b-ddr-clkc",
        data: &meson8_ddr_clkc_data,
    },
    of_device_id { /* sentinel */ },
];

static mut meson8_ddr_clkc_driver: platform_driver = platform_driver {
    probe: Some(meson_clkc_mmio_probe),
    driver: driver {
        name: "meson8-ddr-clkc",
        of_match_table: &meson8_ddr_clkc_match_table,
    },
};

builtin_platform_driver!(meson8_ddr_clkc_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
