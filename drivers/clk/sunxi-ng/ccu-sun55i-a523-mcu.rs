// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Chen-Yu Tsai <wens@csie.org>
 *
 * Based on the A523 CCU driver:
 *   Copyright (C) 2023-2024 Arm Ltd.
 */

// External Linux, device-tree, and sunxi CCU declarations are supplied by
// other translation units.

static OSC24M: [ClkParentData; 1] = [ClkParentData { fw_name: "hosc" }];
static AHB: [ClkParentData; 1] = [ClkParentData { fw_name: "r-ahb" }];
static APB: [ClkParentData; 1] = [ClkParentData { fw_name: "r-apb0" }];

const SUN55I_A523_PLL_AUDIO1_REG: usize = 0x00c;
static mut PLL_AUDIO1_SDM_TABLE: [CcuSdmSetting; 3] = [
    CcuSdmSetting { rate: 2167603200, pattern: 0xa000a234, m: 1, n: 90 },
    CcuSdmSetting { rate: 2359296000, pattern: 0xa0009ba6, m: 1, n: 98 },
    CcuSdmSetting { rate: 1806336000, pattern: 0xa000872b, m: 1, n: 75 },
];

static mut PLL_AUDIO1_CLK: CcuNm = CcuNm {
    enable: BIT(27), lock: BIT(28),
    n: _SUNXI_CCU_MULT_MIN(8, 8, 11), m: _SUNXI_CCU_DIV(1, 1),
    sdm: _SUNXI_CCU_SDM(PLL_AUDIO1_SDM_TABLE, BIT(24), 0x010, BIT(31)),
    min_rate: 180000000u32, max_rate: 3500000000u32,
    common: CcuCommon { reg: 0x00c, features: CCU_FEATURE_SIGMA_DELTA_MOD,
        hw: CLK_HW_INIT_PARENTS_DATA("pll-audio1", OSC24M, &ccu_nm_ops,
                                    CLK_SET_RATE_GATE) },
};

// /2 and /5 dividers are programmable, but the BSP values are used here.
static PLL_AUDIO1_DIV_PARENTS: [*const ClkHw; 1] = [unsafe { &PLL_AUDIO1_CLK.common.hw }];
CLK_FIXED_FACTOR_HWS!(pll_audio1_div2_clk, "pll-audio1-div2", PLL_AUDIO1_DIV_PARENTS, 2, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio1_div5_clk, "pll-audio1-div5", PLL_AUDIO1_DIV_PARENTS, 5, 1, CLK_SET_RATE_PARENT);

SUNXI_CCU_M_WITH_GATE!(audio_out_clk, "audio-out", "pll-audio1-div2", 0x01c, 0, 5, BIT(31), CLK_SET_RATE_PARENT);

static DSP_PARENTS: [ClkParentData; 6] = [
    ClkParentData { fw_name: "hosc" }, ClkParentData { fw_name: "losc" },
    ClkParentData { fw_name: "iosc" },
    // Order follows BSP code; it is opposite in the manual.
    ClkParentData { hw: unsafe { &pll_audio1_div5_clk.hw } },
    ClkParentData { hw: unsafe { &pll_audio1_div2_clk.hw } },
    ClkParentData { fw_name: "dsp" },
];
SUNXI_CCU_M_DATA_WITH_MUX_GATE!(dsp_clk, "mcu-dsp", DSP_PARENTS, 0x0020, 0, 5, 24, 3, BIT(31), 0);

