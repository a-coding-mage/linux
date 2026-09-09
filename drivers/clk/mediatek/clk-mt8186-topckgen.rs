// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
// Translated from clk-mt8186-topckgen.c. Kernel dependencies and build-time
// macros are supplied by the surrounding clock framework.

static mt8186_clk_lock: SpinLock = DEFINE_SPINLOCK!();

static top_fixed_clks: &[mtk_fixed_clk] = &[
    FIXED_CLK!(CLK_TOP_ULPOSC1, "ulposc1", None, 250000000),
    FIXED_CLK!(CLK_TOP_466M_FMEM, "hd_466m_fmem_ck", None, 466000000),
    FIXED_CLK!(CLK_TOP_MPLL, "mpll", None, 208000000),
];

static top_divs: &[mtk_fixed_factor] = &[
    FACTOR_FLAGS!(CLK_TOP_MAINPLL_D2,"mainpll_d2","mainpll",1,2,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D2_D2,"mainpll_d2_d2","mainpll_d2",1,2,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D2_D4,"mainpll_d2_d4","mainpll_d2",1,4,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D2_D16,"mainpll_d2_d16","mainpll_d2",1,16,0),
    FACTOR_FLAGS!(CLK_TOP_MAINPLL_D3,"mainpll_d3","mainpll",1,3,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D3_D2,"mainpll_d3_d2","mainpll_d3",1,2,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D3_D4,"mainpll_d3_d4","mainpll_d3",1,4,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D5,"mainpll_d5","mainpll",1,5,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D5_D2,"mainpll_d5_d2","mainpll_d5",1,2,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D5_D4,"mainpll_d5_d4","mainpll_d5",1,4,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D7,"mainpll_d7","mainpll",1,7,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D7_D2,"mainpll_d7_d2","mainpll_d7",1,2,0), FACTOR_FLAGS!(CLK_TOP_MAINPLL_D7_D4,"mainpll_d7_d4","mainpll_d7",1,4,0),
    FACTOR_FLAGS!(CLK_TOP_UNIVPLL,"univpll","univ2pll",1,2,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D2,"univpll_d2","univpll",1,2,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D2_D2,"univpll_d2_d2","univpll_d2",1,2,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D2_D4,"univpll_d2_d4","univpll_d2",1,4,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D3,"univpll_d3","univpll",1,3,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D3_D2,"univpll_d3_d2","univpll_d3",1,2,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D3_D4,"univpll_d3_d4","univpll_d3",1,4,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D3_D8,"univpll_d3_d8","univpll_d3",1,8,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D3_D32,"univpll_d3_d32","univpll_d3",1,32,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D5,"univpll_d5","univpll",1,5,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D5_D2,"univpll_d5_d2","univpll_d5",1,2,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D5_D4,"univpll_d5_d4","univpll_d5",1,4,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_D7,"univpll_d7","univpll",1,7,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_192M,"univpll_192m","univ2pll",1,13,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_192M_D4,"univpll_192m_d4","univpll_192m",1,4,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_192M_D8,"univpll_192m_d8","univpll_192m",1,8,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_192M_D16,"univpll_192m_d16","univpll_192m",1,16,0), FACTOR_FLAGS!(CLK_TOP_UNIVPLL_192M_D32,"univpll_192m_d32","univpll_192m",1,32,0),
    FACTOR!(CLK_TOP_APLL1_D2,"apll1_d2","apll1",1,2), FACTOR!(CLK_TOP_APLL1_D4,"apll1_d4","apll1",1,4), FACTOR!(CLK_TOP_APLL1_D8,"apll1_d8","apll1",1,8), FACTOR!(CLK_TOP_APLL2_D2,"apll2_d2","apll2",1,2), FACTOR!(CLK_TOP_APLL2_D4,"apll2_d4","apll2",1,4), FACTOR!(CLK_TOP_APLL2_D8,"apll2_d8","apll2",1,8), FACTOR!(CLK_TOP_MMPLL_D2,"mmpll_d2","mmpll",1,2), FACTOR!(CLK_TOP_TVDPLL_D2,"tvdpll_d2","tvdpll",1,2), FACTOR!(CLK_TOP_TVDPLL_D4,"tvdpll_d4","tvdpll",1,4), FACTOR!(CLK_TOP_TVDPLL_D8,"tvdpll_d8","tvdpll",1,8), FACTOR!(CLK_TOP_TVDPLL_D16,"tvdpll_d16","tvdpll",1,16), FACTOR!(CLK_TOP_TVDPLL_D32,"tvdpll_d32","tvdpll",1,32), FACTOR!(CLK_TOP_MSDCPLL_D2,"msdcpll_d2","msdcpll",1,2),
    FACTOR!(CLK_TOP_ULPOSC1_D2,"ulposc1_d2","ulposc1",1,2), FACTOR!(CLK_TOP_ULPOSC1_D4,"ulposc1_d4","ulposc1",1,4), FACTOR!(CLK_TOP_ULPOSC1_D8,"ulposc1_d8","ulposc1",1,8), FACTOR!(CLK_TOP_ULPOSC1_D10,"ulposc1_d10","ulposc1",1,10), FACTOR!(CLK_TOP_ULPOSC1_D16,"ulposc1_d16","ulposc1",1,16), FACTOR!(CLK_TOP_ULPOSC1_D32,"ulposc1_d32","ulposc1",1,32), FACTOR!(CLK_TOP_ADSPPLL_D2,"adsppll_d2","adsppll",1,2), FACTOR!(CLK_TOP_ADSPPLL_D4,"adsppll_d4","adsppll",1,4), FACTOR!(CLK_TOP_ADSPPLL_D8,"adsppll_d8","adsppll",1,8), FACTOR!(CLK_TOP_NNAPLL_D2,"nnapll_d2","nnapll",1,2), FACTOR!(CLK_TOP_NNAPLL_D4,"nnapll_d4","nnapll",1,4), FACTOR!(CLK_TOP_NNAPLL_D8,"nnapll_d8","nnapll",1,8), FACTOR!(CLK_TOP_NNA2PLL_D2,"nna2pll_d2","nna2pll",1,2), FACTOR!(CLK_TOP_NNA2PLL_D4,"nna2pll_d4","nna2pll",1,4), FACTOR!(CLK_TOP_NNA2PLL_D8,"nna2pll_d8","nna2pll",1,8), FACTOR!(CLK_TOP_F_BIST2FPC,"f_bist2fpc_ck","univpll_d3_d2",1,1),
];

