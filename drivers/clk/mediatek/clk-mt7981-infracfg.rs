// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
 * Author: Jianhui Zhao <zhaojh329@gmail.com>
 * Author: Daniel Golle <daniel@makrotopia.org>
 */

// C dependencies: linux clock/provider, platform-device, clk-mtk, clk-gate,
// clk-mux, and the MT7981 clock bindings are supplied by other translation units.

static DEFINE_SPINLOCK!(mt7981_clk_lock);

static infra_divs: [mtk_fixed_factor; 1] = [
    FACTOR!(CLK_INFRA_66M_MCK, "infra_66m_mck", "sysaxi_sel", 1, 2),
];

static infra_uart_parent: [&str; 2] = ["csw_f26m_sel", "uart_sel"];
static infra_spi0_parents: [&str; 2] = ["i2c_sel", "spi_sel"];
static infra_spi1_parents: [&str; 2] = ["i2c_sel", "spim_mst_sel"];
static infra_pwm1_parents: [&str; 1] = ["pwm_sel"];
static infra_pwm_bsel_parents: [&str; 4] = [
    "cb_rtc_32p7k", "csw_f26m_sel", "infra_66m_mck", "pwm_sel",
];
static infra_pcie_parents: [&str; 4] = [
    "cb_rtc_32p7k", "csw_f26m_sel", "cb_cksq_40m", "pextp_tl_ck_sel",
];

static infra_muxes: [mtk_mux; 11] = [
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_UART0_SEL, "infra_uart0_sel", infra_uart_parent, 0x0018, 0x0010, 0x0014, 0, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_UART1_SEL, "infra_uart1_sel", infra_uart_parent, 0x0018, 0x0010, 0x0014, 1, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_UART2_SEL, "infra_uart2_sel", infra_uart_parent, 0x0018, 0x0010, 0x0014, 2, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_SPI0_SEL, "infra_spi0_sel", infra_spi0_parents, 0x0018, 0x0010, 0x0014, 4, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_SPI1_SEL, "infra_spi1_sel", infra_spi1_parents, 0x0018, 0x0010, 0x0014, 5, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_SPI2_SEL, "infra_spi2_sel", infra_spi0_parents, 0x0018, 0x0010, 0x0014, 6, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_PWM1_SEL, "infra_pwm1_sel", infra_pwm1_parents, 0x0018, 0x0010, 0x0014, 9, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_PWM2_SEL, "infra_pwm2_sel", infra_pwm1_parents, 0x0018, 0x0010, 0x0014, 11, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_PWM3_SEL, "infra_pwm3_sel", infra_pwm1_parents, 0x0018, 0x0010, 0x0014, 15, 1, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_PWM_BSEL, "infra_pwm_bsel", infra_pwm_bsel_parents, 0x0018, 0x0010, 0x0014, 13, 2, -1, -1, -1),
    MUX_GATE_CLR_SET_UPD!(CLK_INFRA_PCIE_SEL, "infra_pcie_sel", infra_pcie_parents, 0x0028, 0x0020, 0x0024, 0, 2, -1, -1, -1),
];

static infra0_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x40, clr_ofs: 0x44, sta_ofs: 0x48 };
static infra1_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x50, clr_ofs: 0x54, sta_ofs: 0x58 };
static infra2_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x60, clr_ofs: 0x64, sta_ofs: 0x68 };

macro_rules! GATE_INFRA0 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &infra0_cg_regs, shift: $shift, ops: &mtk_clk_gate_ops_setclr } }; }
macro_rules! GATE_INFRA1 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &infra1_cg_regs, shift: $shift, ops: &mtk_clk_gate_ops_setclr } }; }
macro_rules! GATE_INFRA2 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &infra2_cg_regs, shift: $shift, ops: &mtk_clk_gate_ops_setclr } }; }

