// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Icenowy Zheng <icenowy@aosc.xyz>
 */

// Linux clock-provider, module, OF, platform-device, and local CCU headers
// provide the types, constants, operations, and helper macros referenced below.

static AR100_PARENTS: [ClkParentData; 4] = [
    ClkParentData { fw_name: "losc" },
    ClkParentData { fw_name: "hosc" },
    ClkParentData { fw_name: "pll-periph" },
    ClkParentData { fw_name: "iosc" },
];

static AR100_PREDIVS: [CcuMuxVarPrediv; 1] = [
    CcuMuxVarPrediv { index: 2, shift: 8, width: 5 },
];

static mut AR100_CLK: CcuDiv = CcuDiv {
    div: sunxi_ccu_div_flags(4, 2, CLK_DIVIDER_POWER_OF_TWO),
    mux: CcuMux {
        shift: 16,
        width: 2,
        var_predivs: AR100_PREDIVS.as_ptr(),
        n_var_predivs: AR100_PREDIVS.len(),
        ..CcuMux::EMPTY
    },
    common: CcuCommon {
        reg: 0x00,
        features: CCU_FEATURE_VARIABLE_PREDIV,
        hw: HwInit::parents_data("ar100", AR100_PARENTS.as_ptr(), &ccu_div_ops, 0),
        ..CcuCommon::EMPTY
    },
};

static mut AHB0_CLK: ClkFixedFactorHw = clk_fixed_factor_hw!("ahb0", AR100_CLK.common.hw, 1, 1, 0);
static mut APB0_CLK: CcuM = sunxi_ccu_m!("apb0", "ahb0", 0x0c, 0, 2, 0);

/* Reused by all APB0 gates; it is mutable for the A83T variant. */
static mut APB0_GATE_PARENT: [*const ClkHw; 1] = [core::ptr::null()];
static mut APB0_PIO_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-pio", APB0_GATE_PARENT, 0x28, BIT(0), 0);
static mut APB0_IR_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-ir", APB0_GATE_PARENT, 0x28, BIT(1), 0);
static mut APB0_TIMER_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-timer", APB0_GATE_PARENT, 0x28, BIT(2), 0);
static mut APB0_RSB_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-rsb", APB0_GATE_PARENT, 0x28, BIT(3), 0);
static mut APB0_UART_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-uart", APB0_GATE_PARENT, 0x28, BIT(4), 0);
static mut APB0_I2C_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-i2c", APB0_GATE_PARENT, 0x28, BIT(6), 0);
static mut APB0_TWD_CLK: CcuGate = sunxi_ccu_gate_hws!("apb0-twd", APB0_GATE_PARENT, 0x28, BIT(7), 0);

static R_MOD0_DEFAULT_PARENTS: [&str; 2] = ["osc32k", "osc24M"];
static mut IR_CLK: CcuMp = sunxi_ccu_mp_with_mux_gate!("ir", R_MOD0_DEFAULT_PARENTS, 0x54, 0, 4, 16, 2, 24, 2, BIT(31), 0);

static A83T_R_MOD0_PARENTS: [ClkParentData; 2] = [
    ClkParentData { fw_name: "iosc" },
    ClkParentData { fw_name: "hosc" },
];
static A83T_IR_PREDIVS: [CcuMuxFixedPrediv; 1] = [CcuMuxFixedPrediv { index: 0, div: 16 }];
static mut A83T_IR_CLK: CcuMp = CcuMp {
    enable: BIT(31),
    m: sunxi_ccu_div(0, 4),
    p: sunxi_ccu_div(16, 2),
    mux: CcuMux { shift: 24, width: 2, fixed_predivs: A83T_IR_PREDIVS.as_ptr(), n_predivs: A83T_IR_PREDIVS.len(), ..CcuMux::EMPTY },
    common: CcuCommon { reg: 0x54, features: CCU_FEATURE_VARIABLE_PREDIV, hw: HwInit::parents_data("ir", A83T_R_MOD0_PARENTS.as_ptr(), &ccu_mp_ops, 0), ..CcuCommon::EMPTY },
};

static mut SUN8I_R_CCU_CLKS: [*mut CcuCommon; 11] = [
    unsafe { &raw mut AR100_CLK.common }, unsafe { &raw mut APB0_CLK.common },
    unsafe { &raw mut APB0_PIO_CLK.common }, unsafe { &raw mut APB0_IR_CLK.common },
    unsafe { &raw mut APB0_TIMER_CLK.common }, unsafe { &raw mut APB0_RSB_CLK.common },
    unsafe { &raw mut APB0_UART_CLK.common }, unsafe { &raw mut APB0_I2C_CLK.common },
    unsafe { &raw mut APB0_TWD_CLK.common }, unsafe { &raw mut IR_CLK.common },
    unsafe { &raw mut A83T_IR_CLK.common },
];

