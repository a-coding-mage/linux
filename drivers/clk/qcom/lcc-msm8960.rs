// SPDX-License-Identifier: GPL-2.0-only
/* Translated from lcc-msm8960.c. C headers and external kernel symbols are
 * intentionally represented as dependencies supplied by the surrounding tree. */

static mut pxo_parent_data: clk_parent_data = clk_parent_data { fw_name: "pxo", name: "pxo_board" };

static mut pll4: clk_pll = clk_pll {
    l_reg: 0x4, m_reg: 0x8, n_reg: 0xc, config_reg: 0x14,
    mode_reg: 0x0, status_reg: 0x18, status_bit: 16,
    clkr: clk_regmap { hw: clk_hw_init { name: "pll4", parent_data: &mut pxo_parent_data, num_parents: 1, ops: &clk_pll_ops } },
};

enum { P_PXO, P_PLL4 }

static lcc_pxo_pll4_map: [parent_map; 2] = [parent_map { parent: P_PXO, val: 0 }, parent_map { parent: P_PLL4, val: 2 }];
static mut lcc_pxo_pll4: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: "pxo", name: "pxo_board" },
    clk_parent_data { fw_name: "pll4_vote", name: "pll4_vote" },
];

static clk_tbl_aif_osr_492: [freq_tbl; 13] = [
    freq_tbl(512000,P_PLL4,4,1,240), freq_tbl(768000,P_PLL4,4,1,160), freq_tbl(1024000,P_PLL4,4,1,120),
    freq_tbl(1536000,P_PLL4,4,1,80), freq_tbl(2048000,P_PLL4,4,1,60), freq_tbl(3072000,P_PLL4,4,1,40),
    freq_tbl(4096000,P_PLL4,4,1,30), freq_tbl(6144000,P_PLL4,4,1,20), freq_tbl(8192000,P_PLL4,4,1,15),
    freq_tbl(12288000,P_PLL4,4,1,10), freq_tbl(24576000,P_PLL4,4,1,5), freq_tbl(27000000,P_PXO,1,0,0), freq_tbl::default(),
];
static clk_tbl_aif_osr_393: [freq_tbl; 13] = [
    freq_tbl(512000,P_PLL4,4,1,192), freq_tbl(768000,P_PLL4,4,1,128), freq_tbl(1024000,P_PLL4,4,1,96),
    freq_tbl(1536000,P_PLL4,4,1,64), freq_tbl(2048000,P_PLL4,4,1,48), freq_tbl(3072000,P_PLL4,4,1,32),
    freq_tbl(4096000,P_PLL4,4,1,24), freq_tbl(6144000,P_PLL4,4,1,16), freq_tbl(8192000,P_PLL4,4,1,12),
    freq_tbl(12288000,P_PLL4,4,1,8), freq_tbl(24576000,P_PLL4,4,1,4), freq_tbl(27000000,P_PXO,1,0,0), freq_tbl::default(),
];

