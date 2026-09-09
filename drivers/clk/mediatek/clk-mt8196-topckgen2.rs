// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of clk-mt8196-topckgen2.c. External kernel items and
 * declaration-like macros are supplied by the surrounding translation unit. */

const CKSYS2_CLK_CFG_UPDATE: u32 = 0x0004;
const CKSYS2_CLK_CFG_0: u32 = 0x0010;
const CKSYS2_CLK_CFG_0_SET: u32 = 0x0014;
const CKSYS2_CLK_CFG_0_CLR: u32 = 0x0018;
const CKSYS2_CLK_CFG_1: u32 = 0x0020;
const CKSYS2_CLK_CFG_1_SET: u32 = 0x0024;
const CKSYS2_CLK_CFG_1_CLR: u32 = 0x0028;
const CKSYS2_CLK_CFG_2: u32 = 0x0030;
const CKSYS2_CLK_CFG_2_SET: u32 = 0x0034;
const CKSYS2_CLK_CFG_2_CLR: u32 = 0x0038;
const CKSYS2_CLK_CFG_3: u32 = 0x0040;
const CKSYS2_CLK_CFG_3_SET: u32 = 0x0044;
const CKSYS2_CLK_CFG_3_CLR: u32 = 0x0048;
const CKSYS2_CLK_CFG_4: u32 = 0x0050;
const CKSYS2_CLK_CFG_4_SET: u32 = 0x0054;
const CKSYS2_CLK_CFG_4_CLR: u32 = 0x0058;
const CKSYS2_CLK_CFG_5: u32 = 0x0060;
const CKSYS2_CLK_CFG_5_SET: u32 = 0x0064;
const CKSYS2_CLK_CFG_5_CLR: u32 = 0x0068;
const CKSYS2_CLK_CFG_6: u32 = 0x0070;
const CKSYS2_CLK_CFG_6_SET: u32 = 0x0074;
const CKSYS2_CLK_CFG_6_CLR: u32 = 0x0078;
const CKSYS2_CLK_FENC_STATUS_MON_0: u32 = 0x0174;

const HWV_CG_30_SET: u32 = 0x0058;
const HWV_CG_30_CLR: u32 = 0x005c;
const HWV_CG_30_DONE: u32 = 0x2c2c;
const MM_HWV_CG_30_SET: u32 = 0x00f0;
const MM_HWV_CG_30_CLR: u32 = 0x00f4;
const MM_HWV_CG_30_DONE: u32 = 0x2c78;
const MM_HWV_CG_31_SET: u32 = 0x00f8;
const MM_HWV_CG_31_CLR: u32 = 0x00fc;
const MM_HWV_CG_31_DONE: u32 = 0x2c7c;
const MM_HWV_CG_32_SET: u32 = 0x0100;
const MM_HWV_CG_32_CLR: u32 = 0x0104;
const MM_HWV_CG_32_DONE: u32 = 0x2c80;
const MM_HWV_CG_33_SET: u32 = 0x0108;
const MM_HWV_CG_33_CLR: u32 = 0x010c;
const MM_HWV_CG_33_DONE: u32 = 0x2c84;
const MM_HWV_CG_34_SET: u32 = 0x0110;
const MM_HWV_CG_34_CLR: u32 = 0x0114;
const MM_HWV_CG_34_DONE: u32 = 0x2c88;
const MM_HWV_CG_35_SET: u32 = 0x0118;
const MM_HWV_CG_35_CLR: u32 = 0x011c;
const MM_HWV_CG_35_DONE: u32 = 0x2c8c;
const MM_HWV_CG_36_SET: u32 = 0x0120;
const MM_HWV_CG_36_CLR: u32 = 0x0124;
const MM_HWV_CG_36_DONE: u32 = 0x2c90;

