// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017 Icenowy Zheng <icenowy@aosc.xyz>
 */

// Linux clock-provider, module, OF, platform-device, and local CCU dependencies
// are supplied by the surrounding translation unit.

/*
 * Information about AR100 and AHB/APB clocks in R_CCU are gathered from
 * clock definitions in the BSP source code.
 */

static AR100_R_APB2_PARENTS: [&'static str; 4] = ["osc24M", "osc32k", "iosc", "pll-periph0"];
static AR100_R_APB2_PREDIVS: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv { index: 3, shift: 0, width: 5 }];

static mut ar100_clk: ccu_div = ccu_div {
    div: _SUNXI_CCU_DIV_FLAGS!(8, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: ccu_mux { shift: 24, width: 2, var_predivs: AR100_R_APB2_PREDIVS.as_ptr(), n_var_predivs: ARRAY_SIZE!(AR100_R_APB2_PREDIVS) },
    common: ccu_common { reg: 0x000, features: CCU_FEATURE_VARIABLE_PREDIV, hw: CLK_HW_INIT_PARENTS!("ar100", AR100_R_APB2_PARENTS.as_ptr(), &ccu_div_ops, 0) },
};

static r_ahb_clk: clk_fixed_factor_hw = CLK_FIXED_FACTOR_HW!("r-ahb", unsafe { &ar100_clk.common.hw }, 1, 1, 0);
static r_apb1_clk: ccu_m = SUNXI_CCU_M!("r-apb1", "r-ahb", 0x00c, 0, 2, 0);

static mut r_apb2_clk: ccu_div = ccu_div {
    div: _SUNXI_CCU_DIV_FLAGS!(8, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: ccu_mux { shift: 24, width: 2, var_predivs: AR100_R_APB2_PREDIVS.as_ptr(), n_var_predivs: ARRAY_SIZE!(AR100_R_APB2_PREDIVS) },
    common: ccu_common { reg: 0x010, features: CCU_FEATURE_VARIABLE_PREDIV, hw: CLK_HW_INIT_PARENTS!("r-apb2", AR100_R_APB2_PARENTS.as_ptr(), &ccu_div_ops, 0) },
};

static r_apb1_timer_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-timer", "r-apb1", 0x11c, BIT!(0), 0);
static r_apb1_twd_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-twd", "r-apb1", 0x12c, BIT!(0), 0);
static r_apb1_pwm_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-pwm", "r-apb1", 0x13c, BIT!(0), 0);
static r_apb2_uart_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb2-uart", "r-apb2", 0x18c, BIT!(0), 0);
static r_apb2_i2c_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb2-i2c", "r-apb2", 0x19c, BIT!(0), 0);
static r_apb2_rsb_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb2-rsb", "r-apb2", 0x1bc, BIT!(0), 0);
static r_apb1_ir_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-ir", "r-apb1", 0x1cc, BIT!(0), 0);
static r_apb1_w1_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-w1", "r-apb1", 0x1ec, BIT!(0), 0);
static r_apb1_rtc_clk: ccu_gate = SUNXI_CCU_GATE!("r-apb1-rtc", "r-apb1", 0x20c, BIT!(0), CLK_IGNORE_UNUSED);

/* Information of IR(RX) mod clock is gathered from BSP source code */
static R_MOD0_DEFAULT_PARENTS: [&'static str; 2] = ["osc32k", "osc24M"];
static ir_clk: ccu_mp = SUNXI_CCU_MP_WITH_MUX_GATE!("ir", R_MOD0_DEFAULT_PARENTS.as_ptr(), 0x1c0, 0, 5, 8, 2, 24, 1, BIT!(31), 0);

/* BSP didn't use the 1-wire function at all now, and the information about
 * this mod clock is guessed from the IR mod clock above. */
static w1_clk: ccu_mp = SUNXI_CCU_MP_WITH_MUX_GATE!("w1", R_MOD0_DEFAULT_PARENTS.as_ptr(), 0x1e0, 0, 5, 8, 2, 24, 1, BIT!(31), 0);

static mut sun50i_h6_r_ccu_clks: [*mut ccu_common; 14] = [
    unsafe { &mut ar100_clk.common }, &r_apb1_clk.common, unsafe { &mut r_apb2_clk.common },
    &r_apb1_timer_clk.common, &r_apb1_twd_clk.common, &r_apb1_pwm_clk.common,
    &r_apb2_uart_clk.common, &r_apb2_i2c_clk.common, &r_apb2_rsb_clk.common,
    &r_apb1_ir_clk.common, &r_apb1_w1_clk.common, &r_apb1_rtc_clk.common,
    &ir_clk.common, &w1_clk.common,
];

static sun50i_h6_r_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data {
    hws: [
        [CLK_AR100] = unsafe { &ar100_clk.common.hw }, [CLK_R_AHB] = &r_ahb_clk.hw,
        [CLK_R_APB1] = &r_apb1_clk.common.hw, [CLK_R_APB2] = unsafe { &r_apb2_clk.common.hw },
        [CLK_R_APB1_TIMER] = &r_apb1_timer_clk.common.hw, [CLK_R_APB1_TWD] = &r_apb1_twd_clk.common.hw,
        [CLK_R_APB1_PWM] = &r_apb1_pwm_clk.common.hw, [CLK_R_APB2_UART] = &r_apb2_uart_clk.common.hw,
        [CLK_R_APB2_I2C] = &r_apb2_i2c_clk.common.hw, [CLK_R_APB2_RSB] = &r_apb2_rsb_clk.common.hw,
        [CLK_R_APB1_IR] = &r_apb1_ir_clk.common.hw, [CLK_R_APB1_W1] = &r_apb1_w1_clk.common.hw,
        [CLK_R_APB1_RTC] = &r_apb1_rtc_clk.common.hw, [CLK_IR] = &ir_clk.common.hw, [CLK_W1] = &w1_clk.common.hw,
    ], num: CLK_NUMBER,
};

static sun50i_h616_r_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data {
    hws: [[CLK_R_AHB] = &r_ahb_clk.hw, [CLK_R_APB1] = &r_apb1_clk.common.hw,
        [CLK_R_APB2] = unsafe { &r_apb2_clk.common.hw }, [CLK_R_APB1_TWD] = &r_apb1_twd_clk.common.hw,
        [CLK_R_APB2_I2C] = &r_apb2_i2c_clk.common.hw, [CLK_R_APB2_RSB] = &r_apb2_rsb_clk.common.hw,
        [CLK_R_APB1_IR] = &r_apb1_ir_clk.common.hw, [CLK_R_APB1_RTC] = &r_apb1_rtc_clk.common.hw,
        [CLK_IR] = &ir_clk.common.hw], num: CLK_NUMBER,
};

static sun50i_h6_r_ccu_resets: [ccu_reset_map; 8] = [
    [RST_R_APB1_TIMER] = ccu_reset_map { reg: 0x11c, bit: BIT!(16) }, [RST_R_APB1_TWD] = ccu_reset_map { reg: 0x12c, bit: BIT!(16) },
    [RST_R_APB1_PWM] = ccu_reset_map { reg: 0x13c, bit: BIT!(16) }, [RST_R_APB2_UART] = ccu_reset_map { reg: 0x18c, bit: BIT!(16) },
    [RST_R_APB2_I2C] = ccu_reset_map { reg: 0x19c, bit: BIT!(16) }, [RST_R_APB2_RSB] = ccu_reset_map { reg: 0x1bc, bit: BIT!(16) },
    [RST_R_APB1_IR] = ccu_reset_map { reg: 0x1cc, bit: BIT!(16) }, [RST_R_APB1_W1] = ccu_reset_map { reg: 0x1ec, bit: BIT!(16) },
];
static sun50i_h616_r_ccu_resets: [ccu_reset_map; 4] = [
    [RST_R_APB1_TWD] = ccu_reset_map { reg: 0x12c, bit: BIT!(16) }, [RST_R_APB2_I2C] = ccu_reset_map { reg: 0x19c, bit: BIT!(16) },
    [RST_R_APB2_RSB] = ccu_reset_map { reg: 0x1bc, bit: BIT!(16) }, [RST_R_APB1_IR] = ccu_reset_map { reg: 0x1cc, bit: BIT!(16) },
];

static sun50i_h6_r_ccu_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: sun50i_h6_r_ccu_clks.as_ptr(), num_ccu_clks: ARRAY_SIZE!(sun50i_h6_r_ccu_clks), hw_clks: &sun50i_h6_r_hw_clks, resets: sun50i_h6_r_ccu_resets.as_ptr(), num_resets: ARRAY_SIZE!(sun50i_h6_r_ccu_resets) };
static sun50i_h616_r_ccu_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: sun50i_h6_r_ccu_clks.as_ptr(), num_ccu_clks: ARRAY_SIZE!(sun50i_h6_r_ccu_clks), hw_clks: &sun50i_h616_r_hw_clks, resets: sun50i_h616_r_ccu_resets.as_ptr(), num_resets: ARRAY_SIZE!(sun50i_h616_r_ccu_resets) };

unsafe fn sun50i_h6_r_ccu_probe(pdev: *mut platform_device) -> i32 {
    let desc = of_device_get_match_data(&(*pdev).dev);
    if desc.is_null() { return -EINVAL; }
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) { return PTR_ERR!(reg); }
    devm_sunxi_ccu_probe(&(*pdev).dev, reg, desc)
}

static sun50i_h6_r_ccu_ids: [of_device_id; 3] = [
    of_device_id { compatible: "allwinner,sun50i-h6-r-ccu", data: &sun50i_h6_r_ccu_desc },
    of_device_id { compatible: "allwinner,sun50i-h616-r-ccu", data: &sun50i_h616_r_ccu_desc },
    of_device_id::default(),
];

static mut sun50i_h6_r_ccu_driver: platform_driver = platform_driver {
    probe: Some(sun50i_h6_r_ccu_probe),
    driver: driver { name: "sun50i-h6-r-ccu", suppress_bind_attrs: true, of_match_table: sun50i_h6_r_ccu_ids.as_ptr() },
};

module_platform_driver!(sun50i_h6_r_ccu_driver);
// MODULE_IMPORT_NS("SUNXI_CCU");
// MODULE_DESCRIPTION("Support for the Allwinner H6 and H616 PRCM CCU");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
