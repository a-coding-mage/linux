// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Icenowy Zheng <icenowy@aosc.io>
 */

// Kernel and local CCU dependencies supplied externally.

static SUNXI_CCU_GATE!(bus_mixer0_clk, "bus-mixer0", "bus-de", 0x04, BIT!(0), 0);
static SUNXI_CCU_GATE!(bus_mixer1_clk, "bus-mixer1", "bus-de", 0x04, BIT!(1), 0);
static SUNXI_CCU_GATE!(bus_wb_clk, "bus-wb", "bus-de", 0x04, BIT!(2), 0);
static SUNXI_CCU_GATE!(bus_rot_clk, "bus-rot", "bus-de", 0x04, BIT!(3), 0);

static SUNXI_CCU_GATE!(mixer0_clk, "mixer0", "mixer0-div", 0x00, BIT!(0), CLK_SET_RATE_PARENT);
static SUNXI_CCU_GATE!(mixer1_clk, "mixer1", "mixer1-div", 0x00, BIT!(1), CLK_SET_RATE_PARENT);
static SUNXI_CCU_GATE!(wb_clk, "wb", "wb-div", 0x00, BIT!(2), CLK_SET_RATE_PARENT);
static SUNXI_CCU_GATE!(rot_clk, "rot", "rot-div", 0x00, BIT!(3), CLK_SET_RATE_PARENT);

