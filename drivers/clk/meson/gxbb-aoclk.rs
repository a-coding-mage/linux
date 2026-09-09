// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/*
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */
// C dependencies: linux/platform_device.h, linux/mfd/syscon.h, linux/module.h,
// meson-aoclk.h, clk-regmap.h, clk-dualdiv.h, and GXBB clock/reset bindings.

const AO_RTI_PWR_CNTL_REG1: u32 = 0x0c;
const AO_RTI_PWR_CNTL_REG0: u32 = 0x10;
const AO_RTI_GEN_CNTL_REG0: u32 = 0x40;
const AO_OSCIN_CNTL: u32 = 0x58;
const AO_CRT_CLK_CNTL1: u32 = 0x68;
const AO_RTC_ALT_CLK_CNTL0: u32 = 0x94;
const AO_RTC_ALT_CLK_CNTL1: u32 = 0x98;

static gxbb_ao_pclk_parents: clk_parent_data = clk_parent_data { fw_name: "mpeg-clk" };

static mut gxbb_ao_remote: clk_regmap = MESON_PCLK!(gxbb_ao_remote, AO_RTI_GEN_CNTL_REG0, 0, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);
static mut gxbb_ao_i2c_master: clk_regmap = MESON_PCLK!(gxbb_ao_i2c_master, AO_RTI_GEN_CNTL_REG0, 1, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);
static mut gxbb_ao_i2c_slave: clk_regmap = MESON_PCLK!(gxbb_ao_i2c_slave, AO_RTI_GEN_CNTL_REG0, 2, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);
static mut gxbb_ao_uart1: clk_regmap = MESON_PCLK!(gxbb_ao_uart1, AO_RTI_GEN_CNTL_REG0, 3, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);
static mut gxbb_ao_uart2: clk_regmap = MESON_PCLK!(gxbb_ao_uart2, AO_RTI_GEN_CNTL_REG0, 5, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);
static mut gxbb_ao_ir_blaster: clk_regmap = MESON_PCLK!(gxbb_ao_ir_blaster, AO_RTI_GEN_CNTL_REG0, 6, &gxbb_ao_pclk_parents, CLK_IGNORE_UNUSED);

static mut gxbb_ao_cts_oscin: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTI_PWR_CNTL_REG0, bit_idx: 6 },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_cts_oscin", ops: &clk_regmap_gate_ro_ops,
        parent_data: &clk_parent_data { fw_name: "xtal" }, num_parents: 1,
    }},
};

static mut gxbb_ao_32k_pre: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTC_ALT_CLK_CNTL0, bit_idx: 31 },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_32k_pre", ops: &clk_regmap_gate_ops,
        parent_hws: &[&gxbb_ao_cts_oscin.hw], num_parents: 1,
    }},
};

static gxbb_32k_div_table: [meson_clk_dualdiv_param; 2] = [
    meson_clk_dualdiv_param { dual: 1, n1: 733, m1: 8, n2: 732, m2: 11 },
    meson_clk_dualdiv_param { dual: 0, n1: 0, m1: 0, n2: 0, m2: 0 },
];

static mut gxbb_ao_32k_div: clk_regmap = clk_regmap {
    data: &meson_clk_dualdiv_data {
        n1: clk_regmap_field { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 0, width: 12 },
        n2: clk_regmap_field { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 12, width: 12 },
        m1: clk_regmap_field { reg_off: AO_RTC_ALT_CLK_CNTL1, shift: 0, width: 12 },
        m2: clk_regmap_field { reg_off: AO_RTC_ALT_CLK_CNTL1, shift: 12, width: 12 },
        dual: clk_regmap_field { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 28, width: 1 },
        table: &gxbb_32k_div_table,
    },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_32k_div", ops: &meson_clk_dualdiv_ops,
        parent_hws: &[&gxbb_ao_32k_pre.hw], num_parents: 1,
    }},
};

