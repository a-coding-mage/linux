// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 huangzhenwei@allwinnertech.com
 * Copyright (C) 2021 Samuel Holland <samuel@sholland.org>
 */

// Kernel headers and local CCU definitions are supplied by the surrounding
// translation unit.

static R_AHB_APB0_PARENTS: [clk_parent_data; 4] = [
    clk_parent_data { fw_name: "hosc" },
    clk_parent_data { fw_name: "losc" },
    clk_parent_data { fw_name: "iosc" },
    clk_parent_data { fw_name: "pll-periph" },
];

sunxi_ccu_mp_data_with_mux!(r_ahb_clk, "r-ahb", R_AHB_APB0_PARENTS, 0x000,
                            0, 5, /* M */ 8, 2, /* P */
                            24, 3, /* mux */ 0);
static r_ahb_hw: *const clk_hw = unsafe { &r_ahb_clk.common.hw };

sunxi_ccu_mp_data_with_mux!(r_apb0_clk, "r-apb0", R_AHB_APB0_PARENTS, 0x00c,
                            0, 5, /* M */ 8, 2, /* P */
                            24, 3, /* mux */ 0);
static r_apb0_hw: *const clk_hw = unsafe { &r_apb0_clk.common.hw };

sunxi_ccu_gate_hws!(bus_r_timer_clk, "bus-r-timer", &r_apb0_hw, 0x11c, BIT(0), 0);
sunxi_ccu_gate_hws!(bus_r_twd_clk, "bus-r-twd", &r_apb0_hw, 0x12c, BIT(0), 0);
sunxi_ccu_gate_hws!(bus_r_ppu_clk, "bus-r-ppu", &r_apb0_hw, 0x1ac, BIT(0), 0);

static R_IR_RX_PARENTS: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: "losc" },
    clk_parent_data { fw_name: "hosc" },
];
sunxi_ccu_mp_data_with_mux_gate!(r_ir_rx_clk, "r-ir-rx", R_IR_RX_PARENTS, 0x1c0,
                                 0, 5, /* M */ 8, 2, /* P */
                                 24, 2, /* mux */ BIT(31), /* gate */ 0);

sunxi_ccu_gate_hws!(bus_r_ir_rx_clk, "bus-r-ir-rx", &r_apb0_hw, 0x1cc, BIT(0), 0);
sunxi_ccu_gate_hws!(bus_r_rtc_clk, "bus-r-rtc", &r_ahb_hw, 0x20c, BIT(0), 0);
sunxi_ccu_gate_hws!(bus_r_cpucfg_clk, "bus-r-cpucfg", &r_apb0_hw, 0x22c, BIT(0), 0);

static mut SUN20I_D1_R_CCU_CLKS: [*mut ccu_common; 9] = [
    unsafe { &mut r_ahb_clk.common }, unsafe { &mut r_apb0_clk.common },
    unsafe { &mut bus_r_timer_clk.common }, unsafe { &mut bus_r_twd_clk.common },
    unsafe { &mut bus_r_ppu_clk.common }, unsafe { &mut r_ir_rx_clk.common },
    unsafe { &mut bus_r_ir_rx_clk.common }, unsafe { &mut bus_r_rtc_clk.common },
    unsafe { &mut bus_r_cpucfg_clk.common },
];

static mut SUN20I_D1_R_HW_CLKS: clk_hw_onecell_data = clk_hw_onecell_data {
    num: CLK_NUMBER,
    hws: [
        [CLK_R_AHB] = unsafe { &r_ahb_clk.common.hw },
        [CLK_R_APB0] = unsafe { &r_apb0_clk.common.hw },
        [CLK_BUS_R_TIMER] = unsafe { &bus_r_timer_clk.common.hw },
        [CLK_BUS_R_TWD] = unsafe { &bus_r_twd_clk.common.hw },
        [CLK_BUS_R_PPU] = unsafe { &bus_r_ppu_clk.common.hw },
        [CLK_R_IR_RX] = unsafe { &r_ir_rx_clk.common.hw },
        [CLK_BUS_R_IR_RX] = unsafe { &bus_r_ir_rx_clk.common.hw },
        [CLK_BUS_R_RTC] = unsafe { &bus_r_rtc_clk.common.hw },
        [CLK_BUS_R_CPUCFG] = unsafe { &bus_r_cpucfg_clk.common.hw },
    ],
};

static SUN20I_D1_R_CCU_RESETS: [ccu_reset_map; 6] = [
    [RST_BUS_R_TIMER] = ccu_reset_map { reg: 0x11c, bit: BIT(16) },
    [RST_BUS_R_TWD] = ccu_reset_map { reg: 0x12c, bit: BIT(16) },
    [RST_BUS_R_PPU] = ccu_reset_map { reg: 0x1ac, bit: BIT(16) },
    [RST_BUS_R_IR_RX] = ccu_reset_map { reg: 0x1cc, bit: BIT(16) },
    [RST_BUS_R_RTC] = ccu_reset_map { reg: 0x20c, bit: BIT(16) },
    [RST_BUS_R_CPUCFG] = ccu_reset_map { reg: 0x22c, bit: BIT(16) },
];

static SUN20I_D1_R_CCU_DESC: sunxi_ccu_desc = sunxi_ccu_desc {
    ccu_clks: unsafe { &mut SUN20I_D1_R_CCU_CLKS },
    num_ccu_clks: ARRAY_SIZE!(SUN20I_D1_R_CCU_CLKS),
    hw_clks: unsafe { &mut SUN20I_D1_R_HW_CLKS },
    resets: &SUN20I_D1_R_CCU_RESETS,
    num_resets: ARRAY_SIZE!(SUN20I_D1_R_CCU_RESETS),
};

unsafe fn sun20i_d1_r_ccu_probe(pdev: *mut platform_device) -> c_int {
    let reg: *mut c_void = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) { return PTR_ERR!(reg); }
    devm_sunxi_ccu_probe(&(*pdev).dev, reg, &SUN20I_D1_R_CCU_DESC)
}

static SUN20I_D1_R_CCU_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "allwinner,sun20i-d1-r-ccu" },
    of_device_id { ..Default::default() },
];
MODULE_DEVICE_TABLE!(of, SUN20I_D1_R_CCU_IDS);

static mut SUN20I_D1_R_CCU_DRIVER: platform_driver = platform_driver {
    probe: Some(sun20i_d1_r_ccu_probe),
    driver: device_driver {
        name: "sun20i-d1-r-ccu", suppress_bind_attrs: true,
        of_match_table: SUN20I_D1_R_CCU_IDS.as_ptr(),
    },
};
module_platform_driver!(SUN20I_D1_R_CCU_DRIVER);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner D1/R528/T113 PRCM CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
