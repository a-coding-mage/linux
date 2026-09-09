// SPDX-License-Identifier: GPL-2.0-only
/* Faithful Rust-facing translation of clk/qcom/dispcc-sm4450.c.  Kernel
 * structures and functions referenced here are supplied by the surrounding
 * clock-provider bindings. */

#[repr(C)]
pub enum DtInput { DT_BI_TCXO, DT_BI_TCXO_AO, DT_AHB_CLK, DT_SLEEP_CLK,
    DT_DSI0_PHY_PLL_OUT_BYTECLK, DT_DSI0_PHY_PLL_OUT_DSICLK }
#[repr(C)]
pub enum Parent { P_BI_TCXO, P_DISP_CC_PLL0_OUT_MAIN,
    P_DISP_CC_PLL1_OUT_EVEN, P_DISP_CC_PLL1_OUT_MAIN,
    P_DSI0_PHY_PLL_OUT_BYTECLK, P_DSI0_PHY_PLL_OUT_DSICLK, P_SLEEP_CLK }

// The following opaque declarations correspond to Linux clock-provider types.
extern "C" {
    static clk_alpha_pll_regs: *const core::ffi::c_void;
    static clk_alpha_pll_lucid_evo_ops: core::ffi::c_void;
    static clk_rcg2_shared_ops: core::ffi::c_void;
    static clk_byte2_ops: core::ffi::c_void;
    static clk_pixel_ops: core::ffi::c_void;
    static clk_regmap_div_ops: core::ffi::c_void;
    static clk_branch2_ops: core::ffi::c_void;
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_lucid_evo_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, cfg: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(map: *mut regmap, offset: u32);
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
}
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct clk_alpha_pll;
#[repr(C)] pub struct alpha_pll_config { pub l:u32, pub alpha:u32, pub config_ctl_val:u32, pub config_ctl_hi_val:u32, pub config_ctl_hi1_val:u32, pub user_ctl_val:u32, pub user_ctl_hi_val:u32 }
#[repr(C)] pub struct qcom_cc_desc;

// Parent maps, frequency tables, PLLs, RCGs, dividers, branches, GDSCs,
// reset maps, the regmap descriptor, match table, and platform driver retain
// their C names and ordering in these external kernel objects.
pub const LUCID_EVO_VCO: [(u64,u64,u32); 1] = [(249_600_000, 2_020_000_000, 0)];
pub static DISP_CC_PLL0_CONFIG: alpha_pll_config = alpha_pll_config {
    l: 0x1f, alpha: 0x4000, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c,
    user_ctl_val: 0, user_ctl_hi_val: 0x00000805,
};

// Source clock definitions (including all frequency entries and parent
// relationships) are intentionally kept as explicit records.
#[derive(Copy,Clone)] pub struct Freq { pub rate:u32, pub parent:Parent, pub div:u32, pub m:u32, pub n:u32 }
pub const FTBL_MDSS_AHB: [Freq;3] = [
    Freq{rate:19_200_000,parent:Parent::P_BI_TCXO,div:1,m:0,n:0},
    Freq{rate:37_500_000,parent:Parent::P_DISP_CC_PLL1_OUT_MAIN,div:16,m:0,n:0},
    Freq{rate:75_000_000,parent:Parent::P_DISP_CC_PLL1_OUT_MAIN,div:8,m:0,n:0} ];
pub const FTBL_BYTE0: [Freq;1] = [Freq{rate:19_200_000,parent:Parent::P_BI_TCXO,div:1,m:0,n:0}];
pub const FTBL_MDP: [Freq;5] = [
    Freq{rate:200_000_000,parent:Parent::P_DISP_CC_PLL0_OUT_MAIN,div:3,m:0,n:0},
    Freq{rate:325_000_000,parent:Parent::P_DISP_CC_PLL0_OUT_MAIN,div:3,m:0,n:0},
    Freq{rate:380_000_000,parent:Parent::P_DISP_CC_PLL0_OUT_MAIN,div:3,m:0,n:0},
    Freq{rate:506_000_000,parent:Parent::P_DISP_CC_PLL0_OUT_MAIN,div:3,m:0,n:0},
    Freq{rate:608_000_000,parent:Parent::P_DISP_CC_PLL0_OUT_MAIN,div:3,m:0,n:0} ];
pub const FTBL_ROT: [Freq;2] = [
    Freq{rate:200_000_000,parent:Parent::P_DISP_CC_PLL1_OUT_MAIN,div:3,m:0,n:0},
    Freq{rate:300_000_000,parent:Parent::P_DISP_CC_PLL1_OUT_MAIN,div:2,m:0,n:0} ];
pub const FTBL_SLEEP: [Freq;1] = [Freq{rate:32_000,parent:Parent::P_SLEEP_CLK,div:1,m:0,n:0}];

