// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 MediaTek Inc. */
// Translated from clk-mt7986-topckgen.c. Linux headers/macros are external dependencies.

extern "C" {
    static mut mt7986_clk_lock: spinlock_t;
}

#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

static top_fixed_clks: [mtk_fixed_clk; 2] = [
    FIXED_CLK!(CLK_TOP_XTAL, "top_xtal", "clkxtal", 40000000),
    FIXED_CLK!(CLK_TOP_JTAG, "top_jtag", "clkxtal", 50000000),
];

static top_divs: [mtk_fixed_factor; 23] = [
    FACTOR!(CLK_TOP_XTAL_D2, "top_xtal_d2", "top_xtal", 1, 2),
    FACTOR!(CLK_TOP_RTC_32K, "top_rtc_32k", "top_xtal", 1, 1250),
    FACTOR!(CLK_TOP_RTC_32P7K, "top_rtc_32p7k", "top_xtal", 1, 1220),
    FACTOR!(CLK_TOP_MPLL_D2, "top_mpll_d2", "mpll", 1, 2),
    FACTOR!(CLK_TOP_MPLL_D4, "top_mpll_d4", "mpll", 1, 4),
    FACTOR!(CLK_TOP_MPLL_D8, "top_mpll_d8", "mpll", 1, 8),
    FACTOR!(CLK_TOP_MPLL_D8_D2, "top_mpll_d8_d2", "mpll", 1, 16),
    FACTOR!(CLK_TOP_MPLL_D3_D2, "top_mpll_d3_d2", "mpll", 1, 6),
    FACTOR!(CLK_TOP_MMPLL_D2, "top_mmpll_d2", "mmpll", 1, 2),
    FACTOR!(CLK_TOP_MMPLL_D4, "top_mmpll_d4", "mmpll", 1, 4),
    FACTOR!(CLK_TOP_MMPLL_D8, "top_mmpll_d8", "mmpll", 1, 8),
    FACTOR!(CLK_TOP_MMPLL_D8_D2, "top_mmpll_d8_d2", "mmpll", 1, 16),
    FACTOR!(CLK_TOP_MMPLL_D3_D8, "top_mmpll_d3_d8", "mmpll", 1, 24),
    FACTOR!(CLK_TOP_MMPLL_U2PHY, "top_mmpll_u2phy", "mmpll", 1, 30),
    FACTOR!(CLK_TOP_APLL2_D4, "top_apll2_d4", "apll2", 1, 4),
    FACTOR!(CLK_TOP_NET1PLL_D4, "top_net1pll_d4", "net1pll", 1, 4),
    FACTOR!(CLK_TOP_NET1PLL_D5, "top_net1pll_d5", "net1pll", 1, 5),
    FACTOR!(CLK_TOP_NET1PLL_D5_D2, "top_net1pll_d5_d2", "net1pll", 1, 10),
    FACTOR!(CLK_TOP_NET1PLL_D5_D4, "top_net1pll_d5_d4", "net1pll", 1, 20),
    FACTOR!(CLK_TOP_NET1PLL_D8_D2, "top_net1pll_d8_d2", "net1pll", 1, 16),
    FACTOR!(CLK_TOP_NET1PLL_D8_D4, "top_net1pll_d8_d4", "net1pll", 1, 32),
    FACTOR!(CLK_TOP_NET2PLL_D4, "top_net2pll_d4", "net2pll", 1, 4),
    FACTOR!(CLK_TOP_NET2PLL_D4_D2, "top_net2pll_d4_d2", "net2pll", 1, 8),
    FACTOR!(CLK_TOP_NET2PLL_D3_D2, "top_net2pll_d3_d2", "net2pll", 1, 2),
];

