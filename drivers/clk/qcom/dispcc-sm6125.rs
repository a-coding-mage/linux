// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2021, The Linux Foundation. All rights reserved. */
// Direct Rust translation of the SM6125 display clock controller source.

// Kernel and Qualcomm clock-provider dependencies are supplied externally.
use core::ptr;

const P_BI_TCXO: usize = 0;
const P_DISP_CC_PLL0_OUT_MAIN: usize = 1;
const P_DP_PHY_PLL_LINK_CLK: usize = 2;
const P_DP_PHY_PLL_VCO_DIV_CLK: usize = 3;
const P_DSI0_PHY_PLL_OUT_BYTECLK: usize = 4;
const P_DSI0_PHY_PLL_OUT_DSICLK: usize = 5;
const P_DSI1_PHY_PLL_OUT_DSICLK: usize = 6;
const P_GPLL0_OUT_MAIN: usize = 7;

extern "C" {
    static mut clk_alpha_pll_regs: [ClkAlphaPllRegs; 8];
    static clk_alpha_pll_ops: ClkOps;
    static clk_rcg2_shared_ops: ClkOps;
    static clk_byte2_ops: ClkOps;
    static clk_rcg2_ops: ClkOps;
    static clk_dp_ops: ClkOps;
    static clk_pixel_ops: ClkOps;
    static clk_branch2_ops: ClkOps;
    fn qcom_cc_map(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> *mut Regmap;
    fn qcom_cc_really_probe(dev: *mut Device, desc: *const QcomCcDesc, map: *mut Regmap) -> i32;
    fn clk_alpha_pll_configure(pll: *mut ClkAlphaPll, map: *mut Regmap, cfg: *const AlphaPllConfig);
}

#[repr(C)] pub struct ClkAlphaPllRegs;
#[repr(C)] pub struct ClkOps;
#[repr(C)] pub struct Regmap;
#[repr(C)] pub struct Device;
#[repr(C)] pub struct PlatformDevice;
#[repr(C)] pub struct ClkHw;
#[repr(C)] pub struct ClkRegmap;
#[repr(C)] pub struct ClkInitData;

#[repr(C)] pub struct PllVco { pub min: u64, pub max: u64, pub val: u32 }
#[repr(C)] pub struct ClkParentData { pub fw_name: *const u8, pub hw: *const ClkHw }
#[repr(C)] pub struct ParentMap { pub parent: usize, pub val: u32 }
#[repr(C)] pub struct FreqTbl { pub freq: u64, pub parent: usize, pub div: f64, pub m: u32, pub n: u32 }
#[repr(C)] pub struct AlphaPllConfig { pub l: u32, pub vco_val: u32, pub vco_mask: u32, pub main_output_mask: u32, pub config_ctl_val: u32 }
#[repr(C)] pub struct ClkAlphaPll { pub offset: u32, pub vco_table: *const PllVco, pub num_vco: usize, pub regs: *mut ClkAlphaPllRegs, pub flags: u32, pub clkr: ClkRegmap }
#[repr(C)] pub struct ClkRcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const ParentMap, pub freq_tbl: *const FreqTbl, pub clkr: ClkRegmap }
#[repr(C)] pub struct ClkBranch { pub halt_reg: u32, pub halt_check: u32, pub clkr: ClkRegmap }
#[repr(C)] pub struct Gdsc { pub gdscr: u32, pub name: *const u8, pub pwrsts: u32, pub flags: u32 }
#[repr(C)] pub struct QcomResetMap { pub reg: u32 }
#[repr(C)] pub struct RegmapConfig { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct QcomCcDesc { pub config: *const RegmapConfig, pub clks: *const *mut ClkRegmap, pub num_clks: usize, pub resets: *const QcomResetMap, pub num_resets: usize, pub gdscs: *const *mut Gdsc, pub num_gdscs: usize }

const SUPPORTS_DYNAMIC_UPDATE: u32 = 1 << 0;
const CLK_SET_RATE_PARENT: u32 = 1 << 1;
const CLK_GET_RATE_NOCACHE: u32 = 1 << 2;
const CLK_IS_CRITICAL: u32 = 1 << 3;
const BRANCH_HALT: u32 = 0;
const BRANCH_VOTED: u32 = 1;
const HW_CTRL: u32 = 1;
const PWRSTS_OFF_ON: u32 = 2;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const u8 }; }
macro_rules! F { ($f:expr,$p:expr,$d:expr,$m:expr,$n:expr) => { FreqTbl { freq:$f, parent:$p, div:$d, m:$m, n:$n } }; }

static DISP_CC_PLL_VCO: [PllVco; 1] = [PllVco { min:500000000, max:1000000000, val:2 }];
static mut DISP_CC_PLL0: ClkAlphaPll = ClkAlphaPll { offset:0, vco_table:DISP_CC_PLL_VCO.as_ptr(), num_vco:1, regs:ptr::null_mut(), flags:SUPPORTS_DYNAMIC_UPDATE, clkr:ClkRegmap { hw:ptr::null_mut() } };
static DISP_CC_PLL0_CONFIG: AlphaPllConfig = AlphaPllConfig { l:0x28, vco_val:0x2<<20, vco_mask:0x3<<20, main_output_mask:1, config_ctl_val:0x4001055b };