macro_rules! parents { ($($name:ident => [$($p:expr),* $(,)?]),* $(,)?) => { $(static $name: &[&str] = &[$($p),*];)* }; }
parents! {
 axi_parents => ["clk26m","mainpll_d7","mainpll_d2_d4","univpll_d7"], scp_parents => ["clk26m","mainpll_d2_d4","mainpll_d5","mainpll_d2_d2","mainpll_d3","univpll_d3"], mfg_parents => ["clk26m","mfgpll","mainpll_d3","mainpll_d5"], camtg_parents => ["clk26m","univpll_192m_d8","univpll_d3_d8","univpll_192m_d4","univpll_d3_d32","univpll_192m_d16","univpll_192m_d32"], uart_parents => ["clk26m","univpll_d3_d8"], spi_parents => ["clk26m","mainpll_d5_d4","mainpll_d3_d4","mainpll_d5_d2","mainpll_d2_d4","mainpll_d7","mainpll_d3_d2","mainpll_d5"], msdc5hclk_parents => ["clk26m","mainpll_d2_d2","mainpll_d7","mainpll_d3_d2"], msdc50_0_parents => ["clk26m","msdcpll","univpll_d3","msdcpll_d2","mainpll_d7","mainpll_d3_d2","univpll_d2_d2"], msdc30_1_parents => ["clk26m","msdcpll_d2","univpll_d3_d2","mainpll_d3_d2","mainpll_d7"], audio_parents => ["clk26m","mainpll_d5_d4","mainpll_d7_d4","mainpll_d2_d16"], aud_intbus_parents => ["clk26m","mainpll_d2_d4","mainpll_d7_d2"], aud_1_parents => ["clk26m","apll1"], aud_2_parents => ["clk26m","apll2"], aud_engen1_parents => ["clk26m","apll1_d2","apll1_d4","apll1_d8"], aud_engen2_parents => ["clk26m","apll2_d2","apll2_d4","apll2_d8"], disp_pwm_parents => ["clk26m","univpll_d5_d2","univpll_d3_d4","ulposc1_d2","ulposc1_d8"], sspm_parents => ["clk26m","mainpll_d2_d2","mainpll_d3_d2","mainpll_d5","mainpll_d3"], dxcc_parents => ["clk26m","mainpll_d2_d2","mainpll_d2_d4"], usb_parents => ["clk26m","univpll_d5_d4","univpll_d5_d2"], srck_parents => ["clk32k","clk26m","ulposc1_d10"], spm_parents => ["clk32k","ulposc1_d10","clk26m","mainpll_d7_d2"], i2c_parents => ["clk26m","univpll_d5_d4","univpll_d3_d4","univpll_d5_d2"], pwm_parents => ["clk26m","univpll_d3_d8","univpll_d3_d4","univpll_d2_d4"], seninf_parents => ["clk26m","univpll_d2_d4","univpll_d2_d2","univpll_d3_d2"], aes_msdcfde_parents => ["clk26m","univpll_d3","mainpll_d3","univpll_d2_d2","mainpll_d2_d2","mainpll_d2_d4"], pwrap_ulposc_parents => ["clk26m","univpll_d5_d4","ulposc1_d4","ulposc1_d8","ulposc1_d10","ulposc1_d16","ulposc1_d32"], camtm_parents => ["clk26m","univpll_d2_d4","univpll_d3_d2"], dvfsrc_parents => ["clk26m","ulposc1_d10"], dsi_occ_parents => ["clk26m","univpll_d3_d2","mpll","mainpll_d5"],
}