const TOP_MUX_SENINF0_SHIFT: u32 = 0; const TOP_MUX_SENINF1_SHIFT: u32 = 1;
const TOP_MUX_SENINF2_SHIFT: u32 = 2; const TOP_MUX_SENINF3_SHIFT: u32 = 3;
const TOP_MUX_SENINF4_SHIFT: u32 = 4; const TOP_MUX_SENINF5_SHIFT: u32 = 5;
const TOP_MUX_IMG1_SHIFT: u32 = 6; const TOP_MUX_IPE_SHIFT: u32 = 7;
const TOP_MUX_CAM_SHIFT: u32 = 8; const TOP_MUX_CAMTM_SHIFT: u32 = 9;
const TOP_MUX_DPE_SHIFT: u32 = 10; const TOP_MUX_VDEC_SHIFT: u32 = 11;
const TOP_MUX_CCUSYS_SHIFT: u32 = 12; const TOP_MUX_CCUTM_SHIFT: u32 = 13;
const TOP_MUX_VENC_SHIFT: u32 = 14; const TOP_MUX_DVO_SHIFT: u32 = 15;
const TOP_MUX_DVO_FAVT_SHIFT: u32 = 16; const TOP_MUX_DP1_SHIFT: u32 = 17;
const TOP_MUX_DP0_SHIFT: u32 = 18; const TOP_MUX_DISP_SHIFT: u32 = 19;
const TOP_MUX_MDP_SHIFT: u32 = 20; const TOP_MUX_MMINFRA_SHIFT: u32 = 21;
const TOP_MUX_MMINFRA_SNOC_SHIFT: u32 = 22; const TOP_MUX_MMUP_SHIFT: u32 = 23;
const TOP_MUX_MMINFRA_AO_SHIFT: u32 = 26;

static top_divs: [mtk_fixed_factor; 44] = [
    FACTOR!(CLK_TOP2_MAINPLL2_D2, "mainpll2_d2", "mainpll2", 1, 2), FACTOR!(CLK_TOP2_MAINPLL2_D3, "mainpll2_d3", "mainpll2", 1, 3), FACTOR!(CLK_TOP2_MAINPLL2_D4, "mainpll2_d4", "mainpll2", 1, 4), FACTOR!(CLK_TOP2_MAINPLL2_D4_D2, "mainpll2_d4_d2", "mainpll2", 1, 8), FACTOR!(CLK_TOP2_MAINPLL2_D4_D4, "mainpll2_d4_d4", "mainpll2", 1, 16), FACTOR!(CLK_TOP2_MAINPLL2_D5, "mainpll2_d5", "mainpll2", 1, 5), FACTOR!(CLK_TOP2_MAINPLL2_D5_D2, "mainpll2_d5_d2", "mainpll2", 1, 10), FACTOR!(CLK_TOP2_MAINPLL2_D6, "mainpll2_d6", "mainpll2", 1, 6), FACTOR!(CLK_TOP2_MAINPLL2_D6_D2, "mainpll2_d6_d2", "mainpll2", 1, 12), FACTOR!(CLK_TOP2_MAINPLL2_D7, "mainpll2_d7", "mainpll2", 1, 7), FACTOR!(CLK_TOP2_MAINPLL2_D7_D2, "mainpll2_d7_d2", "mainpll2", 1, 14), FACTOR!(CLK_TOP2_MAINPLL2_D9, "mainpll2_d9", "mainpll2", 1, 9),
    FACTOR!(CLK_TOP2_UNIVPLL2_D3, "univpll2_d3", "univpll2", 1, 3), FACTOR!(CLK_TOP2_UNIVPLL2_D4, "univpll2_d4", "univpll2", 1, 4), FACTOR!(CLK_TOP2_UNIVPLL2_D4_D2, "univpll2_d4_d2", "univpll2", 1, 8), FACTOR!(CLK_TOP2_UNIVPLL2_D5, "univpll2_d5", "univpll2", 1, 5), FACTOR!(CLK_TOP2_UNIVPLL2_D5_D2, "univpll2_d5_d2", "univpll2", 1, 10), FACTOR!(CLK_TOP2_UNIVPLL2_D6, "univpll2_d6", "univpll2", 1, 6), FACTOR!(CLK_TOP2_UNIVPLL2_D6_D2, "univpll2_d6_d2", "univpll2", 1, 12), FACTOR!(CLK_TOP2_UNIVPLL2_D6_D4, "univpll2_d6_d4", "univpll2", 1, 24), FACTOR!(CLK_TOP2_UNIVPLL2_D7, "univpll2_d7", "univpll2", 1, 7),
    FACTOR!(CLK_TOP2_IMGPLL_D2, "imgpll_d2", "imgpll", 1, 2), FACTOR!(CLK_TOP2_IMGPLL_D4, "imgpll_d4", "imgpll", 1, 4), FACTOR!(CLK_TOP2_IMGPLL_D5, "imgpll_d5", "imgpll", 1, 5), FACTOR!(CLK_TOP2_IMGPLL_D5_D2, "imgpll_d5_d2", "imgpll", 1, 10), FACTOR!(CLK_TOP2_MMPLL2_D3, "mmpll2_d3", "mmpll2", 1, 3), FACTOR!(CLK_TOP2_MMPLL2_D4, "mmpll2_d4", "mmpll2", 1, 4), FACTOR!(CLK_TOP2_MMPLL2_D4_D2, "mmpll2_d4_d2", "mmpll2", 1, 8), FACTOR!(CLK_TOP2_MMPLL2_D5, "mmpll2_d5", "mmpll2", 1, 5), FACTOR!(CLK_TOP2_MMPLL2_D5_D2, "mmpll2_d5_d2", "mmpll2", 1, 10), FACTOR!(CLK_TOP2_MMPLL2_D6, "mmpll2_d6", "mmpll2", 1, 6), FACTOR!(CLK_TOP2_MMPLL2_D6_D2, "mmpll2_d6_d2", "mmpll2", 1, 12), FACTOR!(CLK_TOP2_MMPLL2_D7, "mmpll2_d7", "mmpll2", 1, 7), FACTOR!(CLK_TOP2_MMPLL2_D9, "mmpll2_d9", "mmpll2", 1, 9),
    FACTOR!(CLK_TOP2_TVDPLL1_D4, "tvdpll1_d4", "tvdpll1", 1, 4), FACTOR!(CLK_TOP2_TVDPLL1_D8, "tvdpll1_d8", "tvdpll1", 1, 8), FACTOR!(CLK_TOP2_TVDPLL1_D16, "tvdpll1_d16", "tvdpll1", 1, 16), FACTOR!(CLK_TOP2_TVDPLL2_D2, "tvdpll2_d2", "tvdpll2", 1, 2), FACTOR!(CLK_TOP2_TVDPLL2_D4, "tvdpll2_d4", "tvdpll2", 1, 4), FACTOR!(CLK_TOP2_TVDPLL2_D8, "tvdpll2_d8", "tvdpll2", 1, 8), FACTOR!(CLK_TOP2_TVDPLL2_D16, "tvdpll2_d16", "tvdpll2", 92, 1473), FACTOR!(CLK_TOP2_TVDPLL3_D2, "tvdpll3_d2", "tvdpll3", 1, 2), FACTOR!(CLK_TOP2_TVDPLL3_D4, "tvdpll3_d4", "tvdpll3", 1, 4), FACTOR!(CLK_TOP2_TVDPLL3_D8, "tvdpll3_d8", "tvdpll3", 1, 8), FACTOR!(CLK_TOP2_TVDPLL3_D16, "tvdpll3_d16", "tvdpll3", 92, 1473),
];