static mut SUN8I_A83T_R_HW_CLKS: ClkHwOnecellData = hw_clks! {
    [CLK_AR100] = AR100_CLK.common.hw, [CLK_AHB0] = AHB0_CLK.hw, [CLK_APB0] = APB0_CLK.common.hw,
    [CLK_APB0_PIO] = APB0_PIO_CLK.common.hw, [CLK_APB0_IR] = APB0_IR_CLK.common.hw,
    [CLK_APB0_TIMER] = APB0_TIMER_CLK.common.hw, [CLK_APB0_RSB] = APB0_RSB_CLK.common.hw,
    [CLK_APB0_UART] = APB0_UART_CLK.common.hw, [CLK_APB0_I2C] = APB0_I2C_CLK.common.hw,
    [CLK_APB0_TWD] = APB0_TWD_CLK.common.hw, [CLK_IR] = A83T_IR_CLK.common.hw; num = CLK_NUMBER
};
static mut SUN8I_H3_R_HW_CLKS: ClkHwOnecellData = hw_clks! {
    [CLK_AR100] = AR100_CLK.common.hw, [CLK_AHB0] = AHB0_CLK.hw, [CLK_APB0] = APB0_CLK.common.hw,
    [CLK_APB0_PIO] = APB0_PIO_CLK.common.hw, [CLK_APB0_IR] = APB0_IR_CLK.common.hw,
    [CLK_APB0_TIMER] = APB0_TIMER_CLK.common.hw, [CLK_APB0_UART] = APB0_UART_CLK.common.hw,
    [CLK_APB0_I2C] = APB0_I2C_CLK.common.hw, [CLK_APB0_TWD] = APB0_TWD_CLK.common.hw,
    [CLK_IR] = IR_CLK.common.hw; num = CLK_NUMBER
};
static mut SUN50I_A64_R_HW_CLKS: ClkHwOnecellData = hw_clks! {
    [CLK_AR100] = AR100_CLK.common.hw, [CLK_AHB0] = AHB0_CLK.hw, [CLK_APB0] = APB0_CLK.common.hw,
    [CLK_APB0_PIO] = APB0_PIO_CLK.common.hw, [CLK_APB0_IR] = APB0_IR_CLK.common.hw,
    [CLK_APB0_TIMER] = APB0_TIMER_CLK.common.hw, [CLK_APB0_RSB] = APB0_RSB_CLK.common.hw,
    [CLK_APB0_UART] = APB0_UART_CLK.common.hw, [CLK_APB0_I2C] = APB0_I2C_CLK.common.hw,
    [CLK_APB0_TWD] = APB0_TWD_CLK.common.hw, [CLK_IR] = IR_CLK.common.hw; num = CLK_NUMBER
};

static SUN8I_A83T_R_CCU_RESETS: [CcuResetMap; 7] = reset_map! {
    [RST_APB0_IR] = (0xb0, BIT(1)), [RST_APB0_TIMER] = (0xb0, BIT(2)), [RST_APB0_RSB] = (0xb0, BIT(3)),
    [RST_APB0_UART] = (0xb0, BIT(4)), [RST_APB0_I2C] = (0xb0, BIT(6))
};
static SUN8I_H3_R_CCU_RESETS: [CcuResetMap; 7] = reset_map! {
    [RST_APB0_IR] = (0xb0, BIT(1)), [RST_APB0_TIMER] = (0xb0, BIT(2)), [RST_APB0_UART] = (0xb0, BIT(4)), [RST_APB0_I2C] = (0xb0, BIT(6))
};
static SUN50I_A64_R_CCU_RESETS: [CcuResetMap; 7] = reset_map! {
    [RST_APB0_IR] = (0xb0, BIT(1)), [RST_APB0_TIMER] = (0xb0, BIT(2)), [RST_APB0_RSB] = (0xb0, BIT(3)), [RST_APB0_UART] = (0xb0, BIT(4)), [RST_APB0_I2C] = (0xb0, BIT(6))
};

static SUN8I_A83T_R_CCU_DESC: SunxiCcuDesc = sunxi_ccu_desc!(SUN8I_A83T_R_HW_CLKS, SUN8I_A83T_R_CCU_RESETS);
static SUN8I_H3_R_CCU_DESC: SunxiCcuDesc = sunxi_ccu_desc!(SUN8I_H3_R_HW_CLKS, SUN8I_H3_R_CCU_RESETS);
static SUN50I_A64_R_CCU_DESC: SunxiCcuDesc = sunxi_ccu_desc!(SUN50I_A64_R_HW_CLKS, SUN50I_A64_R_CCU_RESETS);

unsafe extern "C" {
    fn of_device_get_match_data(dev: *const Device) -> *const SunxiCcuDesc;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: i32) -> *mut core::ffi::c_void;
    fn devm_sunxi_ccu_probe(dev: *mut Device, reg: *mut core::ffi::c_void, desc: *const SunxiCcuDesc) -> i32;
}

unsafe fn sun8i_r_ccu_probe(pdev: *mut PlatformDevice) -> i32 {
    let desc = of_device_get_match_data((*pdev).dev());
    if desc.is_null() { return -EINVAL; }
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if is_err(reg) { return ptr_err(reg); }
    devm_sunxi_ccu_probe((*pdev).dev_mut(), reg, desc)
}

static SUN8I_R_CCU_IDS: [OfDeviceId; 4] = [
    OfDeviceId { compatible: "allwinner,sun8i-a83t-r-ccu", data: &SUN8I_A83T_R_CCU_DESC },
    OfDeviceId { compatible: "allwinner,sun8i-h3-r-ccu", data: &SUN8I_H3_R_CCU_DESC },
    OfDeviceId { compatible: "allwinner,sun50i-a64-r-ccu", data: &SUN50I_A64_R_CCU_DESC },
    OfDeviceId::EMPTY,
];

static mut SUN8I_R_CCU_DRIVER: PlatformDriver = platform_driver!(
    probe = sun8i_r_ccu_probe,
    name = "sun8i-r-ccu",
    suppress_bind_attrs = true,
    of_match_table = SUN8I_R_CCU_IDS,
);

module_platform_driver!(SUN8I_R_CCU_DRIVER);
module_import_ns!("SUNXI_CCU");
module_description!("Support for Allwinner SoCs' PRCM CCUs");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