static SUNXI_CCU_M!(mixer0_div_clk, "mixer0-div", "de", 0x0c, 0, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(mixer1_div_clk, "mixer1-div", "de", 0x0c, 4, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(wb_div_clk, "wb-div", "de", 0x0c, 8, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(rot_div_clk, "rot-div", "de", 0x0c, 0x0c, 4, CLK_SET_RATE_PARENT);

static SUNXI_CCU_M!(mixer0_div_a83_clk, "mixer0-div", "pll-de", 0x0c, 0, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(mixer1_div_a83_clk, "mixer1-div", "pll-de", 0x0c, 4, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(wb_div_a83_clk, "wb-div", "pll-de", 0x0c, 8, 4, CLK_SET_RATE_PARENT);
static SUNXI_CCU_M!(rot_div_a83_clk, "rot-div", "pll-de", 0x0c, 0x0c, 4, CLK_SET_RATE_PARENT);

static sun8i_de2_ccu_clks: [&'static ccu_common; 16] = [
    &mixer0_clk.common, &mixer1_clk.common, &wb_clk.common, &rot_clk.common,
    &bus_mixer0_clk.common, &bus_mixer1_clk.common, &bus_wb_clk.common, &bus_rot_clk.common,
    &mixer0_div_clk.common, &mixer1_div_clk.common, &wb_div_clk.common, &rot_div_clk.common,
    &mixer0_div_a83_clk.common, &mixer1_div_a83_clk.common, &wb_div_a83_clk.common, &rot_div_a83_clk.common,
];

static sun8i_a83t_de2_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data {
    hws: [
        [CLK_MIXER0] = &mixer0_clk.common.hw, [CLK_MIXER1] = &mixer1_clk.common.hw,
        [CLK_WB] = &wb_clk.common.hw, [CLK_ROT] = &rot_clk.common.hw,
        [CLK_BUS_MIXER0] = &bus_mixer0_clk.common.hw, [CLK_BUS_MIXER1] = &bus_mixer1_clk.common.hw,
        [CLK_BUS_WB] = &bus_wb_clk.common.hw, [CLK_BUS_ROT] = &bus_rot_clk.common.hw,
        [CLK_MIXER0_DIV] = &mixer0_div_a83_clk.common.hw, [CLK_MIXER1_DIV] = &mixer1_div_a83_clk.common.hw,
        [CLK_WB_DIV] = &wb_div_a83_clk.common.hw, [CLK_ROT_DIV] = &rot_div_a83_clk.common.hw,
    ], num: CLK_NUMBER_WITH_ROT,
};

static sun8i_h3_de2_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data { hws: [
    [CLK_MIXER0] = &mixer0_clk.common.hw, [CLK_MIXER1] = &mixer1_clk.common.hw, [CLK_WB] = &wb_clk.common.hw,
    [CLK_BUS_MIXER0] = &bus_mixer0_clk.common.hw, [CLK_BUS_MIXER1] = &bus_mixer1_clk.common.hw, [CLK_BUS_WB] = &bus_wb_clk.common.hw,
    [CLK_MIXER0_DIV] = &mixer0_div_clk.common.hw, [CLK_MIXER1_DIV] = &mixer1_div_clk.common.hw, [CLK_WB_DIV] = &wb_div_clk.common.hw,
], num: CLK_NUMBER_WITHOUT_ROT };
static sun8i_v3s_de2_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data { hws: [
    [CLK_MIXER0] = &mixer0_clk.common.hw, [CLK_WB] = &wb_clk.common.hw,
    [CLK_BUS_MIXER0] = &bus_mixer0_clk.common.hw, [CLK_BUS_WB] = &bus_wb_clk.common.hw,
    [CLK_MIXER0_DIV] = &mixer0_div_clk.common.hw, [CLK_WB_DIV] = &wb_div_clk.common.hw,
], num: CLK_NUMBER_WITHOUT_ROT };
static sun50i_a64_de2_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data { hws: [
    [CLK_MIXER0] = &mixer0_clk.common.hw, [CLK_MIXER1] = &mixer1_clk.common.hw, [CLK_WB] = &wb_clk.common.hw, [CLK_ROT] = &rot_clk.common.hw,
    [CLK_BUS_MIXER0] = &bus_mixer0_clk.common.hw, [CLK_BUS_MIXER1] = &bus_mixer1_clk.common.hw, [CLK_BUS_WB] = &bus_wb_clk.common.hw, [CLK_BUS_ROT] = &bus_rot_clk.common.hw,
    [CLK_MIXER0_DIV] = &mixer0_div_clk.common.hw, [CLK_MIXER1_DIV] = &mixer1_div_clk.common.hw, [CLK_WB_DIV] = &wb_div_clk.common.hw, [CLK_ROT_DIV] = &rot_div_clk.common.hw,
], num: CLK_NUMBER_WITH_ROT };

static sun8i_a83t_de2_resets: [ccu_reset_map; 4] = [
    [RST_MIXER0] = ccu_reset_map { reg: 0x08, bit: BIT!(0) },
    // Mixer1 reset line is shared with wb, so only RST_WB is exported here.
    [RST_WB] = ccu_reset_map { reg: 0x08, bit: BIT!(2) }, [RST_ROT] = ccu_reset_map { reg: 0x08, bit: BIT!(3) },
];
static sun8i_h3_de2_resets: [ccu_reset_map; 3] = [
    [RST_MIXER0] = ccu_reset_map { reg: 0x08, bit: BIT!(0) },
    // Mixer1 reset line is shared with wb, so only RST_WB is exported here.
    // V3s doesn't have mixer1, so it also shares this struct.
    [RST_WB] = ccu_reset_map { reg: 0x08, bit: BIT!(2) },
];
static sun50i_a64_de2_resets: [ccu_reset_map; 4] = [
    [RST_MIXER0] = ccu_reset_map { reg: 0x08, bit: BIT!(0) }, [RST_MIXER1] = ccu_reset_map { reg: 0x08, bit: BIT!(1) },
    [RST_WB] = ccu_reset_map { reg: 0x08, bit: BIT!(2) }, [RST_ROT] = ccu_reset_map { reg: 0x08, bit: BIT!(3) },
];
static sun50i_h5_de2_resets: [ccu_reset_map; 3] = [
    [RST_MIXER0] = ccu_reset_map { reg: 0x08, bit: BIT!(0) }, [RST_MIXER1] = ccu_reset_map { reg: 0x08, bit: BIT!(1) }, [RST_WB] = ccu_reset_map { reg: 0x08, bit: BIT!(2) },
];

// Descriptor declarations preserve the original clock/reset topology.
static sun8i_a83t_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun8i_a83t_de2_hw_clks, resets: &sun8i_a83t_de2_resets, num_resets: ARRAY_SIZE!(sun8i_a83t_de2_resets) };
static sun8i_h3_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun8i_h3_de2_hw_clks, resets: &sun8i_h3_de2_resets, num_resets: ARRAY_SIZE!(sun8i_h3_de2_resets) };
static sun8i_r40_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun50i_a64_de2_hw_clks, resets: &sun8i_a83t_de2_resets, num_resets: ARRAY_SIZE!(sun8i_a83t_de2_resets) };
static sun8i_v3s_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun8i_v3s_de2_hw_clks, resets: &sun8i_a83t_de2_resets, num_resets: ARRAY_SIZE!(sun8i_a83t_de2_resets) };
static sun50i_a64_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun50i_a64_de2_hw_clks, resets: &sun50i_a64_de2_resets, num_resets: ARRAY_SIZE!(sun50i_a64_de2_resets) };
static sun50i_h5_de2_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun8i_h3_de2_hw_clks, resets: &sun50i_h5_de2_resets, num_resets: ARRAY_SIZE!(sun50i_h5_de2_resets) };
static sun50i_h616_de33_clk_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: &sun8i_de2_ccu_clks, num_ccu_clks: ARRAY_SIZE!(sun8i_de2_ccu_clks), hw_clks: &sun8i_h3_de2_hw_clks, resets: &sun50i_h5_de2_resets, num_resets: ARRAY_SIZE!(sun50i_h5_de2_resets) };

