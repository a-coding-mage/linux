// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2019 Amlogic, Inc. All rights reserved.
 * Author: Jian Hu <jian.hu@amlogic.com>
 *
 * Copyright (c) 2023, SberDevices. All Rights Reserved.
 * Author: Dmitry Rokosov <ddrokosov@sberdevices.ru>
 */

// Dependencies supplied by the surrounding kernel clock-controller code.

const ANACTRL_FIXPLL_CTRL0: u32 = 0x0;
const ANACTRL_FIXPLL_CTRL1: u32 = 0x4;
const ANACTRL_FIXPLL_STS: u32 = 0x14;
const ANACTRL_HIFIPLL_CTRL0: u32 = 0xc0;
const ANACTRL_HIFIPLL_CTRL1: u32 = 0xc4;
const ANACTRL_HIFIPLL_CTRL2: u32 = 0xc8;
const ANACTRL_HIFIPLL_CTRL3: u32 = 0xcc;
const ANACTRL_HIFIPLL_CTRL4: u32 = 0xd0;
const ANACTRL_HIFIPLL_STS: u32 = 0xd4;

static mut a1_fixed_pll_dco: clk_regmap = clk_regmap {
    data: &meson_clk_pll_data {
        en: reg_field { reg_off: ANACTRL_FIXPLL_CTRL0, shift: 28, width: 1 },
        m: reg_field { reg_off: ANACTRL_FIXPLL_CTRL0, shift: 0, width: 8 },
        n: reg_field { reg_off: ANACTRL_FIXPLL_CTRL0, shift: 10, width: 5 },
        frac: reg_field { reg_off: ANACTRL_FIXPLL_CTRL1, shift: 0, width: 19 },
        l: reg_field { reg_off: ANACTRL_FIXPLL_STS, shift: 31, width: 1 },
        rst: reg_field { reg_off: ANACTRL_FIXPLL_CTRL0, shift: 29, width: 1 },
    },
    hw: clk_hw_with_init {
        init: &clk_init_data {
            name: "fixed_pll_dco",
            ops: &meson_clk_pll_ro_ops,
            parent_data: &clk_parent_data { fw_name: "fixpll_in" },
            num_parents: 1,
        },
    },
};

static mut a1_fixed_pll: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL0, bit_idx: 20 },
    hw: clk_hw_with_init {
        init: &clk_init_data {
            name: "fixed_pll",
            ops: &clk_regmap_gate_ops,
            parent_hws: &[unsafe { &a1_fixed_pll_dco.hw }],
            num_parents: 1,
        },
    },
};

static a1_hifi_pll_range: pll_mult_range = pll_mult_range { min: 32, max: 64 };

static a1_hifi_pll_init_regs: [reg_sequence; 5] = [
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL1, def: 0x01800000 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL2, def: 0x00001100 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL3, def: 0x100a1100 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL4, def: 0x00302000 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL0, def: 0x01f18000 },
];

static mut a1_hifi_pll: clk_regmap = clk_regmap {
    data: &meson_clk_pll_data {
        en: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL0, shift: 28, width: 1 },
        m: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL0, shift: 0, width: 8 },
        n: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL0, shift: 10, width: 5 },
        frac: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL1, shift: 0, width: 19 },
        l: reg_field { reg_off: ANACTRL_HIFIPLL_STS, shift: 31, width: 1 },
        current_en: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL0, shift: 26, width: 1 },
        l_detect: reg_field { reg_off: ANACTRL_HIFIPLL_CTRL2, shift: 6, width: 1 },
        range: &a1_hifi_pll_range,
        init_regs: &a1_hifi_pll_init_regs,
        init_count: a1_hifi_pll_init_regs.len(),
    },
    hw: clk_hw_with_init {
        init: &clk_init_data {
            name: "hifi_pll",
            ops: &meson_clk_pll_ops,
            parent_data: &clk_parent_data { fw_name: "hifipll_in" },
            num_parents: 1,
        },
    },
};

static mut a1_fclk_div2_div: clk_fixed_factor = clk_fixed_factor {
    mult: 1,
    div: 2,
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div2_div", ops: &clk_fixed_factor_ops,
        parent_hws: &[unsafe { &a1_fixed_pll.hw }], num_parents: 1,
    } },
};

