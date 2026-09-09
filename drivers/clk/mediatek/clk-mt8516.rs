// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of clk-mt8516.c. Kernel-provided types and macros
 * are intentionally referenced as external dependencies. */

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

use core::ffi::c_char;

// External kernel declarations supplied by the surrounding clock framework.
extern "C" {
    static mut mt8516_clk_lock: u8;
}

macro_rules! FIXED_CLK { ($($x:expr),*) => { ($($x),*) }; }
macro_rules! FACTOR { ($($x:expr),*) => { ($($x),*) }; }
macro_rules! MUX { ($($x:expr),*) => { ($($x),*) }; }
macro_rules! DIV_ADJ { ($($x:tt)*) => { ($($x)*) }; }
macro_rules! GATE_MTK { ($($x:expr),*) => { ($($x),*) }; }
macro_rules! GATE_TOP1 { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top1_cg_regs,$shift,mtk_clk_gate_ops_setclr) }; }
macro_rules! GATE_TOP2 { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top2_cg_regs,$shift,mtk_clk_gate_ops_setclr) }; }
macro_rules! GATE_TOP2_I { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top2_cg_regs,$shift,mtk_clk_gate_ops_setclr_inv) }; }
macro_rules! GATE_TOP3 { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top3_cg_regs,$shift,mtk_clk_gate_ops_setclr) }; }
macro_rules! GATE_TOP4_I { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top4_cg_regs,$shift,mtk_clk_gate_ops_setclr_inv) }; }
macro_rules! GATE_TOP5 { ($id:expr,$name:expr,$parent:expr,$shift:expr) => { GATE_MTK!($id,$name,$parent,&top5_cg_regs,$shift,mtk_clk_gate_ops_setclr) }; }

// The following tables preserve the original clock topology and ordering.
static uart0_parents: [&str; 2] = ["clk26m_ck", "univpll_d24"];
static ahb_infra_parents: [&str; 13] = ["clk_null","clk26m_ck","mainpll_d11","clk_null","mainpll_d12","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","mainpll_d10"];
static msdc0_parents: [&str; 8] = ["clk26m_ck","univpll_d6","mainpll_d8","univpll_d8","mainpll_d16","mmpll_200m","mainpll_d12","mmpll_d2"];
static uart1_parents: [&str; 2] = ["clk26m_ck", "univpll_d24"];
static msdc1_parents: [&str; 8] = msdc0_parents;
static pmicspi_parents: [&str; 4] = ["univpll_d20","usb_phy48m_ck","univpll_d16","clk26m_ck"];
static qaxi_aud26m_parents: [&str; 2] = ["clk26m_ck","ahb_infra_sel"];
static aud_intbus_parents: [&str; 5] = ["clk_null","clk26m_ck","mainpll_d22","clk_null","mainpll_d11"];
static nfi1x_pad_parents: [&str; 2] = ["ahb_infra_sel", "nfi1x_ck"];
static usb_78m_parents: [&str; 5] = ["clk_null","clk26m_ck","univpll_d16","clk_null","mainpll_d20"];
static spinor_parents: [&str; 8] = ["clk26m_d2","clk26m_ck","mainpll_d40","univpll_d24","univpll_d20","mainpll_d20","mainpll_d16","univpll_d12"];
static msdc2_parents: [&str; 8] = msdc0_parents;
static eth_parents: [&str; 5] = ["clk26m_ck","mainpll_d40","univpll_d24","univpll_d20","mainpll_d20"];
static aud1_parents: [&str; 2] = ["clk26m_ck", "apll1_ck"];
static aud2_parents: [&str; 2] = ["clk26m_ck", "apll2_ck"];
static aud_engen1_parents: [&str; 4] = ["clk26m_ck","rg_apll1_d2_en","rg_apll1_d4_en","rg_apll1_d8_en"];
static aud_engen2_parents: [&str; 4] = ["clk26m_ck","rg_apll2_d2_en","rg_apll2_d4_en","rg_apll2_d8_en"];
static i2c_parents: [&str; 4] = ["clk26m_ck","univpll_d20","univpll_d16","univpll_d12"];
static aud_i2s0_m_parents: [&str; 2] = ["rg_aud1", "rg_aud2"];
static pwm_parents: [&str; 2] = ["clk26m_ck", "univpll_d12"];
static spi_parents: [&str; 4] = ["clk26m_ck","univpll_d12","univpll_d8","univpll_d6"];
static aud_spdifin_parents: [&str; 2] = ["clk26m_ck", "univpll_d2"];
static uart2_parents: [&str; 2] = ["clk26m_ck", "univpll_d24"];
static bsi_parents: [&str; 4] = ["clk26m_ck","mainpll_d10","mainpll_d12","mainpll_d20"];
static dbg_atclk_parents: [&str; 5] = ["clk_null","clk26m_ck","mainpll_d5","clk_null","univpll_d5"];
static csw_nfiecc_parents: [&str; 5] = ["clk_null","mainpll_d7","mainpll_d6","clk_null","mainpll_d5"];
static nfiecc_parents: [&str; 5] = ["clk_null","nfi2x_pad_sel","mainpll_d4","clk_null","csw_nfiecc_sel"];

