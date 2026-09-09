// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 Yangtao Li <frank@allwinnertech.com>
 */

// External Linux/CCU declarations and macros are supplied by the corresponding
// Rust bindings and are intentionally not redefined here.

static CPUS_R_APB2_PARENTS: [&'static str; 4] = ["dcxo24M", "osc32k", "iosc", "pll-periph0"];
static CPUS_R_APB2_PREDIVS: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv {
    index: 3,
    shift: 0,
    width: 5,
}];

static mut R_CPUS_CLK: ccu_div = ccu_div {
    div: SUNXI_CCU_DIV_FLAGS!(8, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: ccu_mux {
        shift: 24,
        width: 2,
        var_predivs: CPUS_R_APB2_PREDIVS.as_ptr(),
        n_var_predivs: CPUS_R_APB2_PREDIVS.len(),
    },
    common: ccu_common {
        reg: 0x000,
        features: CCU_FEATURE_VARIABLE_PREDIV,
        hw: clk_hw_init_parents!("cpus", CPUS_R_APB2_PARENTS.as_ptr(), &ccu_div_ops, 0),
    },
};

static R_AHB_CLK: clk_fixed_factor_hw = CLK_FIXED_FACTOR_HW!("r-ahb", unsafe { &R_CPUS_CLK.common.hw }, 1, 1, 0);

static mut R_APB1_CLK: ccu_div = ccu_div {
    div: SUNXI_CCU_DIV!(0, 2),
    common: ccu_common {
        reg: 0x00c,
        hw: clk_hw_init!("r-apb1", "r-ahb", &ccu_div_ops, 0),
        ..ccu_common::default()
    },
    ..ccu_div::default()
};

static mut R_APB2_CLK: ccu_div = ccu_div {
    div: SUNXI_CCU_DIV_FLAGS!(8, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: ccu_mux {
        shift: 24,
        width: 2,
        var_predivs: CPUS_R_APB2_PREDIVS.as_ptr(),
        n_var_predivs: CPUS_R_APB2_PREDIVS.len(),
    },
    common: ccu_common {
        reg: 0x010,
        features: CCU_FEATURE_VARIABLE_PREDIV,
        hw: clk_hw_init_parents!("r-apb2", CPUS_R_APB2_PARENTS.as_ptr(), &ccu_div_ops, 0),
    },
};

static CLK_PARENT_R_APB1: [clk_parent_data; 1] = [clk_parent_data {
    hw: unsafe { &R_APB1_CLK.common.hw },
}];
static CLK_PARENT_R_APB2: [clk_parent_data; 1] = [clk_parent_data {
    hw: unsafe { &R_APB2_CLK.common.hw },
}];

SUNXI_CCU_GATE_DATA!(R_APB1_TIMER_CLK, "r-apb1-timer", CLK_PARENT_R_APB1, 0x11c, BIT!(0), 0);
SUNXI_CCU_GATE_DATA!(R_APB1_TWD_CLK, "r-apb1-twd", CLK_PARENT_R_APB1, 0x12c, BIT!(0), 0);

static R_APB1_PWM_CLK_PARENTS: [&'static str; 3] = ["dcxo24M", "osc32k", "iosc"];
SUNXI_CCU_MUX!(R_APB1_PWM_CLK, "r-apb1-pwm", R_APB1_PWM_CLK_PARENTS, 0x130, 24, 2, 0);
SUNXI_CCU_GATE_DATA!(R_APB1_BUS_PWM_CLK, "r-apb1-bus-pwm", CLK_PARENT_R_APB1, 0x13c, BIT!(0), 0);
SUNXI_CCU_GATE_DATA!(R_APB1_PPU_CLK, "r-apb1-ppu", CLK_PARENT_R_APB1, 0x17c, BIT!(0), 0);
SUNXI_CCU_GATE_DATA!(R_APB2_UART_CLK, "r-apb2-uart", CLK_PARENT_R_APB2, 0x18c, BIT!(0), 0);
SUNXI_CCU_GATE_DATA!(R_APB2_I2C0_CLK, "r-apb2-i2c0", CLK_PARENT_R_APB2, 0x19c, BIT!(0), 0);
SUNXI_CCU_GATE_DATA!(R_APB2_I2C1_CLK, "r-apb2-i2c1", CLK_PARENT_R_APB2, 0x19c, BIT!(1), 0);

static R_APB1_IR_RX_PARENTS: [&'static str; 2] = ["osc32k", "dcxo24M"];
SUNXI_CCU_MP_WITH_MUX_GATE!(R_APB1_IR_RX_CLK, "r-apb1-ir-rx", R_APB1_IR_RX_PARENTS,
    0x1c0, 0, 5, /* M */ 8, 2, /* P */ 24, 1, /* mux */ BIT!(31), /* gate */ 0);
SUNXI_CCU_GATE_DATA!(R_APB1_BUS_IR_RX_CLK, "r-apb1-bus-ir-rx", CLK_PARENT_R_APB1, 0x1cc, BIT!(0), 0);
SUNXI_CCU_GATE!(R_AHB_BUS_RTC_CLK, "r-ahb-rtc", "r-ahb", 0x20c, BIT!(0), 0);

static mut SUN50I_A100_R_CCU_CLKS: [*mut ccu_common; 14] = [
    unsafe { &mut R_CPUS_CLK.common }, unsafe { &mut R_APB1_CLK.common }, unsafe { &mut R_APB2_CLK.common },
    unsafe { &mut R_APB1_TIMER_CLK.common }, unsafe { &mut R_APB1_TWD_CLK.common }, unsafe { &mut R_APB1_PWM_CLK.common },
    unsafe { &mut R_APB1_BUS_PWM_CLK.common }, unsafe { &mut R_APB1_PPU_CLK.common }, unsafe { &mut R_APB2_UART_CLK.common },
    unsafe { &mut R_APB2_I2C0_CLK.common }, unsafe { &mut R_APB2_I2C1_CLK.common }, unsafe { &mut R_APB1_IR_RX_CLK.common },
    unsafe { &mut R_APB1_BUS_IR_RX_CLK.common }, unsafe { &mut R_AHB_BUS_RTC_CLK.common },
];

static mut SUN50I_A100_R_HW_CLKS: clk_hw_onecell_data = clk_hw_onecell_data {
    hws: [
        [CLK_R_CPUS] = unsafe { &R_CPUS_CLK.common.hw }, [CLK_R_AHB] = &R_AHB_CLK.hw,
        [CLK_R_APB1] = unsafe { &R_APB1_CLK.common.hw }, [CLK_R_APB2] = unsafe { &R_APB2_CLK.common.hw },
        [CLK_R_APB1_TIMER] = &R_APB1_TIMER_CLK.common.hw, [CLK_R_APB1_TWD] = &R_APB1_TWD_CLK.common.hw,
        [CLK_R_APB1_PWM] = &R_APB1_PWM_CLK.common.hw, [CLK_R_APB1_BUS_PWM] = &R_APB1_BUS_PWM_CLK.common.hw,
        [CLK_R_APB1_PPU] = &R_APB1_PPU_CLK.common.hw, [CLK_R_APB2_UART] = &R_APB2_UART_CLK.common.hw,
        [CLK_R_APB2_I2C0] = &R_APB2_I2C0_CLK.common.hw, [CLK_R_APB2_I2C1] = &R_APB2_I2C1_CLK.common.hw,
        [CLK_R_APB1_IR] = &R_APB1_IR_RX_CLK.common.hw, [CLK_R_APB1_BUS_IR] = &R_APB1_BUS_IR_RX_CLK.common.hw,
        [CLK_R_AHB_BUS_RTC] = &R_AHB_BUS_RTC_CLK.common.hw,
    ],
    num: CLK_NUMBER,
};

static SUN50I_A100_R_CCU_RESETS: [ccu_reset_map; 8] = [
    [RST_R_APB1_TIMER] = ccu_reset_map { reg: 0x11c, bit: BIT!(16) },
    [RST_R_APB1_BUS_PWM] = ccu_reset_map { reg: 0x13c, bit: BIT!(16) },
    [RST_R_APB1_PPU] = ccu_reset_map { reg: 0x17c, bit: BIT!(16) },
    [RST_R_APB2_UART] = ccu_reset_map { reg: 0x18c, bit: BIT!(16) },
    [RST_R_APB2_I2C0] = ccu_reset_map { reg: 0x19c, bit: BIT!(16) },
    [RST_R_APB2_I2C1] = ccu_reset_map { reg: 0x19c, bit: BIT!(17) },
    [RST_R_APB1_BUS_IR] = ccu_reset_map { reg: 0x1cc, bit: BIT!(16) },
    [RST_R_AHB_BUS_RTC] = ccu_reset_map { reg: 0x20c, bit: BIT!(16) },
];

static SUN50I_A100_R_CCU_DESC: sunxi_ccu_desc = sunxi_ccu_desc {
    ccu_clks: SUN50I_A100_R_CCU_CLKS.as_ptr(),
    num_ccu_clks: SUN50I_A100_R_CCU_CLKS.len(),
    hw_clks: &SUN50I_A100_R_HW_CLKS,
    resets: SUN50I_A100_R_CCU_RESETS.as_ptr(),
    num_resets: SUN50I_A100_R_CCU_RESETS.len(),
};

unsafe fn sun50i_a100_r_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg: *mut core::ffi::c_void = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) { return PTR_ERR!(reg); }
    devm_sunxi_ccu_probe((*pdev).dev.as_ref(), reg, &SUN50I_A100_R_CCU_DESC)
}

static SUN50I_A100_R_CCU_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "allwinner,sun50i-a100-r-ccu" },
    of_device_id { ..of_device_id::default() },
];
MODULE_DEVICE_TABLE!(of, SUN50I_A100_R_CCU_IDS);

static mut SUN50I_A100_R_CCU_DRIVER: platform_driver = platform_driver {
    probe: Some(sun50i_a100_r_ccu_probe),
    driver: device_driver {
        name: "sun50i-a100-r-ccu",
        suppress_bind_attrs: true,
        of_match_table: SUN50I_A100_R_CCU_IDS.as_ptr(),
    },
};
module_platform_driver!(SUN50I_A100_R_CCU_DRIVER);

MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A100 PRCM CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