// The following macro expansions preserve the five C clock-construction macros.
macro_rules! CLK_AIF_OSR_SRC { ($p:ident,$ns:expr,$md:expr) => { static mut $p##_osr_src: clk_rcg = clk_rcg::new($ns,$md,stringify!($p),&lcc_pxo_pll4_map,&clk_tbl_aif_osr_393); }; }
macro_rules! CLK_AIF_OSR_CLK { ($p:ident,$ns:expr,$hr:expr,$bit:expr) => { static mut $p##_osr_clk: clk_branch = clk_branch::new($hr,1,$ns,$bit,stringify!($p)); }; }
macro_rules! CLK_AIF_OSR_DIV_CLK { ($p:ident,$ns:expr,$width:expr) => { static mut $p##_div_clk: clk_regmap_div = clk_regmap_div::new($ns,10,$width,stringify!($p)); }; }
macro_rules! CLK_AIF_OSR_BIT_DIV_CLK { ($p:ident,$ns:expr,$hr:expr,$bit:expr) => { static mut $p##_bit_div_clk: clk_branch = clk_branch::new($hr,0,$ns,$bit,stringify!($p)); }; }
macro_rules! CLK_AIF_OSR_BIT_CLK { ($p:ident,$ns:expr,$shift:expr) => { static mut $p##_bit_clk: clk_regmap_mux = clk_regmap_mux::new($ns,$shift,stringify!($p)); }; }

CLK_AIF_OSR_SRC!(mi2s,0x48,0x4c); CLK_AIF_OSR_CLK!(mi2s,0x48,0x50,17); CLK_AIF_OSR_DIV_CLK!(mi2s,0x48,4); CLK_AIF_OSR_BIT_DIV_CLK!(mi2s,0x48,0x50,15); CLK_AIF_OSR_BIT_CLK!(mi2s,0x48,14);

macro_rules! CLK_AIF_OSR_DIV { ($p:ident,$ns:expr,$md:expr,$hr:expr) => { CLK_AIF_OSR_SRC!($p,$ns,$md); CLK_AIF_OSR_CLK!($p,$ns,$hr,21); CLK_AIF_OSR_DIV_CLK!($p,$ns,8); CLK_AIF_OSR_BIT_DIV_CLK!($p,$ns,$hr,19); CLK_AIF_OSR_BIT_CLK!($p,$ns,18); }; }
CLK_AIF_OSR_DIV!(codec_i2s_mic,0x60,0x64,0x68); CLK_AIF_OSR_DIV!(spare_i2s_mic,0x78,0x7c,0x80); CLK_AIF_OSR_DIV!(codec_i2s_spkr,0x6c,0x70,0x74); CLK_AIF_OSR_DIV!(spare_i2s_spkr,0x84,0x88,0x8c);

static clk_tbl_pcm_492: [freq_tbl; 14] = [freq_tbl(256000,P_PLL4,4,1,480),freq_tbl(512000,P_PLL4,4,1,240),freq_tbl(768000,P_PLL4,4,1,160),freq_tbl(1024000,P_PLL4,4,1,120),freq_tbl(1536000,P_PLL4,4,1,80),freq_tbl(2048000,P_PLL4,4,1,60),freq_tbl(3072000,P_PLL4,4,1,40),freq_tbl(4096000,P_PLL4,4,1,30),freq_tbl(6144000,P_PLL4,4,1,20),freq_tbl(8192000,P_PLL4,4,1,15),freq_tbl(12288000,P_PLL4,4,1,10),freq_tbl(24576000,P_PLL4,4,1,5),freq_tbl(27000000,P_PXO,1,0,0),freq_tbl::default()];
static clk_tbl_pcm_393: [freq_tbl; 14] = [freq_tbl(256000,P_PLL4,4,1,384),freq_tbl(512000,P_PLL4,4,1,192),freq_tbl(768000,P_PLL4,4,1,128),freq_tbl(1024000,P_PLL4,4,1,96),freq_tbl(1536000,P_PLL4,4,1,64),freq_tbl(2048000,P_PLL4,4,1,48),freq_tbl(3072000,P_PLL4,4,1,32),freq_tbl(4096000,P_PLL4,4,1,24),freq_tbl(6144000,P_PLL4,4,1,16),freq_tbl(8192000,P_PLL4,4,1,12),freq_tbl(12288000,P_PLL4,4,1,8),freq_tbl(24576000,P_PLL4,4,1,4),freq_tbl(27000000,P_PXO,1,0,0),freq_tbl::default()];

static mut pcm_src: clk_rcg = clk_rcg::new_full(0x54,0x58,16,&lcc_pxo_pll4_map,&clk_tbl_pcm_393,"pcm_src");
static mut pcm_clk_out: clk_branch = clk_branch::new(0x5c,0,0x54,11,"pcm_clk_out");
static mut pcm_clk: clk_regmap_mux = clk_regmap_mux::new(0x54,10,"pcm_clk");
static mut slimbus_src: clk_rcg = clk_rcg::new_full(0xcc,0xd0,8,&lcc_pxo_pll4_map,&clk_tbl_aif_osr_393,"slimbus_src");
static mut audio_slimbus_clk: clk_branch = clk_branch::new(0xd4,0,0xcc,10,"audio_slimbus_clk");
static mut sps_slimbus_clk: clk_branch = clk_branch::new(0xd4,1,0xcc,12,"sps_slimbus_clk");

static lcc_msm8960_clks: [*mut clk_regmap; 30] = [
    &mut pll4.clkr,&mut mi2s_osr_src.clkr,&mut mi2s_osr_clk.clkr,&mut mi2s_div_clk.clkr,&mut mi2s_bit_div_clk.clkr,&mut mi2s_bit_clk.clkr,
    &mut pcm_src.clkr,&mut pcm_clk_out.clkr,&mut pcm_clk.clkr,&mut slimbus_src.clkr,&mut audio_slimbus_clk.clkr,&mut sps_slimbus_clk.clkr,
    &mut codec_i2s_mic_osr_src.clkr,&mut codec_i2s_mic_osr_clk.clkr,&mut codec_i2s_mic_div_clk.clkr,&mut codec_i2s_mic_bit_div_clk.clkr,&mut codec_i2s_mic_bit_clk.clkr,
    &mut spare_i2s_mic_osr_src.clkr,&mut spare_i2s_mic_osr_clk.clkr,&mut spare_i2s_mic_div_clk.clkr,&mut spare_i2s_mic_bit_div_clk.clkr,&mut spare_i2s_mic_bit_clk.clkr,
    &mut codec_i2s_spkr_osr_src.clkr,&mut codec_i2s_spkr_osr_clk.clkr,&mut codec_i2s_spkr_div_clk.clkr,&mut codec_i2s_spkr_bit_div_clk.clkr,&mut codec_i2s_spkr_bit_clk.clkr,
    &mut spare_i2s_spkr_osr_src.clkr,&mut spare_i2s_spkr_osr_clk.clkr,&mut spare_i2s_spkr_div_clk.clkr,&mut spare_i2s_spkr_bit_div_clk.clkr,&mut spare_i2s_spkr_bit_clk.clkr,
];
static lcc_msm8960_regmap_config: regmap_config = regmap_config { reg_bits:32, reg_stride:4, val_bits:32, max_register:0xfc, fast_io:true };
static lcc_msm8960_desc: qcom_cc_desc = qcom_cc_desc { config:&lcc_msm8960_regmap_config, clks:&lcc_msm8960_clks, num_clks:lcc_msm8960_clks.len() };
static lcc_msm8960_match_table: [of_device_id; 4] = [of_device_id::compatible("qcom,lcc-msm8960"),of_device_id::compatible("qcom,lcc-apq8064"),of_device_id::compatible("qcom,lcc-mdm9615"),of_device_id::default()];

unsafe fn lcc_msm8960_probe(pdev: *mut platform_device) -> i32 {
    let mut val: u32 = 0;
    if of_device_is_compatible((*pdev).dev.of_node,"qcom,lcc-mdm9615") { pxo_parent_data.fw_name="cxo"; pxo_parent_data.name="cxo_board"; lcc_pxo_pll4[0].fw_name="cxo"; lcc_pxo_pll4[0].name="cxo_board"; }
    let regmap = qcom_cc_map(pdev,&lcc_msm8960_desc); if IS_ERR(regmap) { return PTR_ERR(regmap); }
    regmap_read(regmap,0x4,&mut val); if val == 0x12 { slimbus_src.freq_tbl=&clk_tbl_aif_osr_492; mi2s_osr_src.freq_tbl=&clk_tbl_aif_osr_492; codec_i2s_mic_osr_src.freq_tbl=&clk_tbl_aif_osr_492; spare_i2s_mic_osr_src.freq_tbl=&clk_tbl_aif_osr_492; codec_i2s_spkr_osr_src.freq_tbl=&clk_tbl_aif_osr_492; spare_i2s_spkr_osr_src.freq_tbl=&clk_tbl_aif_osr_492; pcm_src.freq_tbl=&clk_tbl_pcm_492; }
    regmap_write(regmap,0xc4,0x1); qcom_cc_really_probe(&mut (*pdev).dev,&lcc_msm8960_desc,regmap)
}

static lcc_msm8960_driver: platform_driver = platform_driver { probe:lcc_msm8960_probe, name:"lcc-msm8960", of_match_table:&lcc_msm8960_match_table };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