static mut a1_fclk_div2: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL0, bit_idx: 21 },
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div2", ops: &clk_regmap_gate_ops,
        parent_hws: &[unsafe { &a1_fclk_div2_div.hw }], num_parents: 1,
        /* This clock is used by DDR clock in BL2 firmware and is required by
         * the platform to operate correctly. Until the following conditions
         * are met, this clock must be marked as critical:
         * a) Mark the clock used by a firmware resource, if possible
         * b) CCF has a clock hand-off mechanism to keep the clock on until
         *    the proper driver comes along
         */
        flags: CLK_IS_CRITICAL,
    } },
};

static mut a1_fclk_div3_div: clk_fixed_factor = clk_fixed_factor {
    mult: 1, div: 3,
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div3_div", ops: &clk_fixed_factor_ops,
        parent_hws: &[unsafe { &a1_fixed_pll.hw }], num_parents: 1,
    } },
};

static mut a1_fclk_div3: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL0, bit_idx: 22 },
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div3", ops: &clk_regmap_gate_ops,
        parent_hws: &[unsafe { &a1_fclk_div3_div.hw }], num_parents: 1,
        /* This clock is used by APB bus set in boot ROM code and is required
         * by the platform to operate correctly. */
        flags: CLK_IS_CRITICAL,
    } },
};

static mut a1_fclk_div5_div: clk_fixed_factor = clk_fixed_factor {
    mult: 1, div: 5,
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div5_div", ops: &clk_fixed_factor_ops,
        parent_hws: &[unsafe { &a1_fixed_pll.hw }], num_parents: 1,
    } },
};

static mut a1_fclk_div5: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL0, bit_idx: 23 },
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div5", ops: &clk_regmap_gate_ops,
        parent_hws: &[unsafe { &a1_fclk_div5_div.hw }], num_parents: 1,
        /* This clock is used by AXI bus set in Romcode and is required by the
         * platform to operate correctly. */
        flags: CLK_IS_CRITICAL,
    } },
};

static mut a1_fclk_div7_div: clk_fixed_factor = clk_fixed_factor {
    mult: 1, div: 7,
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div7_div", ops: &clk_fixed_factor_ops,
        parent_hws: &[unsafe { &a1_fixed_pll.hw }], num_parents: 1,
    } },
};

static mut a1_fclk_div7: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL0, bit_idx: 24 },
    hw: clk_hw_with_init { init: &clk_init_data {
        name: "fclk_div7", ops: &clk_regmap_gate_ops,
        parent_hws: &[unsafe { &a1_fclk_div7_div.hw }], num_parents: 1,
    } },
};

/* Array of all clocks registered by this provider */
static mut a1_pll_hw_clks: [*mut clk_hw; 11] = [
    [CLKID_FIXED_PLL_DCO] = unsafe { &mut a1_fixed_pll_dco.hw },
    [CLKID_FIXED_PLL] = unsafe { &mut a1_fixed_pll.hw },
    [CLKID_FCLK_DIV2_DIV] = unsafe { &mut a1_fclk_div2_div.hw },
    [CLKID_FCLK_DIV3_DIV] = unsafe { &mut a1_fclk_div3_div.hw },
    [CLKID_FCLK_DIV5_DIV] = unsafe { &mut a1_fclk_div5_div.hw },
    [CLKID_FCLK_DIV7_DIV] = unsafe { &mut a1_fclk_div7_div.hw },
    [CLKID_FCLK_DIV2] = unsafe { &mut a1_fclk_div2.hw },
    [CLKID_FCLK_DIV3] = unsafe { &mut a1_fclk_div3.hw },
    [CLKID_FCLK_DIV5] = unsafe { &mut a1_fclk_div5.hw },
    [CLKID_FCLK_DIV7] = unsafe { &mut a1_fclk_div7.hw },
    [CLKID_HIFI_PLL] = unsafe { &mut a1_hifi_pll.hw },
];

static a1_pll_clkc_data: meson_clkc_data = meson_clkc_data {
    hw_clks: meson_clk_hw_data {
        hws: &a1_pll_hw_clks,
        num: a1_pll_hw_clks.len(),
    },
};

static a1_pll_clkc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,a1-pll-clkc", data: &a1_pll_clkc_data },
    of_device_id {},
];

static mut a1_pll_clkc_driver: platform_driver = platform_driver {
    probe: meson_clkc_mmio_probe,
    driver: driver {
        name: "a1-pll-clkc",
        of_match_table: &a1_pll_clkc_match_table,
    },
};

module_platform_driver!(a1_pll_clkc_driver);

module_description!("Amlogic A1 PLL Clock Controller driver");
module_author!("Jian Hu <jian.hu@amlogic.com>");
module_author!("Dmitry Rokosov <ddrokosov@sberdevices.ru>");
module_license!("GPL");
module_import_ns!("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