static I2S_PARENTS: [ClkParentData; 3] = [
    ClkParentData { fw_name: "pll-audio0-4x" },
    ClkParentData { hw: unsafe { &pll_audio1_div2_clk.hw } },
    ClkParentData { hw: unsafe { &pll_audio1_div5_clk.hw } },
];
SUNXI_CCU_DUALDIV_MUX_GATE!(i2s0_clk, "i2s0", I2S_PARENTS, 0x02c, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_DUALDIV_MUX_GATE!(i2s1_clk, "i2s1", I2S_PARENTS, 0x030, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_DUALDIV_MUX_GATE!(i2s2_clk, "i2s2", I2S_PARENTS, 0x034, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_DUALDIV_MUX_GATE!(i2s3_clk, "i2s3", I2S_PARENTS, 0x038, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);

static I2S3_ASRC_PARENTS: [ClkParentData; 3] = [
    ClkParentData { fw_name: "pll-periph0-300m" },
    ClkParentData { hw: unsafe { &pll_audio1_div2_clk.hw } },
    ClkParentData { hw: unsafe { &pll_audio1_div5_clk.hw } },
];
SUNXI_CCU_DUALDIV_MUX_GATE!(i2s3_asrc_clk, "i2s3-asrc", I2S3_ASRC_PARENTS, 0x03c, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);

SUNXI_CCU_GATE_DATA!(bus_i2s0_clk, "bus-i2s0", APB, 0x040, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_i2s1_clk, "bus-i2s1", APB, 0x040, BIT(1), 0);
SUNXI_CCU_GATE_DATA!(bus_i2s2_clk, "bus-i2s2", APB, 0x040, BIT(2), 0);
SUNXI_CCU_GATE_DATA!(bus_i2s3_clk, "bus-i2s3", APB, 0x040, BIT(3), 0);

static AUDIO_PARENTS: [ClkParentData; 3] = I2S_PARENTS;
SUNXI_CCU_DUALDIV_MUX_GATE!(spdif_tx_clk, "spdif-tx", AUDIO_PARENTS, 0x044, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_DUALDIV_MUX_GATE!(spdif_rx_clk, "spdif-rx", I2S3_ASRC_PARENTS, 0x048, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE_DATA!(bus_spdif_clk, "bus-spdif", APB, 0x04c, BIT(0), 0);
SUNXI_CCU_DUALDIV_MUX_GATE!(dmic_clk, "dmic", AUDIO_PARENTS, 0x050, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE_DATA!(bus_dmic_clk, "bus-dmic", APB, 0x054, BIT(0), 0);
SUNXI_CCU_DUALDIV_MUX_GATE!(audio_dac_clk, "audio-dac", AUDIO_PARENTS, 0x058, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_DUALDIV_MUX_GATE!(audio_adc_clk, "audio-adc", AUDIO_PARENTS, 0x05c, 0, 5, 5, 5, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE_DATA!(bus_audio_codec_clk, "bus-audio-codec", APB, 0x060, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_dsp_msgbox_clk, "bus-dsp-msgbox", AHB, 0x068, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_dsp_cfg_clk, "bus-dsp-cfg", APB, 0x06c, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_npu_hclk, "bus-npu-hclk", AHB, 0x070, BIT(1), 0);
SUNXI_CCU_GATE_DATA!(bus_npu_aclk, "bus-npu-aclk", AHB, 0x070, BIT(2), 0);

static TIMER_PARENTS: [ClkParentData; 4] = [ClkParentData { fw_name: "hosc" }, ClkParentData { fw_name: "losc" }, ClkParentData { fw_name: "iosc" }, ClkParentData { fw_name: "r-ahb" }];
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer0_clk, "mcu-timer0", TIMER_PARENTS, 0x074, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer1_clk, "mcu-timer1", TIMER_PARENTS, 0x078, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer2_clk, "mcu-timer2", TIMER_PARENTS, 0x07c, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer3_clk, "mcu-timer3", TIMER_PARENTS, 0x080, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer4_clk, "mcu-timer4", TIMER_PARENTS, 0x084, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_P_DATA_WITH_MUX_GATE!(mcu_timer5_clk, "mcu-timer5", TIMER_PARENTS, 0x088, 1, 3, 4, 2, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_mcu_timer_clk, "bus-mcu-timer", AHB, 0x08c, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_mcu_dma_clk, "bus-mcu-dma", AHB, 0x104, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(tzma0_clk, "tzma0", AHB, 0x108, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(tzma1_clk, "tzma1", AHB, 0x10c, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_pubsram_clk, "bus-pubsram", AHB, 0x114, BIT(0), 0);
SUNXI_CCU_GATE_FW!(mbus_mcu_clk, "mbus-mcu", "mbus", 0x11c, BIT(1), 0);
SUNXI_CCU_GATE_HW!(mbus_mcu_dma_clk, "mbus-mcu-dma", unsafe { &mbus_mcu_clk.common.hw }, 0x11c, BIT(0), 0);

static RISCV_PWM_PARENTS: [ClkParentData; 3] = [ClkParentData { fw_name: "hosc" }, ClkParentData { fw_name: "losc" }, ClkParentData { fw_name: "iosc" }];
SUNXI_CCU_MUX_DATA_WITH_GATE!(riscv_clk, "riscv", RISCV_PWM_PARENTS, 0x120, 27, 3, BIT(31), 0);
SUNXI_CCU_GATE_DATA!(bus_riscv_cfg_clk, "bus-riscv-cfg", AHB, 0x124, BIT(0), 0);
SUNXI_CCU_GATE_DATA!(bus_riscv_msgbox_clk, "bus-riscv-msgbox", AHB, 0x128, BIT(0), 0);
SUNXI_CCU_MUX_DATA_WITH_GATE!(mcu_pwm0_clk, "mcu-pwm0", RISCV_PWM_PARENTS, 0x130, 24, 3, BIT(31), 0);
SUNXI_CCU_GATE_DATA!(bus_mcu_pwm0_clk, "bus-mcu-pwm0", APB, 0x134, BIT(0), 0);

static mut SUN55I_A523_MCU_CCU_CLKS: [*mut CcuCommon; 42] = [
    &mut PLL_AUDIO1_CLK.common, &mut audio_out_clk.common, &mut dsp_clk.common,
    &mut i2s0_clk.common, &mut i2s1_clk.common, &mut i2s2_clk.common, &mut i2s3_clk.common,
    &mut i2s3_asrc_clk.common, &mut bus_i2s0_clk.common, &mut bus_i2s1_clk.common,
    &mut bus_i2s2_clk.common, &mut bus_i2s3_clk.common, &mut spdif_tx_clk.common,
    &mut spdif_rx_clk.common, &mut bus_spdif_clk.common, &mut dmic_clk.common,
    &mut bus_dmic_clk.common, &mut audio_dac_clk.common, &mut audio_adc_clk.common,
    &mut bus_audio_codec_clk.common, &mut bus_dsp_msgbox_clk.common, &mut bus_dsp_cfg_clk.common,
    &mut bus_npu_aclk.common, &mut bus_npu_hclk.common, &mut mcu_timer0_clk.common,
    &mut mcu_timer1_clk.common, &mut mcu_timer2_clk.common, &mut mcu_timer3_clk.common,
    &mut mcu_timer4_clk.common, &mut mcu_timer5_clk.common, &mut bus_mcu_timer_clk.common,
    &mut bus_mcu_dma_clk.common, &mut tzma0_clk.common, &mut tzma1_clk.common,
    &mut bus_pubsram_clk.common, &mut mbus_mcu_dma_clk.common, &mut mbus_mcu_clk.common,
    &mut riscv_clk.common, &mut bus_riscv_cfg_clk.common, &mut bus_riscv_msgbox_clk.common,
    &mut mcu_pwm0_clk.common, &mut bus_mcu_pwm0_clk.common,
];

static mut SUN55I_A523_MCU_CCU_RESETS: [CcuResetMap; 19] = [
    [RST_BUS_MCU_I2S0] = CcuResetMap { reg: 0x0040, bit: BIT(16) },
    [RST_BUS_MCU_I2S1] = CcuResetMap { reg: 0x0040, bit: BIT(17) },
    [RST_BUS_MCU_I2S2] = CcuResetMap { reg: 0x0040, bit: BIT(18) },
    [RST_BUS_MCU_I2S3] = CcuResetMap { reg: 0x0040, bit: BIT(19) },
    [RST_BUS_MCU_SPDIF] = CcuResetMap { reg: 0x004c, bit: BIT(16) },
    [RST_BUS_MCU_DMIC] = CcuResetMap { reg: 0x0054, bit: BIT(16) },
    [RST_BUS_MCU_AUDIO_CODEC] = CcuResetMap { reg: 0x0060, bit: BIT(16) },
    [RST_BUS_MCU_DSP_MSGBOX] = CcuResetMap { reg: 0x0068, bit: BIT(16) },
    [RST_BUS_MCU_DSP_CFG] = CcuResetMap { reg: 0x006c, bit: BIT(16) },
    [RST_BUS_MCU_NPU] = CcuResetMap { reg: 0x0070, bit: BIT(16) },
    [RST_BUS_MCU_TIMER] = CcuResetMap { reg: 0x008c, bit: BIT(16) },
    [RST_BUS_MCU_DSP_DEBUG] = CcuResetMap { reg: 0x0100, bit: BIT(16) },
    [RST_BUS_MCU_DSP] = CcuResetMap { reg: 0x0100, bit: BIT(17) },
    [RST_BUS_MCU_DMA] = CcuResetMap { reg: 0x0104, bit: BIT(16) },
    [RST_BUS_MCU_PUBSRAM] = CcuResetMap { reg: 0x0114, bit: BIT(16) },
    [RST_BUS_MCU_RISCV_CFG] = CcuResetMap { reg: 0x0124, bit: BIT(16) },
    [RST_BUS_MCU_RISCV_DEBUG] = CcuResetMap { reg: 0x0124, bit: BIT(17) },
    [RST_BUS_MCU_RISCV_CORE] = CcuResetMap { reg: 0x0124, bit: BIT(18) },
    [RST_BUS_MCU_RISCV_MSGBOX] = CcuResetMap { reg: 0x0128, bit: BIT(16) },
    [RST_BUS_MCU_PWM0] = CcuResetMap { reg: 0x0134, bit: BIT(16) },
];

static SUN55I_A523_MCU_CCU_DESC: SunxiCcuDesc = SunxiCcuDesc {
    ccu_clks: SUN55I_A523_MCU_CCU_CLKS,
    num_ccu_clks: ARRAY_SIZE(SUN55I_A523_MCU_CCU_CLKS),
    hw_clks: &SUN55I_A523_MCU_HW_CLKS,
    resets: SUN55I_A523_MCU_CCU_RESETS,
    num_resets: ARRAY_SIZE(SUN55I_A523_MCU_CCU_RESETS),
};

unsafe fn sun55i_a523_mcu_ccu_probe(pdev: *mut PlatformDevice) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN55I_A523_PLL_AUDIO1_REG));
    val |= BIT(31) | BIT(30) | BIT(29);
    val &= !(GENMASK(22, 20) | GENMASK(18, 16));
    val |= (4 << 20) | (1 << 16);
    writel(val, reg.add(SUN55I_A523_PLL_AUDIO1_REG));
    let ret = devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &SUN55I_A523_MCU_CCU_DESC);
    if ret != 0 { return ret; }
    0
}

static SUN55I_A523_MCU_CCU_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "allwinner,sun55i-a523-mcu-ccu" }, OfDeviceId::empty(),
];
static mut SUN55I_A523_MCU_CCU_DRIVER: PlatformDriver = PlatformDriver {
    probe: sun55i_a523_mcu_ccu_probe,
    driver: Driver { name: "sun55i-a523-mcu-ccu", suppress_bind_attrs: true, of_match_table: SUN55I_A523_MCU_CCU_IDS },
};
module_platform_driver!(SUN55I_A523_MCU_CCU_DRIVER);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A523 MCU CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