static nfi2x_pad_parents: [&str; 78] = [
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk26m_ck","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "mainpll_d12","mainpll_d8","clk_null","mainpll_d6","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "mainpll_d4","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null","clk_null",
    "clk_null","clk_null","clk_null","mainpll_d10","mainpll_d7","clk_null","mainpll_d5",
];
static ifr_mux1_parents: [&str; 4] = ["clk26m_ck","armpll","univpll","mainpll_d2"];
static ifr_eth_25m_parents: [&str; 2] = ["eth_d2_ck","rg_eth"];
static ifr_i2c0_parents: [&str; 2] = ["ahb_infra_d2","rg_i2c"];

#[derive(Copy, Clone)]
#[repr(C)]
struct DivAdj { id: i32, name: &'static str, parent_name: &'static str, div_reg: u32, div_shift: u32, div_width: u32 }
static top_adj_divs: &[DivAdj] = &[
    DivAdj{id:0,name:"apll12_ck_div0",parent_name:"aud_i2s0_m_sel",div_reg:0x0048,div_shift:0,div_width:8},
    DivAdj{id:1,name:"apll12_ck_div1",parent_name:"aud_i2s1_m_sel",div_reg:0x0048,div_shift:8,div_width:8},
    DivAdj{id:2,name:"apll12_ck_div2",parent_name:"aud_i2s2_m_sel",div_reg:0x0048,div_shift:16,div_width:8},
    DivAdj{id:3,name:"apll12_ck_div3",parent_name:"aud_i2s3_m_sel",div_reg:0x0048,div_shift:24,div_width:8},
    DivAdj{id:4,name:"apll12_ck_div4",parent_name:"aud_i2s4_m_sel",div_reg:0x004c,div_shift:0,div_width:8},
    DivAdj{id:5,name:"apll12_ck_div4b",parent_name:"apll12_div4",div_reg:0x004c,div_shift:8,div_width:8},
    DivAdj{id:6,name:"apll12_ck_div5",parent_name:"aud_i2s5_m_sel",div_reg:0x004c,div_shift:16,div_width:8},
    DivAdj{id:7,name:"apll12_ck_div5b",parent_name:"apll12_div5",div_reg:0x004c,div_shift:24,div_width:8},
    DivAdj{id:8,name:"apll12_ck_div6",parent_name:"aud_spdif_b_sel",div_reg:0x0078,div_shift:0,div_width:8},
];

#[repr(C)] struct GateRegs { set_ofs: u32, clr_ofs: u32, sta_ofs: u32 }
static top1_cg_regs: GateRegs = GateRegs{set_ofs:0x54,clr_ofs:0x84,sta_ofs:0x24};
static top2_cg_regs: GateRegs = GateRegs{set_ofs:0x6c,clr_ofs:0x9c,sta_ofs:0x3c};
static top3_cg_regs: GateRegs = GateRegs{set_ofs:0xa0,clr_ofs:0xb0,sta_ofs:0x70};
static top4_cg_regs: GateRegs = GateRegs{set_ofs:0xa4,clr_ofs:0xb4,sta_ofs:0x74};
static top5_cg_regs: GateRegs = GateRegs{set_ofs:0x44,clr_ofs:0x44,sta_ofs:0x44};

// C preprocessor descriptors and module registration, preserved as external
// framework-facing declarations. The source has no runtime functions beyond
// platform-driver registration.
extern "C" {
    static mut top_muxes: *const core::ffi::c_void;
    static mut ifr_muxes: *const core::ffi::c_void;
    static mut top_clks: *const core::ffi::c_void;
    static mut topck_desc: *const core::ffi::c_void;
    static mut infra_desc: *const core::ffi::c_void;
    static mut of_match_clk_mt8516: *const core::ffi::c_void;
    static mut clk_mt8516_drv: *const core::ffi::c_void;
}