macro_rules! parents { ($($x:expr),* $(,)?) => { &[$($x),*] as &'static [&'static str] }; }
static seninf_parents: &[&str] = parents!["clk26m","ck_osc_d10","ck_osc_d8","ck_osc_d5","ck_osc_d4","univpll2_d6_d2","mainpll2_d9","ck_osc_d2","mainpll2_d4_d2","univpll2_d4_d2","mmpll2_d4_d2","univpll2_d7","mainpll2_d6","mmpll2_d7","univpll2_d6","univpll2_d5"];
static img1_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_osc_d3","mmpll2_d6_d2","ck_osc_d2","imgpll_d5_d2","mmpll2_d5_d2","univpll2_d4_d2","mmpll2_d4_d2","mmpll2_d7","univpll2_d6","mmpll2_d6","univpll2_d5","mmpll2_d5","univpll2_d4","imgpll_d4"];
static ipe_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_osc_d3","ck_osc_d2","univpll2_d6","mmpll2_d6","univpll2_d5","imgpll_d5","ck_mainpll_d4","mmpll2_d5","imgpll_d4"];
static cam_parents: &[&str] = parents!["clk26m","ck_osc_d10","ck_osc_d4","ck_osc_d3","ck_osc_d2","mmpll2_d5_d2","univpll2_d4_d2","univpll2_d7","mmpll2_d7","univpll2_d6","mmpll2_d6","univpll2_d5","mmpll2_d5","univpll2_d4","imgpll_d4","mmpll2_d4"];
static camtm_parents: &[&str] = parents!["clk26m","univpll2_d6_d4","ck_osc_d4","ck_osc_d3","univpll2_d6_d2"];
static dpe_parents: &[&str] = parents!["clk26m","mmpll2_d5_d2","univpll2_d4_d2","mmpll2_d7","univpll2_d6","mmpll2_d6","univpll2_d5","mmpll2_d5","imgpll_d4","mmpll2_d4"];
static vdec_parents: &[&str] = parents!["clk26m","ck_mainpll_d5_d2","mainpll2_d4_d4","mainpll2_d7_d2","mainpll2_d6_d2","mainpll2_d5_d2","mainpll2_d9","mainpll2_d4_d2","mainpll2_d7","mainpll2_d6","univpll2_d6","mainpll2_d5","mainpll2_d4","imgpll_d2"];
static ccusys_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_osc_d3","ck_osc_d2","mmpll2_d5_d2","univpll2_d4_d2","mmpll2_d7","univpll2_d6","mmpll2_d6","univpll2_d5","mainpll2_d4","mainpll2_d3","univpll2_d3"];
static ccutm_parents: &[&str] = parents!["clk26m","univpll2_d6_d4","ck_osc_d4","ck_osc_d3","univpll2_d6_d2"];
static venc_parents: &[&str] = parents!["clk26m","mainpll2_d5_d2","univpll2_d5_d2","mainpll2_d4_d2","mmpll2_d9","univpll2_d4_d2","mmpll2_d4_d2","mainpll2_d6","univpll2_d6","mainpll2_d5","mmpll2_d6","univpll2_d5","mainpll2_d4","univpll2_d4","univpll2_d3"];
static dp1_parents: &[&str] = parents!["clk26m","tvdpll2_d16","tvdpll2_d8","tvdpll2_d4","tvdpll2_d2"];
static dp0_parents: &[&str] = parents!["clk26m","tvdpll1_d16","tvdpll1_d8","tvdpll1_d4","ck_tvdpll1_d2"];
static disp_parents: &[&str] = parents!["clk26m","ck_mainpll_d5_d2","ck_mainpll_d4_d2","ck_mainpll_d6","mainpll2_d5","mmpll2_d6","mainpll2_d4","univpll2_d4","mainpll2_d3"];
static mdp_parents: &[&str] = parents!["clk26m","ck_mainpll_d5_d2","mainpll2_d5_d2","mmpll2_d6_d2","mainpll2_d9","mainpll2_d4_d2","mainpll2_d7","mainpll2_d6","mainpll2_d5","mmpll2_d6","mainpll2_d4","univpll2_d4","mainpll2_d3"];
static mminfra_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_mainpll_d7_d2","ck_mainpll_d5_d2","ck_mainpll_d9","mmpll2_d6_d2","mainpll2_d4_d2","ck_mainpll_d6","univpll2_d6","mainpll2_d5","mmpll2_d6","univpll2_d5","mainpll2_d4","univpll2_d4","mainpll2_d3","univpll2_d3"];
static mminfra_snoc_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_mainpll_d7_d2","ck_mainpll_d9","ck_mainpll_d7","ck_mainpll_d6","mmpll2_d4_d2","ck_mainpll_d5","ck_mainpll_d4","univpll2_d4","mmpll2_d4","mainpll2_d3","univpll2_d3","mmpll2_d3","mainpll2_d2"];
static mmup_parents: &[&str] = parents!["clk26m","mainpll2_d6","mainpll2_d5","ck_osc_d2","ck_osc","ck_mainpll_d4","univpll2_d4","mainpll2_d3"];
static mminfra_ao_parents: &[&str] = parents!["clk26m","ck_osc_d4","ck_mainpll_d3"];
static dvo_parents: &[&str] = parents!["clk26m","tvdpll3_d16","tvdpll3_d8","tvdpll3_d4","tvdpll3_d2"];
static dvo_favt_parents: &[&str] = parents!["clk26m","tvdpll3_d16","tvdpll3_d8","tvdpll3_d4","vlp_apll1","vlp_apll2","tvdpll3_d2"];