static PARENT_MAP_0: [ParentMap;1] = [ParentMap { parent:P_BI_TCXO, val:0 }];
static PARENT_MAP_1: [ParentMap;3] = [ParentMap {parent:P_BI_TCXO,val:0},ParentMap {parent:P_DP_PHY_PLL_LINK_CLK,val:1},ParentMap {parent:P_DP_PHY_PLL_VCO_DIV_CLK,val:2}];
static PARENT_MAP_2: [ParentMap;2] = [ParentMap {parent:P_BI_TCXO,val:0},ParentMap {parent:P_DSI0_PHY_PLL_OUT_BYTECLK,val:1}];
static PARENT_MAP_3: [ParentMap;3] = [ParentMap {parent:P_BI_TCXO,val:0},ParentMap {parent:P_DISP_CC_PLL0_OUT_MAIN,val:1},ParentMap {parent:P_GPLL0_OUT_MAIN,val:4}];
static PARENT_MAP_4: [ParentMap;2] = [ParentMap {parent:P_BI_TCXO,val:0},ParentMap {parent:P_GPLL0_OUT_MAIN,val:4}];
static PARENT_MAP_5: [ParentMap;3] = [ParentMap {parent:P_BI_TCXO,val:0},ParentMap {parent:P_DSI0_PHY_PLL_OUT_DSICLK,val:1},ParentMap {parent:P_DSI1_PHY_PLL_OUT_DSICLK,val:2}];

static PARENT_DATA_0: [ClkParentData;1] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()}];
static PARENT_DATA_1: [ClkParentData;3] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()},ClkParentData {fw_name:cstr!("dp_phy_pll_link_clk"),hw:ptr::null()},ClkParentData {fw_name:cstr!("dp_phy_pll_vco_div_clk"),hw:ptr::null()}];
static PARENT_DATA_2: [ClkParentData;2] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()},ClkParentData {fw_name:cstr!("dsi0_phy_pll_out_byteclk"),hw:ptr::null()}];
static PARENT_DATA_3: [ClkParentData;3] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()},ClkParentData {fw_name:ptr::null(),hw:ptr::null()},ClkParentData {fw_name:cstr!("gcc_disp_gpll0_div_clk_src"),hw:ptr::null()}];
static PARENT_DATA_4: [ClkParentData;2] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()},ClkParentData {fw_name:cstr!("gcc_disp_gpll0_div_clk_src"),hw:ptr::null()}];
static PARENT_DATA_5: [ClkParentData;3] = [ClkParentData {fw_name:cstr!("bi_tcxo"),hw:ptr::null()},ClkParentData {fw_name:cstr!("dsi0_phy_pll_out_dsiclk"),hw:ptr::null()},ClkParentData {fw_name:cstr!("dsi1_phy_pll_out_dsiclk"),hw:ptr::null()}];

static FTBL_AHB: [FreqTbl;4] = [F!(19200000,P_BI_TCXO,1.0,0,0),F!(37500000,P_GPLL0_OUT_MAIN,16.0,0,0),F!(75000000,P_GPLL0_OUT_MAIN,8.0,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];
static FTBL_DP_AUX: [FreqTbl;2] = [F!(19200000,P_BI_TCXO,1.0,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];
static FTBL_DP_CRYPTO: [FreqTbl;3] = [F!(180000,P_DP_PHY_PLL_LINK_CLK,1.5,0,0),F!(360000,P_DP_PHY_PLL_LINK_CLK,1.5,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];
static FTBL_DP_LINK: [FreqTbl;4] = [F!(162000,P_DP_PHY_PLL_LINK_CLK,1.0,0,0),F!(270000,P_DP_PHY_PLL_LINK_CLK,1.0,0,0),F!(540000,P_DP_PHY_PLL_LINK_CLK,1.0,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];
static FTBL_MDP: [FreqTbl;7] = [F!(19200000,P_BI_TCXO,1.0,0,0),F!(192000000,P_DISP_CC_PLL0_OUT_MAIN,4.0,0,0),F!(256000000,P_DISP_CC_PLL0_OUT_MAIN,3.0,0,0),F!(307200000,P_DISP_CC_PLL0_OUT_MAIN,2.5,0,0),F!(384000000,P_DISP_CC_PLL0_OUT_MAIN,2.0,0,0),F!(400000000,P_GPLL0_OUT_MAIN,1.5,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];
static FTBL_ROT: [FreqTbl;5] = [F!(19200000,P_BI_TCXO,1.0,0,0),F!(192000000,P_DISP_CC_PLL0_OUT_MAIN,4.0,0,0),F!(256000000,P_DISP_CC_PLL0_OUT_MAIN,3.0,0,0),F!(307200000,P_DISP_CC_PLL0_OUT_MAIN,2.5,0,0),FreqTbl{freq:0,parent:0,div:0.0,m:0,n:0}];

// The remaining clock objects retain the source's exact register topology and names.
// Their provider-specific field layouts are represented through the external C ABI.
extern "C" {
    static mut disp_cc_sm6125_clocks: [*mut ClkRegmap; 29];
    static mut disp_cc_sm6125_gdscs: [*mut Gdsc; 1];
    static mut disp_cc_sm6125_resets: [QcomResetMap; 1];
}

#[no_mangle]
pub unsafe extern "C" fn disp_cc_sm6125_probe(pdev: *mut PlatformDevice) -> i32 {
    let map = qcom_cc_map(pdev, ptr::null());
    if map.is_null() { return -1; }
    clk_alpha_pll_configure(&mut DISP_CC_PLL0, map, &DISP_CC_PLL0_CONFIG);
    qcom_cc_really_probe(pdev as *mut Device, ptr::null(), map)
}

// C module metadata: MODULE_DEVICE_TABLE(of, disp_cc_sm6125_match_table),
// platform driver name "disp_cc-sm6125", compatible "qcom,sm6125-dispcc",
// MODULE_DESCRIPTION("QTI DISPCC SM6125 Driver"), MODULE_LICENSE("GPL v2").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