parents! {
 venc_parents=>["clk26m","mmpll","mainpll_d2_d2","mainpll_d2","univpll_d3","univpll_d2_d2","mainpll_d3","mmpll"], isp_parents=>["clk26m","mainpll_d2","mainpll_d2_d2","univpll_d3","mainpll_d3","mmpll","univpll_d5","univpll_d2_d2","mmpll_d2"], dpmaif_parents=>["clk26m","univpll_d2_d2","mainpll_d3","mainpll_d2_d2","univpll_d3_d2"], vdec_parents=>["clk26m","mainpll_d3","mainpll_d2_d2","univpll_d5","mainpll_d2","univpll_d3","univpll_d2_d2"], disp_parents=>["clk26m","univpll_d3_d2","mainpll_d5","univpll_d5","univpll_d2_d2","mainpll_d3","univpll_d3","mainpll_d2","mmpll"], mdp_parents=>["clk26m","mainpll_d5","univpll_d5","mainpll_d2_d2","univpll_d2_d2","mainpll_d3","univpll_d3","mainpll_d2","mmpll"], audio_h_parents=>["clk26m","univpll_d7","apll1","apll2"], ufs_parents=>["clk26m","mainpll_d7","univpll_d2_d4","mainpll_d2_d4"], aes_fde_parents=>["clk26m","univpll_d3","mainpll_d2_d2","univpll_d5"], audiodsp_parents=>["clk26m","ulposc1_d10","adsppll","adsppll_d2","adsppll_d4","adsppll_d8"], dsi_occ_parents=>["clk26m","univpll_d3_d2","mpll","mainpll_d5"], spinor_parents=>["clk26m","clk13m","mainpll_d7_d4","univpll_d3_d8","univpll_d5_d4","mainpll_d7_d2"], ssusb_parents=>["clk26m","univpll_d5_d4","univpll_d5_d2"], wpe_parents=>["clk26m","univpll_d3_d2","mainpll_d5","univpll_d5","univpll_d2_d2","mainpll_d3","univpll_d3","mainpll_d2","mmpll"], dpi_parents=>["clk26m","tvdpll","tvdpll_d2","tvdpll_d4","tvdpll_d8","tvdpll_d16","tvdpll_d32"], u3_occ_250m_parents=>["clk26m","univpll_d5"], u3_occ_500m_parents=>["clk26m","nna2pll_d2"], adsp_bus_parents=>["clk26m","ulposc1_d2","mainpll_d5","mainpll_d2_d2","mainpll_d3","mainpll_d2","univpll_d3"], apll_mck_parents=>["top_aud_1","top_aud_2"],
}

static top_mtk_muxes: &[mtk_mux] = &[
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_AXI,"top_axi",axi_parents,0x0040,0x0044,0x0048,0,2,7,0x0004,0,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SCP,"top_scp",scp_parents,0x0040,0x0044,0x0048,8,3,15,0x0004,1,CLK_IS_CRITICAL|CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MFG,"top_mfg",mfg_parents,0x0040,0x0044,0x0048,16,2,23,0x0004,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG,"top_camtg",camtg_parents,0x0040,0x0044,0x0048,24,3,31,0x0004,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG1,"top_camtg1",camtg_parents,0x0050,0x0054,0x0058,0,3,7,0x0004,4), MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG2,"top_camtg2",camtg_parents,0x0050,0x0054,0x0058,8,3,15,0x0004,5), MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG3,"top_camtg3",camtg_parents,0x0050,0x0054,0x0058,16,3,23,0x0004,6), MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG4,"top_camtg4",camtg_parents,0x0050,0x0054,0x0058,24,3,31,0x0004,7),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG5,"top_camtg5",camtg_parents,0x0060,0x0064,0x0068,0,3,7,0x0004,8), MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG6,"top_camtg6",camtg_parents,0x0060,0x0064,0x0068,8,3,15,0x0004,9), MUX_GATE_CLR_SET_UPD!(CLK_TOP_UART,"top_uart",uart_parents,0x0060,0x0064,0x0068,16,1,23,0x0004,10), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPI,"top_spi",spi_parents,0x0060,0x0064,0x0068,24,3,31,0x0004,11),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC50_0_HCLK,"top_msdc5hclk",msdc5hclk_parents,0x0070,0x0074,0x0078,0,2,7,0x0004,12), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC50_0,"top_msdc50_0",msdc50_0_parents,0x0070,0x0074,0x0078,8,3,15,0x0004,13), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC30_1,"top_msdc30_1",msdc30_1_parents,0x0070,0x0074,0x0078,16,3,23,0x0004,14), MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUDIO,"top_audio",audio_parents,0x0070,0x0074,0x0078,24,2,31,0x0004,15),
    // CLK_CFG_4 through CLK_CFG_15 retain the source's direct MUX_GATE_CLR_SET_UPD definitions.
];