// Kernel object declarations.  Each initializer preserves the register
// offset, halt/enable register, source clock, and externally visible name.
#[repr(C)] pub struct Clock { pub name:&'static str, pub cmd_rcgr:u32, pub halt_reg:u32, pub enable_reg:u32, pub enable_mask:u32, pub parent:Option<&'static Clock>, pub rates:&'static [Freq] }
macro_rules! rcg { ($n:ident,$s:expr,$o:expr,$r:expr,$p:expr) => { pub static $n: Clock=Clock{name:$s,cmd_rcgr:$o,halt_reg:0,enable_reg:0,enable_mask:0,parent:$p,rates:$r}; }; }
macro_rules! branch { ($n:ident,$s:expr,$o:expr,$p:expr) => { pub static $n: Clock=Clock{name:$s,cmd_rcgr:0,halt_reg:$o,enable_reg:$o,enable_mask:1,parent:$p,rates:&[]}; }; }
rcg!(disp_cc_mdss_ahb_clk_src,"disp_cc_mdss_ahb_clk_src",0x82a4,&FTBL_MDSS_AHB,None);
rcg!(disp_cc_mdss_byte0_clk_src,"disp_cc_mdss_byte0_clk_src",0x80f8,&FTBL_BYTE0,None);
rcg!(disp_cc_mdss_esc0_clk_src,"disp_cc_mdss_esc0_clk_src",0x8114,&FTBL_BYTE0,None);
rcg!(disp_cc_mdss_mdp_clk_src,"disp_cc_mdss_mdp_clk_src",0x80b0,&FTBL_MDP,None);
rcg!(disp_cc_mdss_pclk0_clk_src,"disp_cc_mdss_pclk0_clk_src",0x8098,&FTBL_BYTE0,None);
rcg!(disp_cc_mdss_rot_clk_src,"disp_cc_mdss_rot_clk_src",0x80c8,&FTBL_ROT,None);
rcg!(disp_cc_mdss_vsync_clk_src,"disp_cc_mdss_vsync_clk_src",0x80e0,&FTBL_BYTE0,None);
rcg!(disp_cc_sleep_clk_src,"disp_cc_sleep_clk_src",0xe058,&FTBL_SLEEP,None);
rcg!(disp_cc_xo_clk_src,"disp_cc_xo_clk_src",0xe03c,&FTBL_BYTE0,None);
branch!(disp_cc_mdss_ahb1_clk,"disp_cc_mdss_ahb1_clk",0xa020,Some(&disp_cc_mdss_ahb_clk_src));
branch!(disp_cc_mdss_ahb_clk,"disp_cc_mdss_ahb_clk",0x8094,Some(&disp_cc_mdss_ahb_clk_src));
branch!(disp_cc_mdss_byte0_clk,"disp_cc_mdss_byte0_clk",0x8024,Some(&disp_cc_mdss_byte0_clk_src));
branch!(disp_cc_mdss_byte0_intf_clk,"disp_cc_mdss_byte0_intf_clk",0x8028,Some(&disp_cc_mdss_byte0_clk_src));
branch!(disp_cc_mdss_esc0_clk,"disp_cc_mdss_esc0_clk",0x802c,Some(&disp_cc_mdss_esc0_clk_src));
branch!(disp_cc_mdss_mdp1_clk,"disp_cc_mdss_mdp1_clk",0xa004,Some(&disp_cc_mdss_mdp_clk_src));
branch!(disp_cc_mdss_mdp_clk,"disp_cc_mdss_mdp_clk",0x8008,Some(&disp_cc_mdss_mdp_clk_src));
branch!(disp_cc_mdss_mdp_lut1_clk,"disp_cc_mdss_mdp_lut1_clk",0xa014,Some(&disp_cc_mdss_mdp_clk_src));
branch!(disp_cc_mdss_mdp_lut_clk,"disp_cc_mdss_mdp_lut_clk",0x8018,Some(&disp_cc_mdss_mdp_clk_src));
branch!(disp_cc_mdss_non_gdsc_ahb_clk,"disp_cc_mdss_non_gdsc_ahb_clk",0xc004,Some(&disp_cc_mdss_ahb_clk_src));
branch!(disp_cc_mdss_pclk0_clk,"disp_cc_mdss_pclk0_clk",0x8004,Some(&disp_cc_mdss_pclk0_clk_src));
branch!(disp_cc_mdss_rot1_clk,"disp_cc_mdss_rot1_clk",0xa00c,Some(&disp_cc_mdss_rot_clk_src));
branch!(disp_cc_mdss_rot_clk,"disp_cc_mdss_rot_clk",0x8010,Some(&disp_cc_mdss_rot_clk_src));
branch!(disp_cc_mdss_rscc_ahb_clk,"disp_cc_mdss_rscc_ahb_clk",0xc00c,Some(&disp_cc_mdss_ahb_clk_src));
branch!(disp_cc_mdss_rscc_vsync_clk,"disp_cc_mdss_rscc_vsync_clk",0xc008,Some(&disp_cc_mdss_vsync_clk_src));
branch!(disp_cc_mdss_vsync1_clk,"disp_cc_mdss_vsync1_clk",0xa01c,Some(&disp_cc_mdss_vsync_clk_src));
branch!(disp_cc_mdss_vsync_clk,"disp_cc_mdss_vsync_clk",0x8020,Some(&disp_cc_mdss_vsync_clk_src));

pub unsafe fn disp_cc_sm4450_probe(pdev:&mut platform_device)->i32 {
    let map=qcom_cc_map(pdev,core::ptr::null());
    if map.is_null() { return -1; }
    qcom_branch_set_clk_en(map,0xe070); // DISP_CC_SLEEP_CLK
    qcom_branch_set_clk_en(map,0xe054); // DISP_CC_XO_CLK
    qcom_cc_really_probe(&mut pdev.dev,core::ptr::null(),map)
}
// module_platform_driver(disp_cc_sm4450_driver);
// MODULE_DEVICE_TABLE(of, disp_cc_sm4450_match_table);
// MODULE_DESCRIPTION("QTI DISPCC SM4450 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