static mut gxbb_ao_32k_sel: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTC_ALT_CLK_CNTL1, mask: 0x1, shift: 24, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_32k_sel", ops: &clk_regmap_mux_ops,
        parent_hws: &[&gxbb_ao_32k_div.hw, &gxbb_ao_32k_pre.hw], num_parents: 2,
        flags: CLK_SET_RATE_PARENT,
    }},
};

static mut gxbb_ao_32k: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTC_ALT_CLK_CNTL0, bit_idx: 30 },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_32k", ops: &clk_regmap_gate_ops,
        parent_hws: &[&gxbb_ao_32k_sel.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT,
    }},
};

static mut gxbb_ao_cts_rtc_oscin: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTI_PWR_CNTL_REG0, mask: 0x7, shift: 10, table: &[1, 2, 3, 4], flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_cts_rtc_oscin", ops: &clk_regmap_mux_ops,
        parent_data: &[clk_parent_data { fw_name: "ext-32k-0" }, clk_parent_data { fw_name: "ext-32k-1" }, clk_parent_data { fw_name: "ext-32k-2" }, clk_parent_data { hw: &gxbb_ao_32k.hw }],
        num_parents: 4, flags: CLK_SET_RATE_PARENT,
    }},
};

static mut gxbb_ao_clk81: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTI_PWR_CNTL_REG0, mask: 0x1, shift: 0, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_clk81", ops: &clk_regmap_mux_ro_ops,
        parent_data: &[clk_parent_data { fw_name: "mpeg-clk" }, clk_parent_data { hw: &gxbb_ao_cts_rtc_oscin.hw }],
        num_parents: 2, flags: CLK_SET_RATE_PARENT,
    }},
};

static mut gxbb_ao_cts_cec: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_CRT_CLK_CNTL1, mask: 0x1, shift: 27, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data {
        name: "ao_cts_cec", ops: &clk_regmap_mux_ops,
        // FIXME: fake parent retained because CCF may call get_parent() for the boot-selected unknown input.
        parent_data: &[clk_parent_data { name: "fixme", index: -1 }, clk_parent_data { hw: &gxbb_ao_cts_rtc_oscin.hw }],
        num_parents: 2, flags: CLK_SET_RATE_PARENT,
    }},
};

static gxbb_ao_reset: [u32; 6] = [16, 18, 19, 17, 22, 23];

static mut gxbb_ao_hw_clks: [*mut clk_hw; 14] = [
    &mut gxbb_ao_remote.hw, &mut gxbb_ao_i2c_master.hw, &mut gxbb_ao_i2c_slave.hw,
    &mut gxbb_ao_uart1.hw, &mut gxbb_ao_uart2.hw, &mut gxbb_ao_ir_blaster.hw,
    &mut gxbb_ao_cts_cec.hw, &mut gxbb_ao_cts_oscin.hw, &mut gxbb_ao_32k_pre.hw,
    &mut gxbb_ao_32k_div.hw, &mut gxbb_ao_32k_sel.hw, &mut gxbb_ao_32k.hw,
    &mut gxbb_ao_cts_rtc_oscin.hw, &mut gxbb_ao_clk81.hw,
];

static gxbb_ao_clkc_data: meson_aoclk_data = meson_aoclk_data {
    reset_reg: AO_RTI_GEN_CNTL_REG0, num_reset: gxbb_ao_reset.len(), reset: &gxbb_ao_reset,
    clkc_data: meson_clkc_data { hw_clks: meson_clk_hw_data { hws: &gxbb_ao_hw_clks, num: gxbb_ao_hw_clks.len() } },
};

static gxbb_ao_clkc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,meson-gx-aoclkc", data: &gxbb_ao_clkc_data.clkc_data },
    of_device_id::default(),
];

static mut gxbb_ao_clkc_driver: platform_driver = platform_driver {
    probe: Some(meson_aoclkc_probe),
    driver: driver { name: "gxbb-aoclkc", of_match_table: &gxbb_ao_clkc_match_table },
};

module_platform_driver!(gxbb_ao_clkc_driver);
// MODULE_DEVICE_TABLE(of, gxbb_ao_clkc_match_table);
// MODULE_DESCRIPTION("Amlogic GXBB Always-ON Clock Controller driver");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