macro_rules! parents { ($($name:ident => [$($p:expr),* $(,)?]),* $(,)?) => { $(static $name: &'static [&'static str] = &[$($p),*];)* }; }
parents! {
 nfi1x_parents => ["top_xtal","top_mmpll_d8","top_net1pll_d8_d2","top_net2pll_d3_d2","top_mpll_d4","top_mmpll_d8_d2","top_wedmcupll_d5_d2","top_mpll_d8"],
 spinfi_parents => ["top_xtal_d2","top_xtal","top_net1pll_d5_d4","top_mpll_d4","top_mmpll_d8_d2","top_wedmcupll_d5_d2","top_mmpll_d3_d8","top_mpll_d8"],
 spi_parents => ["top_xtal","top_mpll_d2","top_mmpll_d8","top_net1pll_d8_d2","top_net2pll_d3_d2","top_net1pll_d5_d4","top_mpll_d4","top_wedmcupll_d5_d2"],
 uart_parents => ["top_xtal","top_mpll_d8","top_mpll_d8_d2"],
 pwm_parents => ["top_xtal","top_net1pll_d8_d2","top_net1pll_d5_d4","top_mpll_d4"],
 i2c_parents => ["top_xtal","top_net1pll_d5_d4","top_mpll_d4","top_net1pll_d8_d4"],
 pextp_tl_ck_parents => ["top_xtal","top_net1pll_d5_d4","top_net2pll_d4_d2","top_rtc_32k"],
 emmc_250m_parents => ["top_xtal","top_net1pll_d5_d2"], emmc_416m_parents => ["top_xtal","mpll"],
 f_26m_adc_parents => ["top_xtal","top_mpll_d8_d2"], dramc_md32_parents => ["top_xtal","top_mpll_d2"],
 sysaxi_parents => ["top_xtal","top_net1pll_d8_d2","top_net2pll_d4"], sysapb_parents => ["top_xtal","top_mpll_d3_d2","top_net2pll_d4_d2"],
 arm_db_main_parents => ["top_xtal","top_net2pll_d3_d2"], arm_db_jtsel_parents => ["top_jtag","top_xtal"],
 netsys_parents => ["top_xtal","top_mmpll_d4"], netsys_500m_parents => ["top_xtal","top_net1pll_d5"],
 netsys_mcu_parents => ["top_xtal","wedmcupll","top_mmpll_d2","top_net1pll_d4","top_net1pll_d5"],
 netsys_2x_parents => ["top_xtal","net2pll","wedmcupll","top_mmpll_d2"], sgm_325m_parents => ["top_xtal","sgmpll"],
 sgm_reg_parents => ["top_xtal","top_net1pll_d8_d4"], a1sys_parents => ["top_xtal","top_apll2_d4"],
 conn_mcusys_parents => ["top_xtal","top_mmpll_d2"], eip_b_parents => ["top_xtal","net2pll"],
 aud_l_parents => ["top_xtal","apll2","top_mpll_d8_d2"], a_tuner_parents => ["top_xtal","top_apll2_d4","top_mpll_d8_d2"],
 u2u3_sys_parents => ["top_xtal","top_net1pll_d5_d4"], da_u2_refsel_parents => ["top_xtal","top_mmpll_u2phy"],
}