static infra_clks: [mtk_gate; 49] = [
    GATE_INFRA0!(CLK_INFRA_GPT_STA, "infra_gpt_sta", "infra_66m_mck", 0), GATE_INFRA0!(CLK_INFRA_PWM_HCK, "infra_pwm_hck", "infra_66m_mck", 1), GATE_INFRA0!(CLK_INFRA_PWM_STA, "infra_pwm_sta", "infra_pwm_bsel", 2), GATE_INFRA0!(CLK_INFRA_PWM1_CK, "infra_pwm1", "infra_pwm1_sel", 3), GATE_INFRA0!(CLK_INFRA_PWM2_CK, "infra_pwm2", "infra_pwm2_sel", 4), GATE_INFRA0!(CLK_INFRA_CQ_DMA_CK, "infra_cq_dma", "sysaxi", 6), GATE_INFRA0!(CLK_INFRA_AUD_BUS_CK, "infra_aud_bus", "sysaxi", 8), GATE_INFRA0!(CLK_INFRA_AUD_26M_CK, "infra_aud_26m", "csw_f26m_sel", 9), GATE_INFRA0!(CLK_INFRA_AUD_L_CK, "infra_aud_l", "aud_l", 10), GATE_INFRA0!(CLK_INFRA_AUD_AUD_CK, "infra_aud_aud", "a1sys", 11), GATE_INFRA0!(CLK_INFRA_AUD_EG2_CK, "infra_aud_eg2", "a_tuner", 13), GATE_INFRA0!(CLK_INFRA_DRAMC_26M_CK, "infra_dramc_26m", "csw_f26m_sel", 14), GATE_INFRA0!(CLK_INFRA_DBG_CK, "infra_dbg", "infra_66m_mck", 15), GATE_INFRA0!(CLK_INFRA_AP_DMA_CK, "infra_ap_dma", "infra_66m_mck", 16), GATE_INFRA0!(CLK_INFRA_SEJ_CK, "infra_sej", "infra_66m_mck", 24), GATE_INFRA0!(CLK_INFRA_SEJ_13M_CK, "infra_sej_13m", "csw_f26m_sel", 25), GATE_INFRA0!(CLK_INFRA_PWM3_CK, "infra_pwm3", "infra_pwm3_sel", 27),
    GATE_INFRA1!(CLK_INFRA_THERM_CK, "infra_therm", "csw_f26m_sel", 0), GATE_INFRA1!(CLK_INFRA_I2C0_CK, "infra_i2c0", "i2c_bck", 1), GATE_INFRA1!(CLK_INFRA_UART0_CK, "infra_uart0", "infra_uart0_sel", 2), GATE_INFRA1!(CLK_INFRA_UART1_CK, "infra_uart1", "infra_uart1_sel", 3), GATE_INFRA1!(CLK_INFRA_UART2_CK, "infra_uart2", "infra_uart2_sel", 4), GATE_INFRA1!(CLK_INFRA_SPI2_CK, "infra_spi2", "infra_spi2_sel", 6), GATE_INFRA1!(CLK_INFRA_SPI2_HCK_CK, "infra_spi2_hck", "infra_66m_mck", 7), GATE_INFRA1!(CLK_INFRA_NFI1_CK, "infra_nfi1", "nfi1x", 8), GATE_INFRA1!(CLK_INFRA_SPINFI1_CK, "infra_spinfi1", "spinfi_bck", 9), GATE_INFRA1!(CLK_INFRA_NFI_HCK_CK, "infra_nfi_hck", "infra_66m_mck", 10), GATE_INFRA1!(CLK_INFRA_SPI0_CK, "infra_spi0", "infra_spi0_sel", 11), GATE_INFRA1!(CLK_INFRA_SPI1_CK, "infra_spi1", "infra_spi1_sel", 12), GATE_INFRA1!(CLK_INFRA_SPI0_HCK_CK, "infra_spi0_hck", "infra_66m_mck", 13), GATE_INFRA1!(CLK_INFRA_SPI1_HCK_CK, "infra_spi1_hck", "infra_66m_mck", 14), GATE_INFRA1!(CLK_INFRA_FRTC_CK, "infra_frtc", "cb_rtc_32k", 15), GATE_INFRA1!(CLK_INFRA_MSDC_CK, "infra_msdc", "emmc_400m", 16), GATE_INFRA1!(CLK_INFRA_MSDC_HCK_CK, "infra_msdc_hck", "emmc_208m", 17), GATE_INFRA1!(CLK_INFRA_MSDC_133M_CK, "infra_msdc_133m", "sysaxi", 18), GATE_INFRA1!(CLK_INFRA_MSDC_66M_CK, "infra_msdc_66m", "sysaxi", 19), GATE_INFRA1!(CLK_INFRA_ADC_26M_CK, "infra_adc_26m", "infra_adc_frc", 20), GATE_INFRA1!(CLK_INFRA_ADC_FRC_CK, "infra_adc_frc", "csw_f26m", 21), GATE_INFRA1!(CLK_INFRA_FBIST2FPC_CK, "infra_fbist2fpc", "nfi1x", 23), GATE_INFRA1!(CLK_INFRA_I2C_MCK_CK, "infra_i2c_mck", "sysaxi", 25), GATE_INFRA1!(CLK_INFRA_I2C_PCK_CK, "infra_i2c_pck", "infra_66m_mck", 26),
    GATE_INFRA2!(CLK_INFRA_IUSB_133_CK, "infra_iusb_133", "sysaxi", 0), GATE_INFRA2!(CLK_INFRA_IUSB_66M_CK, "infra_iusb_66m", "sysaxi", 1), GATE_INFRA2!(CLK_INFRA_IUSB_SYS_CK, "infra_iusb_sys", "u2u3_sys", 2), GATE_INFRA2!(CLK_INFRA_IUSB_CK, "infra_iusb", "u2u3_ref", 3), GATE_INFRA2!(CLK_INFRA_IPCIE_CK, "infra_ipcie", "pextp_tl", 12), GATE_INFRA2!(CLK_INFRA_IPCIE_PIPE_CK, "infra_ipcie_pipe", "cb_cksq_40m", 13), GATE_INFRA2!(CLK_INFRA_IPCIER_CK, "infra_ipcier", "csw_f26m", 14), GATE_INFRA2!(CLK_INFRA_IPCIEB_CK, "infra_ipcieb", "sysaxi", 15),
];

static infracfg_desc: mtk_clk_desc = mtk_clk_desc {
    factor_clks: infra_divs, num_factor_clks: ARRAY_SIZE!(infra_divs),
    mux_clks: infra_muxes, num_mux_clks: ARRAY_SIZE!(infra_muxes),
    clks: infra_clks, num_clks: ARRAY_SIZE!(infra_clks), clk_lock: &mt7981_clk_lock,
};

static of_match_clk_mt7981_infracfg: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt7981-infracfg", data: &infracfg_desc },
    of_device_id { /* sentinel */ ..Default::default() },
];
MODULE_DEVICE_TABLE!(of, of_match_clk_mt7981_infracfg);

static mut clk_mt7981_infracfg_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe, remove: mtk_clk_simple_remove,
    driver: driver { name: "clk-mt7981-infracfg", of_match_table: of_match_clk_mt7981_infracfg },
};
module_platform_driver!(clk_mt7981_infracfg_drv);

MODULE_DESCRIPTION!("MediaTek MT7981 infracfg clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