unsafe fn sunxi_de2_clk_probe(pdev: *mut platform_device) -> i32 {
    let mut ccu_desc = of_device_get_match_data(&(*pdev).dev);
    if ccu_desc.is_null() { return -EINVAL; }
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let bus_clk = devm_clk_get(&(*pdev).dev, "bus");
    if IS_ERR(bus_clk) { return dev_err_probe(&(*pdev).dev, PTR_ERR(bus_clk), "Couldn't get bus clk\n"); }
    let mod_clk = devm_clk_get(&(*pdev).dev, "mod");
    if IS_ERR(mod_clk) { return dev_err_probe(&(*pdev).dev, PTR_ERR(mod_clk), "Couldn't get mod clk\n"); }
    let rstc = devm_reset_control_get_exclusive(&(*pdev).dev, core::ptr::null());
    if IS_ERR(rstc) { return dev_err_probe(&(*pdev).dev, PTR_ERR(rstc), "Couldn't get reset control\n"); }
    let mut ret = clk_prepare_enable(bus_clk);
    if ret != 0 { dev_err(&(*pdev).dev, "Couldn't enable bus clk: %d\n", ret); return ret; }
    ret = clk_prepare_enable(mod_clk);
    if ret != 0 { dev_err(&(*pdev).dev, "Couldn't enable mod clk: %d\n", ret); clk_disable_unprepare(bus_clk); return ret; }
    ret = reset_control_deassert(rstc);
    if ret != 0 { dev_err(&(*pdev).dev, "Couldn't deassert reset control: %d\n", ret); clk_disable_unprepare(mod_clk); clk_disable_unprepare(bus_clk); return ret; }
    if of_device_is_compatible((*pdev).dev.of_node, "allwinner,sun50i-h616-de33-clk") {
        writel(0, reg.add(0x24)); writel(0x0000a980, reg.add(0x28));
    }
    ret = devm_sunxi_ccu_probe(&(*pdev).dev, reg, ccu_desc);
    if ret != 0 { reset_control_assert(rstc); clk_disable_unprepare(mod_clk); clk_disable_unprepare(bus_clk); }
    ret
}

static sunxi_de2_clk_ids: [of_device_id; 9] = [
    of_device_id { compatible: "allwinner,sun8i-a83t-de2-clk", data: &sun8i_a83t_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun8i-h3-de2-clk", data: &sun8i_h3_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun8i-r40-de2-clk", data: &sun8i_r40_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun8i-v3s-de2-clk", data: &sun8i_v3s_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun50i-a64-de2-clk", data: &sun50i_a64_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun50i-h5-de2-clk", data: &sun50i_h5_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun50i-h6-de3-clk", data: &sun50i_h5_de2_clk_desc },
    of_device_id { compatible: "allwinner,sun50i-h616-de33-clk", data: &sun50i_h616_de33_clk_desc },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static sunxi_de2_clk_driver: platform_driver = platform_driver {
    probe: Some(sunxi_de2_clk_probe),
    driver: driver { name: "sunxi-de2-clks", of_match_table: &sunxi_de2_clk_ids },
};
module_platform_driver!(sunxi_de2_clk_driver);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner SoCs DE2 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