static top_muxes: [mtk_mux; 40] = [
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NFI1X_SEL,"nfi1x_sel",nfi1x_parents,0x000,0x004,0x008,0,3,7,0x1C0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPINFI_SEL,"spinfi_sel",spinfi_parents,0x000,0x004,0x008,8,3,15,0x1C0,1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPI_SEL,"spi_sel",spi_parents,0x000,0x004,0x008,16,3,23,0x1C0,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPIM_MST_SEL,"spim_mst_sel",spi_parents,0x000,0x004,0x008,24,3,31,0x1C0,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_UART_SEL,"uart_sel",uart_parents,0x010,0x014,0x018,0,2,7,0x1C0,4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PWM_SEL,"pwm_sel",pwm_parents,0x010,0x014,0x018,8,2,15,0x1C0,5),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_I2C_SEL,"i2c_sel",i2c_parents,0x010,0x014,0x018,16,2,23,0x1C0,6),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_TL_SEL,"pextp_tl_ck_sel",pextp_tl_ck_parents,0x010,0x014,0x018,24,2,31,0x1C0,7),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_EMMC_250M_SEL,"emmc_250m_sel",emmc_250m_parents,0x020,0x024,0x028,0,1,7,0x1C0,8,0),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_EMMC_416M_SEL,"emmc_416m_sel",emmc_416m_parents,0x020,0x024,0x028,8,1,15,0x1C0,9,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_F_26M_ADC_SEL,"f_26m_adc_sel",f_26m_adc_parents,0x020,0x024,0x028,16,1,23,0x1C0,10),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_DRAMC_SEL,"dramc_sel",f_26m_adc_parents,0x020,0x024,0x028,24,1,31,0x1C0,11,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_DRAMC_MD32_SEL,"dramc_md32_sel",dramc_md32_parents,0x030,0x034,0x038,0,1,7,0x1C0,12,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SYSAXI_SEL,"sysaxi_sel",sysaxi_parents,0x030,0x034,0x038,8,2,15,0x1C0,13,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SYSAPB_SEL,"sysapb_sel",sysapb_parents,0x030,0x034,0x038,16,2,23,0x1C0,14,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ARM_DB_MAIN_SEL,"arm_db_main_sel",arm_db_main_parents,0x030,0x034,0x038,24,1,31,0x1C0,15),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ARM_DB_JTSEL,"arm_db_jtsel",arm_db_jtsel_parents,0x040,0x044,0x048,0,1,7,0x1C0,16),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_SEL,"netsys_sel",netsys_parents,0x040,0x044,0x048,8,1,15,0x1C0,17),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_500M_SEL,"netsys_500m_sel",netsys_500m_parents,0x040,0x044,0x048,16,1,23,0x1C0,18),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_MCU_SEL,"netsys_mcu_sel",netsys_mcu_parents,0x040,0x044,0x048,24,3,31,0x1C0,19),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_2X_SEL,"netsys_2x_sel",netsys_2x_parents,0x050,0x054,0x058,0,2,7,0x1C0,20),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SGM_325M_SEL,"sgm_325m_sel",sgm_325m_parents,0x050,0x054,0x058,8,1,15,0x1C0,21),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SGM_REG_SEL,"sgm_reg_sel",sgm_reg_parents,0x050,0x054,0x058,16,1,23,0x1C0,22,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_A1SYS_SEL,"a1sys_sel",a1sys_parents,0x050,0x054,0x058,24,1,31,0x1C0,23),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CONN_MCUSYS_SEL,"conn_mcusys_sel",conn_mcusys_parents,0x060,0x064,0x068,0,1,7,0x1C0,24),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_EIP_B_SEL,"eip_b_sel",eip_b_parents,0x060,0x064,0x068,8,1,15,0x1C0,25),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PCIE_PHY_SEL,"pcie_phy_sel",f_26m_adc_parents,0x060,0x064,0x068,16,1,23,0x1C0,26),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB3_PHY_SEL,"usb3_phy_sel",f_26m_adc_parents,0x060,0x064,0x068,24,1,31,0x1C0,27),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_F26M_SEL,"csw_f26m_sel",f_26m_adc_parents,0x070,0x074,0x078,0,1,7,0x1C0,28,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_L_SEL,"aud_l_sel",aud_l_parents,0x070,0x074,0x078,8,2,15,0x1C0,29),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_A_TUNER_SEL,"a_tuner_sel",a_tuner_parents,0x070,0x074,0x078,16,2,23,0x1C0,30),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_U2U3_SEL,"u2u3_sel",f_26m_adc_parents,0x070,0x074,0x078,24,1,31,0x1C4,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_U2U3_SYS_SEL,"u2u3_sys_sel",u2u3_sys_parents,0x080,0x084,0x088,0,1,7,0x1C4,1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_U2U3_XHCI_SEL,"u2u3_xhci_sel",u2u3_sys_parents,0x080,0x084,0x088,8,1,15,0x1C4,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_U2_REFSEL,"da_u2_refsel",da_u2_refsel_parents,0x080,0x084,0x088,16,1,23,0x1C4,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_U2_CK_1P_SEL,"da_u2_ck_1p_sel",da_u2_refsel_parents,0x080,0x084,0x088,24,1,31,0x1C4,4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AP2CNN_HOST_SEL,"ap2cnn_host_sel",sgm_reg_parents,0x090,0x094,0x098,0,1,7,0x1C4,5),
];

static topck_desc: mtk_clk_desc = mtk_clk_desc { fixed_clks: top_fixed_clks.as_ptr(), num_fixed_clks: top_fixed_clks.len(), factor_clks: top_divs.as_ptr(), num_factor_clks: top_divs.len(), mux_clks: top_muxes.as_ptr(), num_mux_clks: top_muxes.len(), clk_lock: unsafe { &mut mt7986_clk_lock } };

static of_match_clk_mt7986_topckgen: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("mediatek,mt7986-topckgen"), data: &topck_desc as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut clk_mt7986_topckgen_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe), remove: Some(mtk_clk_simple_remove),
    driver: device_driver { name: cstr!("clk-mt7986-topckgen"), of_match_table: of_match_clk_mt7986_topckgen.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7986_topckgen);
// module_platform_driver(clk_mt7986_topckgen_drv);
// MODULE_DESCRIPTION("MediaTek MT7986 top clock generators driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