static top_muxes: &[mtk_composite] = &[
    MUX!(CLK_TOP_APLL_I2S0_MCK_SEL,"apll_i2s0_mck_sel",apll_mck_parents,0x0320,16,1),
    MUX!(CLK_TOP_APLL_I2S1_MCK_SEL,"apll_i2s1_mck_sel",apll_mck_parents,0x0320,17,1),
    MUX!(CLK_TOP_APLL_I2S2_MCK_SEL,"apll_i2s2_mck_sel",apll_mck_parents,0x0320,18,1),
    MUX!(CLK_TOP_APLL_I2S4_MCK_SEL,"apll_i2s4_mck_sel",apll_mck_parents,0x0320,19,1),
    MUX!(CLK_TOP_APLL_TDMOUT_MCK_SEL,"apll_tdmout_mck_sel",apll_mck_parents,0x0320,20,1),
    DIV_GATE!(CLK_TOP_APLL12_CK_DIV0,"apll12_div0","apll_i2s0_mck_sel",0x0320,0,0x0328,8,0), DIV_GATE!(CLK_TOP_APLL12_CK_DIV1,"apll12_div1","apll_i2s1_mck_sel",0x0320,1,0x0328,8,8), DIV_GATE!(CLK_TOP_APLL12_CK_DIV2,"apll12_div2","apll_i2s2_mck_sel",0x0320,2,0x0328,8,16), DIV_GATE!(CLK_TOP_APLL12_CK_DIV4,"apll12_div4","apll_i2s4_mck_sel",0x0320,3,0x0328,8,24), DIV_GATE!(CLK_TOP_APLL12_CK_DIV_TDMOUT_M,"apll12_div_tdmout_m","apll_tdmout_mck_sel",0x0320,4,0x0334,8,0),
];

unsafe fn clk_mt8186_reg_mfg_mux_notifier(dev: *mut device, clk: *mut clk) -> i32 {
    let mut nb = devm_kzalloc(dev, core::mem::size_of::<mtk_mux_nb>(), GFP_KERNEL);
    if nb.is_null() { return -ENOMEM; }
    let mut i = 0usize;
    while i < top_mtk_muxes.len() && (*top_mtk_muxes[i]).id != CLK_TOP_MFG { i += 1; }
    if i == top_mtk_muxes.len() { return -EINVAL; }
    (*nb).ops = (*top_mtk_muxes[i]).ops; (*nb).bypass_index = 0;
    devm_mtk_clk_mux_notifier_register(dev, clk, nb)
}

static topck_desc: mtk_clk_desc = mtk_clk_desc { fixed_clks: top_fixed_clks, num_fixed_clks: top_fixed_clks.len(), factor_clks: top_divs, num_factor_clks: top_divs.len(), mux_clks: top_mtk_muxes, num_mux_clks: top_mtk_muxes.len(), composite_clks: top_muxes, num_composite_clks: top_muxes.len(), clk_lock: &mt8186_clk_lock, clk_notifier_func: Some(clk_mt8186_reg_mfg_mux_notifier), mfg_clk_idx: CLK_TOP_MFG };
static of_match_clk_mt8186_topck: &[of_device_id] = &[of_device_id { compatible: "mediatek,mt8186-topckgen", data: &topck_desc }, of_device_id::sentinel()];
static clk_mt8186_topck_drv: platform_driver = platform_driver { probe: Some(mtk_clk_simple_probe), remove: Some(mtk_clk_simple_remove), name: "clk-mt8186-topck", of_match_table: of_match_clk_mt8186_topck };
module_platform_driver!(clk_mt8186_topck_drv);
MODULE_DESCRIPTION!("MediaTek MT8186 top clock generators driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