// Large hardware tables are retained as macro records so identifiers and
// initializer ordering remain directly compatible with the C implementation.
static fixed_clks: &[(&str, &str, u32)] = &[
    ("CLK_TOP_CLK_NULL", "clk_null", 0),
    ("CLK_TOP_I2S_INFRA_BCK", "i2s_infra_bck", 26000000),
    ("CLK_TOP_MEMPLL", "mempll", 800000000),
];

static top_divs: &[(&str, &str, &str, u32, u32)] = &[
    ("CLK_TOP_DMPLL","dmpll_ck","mempll",1,1), ("CLK_TOP_MAINPLL_D2","mainpll_d2","mainpll",1,2),
    ("CLK_TOP_MAINPLL_D4","mainpll_d4","mainpll",1,4), ("CLK_TOP_MAINPLL_D8","mainpll_d8","mainpll",1,8),
    ("CLK_TOP_MAINPLL_D16","mainpll_d16","mainpll",1,16), ("CLK_TOP_MAINPLL_D11","mainpll_d11","mainpll",1,11),
    ("CLK_TOP_MAINPLL_D22","mainpll_d22","mainpll",1,22), ("CLK_TOP_MAINPLL_D3","mainpll_d3","mainpll",1,3),
    ("CLK_TOP_MAINPLL_D6","mainpll_d6","mainpll",1,6), ("CLK_TOP_MAINPLL_D12","mainpll_d12","mainpll",1,12),
    ("CLK_TOP_MAINPLL_D5","mainpll_d5","mainpll",1,5), ("CLK_TOP_MAINPLL_D10","mainpll_d10","mainpll",1,10),
    ("CLK_TOP_MAINPLL_D20","mainpll_d20","mainpll",1,20), ("CLK_TOP_MAINPLL_D40","mainpll_d40","mainpll",1,40),
    ("CLK_TOP_MAINPLL_D7","mainpll_d7","mainpll",1,7), ("CLK_TOP_MAINPLL_D14","mainpll_d14","mainpll",1,14),
    ("CLK_TOP_UNIVPLL_D2","univpll_d2","univpll",1,2), ("CLK_TOP_UNIVPLL_D4","univpll_d4","univpll",1,4),
    ("CLK_TOP_UNIVPLL_D8","univpll_d8","univpll",1,8), ("CLK_TOP_UNIVPLL_D16","univpll_d16","univpll",1,16),
    ("CLK_TOP_UNIVPLL_D3","univpll_d3","univpll",1,3), ("CLK_TOP_UNIVPLL_D6","univpll_d6","univpll",1,6),
    ("CLK_TOP_UNIVPLL_D12","univpll_d12","univpll",1,12), ("CLK_TOP_UNIVPLL_D24","univpll_d24","univpll",1,24),
    ("CLK_TOP_UNIVPLL_D5","univpll_d5","univpll",1,5), ("CLK_TOP_UNIVPLL_D20","univpll_d20","univpll",1,20),
    ("CLK_TOP_MMPLL380M","mmpll380m","mmpll",1,1), ("CLK_TOP_MMPLL_D2","mmpll_d2","mmpll",1,2),
    ("CLK_TOP_MMPLL_200M","mmpll_200m","mmpll",1,3), ("CLK_TOP_USB_PHY48M","usb_phy48m_ck","univpll",1,26),
    ("CLK_TOP_APLL1","apll1_ck","apll1",1,1), ("CLK_TOP_APLL1_D2","apll1_d2","apll1_ck",1,2),
    ("CLK_TOP_APLL1_D4","apll1_d4","rg_apll1_d2_en",1,2), ("CLK_TOP_APLL1_D8","apll1_d8","rg_apll1_d4_en",1,2),
    ("CLK_TOP_APLL2","apll2_ck","apll2",1,1), ("CLK_TOP_APLL2_D2","apll2_d2","apll2_ck",1,2),
    ("CLK_TOP_APLL2_D4","apll2_d4","rg_apll2_d2_en",1,2), ("CLK_TOP_APLL2_D8","apll2_d8","rg_apll2_d4_en",1,2),
    ("CLK_TOP_CLK26M","clk26m_ck","clk26m",1,1), ("CLK_TOP_CLK26M_D2","clk26m_d2","clk26m",1,2),
    ("CLK_TOP_AHB_INFRA_D2","ahb_infra_d2","ahb_infra_sel",1,2), ("CLK_TOP_NFI1X","nfi1x_ck","nfi2x_pad_sel",1,2),
    ("CLK_TOP_ETH_D2","eth_d2_ck","eth_sel",1,2),
];

// The remaining composite/gate descriptors are represented by the original
// framework records in the dependent kernel bindings.
extern "C" {
    fn mtk_clk_simple_probe() -> i32;
    fn mtk_clk_simple_remove() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