// MUX declarations retain the C driver's macro-defined layout and ordering.
static top_muxes: [mtk_mux; 25] = [
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF0,"seninf0",seninf_parents,CKSYS2_CLK_CFG_0,CKSYS2_CLK_CFG_0_SET,CKSYS2_CLK_CFG_0_CLR,MM_HWV_CG_30_DONE,MM_HWV_CG_30_SET,MM_HWV_CG_30_CLR,0,4,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF0_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,31),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF1,"seninf1",seninf_parents,CKSYS2_CLK_CFG_0,CKSYS2_CLK_CFG_0_SET,CKSYS2_CLK_CFG_0_CLR,MM_HWV_CG_30_DONE,MM_HWV_CG_30_SET,MM_HWV_CG_30_CLR,8,4,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF1_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,30),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF2,"seninf2",seninf_parents,CKSYS2_CLK_CFG_0,CKSYS2_CLK_CFG_0_SET,CKSYS2_CLK_CFG_0_CLR,MM_HWV_CG_30_DONE,MM_HWV_CG_30_SET,MM_HWV_CG_30_CLR,16,4,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF2_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,29),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF3,"seninf3",seninf_parents,CKSYS2_CLK_CFG_0,CKSYS2_CLK_CFG_0_SET,CKSYS2_CLK_CFG_0_CLR,MM_HWV_CG_30_DONE,MM_HWV_CG_30_SET,MM_HWV_CG_30_CLR,24,4,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF3_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,28),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF4,"seninf4",seninf_parents,CKSYS2_CLK_CFG_1,CKSYS2_CLK_CFG_1_SET,CKSYS2_CLK_CFG_1_CLR,MM_HWV_CG_31_DONE,MM_HWV_CG_31_SET,MM_HWV_CG_31_CLR,0,4,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF4_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,27),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_SENINF5,"seninf5",seninf_parents,CKSYS2_CLK_CFG_1,CKSYS2_CLK_CFG_1_SET,CKSYS2_CLK_CFG_1_CLR,MM_HWV_CG_31_DONE,MM_HWV_CG_31_SET,MM_HWV_CG_31_CLR,8,4,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_SENINF5_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,26),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_IMG1,"img1",img1_parents,CKSYS2_CLK_CFG_1,CKSYS2_CLK_CFG_1_SET,CKSYS2_CLK_CFG_1_CLR,MM_HWV_CG_31_DONE,MM_HWV_CG_31_SET,MM_HWV_CG_31_CLR,16,4,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_IMG1_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,25),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_IPE,"ipe",ipe_parents,CKSYS2_CLK_CFG_1,CKSYS2_CLK_CFG_1_SET,CKSYS2_CLK_CFG_1_CLR,MM_HWV_CG_31_DONE,MM_HWV_CG_31_SET,MM_HWV_CG_31_CLR,24,4,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_IPE_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,24),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_CAM,"cam",cam_parents,CKSYS2_CLK_CFG_2,CKSYS2_CLK_CFG_2_SET,CKSYS2_CLK_CFG_2_CLR,MM_HWV_CG_32_DONE,MM_HWV_CG_32_SET,MM_HWV_CG_32_CLR,0,4,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_CAM_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,23),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_CAMTM,"camtm",camtm_parents,CKSYS2_CLK_CFG_2,CKSYS2_CLK_CFG_2_SET,CKSYS2_CLK_CFG_2_CLR,MM_HWV_CG_32_DONE,MM_HWV_CG_32_SET,MM_HWV_CG_32_CLR,8,3,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_CAMTM_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,22),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_DPE,"dpe",dpe_parents,CKSYS2_CLK_CFG_2,CKSYS2_CLK_CFG_2_SET,CKSYS2_CLK_CFG_2_CLR,MM_HWV_CG_32_DONE,MM_HWV_CG_32_SET,MM_HWV_CG_32_CLR,16,4,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DPE_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,21),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_VDEC,"vdec",vdec_parents,CKSYS2_CLK_CFG_2,CKSYS2_CLK_CFG_2_SET,CKSYS2_CLK_CFG_2_CLR,MM_HWV_CG_32_DONE,MM_HWV_CG_32_SET,MM_HWV_CG_32_CLR,24,4,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_VDEC_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,20),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_CCUSYS,"ccusys",ccusys_parents,CKSYS2_CLK_CFG_3,CKSYS2_CLK_CFG_3_SET,CKSYS2_CLK_CFG_3_CLR,MM_HWV_CG_33_DONE,MM_HWV_CG_33_SET,MM_HWV_CG_33_CLR,0,4,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_CCUSYS_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,19),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_CCUTM,"ccutm",ccutm_parents,CKSYS2_CLK_CFG_3,CKSYS2_CLK_CFG_3_SET,CKSYS2_CLK_CFG_3_CLR,MM_HWV_CG_33_DONE,MM_HWV_CG_33_SET,MM_HWV_CG_33_CLR,8,3,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_CCUTM_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,18),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_VENC,"venc",venc_parents,CKSYS2_CLK_CFG_3,CKSYS2_CLK_CFG_3_SET,CKSYS2_CLK_CFG_3_CLR,MM_HWV_CG_33_DONE,MM_HWV_CG_33_SET,MM_HWV_CG_33_CLR,16,4,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_VENC_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,17),
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_TOP2_DVO,"dvo",dvo_parents,CKSYS2_CLK_CFG_3,CKSYS2_CLK_CFG_3_SET,CKSYS2_CLK_CFG_3_CLR,24,3,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DVO_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,16),
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_TOP2_DVO_FAVT,"dvo_favt",dvo_favt_parents,CKSYS2_CLK_CFG_4,CKSYS2_CLK_CFG_4_SET,CKSYS2_CLK_CFG_4_CLR,0,3,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DVO_FAVT_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,15),
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_TOP2_DP1,"dp1",dp1_parents,CKSYS2_CLK_CFG_4,CKSYS2_CLK_CFG_4_SET,CKSYS2_CLK_CFG_4_CLR,8,3,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DP1_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,14),
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_TOP2_DP0,"dp0",dp0_parents,CKSYS2_CLK_CFG_4,CKSYS2_CLK_CFG_4_SET,CKSYS2_CLK_CFG_4_CLR,16,3,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DP0_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,13),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_DISP,"disp",disp_parents,CKSYS2_CLK_CFG_4,CKSYS2_CLK_CFG_4_SET,CKSYS2_CLK_CFG_4_CLR,MM_HWV_CG_34_DONE,MM_HWV_CG_34_SET,MM_HWV_CG_34_CLR,24,4,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_DISP_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,12),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_MDP,"mdp",mdp_parents,CKSYS2_CLK_CFG_5,CKSYS2_CLK_CFG_5_SET,CKSYS2_CLK_CFG_5_CLR,MM_HWV_CG_35_DONE,MM_HWV_CG_35_SET,MM_HWV_CG_35_CLR,0,4,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_MDP_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,11),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_MMINFRA,"mminfra",mminfra_parents,CKSYS2_CLK_CFG_5,CKSYS2_CLK_CFG_5_SET,CKSYS2_CLK_CFG_5_CLR,MM_HWV_CG_35_DONE,MM_HWV_CG_35_SET,MM_HWV_CG_35_CLR,8,4,15,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_MMINFRA_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,10),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_MMINFRA_SNOC,"mminfra_snoc",mminfra_snoc_parents,CKSYS2_CLK_CFG_5,CKSYS2_CLK_CFG_5_SET,CKSYS2_CLK_CFG_5_CLR,MM_HWV_CG_35_DONE,MM_HWV_CG_35_SET,MM_HWV_CG_35_CLR,16,4,23,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_MMINFRA_SNOC_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,9),
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_TOP2_MMUP,"mmup",mmup_parents,CKSYS2_CLK_CFG_5,CKSYS2_CLK_CFG_5_SET,CKSYS2_CLK_CFG_5_CLR,24,3,31,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_MMUP_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,8),
    MUX_GATE_HWV_FENC_CLR_SET_UPD!(CLK_TOP2_MMINFRA_AO,"mminfra_ao",mminfra_ao_parents,CKSYS2_CLK_CFG_6,CKSYS2_CLK_CFG_6_SET,CKSYS2_CLK_CFG_6_CLR,MM_HWV_CG_36_DONE,MM_HWV_CG_36_SET,MM_HWV_CG_36_CLR,16,2,7,CKSYS2_CLK_CFG_UPDATE,TOP_MUX_MMINFRA_AO_SHIFT,CKSYS2_CLK_FENC_STATUS_MON_0,5),
];

static topck_desc: mtk_clk_desc = mtk_clk_desc { factor_clks: top_divs, num_factor_clks: ARRAY_SIZE!(top_divs), mux_clks: top_muxes, num_mux_clks: ARRAY_SIZE!(top_muxes) };
static of_match_clk_mt8196_ck: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8196-topckgen-gp2", data: &topck_desc },
    of_device_id { /* sentinel */ ..Default::default() },
];
MODULE_DEVICE_TABLE!(of, of_match_clk_mt8196_ck);
static mut clk_mt8196_topck_drv: platform_driver = platform_driver { probe: Some(mtk_clk_simple_probe), remove: Some(mtk_clk_simple_remove), driver: driver { name: "clk-mt8196-topck2", of_match_table: &of_match_clk_mt8196_ck } };
MODULE_DESCRIPTION!("MediaTek MT8196 GP2 top clock generators driver");
module_platform_driver!(clk_mt8196_topck_drv);
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
